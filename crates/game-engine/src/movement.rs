use bevy::prelude::*;
use hexx::*;
use dragons_core::{components::*, resources::*};

/// Plugin for hex-based movement system
pub struct HexMovementPlugin;

impl Plugin for HexMovementPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(HexLayout {
                orientation: HexOrientation::Flat,
                origin: Vec2::ZERO,
                hex_size: Vec2::splat(1.0),
                invert_x: false,
                invert_y: false,
            })
            .init_resource::<MovementState>()
            .add_systems(Update, (
                handle_keyboard_movement,
                handle_mouse_movement,
                update_world_positions,
                smooth_movement_interpolation,
            ).chain());
    }
}

#[derive(Resource)]
pub struct MovementState {
    pub selected_hex: Option<Hex>,
    pub path: Vec<Hex>,
    pub movement_speed: f32,
    pub running: bool,
}

impl Default for MovementState {
    fn default() -> Self {
        Self {
            selected_hex: None,
            path: Vec::new(),
            movement_speed: 5.0,
            running: false,
        }
    }
}

#[derive(Component)]
pub struct MovementTarget {
    pub target: Vec3,
    pub speed: f32,
}

#[derive(Component)]
pub struct MovementPoints {
    pub current: u32,
    pub max: u32,
}

impl Default for MovementPoints {
    fn default() -> Self {
        Self {
            current: 3,
            max: 3,
        }
    }
}

/// Handle hex movement with Q,W,E,A,S,D keys
fn handle_keyboard_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut HexPosition, With<Player>>,
    hex_world: Res<HexWorld>,
    mut movement_state: ResMut<MovementState>,
) {
    let Ok(mut hex_pos) = player_query.get_single_mut() else {
        return;
    };
    
    // Check for shift (running)
    movement_state.running = keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight);
    
    let mut direction = None;
    
    // Map keys to hex directions (flat-topped hexagon)
    if keyboard.just_pressed(KeyCode::KeyW) {
        direction = Some(Direction::TopRight);
    } else if keyboard.just_pressed(KeyCode::KeyE) {
        direction = Some(Direction::Right);
    } else if keyboard.just_pressed(KeyCode::KeyD) {
        direction = Some(Direction::BottomRight);
    } else if keyboard.just_pressed(KeyCode::KeyS) {
        direction = Some(Direction::BottomLeft);
    } else if keyboard.just_pressed(KeyCode::KeyA) {
        direction = Some(Direction::Left);
    } else if keyboard.just_pressed(KeyCode::KeyQ) {
        direction = Some(Direction::TopLeft);
    }
    
    if let Some(dir) = direction {
        let new_hex = hex_pos.0.neighbor(dir);
        
        // Check if the new position is valid and passable
        if let Some(tile) = hex_world.tiles.get(&new_hex) {
            if tile.passable {
                hex_pos.0 = new_hex;
                info!("Player moved to {:?}", new_hex);
            } else {
                warn!("Cannot move to {:?} - tile not passable", new_hex);
            }
        }
    }
}

/// Handle mouse click movement with pathfinding
fn handle_mouse_movement(
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    hex_layout: Res<HexLayout>,
    mut player_query: Query<(&mut HexPosition, Entity), With<Player>>,
    mut movement_state: ResMut<MovementState>,
    hex_world: Res<HexWorld>,
    mut commands: Commands,
) {
    if !mouse_button.just_pressed(MouseButton::Left) {
        return;
    }
    
    let Ok(window) = windows.get_single() else {
        return;
    };
    
    let Some(cursor_position) = window.cursor_position() else {
        return;
    };
    
    let Ok((camera, camera_transform)) = camera_query.get_single() else {
        return;
    };
    
    // Cast ray from camera through cursor position
    let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };
    
    // Intersect ray with ground plane (y = 0)
    let t = -ray.origin.y / ray.direction.y;
    if t < 0.0 {
        return;
    }
    
    let world_pos = ray.origin + ray.direction * t;
    let hex_pos = hex_layout.world_pos_to_hex(Vec2::new(world_pos.x, world_pos.z));
    
    // Check if hex is valid and passable
    if !hex_world.tiles.get(&hex_pos).map_or(false, |tile| tile.passable) {
        warn!("Cannot move to {:?} - invalid or impassable tile", hex_pos);
        return;
    }
    
    let Ok((mut player_hex, player_entity)) = player_query.get_single_mut() else {
        return;
    };
    
    // Check for shift+click (running)
    let keyboard = windows.single().focused;
    
    // Simple pathfinding (A* would be better but for now just move directly)
    let path = find_path(player_hex.0, hex_pos, &hex_world);
    
    if !path.is_empty() {
        movement_state.path = path;
        movement_state.selected_hex = Some(hex_pos);
        
        // Add movement target component for smooth interpolation
        if let Some(next_hex) = movement_state.path.first() {
            let world_target = hex_layout.hex_to_world_pos(*next_hex);
            commands.entity(player_entity).insert(MovementTarget {
                target: Vec3::new(world_target.x, 0.5, world_target.y),
                speed: if movement_state.running { 10.0 } else { 5.0 },
            });
        }
        
        info!("Path to {:?}: {:?}", hex_pos, path);
    }
}

