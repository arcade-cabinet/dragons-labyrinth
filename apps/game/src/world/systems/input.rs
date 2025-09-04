use bevy::prelude::*;
use bevy::input::touch::TouchPhase;
use crate::world::components::{Player, HexPosition};
use crate::world::state::WorldState;
use dl_types::world::HexCoord;
use crate::utils::hex::{hex_to_world, world_to_hex, hex_distance};
use crate::spatial::SpatialContainer;

/// Cross-platform input system supporting touch, mouse, and keyboard
pub fn cross_platform_input_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut player_query: Query<(&mut Transform, &mut HexPosition), With<Player>>,
    mut world_state: ResMut<WorldState>,
    spatial_container: Res<SpatialContainer>,
) {
    if let Ok((mut player_transform, mut player_hex)) = player_query.get_single_mut() {
        // Handle keyboard input (desktop)
        if let Some(target_hex) = handle_keyboard_input(&keyboard, &player_hex) {
            move_player_to_hex(target_hex, &mut player_transform, &mut player_hex, &mut world_state);
        }
        
        // Handle mouse input (desktop)
        if let Some(target_hex) = handle_mouse_input(&mouse, &windows, &camera_query) {
            pathfind_and_move_player(target_hex, &mut player_transform, &mut player_hex, &mut world_state, &spatial_container);
        }
        
        // Handle touch input (mobile/tablet)
        if let Some(target_hex) = handle_touch_input(&touches, &windows, &camera_query) {
            pathfind_and_move_player(target_hex, &mut player_transform, &mut player_hex, &mut world_state, &spatial_container);
        }
    }
}

/// Handle keyboard input for hex movement (Q/W/E/A/S/D + Arrow keys)
fn handle_keyboard_input(keyboard: &Res<ButtonInput<KeyCode>>, current_hex: &HexPosition) -> Option<HexCoord> {
    let current = HexCoord::new(current_hex.q, current_hex.r);
    
    if keyboard.just_pressed(KeyCode::KeyW) || keyboard.just_pressed(KeyCode::ArrowUp) {
        Some(hex_neighbor(current, HexDirection::North))
    } else if keyboard.just_pressed(KeyCode::KeyS) || keyboard.just_pressed(KeyCode::ArrowDown) {
        Some(hex_neighbor(current, HexDirection::South))
    } else if keyboard.just_pressed(KeyCode::KeyA) || keyboard.just_pressed(KeyCode::ArrowLeft) {
        Some(hex_neighbor(current, HexDirection::NorthWest))
    } else if keyboard.just_pressed(KeyCode::KeyD) || keyboard.just_pressed(KeyCode::ArrowRight) {
        Some(hex_neighbor(current, HexDirection::SouthEast))
    } else if keyboard.just_pressed(KeyCode::KeyQ) {
        Some(hex_neighbor(current, HexDirection::NorthEast))
    } else if keyboard.just_pressed(KeyCode::KeyE) {
        Some(hex_neighbor(current, HexDirection::SouthWest))
    } else {
        None
    }
}

/// Handle mouse click input for tap-to-move
fn handle_mouse_input(
    mouse: &Res<ButtonInput<MouseButton>>,
    windows: &Query<&Window>,
    camera_query: &Query<(&Camera, &GlobalTransform)>,
) -> Option<HexCoord> {
    if mouse.just_pressed(MouseButton::Left) {
        if let Ok(window) = windows.get_single() {
            if let Some(cursor_pos) = window.cursor_position() {
                return screen_to_hex_coord(cursor_pos, camera_query);
            }
        }
    }
    None
}

/// Handle touch input for mobile tap-to-move
fn handle_touch_input(
    touches: &Res<Touches>,
    windows: &Query<&Window>,
    camera_query: &Query<(&Camera, &GlobalTransform)>,
) -> Option<HexCoord> {
    for touch in touches.iter() {
        if touch.phase == TouchPhase::Started {
            return screen_to_hex_coord(touch.position(), camera_query);
        }
    }
    None
}

/// Convert screen coordinates to hex coordinates
fn screen_to_hex_coord(
    screen_pos: Vec2,
    camera_query: &Query<(&Camera, &GlobalTransform)>,
) -> Option<HexCoord> {
    if let Ok((camera, camera_transform)) = camera_query.get_single() {
        // Convert screen position to world position
        if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, screen_pos) {
            let world_3d = Vec3::new(world_pos.x, 0.0, world_pos.y);
            return Some(world_to_hex(world_3d));
        }
    }
    None
}

