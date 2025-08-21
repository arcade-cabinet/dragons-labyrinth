use bevy::prelude::*;
use hexx::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Hex board system using Hexx for proper hex mathematics
#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct HexBoard {
    pub layout: HexLayout,
    pub size: Hex,
    pub cost_grid: HashMap<Hex, f32>,
    pub material_weights: HashMap<Hex, MaterialWeights>,
    pub interactive_objects: Vec<InteractiveObject>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MaterialWeights {
    pub grass: f32,
    pub dirt: f32, 
    pub sand: f32,
    pub rock: f32,
    pub water: f32,
    pub lava: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InteractiveObject {
    pub id: String,
    pub position: Hex,
    pub object_type: ObjectType,
    pub tags: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ObjectType {
    Door,
    Well,
    Altar,
    TestEntry(TestType),
    Mount(String),
    Npc(String),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TestType {
    Strength,
    Harmony,
    Morality,
}

impl Default for HexBoard {
    fn default() -> Self {
        Self {
            layout: HexLayout::POINTY,
            size: Hex::new(20, 20),
            cost_grid: HashMap::new(),
            material_weights: HashMap::new(),
            interactive_objects: Vec::new(),
        }
    }
}

impl HexBoard {
    // Generate navigation grid following vision document beauty texture approach
    pub fn generate_from_splatmap(&mut self, splatmap_path: &str, dread_level: u8) {
        // Load splatmap texture and extract material weights
        // Each RGBA channel represents a material weight
        for q in -self.size.x..=self.size.x {
            let r1 = (-self.size.y).max(-q - self.size.y);
            let r2 = self.size.y.min(-q + self.size.y);
            
            for r in r1..=r2 {
                let hex = Hex::new(q, r);
                
                // Sample splatmap at hex position
                let (x, y) = self.layout.hex_to_world_pos(hex);
                let weights = self.sample_splatmap_at(splatmap_path, x, y);
                
                // Calculate movement cost based on material weights and dread level
                let base_cost = self.calculate_movement_cost(&weights);
                let dread_modifier = 1.0 + (dread_level as f32 * 0.2); // Costs increase with dread
                
                self.cost_grid.insert(hex, base_cost * dread_modifier);
                self.material_weights.insert(hex, weights);
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
                    self.cost_grid.get(&neighbor).map(|&cost| (neighbor, cost))
                })
                .collect::<Vec<_>>()
        };
        
        let heuristic = |hex: Hex| hex.distance_to(goal) as f32;
        
        a_star(&start, successors, heuristic, |hex| *hex == goal)
            .map(|(path, _cost)| path)
    }
    
    // Field of view calculation for Tests of Harmony
    pub fn calculate_field_of_view(&self, center: Hex, range: u32) -> Vec<Hex> {
        use hexx::algorithms::field_of_view;
        
        let is_transparent = |hex: Hex| {
            // Check if hex blocks line of sight based on materials
            if let Some(weights) = self.material_weights.get(&hex) {
                weights.rock < 0.7 && weights.lava < 0.5 // Rock and lava block sight
            } else {
                true
            }
        };
        
        field_of_view(center, range, is_transparent)
    }
    
    // Mount aura system - modifies terrain costs within radius
    pub fn apply_mount_aura(&mut self, center: Hex, mount_type: &str, alignment: f32) {
        let aura_range = self.calculate_aura_range(mount_type, alignment);
        
        for hex in center.spiral_range(0..=aura_range) {
            if let Some(cost) = self.cost_grid.get_mut(&hex) {
                let distance = center.distance_to(hex) as f32;
                let strength = (1.0 - distance / aura_range as f32).max(0.0);
                
                match mount_type {
                    "seastrider" => {
                        // Reduces water costs
                        if let Some(weights) = self.material_weights.get(&hex) {
                            if weights.water > 0.5 {
                                *cost *= 1.0 - (strength * 0.8);
                            }
                        }
                    },
                    "rock_crusher" => {
                        // Reduces rock/mountain costs
                        if let Some(weights) = self.material_weights.get(&hex) {
                            if weights.rock > 0.5 {
                                *cost *= 1.0 - (strength * 0.7);
                            }
                        }
                    },
                    _ => {}
                }
            }
        }
    }
    
    // Generate setpiece board for Tests as per vision document
    pub fn create_setpiece_board(test_type: TestType, dread_level: u8) -> Self {
        let mut board = HexBoard::default();
        board.size = Hex::new(10, 10); // Smaller focused boards
        
        match test_type {
            TestType::Strength => {
                // Tactical positioning layout
                board.generate_strength_test_layout(dread_level);
            },
            TestType::Harmony => {
                // Synchronization challenge layout
                board.generate_harmony_test_layout(dread_level);
            },
            TestType::Morality => {
                // Choice consequence layout
                board.generate_morality_test_layout(dread_level);
            },
        }
        
        board
    }
    
    // Private helper methods
    fn sample_splatmap_at(&self, _splatmap_path: &str, _x: f32, _y: f32) -> MaterialWeights {
        // In real implementation, would load and sample texture
        // For now, generate procedural weights
        MaterialWeights {
            grass: 0.6,
            dirt: 0.2,
            sand: 0.1,
            rock: 0.05,
            water: 0.03,
            lava: 0.02,
        }
    }
    
    fn calculate_movement_cost(&self, weights: &MaterialWeights) -> f32 {
        // Base costs per material type
        let grass_cost = 1.0;
        let dirt_cost = 1.2;
        let sand_cost = 1.5;
        let rock_cost = 3.0;
        let water_cost = 5.0;
        let lava_cost = 10.0;
        
        weights.grass * grass_cost +
        weights.dirt * dirt_cost +
        weights.sand * sand_cost +
        weights.rock * rock_cost +
        weights.water * water_cost +
        weights.lava * lava_cost
    }
    
    fn calculate_aura_range(&self, mount_type: &str, alignment: f32) -> u32 {
        let base_range = match mount_type {
            "seastrider" => 3,
            "rock_crusher" => 2,
            _ => 1,
        };
        
        // Alignment affects aura strength: good alignment = bonding, evil = enslavement
        let alignment_bonus = if alignment > 0.5 {
            2 // Bonded mount has larger aura
        } else if alignment < -0.5 {
            1 // Enslaved mount has reduced aura
        } else {
            0 // Neutral
        };
        
        base_range + alignment_bonus
    }
    
    fn generate_strength_test_layout(&mut self, dread_level: u8) {
        // Create tactical encounter space
        for hex in Hex::ZERO.spiral_range(0..=5) {
            let distance = Hex::ZERO.distance_to(hex);
            
            let weights = if distance <= 2 {
                // Center area - clear for movement
                MaterialWeights {
                    grass: 0.8,
                    dirt: 0.2,
                    sand: 0.0,
                    rock: 0.0,
                    water: 0.0,
                    lava: 0.0,
                }
            } else {
                // Outer ring - obstacles based on dread level
                let obstacle_intensity = dread_level as f32 / 4.0;
                MaterialWeights {
                    grass: 0.3 * (1.0 - obstacle_intensity),
                    dirt: 0.2,
                    sand: 0.1,
                    rock: 0.3 * obstacle_intensity,
                    water: 0.05 * obstacle_intensity,
                    lava: 0.05 * obstacle_intensity,
                }
            };
            
            let cost = self.calculate_movement_cost(&weights);
            self.cost_grid.insert(hex, cost);
            self.material_weights.insert(hex, weights);
        }
        
        // Add spawn points for enemies
        let spawn_points = vec![
            Hex::new(4, 0), Hex::new(-2, 4), Hex::new(-2, -4)
        ];
        
        for (i, spawn) in spawn_points.iter().enumerate() {
            self.interactive_objects.push(InteractiveObject {
                id: format!("enemy_spawn_{}", i),
                position: *spawn,
                object_type: ObjectType::TestEntry(TestType::Strength),
                tags: vec!["spawn".to_string(), "enemy".to_string()],
            });
        }
    }
    
    fn generate_harmony_test_layout(&mut self, _dread_level: u8) {
        // Create symmetrical layout for synchronization challenges
        for hex in Hex::ZERO.spiral_range(0..=4) {
            let weights = MaterialWeights {
                grass: 0.9,
                dirt: 0.1,
                sand: 0.0,
                rock: 0.0,
                water: 0.0,
                lava: 0.0,
            };
            
            let cost = self.calculate_movement_cost(&weights);
            self.cost_grid.insert(hex, cost);
            self.material_weights.insert(hex, weights);
        }
        
        // Add harmony nodes in pattern
        let harmony_positions = vec![
            Hex::new(0, 3), Hex::new(3, 0), Hex::new(0, -3), Hex::new(-3, 0)
        ];
        
        for (i, pos) in harmony_positions.iter().enumerate() {
            self.interactive_objects.push(InteractiveObject {
                id: format!("harmony_node_{}", i),
                position: *pos,
                object_type: ObjectType::TestEntry(TestType::Harmony),
                tags: vec!["harmony".to_string(), "sync".to_string()],
            });
        }
    }
    
    fn generate_morality_test_layout(&mut self, dread_level: u8) {
        // Create choice-focused layout
        for hex in Hex::ZERO.spiral_range(0..=3) {
            let weights = MaterialWeights {
                grass: 0.5,
                dirt: 0.5,
                sand: 0.0,
                rock: 0.0,
                water: 0.0,
                lava: 0.0,
            };
            
            let cost = self.calculate_movement_cost(&weights);
            self.cost_grid.insert(hex, cost);
            self.material_weights.insert(hex, weights);
        }
        
        // Add moral choice objects
        let choice_intensity = match dread_level {
            0..=1 => "minor",
            2..=3 => "significant", 
            4 => "ultimate",
            _ => "unknown",
        };
        
        self.interactive_objects.push(InteractiveObject {
            id: "moral_choice_altar".to_string(),
            position: Hex::ZERO,
            object_type: ObjectType::Altar,
            tags: vec!["choice".to_string(), choice_intensity.to_string()],
        });
    }
}

// Plugin for hex board management
pub struct HexBoardPlugin;

impl Plugin for HexBoardPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                update_mount_auras,
                process_board_interactions,
                handle_setpiece_transitions,
            ));
    }
}

