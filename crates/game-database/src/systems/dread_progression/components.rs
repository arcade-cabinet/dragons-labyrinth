//! Dread Progression System - ECS Components
//!
//! Production-ready components for Dragon's Labyrinth's core horror mechanic:
//! the dread level system (0-4) that transforms ALL game systems as horror intensifies.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

/// Master dread level component - controls all horror progression
#[derive(Component, Reflect, Debug, Clone, Serialize, Deserialize)]
#[reflect(Component)]
pub struct DreadLevel {
    pub current_level: u8,        // 0-4 dread intensity (0=Peace, 4=Horror)
    pub previous_level: u8,       // Previous dread level for transition detection
    pub progression_rate: f32,    // How fast dread increases/decreases per second
    pub stability: f32,           // 0.0-1.0 how stable current level is
    pub external_factors: f32,    // -1.0 to 1.0 external influences on dread
    
    // Dread sources and triggers
    pub active_sources: Vec<DreadSource>, // What's causing current dread
    pub environmental_dread: f32, // 0.0-1.0 dread from environment
    pub narrative_dread: f32,     // 0.0-1.0 dread from story progression
    pub companion_dread: f32,     // 0.0-1.0 dread from companion states
    pub player_action_dread: f32, // 0.0-1.0 dread from player choices
    
    // System modification tracking
    pub affected_systems: HashMap<String, SystemDreadModification>, // Which systems are affected
    pub last_update: i64,         // Timestamp of last dread level change
    pub transition_duration: f32, // Seconds for smooth level transitions
}

/// Individual source of dread with specific properties
#[derive(Reflect, Debug, Clone, Serialize, Deserialize)]
pub struct DreadSource {
    pub source_id: String,        // "dragon_presence", "companion_breakdown", "reality_distortion"
    pub source_type: String,      // "environmental", "narrative", "psychological", "supernatural"
    pub intensity: f32,           // 0.0-5.0 intensity contribution
    pub decay_rate: f32,          // How fast this source fades
    pub radius: Option<f32>,      // Spatial radius of effect (for environmental sources)
    pub duration_remaining: f32,  // Seconds until source expires
    pub compounding_factor: f32,  // How this source amplifies other sources
    pub description: String,      // Human-readable description
}

/// How a specific game system is modified by current dread level
#[derive(Reflect, Debug, Clone, Serialize, Deserialize)]
pub struct SystemDreadModification {
    pub system_name: String,      // "combat", "dialogue", "hex_rendering", etc.
    pub modification_type: String, // "scaling", "replacement", "corruption", "enhancement"
    pub base_values: HashMap<String, f32>, // Original system values
    pub dread_modifiers: [f32; 5], // Modifiers for dread levels 0-4
    pub active_modifications: Vec<ActiveModification>, // Currently applied changes
    pub last_applied_level: u8,   // Last dread level this was applied for
}

/// Currently active modification to a system parameter
#[derive(Reflect, Debug, Clone, Serialize, Deserialize)]
pub struct ActiveModification {
    pub parameter_name: String,   // What parameter is being modified
    pub original_value: f32,      // Original value before dread modification
    pub modified_value: f32,      // Current value with dread applied
    pub modification_reason: String, // Why this modification was applied
    pub reversible: bool,         // Can this be undone when dread decreases?
}

/// Component for entities that generate dread aura
#[derive(Component, Reflect, Debug, Clone, Serialize, Deserialize)]
#[reflect(Component)]
pub struct DreadAura {
    pub aura_type: String,        // "dragon", "corruption", "madness", "void"
    pub base_intensity: f32,      // 0.0-5.0 base dread generation
    pub current_intensity: f32,   // Current intensity (may vary)
    pub effective_radius: f32,    // Radius in game units
    pub falloff_curve: String,    // "linear", "exponential", "inverse_square"
    pub penetration: f32,         // 0.0-1.0 ability to penetrate barriers
    pub resonance_frequency: f32, // How this aura interacts with others
    pub pulsing: Option<DreadPulse>, // Optional pulsing pattern
}

