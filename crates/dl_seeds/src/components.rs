//! ECS components used by the generated code

use bevy::prelude::{Component, Entity};
use std::collections::HashMap;

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

// New generated components for regions
#[derive(Component, Debug, Clone)]
pub struct RegionId(pub String);

#[derive(Component, Debug, Clone)]
pub struct RegionName(pub String);

#[derive(Component, Debug, Clone)]
pub struct CorruptionLevel(pub f32);

#[derive(Component, Debug, Clone, Default)]
pub struct BiomeFeatures {
    pub features: Vec<String>,
}

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

// Spatial container component for O(1) lookups
#[derive(Component, Default, Debug)]
pub struct SpatialContainer {
    pub hex_entities: HashMap<(i32, i32), Entity>,
    pub region_entities: HashMap<String, Entity>,
    pub dungeon_entities: HashMap<String, Entity>,
}

impl SpatialContainer {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn register_hex_entity(&mut self, coords: (i32, i32), entity: Entity) {
        self.hex_entities.insert(coords, entity);
    }
    
    pub fn get_hex_entity(&self, coords: (i32, i32)) -> Option<Entity> {
        self.hex_entities.get(&coords).copied()
    }
    
    pub fn register_region_entity(&mut self, uuid: String, entity: Entity) {
        self.region_entities.insert(uuid, entity);
    }
    
    pub fn get_entities_at_hex(&self, coords: (i32, i32)) -> Vec<Entity> {
        let mut entities = Vec::new();
        if let Some(entity) = self.hex_entities.get(&coords) {
            entities.push(*entity);
        }
        entities
    }

    pub fn register_dungeon_area_entity(&mut self, uuid: String, entity: Entity) {
        self.dungeon_entities.insert(uuid, entity);
    }
}

// Static data types
#[derive(Debug, Clone)]
pub struct HexData {
    pub uuid: &'static str,
    pub q: i32,
    pub r: i32,
    pub biome: &'static str,
}

#[derive(Debug, Clone)]
pub struct HexStaticData {
    pub uuid: &'static str,
    pub q: i32,
    pub r: i32,
    pub biome: &'static str,
}

#[derive(Debug, Clone)]
pub struct RegionMetadata {
    pub uuid: &'static str,
    pub name: &'static str,
    pub base_corruption: f32,
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
