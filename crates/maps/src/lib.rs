// Map generation system combining mapgen algorithms with bevy_ecs_tilemap visuals
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use mapgen::{
    Map, MapBuilder, 
    filter::{
        AreaStartingPosition, CullUnreachable, DistantExit,
        DrunkardsWalk, NearestCorridors, NoiseGenerator,
        CellularAutomata, BspInterior, BspRooms,
    },
    geometry::{Point, Rect},
};
use hexx::Hex;
use crate::components::*;
use crate::resources::*;
use crate::generators::*;

/// Map generation plugin for Dragon's Labyrinth
pub struct MapGenerationPlugin;

impl Plugin for MapGenerationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TilemapPlugin)
            .init_resource::<MapGeneratorState>()
            .add_systems(Startup, setup_tilemap)
            .add_systems(Update, (
                generate_maps_for_dread_level,
                update_tilemap_visuals,
                apply_corruption_to_tilemap,
            ));
    }
}

/// Map generator state tracking generation progress
#[derive(Resource, Default)]
pub struct MapGeneratorState {
    pub current_map: Option<GeneratedMap>,
    pub generation_seeds: HashMap<u8, u64>, // Dread level -> seed
    pub tile_storage: Option<TileStorage>,
    pub tilemap_entity: Option<Entity>,
}

/// Generated map with metadata
#[derive(Clone)]
pub struct GeneratedMap {
    pub map_data: Map,
    pub dread_level: u8,
    pub biome_type: BiomeType,
    pub spawn_point: Point,
    pub exit_point: Point,
    pub boss_locations: Vec<Point>,
    pub treasure_locations: Vec<Point>,
    pub corruption_zones: Vec<Rect>,
}

/// Map generation profiles for each dread stage
pub enum MapProfile {
    Peace,      // Open meadows, simple paths
    Unease,     // Darker forests, winding paths
    Dread,      // Swamps, mazes, dead ends
    Terror,     // Ruins, shifting geometry
    Horror,     // Labyrinth, non-euclidean spaces
}

impl MapProfile {
    /// Generate map based on dread level
    pub fn generate(&self, width: usize, height: usize, seed: u64) -> GeneratedMap {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        
        match self {
            MapProfile::Peace => Self::generate_peaceful_meadow(width, height, &mut rng),
            MapProfile::Unease => Self::generate_uneasy_forest(width, height, &mut rng),
            MapProfile::Dread => Self::generate_dread_swamp(width, height, &mut rng),
            MapProfile::Terror => Self::generate_terror_ruins(width, height, &mut rng),
            MapProfile::Horror => Self::generate_horror_labyrinth(width, height, &mut rng),
        }
    }
    
    /// Generate peaceful meadow map
    fn generate_peaceful_meadow(width: usize, height: usize, rng: &mut impl rand::Rng) -> GeneratedMap {
        let mut builder = MapBuilder::new(width, height);
        
        // Open map with gentle noise
        builder = builder.with(NoiseGenerator::new(0.3)); // Low density for open spaces
        builder = builder.with(AreaStartingPosition::new(mapgen::XStart::LEFT, mapgen::YStart::TOP));
        builder = builder.with(DistantExit::new());
        builder = builder.with(CullUnreachable::new());
        
        let map = builder.build_with_rng(rng);
        
        GeneratedMap {
            spawn_point: map.starting_point.unwrap_or(Point::new(1, 1)),
            exit_point: map.exit_point.unwrap_or(Point::new(width - 2, height - 2)),
            boss_locations: vec![],
            treasure_locations: Self::find_treasure_spots(&map, 3),
            corruption_zones: vec![],
            biome_type: BiomeType::Meadow,
            dread_level: 0,
            map_data: map,
        }
    }
    
    /// Generate uneasy forest map
    fn generate_uneasy_forest(width: usize, height: usize, rng: &mut impl rand::Rng) -> GeneratedMap {
        let mut builder = MapBuilder::new(width, height);
        
        // Denser forest with cellular automata
        builder = builder.with(NoiseGenerator::new(0.55)); // Higher density for forest
        builder = builder.with(CellularAutomata::new());
        builder = builder.with(AreaStartingPosition::new(mapgen::XStart::LEFT, mapgen::YStart::CENTER));
        builder = builder.with(DistantExit::new());
        builder = builder.with(CullUnreachable::new());
        
        let map = builder.build_with_rng(rng);
        
        // Add Hollow Caretaker boss location
        let boss_location = Point::new(width / 2, height / 2);
        
        GeneratedMap {
            spawn_point: map.starting_point.unwrap_or(Point::new(1, height / 2)),
            exit_point: map.exit_point.unwrap_or(Point::new(width - 2, height / 2)),
            boss_locations: vec![boss_location],
            treasure_locations: Self::find_treasure_spots(&map, 5),
            corruption_zones: vec![Rect::new(width/3, height/3, 10, 10)],
            biome_type: BiomeType::Forest,
            dread_level: 1,
            map_data: map,
        }
    }
    
