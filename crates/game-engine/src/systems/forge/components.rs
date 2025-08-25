//! Sentimental Item & Forge System - ECS Components
//!
//! Production-ready components for Dragon's Labyrinth's unique dual-path morality system:
//! sentimental items become forge reagents for light (essence) vs dark (blood) path choices.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

/// Sentimental item that can become a forge reagent
#[derive(Component, Reflect, Debug, Clone, Serialize, Deserialize)]
#[reflect(Component)]
pub struct SentimentalItem {
    pub item_id: Uuid,
    pub player_id: Uuid,
    pub item_base_id: Uuid,         // References base item data
    
    // Sentimental value system (core Dragon's Labyrinth mechanic)
    pub memory_description: String, // Why this item is meaningful to player
    pub emotional_weight: f32,      // 0.0-1.0 how much this matters emotionally
    pub sentimental_category: String, // "friendship", "love", "loss", "hope", "fear", "triumph"
    pub acquired_story_context: String, // When/where/how player got this item
    pub associated_memories: Vec<SentimentalMemory>, // Specific memories tied to item
    
    // Forge reagent properties
    pub forge_reagent_power: f32,   // Total power when used as reagent
    pub light_path_compatibility: f32, // 0.0-1.0 compatibility with High Elves forge (essence)
    pub dark_path_compatibility: f32,  // 0.0-1.0 compatibility with Cursed forge (blood)
    pub essence_vs_blood_ratio: f32,   // -1.0 to 1.0 (pure essence to pure blood)
    
    // Sacrifice mechanics (unique to Dragon's Labyrinth)
    pub can_be_sacrificed: bool,    // Can be offered to forge
    pub sacrifice_resistance: f32,  // 0.0-1.0 how hard it is to give up
    pub sacrifice_consequences: Vec<SacrificeConsequence>, // What happens if sacrificed
    pub sacrifice_alternatives: Vec<String>, // Other ways to use this item's power
    
    // Memory triggers and emotional resonance
    pub triggers_memory: bool,      // Does this trigger flashbacks/memories?
    pub memory_intensity: f32,      // 0.0-1.0 intensity of triggered memories
    pub emotional_resonance: Vec<EmotionalResonance>, // How this affects player emotionally
    pub companion_reactions: HashMap<Uuid, f32>, // How companions react to this item
    
    // Acquisition tracking
    pub acquired_timestamp: i64,    // When item was acquired
    pub acquisition_circumstances: String, // Circumstances of acquisition
    pub witnesses_to_acquisition: Vec<Uuid>, // Companions who witnessed acquisition
}

/// Specific memory associated with sentimental item
#[derive(Reflect, Debug, Clone, Serialize, Deserialize)]
pub struct SentimentalMemory {
    pub memory_id: String,
    pub memory_type: String,        // "joyful", "traumatic", "bittersweet", "triumphant"
    pub memory_description: String, // Detailed memory description
    pub emotional_intensity: f32,   // 0.0-1.0 emotional intensity
    pub memory_clarity: f32,        // 0.0-1.0 how clear/vivid the memory is
    pub associated_people: Vec<String>, // People involved in memory
    pub location_context: String,   // Where memory took place
    pub sensory_details: Vec<String>, // Sensory aspects of memory
}

/// Emotional resonance created by sentimental item
#[derive(Reflect, Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalResonance {
    pub emotion_type: String,       // "love", "grief", "joy", "regret", "hope"
    pub resonance_strength: f32,    // 0.0-1.0 strength of emotional response
    pub duration: f32,              // How long emotional response lasts (seconds)
    pub triggers: Vec<String>,      // What triggers this emotional response
    pub effects_on_behavior: Vec<String>, // How this emotion affects player behavior
}

