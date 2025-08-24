//! Levels 61-120: Journey FROM the Labyrinth
//! 
//! The return journey where everything has changed.
//! The world is corrupted, companions may betray, and the truth emerges.

use super::*;

/// Generate levels 61-120
pub fn get_levels() -> Vec<Level> {
    let mut levels = Vec::new();
    
    // EMERGENCE FROM LABYRINTH (L61-65)
    levels.push(Level {
        id: 61,
        name: "Shattered Exit".to_string(),
        description: "Emerge from the labyrinth forever changed. The world looks different.".to_string(),
        level_type: LevelType::HexOverworld {
            tiles: vec![],
            weather: WeatherType::AshFall,
            elevation_map: vec![vec![0; 10]; 10],
        },
        dread_level: 3,
        mechanics_introduced: vec![GameMechanic::VoidRifts],
        encounters: vec![
            Encounter::Hazard {
                hazard_type: "Void Leak".to_string(),
                damage_per_second: 2.0,
            }
        ],
        narrative_beats: vec![
            "You killed the dragon fragment".to_string(),
            "Void energy seeps into the world".to_string(),
            "Your appearance has changed".to_string(),
        ],
        companion_dialogue: Some(CompanionDialogue {
            elena: vec!["What... what have we done?".to_string()],
            marcus: vec!["The mission isn't over yet.".to_string()],
            quinn: vec!["The void... it's everywhere now.".to_string()],
        }),
    });
    
    // CORRUPTED HOMELAND (L80-90)
    levels.push(Level {
        id: 80,
        name: "Dragon's Echo".to_string(),
        description: "Face another fragment of the dragon you're becoming.".to_string(),
        level_type: LevelType::BossArena {
            boss_type: BossType::DragonFragment,
            arena_layout: ArenaLayout::VoidPlatforms,
            phases: 3,
        },
        dread_level: 3,
        mechanics_introduced: vec![GameMechanic::ElementalDamage],
        encounters: vec![],
        narrative_beats: vec![
            "The fragment recognizes you".to_string(),
            "It speaks with your father's voice".to_string(),
            "You absorb its power after defeating it".to_string(),
        ],
        companion_dialogue: None,
    });
    
    // VOID HERALD (L100)
    levels.push(Level {
        id: 100,
        name: "Herald of the End".to_string(),
        description: "The Void Herald emerges to test your resolve.".to_string(),
        level_type: LevelType::BossArena {
            boss_type: BossType::VoidHerald,
            arena_layout: ArenaLayout::ShiftingMaze,
            phases: 4,
        },
        dread_level: 4,
        mechanics_introduced: vec![],
        encounters: vec![],
        narrative_beats: vec![
            "The Herald shows you possible futures".to_string(),
            "All paths lead to becoming the dragon".to_string(),
            "Your only choice is how you'll use that power".to_string(),
        ],
        companion_dialogue: None,
    });
    
    // COMPANION BETRAYAL (L120)
    levels.push(Level {
        id: 120,
        name: "The Hardest Choice".to_string(),
        description: "Your companion becomes corrupted. You must face them.".to_string(),
        level_type: LevelType::BossArena {
            boss_type: BossType::FallenCompanion,
            arena_layout: ArenaLayout::CircularPit,
            phases: 3,
        },
        dread_level: 4,
        mechanics_introduced: vec![],
        encounters: vec![],
        narrative_beats: vec![
            "Companion consumed by void".to_string(),
            "They beg you to save them".to_string(),
            "Choice: Kill them or join them".to_string(),
        ],
        companion_dialogue: Some(CompanionDialogue {
            elena: vec!["Please... I can't control it anymore...".to_string()],
            marcus: vec!["This is what we trained for. Do it.".to_string()],
            quinn: vec!["We both knew this would happen...".to_string()],
        }),
    });
    
    levels
}