/// Move player directly to adjacent hex (keyboard movement)
fn move_player_to_hex(
    target_hex: HexCoord,
    player_transform: &mut Transform,
    player_hex: &mut HexPosition,
    world_state: &mut ResMut<WorldState>,
) {
    let target_world = hex_to_world(target_hex);
    player_transform.translation = target_world + Vec3::new(0.0, 1.0, 0.0);
    player_hex.q = target_hex.q;
    player_hex.r = target_hex.r;
    world_state.player_hex = Some(target_hex);
    
    info!("Player moved to hex: {:?}", target_hex);
}

/// Use A* pathfinding to move player to distant hex (mouse/touch movement)
fn pathfind_and_move_player(
    target_hex: HexCoord,
    player_transform: &mut Transform,
    player_hex: &mut HexPosition,
    world_state: &mut ResMut<WorldState>,
    spatial_container: &Res<SpatialContainer>,
) {
    let current_hex = HexCoord::new(player_hex.q, player_hex.r);
    
    // For now, move directly to target (A* pathfinding to be implemented)
    // TODO: Implement proper A* pathfinding with obstacle avoidance
    let target_world = hex_to_world(target_hex);
    player_transform.translation = target_world + Vec3::new(0.0, 1.0, 0.0);
    player_hex.q = target_hex.q;
    player_hex.r = target_hex.r;
    world_state.player_hex = Some(target_hex);
    
    info!("Player pathfound to hex: {:?}", target_hex);
}

/// A* pathfinding implementation for hex grid
pub fn astar_pathfind(
    start: HexCoord,
    goal: HexCoord,
    spatial_container: &SpatialContainer,
) -> Option<Vec<HexCoord>> {
    use std::collections::{BinaryHeap, HashMap};
    use std::cmp::Ordering;
    
    #[derive(Copy, Clone, Eq, PartialEq)]
    struct State {
        cost: i32,
        position: HexCoord,
    }
    
    impl Ord for State {
        fn cmp(&self, other: &Self) -> Ordering {
            other.cost.cmp(&self.cost)
        }
    }
    
    impl PartialOrd for State {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }
    
    let mut heap = BinaryHeap::new();
    let mut distances: HashMap<HexCoord, i32> = HashMap::new();
    let mut came_from: HashMap<HexCoord, HexCoord> = HashMap::new();
    
    heap.push(State { cost: 0, position: start });
    distances.insert(start, 0);
    
    while let Some(State { cost, position }) = heap.pop() {
        if position == goal {
            // Reconstruct path
            let mut path = Vec::new();
            let mut current = goal;
            
            while current != start {
                path.push(current);
                current = came_from[&current];
            }
            path.push(start);
            path.reverse();
            
            return Some(path);
        }
        
        if cost > distances.get(&position).copied().unwrap_or(i32::MAX) {
            continue;
        }
        
        // Check all hex neighbors
        for direction in [
            HexDirection::North,
            HexDirection::NorthEast,
            HexDirection::SouthEast,
            HexDirection::South,
            HexDirection::SouthWest,
            HexDirection::NorthWest,
        ] {
            let neighbor = hex_neighbor(position, direction);
            let movement_cost = get_movement_cost(neighbor, spatial_container);
            let new_cost = cost + movement_cost;
            
            if new_cost < distances.get(&neighbor).copied().unwrap_or(i32::MAX) {
                distances.insert(neighbor, new_cost);
                came_from.insert(neighbor, position);
                
                let heuristic = hex_distance(neighbor, goal) as i32;
                heap.push(State {
                    cost: new_cost + heuristic,
                    position: neighbor,
                });
            }
        }
    }
    
    None // No path found
}

/// Get movement cost for a hex (for pathfinding)
fn get_movement_cost(hex: HexCoord, spatial_container: &SpatialContainer) -> i32 {
    // Check if hex has obstacles or difficult terrain
    let entities = spatial_container.get_entities_at_hex((hex.q, hex.r));
    if !entities.is_empty() {
        // TODO: Check for obstacles, difficult terrain, etc.
        // For now, all hexes have base cost of 1
        1
    } else {
        // Unknown/unloaded hex - higher cost
        2
    }
}

/// Get hex neighbor in a specific direction
fn hex_neighbor(hex: HexCoord, direction: HexDirection) -> HexCoord {
    match direction {
        HexDirection::North => HexCoord::new(hex.q, hex.r + 1),
        HexDirection::NorthEast => HexCoord::new(hex.q + 1, hex.r),
        HexDirection::SouthEast => HexCoord::new(hex.q + 1, hex.r - 1),
        HexDirection::South => HexCoord::new(hex.q, hex.r - 1),
        HexDirection::SouthWest => HexCoord::new(hex.q - 1, hex.r),
        HexDirection::NorthWest => HexCoord::new(hex.q - 1, hex.r + 1),
    }
}

#[derive(Clone, Copy, Debug)]
pub enum HexDirection {
    North,
    NorthEast,
    SouthEast,
    South,
    SouthWest,
    NorthWest,
}
