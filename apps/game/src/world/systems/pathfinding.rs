use bevy::prelude::*;
use std::collections::{HashMap, VecDeque, BinaryHeap};
use std::cmp::Ordering;
use crate::world::components::tiles::{HexCoord, BiomeType};

#[derive(Component, Debug)]
pub struct PlayerPosition {
    pub hex_coord: HexCoord,
}

#[derive(Component, Debug)]
pub struct MovementPath {
    pub path: Vec<HexCoord>,
    pub movement_type: MovementType,
    pub fatigue_cost: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MovementType {
    Walk,
    Run,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TileAccessibility {
    Passable,      // Green - can move here easily
    Difficult,     // Yellow - movement penalty or negative effect
    Impassable,    // Red - cannot move here
}

#[derive(Debug)]
struct PathNode {
    coord: HexCoord,
    g_cost: f32,     // Cost from start
    h_cost: f32,     // Heuristic cost to goal
    parent: Option<HexCoord>,
}

impl PathNode {
    fn f_cost(&self) -> f32 {
        self.g_cost + self.h_cost
    }
}

impl Eq for PathNode {}

impl PartialEq for PathNode {
    fn eq(&self, other: &Self) -> bool {
        self.coord == other.coord
    }
}

impl Ord for PathNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering for min-heap behavior
        other.f_cost().partial_cmp(&self.f_cost()).unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for PathNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Resource, Debug)]
pub struct MovementPreview {
    pub highlighted_tiles: HashMap<HexCoord, TileAccessibility>,
    pub max_movement_range: u32,
    pub movement_costs: HashMap<HexCoord, f32>,
}

impl Default for MovementPreview {
    fn default() -> Self {
        Self {
            highlighted_tiles: HashMap::new(),
            max_movement_range: 8, // Base walking distance per day
            movement_costs: HashMap::new(),
        }
    }
}

pub fn get_hex_neighbors(coord: HexCoord) -> Vec<HexCoord> {
    // Hexagonal grid neighbors (6 directions)
    vec![
        HexCoord { x: coord.x + 1, y: coord.y },
        HexCoord { x: coord.x - 1, y: coord.y },
        HexCoord { x: coord.x, y: coord.y + 1 },
        HexCoord { x: coord.x, y: coord.y - 1 },
        HexCoord { x: coord.x + 1, y: coord.y - 1 },
        HexCoord { x: coord.x - 1, y: coord.y + 1 },
    ]
}

pub fn calculate_hex_distance(a: HexCoord, b: HexCoord) -> f32 {
    let dx = (a.x - b.x) as f32;
    let dy = (a.y - b.y) as f32;
    let dz = (-a.x - a.y + b.x + b.y) as f32;
    
    (dx.abs() + dy.abs() + dz.abs()) / 2.0
}

pub fn get_movement_cost(biome: &BiomeType, weather_intensity: f32) -> f32 {
    let base_cost = match biome {
        BiomeType::Grassland => 1.0,
        BiomeType::Forest => 1.2,
        BiomeType::Desert => 1.5,
        BiomeType::Mountain => 2.0,
        BiomeType::Swamp => 1.8,
        BiomeType::Water => 1.3, // Assuming shallow water/fording
        BiomeType::Lava => 3.0,
        BiomeType::Void => 4.0,
        BiomeType::Corrupted(_) => 2.5,
    };
    
    // Weather makes movement more difficult
    base_cost * (1.0 + weather_intensity * 0.5)
}

pub fn get_tile_accessibility(biome: &BiomeType, player_level: u32) -> TileAccessibility {
    match biome {
        BiomeType::Grassland | BiomeType::Forest => TileAccessibility::Passable,
        BiomeType::Desert | BiomeType::Mountain | BiomeType::Swamp => {
            if player_level >= 5 {
                TileAccessibility::Difficult
            } else {
                TileAccessibility::Impassable
            }
        },
        BiomeType::Water => TileAccessibility::Difficult,
        BiomeType::Lava => {
            if player_level >= 20 {
                TileAccessibility::Difficult
            } else {
                TileAccessibility::Impassable
            }
        },
        BiomeType::Void => {
            if player_level >= 40 {
                TileAccessibility::Difficult
            } else {
                TileAccessibility::Impassable
            }
        },
        BiomeType::Corrupted(_) => {
            if player_level >= 10 {
                TileAccessibility::Difficult
            } else {
                TileAccessibility::Impassable
            }
        },
    }
}

