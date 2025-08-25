//! Dread Progression System - ECS Events
//!
//! Production-ready events for the master horror orchestrator system that transforms
//! ALL Dragon's Labyrinth systems based on dread level progression (0-4).

use bevy::prelude::*;
use uuid::Uuid;
use crate::systems::dread_progression::components::*;
use crate::systems::dread_progression::resources::*;
use std::collections::HashMap;

/// Core event for dread level changes - triggers system transformations
#[derive(Event, Debug, Clone)]
pub struct DreadLevelChangeEvent {
    pub old_level: u8,            // Previous dread level (0-4)
    pub new_level: u8,            // New dread level (0-4)
    pub trigger_source: String,   // What caused this change
    pub affected_regions: Vec<String>, // Which regions are affected
    pub transition_speed: f32,    // How fast the transition should occur
    pub forced_transition: bool,  // Is this change forced (can't be resisted)?
    pub player_adaptation_override: bool, // Should player adaptation be ignored?
    pub system_priority_overrides: HashMap<String, u8>, // Override system priorities
}

/// Event for adding new dread sources to the world
#[derive(Event, Debug, Clone)]
pub struct DreadSourceEvent {
    pub action: DreadSourceAction,
    pub source: DreadSource,
    pub spatial_position: Option<Vec3>, // Position in world (for spatial dread sources)
    pub affected_entities: Vec<Entity>, // Entities directly affected
    pub propagation_rules: Vec<DreadPropagationRule>, // How this source spreads
}

/// Actions that can be performed on dread sources
#[derive(Debug, Clone)]
pub enum DreadSourceAction {
    Add,                          // Add new dread source
    Remove(String),               // Remove dread source by ID
    Modify(String),               // Modify existing dread source
    Amplify(String, f32),         // Amplify existing source by multiplier
    Decay(String, f32),           // Decay source by amount per second
    Merge(Vec<String>),           // Merge multiple sources into one
    Split(String, Vec<DreadSource>), // Split source into multiple sources
}

/// Rules for how dread propagates through the world
#[derive(Debug, Clone)]
pub struct DreadPropagationRule {
    pub propagation_type: String, // "spatial", "narrative", "social", "temporal"
    pub propagation_speed: f32,   // How fast dread spreads
    pub propagation_decay: f32,   // How much intensity is lost during spread
    pub propagation_barriers: Vec<String>, // What can block propagation
    pub propagation_amplifiers: Vec<String>, // What amplifies propagation
}

/// Event for system transformation requests
#[derive(Event, Debug, Clone)]
pub struct SystemTransformationEvent {
    pub system_name: String,      // Which system to transform
    pub target_dread_level: u8,   // Target dread level for transformation
    pub transformation_reason: String, // Why this transformation is happening
    pub override_existing: bool,  // Should existing transformations be overridden?
    pub transformation_parameters: HashMap<String, f32>, // Custom transformation parameters
    pub rollback_conditions: Vec<String>, // Conditions that would reverse this transformation
    pub affected_entities: Vec<Entity>, // Specific entities affected by transformation
}

/// Event for dread milestone achievements
#[derive(Event, Debug, Clone)]
pub struct DreadMilestoneEvent {
    pub milestone: DreadMilestone,
    pub achievement_context: String, // Context in which milestone was achieved
    pub player_actions_involved: Vec<String>, // Player actions that led to this
    pub companion_reactions: HashMap<Uuid, String>, // How each companion reacts
    pub world_state_changes: Vec<WorldStateChange>, // How the world changes
    pub narrative_consequences: Vec<String>, // Story consequences
}

/// Changes to world state from milestone achievement
#[derive(Debug, Clone)]
pub struct WorldStateChange {
    pub change_type: String,      // "unlock_system", "modify_parameter", "spawn_entity"
    pub target: String,           // What is being changed
    pub change_value: String,     // New value or modification
    pub permanent: bool,          // Is this change permanent?
}

/// Event for dragon presence affecting dread (unique to Dragon's Labyrinth)
#[derive(Event, Debug, Clone)]
pub struct DragonPresenceEvent {
    pub dragon_entity: Entity,
    pub dragon_position: Vec3,
    pub player_distance: f32,     // Distance to player
    pub companion_distances: HashMap<Uuid, f32>, // Distances to each companion
    pub dragon_activity: String, // What the dragon is doing
    pub dragon_intelligence_level: f32, // How intelligent dragon appears
    pub stalking_duration: f32,   // How long dragon has been stalking
    pub visibility_to_party: f32, // 0.0-1.0 how visible dragon is to party
    pub dread_contribution: f32,  // Calculated dread contribution
}

