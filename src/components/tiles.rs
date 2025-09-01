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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BiomeType {
    Grassland,
    Forest,
    Mountain,
    Desert,
    Swamp,
    Water,
    Lava,
    Void,
    Corrupted(Box<BiomeType>), // Corrupted version of base biome
}

impl BiomeType {
    pub fn get_movement_multiplier(&self) -> f32 {
        match self {
            BiomeType::Grassland => 1.0,
            BiomeType::Forest => 0.8,
            BiomeType::Mountain => 0.6,
            BiomeType::Desert => 0.7,
            BiomeType::Swamp => 0.5,
            BiomeType::Water => 0.3,
            BiomeType::Lava => 0.2,
            BiomeType::Void => 1.5, // Faster but dangerous
            BiomeType::Corrupted(base) => base.get_movement_multiplier() * 0.8,
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
            BiomeType::Corrupted(base) => base.get_mounted_multiplier() * 0.5,
        }
    }
    
    pub fn get_damage_per_turn(&self) -> f32 {
        match self {
            BiomeType::Desert => 1.0,
            BiomeType::Lava => 10.0,
            BiomeType::Void => 2.0,
            BiomeType::Corrupted(_) => 1.5,
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
            BiomeType::Void => vec![BiomeType::Lava, BiomeType::Corrupted(Box::new(BiomeType::Grassland))],
            BiomeType::Corrupted(base) => {
                let mut neighbors = base.get_compatible_neighbors();
                neighbors.push(BiomeType::Void);
                neighbors
            }
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
