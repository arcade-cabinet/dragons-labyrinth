//! Hex world system for tactical overworld exploration
//!
//! This system handles the 2.5D hex-based overworld where players
//! explore between 3D labyrinth sections.

use bevy::prelude::*;
use hexx::{Hex, HexLayout, HexOrientation};
use crate::resources::DreadLevel;

/// Plugin for the hex world system
pub struct HexWorldPlugin;

impl Plugin for HexWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_hex_world)
            .add_systems(Update, (
                update_hex_visibility,
                handle_hex_movement,
                apply_weather_effects,
                update_corruption_visuals,
            ))
            .insert_resource(HexWorldState::default())
            .insert_resource(HexLayout {
                orientation: HexOrientation::Pointy,
                origin: Vec2::ZERO,
                hex_size: Vec2::new(1.0, 1.0),
                invert_x: false,
                invert_y: false,
            });
    }
}

/// State of the hex world
#[derive(Resource, Default)]
pub struct HexWorldState {
    pub current_hex: Hex,
    pub visible_radius: u32,
    pub explored_hexes: Vec<Hex>,
    pub weather: Weather,
    pub time_of_day: f32, // 0.0 to 1.0
}

/// Weather system that affects visibility and movement
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Weather {
    Clear,
    Rain,
    Storm,
    Fog,
    Snow,
    AshFall,
    VoidStorm,
}

impl Default for Weather {
    fn default() -> Self {
        Weather::Clear
    }
}

/// Component for hex tiles
#[derive(Component)]
pub struct HexTile {
    pub hex: Hex,
    pub tile_type: TileType,
    pub elevation: i8,
    pub passable: bool,
    pub corruption: f32,
    pub discovered: bool,
}

/// Types of hex tiles
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TileType {
    // Natural terrain
    Grass,
    Forest,
    Mountain,
    Water,
    Swamp,
    Desert,
    Snow,
    
    // Structures
    Village,
    Ruins,
    Shrine,
    Labyrinth,
    
    // Corrupted
    VoidRift,
    CorruptedLand,
    CrystalField,
}

/// Component for entities on hex tiles
#[derive(Component)]
pub struct HexPosition {
    pub hex: Hex,
}

/// Movement component for hex-based movement
#[derive(Component)]
pub struct HexMovement {
    pub speed: f32,
    pub destination: Option<Hex>,
    pub path: Vec<Hex>,
}

/// Setup the initial hex world
fn setup_hex_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    hex_layout: Res<HexLayout>,
) {
    // Generate initial visible area
    let center = Hex::ZERO;
    let radius = 10;
    
    for hex in center.range(radius as u32) {
        spawn_hex_tile(
            &mut commands,
            &mut meshes,
            &mut materials,
            &hex_layout,
            hex,
            determine_tile_type(hex),
        );
    }
}

/// Spawn a single hex tile
fn spawn_hex_tile(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    layout: &HexLayout,
    hex: Hex,
    tile_type: TileType,
) {
    let world_pos = layout.hex_to_world_pos(hex);
    let elevation = calculate_elevation(hex, tile_type);
    let corruption = calculate_corruption(hex);
    
    // Create hex mesh
    let mesh = meshes.add(create_hex_mesh());
    
    // Determine material based on tile type and corruption
    let material = materials.add(StandardMaterial {
        base_color: get_tile_color(tile_type, corruption),
        perceptual_roughness: get_tile_roughness(tile_type),
        metallic: if corruption > 0.5 { 0.2 } else { 0.0 },
        ..default()
    });
    
    commands.spawn((
        Mesh3d(mesh),
        MeshMaterial3d(material),
        Transform::from_xyz(
            world_pos.x,
            elevation as f32 * 0.5,
            world_pos.y,
        ),
        HexTile {
            hex,
            tile_type,
            elevation,
            passable: is_passable(tile_type),
            corruption,
            discovered: false,
        },
    ));
}

