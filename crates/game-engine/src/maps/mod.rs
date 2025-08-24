use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use hexx::{Hex, HexLayout};
use std::collections::HashMap;

/// Map system integrating bevy_ecs_tilemap for efficient hex tile rendering
/// This provides the foundation for the MapsAgent's hex world generation
pub struct MapsPlugin;

impl Plugin for MapsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TilemapPlugin)
            .init_resource::<TilemapAssets>()
            .init_resource::<MapConfiguration>()
            .add_systems(Startup, setup_tilemap_assets)
            .add_systems(Update, (
                update_tilemap_from_dread,
                process_corruption_spread,
                handle_tile_interactions,
                update_tile_visuals,
            ).chain());
    }
}

/// Resource holding tilemap texture handles for different dread levels
#[derive(Resource, Default)]
pub struct TilemapAssets {
    /// Texture atlases for each dread level (0-4)
    pub texture_atlases: HashMap<u8, Handle<Image>>,
    /// Corruption overlay textures
    pub corruption_overlays: HashMap<u8, Handle<Image>>,
    /// Biome-specific tile indices
    pub biome_indices: HashMap<String, TileTextureIndex>,
}

/// Configuration for the hex tilemap
#[derive(Resource)]
pub struct MapConfiguration {
    pub hex_layout: HexLayout,
    pub map_size: TilemapSize,
    pub tile_size: TilemapTileSize,
    pub grid_size: TilemapGridSize,
    pub current_dread_level: u8,
}

impl Default for MapConfiguration {
    fn default() -> Self {
        Self {
            hex_layout: HexLayout::POINTY,
            map_size: TilemapSize { x: 100, y: 100 },
            tile_size: TilemapTileSize { x: 32.0, y: 32.0 },
            grid_size: TilemapGridSize { x: 32.0, y: 32.0 },
            current_dread_level: 0,
        }
    }
}

/// Component for tiles that can corrupt
#[derive(Component)]
pub struct CorruptibleTile {
    pub base_biome: BiomeType,
    pub corruption_level: f32,
    pub corruption_resistance: f32,
}

/// Biome types that affect tile appearance and corruption spread
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BiomeType {
    Grassland,
    Forest,
    Desert,
    Swamp,
    Mountain,
    Volcanic,
    Corrupted,
}

impl BiomeType {
    /// Get the base texture index for this biome
    fn get_base_index(&self) -> TileTextureIndex {
        match self {
            BiomeType::Grassland => TileTextureIndex(0),
            BiomeType::Forest => TileTextureIndex(10),
            BiomeType::Desert => TileTextureIndex(20),
            BiomeType::Swamp => TileTextureIndex(30),
            BiomeType::Mountain => TileTextureIndex(40),
            BiomeType::Volcanic => TileTextureIndex(50),
            BiomeType::Corrupted => TileTextureIndex(60),
        }
    }
    
    /// Get corruption resistance for this biome
    fn get_corruption_resistance(&self) -> f32 {
        match self {
            BiomeType::Grassland => 0.3,
            BiomeType::Forest => 0.5,
            BiomeType::Desert => 0.4,
            BiomeType::Swamp => 0.2,  // Swamps corrupt easily
            BiomeType::Mountain => 0.7,
            BiomeType::Volcanic => 0.1, // Already chaotic
            BiomeType::Corrupted => 0.0,
        }
    }
}

/// Create the initial tilemap with AI-generated tile data
pub fn create_hex_tilemap(
    mut commands: Commands,
    tilemap_assets: Res<TilemapAssets>,
    map_config: Res<MapConfiguration>,
    asset_server: Res<AssetServer>,
) -> Entity {
    let texture_handle = tilemap_assets.texture_atlases
        .get(&map_config.current_dread_level)
        .cloned()
        .unwrap_or_else(|| asset_server.load("textures/hex_tiles_dread_0.png"));
    
    let map_size = map_config.map_size;
    let grid_size = map_config.grid_size;
    let map_type = TilemapType::Hexagon(HexCoordSystem::RowEven);
    
    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(map_size);
    
    // Generate tiles based on AI-generated world data
    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let biome = determine_biome_for_position(x, y, map_config.current_dread_level);
            
            let tile_entity = commands
                .spawn((
                    TileBundle {
                        position: tile_pos,
                        tilemap_id: TilemapId(tilemap_entity),
                        texture_index: biome.get_base_index(),
                        ..Default::default()
                    },
                    CorruptibleTile {
                        base_biome: biome,
                        corruption_level: 0.0,
                        corruption_resistance: biome.get_corruption_resistance(),
                    },
                ))
                .id();
            
            tile_storage.set(&tile_pos, tile_entity);
        }
    }
    
    commands.entity(tilemap_entity).insert((
        TilemapBundle {
            grid_size,
            map_type,
            size: map_size,
            storage: tile_storage,
            texture: TilemapTexture::Single(texture_handle),
            tile_size: map_config.tile_size,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        },
        Name::new("HexTilemap"),
    ));
    
    tilemap_entity
}

