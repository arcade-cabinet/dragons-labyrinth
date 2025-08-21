use bevy::prelude::*;
use dragons_core::components::*;
use hexx::{Hex, HexLayout, HexOrientation};
use noise::{NoiseFn, Perlin};
use rand::Rng;

/// Preset map configurations for Dragon's Labyrinth
pub struct MapPresets;

impl MapPresets {
    /// Create the starting village map (20x20 hexes)
    pub fn create_starting_village(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        hex_world: &mut ResMut<HexWorld>,
    ) {
        let map_radius = 10; // Creates roughly 20x20 hex map
        let perlin = Perlin::new(42); // Seed for consistent generation
        
        // Clear existing tiles
        hex_world.tiles.clear();
        
        // Village center
        let village_center = Hex::ZERO;
        let village_radius = 3;
        
        // Forest boundaries
        let forest_start_radius = 5;
        
        // Generate hex tiles
        for q in -map_radius..=map_radius {
            for r in -map_radius..=map_radius {
                let s = -q - r;
                if s.abs() > map_radius {
                    continue;
                }
                
                let hex = Hex::new(q, r);
                let distance_from_center = hex.unsigned_distance_to(village_center) as f32;
                
                // Determine tile type based on location
                let tile_type = if distance_from_center <= village_radius as f32 {
                    // Village area - safe zone
                    TileType::Stone // Cobblestone paths
                } else if distance_from_center <= forest_start_radius as f32 {
                    // Meadow around village
                    TileType::Grass
                } else {
                    // Forest area - tutorial combat zone
                    if rand::thread_rng().gen_bool(0.7) {
                        TileType::Forest
                    } else {
                        TileType::Grass
                    }
                };
                
                // Use Perlin noise for elevation variation
                let noise_scale = 0.1;
                let elevation = perlin.get([
                    q as f64 * noise_scale,
                    r as f64 * noise_scale,
                ]) as f32 * 0.5;
                
                // Check for corruption (none in starting area)
                let corruption = if distance_from_center > 8.0 {
                    // Slight corruption at edges
                    (distance_from_center - 8.0) * 0.05
                } else {
                    0.0
                };
                
                let tile = HexTile {
                    hex,
                    tile_type,
                    dread_level: 0, // Peace stage
                    corruption: corruption.min(0.3),
                    elevation,
                    passable: tile_type != TileType::Forest || rand::thread_rng().gen_bool(0.3), // Some forest tiles are passable
                };
                
                // Spawn tile entity
                spawn_hex_tile(commands, meshes, materials, &tile);
                
                // Add to world
                hex_world.tiles.insert(hex, tile);
            }
        }
        
        // Add special locations
        add_village_buildings(commands, meshes, materials);
        add_forest_features(commands, meshes, materials, hex_world);
        spawn_village_npcs(commands, meshes, materials);
        spawn_forest_enemies(commands, meshes, materials);
        
        info!("Created starting village map with {} tiles", hex_world.tiles.len());
    }
    
    /// Create a corrupted grove for the first boss
    pub fn create_corrupted_grove(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        hex_world: &mut ResMut<HexWorld>,
    ) {
        let grove_center = Hex::new(15, 0);
        let grove_radius = 5;
        
        // Create corrupted area
        for q in -grove_radius..=grove_radius {
            for r in -grove_radius..=grove_radius {
                let hex = grove_center + Hex::new(q, r);
                
                if let Some(tile) = hex_world.tiles.get_mut(&hex) {
                    let distance = hex.unsigned_distance_to(grove_center) as f32;
                    let corruption_intensity = 1.0 - (distance / grove_radius as f32);
                    
                    tile.corruption = corruption_intensity;
                    tile.dread_level = 1; // Unease stage
                    
                    if corruption_intensity > 0.5 {
                        tile.tile_type = TileType::Corrupted;
                    } else if corruption_intensity > 0.3 {
                        tile.tile_type = TileType::Swamp;
                    }
                }
            }
        }
        
        // Spawn boss
        spawn_hollow_caretaker_boss(commands, meshes, materials, grove_center);
    }
}