/// Event for companion psychology affecting dread levels
#[derive(Event, Debug, Clone)]
pub struct CompanionDreadEvent {
    pub companion_entity: Entity,
    pub companion_id: Uuid,
    pub companion_type: String,   // "einar", "mira", "sorin", "tamara"
    pub dread_effect: CompanionDreadEffect, // How companion affects dread
    pub emotional_state: String,  // Current emotional state of companion
    pub trauma_level: f32,        // Current trauma level
    pub support_given: f32,       // Support companion is providing to others
    pub support_needed: f32,      // Support companion needs from others
}

/// How a companion affects dread levels
#[derive(Debug, Clone)]
pub enum CompanionDreadEffect {
    ReduceDread(f32),             // Companion reduces party dread by amount
    IncreaseDread(f32),           // Companion increases party dread by amount
    StabilizeDread,               // Companion prevents dread fluctuations
    AmplifyDread(f32),            // Companion amplifies existing dread by multiplier
    AbsorbDread(f32),             // Companion absorbs dread from party (taking it on themselves)
    ReflectDread(f32),            // Companion reflects dread back to source
}

/// Event for environmental dread changes
#[derive(Event, Debug, Clone)]
pub struct EnvironmentalDreadEvent {
    pub location_id: String,      // Which location is affected
    pub environmental_factors: Vec<EnvironmentalDreadFactor>, // Contributing factors
    pub weather_influence: f32,   // How weather affects dread here
    pub corruption_level: f32,    // Corruption level at this location
    pub isolation_factor: f32,    // How isolated this location feels
    pub historical_trauma: f32,   // Dread from past events at this location
    pub escape_routes_available: usize, // Number of escape routes
    pub safe_zones_present: usize, // Number of safe zones
}

/// Factors contributing to environmental dread
#[derive(Debug, Clone)]
pub struct EnvironmentalDreadFactor {
    pub factor_type: String,      // "terrain", "lighting", "sounds", "smells", "temperature"
    pub intensity: f32,           // 0.0-1.0 intensity of this factor
    pub temporal_pattern: String, // "constant", "increasing", "pulsing", "random"
    pub player_sensitivity: f32,  // How sensitive player is to this factor
    pub companion_sensitivities: HashMap<Uuid, f32>, // Companion sensitivities
}

/// Event for narrative dread progression
#[derive(Event, Debug, Clone)]
pub struct NarrativeDreadEvent {
    pub story_beat_id: String,    // Current story beat
    pub narrative_progression: NarrativeDreadProgression, // Progression details
    pub revelation_type: String,  // Type of revelation triggering dread
    pub dramatic_irony_level: f32, // How much player knows vs companions
    pub foreshadowing_elements: Vec<String>, // Foreshadowing contributing to dread
    pub player_agency_level: f32, // How much control player has
    pub narrative_tension_curve: Vec<f32>, // Tension progression over time
}

/// Event for reality distortion manifestations (high dread levels)
#[derive(Event, Debug, Clone)]
pub struct RealityDistortionEvent {
    pub distortion_type: String,  // "geometric", "temporal", "causal", "perceptual"
    pub manifestation: DistortionManifestation, // Specific manifestation
    pub affected_area: Vec3,      // Center of affected area
    pub distortion_radius: f32,   // Radius of effect
    pub stability: f32,           // 0.0-1.0 how stable the distortion is
    pub player_perception: f32,   // How much player notices (0.0-1.0)
    pub companion_perceptions: HashMap<Uuid, f32>, // How much each companion notices
    pub gameplay_effects: Vec<GameplayEffect>, // Specific gameplay impacts
}

/// Specific gameplay effects from reality distortion
#[derive(Debug, Clone)]
pub struct GameplayEffect {
    pub effect_type: String,      // "navigation_interference", "combat_modifier", "dialogue_corruption"
    pub effect_strength: f32,     // Strength of the effect
    pub duration: Option<f32>,    // Duration in seconds (None = permanent while distortion active)
    pub affected_systems: Vec<String>, // Which systems are affected
}

/// Event for player adaptation to dread
#[derive(Event, Debug, Clone)]
pub struct PlayerDreadAdaptationEvent {
    pub player_id: Uuid,
    pub adaptation_type: AdaptationType, // Type of adaptation occurring
    pub dread_level_exposed: u8,  // Which dread level player was exposed to
    pub exposure_duration: f32,   // How long exposure lasted
    pub adaptation_gained: f32,   // How much adaptation was gained
    pub stress_response: f32,     // Player's stress response level
    pub habituation_sources: Vec<String>, // Sources player is habituating to
    pub sensitization_triggers: Vec<String>, // Things making player more sensitive
}

