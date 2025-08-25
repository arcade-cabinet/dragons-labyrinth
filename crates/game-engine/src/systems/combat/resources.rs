//! Combat Resources - ECS resources for combat state management

use bevy::prelude::*;
use std::collections::HashMap;
use uuid::Uuid;
use super::components::*;
use super::events::*;

/// Main combat state resource
#[derive(Resource, Debug, Clone)]
pub struct CombatState {
    pub phase: CombatPhase,
    pub current_turn: u32,
    pub turn_order: Vec<Entity>,
    pub player_entity: Option<Entity>,
    pub creature_entities: Vec<Entity>,
    pub environment: Option<CombatEnvironment>,
    pub player_hp_percentage: Option<f32>,
}

impl Default for CombatState {
    fn default() -> Self {
        Self {
            phase: CombatPhase::None,
            current_turn: 0,
            turn_order: Vec::new(),
            player_entity: None,
            creature_entities: Vec::new(),
            environment: None,
            player_hp_percentage: None,
        }
    }
}

/// Combat environment data
#[derive(Debug, Clone)]
pub struct CombatEnvironment {
    pub terrain: String,
    pub weather_effects: Option<WeatherEffects>,
    pub corruption_level: f32,
    pub lighting: LightingCondition,
    pub hazards: Vec<EnvironmentalHazard>,
}

#[derive(Debug, Clone)]
pub struct WeatherEffects {
    pub visibility_modifier: f32,
    pub movement_modifier: f32,
    pub combat_effects: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum LightingCondition {
    Bright,
    Normal,
    Dim,
    Dark,
    Darkness,
}

#[derive(Debug, Clone)]
pub struct EnvironmentalHazard {
    pub hazard_type: String,
    pub area: Vec<CombatPosition>,
    pub damage_per_turn: Option<i32>,
    pub save_dc: Option<i32>,
}

/// Creature template cache for spawning
#[derive(Resource, Debug, Clone, Default)]
pub struct CreatureTemplateCache {
    pub templates: HashMap<String, CreatureTemplate>,
}

#[derive(Debug, Clone)]
pub struct CreatureTemplate {
    pub name: String,
    pub creature_type: String,
    pub challenge_rating: String,
    pub armor_class: i32,
    pub hit_points_formula: String,
    pub abilities: CreatureAbilityScores,
    pub actions: Vec<CombatAction>,
    pub special_abilities: Vec<String>,
    pub damage_resistances: Vec<DamageType>,
    pub damage_immunities: Vec<DamageType>,
    pub condition_immunities: Vec<StatusEffectType>,
    pub senses: Vec<String>,
    pub languages: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct CreatureAbilityScores {
    pub strength: i32,
    pub dexterity: i32,
    pub constitution: i32,
    pub intelligence: i32,
    pub wisdom: i32,
    pub charisma: i32,
}

/// Encounter tables for random encounters
#[derive(Resource, Debug, Clone, Default)]
pub struct EncounterTables {
    pub tables: HashMap<String, BiomeEncounterTable>, // biome -> encounter table
}

#[derive(Debug, Clone)]
pub struct BiomeEncounterTable {
    pub biome: String,
    pub base_probability: f32,
    pub encounters: Vec<EncounterEntry>,
}

#[derive(Debug, Clone)]
pub struct EncounterEntry {
    pub roll_range: (u32, u32), // min, max roll
    pub creature_names: Vec<String>,
    pub quantity_formula: String, // "1", "1d4", "2d6", etc.
    pub special_conditions: Vec<String>,
}

/// Combat settings and configuration
#[derive(Resource, Debug, Clone)]
pub struct CombatSettings {
    pub initiative_advantage: bool, // Player gets advantage on initiative
    pub death_saves_enabled: bool,
    pub flanking_rules: bool,
    pub opportunity_attacks: bool,
    pub horror_escalation: bool, // Horror effects increase over time
    pub companion_morale_effects: bool,
    pub environmental_effects: bool,
    pub critical_hit_multiplier: f32,
}

impl Default for CombatSettings {
    fn default() -> Self {
        Self {
            initiative_advantage: false,
            death_saves_enabled: true,
            flanking_rules: true,
            opportunity_attacks: true,
            horror_escalation: true,
            companion_morale_effects: true,
            environmental_effects: true,
            critical_hit_multiplier: 2.0,
        }
    }
}

/// Active turn tracker
#[derive(Resource, Debug, Clone, Default)]
pub struct ActiveTurn {
    pub current_entity: Option<Entity>,
    pub actions_remaining: u32,
    pub movement_remaining: u32,
    pub turn_timer: f32, // Time limit for turns
}

/// Combat statistics tracking
#[derive(Resource, Debug, Clone, Default)]
pub struct CombatStatistics {
    pub total_combats: u32,
    pub player_victories: u32,
    pub player_defeats: u32,
    pub average_combat_length: f32,
    pub total_damage_dealt: u32,
    pub total_damage_taken: u32,
    pub creatures_defeated: HashMap<String, u32>, // creature_type -> count
    pub horror_events_triggered: u32,
}

impl CombatStatistics {
    pub fn record_combat_end(&mut self, result: &CombatResult, rounds: u32) {
        self.total_combats += 1;
        match result {
            CombatResult::Victory => self.player_victories += 1,
            CombatResult::Defeat => self.player_defeats += 1,
            CombatResult::Flee => {}
        }
        
        // Update average combat length
        self.average_combat_length = (self.average_combat_length * (self.total_combats - 1) as f32 + rounds as f32) / self.total_combats as f32;
    }
    
