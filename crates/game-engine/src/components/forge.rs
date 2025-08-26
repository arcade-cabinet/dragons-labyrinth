//! Forge system components for sophisticated dual-path morality system

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Sentimental items collected throughout game as forge reagents
#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub struct SentimentalItem {
    pub id: Uuid,
    pub player_entity: Entity,
    pub item_entity: Entity,
    
    // Sentimental value system
    pub memory_description: String,       // Why this item is meaningful
    pub emotional_weight: f32,           // 0.0-1.0 how much this matters to player
    pub sentimental_category: SentimentalCategory,
    pub acquired_story_context: String, // When/where player got this item
    
    // Forge integration
    pub forge_reagent_power: f32,       // Power when used as reagent
    pub light_path_compatibility: f32,  // 0.0-1.0 compatibility with High Elves forge
    pub dark_path_compatibility: f32,   // 0.0-1.0 compatibility with Cursed forge
    pub essence_vs_blood_ratio: f32,    // -1.0 to 1.0 (essence to blood spectrum)
    
    // Memory integration
    pub triggers_memory: bool,          // Does this trigger flashbacks?
    pub memory_fragments: Vec<MemoryFragment>,
    pub emotional_resonance: EmotionalResonance,
    
    // Sacrifice mechanics
    pub can_be_sacrificed: bool,        // Can be offered to forge
    pub sacrifice_resistance: f32,      // How hard it is to give up
    pub sacrifice_consequences: Vec<SacrificeConsequence>,
    pub sacrifice_alternatives: Vec<SacrificeAlternative>,
    
    pub acquired_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub enum SentimentalCategory {
    Friendship,
    Love,
    Loss,
    Hope,
    Fear,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub struct MemoryFragment {
    pub description: String,
    pub emotional_intensity: f32,
    pub associated_location: Option<String>,
    pub associated_companions: Vec<String>,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub struct EmotionalResonance {
    pub primary_emotion: String,
    pub intensity_modifier: f32,
    pub duration_days: u32,
    pub affects_companions: bool,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub struct SacrificeConsequence {
    pub consequence_type: String,
    pub description: String,
    pub permanence: bool,
    pub affects_companions: bool,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub struct SacrificeAlternative {
    pub alternative_type: String,
    pub description: String,
    pub requirements: Vec<String>,
    pub outcomes: Vec<String>,
}

/// Forge progression tracking for ultimate endgame trials
#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub struct ForgeProgress {
    pub id: Uuid,
    pub player_entity: Entity,
        
    // Forge path choice
    pub chosen_forge_path: Option<ForgePath>,
    pub path_commitment_level: f32,        // 0.0-1.0 how committed to path
    pub can_still_switch_paths: bool,     // Can player change their mind?
    
    // Trial progression (tests ALL game systems)
    pub trials_completed: Vec<TrialType>,
    pub trial_scores: HashMap<TrialType, f32>,
    pub current_trial: Option<TrialType>,
    pub trial_failures: Vec<TrialFailure>,
    
    // Trial categories
    pub hex_navigation_trials: Vec<HexNavigationTrial>,
    pub mounted_combat_trials: Vec<MountedCombatTrial>,
    pub first_person_trials: Vec<FirstPersonTrial>,
    pub party_coordination_trials: Vec<PartyCoordinationTrial>,
    
    // Sentimental reagent collection
    pub reagents_collected: Vec<Entity>,  // References to SentimentalItem entities
    pub reagents_used: Vec<Entity>,       // Items already consumed
    pub reagent_essence_power: f32,       // Total essence power available
    pub reagent_blood_power: f32,         // Total blood power available
    
    // Companion sacrifice system
    pub companions_offered: Vec<Entity>,  // Companion entities offered for sacrifice
    pub sacrifice_method: Option<SacrificeMethod>,
    pub sacrifice_resistance_overcome: Vec<ResistanceMethod>,
    pub mythic_gear_earned: Vec<MythicGear>,
    
    // Forge readiness assessment
    pub readiness_score: f32,             // 0.0-1.0 ready for final forge use
    pub missing_requirements: Vec<ForgeRequirement>,
    pub forge_master_approval: bool,     // Has forge master approved player?
    
    // Ultimate choices
    pub final_forge_decision: Option<FinalForgeDecision>,
    pub gear_blessing_type: Option<GearBlessingType>,
    pub forge_completion_timestamp: Option<DateTime<Utc>>,
    
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub enum ForgePath {
    Light,  // High Elves forge
    Dark,   // Cursed forge
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize, Hash, Eq)]
#[reflect(Component)]
pub enum TrialType {
    HexNavigation,
    MountedCombat,
    FirstPerson,
    PartyCoordination,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub struct TrialFailure {
    pub trial_type: TrialType,
    pub attempt_number: u32,
    pub failure_reason: String,
    pub lessons_learned: Vec<String>,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub struct HexNavigationTrial {
    pub trial_name: String,
    pub complexity_level: u32,
    pub required_moves: u32,
    pub time_limit_turns: Option<u32>,
    pub obstacles: Vec<String>,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub struct MountedCombatTrial {
    pub trial_name: String,
    pub mount_type_required: String,
    pub enemy_types: Vec<String>,
    pub terrain_challenges: Vec<String>,
    pub coordination_requirements: Vec<String>,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub struct FirstPersonTrial {
    pub trial_name: String,
    pub perspective_requirements: Vec<String>,
    pub precision_targets: Vec<String>,
    pub environmental_hazards: Vec<String>,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub struct PartyCoordinationTrial {
    pub trial_name: String,
    pub required_companions: u32,
    pub coordination_tasks: Vec<String>,
    pub communication_challenges: Vec<String>,
    pub failure_consequences: Vec<String>,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub enum SacrificeMethod {
    Essence,  // Painless
    Blood,    // Painful
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub struct ResistanceMethod {
    pub method_type: String,
    pub description: String,
    pub moral_weight: f32,
    pub effectiveness: f32,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub struct MythicGear {
    pub gear_name: String,
    pub blessing_type: GearBlessingType,
    pub power_level: f32,
    pub special_abilities: Vec<String>,
    pub lore_description: String,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub enum GearBlessingType {
    LightBlessing,
    DarkCurse,
    BalancedPower,
    LegendaryFailure,  // Special gear from forge failures
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub struct ForgeRequirement {
    pub requirement_type: String,
    pub description: String,
    pub current_progress: f32,
    pub required_threshold: f32,
    pub can_be_bypassed: bool,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub enum FinalForgeDecision {
    CreateProtectiveGear,
    CreatePowerfulWeapon,
    BlessCompanions,
    TransformSelf,
    RejectForge,
}

/// Forge location component for identifying forge entities
#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub struct ForgeLocation {
    pub forge_type: ForgePath,
    pub power_level: f32,
    pub is_active: bool,
    pub forge_master_present: bool,
    pub required_offerings: Vec<String>,
}

/// Event fired when player interacts with forge
#[derive(Event, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Event)]
pub struct ForgeInteractionEvent {
    pub player_entity: Entity,
    pub forge_entity: Entity,
    pub interaction_type: ForgeInteractionType,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub enum ForgeInteractionType {
    Examine,
    OfferReagent(Entity),
    OfferCompanion(Entity),
    CommitToPath(ForgePath),
    BeginTrial(TrialType),
    FinalForging,
}