/// Types of dread adaptation
#[derive(Debug, Clone)]
pub enum AdaptationType {
    Habituation,                  // Getting used to dread source
    Sensitization,                // Becoming more sensitive to dread
    Breakthrough,                 // Dread overcomes adaptation
    Recovery,                     // Recovering from dread exposure
    Desensitization,             // Actively becoming less sensitive
    Retraumatization,            // Previous trauma reactivated by current dread
}

/// Event for dread aura interactions
#[derive(Event, Debug, Clone)]
pub struct DreadAuraInteractionEvent {
    pub primary_aura_entity: Entity, // Primary aura
    pub secondary_aura_entity: Entity, // Secondary aura
    pub interaction_type: String, // "amplification", "interference", "resonance", "cancellation"
    pub interaction_strength: f32, // Strength of interaction
    pub resulting_dread_change: f32, // Net change in dread from interaction
    pub affected_radius: f32,     // Radius of interaction effect
    pub duration: f32,            // Duration of interaction effect
    pub stability: f32,           // How stable this interaction is
}

/// Event for system corruption due to high dread
#[derive(Event, Debug, Clone)]
pub struct SystemCorruptionEvent {
    pub system_name: String,      // Which system is being corrupted
    pub corruption_type: String,  // Type of corruption
    pub corruption_level: f32,    // 0.0-1.0 level of corruption
    pub corruption_source: String, // What's causing the corruption
    pub affected_parameters: Vec<String>, // Which system parameters are corrupted
    pub recovery_possible: bool,  // Can this corruption be fixed?
    pub recovery_requirements: Vec<String>, // What's needed to fix it
    pub cascading_effects: Vec<String>, // Other systems that might be affected
}

/// Event for dread resistance and mitigation
#[derive(Event, Debug, Clone)]
pub struct DreadResistanceEvent {
    pub resistance_source: String, // Source of resistance
    pub resistance_type: String,  // "natural", "learned", "item_based", "social_support"
    pub resistance_strength: f32, // How much resistance is provided
    pub protected_entities: Vec<Entity>, // Entities receiving protection
    pub resistance_duration: Option<f32>, // Duration of resistance (None = permanent)
    pub resistance_cost: Option<String>, // Cost of maintaining resistance
    pub breakthrough_threshold: f32, // Dread level that overcomes this resistance
}

/// Event for dread contagion spreading
#[derive(Event, Debug, Clone)]
pub struct DreadContagionEvent {
    pub source_entity: Entity,    // Entity spreading dread
    pub affected_entities: Vec<Entity>, // Entities receiving dread
    pub contagion_type: String,   // Type of contagion
    pub transmission_method: String, // How dread is spreading
    pub contagion_strength: f32,  // Strength of contagion
    pub resistance_factors: Vec<String>, // Things that resist contagion
    pub amplification_factors: Vec<String>, // Things that amplify contagion
    pub quarantine_possible: bool, // Can contagion be contained?
}

/// Event for triggering dread spikes from narrative events
#[derive(Event, Debug, Clone)]
pub struct DreadSpikeEvent {
    pub spike_trigger: String,    // What triggered the spike
    pub spike_data: DreadSpike,   // Spike configuration
    pub trigger_context: String, // Context of the trigger
    pub player_preparation: f32,  // How prepared player was (0.0-1.0)
    pub companion_reactions: HashMap<Uuid, CompanionSpikeReaction>, // How companions react
    pub mitigation_available: Vec<String>, // Ways to mitigate the spike
    pub cascading_consequences: Vec<String>, // Follow-up effects
}

/// How a companion reacts to a dread spike
#[derive(Debug, Clone)]
pub struct CompanionSpikeReaction {
    pub reaction_type: String,    // "panic", "denial", "acceptance", "protection"
    pub reaction_intensity: f32,  // Strength of reaction
    pub support_offered: f32,     // Support companion offers to others
    pub support_needed: f32,      // Support companion needs from others
    pub behavioral_changes: Vec<String>, // How behavior changes
    pub dialogue_modifications: Vec<String>, // How dialogue changes
}

/// Event for dread level stabilization and destabilization
#[derive(Event, Debug, Clone)]
pub struct DreadStabilityEvent {
    pub stability_change: f32,    // Change in stability (-1.0 to 1.0)
    pub stability_factors: Vec<StabilityFactor>, // Factors affecting stability
    pub current_stability: f32,   // Current stability level
    pub required_stability: f32,  // Required stability for current dread level
    pub stabilization_methods: Vec<String>, // Available stabilization methods
    pub destabilization_threats: Vec<String>, // Threats to stability
}