/// Consequence of sacrificing a sentimental item
#[derive(Reflect, Debug, Clone, Serialize, Deserialize)]
pub struct SacrificeConsequence {
    pub consequence_type: String,   // "memory_loss", "emotional_numbing", "guilt", "empowerment"
    pub severity: f32,              // 0.0-1.0 severity of consequence
    pub duration: Option<f32>,      // Duration in seconds (None = permanent)
    pub mitigation_options: Vec<String>, // Ways to reduce or undo consequence
    pub character_development_impact: f32, // How this affects character growth
}

/// Current forge trial component (tests ALL game systems)
#[derive(Component, Reflect, Debug, Clone, Serialize, Deserialize)]
#[reflect(Component)]
pub struct ForgeTrial {
    pub trial_id: String,           // Unique trial identifier
    pub trial_type: String,         // "hex_navigation", "mounted_combat", "first_person", "party_coordination"
    pub trial_stage: ForgeTrialStage, // Current stage of trial
    pub progress: f32,              // 0.0-1.0 progress through current stage
    
    // Trial configuration
    pub required_systems: Vec<String>, // Which game systems this trial tests
    pub difficulty_modifiers: HashMap<String, f32>, // Difficulty modifications
    pub success_criteria: Vec<TrialSuccessCriterion>, // What defines success
    pub failure_conditions: Vec<String>, // What causes trial failure
    
    // Forge path requirements
    pub light_path_requirements: Vec<String>, // Requirements for light path attempt
    pub dark_path_requirements: Vec<String>,  // Requirements for dark path attempt
    pub neutral_path_available: bool, // Can trial be completed without path choice?
    
    // Sentimental reagent integration
    pub required_reagent_power: f32, // Minimum reagent power needed
    pub reagent_consumption: Vec<ReagentConsumption>, // How reagents are consumed
    pub reagent_alternatives: Vec<String>, // Alternative ways to power trial
    
    // Companion involvement
    pub companion_roles: HashMap<Uuid, String>, // Role each companion plays
    pub companion_sacrifices_required: Vec<CompanionSacrificeOption>, // Sacrifice options
    pub companion_protection_available: bool, // Can companions be protected?
    
    // Trial rewards and consequences
    pub success_rewards: Vec<TrialReward>, // Rewards for successful completion
    pub failure_consequences: Vec<TrialConsequence>, // Consequences of failure
    pub partial_completion_effects: Vec<String>, // Effects of partial completion
}

/// Stages of forge trial progression
#[derive(Reflect, Debug, Clone, Serialize, Deserialize)]
pub enum ForgeTrialStage {
    Preparation,      // Gathering reagents and preparing
    SystemTest,       // Testing specific game systems
    PathChoice,       // Choosing between light and dark path
    Sacrifice,        // Making required sacrifices
    Forging,          // Actual forging process
    Completion,       // Trial completed successfully
    Failure,          // Trial failed
}

/// Success criterion for forge trial
#[derive(Reflect, Debug, Clone, Serialize, Deserialize)]
pub struct TrialSuccessCriterion {
    pub criterion_type: String,     // Type of success criterion
    pub target_value: f32,          // Target value to achieve
    pub measurement_method: String, // How success is measured
    pub weight: f32,                // 0.0-1.0 weight of this criterion
    pub system_integration: String, // Which system this criterion tests
}

/// How reagents are consumed during trials
#[derive(Reflect, Debug, Clone, Serialize, Deserialize)]
pub struct ReagentConsumption {
    pub reagent_category: String,   // Category of reagent consumed
    pub consumption_rate: f32,      // Rate of consumption per second
    pub consumption_efficiency: f32, // 0.0-1.0 efficiency of consumption
    pub waste_products: Vec<String>, // What's left after consumption
    pub recovery_potential: f32,    // 0.0-1.0 how much can be recovered if trial fails
}