    /// Generate dread swamp map
    fn generate_dread_swamp(width: usize, height: usize, rng: &mut impl rand::Rng) -> GeneratedMap {
        let mut builder = MapBuilder::new(width, height);
        
        // Complex swamp with multiple passes
        builder = builder.with(NoiseGenerator::new(0.6));
        builder = builder.with(DrunkardsWalk::winding_passages());
        builder = builder.with(CellularAutomata::new());
        builder = builder.with(AreaStartingPosition::new(mapgen::XStart::LEFT, mapgen::YStart::BOTTOM));
        builder = builder.with(DistantExit::new());
        builder = builder.with(CullUnreachable::new());
        
        let map = builder.build_with_rng(rng);
        
        // Add Forsaken Knight boss
        let boss_location = Point::new(3 * width / 4, height / 2);
        
        GeneratedMap {
            spawn_point: map.starting_point.unwrap_or(Point::new(1, height - 2)),
            exit_point: map.exit_point.unwrap_or(Point::new(width - 2, 1)),
            boss_locations: vec![boss_location],
            treasure_locations: Self::find_treasure_spots(&map, 7),
            corruption_zones: vec![
                Rect::new(width/4, height/4, 15, 15),
                Rect::new(width/2, height/2, 20, 20),
            ],
            biome_type: BiomeType::Swamp,
            dread_level: 2,
            map_data: map,
        }
    }
    
    /// Generate terror ruins map
    fn generate_terror_ruins(width: usize, height: usize, rng: &mut impl rand::Rng) -> GeneratedMap {
        let mut builder = MapBuilder::new(width, height);
        
        // BSP rooms for ruins structure
        builder = builder.with(BspRooms::new());
        builder = builder.with(NearestCorridors::new());
        builder = builder.with(NoiseGenerator::new(0.2)); // Add rubble
        builder = builder.with(AreaStartingPosition::new(mapgen::XStart::CENTER, mapgen::YStart::BOTTOM));
        builder = builder.with(DistantExit::new());
        builder = builder.with(CullUnreachable::new());
        
        let map = builder.build_with_rng(rng);
        
        // Multiple boss locations for companion betrayals
        let boss_locations = vec![
            Point::new(width / 3, height / 3),
            Point::new(2 * width / 3, 2 * height / 3),
        ];
        
        GeneratedMap {
            spawn_point: map.starting_point.unwrap_or(Point::new(width / 2, height - 2)),
            exit_point: map.exit_point.unwrap_or(Point::new(width / 2, 1)),
            boss_locations,
            treasure_locations: Self::find_treasure_spots(&map, 10),
            corruption_zones: vec![
                Rect::new(0, 0, width / 2, height / 2),
                Rect::new(width / 2, height / 2, width / 2, height / 2),
            ],
            biome_type: BiomeType::Ruins,
            dread_level: 3,
            map_data: map,
        }
    }
    
    /// Generate horror labyrinth map
    fn generate_horror_labyrinth(width: usize, height: usize, rng: &mut impl rand::Rng) -> GeneratedMap {
        let mut builder = MapBuilder::new(width, height);
        
        // Complex labyrinth with BSP interior
        builder = builder.with(BspInterior::new());
        builder = builder.with(DrunkardsWalk::winding_passages());
        builder = builder.with(CellularAutomata::new());
        builder = builder.with(AreaStartingPosition::new(mapgen::XStart::CENTER, mapgen::YStart::CENTER));
        builder = builder.with(DistantExit::new());
        
        let map = builder.build_with_rng(rng);
        
        // Dragon's lair at the center
        let dragon_location = Point::new(width / 2, height / 2);
        
        GeneratedMap {
            spawn_point: map.starting_point.unwrap_or(Point::new(width / 2, height / 2)),
            exit_point: map.exit_point.unwrap_or(Point::new(1, 1)),
            boss_locations: vec![dragon_location],
            treasure_locations: Self::find_treasure_spots(&map, 15),
            corruption_zones: vec![
                Rect::new(0, 0, width, height), // Entire map is corrupted
            ],
            biome_type: BiomeType::Labyrinth,
            dread_level: 4,
            map_data: map,
        }
    }
    