/// Factor affecting dread stability
#[derive(Debug, Clone)]
pub struct StabilityFactor {
    pub factor_type: String,      // Type of stability factor
    pub stability_contribution: f32, // How much this factor contributes
    pub factor_duration: Option<f32>, // How long this factor lasts
    pub controllable: bool,       // Can player influence this factor?
}

/// Event for requesting system parameter modifications due to dread
#[derive(Event, Debug, Clone)]
pub struct DreadParameterModificationEvent {
    pub system_name: String,      // Target system
    pub parameter_modifications: Vec<ParameterModification>, // Specific modifications
    pub modification_reason: String, // Why these modifications are happening
    pub priority: u8,             // Priority of these modifications
    pub duration: Option<f32>,    // Duration of modifications (None = permanent)
    pub rollback_triggers: Vec<String>, // What would cause rollback
}

/// Specific parameter modification
#[derive(Debug, Clone)]
pub struct ParameterModification {
    pub parameter_name: String,   // Name of parameter to modify
    pub modification_type: String, // "multiply", "add", "replace", "toggle"
    pub modification_value: f32,  // Value for modification
    pub original_value: Option<f32>, // Original value (for rollback)
    pub transition_speed: f32,    // How fast to apply modification
}

/// Event for dread-based feature toggling
#[derive(Event, Debug, Clone)]
pub struct DreadFeatureToggleEvent {
    pub system_name: String,      // Which system's features to toggle
    pub feature_toggles: Vec<FeatureToggle>, // Specific feature changes
    pub toggle_reason: String,    // Why features are being toggled
    pub affected_entities: Vec<Entity>, // Entities affected by toggles
    pub player_notification: bool, // Should player be notified?
    pub companion_awareness: HashMap<Uuid, bool>, // Which companions notice
}

/// Specific feature toggle
#[derive(Debug, Clone)]
pub struct FeatureToggle {
    pub feature_name: String,     // Name of feature
    pub new_state: bool,          // New enabled/disabled state
    pub toggle_conditions: Vec<String>, // Conditions that triggered toggle
    pub alternative_features: Vec<String>, // Features that might replace this one
    pub restoration_requirements: Vec<String>, // Requirements to restore feature
}

/// Event for cross-system dread integration
#[derive(Event, Debug, Clone)]
pub struct DreadIntegrationEvent {
    pub primary_system: String,   // System initiating integration
    pub target_systems: Vec<String>, // Systems being integrated with
    pub integration_type: String, // Type of integration
    pub integration_data: HashMap<String, f32>, // Data to share between systems
    pub synchronization_required: bool, // Do systems need to synchronize?
    pub conflict_resolution: String, // How to resolve conflicts
}

/// Event for emergency dread interventions
#[derive(Event, Debug, Clone)]
pub struct DreadEmergencyEvent {
    pub emergency_type: String,   // Type of emergency intervention
    pub trigger_conditions: Vec<String>, // What triggered the emergency
    pub emergency_actions: Vec<EmergencyAction>, // Actions to take
    pub affected_systems: Vec<String>, // Systems that need emergency modification
    pub player_safety_measures: Vec<String>, // Measures to protect player experience
    pub recovery_plan: Vec<String>, // Plan for returning to normal
}

/// Emergency action to take
#[derive(Debug, Clone)]
pub struct EmergencyAction {
    pub action_type: String,      // Type of emergency action
    pub action_target: String,    // Target of the action
    pub action_parameters: HashMap<String, f32>, // Action parameters
    pub action_priority: u8,      // Priority of this action
    pub action_duration: Option<f32>, // Duration of action
    pub success_criteria: Vec<String>, // How to know if action succeeded
}

/// Event for tracking dread progression analytics
#[derive(Event, Debug, Clone)]
pub struct DreadAnalyticsEvent {
    pub analytics_type: String,   // Type of analytics data
    pub data_points: HashMap<String, f32>, // Specific data points
    pub player_behavior: Vec<String>, // Player behaviors observed
    pub system_performance: HashMap<String, f32>, // How systems are performing
    pub balance_recommendations: Vec<String>, // Recommendations for balance changes
    pub player_feedback_indicators: Vec<String>, // Indicators of player experience
}

/// Event for dread system debugging and monitoring
#[derive(Event, Debug, Clone)]
pub struct DreadDebugEvent {
    pub debug_type: String,       // Type of debug information
    pub system_states: HashMap<String, String>, // Current states of systems
    pub active_modifications: Vec<String>, // Currently active modifications
    pub error_conditions: Vec<String>, // Any error conditions detected
    pub performance_metrics: HashMap<String, f32>, // Performance metrics
    pub recommended_actions: Vec<String>, // Recommended debugging actions
}
