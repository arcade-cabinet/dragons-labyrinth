use bevy::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct Dungeon {
    pub name: String,
    pub dungeon_type: DungeonType,
    pub threat_level: u32,
    pub estimated_rooms: u32,
    pub complexity_level: ComplexityLevel,
    pub navigation_difficulty: NavigationDifficulty,
    pub corruption_influence: u32,
    pub exploration_difficulty: ExplorationDifficulty,
    pub threat_density: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DungeonType {
    Crypt,
    Cave,
    Temple,
    Lair,
    Hideout,
    Pit,
    Generic,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ComplexityLevel {
    Simple,
    Moderate,
    Complex,
    VeryComplex,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NavigationDifficulty {
    Trivial,
    Easy,
    Moderate,
    Hard,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExplorationDifficulty {
    Trivial,
    Easy,
    Moderate,
    Hard,
    Extreme,
    Nightmare,
}

impl Dungeon {
pub fn bowel_of_the_raging_pits() -> Self {
        Dungeon {
            name: "Bowel of the Raging Pits".to_string(),
            dungeon_type: DungeonType::Pit,
            threat_level: 5,
            estimated_rooms: 249,
            complexity_level: ComplexityLevel::VeryComplex,
            navigation_difficulty: NavigationDifficulty::Hard,
            corruption_influence: 1,
            exploration_difficulty: ExplorationDifficulty::Nightmare,
            threat_density: 491.6666666666667,
        }
    }
pub fn caverns_of_the_burning_souls() -> Self {
        Dungeon {
            name: "Caverns of the Burning Souls".to_string(),
            dungeon_type: DungeonType::Cave,
            threat_level: 5,
            estimated_rooms: 186,
            complexity_level: ComplexityLevel::VeryComplex,
            navigation_difficulty: NavigationDifficulty::Hard,
            corruption_influence: 1,
            exploration_difficulty: ExplorationDifficulty::Nightmare,
            threat_density: 516.6666666666667,
        }
    }
pub fn caverns_of_the_infernal_lich() -> Self {
        Dungeon {
            name: "Caverns of the Infernal Lich".to_string(),
            dungeon_type: DungeonType::Cave,
            threat_level: 5,
            estimated_rooms: 171,
            complexity_level: ComplexityLevel::VeryComplex,
            navigation_difficulty: NavigationDifficulty::Hard,
            corruption_influence: 3,
            exploration_difficulty: ExplorationDifficulty::Nightmare,
            threat_density: 627.2727272727273,
        }
    }
pub fn crypt_of_the_corrupted_order() -> Self {
        Dungeon {
            name: "Crypt of the Corrupted Order".to_string(),
            dungeon_type: DungeonType::Crypt,
            threat_level: 5,
            estimated_rooms: 663,
            complexity_level: ComplexityLevel::VeryComplex,
            navigation_difficulty: NavigationDifficulty::Hard,
            corruption_influence: 4,
            exploration_difficulty: ExplorationDifficulty::Nightmare,
            threat_density: 923.4782608695652,
        }
    }
pub fn crypt_of_the_infernal_blades() -> Self {
        Dungeon {
            name: "Crypt of the Infernal Blades".to_string(),
            dungeon_type: DungeonType::Crypt,
            threat_level: 5,
            estimated_rooms: 345,
            complexity_level: ComplexityLevel::VeryComplex,
            navigation_difficulty: NavigationDifficulty::Hard,
            corruption_influence: 4,
            exploration_difficulty: ExplorationDifficulty::Nightmare,
            threat_density: 729.2307692307692,
        }
    }
pub fn crypt_of_the_mourning_goblin() -> Self {
        Dungeon {
            name: "Crypt of the Mourning Goblin".to_string(),
            dungeon_type: DungeonType::Crypt,
            threat_level: 5,
            estimated_rooms: 1397,
            complexity_level: ComplexityLevel::VeryComplex,
            navigation_difficulty: NavigationDifficulty::Hard,
            corruption_influence: 2,
            exploration_difficulty: ExplorationDifficulty::Nightmare,
            threat_density: 753.9419087136929,
        }
    }
pub fn crypt_of_the_unholy_goblin() -> Self {
        Dungeon {
            name: "Crypt of the Unholy Goblin".to_string(),
            dungeon_type: DungeonType::Crypt,
            threat_level: 5,
            estimated_rooms: 268,
            complexity_level: ComplexityLevel::VeryComplex,
            navigation_difficulty: NavigationDifficulty::Hard,
            corruption_influence: 3,
            exploration_difficulty: ExplorationDifficulty::Nightmare,
            threat_density: 739.5833333333333,
        }
    }
pub fn crypt_of_the_violent_ogre() -> Self {
        Dungeon {
            name: "Crypt of the Violent Ogre".to_string(),
            dungeon_type: DungeonType::Crypt,
            threat_level: 5,
            estimated_rooms: 1034,
            complexity_level: ComplexityLevel::VeryComplex,
            navigation_difficulty: NavigationDifficulty::Hard,
            corruption_influence: 3,
            exploration_difficulty: ExplorationDifficulty::Nightmare,
            threat_density: 859.7701149425287,
        }
    }
pub fn hideout_of_the_corrupted_order() -> Self {
        Dungeon {
            name: "Hideout of the Corrupted Order".to_string(),
            dungeon_type: DungeonType::Hideout,
            threat_level: 5,
            estimated_rooms: 129,
            complexity_level: ComplexityLevel::VeryComplex,
            navigation_difficulty: NavigationDifficulty::Hard,
            corruption_influence: 3,
            exploration_difficulty: ExplorationDifficulty::Nightmare,
            threat_density: 486.9565217391305,
        }
    }
pub fn hideout_of_the_unspoken_desire() -> Self {
        Dungeon {
            name: "Hideout of the Unspoken Desire".to_string(),
            dungeon_type: DungeonType::Hideout,
            threat_level: 5,
            estimated_rooms: 274,
            complexity_level: ComplexityLevel::VeryComplex,
            navigation_difficulty: NavigationDifficulty::Hard,
            corruption_influence: 0,
            exploration_difficulty: ExplorationDifficulty::Nightmare,
            threat_density: 583.6734693877551,
        }
    }
pub fn lair_of_the_foresaken_desire() -> Self {
        Dungeon {
            name: "Lair of the Foresaken Desire".to_string(),
            dungeon_type: DungeonType::Lair,
            threat_level: 5,
            estimated_rooms: 235,
            complexity_level: ComplexityLevel::VeryComplex,
            navigation_difficulty: NavigationDifficulty::Hard,
            corruption_influence: 0,
            exploration_difficulty: ExplorationDifficulty::Nightmare,
            threat_density: 643.1818181818181,
        }
    }
pub fn lair_of_the_mourning_hopes() -> Self {
        Dungeon {
            name: "Lair of the Mourning Hopes".to_string(),
            dungeon_type: DungeonType::Lair,
            threat_level: 5,
            estimated_rooms: 251,
            complexity_level: ComplexityLevel::VeryComplex,
            navigation_difficulty: NavigationDifficulty::Hard,
            corruption_influence: 1,
            exploration_difficulty: ExplorationDifficulty::Nightmare,
            threat_density: 595.4545454545454,
        }
    }
pub fn shrine_of_the_infernal_blades() -> Self {
        Dungeon {
            name: "Shrine of the Infernal Blades".to_string(),
            dungeon_type: DungeonType::Temple,
            threat_level: 5,
            estimated_rooms: 95,
            complexity_level: ComplexityLevel::VeryComplex,
            navigation_difficulty: NavigationDifficulty::Hard,
            corruption_influence: 3,
            exploration_difficulty: ExplorationDifficulty::Nightmare,
            threat_density: 436.84210526315786,
        }
    }
pub fn shrine_of_the_infernal_desire() -> Self {
        Dungeon {
            name: "Shrine of the Infernal Desire".to_string(),
            dungeon_type: DungeonType::Temple,
            threat_level: 5,
            estimated_rooms: 283,
            complexity_level: ComplexityLevel::VeryComplex,
            navigation_difficulty: NavigationDifficulty::Hard,
            corruption_influence: 3,
            exploration_difficulty: ExplorationDifficulty::Nightmare,
            threat_density: 761.7021276595744,
        }
    }
pub fn temple_of_the_violent_ogre() -> Self {
        Dungeon {
            name: "Temple of the Violent Ogre".to_string(),
            dungeon_type: DungeonType::Temple,
            threat_level: 5,
            estimated_rooms: 264,
            complexity_level: ComplexityLevel::VeryComplex,
            navigation_difficulty: NavigationDifficulty::Hard,
            corruption_influence: 2,
            exploration_difficulty: ExplorationDifficulty::Nightmare,
            threat_density: 572.093023255814,
        }
    }
pub fn tomb_of_the_cursed_pits() -> Self {
        Dungeon {
            name: "Tomb of the Cursed Pits".to_string(),
            dungeon_type: DungeonType::Crypt,
            threat_level: 5,
            estimated_rooms: 530,
            complexity_level: ComplexityLevel::VeryComplex,
            navigation_difficulty: NavigationDifficulty::Hard,
            corruption_influence: 4,
            exploration_difficulty: ExplorationDifficulty::Nightmare,
            threat_density: 812.7659574468086,
        }
    }
pub fn tomb_of_the_grey_ogre() -> Self {
        Dungeon {
            name: "Tomb of the Grey Ogre".to_string(),
            dungeon_type: DungeonType::Crypt,
            threat_level: 5,
            estimated_rooms: 305,
            complexity_level: ComplexityLevel::VeryComplex,
            navigation_difficulty: NavigationDifficulty::Hard,
            corruption_influence: 1,
            exploration_difficulty: ExplorationDifficulty::Nightmare,
            threat_density: 620.0,
        }
    }
pub fn tomb_of_the_unspoken_skeletons() -> Self {
        Dungeon {
            name: "Tomb of the Unspoken Skeletons".to_string(),
            dungeon_type: DungeonType::Crypt,
            threat_level: 5,
            estimated_rooms: 350,
            complexity_level: ComplexityLevel::VeryComplex,
            navigation_difficulty: NavigationDifficulty::Hard,
            corruption_influence: 1,
            exploration_difficulty: ExplorationDifficulty::Nightmare,
            threat_density: 752.9411764705882,
        }
    }
}

// Dungeon-specific systems
pub fn spawn_dungeons(mut commands: Commands) {
commands.spawn((
        Dungeon::bowel_of_the_raging_pits(),
        HexTile { 
            q: 37, 
            r: 31, 
            biome: "wet_meadow".to_string(), 
            distance_band: "dread".to_string() 
        },
    ));
commands.spawn((
        Dungeon::caverns_of_the_burning_souls(),
        HexTile { 
            q: 36, 
            r: 32, 
            biome: "ashen_forest".to_string(), 
            distance_band: "dread".to_string() 
        },
    ));
commands.spawn((
        Dungeon::caverns_of_the_infernal_lich(),
        HexTile { 
            q: 47, 
            r: 33, 
            biome: "rust_plains".to_string(), 
            distance_band: "terror".to_string() 
        },
    ));
commands.spawn((
        Dungeon::crypt_of_the_corrupted_order(),
        HexTile { 
            q: 47, 
            r: 34, 
            biome: "bone_forest".to_string(), 
            distance_band: "terror".to_string() 
        },
    ));
commands.spawn((
        Dungeon::crypt_of_the_infernal_blades(),
        HexTile { 
            q: 47, 
            r: 38, 
            biome: "bone_forest".to_string(), 
            distance_band: "terror".to_string() 
        },
    ));
commands.spawn((
        Dungeon::crypt_of_the_mourning_goblin(),
        HexTile { 
            q: 30, 
            r: 25, 
            biome: "black_swamp".to_string(), 
            distance_band: "dread".to_string() 
        },
    ));
commands.spawn((
        Dungeon::crypt_of_the_unholy_goblin(),
        HexTile { 
            q: 47, 
            r: 44, 
            biome: "bone_forest".to_string(), 
            distance_band: "terror".to_string() 
        },
    ));
commands.spawn((
        Dungeon::crypt_of_the_violent_ogre(),
        HexTile { 
            q: 48, 
            r: 33, 
            biome: "bone_forest".to_string(), 
            distance_band: "terror".to_string() 
        },
    ));
commands.spawn((
        Dungeon::hideout_of_the_corrupted_order(),
        HexTile { 
            q: 45, 
            r: 45, 
            biome: "rust_plains".to_string(), 
            distance_band: "terror".to_string() 
        },
    ));
commands.spawn((
        Dungeon::hideout_of_the_unspoken_desire(),
        HexTile { 
            q: 32, 
            r: 28, 
            biome: "wet_meadow".to_string(), 
            distance_band: "dread".to_string() 
        },
    ));
commands.spawn((
        Dungeon::lair_of_the_foresaken_desire(),
        HexTile { 
            q: 30, 
            r: 23, 
            biome: "ashen_forest".to_string(), 
            distance_band: "dread".to_string() 
        },
    ));
commands.spawn((
        Dungeon::lair_of_the_mourning_hopes(),
        HexTile { 
            q: 37, 
            r: 28, 
            biome: "ashen_forest".to_string(), 
            distance_band: "dread".to_string() 
        },
    ));
commands.spawn((
        Dungeon::shrine_of_the_infernal_blades(),
        HexTile { 
            q: 41, 
            r: 42, 
            biome: "fungal_cathedral".to_string(), 
            distance_band: "terror".to_string() 
        },
    ));
commands.spawn((
        Dungeon::shrine_of_the_infernal_desire(),
        HexTile { 
            q: 44, 
            r: 33, 
            biome: "fungal_cathedral".to_string(), 
            distance_band: "terror".to_string() 
        },
    ));
commands.spawn((
        Dungeon::temple_of_the_violent_ogre(),
        HexTile { 
            q: 37, 
            r: 34, 
            biome: "fungal_cathedral".to_string(), 
            distance_band: "terror".to_string() 
        },
    ));
commands.spawn((
        Dungeon::tomb_of_the_cursed_pits(),
        HexTile { 
            q: 51, 
            r: 34, 
            biome: "bone_forest".to_string(), 
            distance_band: "terror".to_string() 
        },
    ));
commands.spawn((
        Dungeon::tomb_of_the_grey_ogre(),
        HexTile { 
            q: 34, 
            r: 25, 
            biome: "wet_meadow".to_string(), 
            distance_band: "dread".to_string() 
        },
    ));
commands.spawn((
        Dungeon::tomb_of_the_unspoken_skeletons(),
        HexTile { 
            q: 42, 
            r: 38, 
            biome: "wet_meadow".to_string(), 
            distance_band: "terror".to_string() 
        },
    ));
}

pub fn update_dungeon_threat_levels(
    mut dungeons: Query<&mut Dungeon>,
    player_distance: Res<PlayerDistance>,
) {
    for mut dungeon in dungeons.iter_mut() {
        // Dungeons become more dangerous as player moves toward dragon
        let distance_multiplier = player_distance.threat_multiplier();
        let base_threat = dungeon.threat_level;
        
        // Apply distance-based threat scaling
        if distance_multiplier > 1.0 {
            // Dungeon threat increases with corruption levels
        }
    }
}

pub fn generate_dungeon_encounters(
    dungeons: Query<&Dungeon>,
    mut encounter_events: EventWriter<EncounterSpawnEvent>,
) {
    for dungeon in dungeons.iter() {
        // Generate encounters based on dungeon characteristics
        let encounter_rate = dungeon.threat_density * dungeon.threat_level as f32;
        
        if encounter_rate > 5.0 {
            encounter_events.send(EncounterSpawnEvent {
                dungeon_name: dungeon.name.clone(),
                threat_level: dungeon.threat_level,
                encounter_type: match dungeon.dungeon_type {
                    DungeonType::Crypt => EncounterType::Undead,
                    DungeonType::Temple => EncounterType::Cultist,
                    DungeonType::Lair => EncounterType::Beast,
                    _ => EncounterType::Generic,
                },
            });
        }
    }
}

#[derive(Event)]
pub struct EncounterSpawnEvent {
    pub dungeon_name: String,
    pub threat_level: u32,
    pub encounter_type: EncounterType,
}

#[derive(Debug, Clone)]
pub enum EncounterType {
    Undead,
    Cultist,
    Beast,
    Generic,
}