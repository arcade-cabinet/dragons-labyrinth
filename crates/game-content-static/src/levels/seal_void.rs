//! Levels 121-180: Seal the Void
//! 
//! The final act where you must seal the void you opened,
//! becoming the very thing you sought to destroy.

use super::*;

/// Generate levels 121-180
pub fn get_levels() -> Vec<Level> {
    let mut levels = Vec::new();
    
    // POST-BETRAYAL (L121-130)
    levels.push(Level {
        id: 121,
        name: "Alone".to_string(),
        description: "With your companion gone, face the truth of your transformation.".to_string(),
        level_type: LevelType::HexOverworld {
            tiles: vec![],
            weather: WeatherType::VoidStorm,
            elevation_map: vec![vec![0; 10]; 10],
        },
        dread_level: 4,
        mechanics_introduced: vec![],
        encounters: vec![
            Encounter::Hazard {
                hazard_type: "Void Corruption".to_string(),
                damage_per_second: 5.0,
            }
        ],
        narrative_beats: vec![
            "Your body is changing".to_string(),
            "Scales appear on your skin".to_string(),
            "NPCs flee in terror".to_string(),
        ],
        companion_dialogue: None,  // Alone now
    });
    
    // MIRROR SELF (L140)
    levels.push(Level {
        id: 140,
        name: "The Mirror".to_string(),
        description: "Face yourself - or what you could have been.".to_string(),
        level_type: LevelType::BossArena {
            boss_type: BossType::MirrorSelf,
            arena_layout: ArenaLayout::BrokenBridge,
            phases: 5,
        },
        dread_level: 4,
        mechanics_introduced: vec![],
        encounters: vec![],
        narrative_beats: vec![
            "Fight your uncorrupted self".to_string(),
            "They use all your techniques".to_string(),
            "Victory means accepting corruption".to_string(),
        ],
        companion_dialogue: None,
    });
    
    // VOID DRAGON (L160)
    levels.push(Level {
        id: 160,
        name: "The Void Dragon".to_string(),
        description: "Face the void-corrupted version of the dragon.".to_string(),
        level_type: LevelType::BossArena {
            boss_type: BossType::VoidDragon,
            arena_layout: ArenaLayout::VoidPlatforms,
            phases: 6,
        },
        dread_level: 4,
        mechanics_introduced: vec![],
        encounters: vec![],
        narrative_beats: vec![
            "The void dragon is pure destruction".to_string(),
            "It shows you the end of all things".to_string(),
            "Defeating it requires becoming it".to_string(),
        ],
        companion_dialogue: None,
    });
    
    // TRUE ENDING (L180)
    levels.push(Level {
        id: 179,
        name: "The Final Labyrinth".to_string(),
        description: "Return to where it all began - the true labyrinth at world's heart.".to_string(),
        level_type: LevelType::Labyrinth3D {
            maze_seed: 179,
            corruption_level: 1.0,
            rooms: generate_final_labyrinth_rooms(),
        },
        dread_level: 4,
        mechanics_introduced: vec![],
        encounters: vec![],
        narrative_beats: vec![
            "The labyrinth recognizes its new master".to_string(),
            "Every room holds a memory".to_string(),
            "Your father's ghost guides you".to_string(),
        ],
        companion_dialogue: None,
    });
    
    levels.push(Level {
        id: 180,
        name: "The Dragon's Choice".to_string(),
        description: "Become the guardian. Seal the void. Accept your fate.".to_string(),
        level_type: LevelType::BossArena {
            boss_type: BossType::TrueDragon,
            arena_layout: ArenaLayout::CrumblingTower,
            phases: 7,
        },
        dread_level: 4,
        mechanics_introduced: vec![],
        encounters: vec![],
        narrative_beats: vec![
            "You ARE the true dragon now".to_string(),
            "Choice: Seal void (sacrifice self) or Rule (become tyrant)".to_string(),
            "The child from L3 watches with void-touched eyes".to_string(),
            "They will be the next hero to face you".to_string(),
        ],
        companion_dialogue: None,
    });
    
    levels
}

fn generate_final_labyrinth_rooms() -> Vec<RoomConfig> {
    vec![
        RoomConfig {
            id: 1,
            room_type: "memory_home".to_string(),
            size: (20, 20, 10),
            connections: vec![2],
            loot: vec!["Father's Letter".to_string()],
            enemies: vec![],
        },
        RoomConfig {
            id: 2,
            room_type: "memory_companion".to_string(),
            size: (15, 15, 8),
            connections: vec![1, 3],
            loot: vec!["Companion's Token".to_string()],
            enemies: vec![],
        },
        RoomConfig {
            id: 3,
            room_type: "memory_dragon".to_string(),
            size: (30, 30, 15),
            connections: vec![2, 4],
            loot: vec![],
            enemies: vec!["Echo of Dragon".to_string()],
        },
        RoomConfig {
            id: 4,
            room_type: "heart_of_void".to_string(),
            size: (40, 40, 20),
            connections: vec![3],
            loot: vec!["Crown of the Dragon".to_string()],
            enemies: vec![],
        },
    ]
}
