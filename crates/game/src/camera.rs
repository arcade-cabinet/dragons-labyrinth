use bevy::prelude::*;
use bevy::input::mouse::{MouseWheel, MouseScrollUnit};
use dragons_core::components::Player;

/// Plugin for camera control system
pub struct CameraControllerPlugin;

impl Plugin for CameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CameraSettings>()
            .add_systems(Startup, setup_camera_controller)
            .add_systems(Update, (
                follow_player,
                handle_camera_zoom,
                handle_camera_toggle,
                handle_edge_scrolling,
                smooth_camera_movement,
            ));
    }
}

#[derive(Resource)]
pub struct CameraSettings {
    pub follow_speed: f32,
    pub zoom_speed: f32,
    pub min_zoom: f32,
    pub max_zoom: f32,
    pub current_zoom: f32,
    pub edge_scroll_speed: f32,
    pub edge_threshold: f32,
    pub is_3d_mode: bool,
    pub free_camera: bool,
    pub camera_offset: Vec3,
}

impl Default for CameraSettings {
    fn default() -> Self {
        Self {
            follow_speed: 5.0,
            zoom_speed: 10.0,
            min_zoom: 5.0,
            max_zoom: 50.0,
            current_zoom: 20.0,
            edge_scroll_speed: 20.0,
            edge_threshold: 50.0,
            is_3d_mode: true,
            free_camera: false,
            camera_offset: Vec3::new(0.0, 20.0, 12.0),
        }
    }
}

#[derive(Component)]
pub struct CameraController {
    pub target: Option<Vec3>,
    pub smoothed_position: Vec3,
}

fn setup_camera_controller(
    mut commands: Commands,
    mut camera_query: Query<Entity, With<Camera3d>>,
) {
    if let Ok(camera_entity) = camera_query.get_single_mut() {
        commands.entity(camera_entity).insert(CameraController {
            target: None,
            smoothed_position: Vec3::new(12.0, 20.0, 12.0),
        });
    }
}

/// Follow the player with smooth damping
fn follow_player(
    player_query: Query<&Transform, (With<Player>, Without<Camera3d>)>,
    mut camera_query: Query<&mut CameraController, With<Camera3d>>,
    settings: Res<CameraSettings>,
) {
    if settings.free_camera {
        return;
    }
    
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };
    
    let Ok(mut controller) = camera_query.get_single_mut() else {
        return;
    };
    
    controller.target = Some(player_transform.translation);
}

/// Handle mouse wheel zoom
fn handle_camera_zoom(
    mut scroll_events: EventReader<MouseWheel>,
    mut settings: ResMut<CameraSettings>,
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
) {
    for event in scroll_events.read() {
        let delta = match event.unit {
            MouseScrollUnit::Line => event.y,
            MouseScrollUnit::Pixel => event.y / 100.0,
        };
        
        settings.current_zoom = (settings.current_zoom - delta * settings.zoom_speed)
            .clamp(settings.min_zoom, settings.max_zoom);
        
        // Update camera distance
        if let Ok(mut transform) = camera_query.get_single_mut() {
            let direction = transform.translation.normalize();
            let new_distance = settings.current_zoom;
            
            if settings.is_3d_mode {
                // 3D isometric view
                transform.translation = Vec3::new(
                    new_distance * 0.6,
                    new_distance,
                    new_distance * 0.6,
                );
            } else {
                // 2D top-down view
                transform.translation = Vec3::new(
                    0.0,
                    new_distance,
                    0.1, // Slight offset to avoid gimbal lock
                );
                transform.look_at(Vec3::ZERO, Vec3::Y);
            }
        }
    }
}

/// Toggle between 2D and 3D view with Tab key
fn handle_camera_toggle(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut settings: ResMut<CameraSettings>,
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
) {
    if keyboard.just_pressed(KeyCode::Tab) {
        settings.is_3d_mode = !settings.is_3d_mode;
        
        if let Ok(mut transform) = camera_query.get_single_mut() {
            if settings.is_3d_mode {
                // Switch to 3D isometric view
                transform.translation = Vec3::new(
                    settings.current_zoom * 0.6,
                    settings.current_zoom,
                    settings.current_zoom * 0.6,
                );
                transform.look_at(Vec3::ZERO, Vec3::Y);
                info!("Switched to 3D view");
            } else {
                // Switch to 2D top-down view
                transform.translation = Vec3::new(
                    0.0,
                    settings.current_zoom,
                    0.1,
                );
                transform.rotation = Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2);
                info!("Switched to 2D view");
            }
        }
    }
    
    // Toggle free camera mode with F key
    if keyboard.just_pressed(KeyCode::KeyF) {
        settings.free_camera = !settings.free_camera;
        info!("Free camera mode: {}", settings.free_camera);
    }
}

