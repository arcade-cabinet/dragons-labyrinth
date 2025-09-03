use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use std::collections::HashMap;

use crate::world::components::{Tile, BiomeType, PathOverlay, FeatureOverlay, HexPosition, HexId, HexCorrelations};
use crate::world::state::{WorldState, AssetHandles};
use crate::utils::hex::*;
use crate::spatial::SpatialContainer;

// Include generated world resources
include!(concat!(env!("OUT_DIR"), "/generated_world.rs"));

/// Entity set at a hex coordinate from generated resources
#[derive(Debug, Clone, Default)]
pub struct HexEntitySet {
    pub settlements: Vec<String>,
    pub factions: Vec<String>,
    pub npcs: Vec<String>,
    pub dungeons: Vec<String>,
    pub special_features: Vec<String>,
}

/// Layer cake hex world system that uses generated dual pattern resources
/// Replaces static mapgen with sophisticated generated content
pub fn layer_cake_hex_world_system(
    mut commands: Commands,
    mut world_state: ResMut<WorldState>,
    mut spatial_container: ResMut<SpatialContainer>,
    player_query: Query<&Transform, (With<crate::world::components::Player>, Changed<Transform>)>,
    asset_handles: Res<AssetHandles>,
    mut tilemap_query: Query<&mut TilemapStorage>,
    correlations: Res<EntityCorrelations>,
) {
    // Use generated resources instead of procedural generation
    if let Ok(player_transform) = player_query.get_single() {
        let player_hex = world_to_hex(player_transform.translation);
        
        // Get hex tiles to load around player using generated resources
        let hexes_to_load = get_hexes_around_point(player_hex, 3);
        
        for hex_coord in hexes_to_load {
            if !world_state.loaded_hexes.contains(&hex_coord) {
                load_generated_hex_with_correlations(
                    hex_coord,
                    &mut commands,
                    &mut world_state,
                    &mut spatial_container,
                    &asset_handles,
                    &mut tilemap_query,
                    &correlations,
                );
                world_state.loaded_hexes.insert(hex_coord);
            }
        }
    } else if world_state.loaded_hexes.is_empty() {
        // Initial world loading around origin using generated resources
        let origin_hexes = get_hexes_around_point(HexCoord::new(0, 0), 5);
        for hex_coord in origin_hexes {
            load_generated_hex_with_correlations(
                hex_coord,
                &mut commands,
                &mut world_state,
                &mut spatial_container,
                &asset_handles,
                &mut tilemap_query,
                &correlations,
            );
            world_state.loaded_hexes.insert(hex_coord);
        }
        
        // Establish player starting position based on monster CR analysis
        establish_player_starting_position(&mut commands, &correlations, &spatial_container);
    }
}

/// Load a hex tile using generated resources with all correlated entities
fn load_generated_hex_with_correlations(
    hex_coord: HexCoord,
    commands: &mut Commands,
    world_state: &mut ResMut<WorldState>,
    spatial_container: &mut ResMut<SpatialContainer>,
    asset_handles: &Res<AssetHandles>,
    tilemap_query: &mut Query<&mut TilemapStorage>,
    correlations: &Res<EntityCorrelations>,
) {
    // Query generated resources for this hex coordinate
    let hex_entities = correlations.get_entities_at_hex((hex_coord.x, hex_coord.y));
    
    // Determine biome type from generated data or default based on distance
    let biome_type = determine_biome_from_correlations(hex_coord, hex_entities);
    
    // Create tile entity with bevy_ecs_tilemap integration
    let tile_pos = TilePos::new(hex_coord.x as u32, hex_coord.y as u32);
    let texture_index = get_texture_index_for_biome(&biome_type);
    
    if let Ok(mut tilemap_storage) = tilemap_query.get_single_mut() {
        let tile_entity = commands.spawn((
            TileBundle {
                position: tile_pos,
                tilemap_id: TilemapId(world_state.tilemap_entity.unwrap()),
                texture_index,
                ..default()
            },
            Tile {
                coords: hex_coord,
                biome_type: biome_type.clone(),
                paths: Vec::new(),
                features: Vec::new(),
            },
            HexPosition { q: hex_coord.x, r: hex_coord.y },
            HexId(format!("hex_{}_{}", hex_coord.x, hex_coord.y)),
            HexCorrelations {
                settlements: hex_entities.settlements.clone(),
                factions: hex_entities.factions.clone(),
                npcs: hex_entities.npcs.clone(),
                nearby_dungeons: hex_entities.dungeons.clone(),
            },
            Name::new(format!("GeneratedTile_{:?}", hex_coord)),
        )).id();
        
        tilemap_storage.set(&tile_pos, tile_entity);
        
        // Register in spatial container for O(1) lookups
        spatial_container.register_hex_entity((hex_coord.x, hex_coord.y), tile_entity);
        
        // Spawn correlated entities at this hex using generated data
        spawn_correlated_entities_at_hex(
            commands,
            tile_entity,
            hex_coord,
            hex_entities,
            &biome_type,
        );
    }
}