/// Option for sacrificing companion in forge trial
#[derive(Reflect, Debug, Clone, Serialize, Deserialize)]
pub struct CompanionSacrificeOption {
    pub companion_id: Uuid,
    pub sacrifice_type: String,     // "essence" (painless) or "blood" (painful)
    pub power_contribution: f32,    // How much power this sacrifice provides
    pub companion_consent: bool,    // Does companion consent to sacrifice?
    pub resistance_level: f32,      // 0.0-1.0 how much companion resists
    pub alternative_contributions: Vec<String>, // Non-sacrifice ways companion can help
    pub protection_requirements: Vec<String>, // What's needed to protect companion
}

/// Reward for successful forge trial completion
#[derive(Reflect, Debug, Clone, Serialize, Deserialize)]
pub struct TrialReward {
    pub reward_type: String,        // Type of reward
    pub reward_value: f32,          // Value/magnitude of reward
    pub path_specific: Option<String>, // Is this reward specific to a path?
    pub permanent: bool,            // Is this reward permanent?
    pub transferable: bool,         // Can this reward be shared/transferred?
}

/// Consequence of forge trial failure
#[derive(Reflect, Debug, Clone, Serialize, Deserialize)]
pub struct TrialConsequence {
    pub consequence_type: String,   // Type of consequence
    pub severity: f32,              // 0.0-1.0 severity of consequence
    pub affects_companions: bool,   // Does this affect companions too?
    pub reversible: bool,           // Can this consequence be undone?
    pub lesson_learned: Option<String>, // What player learns from failure
}

/// Forge path progression component
#[derive(Component, Reflect, Debug, Clone, Serialize, Deserialize)]
#[reflect(Component)]
pub struct ForgePathProgression {
    pub player_id: Uuid,
    pub chosen_path: Option<ForgePath>, // Current path choice
    pub path_commitment_level: f32, // 0.0-1.0 how committed to chosen path
    pub can_still_switch_paths: bool, // Can player change their mind?
    pub path_switching_cost: f32,   // Cost of switching paths
    
    // Path-specific progress
    pub light_path_progress: f32,   // 0.0-1.0 progress on light path
    pub dark_path_progress: f32,    // 0.0-1.0 progress on dark path
    pub path_mastery_level: f32,    // 0.0-1.0 mastery of chosen path
    
    // Forge master relationship
    pub light_forge_master_approval: f32, // 0.0-1.0 High Elf forge master approval
    pub dark_forge_master_approval: f32,  // 0.0-1.0 Cursed forge master approval
    pub forge_master_tests_passed: Vec<String>, // Tests passed with forge masters
    
    // Ultimate readiness
    pub readiness_for_final_forge: f32, // 0.0-1.0 ready for ultimate forge use
    pub missing_requirements: Vec<String>, // What's still needed for final forge
    pub estimated_time_to_readiness: Option<f32>, // Estimated time to readiness (seconds)
}

/// Forge paths available to player
#[derive(Reflect, Debug, Clone, Serialize, Deserialize)]
pub enum ForgePath {
    Light,      // High Elves forge - essence-based, painless sacrifices
    Dark,       // Cursed forge - blood-based, painful sacrifices
    Balanced,   // Attempting to balance both (very difficult)
}

/// Forge reagent collection component
#[derive(Component, Reflect, Debug, Clone, Serialize, Deserialize)]
#[reflect(Component)]
pub struct ForgeReagentCollection {
    pub player_id: Uuid,
    pub collected_reagents: Vec<CollectedReagent>, // All collected sentimental items
    pub reagent_power_totals: ReagentPowerTotals, // Power available for forging
    pub reagent_categories: HashMap<String, usize>, // Count by category
    pub reagent_quality_distribution: Vec<f32>, // Quality distribution of reagents
    
    // Forge readiness assessment
    pub sufficient_for_light_path: bool, // Enough reagents for light path?
    pub sufficient_for_dark_path: bool,  // Enough reagents for dark path?
    pub optimal_path_recommendation: Option<ForgePath>, // Recommended path based on reagents
    pub suboptimal_path_penalties: HashMap<ForgePath, f32>, // Penalties for non-optimal paths
}

