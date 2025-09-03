use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::utils::hex::HexCoord;

#[derive(Component, Debug, Clone)]
pub struct Tile {
    pub coords: HexCoord,
    pub biome_type: BiomeType,
    pub paths: Vec<Entity>,
    pub features: Vec<Entity>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BiomeType {
    // Core biomes
    Grassland,
    Forest,
    Mountain,
    Desert,
    Swamp,
    Water,
    Snow,
    Lava,
    Void,
    
    // Transitional biomes
    ForestGrassland,
    MountainForest,
    DesertMountain,
    SwampWater,
    SnowMountain,
    
    // Corrupted variants
    CorruptedGrassland,
    CorruptedForest,
    CorruptedMountain,
    CorruptedDesert,
    CorruptedSwamp,
    CorruptedWater,
    CorruptedSnow,
    
    // Void-touched variants
    VoidGrassland,
    VoidForest,
    VoidMountain,
    VoidDesert,
    VoidSwamp,
    VoidWater,
    VoidSnow,
    VoidLava,
}

impl BiomeType {
    pub fn get_movement_multiplier(&self) -> f32 {
        match self {
            // Core biomes
            BiomeType::Grassland => 1.0,
            BiomeType::Forest => 0.8,
            BiomeType::Mountain => 0.6,
            BiomeType::Desert => 0.7,
            BiomeType::Swamp => 0.5,
            BiomeType::Water => 0.3,
            BiomeType::Snow => 0.4,
            BiomeType::Lava => 0.2,
            BiomeType::Void => 1.5, // Faster but dangerous
            
            // Transitional biomes (average of components)
            BiomeType::ForestGrassland => 0.9,
            BiomeType::MountainForest => 0.7,
            BiomeType::DesertMountain => 0.65,
            BiomeType::SwampWater => 0.4,
            BiomeType::SnowMountain => 0.5,
            
            // Corrupted variants (reduced movement)
            BiomeType::CorruptedGrassland => 0.8,
            BiomeType::CorruptedForest => 0.6,
            BiomeType::CorruptedMountain => 0.4,
            BiomeType::CorruptedDesert => 0.5,
            BiomeType::CorruptedSwamp => 0.3,
            BiomeType::CorruptedWater => 0.2,
            BiomeType::CorruptedSnow => 0.3,
            
            // Void-touched variants (unpredictable movement)
            BiomeType::VoidGrassland => 1.2,
            BiomeType::VoidForest => 0.9,
            BiomeType::VoidMountain => 0.8,
            BiomeType::VoidDesert => 1.0,
            BiomeType::VoidSwamp => 0.7,
            BiomeType::VoidWater => 0.5,
            BiomeType::VoidSnow => 0.6,
            BiomeType::VoidLava => 0.4,
        }
    }
    
    pub fn get_mounted_multiplier(&self) -> f32 {
        match self {
            BiomeType::Grassland => 1.5,
            BiomeType::Forest => 0.6,
            BiomeType::Mountain => 0.3,
            BiomeType::Desert => 1.2,
            BiomeType::Swamp => 0.2,
            BiomeType::Water => 0.1,
            BiomeType::Lava => 0.1,
            BiomeType::Void => 0.8,
            
            // Transitional biomes
            BiomeType::ForestGrassland => 1.05,
            BiomeType::MountainForest => 0.45,
            BiomeType::DesertMountain => 0.75,
            BiomeType::SwampWater => 0.15,
            BiomeType::SnowMountain => 0.4,
            BiomeType::Snow => 0.5,
            
            // Corrupted variants (reduced mount effectiveness)
            BiomeType::CorruptedGrassland => 0.75,
            BiomeType::CorruptedForest => 0.3,
            BiomeType::CorruptedMountain => 0.15,
            BiomeType::CorruptedDesert => 0.6,
            BiomeType::CorruptedSwamp => 0.1,
            BiomeType::CorruptedWater => 0.05,
            BiomeType::CorruptedSnow => 0.25,
            
            // Void variants (unpredictable mount behavior)
            BiomeType::VoidGrassland => 1.2,
            BiomeType::VoidForest => 0.5,
            BiomeType::VoidMountain => 0.4,
            BiomeType::VoidDesert => 1.0,
            BiomeType::VoidSwamp => 0.3,
            BiomeType::VoidWater => 0.2,
            BiomeType::VoidSnow => 0.4,
            BiomeType::VoidLava => 0.2,
        }
    }
    