/// Create a hexagonal mesh
fn create_hex_mesh() -> Mesh {
    // Create a hexagon with 6 vertices
    let mut mesh = Mesh::new(bevy::render::mesh::PrimitiveTopology::TriangleList, bevy::render::render_resource::RenderAssetUsages::RENDER_WORLD);
    
    // Hex vertices (pointy-top orientation)
    let angle_step = std::f32::consts::TAU / 6.0;
    let mut vertices = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    
    // Center vertex
    vertices.push([0.0, 0.0, 0.0]);
    normals.push([0.0, 1.0, 0.0]);
    uvs.push([0.5, 0.5]);
    
    // Outer vertices
    for i in 0..6 {
        let angle = i as f32 * angle_step;
        let x = angle.cos();
        let z = angle.sin();
        vertices.push([x, 0.0, z]);
        normals.push([0.0, 1.0, 0.0]);
        uvs.push([(x + 1.0) * 0.5, (z + 1.0) * 0.5]);
    }
    
    // Create triangles
    let mut indices = Vec::new();
    for i in 0..6 {
        indices.push(0);
        indices.push(i + 1);
        indices.push(if i == 5 { 1 } else { i + 2 });
    }
    
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(bevy::render::mesh::Indices::U32(indices));
    
    mesh
}

/// Determine tile type based on hex position (procedural generation)
fn determine_tile_type(hex: Hex) -> TileType {
    // Simple procedural generation based on distance from center
    let distance = hex.length();
    
    if distance == 0 {
        TileType::Village
    } else if distance < 3 {
        TileType::Grass
    } else if distance < 6 {
        if hex.x % 3 == 0 {
            TileType::Forest
        } else {
            TileType::Grass
        }
    } else if distance < 10 {
        if hex.y % 4 == 0 {
            TileType::Mountain
        } else if hex.x % 5 == 0 {
            TileType::Swamp
        } else {
            TileType::Forest
        }
    } else {
        TileType::CorruptedLand
    }
}

/// Calculate elevation for a hex tile
fn calculate_elevation(hex: Hex, tile_type: TileType) -> i8 {
    match tile_type {
        TileType::Mountain => 3,
        TileType::Forest => 1,
        TileType::Water => -1,
        TileType::Swamp => -1,
        _ => 0,
    }
}

/// Calculate corruption level for a hex
fn calculate_corruption(hex: Hex) -> f32 {
    let distance = hex.length() as f32;
    (distance / 20.0).min(1.0)
}

/// Get color for a tile type
fn get_tile_color(tile_type: TileType, corruption: f32) -> Color {
    let base_color = match tile_type {
        TileType::Grass => Color::srgb(0.2, 0.7, 0.2),
        TileType::Forest => Color::srgb(0.1, 0.5, 0.1),
        TileType::Mountain => Color::srgb(0.5, 0.5, 0.5),
        TileType::Water => Color::srgb(0.2, 0.4, 0.8),
        TileType::Swamp => Color::srgb(0.3, 0.4, 0.2),
        TileType::Desert => Color::srgb(0.9, 0.8, 0.6),
        TileType::Snow => Color::srgb(0.9, 0.9, 1.0),
        TileType::Village => Color::srgb(0.6, 0.5, 0.4),
        TileType::Ruins => Color::srgb(0.4, 0.4, 0.4),
        TileType::Shrine => Color::srgb(0.7, 0.7, 0.8),
        TileType::Labyrinth => Color::srgb(0.2, 0.1, 0.3),
        TileType::VoidRift => Color::srgb(0.1, 0.0, 0.2),
        TileType::CorruptedLand => Color::srgb(0.3, 0.1, 0.3),
        TileType::CrystalField => Color::srgb(0.4, 0.2, 0.6),
    };
    
    // Mix with corruption color
    if corruption > 0.0 {
        let corruption_color = Color::srgb(0.5, 0.0, 0.5);
        base_color.mix(&corruption_color, corruption)
    } else {
        base_color
    }
}

/// Get roughness for a tile type
fn get_tile_roughness(tile_type: TileType) -> f32 {
    match tile_type {
        TileType::Water => 0.1,
        TileType::Snow => 0.2,
        TileType::CrystalField => 0.1,
        _ => 0.8,
    }
}

/// Check if a tile type is passable
fn is_passable(tile_type: TileType) -> bool {
    !matches!(tile_type, TileType::Mountain | TileType::Water)
}

/// Update hex visibility based on player position
fn update_hex_visibility(
    mut hex_tiles: Query<(&HexTile, &mut Visibility)>,
    hex_state: Res<HexWorldState>,
) {
    for (tile, mut visibility) in &mut hex_tiles {
        let distance = tile.hex.distance_to(hex_state.current_hex) as u32;
        
        // Apply fog of war
        *visibility = if distance <= hex_state.visible_radius {
            Visibility::Visible
        } else if hex_state.explored_hexes.contains(&tile.hex) {
            Visibility::Hidden  // Explored but not currently visible
        } else {
            Visibility::Hidden
        };
    }
}