/// Pulsing pattern for dread aura
#[derive(Reflect, Debug, Clone, Serialize, Deserialize)]
pub struct DreadPulse {
    pub pulse_period: f32,        // Seconds between pulses
    pub pulse_amplitude: f32,     // 0.0-1.0 strength variation
    pub pulse_shape: String,      // "sine", "square", "sawtooth", "random"
    pub current_phase: f32,       // Current position in pulse cycle
}

/// Component for tracking dread resistance and resilience
#[derive(Component, Reflect, Debug, Clone, Serialize, Deserialize)]
#[reflect(Component)]
pub struct DreadResistance {
    pub base_resistance: f32,     // 0.0-1.0 natural resistance to dread
    pub acquired_resistance: f32, // 0.0-1.0 learned resistance through experience
    pub temporary_resistance: f32, // 0.0-1.0 temporary buffs/items
    pub resistance_sources: Vec<ResistanceSource>, // What provides resistance
    pub breakdown_threshold: f32, // Dread level where resistance fails
    pub recovery_rate: f32,       // How fast resistance regenerates
    pub current_strain: f32,      // 0.0-1.0 current stress on resistance
}

/// Source of dread resistance
#[derive(Reflect, Debug, Clone, Serialize, Deserialize)]
pub struct ResistanceSource {
    pub source_id: String,        // "companion_support", "blessed_item", "mental_training"
    pub resistance_amount: f32,   // How much resistance this provides
    pub source_type: String,      // "psychological", "magical", "social", "spiritual"
    pub stability: f32,           // 0.0-1.0 how stable this resistance is
    pub duration_remaining: Option<f32>, // Seconds remaining (None for permanent)
}

/// Component for dread-based system transformations
#[derive(Component, Reflect, Debug, Clone, Serialize, Deserialize)]
#[reflect(Component)]
pub struct DreadTransformation {
    pub entity_id: Uuid,
    pub transformation_type: String, // "visual", "behavioral", "mechanical", "structural"
    pub base_state: String,       // Original state before dread transformation
    pub dread_states: [String; 5], // States for dread levels 0-4
    pub current_state: String,    // Current transformed state
    pub transition_speed: f32,    // How fast transformation occurs
    pub reversibility: f32,       // 0.0-1.0 how easily this can be reversed
    pub stability_requirement: f32, // How stable dread must be to maintain transformation
}

/// Component for narrative dread progression
#[derive(Component, Reflect, Debug, Clone, Serialize, Deserialize)]
#[reflect(Component)]
pub struct NarrativeDreadProgression {
    pub story_beat_id: String,    // Current story beat contributing to dread
    pub act: u8,                  // Story act (1-3, roughly corresponding to dread progression)
    pub chapter: u8,              // Chapter within act
    pub scene: u8,                // Scene within chapter
    pub narrative_tension: f32,   // 0.0-1.0 current narrative tension
    pub dramatic_irony_level: f32, // 0.0-1.0 player knows more than companions
    pub foreshadowing_intensity: f32, // 0.0-1.0 how much horror is hinted at
    pub revelation_proximity: f32, // 0.0-1.0 how close to major revelation
    pub character_agency: f32,    // 0.0-1.0 how much control player has (decreases with dread)
}

/// Component for environmental dread manifestation
#[derive(Component, Reflect, Debug, Clone, Serialize, Deserialize)]
#[reflect(Component)]
pub struct EnvironmentalDread {
    pub location_id: String,      // Specific location identifier
    pub base_dread_level: f32,    // 0.0-5.0 inherent dread of this location
    pub corruption_influence: f32, // 0.0-1.0 how much corruption affects dread here
    pub historical_trauma: f32,   // 0.0-1.0 dread from past events at this location
    pub supernatural_presence: f32, // 0.0-1.0 otherworldly influence
    pub isolation_factor: f32,    // 0.0-1.0 how isolated/trapped the location feels
    pub escape_routes: Vec<String>, // Available escape routes (affects dread)
    pub safe_zones: Vec<String>,  // Areas within location that feel safer
    pub dread_amplifiers: Vec<String>, // Environmental features that increase dread
}