pub fn find_path_astar(
    start: HexCoord,
    goal: HexCoord,
    tile_map: &HashMap<HexCoord, BiomeType>,
    max_distance: f32,
    player_level: u32,
    weather_intensity: f32,
) -> Option<Vec<HexCoord>> {
    let mut open_set = BinaryHeap::new();
    let mut came_from: HashMap<HexCoord, HexCoord> = HashMap::new();
    let mut g_score: HashMap<HexCoord, f32> = HashMap::new();
    
    g_score.insert(start, 0.0);
    open_set.push(PathNode {
        coord: start,
        g_cost: 0.0,
        h_cost: calculate_hex_distance(start, goal),
        parent: None,
    });
    
    while let Some(current_node) = open_set.pop() {
        let current = current_node.coord;
        
        if current == goal {
            // Reconstruct path
            let mut path = vec![current];
            let mut current_coord = current;
            
            while let Some(&parent) = came_from.get(&current_coord) {
                path.push(parent);
                current_coord = parent;
            }
            
            path.reverse();
            return Some(path);
        }
        
        for neighbor in get_hex_neighbors(current) {
            // Check if neighbor exists in tile map
            if let Some(neighbor_biome) = tile_map.get(&neighbor) {
                let accessibility = get_tile_accessibility(neighbor_biome, player_level);
                
                if accessibility == TileAccessibility::Impassable {
                    continue;
                }
                
                let movement_cost = get_movement_cost(neighbor_biome, weather_intensity);
                let tentative_g_score = g_score[&current] + movement_cost;
                
                // Don't go beyond max movement range
                if tentative_g_score > max_distance {
                    continue;
                }
                
                if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&f32::INFINITY) {
                    came_from.insert(neighbor, current);
                    g_score.insert(neighbor, tentative_g_score);
                    
                    let h_cost = calculate_hex_distance(neighbor, goal);
                    open_set.push(PathNode {
                        coord: neighbor,
                        g_cost: tentative_g_score,
                        h_cost,
                        parent: Some(current),
                    });
                }
            }
        }
    }
    
    None // No path found
}

pub fn calculate_movement_preview(
    player_pos: HexCoord,
    tile_map: &HashMap<HexCoord, BiomeType>,
    max_movement: u32,
    player_level: u32,
    weather_intensity: f32,
) -> MovementPreview {
    let mut preview = MovementPreview {
        max_movement_range: max_movement,
        ..default()
    };
    
    let mut visited = HashMap::new();
    let mut queue = VecDeque::new();
    
    queue.push_back((player_pos, 0.0));
    visited.insert(player_pos, 0.0);
    
    while let Some((current_coord, current_cost)) = queue.pop_front() {
        for neighbor in get_hex_neighbors(current_coord) {
            if let Some(neighbor_biome) = tile_map.get(&neighbor) {
                let accessibility = get_tile_accessibility(neighbor_biome, player_level);
                let movement_cost = get_movement_cost(neighbor_biome, weather_intensity);
                let new_cost = current_cost + movement_cost;
                
                if new_cost <= max_movement as f32 {
                    if let Some(&existing_cost) = visited.get(&neighbor) {
                        if new_cost >= existing_cost {
                            continue;
                        }
                    }
                    
                    visited.insert(neighbor, new_cost);
                    preview.movement_costs.insert(neighbor, new_cost);
                    preview.highlighted_tiles.insert(neighbor, accessibility);
                    
                    if accessibility != TileAccessibility::Impassable {
                        queue.push_back((neighbor, new_cost));
                    }
                }
            }
        }
    }
    
    preview
}

pub fn calculate_fatigue_cost(
    path: &[HexCoord],
    movement_type: MovementType,
    tile_map: &HashMap<HexCoord, BiomeType>,
    weather_intensity: f32,
) -> f32 {
    let mut total_fatigue = 0.0;
    
    for coord in path {
        if let Some(biome) = tile_map.get(coord) {
            let base_cost = get_movement_cost(biome, weather_intensity);
            
            let fatigue_multiplier = match movement_type {
                MovementType::Walk => 1.0,
                MovementType::Run => 1.5, // Running is more tiring
            };
            
            total_fatigue += base_cost * fatigue_multiplier;
        }
    }
    
    total_fatigue
}

pub fn update_movement_preview(
    mut preview: ResMut<MovementPreview>,
    player_query: Query<&PlayerPosition>,
    // TODO: Add tile map query
    // TODO: Add weather system query
) {
    if let Ok(player_pos) = player_query.get_single() {
        // Update movement preview based on current player position
        // This will be called when player selects their character
    }
}

pub fn handle_movement_input(
    // TODO: Add input handling
    // TODO: Add player movement execution
) {
    // Handle mouse clicks for movement
    // Show path preview
    // Execute movement with fatigue calculation
}