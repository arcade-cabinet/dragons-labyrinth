use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use noise::{NoiseFn, Perlin};
use rand::prelude::*;

use crate::components::{Tile, BiomeType, PathOverlay, FeatureOverlay};
use crate::resources::{WorldState, AssetHandles};
use crate::utils::hex::*;

pub fn hex_world_generation_system(
    mut commands: Commands,
    mut world_state: ResMut<WorldState>,
    player_query: Query<&Transform, (With<crate::components::Player>, Changed<Transform>)>,
    asset_handles: Res<AssetHandles>,
    mut tilemap_query: Query<&mut TilemapStorage>,
) {
    // Only generate if player has moved or initial generation needed
    if let Ok(player_transform) = player_query.get_single() {
        let player_hex = world_to_hex(player_transform.translation);
        
        // Check if we need to generate new chunks around player
        let chunks_to_generate = get_chunks_around_point(player_hex, 2);
        
        for chunk_coord in chunks_to_generate {
            if !world_state.generated_chunks.contains(&chunk_coord) {
                generate_chunk(&mut commands, &mut world_state, chunk_coord, &asset_handles, &mut tilemap_query);
                world_state.generated_chunks.insert(chunk_coord);
            }
        }
    } else if world_state.generated_chunks.is_empty() {
        // Initial world generation around origin
        let origin_chunks = get_chunks_around_point(HexCoord::new(0, 0), 3);
        for chunk_coord in origin_chunks {
            generate_chunk(&mut commands, &mut world_state, chunk_coord, &asset_handles, &mut tilemap_query);
            world_state.generated_chunks.insert(chunk_coord);
        }
    }
}

fn generate_chunk(
    commands: &mut Commands,
    world_state: &mut ResMut<WorldState>,
    chunk_coord: ChunkCoord,
    asset_handles: &Res<AssetHandles>,
    tilemap_query: &mut Query<&mut TilemapStorage>,
) {
    let perlin = Perlin::new(world_state.seed as u32);
    let chunk_size = 16;
    
    for x in 0..chunk_size {
        for y in 0..chunk_size {
            let hex_coord = HexCoord::new(
                chunk_coord.x * chunk_size + x,
                chunk_coord.y * chunk_size + y,
            );
            
            // Generate biome using noise
            let noise_value = perlin.get([hex_coord.x as f64 * 0.1, hex_coord.y as f64 * 0.1]);
            let biome_type = get_biome_from_noise(noise_value, &world_state.corruption_map);
            
            // Create tile entity
            let tile_entity = commands.spawn((
                Tile {
                    coords: hex_coord,
                    biome_type: biome_type.clone(),
                    paths: Vec::new(),
                    features: Vec::new(),
                },
                Transform::from_translation(hex_to_world(hex_coord)),
                Name::new(format!("Tile_{:?}", hex_coord)),
            )).id();
            
            // Add to tilemap storage
            if let Ok(mut tilemap_storage) = tilemap_query.get_single_mut() {
                let tile_pos = TilePos::new(hex_coord.x as u32, hex_coord.y as u32);
                let tile_bundle = TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(world_state.tilemap_entity.unwrap()),
                    texture_index: TileTextureIndex(get_texture_index_for_biome(&biome_type)),
                    ..default()
                };
                
                commands.entity(tile_entity).insert(tile_bundle);
                tilemap_storage.set(&tile_pos, tile_entity);
            }
            
            // Generate features based on biome and distance from corruption
            if should_generate_feature(hex_coord, &biome_type) {
                generate_tile_features(commands, tile_entity, hex_coord, &biome_type);
            }
        }
    }
}

fn get_biome_from_noise(noise_value: f64, corruption_map: &std::collections::HashMap<HexCoord, f32>) -> BiomeType {
    match noise_value {
        n if n < -0.5 => BiomeType::Water,
        n if n < -0.2 => BiomeType::Swamp,
        n if n < 0.0 => BiomeType::Grassland,
        n if n < 0.3 => BiomeType::Forest,
        n if n < 0.6 => BiomeType::Mountain,
        _ => BiomeType::Desert,
    }
}

fn get_texture_index_for_biome(biome_type: &BiomeType) -> u32 {
    match biome_type {
        BiomeType::Grassland => 0,
        BiomeType::Forest => 1,
        BiomeType::Mountain => 2,
        BiomeType::Desert => 3,
        BiomeType::Swamp => 4,
        BiomeType::Water => 5,
        BiomeType::Lava => 6,
        BiomeType::Void => 7,
        BiomeType::Corrupted(_) => 8,
    }
}

fn should_generate_feature(hex_coord: HexCoord, biome_type: &BiomeType) -> bool {
    let mut rng = thread_rng();
    let base_chance = match biome_type {
        BiomeType::Grassland => 0.1,
        BiomeType::Forest => 0.15,
        BiomeType::Mountain => 0.08,
        BiomeType::Desert => 0.05,
        BiomeType::Swamp => 0.12,
        BiomeType::Water => 0.02,
        _ => 0.0,
    };
    
    rng.gen::<f32>() < base_chance
}

fn generate_tile_features(
    commands: &mut Commands,
    tile_entity: Entity,
    hex_coord: HexCoord,
    biome_type: &BiomeType,
) {
    let mut rng = thread_rng();
    
    // Generate appropriate features for biome
    let features = match biome_type {
        BiomeType::Grassland => vec!["tavern", "shop", "shrine"],
        BiomeType::Forest => vec!["campsite", "dungeon_entrance", "monster_lair"],
        BiomeType::Mountain => vec!["tower", "treasure_cache", "shrine"],
        BiomeType::Desert => vec!["oasis", "ruins", "treasure_cache"],
        BiomeType::Swamp => vec!["witch_hut", "monster_lair", "cursed_shrine"],
        _ => vec![],
    };
    
    if !features.is_empty() {
        let feature_type = features[rng.gen_range(0..features.len())];
        
        let feature_entity = commands.spawn((
            FeatureOverlay {
                feature_type: feature_type.to_string(),
                model_id: format!("models/{}.glb", feature_type),
                interaction_type: get_interaction_for_feature(feature_type),
            },
            Transform::from_translation(hex_to_world(hex_coord) + Vec3::new(0.0, 1.0, 0.0)),
            Name::new(format!("Feature_{}_{:?}", feature_type, hex_coord)),
        )).id();
        
        // Link feature to tile
        commands.entity(tile_entity).add_child(feature_entity);
    }
}

fn get_interaction_for_feature(feature_type: &str) -> String {
    match feature_type {
        "tavern" => "enter_building",
        "shop" => "enter_building",
        "dungeon_entrance" => "enter_dungeon",
        "shrine" => "pray",
        "campsite" => "rest",
        _ => "examine",
    }.to_string()
}

type ChunkCoord = HexCoord;

fn get_chunks_around_point(center: HexCoord, radius: i32) -> Vec<ChunkCoord> {
    let mut chunks = Vec::new();
    let chunk_size = 16;
    
    for dx in -radius..=radius {
        for dy in -radius..=radius {
            let chunk_x = (center.x + dx * chunk_size) / chunk_size;
            let chunk_y = (center.y + dy * chunk_size) / chunk_size;
            chunks.push(ChunkCoord::new(chunk_x, chunk_y));
        }
    }
    
    chunks
}