/// Edge scrolling for free camera mode
fn handle_edge_scrolling(
    windows: Query<&Window>,
    mut camera_query: Query<(&mut Transform, &CameraController), With<Camera3d>>,
    settings: Res<CameraSettings>,
    time: Res<Time>,
) {
    if !settings.free_camera {
        return;
    }
    
    let Ok(window) = windows.get_single() else {
        return;
    };
    
    let Some(cursor_position) = window.cursor_position() else {
        return;
    };
    
    let Ok((mut transform, _)) = camera_query.get_single_mut() else {
        return;
    };
    
    let mut movement = Vec3::ZERO;
    let threshold = settings.edge_threshold;
    let speed = settings.edge_scroll_speed * time.delta_seconds();
    
    // Check edges
    if cursor_position.x < threshold {
        movement.x -= speed;
    } else if cursor_position.x > window.width() - threshold {
        movement.x += speed;
    }
    
    if cursor_position.y < threshold {
        movement.z += speed;
    } else if cursor_position.y > window.height() - threshold {
        movement.z -= speed;
    }
    
    transform.translation += movement;
}

/// Smooth camera movement with interpolation
fn smooth_camera_movement(
    mut camera_query: Query<(&mut Transform, &mut CameraController), With<Camera3d>>,
    settings: Res<CameraSettings>,
    time: Res<Time>,
) {
    let Ok((mut transform, mut controller)) = camera_query.get_single_mut() else {
        return;
    };
    
    if let Some(target) = controller.target {
        // Calculate desired camera position based on target
        let desired_position = if settings.is_3d_mode {
            target + Vec3::new(
                settings.current_zoom * 0.6,
                settings.current_zoom,
                settings.current_zoom * 0.6,
            )
        } else {
            target + Vec3::new(0.0, settings.current_zoom, 0.1)
        };
        
        // Smooth interpolation
        let lerp_factor = settings.follow_speed * time.delta_seconds();
        controller.smoothed_position = controller.smoothed_position.lerp(desired_position, lerp_factor);
        
        transform.translation = controller.smoothed_position;
        
        // Always look at the target
        if !settings.free_camera {
            transform.look_at(target, Vec3::Y);
        }
    }
}

/// Camera shake effect for impacts
#[derive(Component)]
pub struct CameraShake {
    pub intensity: f32,
    pub duration: f32,
    pub elapsed: f32,
}

pub fn apply_camera_shake(
    mut commands: Commands,
    mut camera_query: Query<(Entity, &mut Transform, &mut CameraShake), With<Camera3d>>,
    time: Res<Time>,
) {
    for (entity, mut transform, mut shake) in camera_query.iter_mut() {
        shake.elapsed += time.delta_seconds();
        
        if shake.elapsed >= shake.duration {
            commands.entity(entity).remove::<CameraShake>();
        } else {
            let progress = shake.elapsed / shake.duration;
            let current_intensity = shake.intensity * (1.0 - progress);
            
            let offset = Vec3::new(
                (time.elapsed_seconds() * 50.0).sin() * current_intensity,
                (time.elapsed_seconds() * 47.0).cos() * current_intensity * 0.5,
                (time.elapsed_seconds() * 43.0).sin() * current_intensity * 0.7,
            );
            
            transform.translation += offset;
        }
    }
}

/// Trigger camera shake
pub fn trigger_camera_shake(
    mut commands: Commands,
    camera_query: Query<Entity, With<Camera3d>>,
    intensity: f32,
    duration: f32,
) {
    if let Ok(camera_entity) = camera_query.get_single() {
        commands.entity(camera_entity).insert(CameraShake {
            intensity,
            duration,
            elapsed: 0.0,
        });
    }
}