fn update_mount_auras(
    mut boards: Query<&mut HexBoard>,
    mount_query: Query<(&Transform, &MountComponent), Changed<Transform>>,
) {
    // Update mount aura effects when mounts move
    for (transform, mount) in mount_query.iter() {
        for mut board in boards.iter_mut() {
            let hex_pos = board.layout.world_pos_to_hex(transform.translation.xz());
            board.apply_mount_aura(hex_pos, &mount.mount_type, mount.alignment);
        }
    }
}

fn process_board_interactions(
    mut interaction_events: EventWriter<BoardInteractionEvent>,
    board_query: Query<&HexBoard>,
    input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    if input.just_pressed(MouseButton::Left) {
        if let Ok(window) = windows.get_single() {
            if let Some(cursor_pos) = window.cursor_position() {
                if let Ok((camera, camera_transform)) = camera_query.get_single() {
                    // Convert screen position to hex coordinate
                    // This would involve proper ray casting in real implementation
                    for board in board_query.iter() {
                        // Check for interactions with objects
                        for obj in &board.interactive_objects {
                            interaction_events.send(BoardInteractionEvent {
                                object_id: obj.id.clone(),
                                position: obj.position,
                                object_type: obj.object_type.clone(),
                            });
                        }
                    }
                }
            }
        }
    }
}

fn handle_setpiece_transitions(
    mut commands: Commands,
    mut interaction_events: EventReader<BoardInteractionEvent>,
    dread_state: Res<crate::resources::DreadState>,
) {
    for event in interaction_events.read() {
        match &event.object_type {
            ObjectType::TestEntry(test_type) => {
                info!("Transitioning to {:?} test at {:?}", test_type, event.position);
                
                // Create setpiece board
                let setpiece = HexBoard::create_setpiece_board(test_type.clone(), dread_state.current_level);
                
                // Spawn setpiece board entity
                commands.spawn((
                    setpiece,
                    SetpieceMarker,
                    Name::new(format!("{:?}_test_board", test_type)),
                ));
            },
            _ => {}
        }
    }
}

// Components and Events
#[derive(Component)]
pub struct MountComponent {
    pub mount_type: String,
    pub alignment: f32,
}

#[derive(Component)]
pub struct SetpieceMarker;

#[derive(Event)]
pub struct BoardInteractionEvent {
    pub object_id: String,
    pub position: Hex,
    pub object_type: ObjectType,
}