    /// Find suitable treasure locations
    fn find_treasure_spots(map: &Map, count: usize) -> Vec<Point> {
        let mut spots = Vec::new();
        let mut checked = 0;
        
        for y in 1..map.height - 1 {
            for x in 1..map.width - 1 {
                if checked >= count * 10 { break; }
                checked += 1;
                
                let idx = map.xy_idx(x, y);
                if map.is_walkable(idx) {
                    // Check if it's a dead end or interesting spot
                    let mut wall_count = 0;
                    for dy in -1..=1 {
                        for dx in -1..=1 {
                            if dx == 0 && dy == 0 { continue; }
                            let check_idx = map.xy_idx((x as i32 + dx) as usize, (y as i32 + dy) as usize);
                            if !map.is_walkable(check_idx) {
                                wall_count += 1;
                            }
                        }
                    }
                    
                    // Dead ends and corners make good treasure spots
                    if wall_count >= 5 {
                        spots.push(Point::new(x, y));
                        if spots.len() >= count { break; }
                    }
                }
            }
            if spots.len() >= count { break; }
        }
        
        spots
    }
}

/// Setup the tilemap system
fn setup_tilemap(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut map_gen_state: ResMut<MapGeneratorState>,
) {
    // Create tilemap entity
    let map_size = TilemapSize { x: 50, y: 50 };
    let tile_size = TilemapTileSize { x: 32.0, y: 32.0 };
    let grid_size = tile_size.into();
    
    let mut tile_storage = TileStorage::empty(map_size);
    let tilemap_entity = commands.spawn_empty().id();
    
    // Generate initial peaceful map
    let profile = MapProfile::Peace;
    let generated_map = profile.generate(50, 50, 12345);
    
    // Convert mapgen map to tilemap
    for y in 0..map_size.y {
        for x in 0..map_size.x {
            let tile_pos = TilePos { x, y };
            let idx = generated_map.map_data.xy_idx(x as usize, y as usize);
            
            let (texture_index, color) = if generated_map.map_data.is_walkable(idx) {
                (0u32, Color::srgb(0.3, 0.8, 0.3)) // Grass
            } else {
                (1u32, Color::srgb(0.5, 0.5, 0.5)) // Wall
            };
            
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: TileTextureIndex(texture_index),
                    color: TileColor(color),
                    ..default()
                })
                .id();
            
            tile_storage.set(&tile_pos, tile_entity);
        }
    }
    
    // Create tilemap bundle
    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type: TilemapType::Square,
        size: map_size,
        storage: tile_storage.clone(),
        texture: TilemapTexture::Single(asset_server.load("textures/tiles.png")),
        tile_size,
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
    
    // Store in resource
    map_gen_state.current_map = Some(generated_map);
    map_gen_state.tile_storage = Some(tile_storage);
    map_gen_state.tilemap_entity = Some(tilemap_entity);
}

/// Generate new maps based on dread level changes
fn generate_maps_for_dread_level(
    dread_state: Res<DreadState>,
    mut map_gen_state: ResMut<MapGeneratorState>,
    mut commands: Commands,
) {
    if !dread_state.is_changed() {
        return;
    }
    
    let profile = match dread_state.level {
        0 => MapProfile::Peace,
        1 => MapProfile::Unease,
        2 => MapProfile::Dread,
        3 => MapProfile::Terror,
        4 => MapProfile::Horror,
        _ => return,
    };
    
    // Generate new map with consistent seed for each dread level
    let seed = map_gen_state.generation_seeds
        .entry(dread_state.level)
        .or_insert_with(|| rand::random());
    
    let new_map = profile.generate(50, 50, *seed);
    
    info!("Generated {} map for dread level {}", 
        match new_map.biome_type {
            BiomeType::Meadow => "peaceful meadow",
            BiomeType::Forest => "uneasy forest",
            BiomeType::Swamp => "dread swamp",
            BiomeType::Ruins => "terror ruins",
            BiomeType::Labyrinth => "horror labyrinth",
        },
        dread_state.level
    );
    
    // Update tilemap visuals
    if let Some(tilemap_entity) = map_gen_state.tilemap_entity {
        if let Some(mut tile_storage) = map_gen_state.tile_storage.clone() {
            // Update tiles based on new map
            for y in 0..50 {
                for x in 0..50 {
                    let tile_pos = TilePos { x, y };
                    let idx = new_map.map_data.xy_idx(x as usize, y as usize);
                    
                    if let Some(tile_entity) = tile_storage.get(&tile_pos) {
                        let (texture_index, color) = get_tile_appearance(&new_map, idx);
                        
                        commands.entity(tile_entity).insert((
                            TileTextureIndex(texture_index),
                            TileColor(color),
                        ));
                    }
                }
            }
            
            map_gen_state.tile_storage = Some(tile_storage);
        }
    }
    
    map_gen_state.current_map = Some(new_map);
}

