use bevy::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct Region {
    pub name: String,
    pub dominant_biome: BiomeType,
    pub biome_distribution: Vec<(BiomeType, u32)>,
    pub corruption_level: u32,
    pub total_hexes: u32,
    pub settlement_density: f32,
    pub connectivity_score: f32,
    pub infrastructure_features: InfrastructureFeatures,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BiomeType {
    WetMeadow,
    AshenForest,
    FloodedVillage,
    BlackSwamp,
    FungalCathedral,
    RustPlains,
    FamineFields,
    BoneForest,
    DragonScar,
    AbyssalChasm,
    // Legacy biomes
    Forest,
    Desert,
    Mountain,
    Plains,
    Swamp,
    Tundra,
}

#[derive(Debug, Clone)]
pub struct InfrastructureFeatures {
    pub rivers: u32,
    pub trails: u32,
    pub harbors: u32,
    pub borders: u32,
    pub bridges: u32,
    pub roads: u32,
}

impl Region {
pub fn aurora_bushes() -> Self {
        Region {
            name: "Aurora Bushes".to_string(),
            dominant_biome: BiomeType::AshenForest,
            biome_distribution: vec![(BiomeType::WetMeadow, 6), (BiomeType::AshenForest, 46), (BiomeType::FloodedVillage, 35), (BiomeType::BlackSwamp, 33), (BiomeType::AbyssalChasm, 12), (BiomeType::Forest, 21), (BiomeType::Forest, 28), (BiomeType::Forest, 17), (BiomeType::Forest, 3), (BiomeType::Forest, 28), (BiomeType::FamineFields, 9), (BiomeType::BoneForest, 13), (BiomeType::RustPlains, 4), (BiomeType::DragonScar, 4), (BiomeType::FungalCathedral, 4), (BiomeType::Forest, 2)],
            corruption_level: 0,
            total_hexes: 28,
            settlement_density: 0.0,
            connectivity_score: 1.0,
            infrastructure_features: InfrastructureFeatures {
                rivers: 7,
                trails: 38,
                harbors: 36,
                borders: 2,
                bridges: 30,
                roads: 0,
            },
        }
    }
pub fn black_shield_timberlands() -> Self {
        Region {
            name: "Black Shield Timberlands".to_string(),
            dominant_biome: BiomeType::AshenForest,
            biome_distribution: vec![(BiomeType::WetMeadow, 4), (BiomeType::AshenForest, 60), (BiomeType::FloodedVillage, 14), (BiomeType::BlackSwamp, 60), (BiomeType::RustPlains, 6), (BiomeType::FamineFields, 9), (BiomeType::BoneForest, 7), (BiomeType::DragonScar, 5), (BiomeType::AbyssalChasm, 20), (BiomeType::Forest, 28), (BiomeType::Forest, 35), (BiomeType::Forest, 16), (BiomeType::Forest, 4), (BiomeType::Forest, 35), (BiomeType::FungalCathedral, 15)],
            corruption_level: 2,
            total_hexes: 35,
            settlement_density: 0.05714285714285714,
            connectivity_score: 1.0,
            infrastructure_features: InfrastructureFeatures {
                rivers: 9,
                trails: 43,
                harbors: 34,
                borders: 1,
                bridges: 35,
                roads: 1,
            },
        }
    }
pub fn blood_blade_fields() -> Self {
        Region {
            name: "Blood Blade Fields".to_string(),
            dominant_biome: BiomeType::WetMeadow,
            biome_distribution: vec![(BiomeType::WetMeadow, 64), (BiomeType::AshenForest, 42), (BiomeType::BlackSwamp, 44), (BiomeType::RustPlains, 7), (BiomeType::BoneForest, 9), (BiomeType::DragonScar, 6), (BiomeType::AbyssalChasm, 20), (BiomeType::Forest, 15), (BiomeType::Forest, 39), (BiomeType::Forest, 30), (BiomeType::Forest, 39), (BiomeType::Forest, 39), (BiomeType::FloodedVillage, 9), (BiomeType::FamineFields, 12), (BiomeType::FungalCathedral, 11)],
            corruption_level: 3,
            total_hexes: 39,
            settlement_density: 0.05128205128205128,
            connectivity_score: 1.0,
            infrastructure_features: InfrastructureFeatures {
                rivers: 5,
                trails: 52,
                harbors: 46,
                borders: 0,
                bridges: 43,
                roads: 0,
            },
        }
    }
pub fn bonecrusher_plains() -> Self {
        Region {
            name: "Bonecrusher Plains".to_string(),
            dominant_biome: BiomeType::WetMeadow,
            biome_distribution: vec![(BiomeType::WetMeadow, 35), (BiomeType::AshenForest, 27), (BiomeType::FloodedVillage, 9), (BiomeType::BlackSwamp, 25), (BiomeType::FungalCathedral, 3), (BiomeType::RustPlains, 7), (BiomeType::FamineFields, 8), (BiomeType::BoneForest, 26), (BiomeType::DragonScar, 6), (BiomeType::AbyssalChasm, 7), (BiomeType::Forest, 7), (BiomeType::Forest, 23), (BiomeType::Forest, 16), (BiomeType::Forest, 23), (BiomeType::Forest, 3), (BiomeType::Forest, 23)],
            corruption_level: 3,
            total_hexes: 23,
            settlement_density: 0.0,
            connectivity_score: 1.0,
            infrastructure_features: InfrastructureFeatures {
                rivers: 3,
                trails: 28,
                harbors: 26,
                borders: 2,
                bridges: 24,
                roads: 1,
            },
        }
    }
pub fn darkfall_dunes() -> Self {
        Region {
            name: "Darkfall Dunes".to_string(),
            dominant_biome: BiomeType::BlackSwamp,
            biome_distribution: vec![(BiomeType::AshenForest, 37), (BiomeType::BlackSwamp, 50), (BiomeType::FamineFields, 8), (BiomeType::BoneForest, 16), (BiomeType::AbyssalChasm, 16), (BiomeType::Forest, 13), (BiomeType::Forest, 34), (BiomeType::Forest, 34), (BiomeType::FungalCathedral, 16), (BiomeType::RustPlains, 7), (BiomeType::DragonScar, 17), (BiomeType::FloodedVillage, 11), (BiomeType::Forest, 12), (BiomeType::WetMeadow, 3), (BiomeType::Forest, 2), (BiomeType::Forest, 3)],
            corruption_level: 2,
            total_hexes: 34,
            settlement_density: 0.0,
            connectivity_score: 1.0,
            infrastructure_features: InfrastructureFeatures {
                rivers: 1,
                trails: 36,
                harbors: 39,
                borders: 2,
                bridges: 35,
                roads: 0,
            },
        }
    }
pub fn darkfall_plains() -> Self {
        Region {
            name: "Darkfall Plains".to_string(),
            dominant_biome: BiomeType::BlackSwamp,
            biome_distribution: vec![(BiomeType::WetMeadow, 32), (BiomeType::AshenForest, 28), (BiomeType::FloodedVillage, 11), (BiomeType::BlackSwamp, 33), (BiomeType::RustPlains, 6), (BiomeType::FamineFields, 5), (BiomeType::BoneForest, 8), (BiomeType::DragonScar, 11), (BiomeType::AbyssalChasm, 9), (BiomeType::Forest, 10), (BiomeType::Forest, 20), (BiomeType::Forest, 15), (BiomeType::Forest, 20), (BiomeType::Forest, 20), (BiomeType::FungalCathedral, 5), (BiomeType::Forest, 3)],
            corruption_level: 2,
            total_hexes: 20,
            settlement_density: 0.1,
            connectivity_score: 1.0,
            infrastructure_features: InfrastructureFeatures {
                rivers: 4,
                trails: 25,
                harbors: 22,
                borders: 1,
                bridges: 20,
                roads: 0,
            },
        }
    }
pub fn fearless_wilds() -> Self {
        Region {
            name: "Fearless Wilds".to_string(),
            dominant_biome: BiomeType::AshenForest,
            biome_distribution: vec![(BiomeType::AshenForest, 49), (BiomeType::FloodedVillage, 48), (BiomeType::BlackSwamp, 34), (BiomeType::Forest, 37), (BiomeType::Forest, 37), (BiomeType::AbyssalChasm, 13), (BiomeType::Forest, 23), (BiomeType::Forest, 15), (BiomeType::FungalCathedral, 15), (BiomeType::FamineFields, 5), (BiomeType::BoneForest, 9), (BiomeType::DragonScar, 1), (BiomeType::RustPlains, 3), (BiomeType::WetMeadow, 3), (BiomeType::Forest, 2)],
            corruption_level: 2,
            total_hexes: 37,
            settlement_density: 0.05405405405405406,
            connectivity_score: 1.0,
            infrastructure_features: InfrastructureFeatures {
                rivers: 10,
                trails: 52,
                harbors: 40,
                borders: 0,
                bridges: 41,
                roads: 1,
            },
        }
    }
pub fn firefly_cliffs() -> Self {
        Region {
            name: "Firefly Cliffs".to_string(),
            dominant_biome: BiomeType::AshenForest,
            biome_distribution: vec![(BiomeType::AshenForest, 36), (BiomeType::FloodedVillage, 11), (BiomeType::BlackSwamp, 36), (BiomeType::BoneForest, 5), (BiomeType::DragonScar, 8), (BiomeType::Forest, 9), (BiomeType::Forest, 34), (BiomeType::Forest, 3), (BiomeType::Forest, 34), (BiomeType::AbyssalChasm, 25), (BiomeType::Forest, 30), (BiomeType::FamineFields, 6), (BiomeType::FungalCathedral, 11), (BiomeType::RustPlains, 5), (BiomeType::WetMeadow, 4), (BiomeType::Forest, 3)],
            corruption_level: 0,
            total_hexes: 34,
            settlement_density: 0.058823529411764705,
            connectivity_score: 1.0,
            infrastructure_features: InfrastructureFeatures {
                rivers: 2,
                trails: 57,
                harbors: 33,
                borders: 10,
                bridges: 41,
                roads: 2,
            },
        }
    }
pub fn goblinchaser_jungle() -> Self {
        Region {
            name: "Goblinchaser Jungle".to_string(),
            dominant_biome: BiomeType::FloodedVillage,
            biome_distribution: vec![(BiomeType::WetMeadow, 1), (BiomeType::AshenForest, 36), (BiomeType::FloodedVillage, 40), (BiomeType::BlackSwamp, 29), (BiomeType::RustPlains, 2), (BiomeType::FamineFields, 11), (BiomeType::BoneForest, 7), (BiomeType::DragonScar, 4), (BiomeType::AbyssalChasm, 14), (BiomeType::Forest, 18), (BiomeType::Forest, 28), (BiomeType::Forest, 11), (BiomeType::Forest, 1), (BiomeType::Forest, 28), (BiomeType::FungalCathedral, 13)],
            corruption_level: 0,
            total_hexes: 28,
            settlement_density: 0.0,
            connectivity_score: 1.0,
            infrastructure_features: InfrastructureFeatures {
                rivers: 6,
                trails: 38,
                harbors: 29,
                borders: 1,
                bridges: 30,
                roads: 1,
            },
        }
    }
pub fn goblinchaser_wilderness() -> Self {
        Region {
            name: "Goblinchaser Wilderness".to_string(),
            dominant_biome: BiomeType::FloodedVillage,
            biome_distribution: vec![(BiomeType::WetMeadow, 1), (BiomeType::AshenForest, 20), (BiomeType::FloodedVillage, 24), (BiomeType::BlackSwamp, 16), (BiomeType::RustPlains, 2), (BiomeType::FamineFields, 3), (BiomeType::BoneForest, 4), (BiomeType::DragonScar, 5), (BiomeType::AbyssalChasm, 8), (BiomeType::Forest, 10), (BiomeType::Forest, 16), (BiomeType::Forest, 9), (BiomeType::Forest, 1), (BiomeType::Forest, 16), (BiomeType::FungalCathedral, 8)],
            corruption_level: 0,
            total_hexes: 16,
            settlement_density: 0.0,
            connectivity_score: 1.0,
            infrastructure_features: InfrastructureFeatures {
                rivers: 5,
                trails: 21,
                harbors: 14,
                borders: 1,
                bridges: 17,
                roads: 2,
            },
        }
    }
pub fn goldenswan_timberlands() -> Self {
        Region {
            name: "Goldenswan Timberlands".to_string(),
            dominant_biome: BiomeType::AshenForest,
            biome_distribution: vec![(BiomeType::AshenForest, 57), (BiomeType::FloodedVillage, 13), (BiomeType::BlackSwamp, 29), (BiomeType::FamineFields, 12), (BiomeType::DragonScar, 7), (BiomeType::Forest, 26), (BiomeType::Forest, 32), (BiomeType::Forest, 16), (BiomeType::Forest, 32), (BiomeType::FungalCathedral, 16), (BiomeType::AbyssalChasm, 16), (BiomeType::WetMeadow, 2), (BiomeType::RustPlains, 3), (BiomeType::BoneForest, 7), (BiomeType::Forest, 2), (BiomeType::Forest, 1)],
            corruption_level: 0,
            total_hexes: 32,
            settlement_density: 0.0,
            connectivity_score: 1.0,
            infrastructure_features: InfrastructureFeatures {
                rivers: 8,
                trails: 43,
                harbors: 34,
                borders: 2,
                bridges: 35,
                roads: 2,
            },
        }
    }
pub fn goldseekers_cliffs() -> Self {
        Region {
            name: "Goldseeker's Cliffs".to_string(),
            dominant_biome: BiomeType::BlackSwamp,
            biome_distribution: vec![(BiomeType::AshenForest, 27), (BiomeType::FloodedVillage, 9), (BiomeType::BlackSwamp, 29), (BiomeType::AbyssalChasm, 25), (BiomeType::Forest, 27), (BiomeType::Forest, 26), (BiomeType::Forest, 27), (BiomeType::FungalCathedral, 8), (BiomeType::BoneForest, 2), (BiomeType::DragonScar, 5), (BiomeType::Forest, 7), (BiomeType::FamineFields, 1), (BiomeType::WetMeadow, 2), (BiomeType::Forest, 2)],
            corruption_level: 0,
            total_hexes: 27,
            settlement_density: 0.0,
            connectivity_score: 1.0,
            infrastructure_features: InfrastructureFeatures {
                rivers: 7,
                trails: 49,
                harbors: 26,
                borders: 11,
                bridges: 33,
                roads: 1,
            },
        }
    }
pub fn grey_mist_snowlands() -> Self {
        Region {
            name: "Grey Mist Snowlands".to_string(),
            dominant_biome: BiomeType::BlackSwamp,
            biome_distribution: vec![(BiomeType::AshenForest, 30), (BiomeType::FloodedVillage, 9), (BiomeType::BlackSwamp, 32), (BiomeType::Forest, 32), (BiomeType::Forest, 32), (BiomeType::BoneForest, 7), (BiomeType::Forest, 6), (BiomeType::FamineFields, 7), (BiomeType::Forest, 1), (BiomeType::FungalCathedral, 3), (BiomeType::DragonScar, 4), (BiomeType::AbyssalChasm, 8), (BiomeType::WetMeadow, 5), (BiomeType::Forest, 5), (BiomeType::Forest, 3)],
            corruption_level: 1,
            total_hexes: 32,
            settlement_density: 0.0625,
            connectivity_score: 1.0,
            infrastructure_features: InfrastructureFeatures {
                rivers: 2,
                trails: 41,
                harbors: 33,
                borders: 0,
                bridges: 36,
                roads: 2,
            },
        }
    }
pub fn heartseeker_forest() -> Self {
        Region {
            name: "Heartseeker Forest".to_string(),
            dominant_biome: BiomeType::AshenForest,
            biome_distribution: vec![(BiomeType::AshenForest, 88), (BiomeType::FloodedVillage, 16), (BiomeType::BlackSwamp, 31), (BiomeType::FamineFields, 12), (BiomeType::BoneForest, 13), (BiomeType::DragonScar, 8), (BiomeType::AbyssalChasm, 15), (BiomeType::Forest, 32), (BiomeType::Forest, 32), (BiomeType::Forest, 17), (BiomeType::Forest, 32), (BiomeType::WetMeadow, 2), (BiomeType::Forest, 2), (BiomeType::Forest, 2), (BiomeType::FungalCathedral, 13), (BiomeType::RustPlains, 3)],
            corruption_level: 0,
            total_hexes: 32,
            settlement_density: 0.0,
            connectivity_score: 1.0,
            infrastructure_features: InfrastructureFeatures {
                rivers: 8,
                trails: 39,
                harbors: 36,
                borders: 3,
                bridges: 33,
                roads: 2,
            },
        }
    }
pub fn heartseeker_moors() -> Self {
        Region {
            name: "Heartseeker Moors".to_string(),
            dominant_biome: BiomeType::FloodedVillage,
            biome_distribution: vec![(BiomeType::AshenForest, 11), (BiomeType::FloodedVillage, 35), (BiomeType::BlackSwamp, 26), (BiomeType::Forest, 19), (BiomeType::Forest, 19), (BiomeType::BoneForest, 5), (BiomeType::Forest, 8), (BiomeType::Forest, 6), (BiomeType::AbyssalChasm, 3), (BiomeType::FungalCathedral, 8), (BiomeType::Forest, 5), (BiomeType::DragonScar, 1), (BiomeType::FamineFields, 2)],
            corruption_level: 0,
            total_hexes: 19,
            settlement_density: 0.10526315789473684,
            connectivity_score: 1.0,
            infrastructure_features: InfrastructureFeatures {
                rivers: 3,
                trails: 21,
                harbors: 16,
                borders: 0,
                bridges: 21,
                roads: 0,
            },
        }
    }
pub fn hells_gate_desert() -> Self {
        Region {
            name: "Hell's Gate Desert".to_string(),
            dominant_biome: BiomeType::BlackSwamp,
            biome_distribution: vec![(BiomeType::AshenForest, 30), (BiomeType::FloodedVillage, 4), (BiomeType::BlackSwamp, 46), (BiomeType::RustPlains, 3), (BiomeType::FamineFields, 4), (BiomeType::BoneForest, 12), (BiomeType::AbyssalChasm, 14), (BiomeType::Forest, 7), (BiomeType::Forest, 45), (BiomeType::Forest, 13), (BiomeType::Forest, 45), (BiomeType::WetMeadow, 2), (BiomeType::Forest, 2), (BiomeType::FungalCathedral, 29), (BiomeType::DragonScar, 7)],
            corruption_level: 4,
            total_hexes: 45,
            settlement_density: 0.0,
            connectivity_score: 1.0,
            infrastructure_features: InfrastructureFeatures {
                rivers: 0,
                trails: 56,
                harbors: 45,
                borders: 2,
                bridges: 46,
                roads: 4,
            },
        }
    }
pub fn holloweye_wilderness() -> Self {
        Region {
            name: "Holloweye Wilderness".to_string(),
            dominant_biome: BiomeType::AshenForest,
            biome_distribution: vec![(BiomeType::AshenForest, 19), (BiomeType::FloodedVillage, 16), (BiomeType::BlackSwamp, 12), (BiomeType::AbyssalChasm, 7), (BiomeType::Forest, 8), (BiomeType::Forest, 15), (BiomeType::Forest, 15), (BiomeType::FungalCathedral, 5), (BiomeType::Forest, 3), (BiomeType::BoneForest, 8), (BiomeType::DragonScar, 1), (BiomeType::RustPlains, 1)],
            corruption_level: 0,
            total_hexes: 15,
            settlement_density: 0.26666666666666666,
            connectivity_score: 1.0,
            infrastructure_features: InfrastructureFeatures {
                rivers: 2,
                trails: 16,
                harbors: 13,
                borders: 0,
                bridges: 15,
                roads: 0,
            },
        }
    }
pub fn iceborn_wilderness() -> Self {
        Region {
            name: "Iceborn Wilderness".to_string(),
            dominant_biome: BiomeType::AshenForest,
            biome_distribution: vec![(BiomeType::AshenForest, 35), (BiomeType::FloodedVillage, 33), (BiomeType::BlackSwamp, 26), (BiomeType::AbyssalChasm, 11), (BiomeType::Forest, 25), (BiomeType::Forest, 25), (BiomeType::DragonScar, 2), (BiomeType::Forest, 15), (BiomeType::Forest, 11), (BiomeType::BoneForest, 4), (BiomeType::FungalCathedral, 12), (BiomeType::FamineFields, 5), (BiomeType::RustPlains, 4)],
            corruption_level: 0,
            total_hexes: 25,
            settlement_density: 0.0,
            connectivity_score: 1.0,
            infrastructure_features: InfrastructureFeatures {
                rivers: 5,
                trails: 36,
                harbors: 30,
                borders: 0,
                bridges: 25,
                roads: 2,
            },
        }
    }
pub fn javelin_plains() -> Self {
        Region {
            name: "Javelin Plains".to_string(),
            dominant_biome: BiomeType::WetMeadow,
            biome_distribution: vec![(BiomeType::WetMeadow, 33), (BiomeType::RustPlains, 2), (BiomeType::Forest, 18), (BiomeType::Forest, 13), (BiomeType::Forest, 18), (BiomeType::Forest, 18), (BiomeType::BlackSwamp, 13), (BiomeType::AshenForest, 6), (BiomeType::FungalCathedral, 5), (BiomeType::FloodedVillage, 3), (BiomeType::AbyssalChasm, 4), (BiomeType::FamineFields, 1), (BiomeType::BoneForest, 2), (BiomeType::Forest, 1)],
            corruption_level: 0,
            total_hexes: 18,
            settlement_density: 0.0,
            connectivity_score: 1.0,
            infrastructure_features: InfrastructureFeatures {
                rivers: 3,
                trails: 24,
                harbors: 15,
                borders: 0,
                bridges: 19,
                roads: 0,
            },
        }
    }
pub fn javelin_wetlands() -> Self {
        Region {
            name: "Javelin Wetlands".to_string(),
            dominant_biome: BiomeType::BlackSwamp,
            biome_distribution: vec![(BiomeType::FloodedVillage, 33), (BiomeType::BlackSwamp, 53), (BiomeType::AbyssalChasm, 4), (BiomeType::Forest, 22), (BiomeType::Forest, 22), (BiomeType::Forest, 22), (BiomeType::AshenForest, 21), (BiomeType::FungalCathedral, 6), (BiomeType::BoneForest, 2), (BiomeType::FamineFields, 6), (BiomeType::Forest, 12), (BiomeType::RustPlains, 1), (BiomeType::Forest, 1), (BiomeType::DragonScar, 1)],
            corruption_level: 0,
            total_hexes: 22,
            settlement_density: 0.0,
            connectivity_score: 1.0,
            infrastructure_features: InfrastructureFeatures {
                rivers: 6,
                trails: 27,
                harbors: 21,
                borders: 0,
                bridges: 23,
                roads: 0,
            },
        }
    }
pub fn moonwatcher_wetlands() -> Self {
        Region {
            name: "Moonwatcher Wetlands".to_string(),
            dominant_biome: BiomeType::BlackSwamp,
            biome_distribution: vec![(BiomeType::AshenForest, 15), (BiomeType::BlackSwamp, 42), (BiomeType::FungalCathedral, 6), (BiomeType::Forest, 17), (BiomeType::Forest, 17), (BiomeType::Forest, 17), (BiomeType::FamineFields, 7), (BiomeType::BoneForest, 4), (BiomeType::Forest, 3), (BiomeType::FloodedVillage, 13), (BiomeType::Forest, 7), (BiomeType::DragonScar, 4), (BiomeType::AbyssalChasm, 2), (BiomeType::WetMeadow, 1), (BiomeType::Forest, 1), (BiomeType::RustPlains, 1)],
            corruption_level: 0,
            total_hexes: 17,
            settlement_density: 0.0,
            connectivity_score: 1.0,
            infrastructure_features: InfrastructureFeatures {
                rivers: 5,
                trails: 18,
                harbors: 13,
                borders: 0,
                bridges: 19,
                roads: 0,
            },
        }
    }
pub fn ragthorn_meadows() -> Self {
        Region {
            name: "Ragthorn Meadows".to_string(),
            dominant_biome: BiomeType::WetMeadow,
            biome_distribution: vec![(BiomeType::WetMeadow, 30), (BiomeType::BlackSwamp, 17), (BiomeType::Forest, 18), (BiomeType::Forest, 15), (BiomeType::Forest, 18), (BiomeType::Forest, 18), (BiomeType::AshenForest, 17), (BiomeType::Forest, 5), (BiomeType::FloodedVillage, 5), (BiomeType::FungalCathedral, 5), (BiomeType::AbyssalChasm, 10), (BiomeType::FamineFields, 4), (BiomeType::BoneForest, 5), (BiomeType::RustPlains, 1), (BiomeType::DragonScar, 1)],
            corruption_level: 0,
            total_hexes: 18,
            settlement_density: 0.0,
            connectivity_score: 1.0,
            infrastructure_features: InfrastructureFeatures {
                rivers: 3,
                trails: 23,
                harbors: 13,
                borders: 2,
                bridges: 20,
                roads: 1,
            },
        }
    }
pub fn ragthorn_woods() -> Self {
        Region {
            name: "Ragthorn Woods".to_string(),
            dominant_biome: BiomeType::AshenForest,
            biome_distribution: vec![(BiomeType::AshenForest, 39), (BiomeType::BlackSwamp, 17), (BiomeType::Forest, 20), (BiomeType::Forest, 20), (BiomeType::Forest, 20), (BiomeType::BoneForest, 5), (BiomeType::AbyssalChasm, 6), (BiomeType::FloodedVillage, 7), (BiomeType::Forest, 9), (BiomeType::FamineFields, 5), (BiomeType::FungalCathedral, 5), (BiomeType::DragonScar, 1), (BiomeType::RustPlains, 3)],
            corruption_level: 0,
            total_hexes: 20,
            settlement_density: 0.1,
            connectivity_score: 1.0,
            infrastructure_features: InfrastructureFeatures {
                rivers: 4,
                trails: 27,
                harbors: 17,
                borders: 1,
                bridges: 20,
                roads: 1,
            },
        }
    }
pub fn thunderwave_woodlands() -> Self {
        Region {
            name: "Thunderwave Woodlands".to_string(),
            dominant_biome: BiomeType::AshenForest,
            biome_distribution: vec![(BiomeType::AshenForest, 62), (BiomeType::BlackSwamp, 27), (BiomeType::FamineFields, 6), (BiomeType::Forest, 29), (BiomeType::Forest, 29), (BiomeType::Forest, 10), (BiomeType::Forest, 29), (BiomeType::RustPlains, 2), (BiomeType::BoneForest, 8), (BiomeType::FloodedVillage, 12), (BiomeType::FungalCathedral, 14), (BiomeType::AbyssalChasm, 7), (BiomeType::DragonScar, 3), (BiomeType::WetMeadow, 1), (BiomeType::Forest, 1)],
            corruption_level: 0,
            total_hexes: 29,
            settlement_density: 0.0,
            connectivity_score: 1.0,
            infrastructure_features: InfrastructureFeatures {
                rivers: 9,
                trails: 39,
                harbors: 31,
                borders: 0,
                bridges: 29,
                roads: 3,
            },
        }
    }
pub fn vicious_crags() -> Self {
        Region {
            name: "Vicious Crags".to_string(),
            dominant_biome: BiomeType::AshenForest,
            biome_distribution: vec![(BiomeType::AshenForest, 32), (BiomeType::FungalCathedral, 11), (BiomeType::FamineFields, 6), (BiomeType::Forest, 8), (BiomeType::Forest, 26), (BiomeType::Forest, 26), (BiomeType::Forest, 26), (BiomeType::BlackSwamp, 25), (BiomeType::DragonScar, 4), (BiomeType::AbyssalChasm, 20), (BiomeType::RustPlains, 2), (BiomeType::FloodedVillage, 2), (BiomeType::WetMeadow, 3), (BiomeType::Forest, 3), (BiomeType::BoneForest, 1)],
            corruption_level: 4,
            total_hexes: 26,
            settlement_density: 0.0,
            connectivity_score: 1.0,
            infrastructure_features: InfrastructureFeatures {
                rivers: 0,
                trails: 46,
                harbors: 30,
                borders: 5,
                bridges: 35,
                roads: 4,
            },
        }
    }
}

// Region-specific systems
pub fn spawn_regions(mut commands: Commands) {
commands.spawn((
        Region::aurora_bushes(),
        BiomeSpawner {
            hex_spawn_count: 28,
            settlement_spawn_points: 0,
            corruption_intensity: 0.0,
        },
    ));
commands.spawn((
        Region::black_shield_timberlands(),
        BiomeSpawner {
            hex_spawn_count: 35,
            settlement_spawn_points: 2,
            corruption_intensity: 0.4,
        },
    ));
commands.spawn((
        Region::blood_blade_fields(),
        BiomeSpawner {
            hex_spawn_count: 39,
            settlement_spawn_points: 2,
            corruption_intensity: 0.6,
        },
    ));
commands.spawn((
        Region::bonecrusher_plains(),
        BiomeSpawner {
            hex_spawn_count: 23,
            settlement_spawn_points: 0,
            corruption_intensity: 0.6,
        },
    ));
commands.spawn((
        Region::darkfall_dunes(),
        BiomeSpawner {
            hex_spawn_count: 34,
            settlement_spawn_points: 0,
            corruption_intensity: 0.4,
        },
    ));
commands.spawn((
        Region::darkfall_plains(),
        BiomeSpawner {
            hex_spawn_count: 20,
            settlement_spawn_points: 2,
            corruption_intensity: 0.4,
        },
    ));
commands.spawn((
        Region::fearless_wilds(),
        BiomeSpawner {
            hex_spawn_count: 37,
            settlement_spawn_points: 2,
            corruption_intensity: 0.4,
        },
    ));
commands.spawn((
        Region::firefly_cliffs(),
        BiomeSpawner {
            hex_spawn_count: 34,
            settlement_spawn_points: 2,
            corruption_intensity: 0.0,
        },
    ));
commands.spawn((
        Region::goblinchaser_jungle(),
        BiomeSpawner {
            hex_spawn_count: 28,
            settlement_spawn_points: 0,
            corruption_intensity: 0.0,
        },
    ));
commands.spawn((
        Region::goblinchaser_wilderness(),
        BiomeSpawner {
            hex_spawn_count: 16,
            settlement_spawn_points: 0,
            corruption_intensity: 0.0,
        },
    ));
commands.spawn((
        Region::goldenswan_timberlands(),
        BiomeSpawner {
            hex_spawn_count: 32,
            settlement_spawn_points: 0,
            corruption_intensity: 0.0,
        },
    ));
commands.spawn((
        Region::goldseekers_cliffs(),
        BiomeSpawner {
            hex_spawn_count: 27,
            settlement_spawn_points: 0,
            corruption_intensity: 0.0,
        },
    ));
commands.spawn((
        Region::grey_mist_snowlands(),
        BiomeSpawner {
            hex_spawn_count: 32,
            settlement_spawn_points: 2,
            corruption_intensity: 0.2,
        },
    ));
commands.spawn((
        Region::heartseeker_forest(),
        BiomeSpawner {
            hex_spawn_count: 32,
            settlement_spawn_points: 0,
            corruption_intensity: 0.0,
        },
    ));
commands.spawn((
        Region::heartseeker_moors(),
        BiomeSpawner {
            hex_spawn_count: 19,
            settlement_spawn_points: 2,
            corruption_intensity: 0.0,
        },
    ));
commands.spawn((
        Region::hells_gate_desert(),
        BiomeSpawner {
            hex_spawn_count: 45,
            settlement_spawn_points: 0,
            corruption_intensity: 0.8,
        },
    ));
commands.spawn((
        Region::holloweye_wilderness(),
        BiomeSpawner {
            hex_spawn_count: 15,
            settlement_spawn_points: 4,
            corruption_intensity: 0.0,
        },
    ));
commands.spawn((
        Region::iceborn_wilderness(),
        BiomeSpawner {
            hex_spawn_count: 25,
            settlement_spawn_points: 0,
            corruption_intensity: 0.0,
        },
    ));
commands.spawn((
        Region::javelin_plains(),
        BiomeSpawner {
            hex_spawn_count: 18,
            settlement_spawn_points: 0,
            corruption_intensity: 0.0,
        },
    ));
commands.spawn((
        Region::javelin_wetlands(),
        BiomeSpawner {
            hex_spawn_count: 22,
            settlement_spawn_points: 0,
            corruption_intensity: 0.0,
        },
    ));
commands.spawn((
        Region::moonwatcher_wetlands(),
        BiomeSpawner {
            hex_spawn_count: 17,
            settlement_spawn_points: 0,
            corruption_intensity: 0.0,
        },
    ));
commands.spawn((
        Region::ragthorn_meadows(),
        BiomeSpawner {
            hex_spawn_count: 18,
            settlement_spawn_points: 0,
            corruption_intensity: 0.0,
        },
    ));
commands.spawn((
        Region::ragthorn_woods(),
        BiomeSpawner {
            hex_spawn_count: 20,
            settlement_spawn_points: 2,
            corruption_intensity: 0.0,
        },
    ));
commands.spawn((
        Region::thunderwave_woodlands(),
        BiomeSpawner {
            hex_spawn_count: 29,
            settlement_spawn_points: 0,
            corruption_intensity: 0.0,
        },
    ));
commands.spawn((
        Region::vicious_crags(),
        BiomeSpawner {
            hex_spawn_count: 26,
            settlement_spawn_points: 0,
            corruption_intensity: 0.8,
        },
    ));
}

pub fn update_regional_corruption(
    mut regions: Query<&mut Region>,
    player_distance: Res<PlayerDistance>,
) {
    for mut region in regions.iter_mut() {
        // Apply distance-based corruption to regions
        let base_corruption = region.corruption_level;
        let distance_corruption = player_distance.corruption_level();
        
        // Regions with high connectivity resist corruption better
        let resistance_factor = region.connectivity_score;
        let effective_corruption = distance_corruption as f32 * (1.0 - resistance_factor * 0.3);
        
        region.corruption_level = base_corruption.saturating_add(effective_corruption as u32);
    }
}

pub fn spawn_biome_hexes(
    regions: Query<&Region>,
    mut commands: Commands,
    mut spawned: Local<bool>,
) {
    if *spawned {
        return;
    }
    *spawned = true;
    
    for region in regions.iter() {
        // Spawn hex tiles based on region biome distribution
        for (biome_type, count) in &region.biome_distribution {
            for i in 0..*count {
                let hex_q = (i % 10) as i32;  // Distribute in 10x grid
                let hex_r = (i / 10) as i32;
                
                commands.spawn(HexTile {
                    q: hex_q,
                    r: hex_r,
                    biome: format!("{:?}", biome_type).to_lowercase(),
                    distance_band: calculate_distance_band(hex_q, hex_r),
                });
            }
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct BiomeSpawner {
    pub hex_spawn_count: u32,
    pub settlement_spawn_points: u32,
    pub corruption_intensity: f32,
}

fn calculate_distance_band(q: i32, r: i32) -> String {
    let distance = ((q * q + r * r + q * r) as f32).sqrt();
    match distance as u32 {
        0..=20 => "peace".to_string(),
        21..=40 => "unease".to_string(),
        41..=60 => "dread".to_string(),
        61..=120 => "terror".to_string(),
        _ => "horror".to_string(),
    }
}