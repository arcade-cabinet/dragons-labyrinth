use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Resource, Default)]
pub struct GameState {
    pub save_data: SaveData,
    pub current_dialogue: Option<String>,
    pub active_encounters: Vec<Entity>,
    pub boss_encounters_completed: Vec<String>,
    pub world_events: Vec<WorldEvent>,
    pub game_time: f32, // Time in seconds since game start
    pub player_choices: Vec<PlayerChoice>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SaveData {
    pub progression: u32,
    pub dread_level: f32,
    pub player_stats: PlayerStats,
    pub companion_states: Vec<CompanionSaveState>,
    pub world_corruption: HashMap<String, f32>, // Serialized hex coords
    pub unlocked_areas: Vec<String>,
    pub completed_encounters: Vec<String>,
    pub story_flags: HashMap<String, bool>,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerStats {
    pub health: f32,
    pub max_health: f32,
    pub sanity: f32,
    pub max_sanity: f32,
    pub position: (i32, i32), // Hex coordinates
    pub mount: Option<String>,
    pub inventory: Vec<ItemSaveState>,
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            health: 100.0,
            max_health: 100.0,
            sanity: 100.0,
            max_sanity: 100.0,
            position: (0, 0),
            mount: None,
            inventory: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanionSaveState {
    pub name: String,
    pub companion_type: String,
    pub stress: f32,
    pub trust: f32,
    pub trauma_level: String,
    pub dialogue_flags: HashMap<String, bool>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemSaveState {
    pub name: String,
    pub item_type: String,
    pub quantity: u32,
    pub data: HashMap<String, String>, // Flexible storage for item-specific data
}

#[derive(Debug, Clone)]
pub struct WorldEvent {
    pub event_type: WorldEventType,
    pub trigger_time: f32,
    pub location: Option<crate::utils::hex::HexCoord>,
    pub is_completed: bool,
    pub consequences: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum WorldEventType {
    CorruptionSpread {
        center: crate::utils::hex::HexCoord,
        radius: u32,
        intensity: f32,
    },
    BossSpawn {
        boss_type: String,
        location: crate::utils::hex::HexCoord,
        cr: u32,
    },
    CompanionCrisis {
        companion_name: String,
        crisis_type: String,
    },
    VoidIncursion {
        tear_location: crate::utils::hex::HexCoord,
        stability: f32,
    },
    PlayerChoice {
        choice_id: String,
        consequences: Vec<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerChoice {
    pub choice_id: String,
    pub choice_text: String,
    pub timestamp: f32,
    pub consequences_applied: bool,
    pub dread_impact: f32,
    pub companion_reactions: HashMap<String, String>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            save_data: SaveData::default(),
            current_dialogue: None,
            active_encounters: Vec::new(),
            boss_encounters_completed: Vec::new(),
            world_events: Vec::new(),
            game_time: 0.0,
            player_choices: Vec::new(),
        }
    }
    
    pub fn add_world_event(&mut self, event: WorldEvent) {
        self.world_events.push(event);
    }
    
    pub fn complete_boss_encounter(&mut self, boss_type: String) {
        if !self.boss_encounters_completed.contains(&boss_type) {
            self.boss_encounters_completed.push(boss_type);
        }
    }
    
    pub fn record_player_choice(&mut self, choice: PlayerChoice) {
        self.player_choices.push(choice);
    }
    
    pub fn update_game_time(&mut self, delta_seconds: f32) {
        self.game_time += delta_seconds;
    }
    
    pub fn get_story_flag(&self, flag_name: &str) -> bool {
        self.save_data.story_flags.get(flag_name).copied().unwrap_or(false)
    }
    
    pub fn set_story_flag(&mut self, flag_name: String, value: bool) {
        self.save_data.story_flags.insert(flag_name, value);
    }
    
    pub fn can_encounter_boss(&self, boss_type: &str) -> bool {
        !self.boss_encounters_completed.contains(&boss_type.to_string())
    }
    
    pub fn get_save_data(&self) -> &SaveData {
        &self.save_data
    }
    
    pub fn apply_save_data(&mut self, save_data: SaveData) {
        self.save_data = save_data;
    }
}

#[derive(Debug, Clone)]
pub struct EncounterState {
    pub encounter_id: String,
    pub encounter_type: EncounterType,
    pub participants: Vec<Entity>,
    pub turn_order: Vec<Entity>,
    pub current_turn: usize,
    pub is_boss_fight: bool,
    pub environmental_effects: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum EncounterType {
    Combat {
        enemy_types: Vec<String>,
        terrain_modifiers: HashMap<String, f32>,
    },
    Social {
        npc_entities: Vec<Entity>,
        dialogue_tree: String,
        reputation_effects: HashMap<String, i32>,
    },
    Environmental {
        challenge_type: String,
        difficulty: u32,
        consequences: Vec<String>,
    },
    Boss {
        boss_name: String,
        phase: u32,
        mechanics: Vec<String>,
        defeat_conditions: Vec<String>,
    },
}

impl EncounterState {
    pub fn new_combat(encounter_id: String, enemy_types: Vec<String>) -> Self {
        Self {
            encounter_id,
            encounter_type: EncounterType::Combat {
                enemy_types,
                terrain_modifiers: HashMap::new(),
            },
            participants: Vec::new(),
            turn_order: Vec::new(),
            current_turn: 0,
            is_boss_fight: false,
            environmental_effects: Vec::new(),
        }
    }
    
    pub fn new_boss(encounter_id: String, boss_name: String) -> Self {
        Self {
            encounter_id,
            encounter_type: EncounterType::Boss {
                boss_name,
                phase: 1,
                mechanics: Vec::new(),
                defeat_conditions: Vec::new(),
            },
            participants: Vec::new(),
            turn_order: Vec::new(),
            current_turn: 0,
            is_boss_fight: true,
            environmental_effects: Vec::new(),
        }
    }
    
    pub fn add_participant(&mut self, entity: Entity) {
        if !self.participants.contains(&entity) {
            self.participants.push(entity);
            self.turn_order.push(entity);
        }
    }
    
    pub fn get_current_actor(&self) -> Option<Entity> {
        if self.current_turn < self.turn_order.len() {
            Some(self.turn_order[self.current_turn])
        } else {
            None
        }
    }
    
    pub fn advance_turn(&mut self) {
        self.current_turn = (self.current_turn + 1) % self.turn_order.len();
    }
}
