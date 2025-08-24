//! Flexible Level System for Dragon's Labyrinth
//! Uses content from game-content-static and game-content-generated

use bevy::prelude::*;
use game_content_static::{
    characters::{CompanionArchetype, NPCArchetype},
    philosophy::{PhilosophyPath, MoralWeight},
    dread::DreadLevel,
    progression::{ProgressionTrigger, MechanicUnlock},
};

/// Level definition that can be placed at ANY level number
#[derive(Component, Clone)]
pub struct LevelContent {
    pub level_range: std::ops::Range<u32>,  // Can span multiple levels
    pub content_type: ContentType,
    pub prerequisites: Vec<Prerequisite>,
    pub outcomes: Vec<Outcome>,
}

#[derive(Clone)]
pub enum ContentType {
    /// Major story beats
    StoryBeat {
        id: String,
        narrative_weight: f32,
        required: bool,  // Can't skip
    },
    
    /// Combat encounters
    Encounter {
        enemy_type: String,
        difficulty_curve: DifficultyCurve,
        variants: Vec<String>,
    },
    
    /// Mechanic introduction
    MechanicIntroduction {
        mechanic: MechanicUnlock,
        organic_trigger: String,  // "When walking becomes tedious"
        fallback_level: u32,      // Latest it can appear
    },
    
    /// Village/Hub
    Settlement {
        size: SettlementSize,
        shops: Vec<String>,
        quest_density: f32,
    },
    
    /// Boss encounter
    Boss {
        boss_type: BossType,
        arena_type: ArenaType,
    },
}

#[derive(Clone)]
pub enum DifficultyCurve {
    Linear(f32),           // Steady increase
    Exponential(f32),      // Ramping up
    Plateau(f32),          // Stays same
    Sawtooth(f32, f32),   // Spike then drop
}

#[derive(Clone)]
pub enum SettlementSize {
    Hamlet,      // 3-5 NPCs
    Village,     // 10-20 NPCs
    Town,        // 30-50 NPCs
    City,        // 100+ NPCs
}

#[derive(Clone)]
pub enum BossType {
    MiniBoss,    // Overworld, skill check
    MajorBoss,   // 3D dungeon, story critical
    OptionalBoss, // Hidden, extra challenge
}

#[derive(Clone)]
pub enum ArenaType {
    Overworld,   // 2.5D hex map
    Dungeon3D,   // DOOM-style
    Unique,      // Special mechanics
}

#[derive(Clone)]
pub enum Prerequisite {
    Level(u32),
    CompanionPresent(CompanionArchetype),
    PhilosophyPath(PhilosophyPath, f32),  // Path and minimum strength
    QuestComplete(String),
    ItemOwned(String),
    DeathCount(u32),
    MountBonded(f32),
}

#[derive(Clone)]
pub enum Outcome {
    UnlockMechanic(MechanicUnlock),
    PhilosophyShift(PhilosophyPath, f32),
    CompanionTrust(f32),
    DreadIncrease(f32),
    ReputationChange(String, f32),  // Faction, amount
    ItemGained(String),
    PermanentChoice(String),
}

/// The actual level progression for Act 1
pub fn create_act1_progression() -> Vec<LevelContent> {
    vec![
        // LEVEL 1: The Door
        LevelContent {
            level_range: 1..2,
            content_type: ContentType::StoryBeat {
                id: "the_door".to_string(),
                narrative_weight: 1.0,
                required: true,
            },
            prerequisites: vec![],
            outcomes: vec![
                Outcome::PermanentChoice("companion".to_string()),
                Outcome::UnlockMechanic(MechanicUnlock::CompanionCombat),
            ],
        },
        
        // LEVELS 1-3: Path to Village (flexible placement)
        LevelContent {
            level_range: 1..4,
            content_type: ContentType::Encounter {
                enemy_type: "wolf".to_string(),
                difficulty_curve: DifficultyCurve::Linear(0.3),
                variants: vec![
                    "starving".to_string(),
                    "mother".to_string(),
                    "rabid".to_string(),
                    "pack".to_string(),
                ],
            },
            prerequisites: vec![],
            outcomes: vec![
                Outcome::PhilosophyShift(PhilosophyPath::Harmony, 0.1),
            ],
        },
        
        // LEVEL 3-5: First Village
        LevelContent {
            level_range: 3..6,
            content_type: ContentType::Settlement {
                size: SettlementSize::Village,
                shops: vec!["inn".to_string(), "blacksmith".to_string(), "general".to_string()],
                quest_density: 0.3,  // Light introduction
            },
            prerequisites: vec![Prerequisite::Level(3)],
            outcomes: vec![
                Outcome::ReputationChange("Haven's Rest".to_string(), 0.2),
            ],
        },
        
        // LEVEL 10: Mini-Boss (flexible ±2 levels)
        LevelContent {
            level_range: 8..12,
            content_type: ContentType::Boss {
                boss_type: BossType::MiniBoss,
                arena_type: ArenaType::Overworld,
            },
            prerequisites: vec![Prerequisite::Level(8)],
            outcomes: vec![
                Outcome::ItemGained("bandit_horn".to_string()),
                Outcome::ReputationChange("region".to_string(), 0.3),
            ],
        },
        
        // LEVEL 20: Bandit Cave + Mount Introduction
        LevelContent {
            level_range: 19..21,
            content_type: ContentType::Boss {
                boss_type: BossType::MajorBoss,
                arena_type: ArenaType::Dungeon3D,
            },
            prerequisites: vec![Prerequisite::Level(19)],
            outcomes: vec![
                Outcome::UnlockMechanic(MechanicUnlock::Mount),  // NOW we get mount!
                Outcome::DreadIncrease(0.1),  // First void touch
                Outcome::PermanentChoice("bandit_fate".to_string()),
            ],
        },
        
        // MOUNT SYSTEM - Introduced AFTER Bandit Cave
        LevelContent {
            level_range: 20..22,
            content_type: ContentType::MechanicIntroduction {
                mechanic: MechanicUnlock::Mount,
                organic_trigger: "Merchant grateful for saving caravan".to_string(),
                fallback_level: 25,  // Must have by L25
            },
            prerequisites: vec![
                Prerequisite::QuestComplete("bandit_cave".to_string()),
            ],
            outcomes: vec![
                Outcome::ItemGained("mount".to_string()),
                Outcome::UnlockMechanic(MechanicUnlock::MountBonding),
            ],
        },
    ]
}
