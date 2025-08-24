//! Levels 1-60: Journey TO the Labyrinth
//! 
//! The opening journey where players learn the world, develop bonds,
//! and slowly realize the horror that awaits.

use super::*;

/// Generate levels 1-60
pub fn get_levels() -> Vec<Level> {
    let mut levels = Vec::new();
    
    // OPENING SEQUENCE (L1-6)
    levels.push(Level {
        id: 1,
        name: "Mother's Plea".to_string(),
        description: "Your mother begs you to find your missing father. Choose to go alone or with a companion.".to_string(),
        level_type: LevelType::HexOverworld {
            tiles: generate_home_village_tiles(),
            weather: WeatherType::Clear,
            elevation_map: vec![vec![0; 10]; 10],
        },
        dread_level: 0,
        mechanics_introduced: vec![GameMechanic::BasicMovement],
        encounters: vec![],
        narrative_beats: vec![
            "Mother reveals father has been missing for weeks".to_string(),
            "Village elder warns of dangers ahead".to_string(),
            "CRITICAL CHOICE: Take companion or go alone".to_string(),
        ],
        companion_dialogue: Some(CompanionDialogue {
            elena: vec!["I've been waiting for adventure my whole life!".to_string()],
            marcus: vec!["Your father trained me well. I won't fail him.".to_string()],
            quinn: vec!["The stars speak of dark times ahead...".to_string()],
        }),
    });
    
    levels.push(Level {
        id: 2,
        name: "Forest Path".to_string(),
        description: "First steps into the wider world. Encounter wolves.".to_string(),
        level_type: LevelType::HexOverworld {
            tiles: generate_forest_tiles(),
            weather: WeatherType::Clear,
            elevation_map: generate_rolling_hills(),
        },
        dread_level: 0,
        mechanics_introduced: vec![GameMechanic::BasicAttack],
        encounters: vec![
            Encounter::Combat {
                enemy_type: "Wolf".to_string(),
                count: 2,
                difficulty: 0.5,
            }
        ],
        narrative_beats: vec![
            "First combat encounter".to_string(),
            "Find father's torn cloak".to_string(),
        ],
        companion_dialogue: Some(CompanionDialogue {
            elena: vec!["Watch out! I'll cover your flank!".to_string()],
            marcus: vec!["Form up! Use what I taught you!".to_string()],
            quinn: vec!["Violence... already it begins.".to_string()],
        }),
    });
    
    levels.push(Level {
        id: 3,
        name: "Crossroads Village".to_string(),
        description: "First village. Learn about missing children and strange dreams.".to_string(),
        level_type: LevelType::HexOverworld {
            tiles: generate_village_tiles(),
            weather: WeatherType::Rain,
            elevation_map: vec![vec![0; 15]; 15],
        },
        dread_level: 0,
        mechanics_introduced: vec![GameMechanic::VillagerTrade],
        encounters: vec![
            Encounter::Social {
                npc_name: "Worried Mother".to_string(),
                interaction_type: "Quest".to_string(),
            },
            Encounter::Social {
                npc_name: "Village Merchant".to_string(),
                interaction_type: "Trade".to_string(),
            },
        ],
        narrative_beats: vec![
            "Learn about missing children".to_string(),
            "Villagers speak of nightmares".to_string(),
            "Find child with void-touched eyes".to_string(),
        ],
        companion_dialogue: None,
    });
    
    // Continue building levels 4-60...
    // Level 5: Philosophy crystal appears
    levels.push(Level {
        id: 5,
        name: "Ancient Shrine".to_string(),
        description: "Discover the philosophy crystal that shows your path.".to_string(),
        level_type: LevelType::HexOverworld {
            tiles: generate_shrine_tiles(),
            weather: WeatherType::Fog,
            elevation_map: generate_elevated_platform(),
        },
        dread_level: 0,
        mechanics_introduced: vec![GameMechanic::PhilosophyChoice],
        encounters: vec![
            Encounter::Puzzle {
                puzzle_type: "Crystal Activation".to_string(),
                complexity: 2,
            }
        ],
        narrative_beats: vec![
            "Crystal resonates with your choices".to_string(),
            "First glimpse of philosophical alignment".to_string(),
        ],
        companion_dialogue: None,
    });
    
    // Level 7: First mount sighting
    levels.push(Level {
        id: 7,
        name: "Wild Plains".to_string(),
        description: "Wide open spaces where wild mounts roam.".to_string(),
        level_type: LevelType::HexOverworld {
            tiles: generate_plains_tiles(),
            weather: WeatherType::Clear,
            elevation_map: vec![vec![0; 20]; 20],
        },
        dread_level: 0,
        mechanics_introduced: vec![GameMechanic::Sprint],
        encounters: vec![
            Encounter::Social {
                npc_name: "Mount Herder".to_string(),
                interaction_type: "Information".to_string(),
            }
        ],
        narrative_beats: vec![
            "See wild mounts in the distance".to_string(),
            "Learn about mount bonding".to_string(),
        ],
        companion_dialogue: None,
    });
    
    // Level 10: Combat variety increases
    levels.push(Level {
        id: 10,
        name: "Bandit Outpost".to_string(),
        description: "First human enemies. Learn about the growing darkness.".to_string(),
        level_type: LevelType::HexOverworld {
            tiles: generate_outpost_tiles(),
            weather: WeatherType::Clear,
            elevation_map: generate_fortified_position(),
        },
        dread_level: 1,  // First dread increase
        mechanics_introduced: vec![GameMechanic::Parry, GameMechanic::HeavyAttack],
        encounters: vec![
            Encounter::Combat {
                enemy_type: "Bandit Scout".to_string(),
                count: 3,
                difficulty: 0.7,
            },
            Encounter::Combat {
                enemy_type: "Bandit Archer".to_string(),
                count: 2,
                difficulty: 0.8,
            },
        ],
        narrative_beats: vec![
            "Bandits speak of 'the coming darkness'".to_string(),
            "Find father's journal page".to_string(),
        ],
        companion_dialogue: None,
    });
    
    // Level 11: Weapon breaks, discover crafting
    levels.push(Level {
        id: 11,
        name: "Broken Bridge".to_string(),
        description: "Your weapon breaks mid-combat. Must craft a replacement.".to_string(),
        level_type: LevelType::HexOverworld {
            tiles: generate_bridge_tiles(),
            weather: WeatherType::Storm,
            elevation_map: generate_canyon_crossing(),
        },
        dread_level: 1,
        mechanics_introduced: vec![GameMechanic::WeaponRepair, GameMechanic::ArmorCrafting],
        encounters: vec![
            Encounter::Combat {
                enemy_type: "Stone Golem".to_string(),
                count: 1,
                difficulty: 1.0,
            }
        ],
        narrative_beats: vec![
            "Weapon shatters on golem".to_string(),
            "Discover abandoned smithy".to_string(),
            "Craft first weapon".to_string(),
        ],
        companion_dialogue: None,
    });
    
    // Level 13: Mount acquisition (was 20, moved earlier per user request)
    levels.push(Level {
        id: 13,
        name: "Mount Sanctuary".to_string(),
        description: "Earn your mount through trial.".to_string(),
        level_type: LevelType::HexOverworld {
            tiles: generate_sanctuary_tiles(),
            weather: WeatherType::Clear,
            elevation_map: vec![vec![0; 15]; 15],
        },
        dread_level: 1,
        mechanics_introduced: vec![GameMechanic::MountRiding, GameMechanic::MountBonding],
        encounters: vec![
            Encounter::Puzzle {
                puzzle_type: "Mount Trust Trial".to_string(),
                complexity: 3,
            }
        ],
        narrative_beats: vec![
            "Prove yourself to wild mount".to_string(),
            "Form permanent bond".to_string(),
            "Mount shows signs of void sensitivity".to_string(),
        ],
        companion_dialogue: None,
    });
    
    // Level 20: First major boss - Bandit Cave
    levels.push(Level {
        id: 20,
        name: "Bandit's Cave".to_string(),
        description: "First 3D labyrinth. Face the Bandit Leader.".to_string(),
        level_type: LevelType::Labyrinth3D {
            maze_seed: 20,
            corruption_level: 0.1,
            rooms: generate_bandit_cave_rooms(),
        },
        dread_level: 1,
        mechanics_introduced: vec![GameMechanic::Dodge, GameMechanic::SecretPassages],
        encounters: vec![
            Encounter::Combat {
                enemy_type: "Bandit Grunt".to_string(),
                count: 5,
                difficulty: 0.8,
            }
        ],
        narrative_beats: vec![
            "Enter first 3D dungeon".to_string(),
            "Navigate maze-like cave".to_string(),
        ],
        companion_dialogue: None,
    });
    
    levels.push(Level {
        id: 21,
        name: "Bandit Leader's Throne".to_string(),
        description: "Face the Bandit Leader in his throne room.".to_string(),
        level_type: LevelType::BossArena {
            boss_type: BossType::BanditLeader,
            arena_layout: ArenaLayout::CircularPit,
            phases: 2,
        },
        dread_level: 1,
        mechanics_introduced: vec![GameMechanic::CriticalHit],
        encounters: vec![],
        narrative_beats: vec![
            "Bandit Leader reveals void corruption".to_string(),
            "Learn about Dragon's influence".to_string(),
            "Father was here weeks ago".to_string(),
        ],
        companion_dialogue: None,
    });
    
    // Continue through level 60...
    // This is where we'd add levels 22-60 following the same pattern
    // Key milestones:
    // - L30: Second village, more quests
    // - L40: Corrupted Knight boss
    // - L50: Dread increases to 2
    // - L60: Reach the Labyrinth entrance
    
    levels
}

