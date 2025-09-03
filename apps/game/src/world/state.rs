use bevy::prelude::*;
use std::collections::{HashMap, HashSet};
use crate::utils::hex::HexCoord;

#[derive(Resource, Default)]
pub struct WorldState {
    pub seed: u64,
    pub tilemap_entity: Option<Entity>,
    pub generated_chunks: HashSet<ChunkCoord>,
    pub loaded_hexes: HashSet<HexCoord>, // Layer cake system - tracks loaded hex tiles
    pub corruption_map: HashMap<HexCoord, f32>,
    pub player_hex: Option<HexCoord>,
    pub world_progression: u32, // 1-180 progression system
}

impl WorldState {
    pub fn new_with_seed(seed: u64) -> Self {
        Self {
            seed,
            tilemap_entity: None,
            generated_chunks: HashSet::new(),
            loaded_hexes: HashSet::new(), // Initialize layer cake system
            corruption_map: HashMap::new(),
            player_hex: Some(HexCoord::new(0, 0)),
            world_progression: 1,
        }
    }
    
    pub fn add_corruption(&mut self, hex: HexCoord, amount: f32) {
        let current = self.corruption_map.entry(hex).or_insert(0.0);
        *current = (*current + amount).clamp(0.0, 1.0);
    }
    
    pub fn get_corruption(&self, hex: HexCoord) -> f32 {
        self.corruption_map.get(&hex).copied().unwrap_or(0.0)
    }
    
    pub fn spread_corruption(&mut self, center: HexCoord, radius: i32, intensity: f32) {
        for dx in -radius..=radius {
            for dy in -radius..=radius {
                let hex = HexCoord::new(center.x + dx, center.y + dy);
                let distance = ((dx * dx + dy * dy) as f32).sqrt();
                let corruption_amount = intensity * (1.0 - (distance / radius as f32)).max(0.0);
                self.add_corruption(hex, corruption_amount);
            }
        }
    }
    
    pub fn advance_progression(&mut self) {
        if self.world_progression < 180 {
            self.world_progression += 1;
        }
    }
    
    pub fn get_progression_phase(&self) -> ProgressionPhase {
        match self.world_progression {
            1..=30 => ProgressionPhase::EarlyGame,
            31..=60 => ProgressionPhase::MidGame,
            61..=120 => ProgressionPhase::LateGame,
            121..=160 => ProgressionPhase::EndGame,
            161..=180 => ProgressionPhase::FinalAct,
            _ => ProgressionPhase::FinalAct,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProgressionPhase {
    EarlyGame,  // Basic world exploration, simple companions
    MidGame,    // Corruption spreading, companion trauma building
    LateGame,   // Major corruption events, boss encounters
    EndGame,    // Reality breaking down, critical choices
    FinalAct,   // Final confrontation, resolution
}

type ChunkCoord = HexCoord;

#[derive(Resource, Default)]
pub struct AssetHandles {
    pub tilemap_texture: Option<Handle<Image>>,
    pub biome_textures: HashMap<String, Handle<Image>>,
    pub feature_models: HashMap<String, Handle<Scene>>,
    pub character_models: HashMap<String, Handle<Scene>>,
    pub dialogue_files: HashMap<String, Handle<bevy::asset::LoadedUntypedAsset>>,
    pub audio_files: HashMap<String, Handle<bevy::audio::AudioSource>>,
    pub fallback_mesh: Option<Handle<Mesh>>,
    pub fallback_material: Option<Handle<StandardMaterial>>,
}

impl AssetHandles {
    pub fn get_biome_texture(&self, biome_type: &str) -> Option<&Handle<Image>> {
        self.biome_textures.get(biome_type)
    }
    
    pub fn get_feature_model(&self, feature_type: &str) -> Option<&Handle<Scene>> {
        self.feature_models.get(feature_type)
    }
    
    pub fn get_character_model(&self, character_type: &str) -> Option<&Handle<Scene>> {
        self.character_models.get(character_type)
    }
    
    pub fn is_tilemap_loaded(&self) -> bool {
        self.tilemap_texture.is_some()
    }
    
    pub fn get_fallback_mesh(&self) -> Option<&Handle<Mesh>> {
        self.fallback_mesh.as_ref()
    }
    
    pub fn get_fallback_material(&self) -> Option<&Handle<StandardMaterial>> {
        self.fallback_material.as_ref()
    }
}

#[derive(Resource)]
pub struct DreadLevel {
    pub current: f32,
    pub phase: crate::world::components::DreadPhase,
    pub phase_changed_this_frame: bool,
    pub progression_rate: f32,
    pub resistance: f32, // Player's resistance to dread
}

impl Default for DreadLevel {
    fn default() -> Self {
        Self {
            current: 0.0,
            phase: crate::world::components::DreadPhase::Peace,
            phase_changed_this_frame: false,
            progression_rate: 1.0,
            resistance: 0.0,
        }
    }
}

impl DreadLevel {
    pub fn add_dread(&mut self, amount: f32) {
        let effective_amount = amount * self.progression_rate * (1.0 - self.resistance);
        self.current = (self.current + effective_amount).clamp(0.0, 150.0);
    }
    
    pub fn remove_dread(&mut self, amount: f32) {
        self.current = (self.current - amount).max(0.0);
    }
    
    pub fn increase_resistance(&mut self, amount: f32) {
        self.resistance = (self.resistance + amount).clamp(0.0, 0.8); // Max 80% resistance
    }
    
    pub fn is_at_critical_level(&self) -> bool {
        self.current >= 100.0
    }
    
    pub fn can_trigger_boss_encounter(&self) -> bool {
        matches!(
            self.phase,
            crate::world::components::DreadPhase::Terror 
                | crate::world::components::DreadPhase::Void 
                | crate::world::components::DreadPhase::BeyondVoid
        )
    }
}
