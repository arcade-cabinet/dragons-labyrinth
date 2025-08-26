//! NPC components for non-player characters in settlements and encounters

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// NPCs - Non-player characters that can be found throughout the world
#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub struct NPC {
    pub id: Uuid,
    
    // Basic NPC data
    pub name: String, // "Gareth the Innkeeper", "Old Woman Martha"
    pub role: NPCRole,
    pub race: NPCRace,
    
    // Location data
    pub hex_tile_entity: Option<Entity>, // What hex tile they're on
    pub settlement_entity: Option<Entity>, // What settlement they belong to
    pub dungeon_entity: Option<Entity>, // What dungeon they're in (if any)
    
    // HBF import data
    pub hbf_uuid: Option<String>, // Original HBF UUID if imported
    pub hbf_x: Option<i32>, // Original HBF coordinates
    pub hbf_y: Option<i32>,
    
    // Character description
    pub description: String, // Physical description and mannerisms
    pub personality: Option<String>, // Personality traits and quirks
    pub background: Option<String>, // Personal history and background
    
    // Stats and abilities (if relevant for encounters)
    pub level: Option<i32>, // Character level (if combatant)
    pub hit_points: Option<i32>, // HP for combat encounters
    pub armor_class: Option<i32>, // AC for combat
    pub ability_scores: Option<AbilityScores>,
    pub equipment: Vec<Entity>, // Equipment entity references
    
    // Social interactions
    pub disposition: i32, // -10 to +10 how they feel about the player
    pub reputation_awareness: i32, // 0-10 how much they know about player's reputation
    pub dialogue_options: Vec<DialogueOption>,
    pub rumors_known: Vec<Rumor>,
    pub services_offered: Vec<NPCService>,
    
    // Economic data
    pub trade_goods: Vec<TradeGood>,
    pub price_modifiers: HashMap<String, f32>,
    pub wealth_level: i32, // 0-10 how wealthy they are
    
    // Relationships
    pub relationships: Vec<NPCRelationship>,
    pub faction: Option<String>, // What faction they belong to
    
    // Behavior and AI
    pub behavior_type: NPCBehaviorType,
    pub daily_schedule: Vec<ScheduleEntry>,
    pub interaction_triggers: Vec<InteractionTrigger>,
    
    // Horror progression integration
    pub corruption_susceptibility: f32, // 0.0-1.0 how easily they're corrupted
    pub current_corruption_level: f32, // 0.0-1.0 current corruption
    pub dread_level_effects: i32, // How they change with world dread 0-4
    pub corrupted_description: Option<String>, // How they appear when corrupted
    pub corruption_triggers: Vec<CorruptionTrigger>,
    
    // Companion interactions
    pub companion_reactions: HashMap<Entity, CompanionReaction>,
    pub companion_memories: Vec<CompanionMemory>,
    
    // Quest and story integration
    pub quest_connections: Vec<QuestConnection>,
    pub story_importance: StoryImportance,
    
    // State tracking
    pub alive: bool, // Is this NPC still alive?
    pub encountered: bool, // Has player met this NPC?
    pub first_met_at: Option<DateTime<Utc>>,
    pub last_interaction_at: Option<DateTime<Utc>>,
    pub times_interacted: i32, // How many times player has talked to them
    
    // Movement and location
    pub mobile: bool, // Does this NPC move around?
    pub movement_pattern: Option<MovementPattern>,
    
    // Timestamps
    pub created_at: DateTime<Utc>,
    pub last_modified: DateTime<Utc>,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub enum NPCRole {
    Innkeeper,
    Merchant,
    Guard,
    Sage,
    Traveler,
    Hermit,
    Blacksmith,
    Healer,
    Noble,
    Farmer,
    Hunter,
    Priest,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub enum NPCRace {
    Human,
    Elf,
    Dwarf,
    Halfling,
    Other(String),
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub struct AbilityScores {
    pub strength: i32,
    pub dexterity: i32,
    pub constitution: i32,
    pub intelligence: i32,
    pub wisdom: i32,
    pub charisma: i32,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub struct DialogueOption {
    pub topic: String,
    pub text: String,
    pub required_reputation: i32,
    pub unlocks_quest: Option<String>,
    pub provides_rumor: Option<String>,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub struct Rumor {
    pub rumor_id: String,
    pub content: String,
    pub reliability: f32,     // 0.0-1.0 how accurate this rumor is
    pub spreading_rate: f32,  // 0.0-1.0 how quickly this spreads
    pub consequences: Vec<String>,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub struct NPCService {
    pub service_type: ServiceType,
    pub description: String,
    pub base_cost: f32,
    pub availability: ServiceAvailability,
    pub quality_level: f32,   // 0.0-1.0 how good they are at this
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub enum ServiceType {
    Healing,
    Trading,
    Information,
    Accommodation,
    Crafting,
    Guidance,
    Entertainment,
    Protection,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub enum ServiceAvailability {
    Always,
    DayOnly,
    NightOnly,
    Seasonal,
    QuestDependent,
    ReputationGated,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub struct TradeGood {
    pub item_type: String,
    pub buy_price: Option<f32>,
    pub sell_price: Option<f32>,
    pub stock_level: i32,
    pub max_stock: i32,
    pub restock_rate: f32,    // How quickly they restock
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub struct NPCRelationship {
    pub related_entity: Entity,  // Other NPC or faction entity
    pub relationship_type: RelationshipType,
    pub strength: f32,        // -1.0 to 1.0 (enemy to ally)
    pub history: Vec<String>,
    pub affects_player_interaction: bool,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub enum RelationshipType {
    Family,
    Friend,
    Enemy,
    Rival,
    Business,
    Romantic,
    Mentor,
    Student,
    Neutral,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub enum NPCBehaviorType {
    Friendly,
    Neutral,
    Hostile,
    Trader,
    Guard,
    Wanderer,
    Hermit,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub struct ScheduleEntry {
    pub time_of_day: u32,     // 0-49 (50 turns per day)
    pub location_description: String,
    pub activity: String,
    pub availability_for_interaction: bool,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub struct InteractionTrigger {
    pub trigger_condition: String,
    pub trigger_response: String,
    pub one_time_only: bool,
    pub requires_items: Vec<String>,
    pub unlocks_content: Vec<String>,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub struct CorruptionTrigger {
    pub trigger_source: String,
    pub corruption_amount: f32,
    pub manifestation: String,
    pub reversible: bool,
    pub affects_behavior: bool,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub struct CompanionReaction {
    pub reaction_type: String,
    pub intensity: f32,       // 0.0-1.0 how strong the reaction
    pub affects_relationship: bool,
    pub dialogue_changes: Vec<String>,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub struct CompanionMemory {
    pub memory_description: String,
    pub emotional_weight: f32,
    pub affects_current_behavior: bool,
    pub related_companion_entity: Entity,
    pub memory_type: CompanionMemoryType,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub enum CompanionMemoryType {
    Positive,
    Negative,
    Traumatic,
    Healing,
    Neutral,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub struct QuestConnection {
    pub quest_id: String,
    pub connection_type: QuestConnectionType,
    pub importance_level: f32,
    pub affects_outcome: bool,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub enum QuestConnectionType {
    QuestGiver,
    QuestTarget,
    QuestHelper,
    QuestObstacle,
    QuestWitness,
    QuestReward,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub struct StoryImportance {
    pub importance_level: f32,  // 0.0-1.0 how important to main story
    pub story_beats: Vec<String>,
    pub narrative_function: String,
    pub can_be_killed: bool,
    pub replacement_if_killed: Option<String>,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub struct MovementPattern {
    pub pattern_type: MovementPatternType,
    pub movement_radius: f32,
    pub movement_frequency: f32,  // How often they move
    pub preferred_locations: Vec<String>,
    pub avoids_locations: Vec<String>,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub enum MovementPatternType {
    Stationary,
    Patrol,
    Random,
    Scheduled,
    Following,
    Fleeing,
}

/// Event fired when player interacts with NPC
#[derive(Event, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Event)]
pub struct NPCInteractionEvent {
    pub player_entity: Entity,
    pub npc_entity: Entity,
    pub interaction_type: NPCInteractionType,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub enum NPCInteractionType {
    Dialogue,
    Trade,
    Service,
    Combat,
    Quest,
    Information,
}
