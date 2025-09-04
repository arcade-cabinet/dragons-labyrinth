//! Region and biome definitions

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

// Re-export BiomeType from dl_types instead of defining our own
pub use dl_types::world::BiomeType;

impl Default for BiomeType {
    fn default() -> Self {
        Self::Grassland
    }
}

impl BiomeType {
    pub fn corruption_level(&self) -> f32 {
        match self {
            // Core biomes (low corruption)
            BiomeType::Grassland => 0.1,
            BiomeType::Forest => 0.2,
            BiomeType::Mountain => 0.15,
            BiomeType::Desert => 0.25,
            BiomeType::Swamp => 0.3,
            BiomeType::Water => 0.1,
            BiomeType::Snow => 0.2,
            BiomeType::Lava => 0.8,
            BiomeType::Void => 1.0,
            
            // Transitional biomes (moderate corruption)
            BiomeType::ForestGrassland => 0.15,
            BiomeType::MountainForest => 0.2,
            BiomeType::DesertMountain => 0.3,
            BiomeType::SwampWater => 0.35,
            BiomeType::SnowMountain => 0.25,
            
            // Corrupted biomes (high corruption)
            BiomeType::CorruptedGrassland => 0.6,
            BiomeType::CorruptedForest => 0.7,
            BiomeType::CorruptedMountain => 0.75,
            BiomeType::CorruptedDesert => 0.8,
            BiomeType::CorruptedSwamp => 0.85,
            BiomeType::CorruptedWater => 0.9,
            BiomeType::CorruptedSnow => 0.7,
            
            // Void-touched biomes (maximum corruption)
            BiomeType::VoidGrassland => 0.9,
            BiomeType::VoidForest => 0.95,
            BiomeType::VoidMountain => 0.85,
            BiomeType::VoidDesert => 0.9,
            BiomeType::VoidSwamp => 0.95,
            BiomeType::VoidWater => 1.0,
            BiomeType::VoidSnow => 0.9,
            BiomeType::VoidLava => 1.0,
        }
    }

    pub fn dread_multiplier(&self) -> f32 {
        match self {
            // Core biomes (base dread)
            BiomeType::Grassland => 1.0,
            BiomeType::Forest => 1.2,
            BiomeType::Mountain => 1.1,
            BiomeType::Desert => 1.3,
            BiomeType::Swamp => 1.5,
            BiomeType::Water => 1.0,
            BiomeType::Snow => 1.2,
            BiomeType::Lava => 2.5,
            BiomeType::Void => 4.0,
            
            // Transitional biomes (moderate dread)
            BiomeType::ForestGrassland => 1.1,
            BiomeType::MountainForest => 1.15,
            BiomeType::DesertMountain => 1.25,
            BiomeType::SwampWater => 1.4,
            BiomeType::SnowMountain => 1.2,
            
            // Corrupted biomes (high dread)
            BiomeType::CorruptedGrassland => 1.9,
            BiomeType::CorruptedForest => 2.2,
            BiomeType::CorruptedMountain => 2.0,
            BiomeType::CorruptedDesert => 2.3,
            BiomeType::CorruptedSwamp => 2.6,
            BiomeType::CorruptedWater => 2.8,
            BiomeType::CorruptedSnow => 2.1,
            
            // Void-touched biomes (extreme dread)
            BiomeType::VoidGrassland => 3.2,
            BiomeType::VoidForest => 3.5,
            BiomeType::VoidMountain => 3.0,
            BiomeType::VoidDesert => 3.3,
            BiomeType::VoidSwamp => 3.8,
            BiomeType::VoidWater => 4.0,
            BiomeType::VoidSnow => 3.1,
            BiomeType::VoidLava => 4.5,
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            // Core biomes  
            BiomeType::Grassland => "Peaceful Grassland",
            BiomeType::Forest => "Ancient Forest",
            BiomeType::Mountain => "Rocky Mountain",
            BiomeType::Desert => "Arid Desert",
            BiomeType::Swamp => "Murky Swamp",
            BiomeType::Water => "Clear Water",
            BiomeType::Snow => "Snow-Covered Peaks",
            BiomeType::Lava => "Lava Fields",
            BiomeType::Void => "Void Nexus",
            
            // Transitional biomes
            BiomeType::ForestGrassland => "Forest Meadow",
            BiomeType::MountainForest => "Mountain Forest",
            BiomeType::DesertMountain => "Desert Highlands",
            BiomeType::SwampWater => "Wetlands",
            BiomeType::SnowMountain => "Alpine Peaks",
            
            // Corrupted biomes
            BiomeType::CorruptedGrassland => "Withered Grassland",
            BiomeType::CorruptedForest => "Blighted Forest",
            BiomeType::CorruptedMountain => "Cursed Peaks",
            BiomeType::CorruptedDesert => "Poison Desert",
            BiomeType::CorruptedSwamp => "Plague Marsh",
            BiomeType::CorruptedWater => "Tainted Waters",
            BiomeType::CorruptedSnow => "Frozen Decay",
            
            // Void-touched biomes
            BiomeType::VoidGrassland => "Void-Touched Plains",
            BiomeType::VoidForest => "Nightmare Woods",
            BiomeType::VoidMountain => "Void Peaks",
            BiomeType::VoidDesert => "Abyssal Sands",
            BiomeType::VoidSwamp => "Void Marsh",
            BiomeType::VoidWater => "Dark Waters",
            BiomeType::VoidSnow => "Void Ice",
            BiomeType::VoidLava => "Void Lava",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RegionType {
    Pastoral,
    Wilderness,
    Corrupted,
    Nightmare,
    Abyssal,
}

impl Default for RegionType {
    fn default() -> Self {
        Self::Pastoral
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Region {
    pub name: String,
    pub region_type: RegionType,
    pub dominant_biome: BiomeType,
    pub corruption_level: f32,
    pub dread_level: f32,
    pub hex_coordinates: Vec<(i32, i32)>,
}

impl Default for Region {
    fn default() -> Self {
        Self {
            name: "Unknown Region".to_string(),
            region_type: RegionType::default(),
            dominant_biome: BiomeType::default(),
            corruption_level: 0.0,
            dread_level: 0.0,
            hex_coordinates: Vec::new(),
        }
    }
}
