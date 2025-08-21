use bevy::prelude::*;

mod components;
mod systems;
mod resources;

use components::*;
use systems::*;
use resources::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Dragon's Labyrinth".into(),
                resolution: (1024.0, 768.0).into(),
                canvas: Some("#bevy".to_owned()),
                ..default()
            }),
            ..default()
        }))
        // Core resources following design bible
        .init_resource::<DreadState>()
        .init_resource::<HexWorld>()
        .init_resource::<NarrativeState>()
        // Core systems following horror-first design
        .add_systems(Startup, (
            setup_camera,
            setup_world,
            setup_lighting,
        ))
        .add_systems(Update, (
            dread_progression_system,
            hex_interaction_system,
            companion_trauma_system,
            world_corruption_system,
            camera_follow_system,
        ))
        .run();
}

fn setup_camera(mut commands: Commands) {
    // Isometric camera for 2.5D hex world
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(10.0, 15.0, 10.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn setup_lighting(mut commands: Commands) {
    // Directional light that can be corrupted based on dread level
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    
    // Ambient light that diminishes with horror progression
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.8,
    });
}

fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut hex_world: ResMut<HexWorld>,
) {
    // Generate initial hex world following design bible
    let world_radius = 20;
    
    for q in -world_radius..=world_radius {
        let r1 = (-world_radius).max(-q - world_radius);
        let r2 = world_radius.min(-q + world_radius);
        
        for r in r1..=r2 {
            let tile = HexTile {
                q,
                r,
                tile_type: TileType::Grass, // Start peaceful
                dread_level: 0,
                biome_features: vec![],
                elevation: 0.0,
            };
            
            // Convert hex coordinates to world position
            let world_pos = hex_to_world(q, r, 0.0);
            
            // Spawn hex tile entity
            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(Cylinder::new(1.0, 0.2).mesh().into()),
                    material: materials.add(StandardMaterial {
                        base_color: Color::srgb(0.3, 0.8, 0.3), // Green for grass
                        ..default()
                    }),
                    transform: Transform::from_translation(world_pos),
                    ..default()
                },
                tile,
            ));
            
            hex_world.tiles.insert((q, r), tile);
        }
    }
}

// Convert hex coordinates to 3D world position
fn hex_to_world(q: i32, r: i32, elevation: f32) -> Vec3 {
    let size = 1.5;
    let x = size * (3.0_f32.sqrt() * q as f32 + 3.0_f32.sqrt() / 2.0 * r as f32);
    let z = size * (3.0 / 2.0 * r as f32);
    Vec3::new(x, elevation, z)
}