/// Spawn all correlated entities at a hex using generated resource data
fn spawn_correlated_entities_at_hex(
    commands: &mut Commands,
    hex_entity: Entity,
    hex_coord: HexCoord,
    hex_entities: &HexEntitySet,
    biome_type: &BiomeType,
) {
    let hex_world_pos = hex_to_world(hex_coord);
    
    // Spawn settlements from generated data
    for settlement_uuid in &hex_entities.settlements {
        let settlement_entity = commands.spawn((
            Transform::from_translation(hex_world_pos + Vec3::new(0.0, 1.0, 0.0)),
            SettlementMarker {
                uuid: settlement_uuid.clone(),
                settlement_type: determine_settlement_type_from_biome(biome_type),
            },
            Name::new(format!("Settlement_{}", settlement_uuid)),
        )).id();
        
        commands.entity(hex_entity).add_child(settlement_entity);
    }
    
    // Spawn faction presence markers from generated data
    for faction_uuid in &hex_entities.factions {
        let faction_entity = commands.spawn((
            Transform::from_translation(hex_world_pos + Vec3::new(1.0, 1.0, 0.0)),
            FactionPresenceMarker {
                uuid: faction_uuid.clone(),
                influence_level: calculate_faction_influence(faction_uuid, hex_coord),
            },
            Name::new(format!("FactionPresence_{}", faction_uuid)),
        )).id();
        
        commands.entity(hex_entity).add_child(faction_entity);
    }
    
    // Spawn NPCs from generated data
    for npc_uuid in &hex_entities.npcs {
        let npc_entity = commands.spawn((
            Transform::from_translation(hex_world_pos + Vec3::new(-1.0, 1.0, 0.0)),
            NPCMarker {
                uuid: npc_uuid.clone(),
                npc_type: "generated".to_string(),
                is_active: true,
            },
            Name::new(format!("NPC_{}", npc_uuid)),
        )).id();
        
        commands.entity(hex_entity).add_child(npc_entity);
    }
    
    // Create dungeon entrance markers for nearby dungeons
    for dungeon_uuid in &hex_entities.dungeons {
        let dungeon_marker_entity = commands.spawn((
            Transform::from_translation(hex_world_pos + Vec3::new(0.0, 1.5, 0.0)),
            DungeonEntranceMarker {
                dungeon_uuid: dungeon_uuid.clone(),
                entrance_type: "portal".to_string(),
                is_accessible: true,
            },
            Name::new(format!("DungeonEntrance_{}", dungeon_uuid)),
        )).id();
        
        commands.entity(hex_entity).add_child(dungeon_marker_entity);
    }
    
    // Add special features based on generated data and biome
    spawn_biome_specific_features(commands, hex_entity, hex_coord, biome_type, hex_entities);
}

/// Establish player starting position based on monster CR analysis
fn establish_player_starting_position(
    commands: &mut Commands,
    correlations: &Res<EntityCorrelations>,
    spatial_container: &SpatialContainer,
) {
    // Find hex coordinates with appropriate starting difficulty
    let suitable_starting_hex = find_suitable_starting_hex(correlations);
    
    match suitable_starting_hex {
        Some(coords) => {
            let world_pos = hex_to_world(HexCoord::new(coords.0, coords.1));
            
            // Spawn player at the suitable starting position
            commands.spawn((
                Transform::from_translation(world_pos + Vec3::new(0.0, 2.0, 0.0)),
                crate::world::components::Player::default(),
                HexPosition { q: coords.0, r: coords.1 },
                Name::new("Player"),
            ));
            
            info!("Player spawned at starting hex {:?} based on monster CR analysis", coords);
        }
        None => {
            // Fallback to origin if no suitable hex found
            let origin_pos = hex_to_world(HexCoord::new(0, 0));
            commands.spawn((
                Transform::from_translation(origin_pos + Vec3::new(0.0, 2.0, 0.0)),
                crate::world::components::Player::default(),
                HexPosition { q: 0, r: 0 },
                Name::new("Player"),
            ));
            
            warn!("No suitable starting hex found, spawning player at origin");
        }
    }
}

