use bevy::prelude::*;
use std::collections::HashMap;

// === Core Hex Components ===

#[derive(Component, Debug, Clone)]
pub struct HexTile { 
    pub q: i32, 
    pub r: i32, 
    pub biome: String, 
    pub distance_band: String 
}

#[derive(Component, Debug, Clone)]
pub struct HexPosition { 
    pub q: i32, 
    pub r: i32 
}

#[derive(Component, Debug, Clone)]
pub struct HexId(pub String);

#[derive(Component, Debug, Clone)]
pub struct RegionId(pub String);

#[derive(Component, Debug, Clone)]
pub struct RegionName(pub String);

#[derive(Component, Debug, Clone)]
pub struct CorruptionLevel(pub f32);

// === Biome Components ===

#[derive(Component, Debug, Clone, Default)]
pub struct BiomeFeatures {
    pub terrain_type: String,
    pub vegetation: String,
    pub hazards: Vec<String>,
    pub resources: Vec<String>,
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

// === Correlation Components ===

#[derive(Component, Debug, Clone, Default)]
pub struct HexCorrelations {
    pub settlements: Vec<String>,
    pub factions: Vec<String>,
    pub npcs: Vec<String>,
    pub nearby_dungeons: Vec<String>,
}

// === Settlement Components ===

#[derive(Component, Debug, Clone)]
pub struct SettlementId(pub String);

#[derive(Component, Debug, Clone)]
pub struct SettlementName(pub String);

#[derive(Component, Debug, Clone)]
pub enum SettlementType {
    Village,
    Town,
    City,
    Outpost,
    Ruins,
}

#[derive(Component, Debug, Clone)]
pub struct Population(pub u32);

// === Faction Components ===

#[derive(Component, Debug, Clone)]
pub struct FactionId(pub String);

#[derive(Component, Debug, Clone)]
pub struct FactionName(pub String);

#[derive(Component, Debug, Clone)]
pub struct FactionInfluence(pub f32);

#[derive(Component, Debug, Clone)]
pub struct Territory {
    pub controlled_hexes: Vec<(i32, i32)>,
    pub influence_radius: u32,
}

// === Dungeon Components ===

#[derive(Component, Debug, Clone)]
pub struct DungeonId(pub String);

#[derive(Component, Debug, Clone)]
pub struct DungeonName(pub String);

#[derive(Component, Debug, Clone)]
pub struct DungeonEntrance {
    pub hex_position: (i32, i32),
    pub entrance_type: String,
}

#[derive(Component, Debug, Clone)]
pub struct DungeonDepth(pub u32);

// === Dungeon Area Components ===

#[derive(Component, Debug, Clone)]
pub struct DungeonAreaId(pub String);

#[derive(Component, Debug, Clone)]
pub struct DungeonAreaName(pub String);

#[derive(Component, Debug, Clone)]
pub struct DungeonConnections {
    pub connected_areas: Vec<String>,
}

#[derive(Component, Debug, Clone)]
pub struct PathfindingNodes(pub Vec<(i32, i32)>);

#[derive(Debug, Clone)]
pub struct DungeonAreaMetadata {
    pub dungeon_uuid: &'static str,
    pub area_uuid: &'static str,
    pub monster_count: usize,
    pub treasure_count: usize,
    pub connection_count: usize,
}

// === NPC Components ===

#[derive(Component, Debug, Clone)]
pub struct NpcId(pub String);

#[derive(Component, Debug, Clone)]
pub struct NpcName(pub String);

#[derive(Component, Debug, Clone)]
pub struct NpcRole(pub String);

#[derive(Component, Debug, Clone)]
pub struct CurrentHex(pub i32, pub i32);

// === Metadata Structures ===

#[derive(Debug, Clone)]
pub struct RegionMetadata {
    pub uuid: &'static str,
    pub name: &'static str,
    pub base_corruption: f32,
}

#[derive(Debug, Clone)]
pub struct HexMetadata {
    pub coords: (i32, i32),
    pub region_uuid: &'static str,
    pub hex_uuid: &'static str,
    pub entity_count: usize,
    pub settlement_count: usize,
    pub faction_count: usize,
    pub npc_count: usize,
    pub dungeon_count: usize,
}

#[derive(Debug, Clone)]
pub struct HexStaticData {
    pub uuid: &'static str,
    pub q: i32,
    pub r: i32,
    pub biome: &'static str,
}
