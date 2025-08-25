//! Hex Rendering Components - ECS components for hex tile visualization

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Hex tile component for ECS integration
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct HexTile {
    pub tile_id: Uuid,
    pub q: i32,
    pub r: i32,
    pub s: i32,
    pub biome_type: String,
    pub hbf_uuid: Option<String>,
}

/// Tile visual data for rendering
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct TileVisuals {
    pub texture_id: String,
    pub base_color: Color,
    pub overlay_textures: Vec<String>,
    pub animation_state: TileAnimationState,
}

#[derive(Debug, Clone, Reflect, Serialize, Deserialize)]
pub enum TileAnimationState {
    Static,
    Pulsing,     // For corruption effects
    Flowing,     // For rivers
    Flickering,  // For unstable areas
}

/// Corruption visualization component
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct CorruptionVisuals {
    pub corruption_level: f32,
    pub dread_intensity: i32,
    pub corruption_overlay: String,
    pub particle_effects: Vec<String>,
    pub color_tint: Color,
}

impl CorruptionVisuals {
    pub fn new(corruption_level: f32, dread_intensity: i32) -> Self {
        Self {
            corruption_level,
            dread_intensity,
            corruption_overlay: Self::get_corruption_overlay(dread_intensity),
            particle_effects: Self::get_particle_effects(corruption_level),
            color_tint: Self::get_corruption_tint(corruption_level),
        }
    }
    
    fn get_corruption_overlay(dread_level: i32) -> String {
        match dread_level {
            0 => "none".to_string(),
            1 => "faint_shadows".to_string(),
            2 => "dark_veins".to_string(),
            3 => "writhing_darkness".to_string(),
            4 => "void_tendrils".to_string(),
            _ => "none".to_string(),
        }
    }
    
    fn get_particle_effects(corruption_level: f32) -> Vec<String> {
        let mut effects = Vec::new();
        if corruption_level > 0.2 {
            effects.push("dark_motes".to_string());
        }
        if corruption_level > 0.5 {
            effects.push("whisper_trails".to_string());
        }
        if corruption_level > 0.8 {
            effects.push("void_cracks".to_string());
        }
        effects
    }
    
    fn get_corruption_tint(corruption_level: f32) -> Color {
        Color::srgba(
            1.0 + corruption_level * 0.3,
            1.0 - corruption_level * 0.5,
            1.0 - corruption_level * 0.4,
            1.0
        )
    }
}

/// Features present on hex tile
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct TileFeatures {
    pub rivers: Vec<u8>,     // River edges (0-5)
    pub trails: Vec<u8>,     // Trail edges (0-5)
    pub settlements: Vec<Uuid>, // Settlement IDs on this tile
    pub dungeons: Vec<Uuid>,    // Dungeon IDs on this tile
    pub special_features: Vec<String>, // "ruins", "bridge", "watchtower", etc.
}

/// Discovery state for exploration mechanics
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct DiscoveryState {
    pub discovered: bool,
    pub fully_explored: bool,
    pub discovery_time: Option<f64>, // Game time when discovered
    pub exploration_progress: f32,   // 0.0 to 1.0
}

impl Default for DiscoveryState {
    fn default() -> Self {
        Self {
            discovered: false,
            fully_explored: false,
            discovery_time: None,
            exploration_progress: 0.0,
        }
    }
}

/// Weather effects on tile
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct TileWeather {
    pub current_condition: String,
    pub visibility_modifier: f32,
    pub movement_modifier: f32,
    pub weather_overlay: Option<String>,
    pub seasonal_effects: Vec<String>,
}

impl Default for TileWeather {
    fn default() -> Self {
        Self {
            current_condition: "Clear".to_string(),
            visibility_modifier: 1.0,
            movement_modifier: 1.0,
            weather_overlay: None,
            seasonal_effects: Vec::new(),
        }
    }
}

/// Tile interaction capabilities
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct TileInteraction {
    pub can_enter: bool,
    pub enter_cost: f32,        // Movement points required
    pub interaction_options: Vec<InteractionOption>,
    pub requires_special_ability: Option<String>,
}

#[derive(Debug, Clone, Reflect, Serialize, Deserialize)]
pub struct InteractionOption {
    pub option_type: InteractionType,
    pub label: String,
    pub description: String,
    pub requirements: Vec<String>,
}

#[derive(Debug, Clone, Reflect, Serialize, Deserialize)]
pub enum InteractionType {
    EnterSettlement,
    EnterDungeon,
    CrossRiver,
    UseTrail,
    Rest,
    Search,
    Investigate,
    UseFeature,
}

/// Viewport tracking for efficient rendering
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct ViewportTile {
    pub in_viewport: bool,
    pub distance_from_center: u32,
    pub render_priority: u32, // Lower = render first
}

/// Tile selection and highlighting
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct TileSelection {
    pub is_selected: bool,
    pub is_highlighted: bool,
    pub selection_type: SelectionType,
    pub highlight_color: Color,
}

#[derive(Debug, Clone, Reflect, Serialize, Deserialize)]
pub enum SelectionType {
    PlayerPosition,
    MovementTarget,
    ExplorationTarget,
    CombatTarget,
    QuestObjective,
}

impl Default for TileSelection {
    fn default() -> Self {
        Self {
            is_selected: false,
            is_highlighted: false,
            selection_type: SelectionType::PlayerPosition,
            highlight_color: Color::WHITE,
        }
    }
}

/// Audio cues associated with tile
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct TileAudio {
    pub ambient_sound: Option<String>,
    pub proximity_sounds: Vec<String>, // Sounds that play when near
    pub interaction_sounds: Vec<String>, // Sounds when interacting
    pub horror_audio: Option<String>,    // Horror-specific audio effects
}

impl Default for TileAudio {
    fn default() -> Self {
        Self {
            ambient_sound: None,
            proximity_sounds: Vec::new(),
            interaction_sounds: Vec::new(),
            horror_audio: None,
        }
    }
}

/// Pathfinding data for movement calculations
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct PathfindingData {
    pub movement_cost: f32,
    pub blocks_movement: bool,
    pub elevation: i32,
    pub pathfinding_tags: Vec<String>, // "water", "difficult", "safe", etc.
}

impl PathfindingData {
    pub fn new(biome: &str, corruption_level: f32) -> Self {
        let base_cost = match biome {
            "swamp" => 2.0,
            "mountain" => 1.5,
            "jungle" => 1.3,
            "forest" => 1.1,
            "plains" => 1.0,
            "desert" => 1.2,
            "tundra" => 1.4,
            _ => 1.0,
        };
        
        let corruption_modifier = 1.0 + (corruption_level * 0.5);
        let final_cost = base_cost * corruption_modifier;
        
        let mut tags = vec![biome.to_string()];
        if corruption_level > 0.3 {
            tags.push("corrupted".to_string());
        }
        if final_cost > 1.5 {
            tags.push("difficult".to_string());
        }
        
        Self {
            movement_cost: final_cost,
            blocks_movement: false,
            elevation: 0, // Would be calculated from biome/features
            pathfinding_tags: tags,
        }
    }
}
