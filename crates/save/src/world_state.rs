//! World state serialization for save games

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use dragons_core::{components::*, resources::*};

/// Complete world state that can be saved/loaded
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldState {
    pub player_state: PlayerSaveData,
    pub map_state: MapSaveData,
    pub entity_states: Vec<EntitySaveData>,
    pub quest_state: QuestSaveData,
    pub inventory: InventorySaveData,
    pub game_flags: GameFlags,
    pub statistics: GameStatistics,
}

impl WorldState {
    /// Capture current world state from ECS
    pub fn capture() -> Self {
        // TODO: Actually query ECS for current state
        Self::default()
    }
    
    /// Restore world state to ECS
    pub fn restore(&self) {
        // TODO: Actually restore state to ECS
        info!("Restoring world state");
    }
}

impl Default for WorldState {
    fn default() -> Self {
        Self {
            player_state: PlayerSaveData::default(),
            map_state: MapSaveData::default(),
            entity_states: Vec::new(),
            quest_state: QuestSaveData::default(),
            inventory: InventorySaveData::default(),
            game_flags: GameFlags::default(),
            statistics: GameStatistics::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlayerSaveData {
    pub position: Vec3,
    pub rotation: Quat,
    pub health: f32,
    pub max_health: f32,
    pub sanity: f32,
    pub max_sanity: f32,
    pub dread_level: u8,
    pub corruption: f32,
    pub experience: u32,
    pub level: u32,
    pub skill_points: u32,
    pub attributes: PlayerAttributes,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlayerAttributes {
    pub strength: u32,
    pub dexterity: u32,
    pub intelligence: u32,
    pub wisdom: u32,
    pub constitution: u32,
    pub charisma: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MapSaveData {
    pub current_map: String,
    pub discovered_tiles: Vec<(i32, i32)>,
    pub fog_of_war: Vec<bool>,
    pub map_markers: Vec<MapMarker>,
    pub visited_maps: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapMarker {
    pub position: Vec2,
    pub label: String,
    pub icon: String,
    pub color: [f32; 4],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntitySaveData {
    pub entity_type: String,
    pub position: Vec3,
    pub rotation: Quat,
    pub health: Option<f32>,
    pub ai_state: Option<String>,
    pub custom_data: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QuestSaveData {
    pub active_quests: Vec<QuestData>,
    pub completed_quests: Vec<String>,
    pub failed_quests: Vec<String>,
    pub quest_variables: std::collections::HashMap<String, i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestData {
    pub id: String,
    pub name: String,
    pub description: String,
    pub objectives: Vec<QuestObjective>,
    pub rewards: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestObjective {
    pub id: String,
    pub description: String,
    pub completed: bool,
    pub current_progress: u32,
    pub required_progress: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InventorySaveData {
    pub items: Vec<ItemData>,
    pub equipped: EquippedItems,
    pub currency: u32,
    pub capacity: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemData {
    pub id: String,
    pub name: String,
    pub quantity: u32,
    pub durability: Option<f32>,
    pub enchantments: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EquippedItems {
    pub weapon: Option<String>,
    pub armor: Option<String>,
    pub accessory1: Option<String>,
    pub accessory2: Option<String>,
    pub consumable_slots: Vec<Option<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GameFlags {
    pub flags: std::collections::HashSet<String>,
    pub counters: std::collections::HashMap<String, i32>,
    pub timers: std::collections::HashMap<String, f32>,
}

impl GameFlags {
    pub fn set_flag(&mut self, flag: &str) {
        self.flags.insert(flag.to_string());
    }
    
    pub fn has_flag(&self, flag: &str) -> bool {
        self.flags.contains(flag)
    }
    
    pub fn increment_counter(&mut self, counter: &str, amount: i32) {
        *self.counters.entry(counter.to_string()).or_insert(0) += amount;
    }
    
    pub fn get_counter(&self, counter: &str) -> i32 {
        self.counters.get(counter).copied().unwrap_or(0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GameStatistics {
    pub play_time: f32,
    pub enemies_defeated: u32,
    pub bosses_defeated: Vec<String>,
    pub deaths: u32,
    pub distance_traveled: f32,
    pub items_collected: u32,
    pub secrets_found: u32,
    pub dialogue_choices: std::collections::HashMap<String, String>,
}

/// System to periodically capture world state for auto-saves
pub fn capture_world_state_system(
    player_query: Query<(&Transform, &Player), With<Player>>,
    mut world_state: ResMut<WorldState>,
) {
    // Capture player state
    if let Ok((transform, player)) = player_query.get_single() {
        world_state.player_state.position = transform.translation;
        world_state.player_state.rotation = transform.rotation;
        // TODO: Capture other player data
    }
    
    // TODO: Capture other world state
}

/// System to restore world state after loading
pub fn restore_world_state_system(
    world_state: Res<WorldState>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    // Restore player position
    if let Ok(mut transform) = player_query.get_single_mut() {
        transform.translation = world_state.player_state.position;
        transform.rotation = world_state.player_state.rotation;
    }
    
    // TODO: Restore other world state
}