/// Individual collected reagent data
#[derive(Reflect, Debug, Clone, Serialize, Deserialize)]
pub struct CollectedReagent {
    pub sentimental_item_id: Uuid,
    pub reagent_power: f32,         // Total power available
    pub essence_power: f32,         // Power available for light path
    pub blood_power: f32,           // Power available for dark path
    pub collection_circumstances: String, // How/when this was obtained
    pub emotional_attachment_level: f32, // Current emotional attachment
    pub sacrifice_readiness: f32,   // 0.0-1.0 readiness to sacrifice this item
}

/// Power totals available for forging
#[derive(Reflect, Debug, Clone, Serialize, Deserialize)]
pub struct ReagentPowerTotals {
    pub total_essence_power: f32,   // Total light path power
    pub total_blood_power: f32,     // Total dark path power
    pub total_neutral_power: f32,   // Power usable by either path
    pub power_quality_bonus: f32,   // Bonus from high-quality reagents
    pub power_synergy_bonus: f32,   // Bonus from reagent combinations
    pub power_stability: f32,       // 0.0-1.0 how stable the power is
}

/// Second chances system component (unique Dragon's Labyrinth mechanic)
#[derive(Component, Reflect, Debug, Clone, Serialize, Deserialize)]
#[reflect(Component)]
pub struct SecondChancesSystem {
    pub player_id: Uuid,
    pub available_second_chances: u8, // Number of second chances available
    pub max_second_chances: u8,     // Maximum second chances possible
    pub second_chance_sources: Vec<SecondChanceSource>, // Where second chances come from
    
    // Second chance usage tracking
    pub used_second_chances: Vec<UsedSecondChance>, // Record of used second chances
    pub pending_second_chance_opportunities: Vec<SecondChanceOpportunity>, // Available uses
    
    // Forge integration
    pub second_chance_forge_enhancement: f32, // How second chances enhance forge power
    pub forge_grants_second_chances: bool, // Does successful forge use grant more?
    pub ultimate_second_chance_available: bool, // Can player undo final forge choice?
}

/// Source of second chances
#[derive(Reflect, Debug, Clone, Serialize, Deserialize)]
pub struct SecondChanceSource {
    pub source_id: String,          // "companion_sacrifice", "perfect_trial", "forge_master_gift"
    pub source_type: String,        // "earned", "granted", "discovered"
    pub power_level: f32,           // How powerful this second chance is
    pub usage_restrictions: Vec<String>, // When/how this can be used
    pub acquisition_story: String,  // How this second chance was obtained
}

/// Record of a used second chance
#[derive(Reflect, Debug, Clone, Serialize, Deserialize)]
pub struct UsedSecondChance {
    pub usage_timestamp: i64,       // When second chance was used
    pub usage_reason: String,       // Why it was used
    pub situation_resolved: String, // What situation was resolved
    pub effectiveness: f32,         // 0.0-1.0 how effective the second chance was
    pub cost_paid: Vec<String>,     // What cost was paid for using it
    pub lessons_learned: Vec<String>, // What player learned from this use
}

/// Opportunity to use a second chance
#[derive(Reflect, Debug, Clone, Serialize, Deserialize)]
pub struct SecondChanceOpportunity {
    pub opportunity_id: String,
    pub situation_description: String, // What situation can be resolved
    pub urgency_level: f32,         // 0.0-1.0 how urgent this opportunity is
    pub success_probability: f32,   // 0.0-1.0 likelihood of successful resolution
    pub costs_required: Vec<String>, // What using second chance here would cost
    pub alternative_solutions: Vec<String>, // Other ways to resolve situation
    pub expiration_time: Option<i64>, // When opportunity expires
}

/// Mythic gear creation component
#[derive(Component, Reflect, Debug, Clone, Serialize, Deserialize)]
#[reflect(Component)]
pub struct MythicGearCreation {
    pub player_id: Uuid,
    pub gear_being_forged: Option<MythicGearBlueprint>, // Currently being created
    pub completed_mythic_gear: Vec<MythicGearItem>, // Successfully created gear
    pub failed_attempts: Vec<FailedForgeAttempt>, // Failed creation attempts
    