// Helper functions to generate tile configurations
fn generate_home_village_tiles() -> Vec<HexTileConfig> {
    vec![
        HexTileConfig {
            q: 0, r: 0,
            tile_type: "village_center".to_string(),
            elevation: 0,
            passable: true,
            corruption: 0.0,
        },
        // Add more tiles...
    ]
}

fn generate_forest_tiles() -> Vec<HexTileConfig> {
    vec![
        HexTileConfig {
            q: 0, r: 0,
            tile_type: "forest".to_string(),
            elevation: 0,
            passable: true,
            corruption: 0.0,
        },
        // Add more tiles...
    ]
}

fn generate_village_tiles() -> Vec<HexTileConfig> {
    vec![
        HexTileConfig {
            q: 0, r: 0,
            tile_type: "village".to_string(),
            elevation: 0,
            passable: true,
            corruption: 0.05,
        },
    ]
}

fn generate_shrine_tiles() -> Vec<HexTileConfig> {
    vec![
        HexTileConfig {
            q: 0, r: 0,
            tile_type: "shrine".to_string(),
            elevation: 2,
            passable: true,
            corruption: 0.0,
        },
    ]
}

fn generate_plains_tiles() -> Vec<HexTileConfig> {
    vec![
        HexTileConfig {
            q: 0, r: 0,
            tile_type: "plains".to_string(),
            elevation: 0,
            passable: true,
            corruption: 0.0,
        },
    ]
}

