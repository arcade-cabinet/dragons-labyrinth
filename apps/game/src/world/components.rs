use bevy::prelude::*;
use avian3d::prelude::*;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Component, Debug, Clone)]
pub struct Tile {
    pub coords: crate::utils::hex::HexCoord,
    pub biome_type: BiomeType,
    pub paths: Vec<PathOverlay>,
    pub features: Vec<FeatureOverlay>,
}

// ===== LAYER CAKE SYSTEM COMPONENTS =====
// These components support the generated dual pattern resources

/// Hex position component for spatial queries
#[derive(Component, Debug, Clone, PartialEq)]
pub struct HexPosition {
    pub q: i32,
    pub r: i32,
}

/// Unique hex identifier
#[derive(Component, Debug, Clone)]
pub struct HexId(pub String);

/// Region identifier for hex tiles
#[derive(Component, Debug, Clone)]
pub struct RegionId(pub String);

/// Hex correlations to all entities at this location
#[derive(Component, Debug, Clone)]
pub struct HexCorrelations {
    pub settlements: Vec<String>,
    pub factions: Vec<String>,
    pub npcs: Vec<String>,
    pub nearby_dungeons: Vec<String>,
}

/// Settlement marker component for generated settlements
#[derive(Component, Debug, Clone)]
pub struct SettlementMarker {
    pub uuid: String,
    pub settlement_type: String,
}

/// Faction presence marker at hex locations
#[derive(Component, Debug, Clone)]
pub struct FactionPresenceMarker {
    pub uuid: String,
    pub influence_level: f32,
}

/// NPC marker component for generated NPCs
#[derive(Component, Debug, Clone)]
pub struct NPCMarker {
    pub uuid: String,
    pub npc_type: String,
    pub is_active: bool,
}

/// Dungeon entrance marker for nearby dungeons
#[derive(Component, Debug, Clone)]
pub struct DungeonEntranceMarker {
    pub dungeon_uuid: String,
    pub entrance_type: String,
    pub is_accessible: bool,
}

// ===== DUNGEON SYSTEM COMPONENTS =====
// These components support the dungeon area pattern

/// Dungeon identifier
#[derive(Component, Debug, Clone)]
pub struct DungeonId(pub String);

/// Dungeon area identifier
#[derive(Component, Debug, Clone)]
pub struct DungeonAreaId(pub String);

/// Dungeon area name
#[derive(Component, Debug, Clone)]
pub struct DungeonAreaName(pub String);

/// Connections between dungeon areas
#[derive(Component, Debug, Clone)]
pub struct DungeonConnections {
    pub connected_areas: Vec<String>,
}

/// Pathfinding nodes for dungeon navigation
#[derive(Component, Debug, Clone)]
pub struct PathfindingNodes(pub Vec<(i32, i32)>);

// ===== METADATA STRUCTURES =====
// Static metadata for efficient queries

/// Static hex metadata for container queries
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

/// Static region metadata
#[derive(Debug, Clone)]
pub struct RegionMetadata {
    pub uuid: &'static str,
    pub name: &'static str,
    pub hex_count: usize,
    pub total_entities: usize,
}

/// Static dungeon area metadata
#[derive(Debug, Clone)]
pub struct DungeonAreaMetadata {
    pub dungeon_uuid: &'static str,
    pub area_uuid: &'static str,
    pub monster_count: usize,
    pub treasure_count: usize,
    pub connection_count: usize,
}

// ===== ADDITIONAL COMPONENTS =====
// Extended components for the full Dragon's Labyrinth system

#[derive(Component, Debug, Clone)]
pub struct RegionName(pub String);

#[derive(Component, Debug, Clone)]
pub struct CorruptionLevel(pub f32);

/// Biome features component for terrain details
#[derive(Component, Debug, Clone, Default)]
pub struct BiomeFeatures {
    pub terrain_type: String,
    pub vegetation: String,
    pub hazards: Vec<String>,
    pub resources: Vec<String>,
}

/// Detailed hex biome enum for Dragon's Labyrinth progression
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

/// Extended settlement components
#[derive(Component, Debug, Clone)]
pub struct Population(pub u32);

#[derive(Component, Debug, Clone)]
pub enum SettlementType {
    Village,
    Town,
    City,
    Outpost,
    Ruins,
}

/// Extended faction components
#[derive(Component, Debug, Clone)]
pub struct FactionInfluence(pub f32);

