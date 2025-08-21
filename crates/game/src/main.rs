use bevy::prelude::*;
use hexx::*;
use dragons_core::{components::*, resources::*};
use dragons_assets::AssetsPlugin;

mod systems;
mod generators;
mod map_generation;
mod database;
mod cutscenes;

use systems::*;
use generators::*;
use map_generation::MapGenerationPlugin;
use database::DatabasePlugin;
use cutscenes::{CutscenePlugin, GameState};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Dragon's Labyrinth - Horror RPG".to_string(),
                    resolution: (1280.0, 720.0).into(),
                    ..default()
                }),
                ..default()
            }),
            AssetsPlugin,
            MapGenerationPlugin,
            DatabasePlugin {
                game_database_path: std::path::PathBuf::from("game.db"),
                app_name: "dragons_labyrinth".to_string(),
            },
            CutscenePlugin,
        ))
        .init_state::<GameState>()
        // Core resources for Dragon's Labyrinth
        .init_resource::<DreadState>()
        .init_resource::<HexWorld>()
        .init_resource::<NarrativeState>()
        .init_resource::<CompanionState>()
        .init_resource::<PlayerState>()
        // Startup systems
        .add_systems(Startup, (
            setup_camera,
            setup_lighting,
            setup_hex_world,
            spawn_player,
            spawn_companions,
            generate_initial_content,
        ))
        // Core game systems
        .add_systems(Update, (
            handle_hex_movement,
            update_dread_progression,
            apply_world_corruption,
            update_companion_trauma,
            process_narrative_events,
            update_lighting_based_on_dread,
        ))
        .run();
}

fn setup_camera(mut commands: Commands) {
    // Isometric camera for 2.5D hex world view
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(12.0, 20.0, 12.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn setup_lighting(mut commands: Commands) {
    // Directional light that gets corrupted as dread increases
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            illuminance: 10000.0,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_euler(
            EulerRot::XYZ,
            -std::f32::consts::PI / 4.0,
            std::f32::consts::PI / 4.0,
            0.0,
        )),
        ..default()
    });
    
    // Ambient light that dims with horror progression
    commands.insert_resource(AmbientLight {
        color: Color::srgb(0.8, 0.8, 0.8),
        brightness: 500.0,
    });
}

fn setup_hex_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut hex_world: ResMut<HexWorld>,
) {
    // Generate initial hex world following design bible
    let world_radius = 15;
    
    for q in -world_radius..=world_radius {
        for r in -world_radius..=world_radius {
            let s = -q - r;
            if s.abs() > world_radius {
                continue;
            }
            
            let hex = Hex::new(q, r);
            let tile = HexTile {
                hex,
                tile_type: TileType::Grass, // Start in Peace stage
                dread_level: 0,
                corruption: 0.0,
                elevation: 0.0,
                passable: true,
            };
            
            // Convert hex coordinates to world position using hexx
            let layout = HexLayout {
                orientation: HexOrientation::Flat,
                origin: Vec2::ZERO,
                hex_size: Vec2::splat(1.0),
                invert_x: false,
                invert_y: false,
            };
            let world_pos = layout.hex_to_world_pos(hex);
            
            // Create hex mesh
            let hex_mesh = create_hex_mesh();
            
            // Spawn hex tile entity with modern Bevy 0.16 API
            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(hex_mesh),
                    material: materials.add(StandardMaterial {
                        base_color: get_tile_color(&tile),
                        ..default()
                    }),
                    transform: Transform::from_xyz(world_pos.x, tile.elevation, world_pos.y),
                    ..default()
                },
                tile,
            ));
            
            hex_world.tiles.insert(hex, tile);
        }
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Capsule3d::new(0.3, 1.0)),
            material: materials.add(StandardMaterial {
                base_color: Color::srgb(0.2, 0.4, 0.8),
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        Player,
        HexPosition(Hex::ZERO),
    ));
}