    pub fn get_damage_per_turn(&self) -> f32 {
        match self {
            BiomeType::Desert => 1.0,
            BiomeType::Lava => 10.0,
            BiomeType::Void => 2.0,
            
            // Corrupted variants cause damage
            BiomeType::CorruptedGrassland => 0.5,
            BiomeType::CorruptedForest => 1.0,
            BiomeType::CorruptedMountain => 2.0,
            BiomeType::CorruptedDesert => 1.5,
            BiomeType::CorruptedSwamp => 2.5,
            BiomeType::CorruptedWater => 3.0,
            BiomeType::CorruptedSnow => 1.8,
            
            // Void variants cause significant damage
            BiomeType::VoidGrassland => 3.0,
            BiomeType::VoidForest => 4.0,
            BiomeType::VoidMountain => 5.0,
            BiomeType::VoidDesert => 3.5,
            BiomeType::VoidSwamp => 6.0,
            BiomeType::VoidWater => 8.0,
            BiomeType::VoidSnow => 4.5,
            BiomeType::VoidLava => 15.0,
            
            _ => 0.0,
        }
    }
    
    pub fn get_compatible_neighbors(&self) -> Vec<BiomeType> {
        match self {
            BiomeType::Grassland => vec![
                BiomeType::Forest, BiomeType::Mountain, 
                BiomeType::Desert, BiomeType::Swamp
            ],
            BiomeType::Forest => vec![
                BiomeType::Grassland, BiomeType::Mountain, BiomeType::Swamp
            ],
            BiomeType::Mountain => vec![
                BiomeType::Grassland, BiomeType::Forest, 
                BiomeType::Desert, BiomeType::Lava
            ],
            BiomeType::Desert => vec![
                BiomeType::Grassland, BiomeType::Mountain, BiomeType::Lava
            ],
            BiomeType::Swamp => vec![
                BiomeType::Grassland, BiomeType::Forest, BiomeType::Water
            ],
            BiomeType::Water => vec![BiomeType::Swamp],
            BiomeType::Lava => vec![BiomeType::Desert, BiomeType::Mountain, BiomeType::Void],
            BiomeType::Void => vec![BiomeType::Lava, BiomeType::VoidLava],
            
            // Transitional biomes
            BiomeType::ForestGrassland => vec![BiomeType::Forest, BiomeType::Grassland],
            BiomeType::MountainForest => vec![BiomeType::Mountain, BiomeType::Forest],
            BiomeType::DesertMountain => vec![BiomeType::Desert, BiomeType::Mountain],
            BiomeType::SwampWater => vec![BiomeType::Swamp, BiomeType::Water],
            BiomeType::SnowMountain => vec![BiomeType::Snow, BiomeType::Mountain],
            BiomeType::Snow => vec![BiomeType::Mountain, BiomeType::SnowMountain],
            
            // Corrupted variants can connect to each other and void
            BiomeType::CorruptedGrassland => vec![BiomeType::CorruptedForest, BiomeType::VoidGrassland],
            BiomeType::CorruptedForest => vec![BiomeType::CorruptedGrassland, BiomeType::CorruptedSwamp, BiomeType::VoidForest],
            BiomeType::CorruptedMountain => vec![BiomeType::CorruptedDesert, BiomeType::VoidMountain],
            BiomeType::CorruptedDesert => vec![BiomeType::CorruptedMountain, BiomeType::VoidDesert],
            BiomeType::CorruptedSwamp => vec![BiomeType::CorruptedForest, BiomeType::CorruptedWater, BiomeType::VoidSwamp],
            BiomeType::CorruptedWater => vec![BiomeType::CorruptedSwamp, BiomeType::VoidWater],
            BiomeType::CorruptedSnow => vec![BiomeType::VoidSnow],
            
            // Void variants connect to corrupted and other void
            BiomeType::VoidGrassland => vec![BiomeType::CorruptedGrassland, BiomeType::VoidForest],
            BiomeType::VoidForest => vec![BiomeType::VoidGrassland, BiomeType::CorruptedForest, BiomeType::VoidSwamp],
            BiomeType::VoidMountain => vec![BiomeType::CorruptedMountain, BiomeType::VoidLava],
            BiomeType::VoidDesert => vec![BiomeType::CorruptedDesert, BiomeType::VoidLava],
            BiomeType::VoidSwamp => vec![BiomeType::VoidForest, BiomeType::CorruptedSwamp, BiomeType::VoidWater],
            BiomeType::VoidWater => vec![BiomeType::VoidSwamp, BiomeType::CorruptedWater],
            BiomeType::VoidSnow => vec![BiomeType::CorruptedSnow, BiomeType::VoidMountain],
            BiomeType::VoidLava => vec![BiomeType::VoidMountain, BiomeType::VoidDesert, BiomeType::Lava],
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct PathOverlay {
    pub path_type: PathType,
    pub texture_id: String,
    pub opacity: f32,
    pub speed_bonus: f32,
    pub comfort_bonus: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PathType {
    DirtTrail,
    WoodenPlanks,
    StoneRoad,
    CobblestoneStreet,
    CorruptedPath,
}

impl PathType {
    pub fn get_speed_bonus(&self) -> f32 {
        match self {
            PathType::DirtTrail => 0.1,
            PathType::WoodenPlanks => 0.2,
            PathType::StoneRoad => 0.3,
            PathType::CobblestoneStreet => 0.4,
            PathType::CorruptedPath => -0.1, // Slower due to corruption
        }
    }
    
    pub fn get_comfort_bonus(&self) -> f32 {
        match self {
            PathType::DirtTrail => 1.0,
            PathType::WoodenPlanks => 2.0,
            PathType::StoneRoad => 3.0,
            PathType::CobblestoneStreet => 5.0,
            PathType::CorruptedPath => -5.0, // Increases companion stress
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct FeatureOverlay {
    pub feature_type: String,
    pub model_id: String,
    pub interaction_type: String,
}

#[derive(Component, Debug)]
pub struct InteractableFeature {
    pub feature_type: FeatureType,
    pub interaction_range: f32,
    pub requires_key: bool,
    pub one_time_use: bool,
    pub used: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FeatureType {
    // Social features
    Tavern {
        keeper_name: String,
        staff_count: u32,
        reputation: i32,
    },
    Shop {
        owner_name: String,
        shop_type: ShopType,
        quality_level: u32,
    },
    Shrine {
        deity: String,
        blessing_type: String,
        corruption_level: f32,
    },
    
    // Encounter features
    DungeonEntrance {
        dungeon_type: String,
        max_cr: u32,
        depth_levels: u32,
    },
    MonsterLair {
        monster_type: String,
        cr: u32,
        treasure_value: u32,
    },
    TreasureCache {
        value: u32,
        trapped: bool,
        hidden: bool,
    },
    
    // Interactive features
    Campsite {
        rest_bonus: f32,
        safety_level: f32,
        has_fire: bool,
    },
    Bridge {
        crosses_terrain: BiomeType,
        structural_integrity: f32,
    },
    Portal {
        destination: HexCoord,
        activation_cost: u32,
        is_active: bool,
    },
    
    // Corruption features
    CorruptionNode {
        corruption_radius: f32,
        intensity: f32,
        spreading: bool,
    },
    VoidTear {
        stability: f32,
        effects_radius: f32,
        monsters_spawn: bool,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ShopType {
    General,
    Weapons,
    Armor,
    Potions,
    Magic,
    Information,
    Mounts,
}