/// Simple pathfinding - just returns direct path for now
fn find_path(start: Hex, end: Hex, hex_world: &HexWorld) -> Vec<Hex> {
    // For now, just return the direct path
    // TODO: Implement proper A* pathfinding
    let mut path = Vec::new();
    let mut current = start;
    
    while current != end {
        // Find the neighbor closest to the target
        let neighbors = current.all_neighbors();
        let mut best_neighbor = current;
        let mut best_distance = current.unsigned_distance_to(end);
        
        for neighbor in neighbors {
            if let Some(tile) = hex_world.tiles.get(&neighbor) {
                if tile.passable {
                    let distance = neighbor.unsigned_distance_to(end);
                    if distance < best_distance {
                        best_distance = distance;
                        best_neighbor = neighbor;
                    }
                }
            }
        }
        
        if best_neighbor == current {
            // No path found
            break;
        }
        
        path.push(best_neighbor);
        current = best_neighbor;
    }
    
    path
}

/// Update world positions based on hex positions
fn update_world_positions(
    mut query: Query<(&HexPosition, &mut Transform), Changed<HexPosition>>,
    hex_layout: Res<HexLayout>,
) {
    for (hex_pos, mut transform) in query.iter_mut() {
        let world_pos = hex_layout.hex_to_world_pos(hex_pos.0);
        transform.translation.x = world_pos.x;
        transform.translation.z = world_pos.y;
        // Keep Y position for elevation
    }
}

/// Smooth movement interpolation
fn smooth_movement_interpolation(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &MovementTarget, &mut HexPosition)>,
    mut movement_state: ResMut<MovementState>,
    hex_layout: Res<HexLayout>,
    time: Res<Time>,
) {
    for (entity, mut transform, target, mut hex_pos) in query.iter_mut() {
        let direction = target.target - transform.translation;
        let distance = direction.length();
        
        if distance < 0.1 {
            // Reached target
            transform.translation = target.target;
            commands.entity(entity).remove::<MovementTarget>();
            
            // Check if there's more path to follow
            if !movement_state.path.is_empty() {
                movement_state.path.remove(0);
                if let Some(next_hex) = movement_state.path.first() {
                    hex_pos.0 = *next_hex;
                    let world_target = hex_layout.hex_to_world_pos(*next_hex);
                    commands.entity(entity).insert(MovementTarget {
                        target: Vec3::new(world_target.x, 0.5, world_target.y),
                        speed: target.speed,
                    });
                }
            }
        } else {
            // Move towards target
            let move_distance = target.speed * time.delta_seconds();
            if move_distance >= distance {
                transform.translation = target.target;
            } else {
                transform.translation += direction.normalize() * move_distance;
            }
        }
    }
}

/// Calculate movement cost based on terrain
pub fn calculate_movement_cost(tile: &HexTile, running: bool) -> u32 {
    let base_cost = match tile.tile_type {
        TileType::Grass => 1,
        TileType::Forest => 2,
        TileType::Swamp => 3,
        TileType::Stone => 1,
        TileType::Corrupted => 2,
    };
    
    if running {
        base_cost * 2
    } else {
        base_cost
    }
}