/// Determine biome type based on position and dread level
fn determine_biome_for_position(x: u32, y: u32, dread_level: u8) -> BiomeType {
    // This would be replaced with AI-generated biome data
    // For now, use procedural generation
    let noise_value = ((x as f32 * 0.1).sin() + (y as f32 * 0.1).cos()) * 0.5 + 0.5;
    
    match dread_level {
        0..=1 => {
            // Peaceful biomes
            if noise_value < 0.3 {
                BiomeType::Grassland
            } else if noise_value < 0.6 {
                BiomeType::Forest
            } else {
                BiomeType::Mountain
            }
        }
        2..=3 => {
            // Transitioning to darker biomes
            if noise_value < 0.2 {
                BiomeType::Swamp
            } else if noise_value < 0.5 {
                BiomeType::Desert
            } else if noise_value < 0.8 {
                BiomeType::Mountain
            } else {
                BiomeType::Volcanic
            }
        }
        4 => {
            // Horror - everything corrupted
            BiomeType::Corrupted
        }
        _ => BiomeType::Grassland,
    }
}

/// Setup tilemap texture assets
fn setup_tilemap_assets(
    mut tilemap_assets: ResMut<TilemapAssets>,
    asset_server: Res<AssetServer>,
) {
    // Load texture atlases for each dread level
    for dread_level in 0..=4 {
        let texture_path = format!("textures/hex_tiles_dread_{}.png", dread_level);
        let texture_handle = asset_server.load(texture_path);
        tilemap_assets.texture_atlases.insert(dread_level, texture_handle);
        
        // Load corruption overlay textures
        let overlay_path = format!("textures/corruption_overlay_{}.png", dread_level);
        let overlay_handle = asset_server.load(overlay_path);
        tilemap_assets.corruption_overlays.insert(dread_level, overlay_handle);
    }
    
    // Setup biome indices
    tilemap_assets.biome_indices.insert("grassland".to_string(), TileTextureIndex(0));
    tilemap_assets.biome_indices.insert("forest".to_string(), TileTextureIndex(10));
    tilemap_assets.biome_indices.insert("desert".to_string(), TileTextureIndex(20));
    tilemap_assets.biome_indices.insert("swamp".to_string(), TileTextureIndex(30));
    tilemap_assets.biome_indices.insert("mountain".to_string(), TileTextureIndex(40));
    tilemap_assets.biome_indices.insert("volcanic".to_string(), TileTextureIndex(50));
    tilemap_assets.biome_indices.insert("corrupted".to_string(), TileTextureIndex(60));
}

/// Update tilemap visuals based on dread level
fn update_tilemap_from_dread(
    mut tilemap_query: Query<(&mut TilemapTexture, &TileStorage)>,
    mut tile_query: Query<(&mut TileTextureIndex, &CorruptibleTile)>,
    tilemap_assets: Res<TilemapAssets>,
    dread_state: Res<crate::resources::DreadState>,
    mut map_config: ResMut<MapConfiguration>,
) {
    if dread_state.current_level != map_config.current_dread_level {
        map_config.current_dread_level = dread_state.current_level;
        
        // Update tilemap texture atlas
        if let Some(new_texture) = tilemap_assets.texture_atlases.get(&dread_state.current_level) {
            for (mut tilemap_texture, tile_storage) in tilemap_query.iter_mut() {
                *tilemap_texture = TilemapTexture::Single(new_texture.clone());
                
                // Update individual tile indices based on corruption
                for tile_entity in tile_storage.iter().flatten() {
                    if let Ok((mut texture_index, corruptible)) = tile_query.get_mut(*tile_entity) {
                        let corruption_offset = (corruptible.corruption_level * 5.0) as u32;
                        let base_index = corruptible.base_biome.get_base_index();
                        *texture_index = TileTextureIndex(base_index.0 + corruption_offset);
                    }
                }
            }
        }
    }
}