    // Forge process tracking
    pub current_forge_session: Option<ForgeSession>, // Active forging session
    pub forge_session_history: Vec<CompletedForgeSession>, // Past sessions
    pub total_forge_experience: f32, // Experience gained from forging
    
    // Gear enhancement system
    pub gear_evolution_potential: HashMap<Uuid, f32>, // How gear can evolve
    pub gear_synergies: Vec<GearSynergy>, // How gear items work together
    pub ultimate_gear_unlocked: bool, // Has ultimate gear been unlocked?
}

/// Blueprint for mythic gear being forged
#[derive(Reflect, Debug, Clone, Serialize, Deserialize)]
pub struct MythicGearBlueprint {
    pub gear_id: String,
    pub gear_type: String,          // "weapon", "armor", "accessory", "tool"
    pub intended_path: ForgePath,   // Which path this gear is for
    pub base_power_level: f32,      // Base power of the gear
    pub required_reagents: Vec<RequiredReagent>, // Reagents needed for creation
    pub forge_techniques: Vec<String>, // Techniques needed for forging
    pub estimated_completion_time: f32, // Estimated time to complete (seconds)
    pub customization_options: Vec<GearCustomization>, // How gear can be customized
}

/// Required reagent for gear creation
#[derive(Reflect, Debug, Clone, Serialize, Deserialize)]
pub struct RequiredReagent {
    pub reagent_category: String,   // Category of reagent needed
    pub minimum_power: f32,         // Minimum power required
    pub preferred_emotional_weight: f32, // Preferred emotional significance
    pub alternative_reagents: Vec<String>, // Alternative reagents that could work
    pub consumption_method: String, // How this reagent is consumed
}

/// Customization option for gear
#[derive(Reflect, Debug, Clone, Serialize, Deserialize)]
pub struct GearCustomization {
    pub customization_type: String, // Type of customization
    pub options_available: Vec<String>, // Available options
    pub reagent_cost: f32,          // Additional reagent cost
    pub skill_requirement: f32,     // Required forging skill
    pub unique_benefits: Vec<String>, // Unique benefits of this customization
}

/// Completed mythic gear item
#[derive(Reflect, Debug, Clone, Serialize, Deserialize)]
pub struct MythicGearItem {
    pub gear_id: Uuid,
    pub gear_name: String,          // Name of the gear
    pub gear_type: String,
    pub path_alignment: ForgePath,  // Which path this gear represents
    pub power_level: f32,           // Current power level
    pub enhancement_level: u8,      // Current enhancement level
    
    // Sentimental resonance (unique to Dragon's Labyrinth)
    pub sentimental_resonance: Vec<SentimentalResonance>, // Memories embedded in gear
    pub emotional_amplification: f32, // How gear amplifies emotions
    pub memory_triggers: Vec<String>, // What memories this gear can trigger
    
    // Mechanical properties
    pub gear_abilities: Vec<GearAbility>, // Special abilities of the gear
    pub stat_bonuses: HashMap<String, f32>, // Stat bonuses provided
    pub conditional_effects: Vec<ConditionalEffect>, // Situational effects
    
    // Evolution and growth
    pub evolution_potential: f32,   // 0.0-1.0 potential for further growth
    pub growth_triggers: Vec<String>, // What triggers gear evolution
    pub maximum_power_potential: f32, // Theoretical maximum power
}

/// Sentimental resonance embedded in mythic gear
#[derive(Reflect, Debug, Clone, Serialize, Deserialize)]
pub struct SentimentalResonance {
    pub source_memory: String,      // Memory this resonance comes from
    pub resonance_type: String,     // Type of emotional resonance
    pub power_contribution: f32,    // How much power this resonance provides
    pub activation_triggers: Vec<String>, // What activates this resonance
    pub emotional_effects: Vec<String>, // Emotional effects when activated
}