/// Component for dread-based reality distortion
#[derive(Component, Reflect, Debug, Clone, Serialize, Deserialize)]
#[reflect(Component)]
pub struct RealityDistortion {
    pub distortion_type: String,  // "geometric", "temporal", "causal", "perceptual"
    pub intensity: f32,           // 0.0-1.0 intensity of distortion
    pub affected_radius: f32,     // Radius of distortion effect
    pub manifestations: Vec<DistortionManifestation>, // Specific reality breaks
    pub stability: f32,           // 0.0-1.0 how consistent the distortion is
    pub player_perception_filter: f32, // 0.0-1.0 how much player notices
    pub companion_awareness: HashMap<Uuid, f32>, // How much each companion notices
}

/// Specific manifestation of reality distortion
#[derive(Reflect, Debug, Clone, Serialize, Deserialize)]
pub struct DistortionManifestation {
    pub manifestation_id: String, // "impossible_geometry", "time_loop", "disappearing_exits"
    pub visual_description: String, // How this appears to player
    pub mechanical_effect: String, // How this affects gameplay
    pub trigger_conditions: Vec<String>, // What triggers this manifestation
    pub duration: f32,            // How long this manifestation lasts
    pub requires_dread_level: u8, // Minimum dread level for this to manifest
}

/// Component for tracking dread contagion between entities
#[derive(Component, Reflect, Debug, Clone, Serialize, Deserialize)]
#[reflect(Component)]
pub struct DreadContagion {
    pub contagion_rate: f32,      // How fast dread spreads from this entity
    pub susceptibility: f32,      // 0.0-1.0 how easily this entity catches dread from others
    pub transmission_range: f32,  // Range for dread contagion
    pub immunity_duration: f32,   // Seconds of immunity after exposure
    pub current_exposure: f32,    // Current accumulated exposure
    pub exposure_threshold: f32,  // Exposure needed to increase dread
    pub recovery_rate: f32,       // How fast exposure decays
}

/// Component for dread-based system overrides
#[derive(Component, Reflect, Debug, Clone, Serialize, Deserialize)]
#[reflect(Component)]
pub struct SystemOverride {
    pub target_system: String,    // Which system to override
    pub override_conditions: Vec<OverrideCondition>, // When to apply override
    pub override_parameters: HashMap<String, f32>, // Parameter overrides
    pub priority: u8,             // Override priority (higher wins)
    pub duration: Option<f32>,    // Duration of override (None = permanent)
    pub stacking_behavior: String, // "replace", "add", "multiply"
}

/// Condition for when to apply system override
#[derive(Reflect, Debug, Clone, Serialize, Deserialize)]
pub struct OverrideCondition {
    pub condition_type: String,   // "dread_level", "location", "narrative_state"
    pub condition_value: String,  // Value to match
    pub comparison: String,       // "equals", "greater_than", "less_than", "contains"
    pub required: bool,           // Must this condition be met?
}

/// Component for player dread adaptation and habituation
#[derive(Component, Reflect, Debug, Clone, Serialize, Deserialize)]
#[reflect(Component)]
pub struct DreadAdaptation {
    pub player_id: Uuid,
    pub adaptation_curve: Vec<AdaptationPoint>, // How player adapts over time
    pub habituation_rate: f32,    // How fast player gets used to dread
    pub sensitization_triggers: Vec<String>, // What makes player more sensitive
    pub current_adaptation_level: f32, // 0.0-1.0 current adaptation
    pub breakthrough_resistance: f32, // 0.0-1.0 resistance to dread spikes
    pub comfort_zone_boundaries: Vec<f32>, // Dread levels player is comfortable with
}

/// Point on player's dread adaptation curve
#[derive(Reflect, Debug, Clone, Serialize, Deserialize)]
pub struct AdaptationPoint {
    pub exposure_time: f32,       // Seconds of exposure to dread level
    pub dread_level: u8,          // Which dread level
    pub adaptation_gained: f32,   // How much adaptation was gained
    pub stress_response: f32,     // Physiological stress response
    pub behavioral_changes: Vec<String>, // How behavior changed
}