#[derive(Component, Debug, Clone)]
pub struct Territory {
    pub controlled_hexes: Vec<(i32, i32)>,
    pub influence_radius: u32,
}

/// Extended dungeon components
#[derive(Component, Debug, Clone)]
pub struct DungeonEntrance {
    pub hex_position: (i32, i32),
    pub entrance_type: String,
}

#[derive(Component, Debug, Clone)]
pub struct DungeonDepth(pub u32);

/// NPC-specific components
#[derive(Component, Debug, Clone)]
pub struct NpcRole(pub String);

#[derive(Component, Debug, Clone)]
pub struct CurrentHex(pub i32, pub i32);

/// Static data for generated content
#[derive(Debug, Clone)]
pub struct HexStaticData {
    pub uuid: &'static str,
    pub q: i32,
    pub r: i32,
    pub biome: &'static str,
}

// ===== LAYER CAKE HELPER FUNCTIONS =====

/// Determine settlement type from biome
pub fn determine_settlement_type_from_biome(biome_type: &BiomeType) -> String {
    match biome_type {
        BiomeType::Grassland => "village".to_string(),
        BiomeType::Forest => "outpost".to_string(),
        BiomeType::Mountain => "fortress".to_string(),
        BiomeType::Desert => "oasis_town".to_string(),
        BiomeType::Swamp => "stilted_village".to_string(),
        BiomeType::Water => "port".to_string(),
        BiomeType::Corrupted(_) => "ruins".to_string(),
        _ => "settlement".to_string(),
    }
}

/// Calculate faction influence at a hex coordinate
pub fn calculate_faction_influence(faction_uuid: &str, hex_coord: crate::utils::hex::HexCoord) -> f32 {
    // Simple distance-based influence calculation
    let distance_from_origin = (hex_coord.x.abs() + hex_coord.y.abs()) as f32;
    
    // Closer to origin = less faction influence (more peaceful)
    // Further from origin = more faction conflict
    match distance_from_origin {
        d if d < 10.0 => 0.2, // Low influence near origin
        d if d < 30.0 => 0.5, // Medium influence in middle bands
        d if d < 60.0 => 0.8, // High influence in outer bands
        _ => 1.0, // Maximum influence in far regions
    }
}

/// Spawn biome-specific features based on generated data
pub fn spawn_biome_specific_features(
    commands: &mut Commands,
    hex_entity: Entity,
    hex_coord: crate::utils::hex::HexCoord,
    biome_type: &BiomeType,
    hex_entities: &super::systems::hex_world::HexEntitySet,
) {
    let hex_world_pos = crate::utils::hex::hex_to_world(hex_coord);
    
    // Spawn biome-specific environmental features
    match biome_type {
        BiomeType::Grassland => {
            if !hex_entities.settlements.is_empty() {
                // Add roads connecting settlements
                let road_feature = commands.spawn((
                    Transform::from_translation(hex_world_pos + Vec3::new(0.0, 0.1, 0.0)),
                    PathOverlay {
                        path_type: "road".to_string(),
                        width: 2.0,
                        material: "cobblestone".to_string(),
                    },
                    Name::new("Road"),
                )).id();
                commands.entity(hex_entity).add_child(road_feature);
            }
        }
        BiomeType::Forest => {
            // Add forest undergrowth and paths
            let undergrowth = commands.spawn((
                Transform::from_translation(hex_world_pos + Vec3::new(0.0, 0.5, 0.0)),
                FeatureOverlay {
                    feature_type: "undergrowth".to_string(),
                    model_id: "models/forest/undergrowth.glb".to_string(),
                    interaction_type: "examine".to_string(),
                },
                Name::new("ForestUndergrowth"),
            )).id();
            commands.entity(hex_entity).add_child(undergrowth);
        }
        BiomeType::Corrupted(_) => {
            // Add corruption effects
            let corruption_effect = commands.spawn((
                Transform::from_translation(hex_world_pos + Vec3::new(0.0, 1.0, 0.0)),
                FeatureOverlay {
                    feature_type: "corruption_mist".to_string(), 
                    model_id: "models/corruption/mist.glb".to_string(),
                    interaction_type: "avoid".to_string(),
                },
                Name::new("CorruptionMist"),
            )).id();
            commands.entity(hex_entity).add_child(corruption_effect);
        }
        _ => {
            // Default environmental features
        }
    }
}