/// Handle hex-based movement
fn handle_hex_movement(
    mut query: Query<(&mut HexPosition, &mut HexMovement, &mut Transform)>,
    time: Res<Time>,
    hex_layout: Res<HexLayout>,
) {
    for (mut pos, mut movement, mut transform) in &mut query {
        if let Some(destination) = movement.destination {
            if pos.hex != destination {
                // Simple movement towards destination
                let current_world = hex_layout.hex_to_world_pos(pos.hex);
                let dest_world = hex_layout.hex_to_world_pos(destination);
                
                let direction = (dest_world - current_world).normalize();
                let distance = (dest_world - current_world).length();
                
                let move_distance = movement.speed * time.delta_secs();
                
                if move_distance >= distance {
                    // Reached destination hex
                    pos.hex = destination;
                    transform.translation.x = dest_world.x;
                    transform.translation.z = dest_world.y;
                    
                    // Move to next hex in path if available
                    if !movement.path.is_empty() {
                        movement.destination = Some(movement.path.remove(0));
                    } else {
                        movement.destination = None;
                    }
                } else {
                    // Move towards destination
                    transform.translation.x += direction.x * move_distance;
                    transform.translation.z += direction.y * move_distance;
                }
            }
        }
    }
}

/// Apply weather effects to the world
fn apply_weather_effects(
    hex_state: Res<HexWorldState>,
    mut fog: Query<&mut FogSettings>,
    mut ambient: Query<&mut AmbientLight>,
) {
    // Apply weather-based visibility and lighting changes
    let (fog_color, fog_falloff, ambient_brightness) = match hex_state.weather {
        Weather::Clear => (Color::srgb(0.8, 0.9, 1.0), FogFalloff::Linear { start: 50.0, end: 200.0 }, 0.5),
        Weather::Rain => (Color::srgb(0.5, 0.5, 0.6), FogFalloff::Linear { start: 30.0, end: 100.0 }, 0.3),
        Weather::Storm => (Color::srgb(0.2, 0.2, 0.3), FogFalloff::Linear { start: 20.0, end: 60.0 }, 0.2),
        Weather::Fog => (Color::srgb(0.7, 0.7, 0.7), FogFalloff::Linear { start: 10.0, end: 40.0 }, 0.4),
        Weather::Snow => (Color::srgb(0.9, 0.9, 1.0), FogFalloff::Linear { start: 25.0, end: 80.0 }, 0.6),
        Weather::AshFall => (Color::srgb(0.3, 0.3, 0.3), FogFalloff::Linear { start: 15.0, end: 50.0 }, 0.2),
        Weather::VoidStorm => (Color::srgb(0.1, 0.0, 0.2), FogFalloff::Linear { start: 10.0, end: 30.0 }, 0.1),
    };
    
    for mut fog_settings in &mut fog {
        fog_settings.color = fog_color;
        fog_settings.falloff = fog_falloff;
    }
    
    for mut light in &mut ambient {
        light.brightness = ambient_brightness;
    }
}

/// Update corruption visual effects
fn update_corruption_visuals(
    mut hex_tiles: Query<(&HexTile, &mut MeshMaterial3d<StandardMaterial>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    dread: Res<DreadLevel>,
    time: Res<Time>,
) {
    let corruption_multiplier = match dread.0 {
        0 => 0.0,
        1 => 0.2,
        2 => 0.5,
        3 => 0.8,
        4 => 1.0,
        _ => 1.0,
    };
    
    for (tile, material_handle) in &mut hex_tiles {
        if let Some(material) = materials.get_mut(&material_handle.0) {
            // Pulse corruption based on dread level
            if tile.corruption > 0.0 {
                let pulse = (time.elapsed_secs() * 2.0).sin() * 0.5 + 0.5;
                let corruption_intensity = tile.corruption * corruption_multiplier * pulse;
                
                // Add purple glow to corrupted tiles
                material.emissive = LinearRgba::new(
                    corruption_intensity * 0.5,
                    0.0,
                    corruption_intensity,
                    1.0,
                );
            }
        }
    }
}