    pub fn record_creature_defeat(&mut self, creature_type: &str) {
        *self.creatures_defeated.entry(creature_type.to_string()).or_insert(0) += 1;
    }
    
    pub fn record_damage(&mut self, damage: u32, is_player_damage: bool) {
        if is_player_damage {
            self.total_damage_dealt += damage;
        } else {
            self.total_damage_taken += damage;
        }
    }
    
    pub fn win_rate(&self) -> f32 {
        if self.total_combats == 0 {
            0.0
        } else {
            self.player_victories as f32 / self.total_combats as f32
        }
    }
}

/// Tactical grid for positioning
#[derive(Resource, Debug, Clone)]
pub struct TacticalGrid {
    pub grid_size: (u32, u32),
    pub terrain_features: HashMap<CombatPosition, TerrainFeature>,
    pub occupied_positions: HashMap<CombatPosition, Entity>,
    pub cover_map: HashMap<CombatPosition, CoverType>,
}

impl Default for TacticalGrid {
    fn default() -> Self {
        Self {
            grid_size: (20, 20),
            terrain_features: HashMap::new(),
            occupied_positions: HashMap::new(),
            cover_map: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum CoverType {
    None,
    Half,    // +2 AC
    ThreeQuarters, // +5 AC
    Total,   // Cannot be targeted
}

impl TacticalGrid {
    pub fn is_position_occupied(&self, position: &CombatPosition) -> bool {
        self.occupied_positions.contains_key(position)
    }
    
    pub fn get_cover_bonus(&self, position: &CombatPosition) -> i32 {
        match self.cover_map.get(position) {
            Some(CoverType::Half) => 2,
            Some(CoverType::ThreeQuarters) => 5,
            Some(CoverType::Total) => 1000, // Effectively untargetable
            _ => 0,
        }
    }
    
    pub fn update_position(&mut self, old_pos: CombatPosition, new_pos: CombatPosition, entity: Entity) {
        self.occupied_positions.remove(&old_pos);
        self.occupied_positions.insert(new_pos, entity);
    }
    
    pub fn add_terrain_feature(&mut self, position: CombatPosition, feature: TerrainFeature) {
        // Update cover based on terrain feature
        let cover = match feature.feature_type.as_str() {
            "wall" | "pillar" => CoverType::Total,
            "boulder" | "tree" => CoverType::ThreeQuarters,
            "bush" | "debris" => CoverType::Half,
            _ => CoverType::None,
        };
        
        self.terrain_features.insert(position.clone(), feature);
        self.cover_map.insert(position, cover);
    }
}

#[derive(Debug, Clone)]
pub struct TerrainFeature {
    pub feature_type: String,
    pub blocks_movement: bool,
    pub provides_cover: bool,
    pub movement_cost: f32,
}

/// Combat phases for state management
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CombatPhase {
    None,
    Initiative,
    PlayerTurn,
    CreatureTurn,
    Resolution,
    Cleanup,
}
