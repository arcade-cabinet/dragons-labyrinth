use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use mapgen::*;
use rand::prelude::*;

use crate::world::components::{Tile, BiomeType, PathOverlay, FeatureOverlay};
use crate::world::state::{WorldState, AssetHandles};
use crate::utils::hex::*;

pub fn hex_world_generation_system(
    mut commands: Commands,
    mut world_state: ResMut<WorldState>,
    player_query: Query<&Transform, (With<crate::world::components::Player>, Changed<Transform>)>,
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
    let chunk_size = 16;
    let mut rng = StdRng::seed_from_u64(world_state.seed);
    
    // Use mapgen to create the terrain
    let mut map_builder = MapBuilder::new(chunk_size as usize, chunk_size as usize)
        .with(NoiseGenerator::new())
        .with(AreaStartingPosition::new(XStart::CENTER, YStart::CENTER))
        .with(CullUnreachable::new())
        .with(VoronoiSpawning::new())
        .with(DistantExit::new());
    
    let map = map_builder.build_map(&mut rng);
    
    for x in 0..chunk_size {
        for y in 0..chunk_size {
            let hex_coord = HexCoord::new(
                chunk_coord.x * chunk_size + x,
                chunk_coord.y * chunk_size + y,
            );
            
            // Get terrain type from mapgen
            let map_idx = (y * chunk_size + x) as usize;
            let tile_type = if map_idx < map.tiles.len() {
                map.tiles[map_idx]
            } else {
                TileType::Wall
            };
            
            // Convert mapgen tile to biome
            let biome_type = match tile_type {
                TileType::Floor => get_biome_from_position(hex_coord, &world_state.corruption_map, &mut rng),
                TileType::Wall => BiomeType::Mountain,
                TileType::DownStairs => BiomeType::Water,
                TileType::UpStairs => BiomeType::Void,
            };
            
            // Create tile entity with proper tilemap integration
            let tile_pos = TilePos::new(hex_coord.x as u32, hex_coord.y as u32);
            let (atlas_index, tile_index) = get_texture_index_for_biome(&biome_type);
            let texture_index = TileTextureIndex(tile_index);
            
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
                    Name::new(format!("Tile_{:?}", hex_coord)),
                )).id();
                
                tilemap_storage.set(&tile_pos, tile_entity);
                
                // Generate features using mapgen spawn points
                if map.spawn_list.iter().any(|(idx, _)| *idx == map_idx) {
                    generate_tile_features(commands, tile_entity, hex_coord, &biome_type);
                }
            }
        }
    }
}

fn get_biome_from_position(
    hex_coord: HexCoord, 
    corruption_map: &std::collections::HashMap<HexCoord, f32>,
    rng: &mut StdRng
) -> BiomeType {
    // Check for corruption first
    if let Some(&corruption_level) = corruption_map.get(&hex_coord) {
        if corruption_level > 0.3 {
            return BiomeType::Corrupted(Box::new(BiomeType::Grassland));
        }
    }
    
    // Use distance from origin to determine biome progression
    let distance_from_origin = (hex_coord.x.abs() + hex_coord.y.abs()) as f32;
    let biome_roll: f32 = rng.gen();
    
    match distance_from_origin {
        d if d < 10.0 => {
            match biome_roll {
                f if f < 0.6 => BiomeType::Grassland,
                f if f < 0.8 => BiomeType::Forest,
                _ => BiomeType::Water,
            }
        }
        d if d < 30.0 => {
            match biome_roll {
                f if f < 0.4 => BiomeType::Forest,
                f if f < 0.6 => BiomeType::Mountain,
                f if f < 0.8 => BiomeType::Desert,
                _ => BiomeType::Swamp,
            }
        }
        d if d < 60.0 => {
            match biome_roll {
                f if f < 0.3 => BiomeType::Desert,
                f if f < 0.6 => BiomeType::Mountain,
                f if f < 0.8 => BiomeType::Lava,
                _ => BiomeType::Corrupted(Box::new(BiomeType::Desert)),
            }
        }
        _ => {
            match biome_roll {
                f if f < 0.5 => BiomeType::Lava,
                f if f < 0.8 => BiomeType::Void,
                _ => BiomeType::Corrupted(Box::new(BiomeType::Void)),
            }
        }
    }
}

fn get_model_path_for_biome(biome_type: &BiomeType) -> String {
    match biome_type {
        // Core biomes
        BiomeType::Grassland => "models/biomes/core/grassland.glb".to_string(),
        BiomeType::Forest => "models/biomes/core/forest.glb".to_string(),
        BiomeType::Mountain => "models/biomes/core/mountain.glb".to_string(),
        BiomeType::Desert => "models/biomes/core/desert.glb".to_string(),
        BiomeType::Swamp => "models/biomes/core/swamp.glb".to_string(),
        BiomeType::Water => "models/biomes/core/water.glb".to_string(),
        BiomeType::Snow => "models/biomes/core/snow.glb".to_string(),
        BiomeType::Lava => "models/biomes/core/lava.glb".to_string(),
        BiomeType::Void => "models/biomes/core/void.glb".to_string(),
        
        // Transitional biomes
        BiomeType::ForestGrassland => "models/biomes/transitional/forest_grassland.glb".to_string(),
        BiomeType::MountainForest => "models/biomes/transitional/mountain_forest.glb".to_string(),
        BiomeType::DesertMountain => "models/biomes/transitional/desert_mountain.glb".to_string(),
        BiomeType::SwampWater => "models/biomes/transitional/swamp_water.glb".to_string(),
        BiomeType::SnowMountain => "models/biomes/transitional/snow_mountain.glb".to_string(),
        
        // Corrupted variants
        BiomeType::CorruptedGrassland => "models/biomes/corrupted/grassland.glb".to_string(),
        BiomeType::CorruptedForest => "models/biomes/corrupted/forest.glb".to_string(),
        BiomeType::CorruptedMountain => "models/biomes/corrupted/mountain.glb".to_string(),
        BiomeType::CorruptedDesert => "models/biomes/corrupted/desert.glb".to_string(),
        BiomeType::CorruptedSwamp => "models/biomes/corrupted/swamp.glb".to_string(),
        BiomeType::CorruptedWater => "models/biomes/corrupted/water.glb".to_string(),
        BiomeType::CorruptedSnow => "models/biomes/corrupted/snow.glb".to_string(),
        
        // Void-touched variants
        BiomeType::VoidGrassland => "models/biomes/void/grassland.glb".to_string(),
        BiomeType::VoidForest => "models/biomes/void/forest.glb".to_string(),
        BiomeType::VoidMountain => "models/biomes/void/mountain.glb".to_string(),
        BiomeType::VoidDesert => "models/biomes/void/desert.glb".to_string(),
        BiomeType::VoidSwamp => "models/biomes/void/swamp.glb".to_string(),
        BiomeType::VoidWater => "models/biomes/void/water.glb".to_string(),
        BiomeType::VoidSnow => "models/biomes/void/snow.glb".to_string(),
        BiomeType::VoidLava => "models/biomes/void/lava.glb".to_string(),
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
