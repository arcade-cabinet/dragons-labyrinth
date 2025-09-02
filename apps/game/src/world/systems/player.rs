use bevy::prelude::*;
use crate::world::components::{Player, Mount};
use crate::world::resources::{WorldState, GameState};
use crate::utils::hex::*;

pub fn player_movement_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<(&mut Transform, &Player, Option<&Mount>)>,
    mut world_state: ResMut<WorldState>,
) {
    if let Ok((mut transform, player, mount)) = player_query.get_single_mut() {
        let mut movement = Vec3::ZERO;
        let base_speed = 5.0;
        let mount_multiplier = mount.map_or(1.0, |m| m.speed_multiplier);
        let movement_speed = base_speed * mount_multiplier;
        
        // Handle hex-based movement input
        if keyboard.just_pressed(KeyCode::KeyW) || keyboard.just_pressed(KeyCode::ArrowUp) {
            movement += get_hex_direction(HexDirection::North);
        }
        if keyboard.just_pressed(KeyCode::KeyS) || keyboard.just_pressed(KeyCode::ArrowDown) {
            movement += get_hex_direction(HexDirection::South);
        }
        if keyboard.just_pressed(KeyCode::KeyA) || keyboard.just_pressed(KeyCode::ArrowLeft) {
            movement += get_hex_direction(HexDirection::NorthWest);
        }
        if keyboard.just_pressed(KeyCode::KeyD) || keyboard.just_pressed(KeyCode::ArrowRight) {
            movement += get_hex_direction(HexDirection::SouthEast);
        }
        if keyboard.just_pressed(KeyCode::KeyQ) {
            movement += get_hex_direction(HexDirection::NorthEast);
        }
        if keyboard.just_pressed(KeyCode::KeyE) {
            movement += get_hex_direction(HexDirection::SouthWest);
        }
        
        if movement.length() > 0.0 {
            // Normalize hex movement and apply to transform
            let target_hex = world_to_hex(transform.translation + movement.normalize());
            let target_world = hex_to_world(target_hex);
            
            // Smooth movement to target position
            let direction = (target_world - transform.translation).normalize();
            transform.translation += direction * movement_speed * time.delta_seconds();
            
            // Snap to hex center when close enough
            if transform.translation.distance(target_world) < 0.1 {
                transform.translation = target_world;
                world_state.player_hex = Some(target_hex);
            }
        }
        
        // Handle mount interactions
        if keyboard.just_pressed(KeyCode::Space) {
            // Try to mount/dismount or interact with features
            handle_player_interaction(&mut world_state, transform.translation);
        }
    }
}

fn handle_player_interaction(world_state: &mut ResMut<WorldState>, player_position: Vec3) {
    let player_hex = world_to_hex(player_position);
    
    // Check for nearby features or mounts
    info!("Player interacting at hex: {:?}", player_hex);
    
    // TODO: Implement feature interaction system
    // TODO: Implement mount system
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

fn get_hex_direction(direction: HexDirection) -> Vec3 {
    let hex_size = 64.0;
    match direction {
        HexDirection::North => Vec3::new(0.0, 0.0, hex_size),
        HexDirection::NorthEast => Vec3::new(hex_size * 0.866, 0.0, hex_size * 0.5),
        HexDirection::SouthEast => Vec3::new(hex_size * 0.866, 0.0, -hex_size * 0.5),
        HexDirection::South => Vec3::new(0.0, 0.0, -hex_size),
        HexDirection::SouthWest => Vec3::new(-hex_size * 0.866, 0.0, -hex_size * 0.5),
        HexDirection::NorthWest => Vec3::new(-hex_size * 0.866, 0.0, hex_size * 0.5),
    }
}

pub fn spawn_player(mut commands: Commands, world_state: Res<WorldState>) {
    commands.spawn((
        Player {
            health: 100.0,
            max_health: 100.0,
            sanity: 100.0,
            max_sanity: 100.0,
            inventory: Vec::new(),
            mount: None,
        },
        Transform::from_translation(Vec3::new(0.0, 1.0, 0.0)),
        GlobalTransform::default(),
        Visibility::default(),
        Name::new("Player"),
    ));
}
