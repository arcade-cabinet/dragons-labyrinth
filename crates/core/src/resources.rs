// Resource definitions for Dragon's Labyrinth global game state
use bevy::prelude::*;
use hexx::Hex;
use std::collections::HashMap;
use crate::components::*;
use crate::generators::*;

/// Global dread state resource - core mechanic of Dragon's Labyrinth
#[derive(Resource, Default, Clone, Debug)]
pub struct DreadState {
    pub level: u8,           // 0-4 (Peace, Unease, Dread, Terror, Horror)
    pub progress: f32,       // 0.0-1.0 progress to next level
    pub total_events: u32,   // Total number of dread-inducing events
    pub corruption_spread: f32, // How far corruption has spread
}

impl DreadState {
    pub fn get_stage_name(&self) -> &'static str {
        match self.level {
            0 => "Peace",
            1 => "Unease", 
            2 => "Dread",
            3 => "Terror",
            4 => "Horror",
            _ => "Unknown",
        }
    }
    
    pub fn increase_dread(&mut self, amount: f32) {
        self.progress += amount;
        if self.progress >= 1.0 && self.level < 4 {
            self.level += 1;
            self.progress = 0.0;
            self.total_events += 1;
        }
    }
    
    pub fn get_corruption_intensity(&self) -> f32 {
        self.level as f32 / 4.0 + self.progress * 0.25
    }
}

/// Hex world resource containing all tiles and world state
#[derive(Resource, Default, Clone)]
pub struct HexWorld {
    pub tiles: HashMap<Hex, HexTile>,
    pub world_radius: i32,
    pub corruption_epicenter: Option<Hex>,
    pub dragon_location: Option<Hex>,
    pub active_regions: Vec<WorldRegion>,
}

#[derive(Clone, Debug)]
pub struct WorldRegion {
    pub name: String,
    pub center: Hex,
    pub radius: i32,
    pub biome: BiomeType,
    pub corruption_level: f32,
    pub spawn_tables: HashMap<String, f32>, // Monster type -> spawn weight
}

#[derive(Clone, Debug)]
pub enum BiomeType {
    Meadow,      // Peace stage
    Forest,      // Unease stage  
    Swamp,       // Dread stage
    Ruins,       // Terror stage
    Labyrinth,   // Horror stage
}

impl HexWorld {
    pub fn get_tile(&self, hex: Hex) -> Option<&HexTile> {
        self.tiles.get(&hex)
    }
    
    pub fn set_tile_corruption(&mut self, hex: Hex, corruption: f32) {
        if let Some(tile) = self.tiles.get_mut(&hex) {
            tile.corruption = corruption.clamp(0.0, 1.0);
            if corruption >= 0.8 {
                tile.tile_type = TileType::Corrupted;
            }
        }
    }
    
    pub fn spread_corruption(&mut self, epicenter: Hex, radius: f32, intensity: f32) {
        for (hex, tile) in self.tiles.iter_mut() {
            let distance = epicenter.distance_to(*hex) as f32;
            if distance <= radius {
                let corruption_amount = intensity * (1.0 - distance / radius);
                tile.corruption = (tile.corruption + corruption_amount).min(1.0);
            }
        }
    }
}

/// Narrative state tracking story progression and choices
#[derive(Resource, Default, Clone, Debug)]
pub struct NarrativeState {
    pub current_act: u8,
    pub completed_quests: Vec<String>,
    pub active_quests: Vec<String>,
    pub moral_choices_made: Vec<MoralChoiceRecord>,
    pub companion_relationships: HashMap<String, CompanionRelationship>,
    pub unlocked_endings: Vec<String>,
    pub player_reputation: PlayerReputation,
}

#[derive(Clone, Debug)]
pub struct MoralChoiceRecord {
    pub quest_id: String,
    pub choice_made: String,
    pub consequences_applied: Vec<String>,
    pub timestamp: f32,
}

#[derive(Clone, Debug)]
pub struct CompanionRelationship {
    pub trust: f32,
    pub fear: f32,
    pub loyalty: f32,
    pub trauma_witnessed: Vec<String>,
    pub status: CompanionStatus,
}

#[derive(Clone, Debug)]
pub enum CompanionStatus {
    Active,
    Fled,
    Betrayed,
    Dead,
    Transformed,
}

#[derive(Clone, Debug, Default)]
pub struct PlayerReputation {
    pub empathy_score: f32,
    pub brutality_score: f32,
    pub wisdom_score: f32,
    pub corruption_resistance: f32,
}

/// Companion state resource
#[derive(Resource, Default, Clone, Debug)]
pub struct CompanionState {
    pub active_companions: Vec<String>,
    pub companion_morale: HashMap<String, f32>,
    pub group_cohesion: f32,
    pub leadership_effectiveness: f32,
}

impl CompanionState {
    pub fn get_group_sanity(&self) -> f32 {
        self.companion_morale.values().sum::<f32>() / self.companion_morale.len().max(1) as f32
    }
    
    pub fn apply_trauma_to_all(&mut self, trauma_amount: f32) {
        for morale in self.companion_morale.values_mut() {
            *morale = (*morale - trauma_amount).max(0.0);
        }
        self.group_cohesion = (self.group_cohesion - trauma_amount * 0.5).max(0.0);
    }
}