/// Spawn a single hex tile
fn spawn_hex_tile(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    tile: &HexTile,
) {
    let layout = HexLayout {
        orientation: HexOrientation::Flat,
        origin: Vec2::ZERO,
        hex_size: Vec2::splat(1.0),
        invert_x: false,
        invert_y: false,
    };
    
    let world_pos = layout.hex_to_world_pos(tile.hex);
    
    // Create hex mesh
    let hex_mesh = create_hex_mesh();
    
    // Determine color based on tile type and corruption
    let base_color = get_tile_color(tile);
    
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(hex_mesh),
            material: materials.add(StandardMaterial {
                base_color,
                perceptual_roughness: match tile.tile_type {
                    TileType::Stone => 0.8,
                    TileType::Grass => 0.9,
                    TileType::Forest => 0.95,
                    TileType::Swamp => 0.6,
                    TileType::Corrupted => 0.4,
                },
                ..default()
            }),
            transform: Transform::from_xyz(world_pos.x, tile.elevation, world_pos.y),
            ..default()
        },
        tile.clone(),
    ));
}

/// Create hexagonal mesh
fn create_hex_mesh() -> Mesh {
    let mut mesh = Mesh::new(
        bevy::render::render_resource::PrimitiveTopology::TriangleList,
        bevy::render::render_asset::RenderAssetUsages::default()
    );
    
    let angle_step = std::f32::consts::PI / 3.0;
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    let mut indices = Vec::new();
    
    // Center vertex
    positions.push([0.0, 0.0, 0.0]);
    normals.push([0.0, 1.0, 0.0]);
    uvs.push([0.5, 0.5]);
    
    // Outer vertices
    for i in 0..6 {
        let angle = i as f32 * angle_step;
        let x = angle.cos();
        let z = angle.sin();
        positions.push([x, 0.0, z]);
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

/// Get color for tile based on type and corruption
fn get_tile_color(tile: &HexTile) -> Color {
    let base_color = match tile.tile_type {
        TileType::Grass => Color::srgb(0.3, 0.7, 0.3),
        TileType::Forest => Color::srgb(0.1, 0.4, 0.1),
        TileType::Swamp => Color::srgb(0.2, 0.3, 0.1),
        TileType::Stone => Color::srgb(0.6, 0.6, 0.6),
        TileType::Corrupted => Color::srgb(0.4, 0.1, 0.1),
    };
    
    if tile.corruption > 0.0 {
        let corruption_color = Color::srgb(0.3, 0.1, 0.1);
        base_color.mix(&corruption_color, tile.corruption)
    } else {
        base_color
    }
}

/// Add village buildings
fn add_village_buildings(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    // Town hall at center
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(2.0, 3.0, 2.0)),
        material: materials.add(StandardMaterial {
            base_color: Color::srgb(0.7, 0.5, 0.3),
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 1.5, 0.0),
        ..default()
    });
    
    // Blacksmith
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(1.5, 2.0, 1.5)),
        material: materials.add(StandardMaterial {
            base_color: Color::srgb(0.4, 0.4, 0.4),
            ..default()
        }),
        transform: Transform::from_xyz(3.0, 1.0, 0.0),
        ..default()
    });
    
    // Inn
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(2.5, 2.5, 2.0)),
        material: materials.add(StandardMaterial {
            base_color: Color::srgb(0.6, 0.4, 0.2),
            ..default()
        }),
        transform: Transform::from_xyz(-3.0, 1.25, 0.0),
        ..default()
    });
}

/// Add forest features
fn add_forest_features(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    hex_world: &HexWorld,
) {
    let mut rng = rand::thread_rng();
    
    // Add trees to forest tiles
    for (hex, tile) in hex_world.tiles.iter() {
        if tile.tile_type == TileType::Forest && rng.gen_bool(0.3) {
            let layout = HexLayout {
                orientation: HexOrientation::Flat,
                origin: Vec2::ZERO,
                hex_size: Vec2::splat(1.0),
                invert_x: false,
                invert_y: false,
            };
            let world_pos = layout.hex_to_world_pos(*hex);
            
            // Simple tree (cylinder trunk + sphere leaves)
            let tree_entity = commands.spawn_empty().id();
            
            // Trunk
            commands.entity(tree_entity).with_children(|parent| {
                parent.spawn(PbrBundle {
                    mesh: meshes.add(Cylinder::new(0.2, 2.0)),
                    material: materials.add(StandardMaterial {
                        base_color: Color::srgb(0.4, 0.2, 0.1),
                        ..default()
                    }),
                    transform: Transform::from_xyz(world_pos.x, 1.0, world_pos.y),
                    ..default()
                });
                
                // Leaves
                parent.spawn(PbrBundle {
                    mesh: meshes.add(Sphere::new(1.0)),
                    material: materials.add(StandardMaterial {
                        base_color: Color::srgb(0.1, 0.5, 0.1),
                        ..default()
                    }),
                    transform: Transform::from_xyz(world_pos.x, 2.5, world_pos.y),
                    ..default()
                });
            });
        }
    }
}