fn generate_outpost_tiles() -> Vec<HexTileConfig> {
    vec![
        HexTileConfig {
            q: 0, r: 0,
            tile_type: "outpost".to_string(),
            elevation: 1,
            passable: true,
            corruption: 0.1,
        },
    ]
}

fn generate_bridge_tiles() -> Vec<HexTileConfig> {
    vec![
        HexTileConfig {
            q: 0, r: 0,
            tile_type: "bridge".to_string(),
            elevation: -1,
            passable: true,
            corruption: 0.0,
        },
    ]
}

fn generate_sanctuary_tiles() -> Vec<HexTileConfig> {
    vec![
        HexTileConfig {
            q: 0, r: 0,
            tile_type: "sanctuary".to_string(),
            elevation: 1,
            passable: true,
            corruption: 0.0,
        },
    ]
}

fn generate_rolling_hills() -> Vec<Vec<i8>> {
    vec![vec![0, 1, 0, 1, 0]; 5]
}

fn generate_elevated_platform() -> Vec<Vec<i8>> {
    vec![vec![2; 5]; 5]
}

fn generate_fortified_position() -> Vec<Vec<i8>> {
    vec![vec![1, 2, 1]; 3]
}

fn generate_canyon_crossing() -> Vec<Vec<i8>> {
    vec![vec![-2, -1, 0, -1, -2]; 5]
}

fn generate_bandit_cave_rooms() -> Vec<RoomConfig> {
    vec![
        RoomConfig {
            id: 1,
            room_type: "entrance".to_string(),
            size: (10, 10, 5),
            connections: vec![2],
            loot: vec![],
            enemies: vec!["Bandit Scout".to_string()],
        },
        RoomConfig {
            id: 2,
            room_type: "corridor".to_string(),
            size: (5, 20, 5),
            connections: vec![1, 3],
            loot: vec!["Health Potion".to_string()],
            enemies: vec![],
        },
        RoomConfig {
            id: 3,
            room_type: "throne_antechamber".to_string(),
            size: (15, 15, 8),
            connections: vec![2, 4],
            loot: vec!["Armor Scrap".to_string()],
            enemies: vec!["Bandit Lieutenant".to_string()],
        },
        RoomConfig {
            id: 4,
            room_type: "throne_room".to_string(),
            size: (20, 20, 10),
            connections: vec![3],
            loot: vec!["Bandit Leader's Sword".to_string()],
            enemies: vec!["Bandit Leader".to_string()],
        },
    ]
}