/// Player state resource
#[derive(Resource, Default, Clone, Debug)]
pub struct PlayerState {
    pub sanity: f32,
    pub health: f32,
    pub corruption_level: f32,
    pub inventory: Vec<String>,
    pub current_hex: Hex,
    pub movement_points: u32,
    pub action_points: u32,
}

impl PlayerState {
    pub fn apply_sanity_damage(&mut self, damage: f32, dread_multiplier: f32) {
        let actual_damage = damage * (1.0 + dread_multiplier);
        self.sanity = (self.sanity - actual_damage).max(0.0);
        
        // Corruption increases as sanity decreases
        if self.sanity < 50.0 {
            self.corruption_level += actual_damage * 0.1;
        }
    }
    
    pub fn is_corrupted(&self) -> bool {
        self.corruption_level >= 0.8 || self.sanity <= 10.0
    }
}

/// Audio state resource for managing dynamic audio
#[derive(Resource, Default, Clone, Debug)]
pub struct AudioState {
    pub ambient_volume: f32,
    pub music_track: Option<String>,
    pub active_soundscapes: Vec<Soundscape>,
    pub audio_corruption_level: f32,
    pub heartbeat_active: bool,
    pub whisper_frequency: f32,
}

#[derive(Clone, Debug)]
pub struct Soundscape {
    pub name: String,
    pub volume: f32,
    pub fade_duration: f32,
    pub dread_responsive: bool,
}

impl AudioState {
    pub fn update_for_dread_level(&mut self, dread_level: u8) {
        self.audio_corruption_level = dread_level as f32 / 4.0;
        
        match dread_level {
            0 => {
                self.ambient_volume = 0.5;
                self.heartbeat_active = false;
                self.whisper_frequency = 0.0;
            },
            1 => {
                self.ambient_volume = 0.4;
                self.heartbeat_active = false;
                self.whisper_frequency = 0.1;
            },
            2 => {
                self.ambient_volume = 0.3;
                self.heartbeat_active = true;
                self.whisper_frequency = 0.3;
            },
            3 => {
                self.ambient_volume = 0.2;
                self.heartbeat_active = true;
                self.whisper_frequency = 0.6;
            },
            4 => {
                self.ambient_volume = 0.1;
                self.heartbeat_active = true;
                self.whisper_frequency = 1.0;
            },
            _ => {}
        }
    }
}

/// Game time resource for tracking passage of time
#[derive(Resource, Default, Clone, Debug)]
pub struct GameTime {
    pub day: u32,
    pub hour: f32,
    pub time_scale: f32,
    pub paused: bool,
}

impl GameTime {
    pub fn advance(&mut self, delta_seconds: f32) {
        if !self.paused {
            self.hour += delta_seconds * self.time_scale / 3600.0; // Convert to game hours
            if self.hour >= 24.0 {
                self.day += 1;
                self.hour -= 24.0;
            }
        }
    }
    
    pub fn is_night(&self) -> bool {
        self.hour < 6.0 || self.hour > 20.0
    }
}

/// Asset generation state resource
#[derive(Resource, Default, Clone, Debug)]
pub struct AssetGenerationState {
    pub style_guide_generated: bool,
    pub sprites_generated: HashMap<String, bool>,
    pub tiles_generated: HashMap<String, bool>,
    pub audio_generated: HashMap<String, bool>,
    pub generation_queue: Vec<AssetGenerationTask>,
}

#[derive(Clone, Debug)]
pub struct AssetGenerationTask {
    pub task_type: AssetType,
    pub parameters: HashMap<String, String>,
    pub priority: u8,
    pub dependencies: Vec<String>,
}

#[derive(Clone, Debug)]
pub enum AssetType {
    Sprite,
    Tile,
    Audio,
    UI,
    Dialogue,
}

/// Performance metrics resource
#[derive(Resource, Default, Clone, Debug)]
pub struct PerformanceMetrics {
    pub fps: f32,
    pub frame_time: f32,
    pub memory_usage: u64,
    pub entity_count: u32,
    pub system_performance: HashMap<String, f32>,
}

/// Input state resource for handling complex input combinations
#[derive(Resource, Default, Clone, Debug)]
pub struct InputState {
    pub movement_input: Option<Hex>,
    pub action_queued: Option<PlayerAction>,
    pub dialogue_choice: Option<u8>,
    pub inventory_open: bool,
    pub debug_mode: bool,
}

#[derive(Clone, Debug)]
pub enum PlayerAction {
    Interact,
    UseItem(String),
    Cast(String),
    Rest,
    Examine,
}

/// Save/Load state resource
#[derive(Resource, Default, Clone, Debug)]
pub struct SaveState {
    pub last_save_time: f32,
    pub auto_save_enabled: bool,
    pub save_slots: HashMap<u8, SaveSlot>,
    pub current_slot: Option<u8>,
}

#[derive(Clone, Debug)]
pub struct SaveSlot {
    pub name: String,
    pub timestamp: String,
    pub dread_level: u8,
    pub location: String,
    pub playtime: f32,
}