/// Component for dread-based audio and visual effects
#[derive(Component, Reflect, Debug, Clone, Serialize, Deserialize)]
#[reflect(Component)]
pub struct DreadEffects {
    pub visual_effects: Vec<DreadVisualEffect>, // Visual manifestations of dread
    pub audio_effects: Vec<DreadAudioEffect>,   // Audio manifestations of dread
    pub haptic_effects: Vec<DreadHapticEffect>, // Haptic feedback for dread
    pub ui_effects: Vec<DreadUIEffect>,         // UI changes due to dread
    pub effect_intensity_multiplier: f32,       // Overall effect intensity
    pub player_sensitivity: f32,  // 0.0-1.0 how sensitive player is to effects
}

/// Visual effect driven by dread level
#[derive(Reflect, Debug, Clone, Serialize, Deserialize)]
pub struct DreadVisualEffect {
    pub effect_type: String,      // "screen_distortion", "color_desaturation", "shadow_depth"
    pub intensity_curve: [f32; 5], // Intensity for dread levels 0-4
    pub trigger_threshold: f32,   // Dread level needed to activate
    pub effect_parameters: HashMap<String, f32>, // Effect-specific parameters
    pub layering_behavior: String, // How this layers with other effects
}

/// Audio effect driven by dread level
#[derive(Reflect, Debug, Clone, Serialize, Deserialize)]
pub struct DreadAudioEffect {
    pub effect_type: String,      // "ambient_distortion", "heartbeat", "whispers"
    pub intensity_curve: [f32; 5], // Intensity for dread levels 0-4
    pub frequency_range: (f32, f32), // Frequency range affected
    pub spatial_behavior: String, // "global", "positional", "binaural"
    pub psychological_trigger: String, // What psychological response this targets
}

/// Haptic feedback driven by dread level
#[derive(Reflect, Debug, Clone, Serialize, Deserialize)]
pub struct DreadHapticEffect {
    pub effect_type: String,      // "tension_pulse", "anxiety_tremor", "shock"
    pub intensity_curve: [f32; 5], // Intensity for dread levels 0-4
    pub pattern: String,          // Vibration pattern
    pub duration: f32,            // Effect duration in seconds
    pub trigger_events: Vec<String>, // What events trigger this haptic
}

/// UI changes driven by dread level
#[derive(Reflect, Debug, Clone, Serialize, Deserialize)]
pub struct DreadUIEffect {
    pub ui_element: String,       // Which UI element is affected
    pub modification_type: String, // "opacity", "position", "color", "availability"
    pub dread_thresholds: [f32; 5], // Values for dread levels 0-4
    pub transition_speed: f32,    // How fast UI changes occur
    pub accessibility_impact: f32, // 0.0-1.0 how much this affects accessibility
}

/// Component for dread milestone tracking
#[derive(Component, Reflect, Debug, Clone, Serialize, Deserialize)]
#[reflect(Component)]
pub struct DreadMilestone {
    pub milestone_id: String,     // Unique milestone identifier
    pub dread_level_required: u8, // Dread level needed to trigger
    pub narrative_context: String, // Story context for milestone
    pub unlock_conditions: Vec<String>, // Additional conditions required
    pub milestone_effects: Vec<MilestoneEffect>, // Effects when milestone is reached
    pub achieved: bool,           // Has this milestone been reached?
    pub achievement_timestamp: Option<i64>, // When milestone was achieved
    pub player_choice_influenced: bool, // Did player choice affect this milestone?
}

/// Effect that occurs when dread milestone is reached
#[derive(Reflect, Debug, Clone, Serialize, Deserialize)]
pub struct MilestoneEffect {
    pub effect_type: String,      // "system_unlock", "narrative_branch", "companion_event"
    pub effect_target: String,    // What is affected by this milestone
    pub effect_parameters: HashMap<String, f32>, // Effect parameters
    pub permanent: bool,          // Is this effect permanent?
    pub reversible: bool,         // Can this effect be undone?
}
