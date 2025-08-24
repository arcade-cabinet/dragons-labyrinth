//! Save game manager for handling save/load operations

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use anyhow::Result;
use chrono::{DateTime, Utc};

use crate::{SaveSlot, SaveConfig, world_state::WorldState};

#[derive(Resource, Default)]
pub struct SaveManager {
    pub save_slots: Vec<SaveMetadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveMetadata {
    pub slot: SaveSlot,
    pub name: String,
    pub description: String,
    pub timestamp: DateTime<Utc>,
    pub play_time: f32,
    pub level: u32,
    pub location: String,
    pub version: String,
    pub thumbnail: Option<Vec<u8>>,
}

#[derive(Serialize, Deserialize)]
pub struct SaveGame {
    pub metadata: SaveMetadata,
    pub world_state: WorldState,
    pub compressed_data: Vec<u8>,
}

impl SaveManager {
    pub fn new() -> Self {
        Self {
            save_slots: Vec::new(),
        }
    }
    
    pub fn save_game(&self, slot: &SaveSlot, description: Option<&str>) -> Result<()> {
        info!("Saving game to slot: {:?}", slot);
        
        // Create save metadata
        let metadata = SaveMetadata {
            slot: slot.clone(),
            name: self.generate_save_name(slot),
            description: description.unwrap_or("Manual Save").to_string(),
            timestamp: Utc::now(),
            play_time: 0.0, // TODO: Track actual play time
            level: 1, // TODO: Get actual level
            location: "Unknown".to_string(), // TODO: Get actual location
            version: env!("CARGO_PKG_VERSION").to_string(),
            thumbnail: None, // TODO: Generate thumbnail
        };
        
        // Collect world state
        let world_state = WorldState::capture();
        
        // Serialize and compress
        let serialized = bincode::serialize(&world_state)?;
        let compressed = zstd::encode_all(&serialized[..], 3)?;
        
        let save_game = SaveGame {
            metadata,
            world_state,
            compressed_data: compressed,
        };
        
        // Write to file
        let save_path = self.get_save_path(slot);
        let save_data = bincode::serialize(&save_game)?;
        fs::write(save_path, save_data)?;
        
        info!("Game saved successfully");
        Ok(())
    }
    
    pub fn load_game(&self, slot: &SaveSlot) -> Result<SaveGame> {
        info!("Loading game from slot: {:?}", slot);
        
        let save_path = self.get_save_path(slot);
        let save_data = fs::read(save_path)?;
        let save_game: SaveGame = bincode::deserialize(&save_data)?;
        
        // Decompress if needed
        if !save_game.compressed_data.is_empty() {
            let decompressed = zstd::decode_all(&save_game.compressed_data[..])?;
            // Additional processing if needed
        }
        
        info!("Game loaded successfully");
        Ok(save_game)
    }
    
    pub fn delete_save(&self, slot: &SaveSlot) -> Result<()> {
        let save_path = self.get_save_path(slot);
        fs::remove_file(save_path)?;
        Ok(())
    }
    
    pub fn list_saves(&self) -> Vec<SaveMetadata> {
        self.save_slots.clone()
    }
    
    pub fn refresh_save_list(&mut self, config: &SaveConfig) -> Result<()> {
        self.save_slots.clear();
        
        let save_dir = &config.save_directory;
        if !save_dir.exists() {
            return Ok(());
        }
        
        for entry in fs::read_dir(save_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().and_then(|s| s.to_str()) == Some("sav") {
                if let Ok(data) = fs::read(&path) {
                    if let Ok(save_game) = bincode::deserialize::<SaveGame>(&data) {
                        self.save_slots.push(save_game.metadata);
                    }
                }
            }
        }
        
        // Sort by timestamp (newest first)
        self.save_slots.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        
        Ok(())
    }
    
    fn get_save_path(&self, slot: &SaveSlot) -> PathBuf {
        let filename = match slot {
            SaveSlot::Auto => "autosave.sav",
            SaveSlot::Quick => "quicksave.sav",
            SaveSlot::Manual(n) => &format!("save_{:03}.sav", n),
        };
        
        // TODO: Get actual save directory from config
        PathBuf::from("saves").join(filename)
    }
    
    fn generate_save_name(&self, slot: &SaveSlot) -> String {
        match slot {
            SaveSlot::Auto => "Auto Save".to_string(),
            SaveSlot::Quick => "Quick Save".to_string(),
            SaveSlot::Manual(n) => format!("Save {}", n),
        }
    }
}

/// Settings that persist across saves
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistentSettings {
    pub master_volume: f32,
    pub music_volume: f32,
    pub sfx_volume: f32,
    pub graphics_quality: GraphicsQuality,
    pub vsync: bool,
    pub fullscreen: bool,
    pub resolution: (u32, u32),
    pub key_bindings: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GraphicsQuality {
    Low,
    Medium,
    High,
    Ultra,
}

impl Default for PersistentSettings {
    fn default() -> Self {
        Self {
            master_volume: 1.0,
            music_volume: 0.7,
            sfx_volume: 1.0,
            graphics_quality: GraphicsQuality::High,
            vsync: true,
            fullscreen: false,
            resolution: (1920, 1080),
            key_bindings: std::collections::HashMap::new(),
        }
    }
}