//! Level definitions for the 180-level journey
//!
//! This module defines all 180 levels with their:
//! - Hex overworld configurations
//! - 3D labyrinth transitions
//! - Boss encounters
//! - Organic mechanic discovery moments

use serde::{Deserialize, Serialize};

pub mod journey_to;      // Levels 1-60: Journey TO the labyrinth
pub mod journey_from;    // Levels 61-120: Journey FROM the labyrinth  
pub mod seal_void;       // Levels 121-180: Seal the void

/// A single level in the game
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Level {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub level_type: LevelType,
    pub dread_level: u8,  // 0-4
    pub mechanics_introduced: Vec<GameMechanic>,
    pub encounters: Vec<Encounter>,
    pub narrative_beats: Vec<String>,
    pub companion_dialogue: Option<CompanionDialogue>,
}

/// Type of level - determines rendering and gameplay
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LevelType {
    /// 2.5D hex overworld exploration
    HexOverworld {
        tiles: Vec<HexTileConfig>,
        weather: WeatherType,
        elevation_map: Vec<Vec<i8>>,
    },
    /// 3D first-person labyrinth (DOOM-style)
    Labyrinth3D {
        maze_seed: u64,
        corruption_level: f32,
        rooms: Vec<RoomConfig>,
    },
    /// Unique boss arena
    BossArena {
        boss_type: BossType,
        arena_layout: ArenaLayout,
        phases: u8,
    },
}

/// Game mechanics that can be discovered
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GameMechanic {
    // Movement
    BasicMovement,
    Sprint,
    Dodge,
    MountRiding,
    
    // Combat
    BasicAttack,
    HeavyAttack,
    Parry,
    CriticalHit,
    ElementalDamage,
    
    // Crafting
    WeaponRepair,
    ArmorCrafting,
    PotionBrewing,
    EnchantmentRunes,
    
    // Social
    CompanionCommands,
    VillagerTrade,
    MountBonding,
    PhilosophyChoice,
    
    // Exploration
    SecretPassages,
    EnvironmentalPuzzles,
    VoidRifts,
}

/// Encounter types for a level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Encounter {
    /// Standard enemy encounter
    Combat {
        enemy_type: String,
        count: u8,
        difficulty: f32,
    },
    /// Puzzle that must be solved
    Puzzle {
        puzzle_type: String,
        complexity: u8,
    },
    /// NPC interaction
    Social {
        npc_name: String,
        interaction_type: String,
    },
    /// Environmental hazard
    Hazard {
        hazard_type: String,
        damage_per_second: f32,
    },
}

/// Weather types for hex overworld
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WeatherType {
    Clear,
    Rain,
    Storm,
    Fog,
    Snow,
    AshFall,
    VoidStorm,
}

/// Boss types for boss arenas
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BossType {
    BanditLeader,      // L20
    CorruptedKnight,   // L40
    SwampWitch,        // L60
    DragonFragment,    // L80
    VoidHerald,        // L100
    FallenCompanion,   // L120
    MirrorSelf,        // L140
    VoidDragon,        // L160
    TrueDragon,        // L180
}

/// Arena layout types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArenaLayout {
    CircularPit,
    BrokenBridge,
    CrumblingTower,
    VoidPlatforms,
    ShiftingMaze,
}

/// Hex tile configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HexTileConfig {
    pub q: i32,  // Hex coordinate
    pub r: i32,  // Hex coordinate
    pub tile_type: String,
    pub elevation: i8,
    pub passable: bool,
    pub corruption: f32,
}

/// Room configuration for 3D labyrinths
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomConfig {
    pub id: u32,
    pub room_type: String,
    pub size: (u8, u8, u8),
    pub connections: Vec<u32>,
    pub loot: Vec<String>,
    pub enemies: Vec<String>,
}

/// Companion dialogue for a level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanionDialogue {
    pub elena: Vec<String>,
    pub marcus: Vec<String>,
    pub quinn: Vec<String>,
}

/// Get all 180 levels
pub fn get_all_levels() -> Vec<Level> {
    let mut levels = Vec::new();
    
    // Journey TO (1-60)
    levels.extend(journey_to::get_levels());
    
    // Journey FROM (61-120)
    levels.extend(journey_from::get_levels());
    
    // Seal Void (121-180)
    levels.extend(seal_void::get_levels());
    
    levels
}

/// Get a specific level by ID
pub fn get_level(id: u32) -> Option<Level> {
    get_all_levels().into_iter().find(|l| l.id == id)
}

/// Get levels for a specific dread level
pub fn get_levels_by_dread(dread: u8) -> Vec<Level> {
    get_all_levels()
        .into_iter()
        .filter(|l| l.dread_level == dread)
        .collect()
}

/// Get the level where a mechanic is introduced
pub fn get_mechanic_introduction(mechanic: GameMechanic) -> Option<Level> {
    get_all_levels()
        .into_iter()
        .find(|l| l.mechanics_introduced.contains(&mechanic))
}
