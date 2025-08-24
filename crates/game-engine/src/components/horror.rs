//! Horror and dread components
//!
//! The emotional core of Dragon's Labyrinth - systems that create
//! and respond to fear, corruption, and psychological horror.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Global dread resource - affects entire game world
#[derive(Resource, Debug, Clone)]
pub struct GlobalDread {
    pub level: u8,           // 0-4 as per design bible
    pub accumulation: f32,   // Progress to next level
    pub locked: bool,        // Can't decrease if true
    pub triggers: Vec<DreadTrigger>,
}

#[derive(Clone, Debug)]
pub struct DreadTrigger {
    pub description: String,
    pub magnitude: f32,
    pub timestamp: f32,
}

/// Corruption component - environmental decay
#[derive(Component, Clone, Debug)]
pub struct Corruption {
    pub level: f32,          // 0.0 = pristine, 1.0 = fully corrupted
    pub spread_rate: f32,
    pub corruption_type: CorruptionType,
    pub visual_stage: CorruptionVisual,
}

#[derive(Clone, Debug)]
pub enum CorruptionType {
    Natural,      // Trees dying, grass withering
    Structural,   // Buildings crumbling
    Biological,   // Flesh walls, organic horror
    Temporal,     // Time distortion
    Mental,       // Reality breaking down
}

#[derive(Clone, Debug)]
pub enum CorruptionVisual {
    Subtle,       // Barely noticeable
    Creeping,     // Spreading stains, cracks
    Manifest,     // Obvious decay
    Overwhelming, // Completely transformed
}

/// Hallucination component for low-sanity visuals
#[derive(Component, Clone, Debug)]
pub struct Hallucination {
    pub hallucination_type: HallucinationType,
    pub intensity: f32,
    pub duration: Timer,
    pub triggered_by_sanity: bool,
}

#[derive(Clone, Debug)]
pub enum HallucinationType {
    FalseEnemy,       // See threats that aren't there
    CompanionChange,  // Companion appears different
    EnvironmentShift, // World transforms briefly
    DeadSpeaking,     // Dead characters appear
    SelfReflection,   // See yourself as monster
    TimeLoop,         // Relive traumatic moment
}

/// Audio horror cues
#[derive(Component, Clone, Debug)]
pub struct HorrorAudio {
    pub cue_type: HorrorAudioType,
    pub range: f32,
    pub intensity: f32,
    pub is_real: bool,        // False if hallucination
    pub dread_required: u8,   // Min dread to hear
}

#[derive(Clone, Debug)]
pub enum HorrorAudioType {
    Breathing,        // Heavy breathing nearby
    Whispers,         // Incomprehensible voices
    Footsteps,        // Following the player
    Screaming,        // Distant or sudden
    Laughing,         // Children's laughter in wrong context
    Scratching,       // In walls
    Heartbeat,        // Player's accelerated heartbeat
    Silence,          // Sudden absence of all sound
}

/// Visual effects for horror moments
#[derive(Component, Clone, Debug)]
pub struct HorrorVisual {
    pub effect_type: HorrorVisualType,
    pub intensity: f32,
    pub duration: Timer,
}

#[derive(Clone, Debug)]
pub enum HorrorVisualType {
    ScreenDistortion,  // Wavering, bending
    ColorDrain,        // World loses color
    DarknessCreep,     // Shadows grow
    BloodOverlay,      // Red tinting
    StaticNoise,       // TV static effect
    DoubleVision,      // Blurred, doubled
    FlashImages,       // Brief horrific images
    GlitchEffect,      // Digital corruption
}

/// Fear response for NPCs and companions
#[derive(Component, Clone, Debug)]
pub struct FearResponse {
    pub fear_level: f32,
    pub fear_type: FearType,
    pub response: FearBehavior,
    pub breaking_point: f32,
}

#[derive(Clone, Debug)]
pub enum FearType {
    Rational,     // Normal fear of danger
    Irrational,   // Phobia-based
    Existential,  // Fear of meaninglessness
    Supernatural, // Fear of unknown
}

#[derive(Clone, Debug)]
pub enum FearBehavior {
    Freeze,       // Paralyzed
    Flight,       // Run away
    Fight,        // Aggressive response
    Fawn,         // Try to appease threat
    Dissociate,   // Mental escape
}

/// Trauma tracking for persistent effects
#[derive(Component, Clone, Debug)]
pub struct Trauma {
    pub trauma_type: TraumaType,
    pub severity: f32,
    pub triggers: Vec<String>,
    pub effects: Vec<TraumaEffect>,
}

#[derive(Clone, Debug)]
pub enum TraumaType {
    Witnessed,     // Saw something horrible
    Experienced,   // Had it happen to them
    Inflicted,     // Did something horrible
    Survived,      // Barely made it
}

#[derive(Clone, Debug)]
pub enum TraumaEffect {
    Nightmares,
    Flashbacks,
    Hypervigilance,
    Numbness,
    Aggression,
    Withdrawal,
}

/// The Forge - transformation system
#[derive(Component, Clone, Debug)]
pub struct ForgeOffering {
    pub offering_type: OfferingType,
    pub power_gained: String,
    pub cost: ForgeCost,
}

#[derive(Clone, Debug)]
pub enum OfferingType {
    Memory,        // Forget something important
    Emotion,       // Lose ability to feel
    Relationship,  // Sever bond with companion
    Identity,      // Part of who you are
    Future,        // Potential paths closed
}

#[derive(Clone, Debug)]
pub enum ForgeCost {
    Permanent,     // Can never recover
    Temporary,     // Might regain later
    Transferable,  // Someone else pays
}

/// Second chances system
#[derive(Component, Clone, Debug)]
pub struct SecondChance {
    pub chances_remaining: u8,
    pub last_death_cause: String,
    pub resurrection_cost: ResurrectionCost,
}

#[derive(Clone, Debug)]
pub enum ResurrectionCost {
    CompanionLife,      // Someone else dies
    MemoryLoss,         // Forget crucial information
    CorruptionSpike,    // World gets worse
    CompanionTrauma,    // Witnesses traumatized
    IdentityFragment,   // Lose part of self
}