/// Special ability of mythic gear
#[derive(Reflect, Debug, Clone, Serialize, Deserialize)]
pub struct GearAbility {
    pub ability_name: String,
    pub ability_type: String,       // "combat", "social", "environmental", "psychological"
    pub power_level: f32,           // Strength of the ability
    pub cooldown: f32,              // Cooldown in seconds
    pub resource_cost: Option<f32>, // Resource cost to activate
    pub activation_conditions: Vec<String>, // When this ability can be used
    pub sentimental_requirement: Option<String>, // Emotional state required
}

/// Conditional effect that activates under specific circumstances
#[derive(Reflect, Debug, Clone, Serialize, Deserialize)]
pub struct ConditionalEffect {
    pub effect_name: String,
    pub trigger_conditions: Vec<String>, // What triggers this effect
    pub effect_power: f32,          // Strength of the effect
    pub duration: Option<f32>,      // Duration of effect (None = permanent)
    pub stacking_behavior: String,  // How multiple instances stack
}

/// Failed forge attempt record
#[derive(Reflect, Debug, Clone, Serialize, Deserialize)]
pub struct FailedForgeAttempt {
    pub attempt_timestamp: i64,     // When attempt was made
    pub failure_reason: String,     // Why the attempt failed
    pub reagents_lost: Vec<Uuid>,   // Reagents lost in failure
    pub lessons_learned: Vec<String>, // What was learned from failure
    pub retry_modifications: Vec<String>, // Suggested modifications for retry
    pub emotional_impact: f32,      // Emotional impact of failure on player
}

/// Active forge session
#[derive(Reflect, Debug, Clone, Serialize, Deserialize)]
pub struct ForgeSession {
    pub session_id: Uuid,
    pub start_time: i64,            // When session started
    pub target_gear: MythicGearBlueprint, // What's being forged
    pub active_reagents: Vec<Uuid>, // Reagents currently being used
    pub session_progress: f32,      // 0.0-1.0 progress through session
    pub current_technique: String, // Current forging technique being used
    pub complications: Vec<ForgeComplication>, // Problems encountered during session
    pub participant_companions: Vec<Uuid>, // Companions helping with forge
}

/// Complication during forge session
#[derive(Reflect, Debug, Clone, Serialize, Deserialize)]
pub struct ForgeComplication {
    pub complication_type: String,  // Type of complication
    pub severity: f32,              // 0.0-1.0 severity
    pub resolution_options: Vec<String>, // Ways to resolve complication
    pub impact_on_success: f32,     // How this affects success probability
    pub learning_opportunity: bool, // Does resolving this teach something?
}

/// Completed forge session record
#[derive(Reflect, Debug, Clone, Serialize, Deserialize)]
pub struct CompletedForgeSession {
    pub session_id: Uuid,
    pub completion_time: i64,       // When session completed
    pub success: bool,              // Was session successful?
    pub gear_created: Option<Uuid>, // Gear created (if successful)
    pub reagents_consumed: Vec<Uuid>, // Reagents that were consumed
    pub experience_gained: f32,     // Forging experience gained
    pub insights_discovered: Vec<String>, // Insights gained during session
    pub participant_growth: HashMap<Uuid, f32>, // How participants grew
}

/// Synergy between multiple mythic gear items
#[derive(Reflect, Debug, Clone, Serialize, Deserialize)]
pub struct GearSynergy {
    pub synergy_name: String,
    pub gear_items: Vec<Uuid>,      // Which gear items create this synergy
    pub synergy_power: f32,         // Power of the synergy effect
    pub synergy_effects: Vec<String>, // Effects of the synergy
    pub activation_requirements: Vec<String>, // What's needed to activate synergy
    pub path_alignment_bonus: f32,  // Bonus if all gear aligns with same path
}