/// Update tilemap visuals based on corruption
fn update_tilemap_visuals(
    map_gen_state: Res<MapGeneratorState>,
    mut tile_query: Query<(&TilePos, &mut TileColor)>,
    dread_state: Res<DreadState>,
) {
    if let Some(current_map) = &map_gen_state.current_map {
        let corruption_intensity = dread_state.get_corruption_intensity();
        
        for (tile_pos, mut tile_color) in tile_query.iter_mut() {
            // Check if tile is in corruption zone
            let point = Point::new(tile_pos.x as usize, tile_pos.y as usize);
            let mut in_corruption = false;
            
            for zone in &current_map.corruption_zones {
                if zone.contains(point) {
                    in_corruption = true;
                    break;
                }
            }
            
            if in_corruption {
                // Apply corruption color shift
                let base_color = tile_color.0;
                let corrupted = Color::srgb(
                    base_color.to_srgba().red * (1.0 - corruption_intensity * 0.5) + corruption_intensity * 0.3,
                    base_color.to_srgba().green * (1.0 - corruption_intensity * 0.7),
                    base_color.to_srgba().blue * (1.0 - corruption_intensity * 0.7),
                );
                tile_color.0 = corrupted;
            }
        }
    }
}

/// Apply corruption effects to tilemap
fn apply_corruption_to_tilemap(
    mut commands: Commands,
    map_gen_state: Res<MapGeneratorState>,
    dread_state: Res<DreadState>,
    time: Res<Time>,
) {
    if let Some(current_map) = &map_gen_state.current_map {
        let corruption_spread_rate = dread_state.level as f32 * 0.1 * time.delta_seconds();
        
        // Spawn corruption particles at boss locations
        for boss_loc in &current_map.boss_locations {
            if rand::random::<f32>() < corruption_spread_rate {
                commands.spawn((
                    TransformBundle::from_transform(
                        Transform::from_xyz(
                            boss_loc.x as f32 * 32.0,
                            boss_loc.y as f32 * 32.0,
                            1.0
                        )
                    ),
                    VisualEffect {
                        effect_type: EffectType::Corruption,
                        duration: 2.0,
                        intensity: dread_state.level as f32 / 4.0,
                        dread_scaling: true,
                    },
                ));
            }
        }
    }
}

/// Get tile appearance based on map data and biome
fn get_tile_appearance(map: &GeneratedMap, idx: usize) -> (u32, Color) {
    let is_walkable = map.map_data.is_walkable(idx);
    
    match map.biome_type {
        BiomeType::Meadow => {
            if is_walkable {
                (0, Color::srgb(0.3, 0.8, 0.3)) // Bright grass
            } else {
                (1, Color::srgb(0.6, 0.6, 0.6)) // Light stone
            }
        },
        BiomeType::Forest => {
            if is_walkable {
                (2, Color::srgb(0.2, 0.5, 0.2)) // Dark grass
            } else {
                (3, Color::srgb(0.3, 0.2, 0.1)) // Tree/wood
            }
        },
        BiomeType::Swamp => {
            if is_walkable {
                (4, Color::srgb(0.2, 0.3, 0.1)) // Murky ground
            } else {
                (5, Color::srgb(0.1, 0.1, 0.05)) // Dead trees
            }
        },
        BiomeType::Ruins => {
            if is_walkable {
                (6, Color::srgb(0.4, 0.4, 0.4)) // Stone floor
            } else {
                (7, Color::srgb(0.2, 0.2, 0.2)) // Broken walls
            }
        },
        BiomeType::Labyrinth => {
            if is_walkable {
                (8, Color::srgb(0.1, 0.1, 0.1)) // Dark floor
            } else {
                (9, Color::srgb(0.05, 0.05, 0.05)) // Void walls
            }
        },
    }
}

use std::collections::HashMap;