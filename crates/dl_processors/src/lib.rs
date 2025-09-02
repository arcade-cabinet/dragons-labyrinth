//! Processor crate for Dragon's Labyrinth
//! 
//! This crate takes the analyzed data from dl_analysis and generates
//! Rust code for the game to use.

use bevy::prelude::*;
use std::path::PathBuf;

/// Components used by the generated code
pub mod components {
    use bevy::prelude::*;
    
    // Hex tile components
    #[derive(Component, Debug, Clone)]
    pub struct HexPosition {
        pub q: i32,
        pub r: i32,
    }
    
    #[derive(Component, Debug, Clone)]
    pub enum HexBiome {
        WetMeadow,
        AshenForest,
        FloodedVillage,
        BlackSwamp,
        FungalCathedral,
        ShadowedFen,
        RustPlains,
        HollowHills,
        CorrodedBattleground,
        FamineFields,
        BoneForest,
        DesolateExpanse,
        DragonScar,
        AbyssalChasm,
        FinalDreadTerrain,
    }
    
    #[derive(Component, Debug, Clone)]
    pub struct HexFeatures {
        pub features: Vec<String>,
    }
    
    #[derive(Component, Debug, Clone)]
    pub struct HexId(pub String);
    
    // Settlement components
    #[derive(Component, Debug, Clone)]
    pub struct SettlementPosition {
        pub q: i32,
        pub r: i32,
    }
    
    #[derive(Component, Debug, Clone)]
    pub struct SettlementName(pub String);
    
    #[derive(Component, Debug, Clone)]
    pub struct SettlementRegion(pub String);
    
    #[derive(Component, Debug, Clone)]
    pub struct Population(pub u32);
    
    #[derive(Component, Debug, Clone)]
    pub struct ThreatLevel(pub String);
    
    #[derive(Component, Debug, Clone)]
    pub struct SettlementFeatures {
        pub features: Vec<String>,
    }
    
    #[derive(Component, Debug, Clone)]
    pub struct SettlementId(pub String);
    
    // Dungeon components
    #[derive(Component, Debug, Clone)]
    pub struct DungeonLevel {
        pub level: u32,
    }
    
    #[derive(Component, Debug, Clone)]
    pub struct DungeonRooms {
        pub count: u32,
    }
    
    #[derive(Component, Debug, Clone)]
    pub struct EncounterDensity(pub String);
    
    #[derive(Component, Debug, Clone)]
    pub struct TreasureLevel(pub String);
    
    #[derive(Component, Debug, Clone)]
    pub struct DungeonAreaId(pub String);
    
    // Static data types
    #[derive(Debug, Clone)]
    pub struct HexData {
        pub uuid: &'static str,
        pub q: i32,
        pub r: i32,
        pub biome: &'static str,
    }
    
    #[derive(Debug, Clone)]
    pub struct SettlementData {
        pub uuid: &'static str,
        pub name: &'static str,
        pub location: (i32, i32),
        pub region: &'static str,
        pub population_estimate: u32,
        pub threat_level: &'static str,
    }
    
    #[derive(Debug, Clone)]
    pub struct DungeonAreaData {
        pub uuid: &'static str,
        pub level: u32,
        pub room_count: u32,
        pub encounter_density: &'static str,
        pub treasure_assessment: &'static str,
    }
}

/// Get the path to the generated code
pub fn generated_dir() -> PathBuf {
    PathBuf::from(env!("OUT_DIR"))
}

/// Include the generated world module
/// This is used by apps/game to include all the generated code
#[macro_export]
macro_rules! include_generated_world {
    () => {
        include!(concat!(env!("OUT_DIR"), "/mod.rs"));
    };
}