/// Spawn village NPCs
fn spawn_village_npcs(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    use crate::combat::Health;
    
    // Spawn friendly NPCs
    let npc_positions = vec![
        (Hex::new(1, 0), "Villager", Color::srgb(0.7, 0.5, 0.3)),
        (Hex::new(-1, 0), "Merchant", Color::srgb(0.5, 0.3, 0.7)),
        (Hex::new(0, 1), "Guard", Color::srgb(0.3, 0.3, 0.5)),
    ];
    
    for (hex, name, color) in npc_positions {
        let layout = HexLayout {
            orientation: HexOrientation::Flat,
            origin: Vec2::ZERO,
            hex_size: Vec2::splat(1.0),
            invert_x: false,
            invert_y: false,
        };
        let world_pos = layout.hex_to_world_pos(hex);
        
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Capsule3d::new(0.25, 0.8)),
                material: materials.add(StandardMaterial {
                    base_color: color,
                    ..default()
                }),
                transform: Transform::from_xyz(world_pos.x, 0.5, world_pos.y),
                ..default()
            },
            Name::new(name),
            HexPosition(hex),
            Health { current: 50.0, max: 50.0 },
        ));
    }
}

/// Spawn forest enemies
fn spawn_forest_enemies(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    use crate::combat::{Health, CombatParticipant};
    use dragons_ai::behaviors::{AIAgent, AIBehavior};
    
    // Spawn some wolves in the forest
    let enemy_positions = vec![
        Hex::new(7, 0),
        Hex::new(8, -2),
        Hex::new(6, 3),
    ];
    
    for hex in enemy_positions {
        let layout = HexLayout {
            orientation: HexOrientation::Flat,
            origin: Vec2::ZERO,
            hex_size: Vec2::splat(1.0),
            invert_x: false,
            invert_y: false,
        };
        let world_pos = layout.hex_to_world_pos(hex);
        
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Capsule3d::new(0.3, 0.6)),
                material: materials.add(StandardMaterial {
                    base_color: Color::srgb(0.3, 0.3, 0.3),
                    ..default()
                }),
                transform: Transform::from_xyz(world_pos.x, 0.3, world_pos.y),
                ..default()
            },
            Name::new("Wolf"),
            HexPosition(hex),
            Health { current: 30.0, max: 30.0 },
            CombatParticipant {
                initiative: 2.0,
                attack_range: 1.0,
                damage_dice: (1, 6),
                armor_class: 12,
                is_enemy: true,
            },
            AIAgent {
                behavior: AIBehavior::Patrol {
                    waypoints: vec![hex, hex + Hex::new(2, 0), hex + Hex::new(0, 2)],
                    current_index: 0,
                },
                detection_range: 4.0,
                attack_range: 1.0,
                flee_threshold: 0.25,
                speed: 4.0,
            },
        ));
    }
}

/// Spawn the Hollow Caretaker boss
fn spawn_hollow_caretaker_boss(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Hex,
) {
    use crate::combat::{Health, CombatParticipant};
    use dragons_ai::behaviors::{AIAgent, AIBehavior};
    
    let layout = HexLayout {
        orientation: HexOrientation::Flat,
        origin: Vec2::ZERO,
        hex_size: Vec2::splat(1.0),
        invert_x: false,
        invert_y: false,
    };
    let world_pos = layout.hex_to_world_pos(position);
    
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Capsule3d::new(0.5, 1.5)),
            material: materials.add(StandardMaterial {
                base_color: Color::srgb(0.1, 0.1, 0.2),
                emissive: Color::srgb(0.0, 0.1, 0.0) * 10.0,
                ..default()
            }),
            transform: Transform::from_xyz(world_pos.x, 0.75, world_pos.y)
                .with_scale(Vec3::splat(1.5)),
            ..default()
        },
        Name::new("Hollow Caretaker"),
        HexPosition(position),
        Health { current: 100.0, max: 100.0 },
        CombatParticipant {
            initiative: 5.0,
            attack_range: 2.0,
            damage_dice: (2, 8),
            armor_class: 15,
            is_enemy: true,
        },
        AIAgent {
            behavior: AIBehavior::Idle,
            detection_range: 6.0,
            attack_range: 2.0,
            flee_threshold: 0.0, // Boss doesn't flee
            speed: 3.0,
        },
    ));
}