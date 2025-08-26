//! Encounter components for sophisticated narrative interactions

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Encounters with NPCs, environments, and narrative situations
#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub struct Encounter {
    pub id: Uuid,
    pub hex_tile_entity: Option<Entity>, // Where this encounter occurs
    pub player_entity: Option<Entity>, // Which player this encounter is for (instance data)
    
    // Encounter metadata
    pub encounter_type: EncounterType,
    pub encounter_name: String,
    pub description: String,
    
    // Narrative requirements
    pub required_dread_level_min: Option<i32>, // Min dread level to trigger
    pub required_dread_level_max: Option<i32>, // Max dread level to trigger
    pub required_items: Vec<Entity>, // Required item entities
    pub required_companion_states: Vec<CompanionRequirement>,
    pub required_story_flags: Vec<String>,
    
    // Encounter state and availability
    pub is_available: bool, // Can this encounter be triggered?
    pub is_repeatable: bool, // Can encounter happen multiple times?
    pub times_encountered: i32, // How many times player has had this encounter
    pub first_encountered_at: Option<DateTime<Utc>>, // When first encountered
    pub last_encountered_at: Option<DateTime<Utc>>, // Most recent encounter
    
    // Horror progression integration
    pub dread_level_impact: f32, // -1.0 to 1.0 change to dread level
    pub corruption_impact: f32, // -1.0 to 1.0 change to corruption
    pub companion_trauma_impact: f32, // Impact on companion psychological states
    
    // Dialogue and narrative content
    pub dialogue_tree_id: Option<String>, // Reference to dialogue system
    pub dialogue_options: Vec<EncounterDialogueOption>,
    pub narrative_outcomes: Vec<NarrativeOutcome>,
    
    // Rewards and consequences
    pub item_rewards: Vec<ItemReward>,
    pub story_flag_changes: HashMap<String, bool>,
    pub companion_relationship_changes: HashMap<Entity, f32>,
    
    // Environmental storytelling
    pub atmospheric_description: Option<String>, // Rich environmental description
    pub horror_description: Option<String>, // Description when horror-influenced
    
    // Asset references for rich presentation
    pub background_image_id: Option<String>, // Visual background for encounter
    pub character_portrait_id: Option<String>, // NPC portrait if applicable
    pub ambient_audio_id: Option<String>, // Background audio
    pub music_track_id: Option<String>, // Music for this encounter
    
    // Forge system integration
    pub light_essence_gain: f32, // Light essence gained from this encounter  
    pub dark_essence_gain: f32, // Dark essence gained from this encounter
    pub sentimental_item_chance: f32, // 0.0-1.0 chance to create sentimental item
    
    // Timestamps
    pub created_at: DateTime<Utc>,
    pub last_modified: DateTime<Utc>,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub enum EncounterType {
    NPCDialogue,
    Environmental,
    Combat,
    Discovery,
    HorrorEvent,
    SentimentalMoment,
    ForgeOpportunity,
    TherapyMoment,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub struct CompanionRequirement {
    pub companion_entity: Entity,
    pub required_state: CompanionStateRequirement,
    pub required_relationship_level: Option<f32>,
    pub required_therapy_progress: Option<f32>,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub enum CompanionStateRequirement {
    Present,
    Absent,
    Healthy,
    Traumatized,
    Recovering,
    Healed,
    Specific(String),
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub struct EncounterDialogueOption {
    pub option_text: String,
    pub required_reputation: Option<i32>,
    pub required_items: Vec<Entity>,
    pub leads_to_outcome: String,
    pub companion_reactions: HashMap<Entity, String>,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub struct NarrativeOutcome {
    pub outcome_id: String,
    pub outcome_description: String,
    pub probability: f32,     // 0.0-1.0 chance of this outcome
    pub requirements_met: bool,
    pub consequences: Vec<EncounterConsequence>,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub struct EncounterConsequence {
    pub consequence_type: ConsequenceType,
    pub description: String,
    pub immediate_effect: bool,
    pub delayed_effect_turns: Option<u32>,
    pub affects_world_state: bool,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub enum ConsequenceType {
    DreadLevelChange,
    CorruptionChange,
    CompanionTrauma,
    CompanionHealing,
    ItemGained,
    ItemLost,
    StoryProgression,
    SentimentalItemCreated,
    RelationshipChange,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub struct ItemReward {
    pub item_entity: Entity,
    pub quantity: u32,
    pub condition: ItemCondition,
    pub special_properties: Vec<String>,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub enum ItemCondition {
    Perfect,
    Good,
    Worn,
    Damaged,
    Magical,
    Cursed,
    Blessed,
}

/// Event fired when encounter is triggered
#[derive(Event, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Event)]
pub struct EncounterTriggerEvent {
    pub encounter_entity: Entity,
    pub player_entity: Entity,
    pub trigger_reason: EncounterTriggerReason,
    pub participating_companions: Vec<Entity>,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub enum EncounterTriggerReason {
    PlayerMovement,
    TimeProgression,
    StoryProgression,
    DreadLevelChange,
    CompanionAction,
    EnvironmentalEvent,
    RandomChance,
}

/// Event fired when encounter concludes
#[derive(Event, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Event)]
pub struct EncounterCompleteEvent {
    pub encounter_entity: Entity,
    pub player_entity: Entity,
    pub chosen_outcome: String,
    pub consequences: Vec<EncounterConsequence>,
    pub sentimental_items_created: Vec<Entity>,
}

/// Component marking entities as encounter locations
#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub struct EncounterLocation {
    pub available_encounters: Vec<Entity>,
    pub encounter_frequency: f32,     // 0.0-1.0 how often encounters happen here
    pub last_encounter_time: Option<DateTime<Utc>>,
    pub cooldown_period: Option<u32>, // Turns before new encounter possible
}