/// Find suitable starting hex based on monster CR and difficulty analysis
fn find_suitable_starting_hex(correlations: &Res<EntityCorrelations>) -> Option<(i32, i32)> {
    // Analyze hex coordinates within starting distance from origin
    let starting_search_radius = 10;
    
    for q in -starting_search_radius..=starting_search_radius {
        for r in -starting_search_radius..=starting_search_radius {
            let hex_entities = correlations.get_entities_at_hex((q, r));
            
            // Check if this hex has appropriate difficulty for starting
            if is_suitable_starting_hex((q, r), hex_entities) {
                return Some((q, r));
            }
        }
    }
    
    None
}

/// Determine if a hex is suitable for player starting position
fn is_suitable_starting_hex(coords: (i32, i32), hex_entities: &HexEntitySet) -> bool {
    let distance_from_origin = (coords.0.abs() + coords.1.abs()) as f32;
    
    // Ideal starting conditions:
    // - Close to origin (distance < 5)
    // - Has at least one settlement (safety)
    // - No dungeons nearby (too dangerous for start)
    // - Some faction presence (interaction opportunities)
    
    distance_from_origin < 5.0
        && !hex_entities.settlements.is_empty()
        && hex_entities.dungeons.is_empty()
        && hex_entities.factions.len() <= 2  // Not too politically complex
}

/// Determine biome type from correlated entities or default logic
fn determine_biome_from_correlations(hex_coord: HexCoord, hex_entities: &HexEntitySet) -> BiomeType {
    // Use generated entity correlations to determine biome
    if !hex_entities.settlements.is_empty() {
        // Hexes with settlements tend to be more hospitable
        BiomeType::Grassland
    } else if !hex_entities.dungeons.is_empty() {
        // Hexes with dungeons tend to be corrupted or dangerous
        BiomeType::Corrupted(Box::new(BiomeType::Forest))
    } else {
        // Use distance-based fallback like original system
        get_biome_from_distance(hex_coord)
    }
}

/// Fallback biome determination based on distance (simplified from original)
fn get_biome_from_distance(hex_coord: HexCoord) -> BiomeType {
    let distance_from_origin = (hex_coord.x.abs() + hex_coord.y.abs()) as f32;
    
    match distance_from_origin {
        d if d < 10.0 => BiomeType::Grassland,
        d if d < 30.0 => BiomeType::Forest,
        d if d < 60.0 => BiomeType::Desert,
        _ => BiomeType::Corrupted(Box::new(BiomeType::Void)),
    }
}

/// Get texture index for biome (simplified)
fn get_texture_index_for_biome(biome_type: &BiomeType) -> TileTextureIndex {
    let index = match biome_type {
        BiomeType::Grassland => 0,
        BiomeType::Forest => 1,
        BiomeType::Mountain => 2,
        BiomeType::Desert => 3,
        BiomeType::Swamp => 4,
        BiomeType::Water => 5,
        BiomeType::Snow => 6,
        BiomeType::Lava => 7,
        BiomeType::Void => 8,
        BiomeType::Corrupted(_) => 9,
        _ => 0, // Default to grassland
    };
    TileTextureIndex(index)
}

/// Get hexes around a point (renamed from chunks)
fn get_hexes_around_point(center: HexCoord, radius: i32) -> Vec<HexCoord> {
    let mut hexes = Vec::new();
    
    for dx in -radius..=radius {
        for dy in -radius..=radius {
            if dx.abs() + dy.abs() <= radius {  // Hex distance constraint
                hexes.push(HexCoord::new(center.x + dx, center.y + dy));
            }
        }
    }
    
    hexes
}

// === OLD SYSTEM FUNCTIONS (REMOVED) ===
// These functions are kept for reference but the new layer cake system
// replaces procedural generation with generated resources
