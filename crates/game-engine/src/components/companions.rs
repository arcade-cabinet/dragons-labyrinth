//! Companion and mount components
//!
//! Companions are the heart of Dragon's Labyrinth - each with their own
//! psychological profile, trauma responses, and potential for betrayal.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Core companion component
#[derive(Component, Clone, Debug)]
pub struct Companion {
    pub name: String,
    pub archetype: CompanionArchetype,
    pub psychology: PsychologicalProfile,
    pub loyalty: f32,       // 0.0 = will betray, 1.0 = absolutely loyal
    pub bond_strength: f32, // How connected to the player
    pub current_state: CompanionState,
}

/// The 12 companion archetypes from the design bible
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum CompanionArchetype {
    Therapist,      // Professional distance, hidden trauma
    Child,          // Innocent but observant
    Medic,          // Heals others, can't heal self
    Scholar,        // Knowledge brings madness
    Warrior,        // Strong but breaking
    Fool,           // Wisdom through madness
    Priest,         // Faith tested by horror
    Thief,          // Survivor with guilty past
    Noble,          // Privilege becomes burden
    Hermit,         // Isolated wisdom
    Artist,         // Sees too much beauty and horror
    Betrayer,       // Destined to turn
}

/// Psychological profile driving companion behavior
#[derive(Clone, Debug)]
pub struct PsychologicalProfile {
    pub trauma_level: f32,          // 0.0 = stable, 1.0 = completely broken
    pub coping_mechanism: CopingMechanism,
    pub breaking_point: BreakingPoint,
    pub fears: Vec<Fear>,
    pub attachments: Vec<String>,   // What they care about
    pub secrets: Vec<CompanionSecret>,
}

#[derive(Clone, Debug)]
pub enum CopingMechanism {
    Denial,           // "Everything is fine"
    Intellectualization, // Over-analyze to avoid feeling
    Humor,            // Laugh to keep from crying
    Aggression,       // Lash out at others
    Withdrawal,       // Pull away from everyone
    Caretaking,       // Focus on others to avoid self
    Substance,        // Numbing through consumption
    Faith,            // Clinging to belief
}

#[derive(Clone, Debug)]
pub enum BreakingPoint {
    WitnessedDeath,   // Seeing someone die
    PersonalBetrayal, // Being betrayed by trusted person
    MoralCompromise,  // Forced to do something against values
    PhysicalTorture,  // Bodily harm
    LossOfIdentity,   // Forgetting who they are
    ForcedChoice,    // Choose who lives/dies
    RealityBreak,    // When world stops making sense
}

#[derive(Clone, Debug)]
pub enum Fear {
    Abandonment,
    Death,
    Darkness,
    Isolation,
    Intimacy,
    Failure,
    TheTruth,       // Learning what's really happening
    Transformation, // Becoming something else
}

#[derive(Clone, Debug)]
pub struct CompanionSecret {
    pub secret_type: SecretType,
    pub revelation_trigger: String,
    pub impact_on_trust: f32,
}

#[derive(Clone, Debug)]
pub enum SecretType {
    DarkPast,         // Did something terrible
    HiddenAgenda,     // Has ulterior motives
    FalseIdentity,    // Not who they claim
    Infected,         // Carrying corruption
    PriorKnowledge,   // Knew about threat beforehand
    Connection,       // Related to antagonist/problem
}

/// Current psychological state
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CompanionState {
    Stable,
    Anxious,
    Stressed,
    Traumatized,
    Breaking,
    Broken,
    Catatonic,
    Hostile,
    Fled,         // Abandoned the party
    Dead,         // Permanent loss
}

/// Mount bonding system
#[derive(Component, Clone, Debug)]
pub struct Mount {
    pub mount_type: MountType,
    pub bond_level: f32,
    pub trust: f32,
    pub abilities: Vec<MountAbility>,
    pub personality: MountPersonality,
}

#[derive(Clone, Debug)]
pub enum MountType {
    Horse,
    Wolf,
    Bear,
    Raven,
    Drake,        // Small dragon
    Nightmare,    // Corrupted mount
    Spectral,     // Ghost mount
}

#[derive(Clone, Debug)]
pub enum MountAbility {
    FastTravel,
    CombatAssist,
    CarryExtra,
    SenseDanger,
    CrossWater,
    NightVision,
    FearResistance,
}

#[derive(Clone, Debug)]
pub enum MountPersonality {
    Loyal,
    Skittish,
    Aggressive,
    Protective,
    Independent,
    Mysterious,
}

/// Relationship tracking between companions
#[derive(Component, Clone, Debug)]
pub struct Relationship {
    pub source: Entity,
    pub target: Entity,
    pub affinity: f32,       // -1.0 = hate, 1.0 = love
    pub trust: f32,
    pub shared_trauma: Vec<String>,
    pub conflicts: Vec<String>,
}