fn spawn_companions(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn Einar - Loyal friend
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Capsule3d::new(0.3, 1.0)),
            material: materials.add(StandardMaterial {
                base_color: Color::srgb(0.6, 0.4, 0.2), // Brown
                ..default()
            }),
            transform: Transform::from_xyz(2.0, 0.5, 0.0),
            ..default()
        },
        Companion {
            name: "Einar".to_string(),
            companion_type: CompanionType::Einar,
            sanity: 100.0,
            loyalty: 100.0,
            trauma_level: 0.0,
        },
        HexPosition(Hex::new(1, 0)),
    ));
    
    // Spawn Mira - Optimist
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Capsule3d::new(0.3, 1.0)),
            material: materials.add(StandardMaterial {
                base_color: Color::srgb(0.9, 0.7, 0.2), // Gold
                ..default()
            }),
            transform: Transform::from_xyz(-2.0, 0.5, 0.0),
            ..default()
        },
        Companion {
            name: "Mira".to_string(),
            companion_type: CompanionType::Mira,
            sanity: 100.0,
            loyalty: 80.0,
            trauma_level: 0.0,
        },
        HexPosition(Hex::new(-1, 0)),
    ));
}

fn generate_initial_content(
    dread_state: Res<DreadState>,
) {
    // Initialize the AI content generator
    let concept = GameConcept {
        name: "Dragon's Labyrinth".to_string(),
        theme: "Psychological Horror RPG".to_string(),
        setting: "Ancient cursed labyrinth".to_string(),
        mood: "Escalating dread and terror".to_string(),
        visual_reference: vec!["Dark Souls".to_string(), "Silent Hill".to_string()],
        dread_progression: DreadProgression {
            stages: vec![
                DreadStage {
                    level: 0,
                    name: "Peace".to_string(),
                    description: "Bright world, mundane quests".to_string(),
                    color_shift: ColorShift {
                        saturation_modifier: 1.0,
                        brightness_modifier: 1.0,
                        hue_shift: 0.0,
                        corruption_overlay: None,
                    },
                    audio_profile: AudioProfile {
                        base_frequency: 220.0,
                        distortion_level: 0.0,
                        reverb_decay: 0.5,
                        ambient_volume: 0.3,
                    },
                    behavior_changes: vec!["Normal NPC behavior".to_string()],
                },
                // Add more stages...
            ],
        },
    };
    
    info!("Game concept initialized: {}", concept.name);
}

// Create a hexagonal mesh using modern Bevy mesh API
fn create_hex_mesh() -> Mesh {
    let mut mesh = Mesh::new(bevy::render::render_resource::PrimitiveTopology::TriangleList, bevy::render::render_asset::RenderAssetUsages::default());
    
    // Hex vertices (flat-topped hexagon)
    let angle_step = std::f32::consts::PI / 3.0;
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    let mut indices = Vec::new();
    
    // Center vertex
    positions.push([0.0, 0.1, 0.0]);
    normals.push([0.0, 1.0, 0.0]);
    uvs.push([0.5, 0.5]);
    
    // Outer vertices
    for i in 0..6 {
        let angle = i as f32 * angle_step;
        let x = angle.cos();
        let z = angle.sin();
        positions.push([x, 0.1, z]);
        normals.push([0.0, 1.0, 0.0]);
        uvs.push([(x + 1.0) * 0.5, (z + 1.0) * 0.5]);
    }
    
    // Create triangles
    for i in 0..6 {
        indices.push(0);
        indices.push(i + 1);
        indices.push(if i == 5 { 1 } else { i + 2 });
    }
    
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(bevy::render::mesh::Indices::U32(indices));
    
    mesh
}

fn get_tile_color(tile: &HexTile) -> Color {
    let base_color = match tile.tile_type {
        TileType::Grass => Color::srgb(0.3, 0.8, 0.3),
        TileType::Forest => Color::srgb(0.1, 0.5, 0.1),
        TileType::Swamp => Color::srgb(0.2, 0.4, 0.1),
        TileType::Stone => Color::srgb(0.5, 0.5, 0.5),
        TileType::Corrupted => Color::srgb(0.5, 0.1, 0.1),
    };
    
    // Apply corruption overlay
    if tile.corruption > 0.0 {
        let corruption_color = Color::srgb(0.3, 0.1, 0.1);
        base_color.mix(&corruption_color, tile.corruption)
    } else {
        base_color
    }
}