/// Process corruption spreading across tiles
fn process_corruption_spread(
    mut tile_query: Query<(&TilePos, &mut CorruptibleTile, &TilemapId)>,
    tilemap_query: Query<&TileStorage>,
    dread_state: Res<crate::resources::DreadState>,
    time: Res<Time>,
) {
    let corruption_rate = dread_state.current_level as f32 * 0.1;
    let delta = time.delta_secs();
    
    // Collect corruption sources
    let mut corruption_sources = Vec::new();
    for (pos, corruptible, _) in tile_query.iter() {
        if corruptible.corruption_level > 0.8 {
            corruption_sources.push(*pos);
        }
    }
    
    // Spread corruption from sources
    for source_pos in corruption_sources {
        // Get neighboring tiles
        let neighbors = get_hex_neighbors(source_pos);
        
        for neighbor_pos in neighbors {
            // Find the neighbor tile entity
            for (pos, mut corruptible, tilemap_id) in tile_query.iter_mut() {
                if *pos == neighbor_pos {
                    // Apply corruption based on resistance
                    let corruption_delta = corruption_rate * delta * (1.0 - corruptible.corruption_resistance);
                    corruptible.corruption_level = (corruptible.corruption_level + corruption_delta).min(1.0);
                    
                    // Change biome type if fully corrupted
                    if corruptible.corruption_level >= 1.0 {
                        corruptible.base_biome = BiomeType::Corrupted;
                    }
                }
            }
        }
    }
}

/// Get neighboring hex positions
fn get_hex_neighbors(pos: TilePos) -> Vec<TilePos> {
    let hex = Hex::new(pos.x as i32, pos.y as i32);
    hex.all_neighbors()
        .into_iter()
        .filter_map(|neighbor| {
            if neighbor.x >= 0 && neighbor.y >= 0 {
                Some(TilePos {
                    x: neighbor.x as u32,
                    y: neighbor.y as u32,
                })
            } else {
                None
            }
        })
        .collect()
}

/// Handle player interactions with tiles
fn handle_tile_interactions(
    tile_query: Query<(&TilePos, &CorruptibleTile)>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    buttons: Res<ButtonInput<MouseButton>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        if let Ok(window) = windows.get_single() {
            if let Some(cursor_pos) = window.cursor_position() {
                // Convert cursor position to tile position
                // This would need proper coordinate conversion in full implementation
                if let Ok((camera, camera_transform)) = camera_query.get_single() {
                    // Ray cast to find clicked tile
                    for (tile_pos, corruptible) in tile_query.iter() {
                        // Check if this tile was clicked
                        info!(
                            "Tile at {:?} clicked - Biome: {:?}, Corruption: {:.2}",
                            tile_pos, corruptible.base_biome, corruptible.corruption_level
                        );
                    }
                }
            }
        }
    }
}

/// Update visual effects on tiles based on corruption
fn update_tile_visuals(
    mut tile_query: Query<(&mut TileColor, &CorruptibleTile), Changed<CorruptibleTile>>,
) {
    for (mut tile_color, corruptible) in tile_query.iter_mut() {
        // Darken tiles based on corruption level
        let darkness = 1.0 - (corruptible.corruption_level * 0.5);
        tile_color.0 = Color::srgb(darkness, darkness, darkness);
        
        // Add purple tint for high corruption
        if corruptible.corruption_level > 0.7 {
            let purple_tint = corruptible.corruption_level - 0.7;
            tile_color.0 = Color::srgb(
                darkness,
                darkness * (1.0 - purple_tint),
                darkness + purple_tint * 0.3,
            );
        }
    }
}

/// Integration with AI-generated map data
pub fn load_ai_generated_map(
    commands: &mut Commands,
    tilemap_assets: &TilemapAssets,
    map_data: &crate::resources::GeneratedMapData,
) -> Entity {
    let map_size = TilemapSize {
        x: map_data.width,
        y: map_data.height,
    };
    
    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(map_size);
    
    // Create tiles from AI-generated data
    for (hex_pos, tile_data) in &map_data.tiles {
        let tile_pos = TilePos {
            x: hex_pos.x as u32,
            y: hex_pos.y as u32,
        };
        
        let tile_entity = commands
            .spawn((
                TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: TileTextureIndex(tile_data.texture_index),
                    ..Default::default()
                },
                CorruptibleTile {
                    base_biome: tile_data.biome,
                    corruption_level: tile_data.initial_corruption,
                    corruption_resistance: tile_data.biome.get_corruption_resistance(),
                },
            ))
            .id();
        
        tile_storage.set(&tile_pos, tile_entity);
    }
    
    tilemap_entity
}

/// Tile data structure for AI-generated maps
#[derive(Clone, Debug)]
pub struct TileData {
    pub texture_index: u32,
    pub biome: BiomeType,
    pub initial_corruption: f32,
    pub interactive_objects: Vec<String>,
}
