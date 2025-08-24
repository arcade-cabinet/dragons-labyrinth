use bevy::prelude::*;
use hexx::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Board rendering system using beauty textures and control maps
#[derive(Component, Clone, Debug)]
pub struct Board {
    pub hex_layout: HexLayout,
    pub size: Hex,
    pub beauty_texture: Handle<Image>,
    pub splatmap: Handle<Image>,
    pub overlay_mask: Handle<Image>,
    pub id_map: Handle<Image>,
    pub interactive_objects: Vec<InteractiveObject>,
    pub dread_level: u8,
}

// Splatmap materials for seamless blending
#[derive(Resource)]
pub struct BoardMaterials {
    pub grass: Handle<Image>,
    pub dirt: Handle<Image>, 
    pub sand: Handle<Image>,
    pub rock: Handle<Image>,
    pub water: Handle<Image>,
    pub lava: Handle<Image>,
    pub corrupted: Handle<Image>,  // Added for horror progression
}

// Interactive objects marked in ID map
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InteractiveObject {
    pub id: String,
    pub position: Hex,
    pub object_type: ObjectType,
    pub tags: Vec<String>,
    pub dread_response: Option<DreadResponse>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ObjectType {
    Door,
    Well,
    Altar,
    TestEntry(TestType),
    LabyrinthEntrance,
    VoidPortal,
    CompanionSpawn(String),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TestType {
    Strength,
    Harmony, 
    Morality,
}

// How objects change based on dread progression
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DreadResponse {
    pub peace_state: ObjectState,
    pub unease_state: ObjectState,
    pub dread_state: ObjectState,
    pub terror_state: ObjectState,
    pub horror_state: ObjectState,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ObjectState {
    pub visible: bool,
    pub interactable: bool,
    pub texture_variant: Option<String>,
    pub audio_cue: Option<String>,
}

// Navigation grid built from splatmap and overlays
#[derive(Resource)]
pub struct NavigationGrid {
    pub cost_map: HashMap<Hex, f32>,
    pub walkable_map: HashMap<Hex, bool>,
    pub hex_layout: HexLayout,
    pub mount_auras: Vec<MountAura>,
}

#[derive(Clone, Debug)]
pub struct MountAura {
    pub center: Hex,
    pub radius: u32,
    pub mount_type: MountType,
    pub alignment_bonus: f32,  // Based on player alignment
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MountType {
    Seastrider,   // Water traversal
    RockCrusher,  // Mountain/lava traversal
    VoidWalker,   // Horror stage traversal
}

impl NavigationGrid {
    pub fn new(hex_layout: HexLayout) -> Self {
        Self {
            cost_map: HashMap::new(),
            walkable_map: HashMap::new(),
            hex_layout,
            mount_auras: Vec::new(),
        }
    }
    
    // Build navigation from splatmap data
    pub fn build_from_splatmap(&mut self, splatmap_data: &[u8], size: Hex) {
        for q in -size.x..=size.x {
            let r1 = (-size.y).max(-q - size.y);
            let r2 = size.y.min(-q + size.y);
            
            for r in r1..=r2 {
                let hex = Hex::new(q, r);
                let pixel_pos = self.hex_to_pixel_coords(hex, size);
                
                if let Some(rgba) = self.sample_splatmap(splatmap_data, pixel_pos, size) {
                    let cost = self.calculate_terrain_cost(rgba);
                    let walkable = cost < f32::INFINITY;
                    
                    self.cost_map.insert(hex, cost);
                    self.walkable_map.insert(hex, walkable);
                }
            }
        }
    }
    
    // Apply mount aura effects to navigation
    pub fn apply_mount_auras(&mut self) {
        for aura in &self.mount_auras {
            for hex in aura.center.ring(aura.radius) {
                if let Some(base_cost) = self.cost_map.get(&hex).cloned() {
                    let modified_cost = match aura.mount_type {
                        MountType::Seastrider => {
                            // Reduce water costs, enable water walking
                            if base_cost > 10.0 { 1.0 + aura.alignment_bonus } else { base_cost }
                        },
                        MountType::RockCrusher => {
                            // Reduce mountain/lava costs
                            if base_cost > 5.0 { 2.0 + aura.alignment_bonus } else { base_cost }
                        },
                        MountType::VoidWalker => {
                            // Enable traversal of void tiles
                            if base_cost == f32::INFINITY { 3.0 + aura.alignment_bonus } else { base_cost }
                        },
                    };
                    
                    self.cost_map.insert(hex, modified_cost);
                    self.walkable_map.insert(hex, modified_cost < f32::INFINITY);
                }
            }
        }
    }
    
    // A* pathfinding using Hexx algorithms
    pub fn find_path(&self, start: Hex, goal: Hex) -> Option<Vec<Hex>> {
        use hexx::algorithms::a_star;
        
        let successors = |hex: Hex| {
            hex.all_neighbors()
                .into_iter()
                .filter_map(|neighbor| {
                    if let Some(&walkable) = self.walkable_map.get(&neighbor) {
                        if walkable {
                            let cost = self.cost_map.get(&neighbor).copied().unwrap_or(1.0);
                            Some((neighbor, cost))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        };
        
        let heuristic = |hex: Hex| hex.unsigned_distance_to(goal) as f32;
        let success = |hex: Hex| hex == goal;
        
        a_star(&start, successors, heuristic, success)
            .map(|(path, _cost)| path)
    }
    
    // Field of view calculation using Hexx
    pub fn calculate_fov(&self, origin: Hex, radius: u32) -> Vec<Hex> {
        use hexx::algorithms::field_of_view;
        
        let is_blocking = |hex: Hex| {
            !self.walkable_map.get(&hex).copied().unwrap_or(false)
        };
        
        field_of_view(origin, radius, is_blocking)
    }
    
    fn hex_to_pixel_coords(&self, hex: Hex, board_size: Hex) -> (u32, u32) {
        let world_pos = self.hex_layout.hex_to_world_pos(hex);
        // Convert world position to texture coordinates
        let texture_size = 1024; // Assuming 1024x1024 textures
        let x = ((world_pos.x + board_size.x as f32) / (board_size.x as f32 * 2.0) * texture_size as f32) as u32;
        let y = ((world_pos.y + board_size.y as f32) / (board_size.y as f32 * 2.0) * texture_size as f32) as u32;
        (x.min(texture_size - 1), y.min(texture_size - 1))
    }
    
    fn sample_splatmap(&self, data: &[u8], pos: (u32, u32), _size: Hex) -> Option<[u8; 4]> {
        let texture_size = 1024;
        let index = ((pos.1 * texture_size + pos.0) * 4) as usize;
        
        if index + 3 < data.len() {
            Some([data[index], data[index + 1], data[index + 2], data[index + 3]])
        } else {
            None
        }
    }
    
    fn calculate_terrain_cost(&self, rgba: [u8; 4]) -> f32 {
        // RGBA channels represent material weights
        let grass_weight = rgba[0] as f32 / 255.0;
        let dirt_weight = rgba[1] as f32 / 255.0;
        let rock_weight = rgba[2] as f32 / 255.0;
        let water_weight = rgba[3] as f32 / 255.0;
        
        // Calculate blended movement cost
        let grass_cost = 1.0;
        let dirt_cost = 1.2;
        let rock_cost = 2.0;
        let water_cost = 10.0; // High cost, requires mount to traverse
        
        grass_cost * grass_weight +
        dirt_cost * dirt_weight +
        rock_cost * rock_weight +
        water_cost * water_weight
    }
}

// Board rendering system
pub fn render_board_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    board_query: Query<&Board, Added<Board>>,
    board_materials: Res<BoardMaterials>,
) {
    for board in board_query.iter() {
        // Create board plane mesh
        let board_mesh = meshes.add(Plane3d::default().mesh().size(20.0, 20.0));
        
        // Create material with splatmap shader
        let board_material = materials.add(StandardMaterial {
            base_color_texture: Some(board.beauty_texture.clone()),
            // Additional textures for splatmap blending would be added here
            // This requires custom shader material in full implementation
            ..default()
        });
        
        // Spawn board entity
        commands.spawn(PbrBundle {
            mesh: board_mesh,
            material: board_material,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        });
        
        // Spawn interactive objects
        for obj in &board.interactive_objects {
            spawn_interactive_object(&mut commands, &mut meshes, &mut materials, obj, board);
        }
    }
}

fn spawn_interactive_object(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    obj: &InteractiveObject,
    board: &Board,
) {
    let world_pos = board.hex_layout.hex_to_world_pos(obj.position);
    
    // Create visual marker based on object type
    let (mesh, material) = match &obj.object_type {
        ObjectType::Door => (
            meshes.add(Cuboid::new(0.5, 2.0, 0.1)),
            materials.add(StandardMaterial {
                base_color: Color::srgb(0.4, 0.2, 0.1),
                ..default()
            })
        ),
        ObjectType::Well => (
            meshes.add(Cylinder::new(0.8, 0.5)),
            materials.add(StandardMaterial {
                base_color: Color::srgb(0.5, 0.5, 0.5),
                ..default()
            })
        ),
        ObjectType::TestEntry(test_type) => {
            let color = match test_type {
                TestType::Strength => Color::srgb(0.8, 0.2, 0.2),
                TestType::Harmony => Color::srgb(0.2, 0.8, 0.2),
                TestType::Morality => Color::srgb(0.2, 0.2, 0.8),
            };
            (
                meshes.add(Cylinder::new(1.0, 0.2)),
                materials.add(StandardMaterial {
                    base_color: color,
                    emissive: color * 0.3,
                    ..default()
                })
            )
        },
        ObjectType::LabyrinthEntrance => (
            meshes.add(Cuboid::new(2.0, 3.0, 0.5)),
            materials.add(StandardMaterial {
                base_color: Color::srgb(0.1, 0.1, 0.1),
                emissive: Color::srgb(0.3, 0.0, 0.3),
                ..default()
            })
        ),
        _ => (
            meshes.add(Sphere::new(0.5)),
            materials.add(StandardMaterial::default())
        ),
    };
    
    commands.spawn((
        PbrBundle {
            mesh,
            material,
            transform: Transform::from_xyz(world_pos.x, 0.5, world_pos.y),
            ..default()
        },
        obj.clone(),
    ));
}

// Plugin for board system
pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<NavigationGrid>()
            .add_systems(Update, render_board_system);
    }
}