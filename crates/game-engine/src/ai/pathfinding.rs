use bevy::prelude::*;
use pathfinding::prelude::{astar, dijkstra, bfs};
use hexx::{Hex, HexLayout};
use std::collections::{HashMap, HashSet};

/// AI Pathfinding system integrating the pathfinding crate
/// This provides sophisticated pathfinding for NPCs, companions, and the dragon
pub struct PathfindingPlugin;

impl Plugin for PathfindingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PathfindingGrid>()
            .init_resource::<PathfindingCache>()
            .add_event::<PathfindingRequest>()
            .add_event::<PathfindingResult>()
            .add_systems(Update, (
                process_pathfinding_requests,
                update_companion_paths,
                update_dragon_stalking_path,
                update_npc_flee_paths,
                visualize_paths,
            ).chain());
    }
}

/// Grid data for pathfinding calculations
#[derive(Resource)]
pub struct PathfindingGrid {
    /// Cost map for each hex position
    pub cost_map: HashMap<Hex, f32>,
    /// Walkable status for each hex
    pub walkable_map: HashMap<Hex, bool>,
    /// Special zones that affect pathfinding
    pub zone_map: HashMap<Hex, PathfindingZone>,
    /// Hex layout for coordinate conversion
    pub hex_layout: HexLayout,
    /// Grid bounds
    pub bounds: (Hex, Hex),
}

impl Default for PathfindingGrid {
    fn default() -> Self {
        Self {
            cost_map: HashMap::new(),
            walkable_map: HashMap::new(),
            zone_map: HashMap::new(),
            hex_layout: HexLayout::POINTY,
            bounds: (Hex::new(-50, -50), Hex::new(50, 50)),
        }
    }
}

/// Cache for storing computed paths
#[derive(Resource, Default)]
pub struct PathfindingCache {
    /// Cached paths indexed by (start, goal)
    pub cached_paths: HashMap<(Hex, Hex), Vec<Hex>>,
    /// Time since last cache clear
    pub cache_age: f32,
    /// Maximum cache age before clearing
    pub max_cache_age: f32,
}

/// Special zones that affect pathfinding behavior
#[derive(Clone, Debug)]
pub enum PathfindingZone {
    /// Areas to avoid (high cost)
    Danger(f32),
    /// Safe zones (reduced cost)
    Safe(f32),
    /// Impassable areas
    Blocked,
    /// Dragon territory (NPCs flee)
    DragonTerritory,
    /// Corruption zones (affects based on dread)
    Corrupted(f32),
}

/// Request for pathfinding calculation
#[derive(Event)]
pub struct PathfindingRequest {
    pub entity: Entity,
    pub start: Hex,
    pub goal: Hex,
    pub pathfinding_type: PathfindingType,
    pub max_cost: Option<f32>,
}

/// Type of pathfinding to perform
#[derive(Clone, Debug)]
pub enum PathfindingType {
    /// Standard A* pathfinding
    Standard,
    /// Flee from a position
    Flee(Hex),
    /// Stalking (tries to stay hidden)
    Stalking,
    /// Group movement (stays together)
    Group(Vec<Entity>),
    /// Patrol route
    Patrol(Vec<Hex>),
}

/// Result of pathfinding calculation
#[derive(Event)]
pub struct PathfindingResult {
    pub entity: Entity,
    pub path: Option<Vec<Hex>>,
    pub total_cost: f32,
}

/// Component for entities that can pathfind
#[derive(Component)]
pub struct Pathfinder {
    pub current_path: Vec<Hex>,
    pub path_index: usize,
    pub movement_speed: f32,
    pub recalculate_distance: f32,
}

impl Default for Pathfinder {
    fn default() -> Self {
        Self {
            current_path: Vec::new(),
            path_index: 0,
            movement_speed: 5.0,
            recalculate_distance: 10.0,
        }
    }
}

/// Process pathfinding requests
fn process_pathfinding_requests(
    mut requests: EventReader<PathfindingRequest>,
    mut results: EventWriter<PathfindingResult>,
    grid: Res<PathfindingGrid>,
    mut cache: ResMut<PathfindingCache>,
    dread_state: Res<crate::resources::DreadState>,
) {
    for request in requests.read() {
        let path = match request.pathfinding_type {
            PathfindingType::Standard => {
                calculate_standard_path(&grid, request.start, request.goal, request.max_cost, &mut cache)
            }
            PathfindingType::Flee(threat_pos) => {
                calculate_flee_path(&grid, request.start, threat_pos, request.max_cost)
            }
            PathfindingType::Stalking => {
                calculate_stalking_path(&grid, request.start, request.goal, dread_state.current_level)
            }
            PathfindingType::Group(ref group_entities) => {
                calculate_group_path(&grid, request.start, request.goal, group_entities.len())
            }
            PathfindingType::Patrol(ref waypoints) => {
                calculate_patrol_path(&grid, request.start, waypoints)
            }
        };
        
        let total_cost = if let Some(ref p) = path {
            calculate_path_cost(&grid, p)
        } else {
            f32::INFINITY
        };
        
        results.send(PathfindingResult {
            entity: request.entity,
            path,
            total_cost,
        });
    }
}

/// Calculate standard A* path
fn calculate_standard_path(
    grid: &PathfindingGrid,
    start: Hex,
    goal: Hex,
    max_cost: Option<f32>,
    cache: &mut PathfindingCache,
) -> Option<Vec<Hex>> {
    // Check cache first
    if let Some(cached) = cache.cached_paths.get(&(start, goal)) {
        return Some(cached.clone());
    }
    
    let result = astar(
        &start,
        |&hex| {
            hex.all_neighbors()
                .into_iter()
                .filter_map(|neighbor| {
                    if let Some(&walkable) = grid.walkable_map.get(&neighbor) {
                        if walkable {
                            let cost = grid.cost_map.get(&neighbor).copied().unwrap_or(1.0);
                            
                            // Apply zone modifiers
                            let zone_cost = if let Some(zone) = grid.zone_map.get(&neighbor) {
                                match zone {
                                    PathfindingZone::Danger(multiplier) => cost * multiplier,
                                    PathfindingZone::Safe(multiplier) => cost / multiplier,
                                    PathfindingZone::Blocked => return None,
                                    PathfindingZone::DragonTerritory => cost * 3.0,
                                    PathfindingZone::Corrupted(level) => cost * (1.0 + level),
                                }
                            } else {
                                cost
                            };
                            
                            if let Some(max) = max_cost {
                                if zone_cost > max {
                                    return None;
                                }
                            }
                            
                            Some((neighbor, zone_cost as u32))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        },
        |&hex| hex.unsigned_distance_to(goal) as u32,
        |&hex| hex == goal,
    );
    
    if let Some((path, _cost)) = result {
        // Cache the result
        cache.cached_paths.insert((start, goal), path.clone());
        Some(path)
    } else {
        None
    }
}

/// Calculate flee path (away from threat)
fn calculate_flee_path(
    grid: &PathfindingGrid,
    start: Hex,
    threat: Hex,
    max_distance: Option<f32>,
) -> Option<Vec<Hex>> {
    // Find positions that maximize distance from threat
    let flee_distance = max_distance.unwrap_or(20.0) as u32;
    
    // Use BFS to find reachable positions
    let reachable = bfs_reach(
        &start,
        |&hex| {
            hex.all_neighbors()
                .into_iter()
                .filter(|neighbor| {
                    grid.walkable_map.get(neighbor).copied().unwrap_or(false)
                })
                .collect::<Vec<_>>()
        },
        |&hex| hex.unsigned_distance_to(start) <= flee_distance,
    );
    
    // Find the reachable position furthest from threat
    let best_flee_position = reachable
        .into_iter()
        .max_by_key(|hex| hex.unsigned_distance_to(threat))
        .unwrap_or(start);
    
    // Path to the flee position
    calculate_standard_path(grid, start, best_flee_position, None, &mut PathfindingCache::default())
}

/// Calculate stalking path (stays in cover/shadows)
fn calculate_stalking_path(
    grid: &PathfindingGrid,
    start: Hex,
    goal: Hex,
    dread_level: u8,
) -> Option<Vec<Hex>> {
    // Prefer paths through corrupted/dark zones at higher dread levels
    let result = astar(
        &start,
        |&hex| {
            hex.all_neighbors()
                .into_iter()
                .filter_map(|neighbor| {
                    if let Some(&walkable) = grid.walkable_map.get(&neighbor) {
                        if walkable {
                            let base_cost = grid.cost_map.get(&neighbor).copied().unwrap_or(1.0);
                            
                            // Prefer corrupted zones for stalking at high dread
                            let stalking_cost = if let Some(zone) = grid.zone_map.get(&neighbor) {
                                match zone {
                                    PathfindingZone::Corrupted(level) => {
                                        // Lower cost for corrupted areas when stalking
                                        base_cost / (1.0 + level * dread_level as f32 * 0.2)
                                    }
                                    PathfindingZone::Safe(_) => base_cost * 2.0, // Avoid safe zones
                                    _ => base_cost,
                                }
                            } else {
                                base_cost
                            };
                            
                            Some((neighbor, stalking_cost as u32))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        },
        |&hex| hex.unsigned_distance_to(goal) as u32,
        |&hex| hex == goal,
    );
    
    result.map(|(path, _)| path)
}

/// Calculate group path (keeps group together)
fn calculate_group_path(
    grid: &PathfindingGrid,
    start: Hex,
    goal: Hex,
    group_size: usize,
) -> Option<Vec<Hex>> {
    // Find path with enough width for group movement
    let result = dijkstra(
        &start,
        |&hex| {
            hex.all_neighbors()
                .into_iter()
                .filter_map(|neighbor| {
                    if let Some(&walkable) = grid.walkable_map.get(&neighbor) {
                        if walkable {
                            // Check if neighboring hexes are also walkable for group
                            let group_walkable = neighbor.all_neighbors()
                                .into_iter()
                                .filter(|n| grid.walkable_map.get(n).copied().unwrap_or(false))
                                .count() >= group_size.min(3);
                            
                            if group_walkable {
                                let cost = grid.cost_map.get(&neighbor).copied().unwrap_or(1.0);
                                Some((neighbor, cost as u32))
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        },
        |&hex| hex == goal,
    );
    
    result.map(|(path, _)| path)
}

/// Calculate patrol path through waypoints
fn calculate_patrol_path(
    grid: &PathfindingGrid,
    start: Hex,
    waypoints: &[Hex],
) -> Option<Vec<Hex>> {
    if waypoints.is_empty() {
        return None;
    }
    
    let mut full_path = Vec::new();
    let mut current = start;
    let mut cache = PathfindingCache::default();
    
    // Path through each waypoint
    for &waypoint in waypoints.iter() {
        if let Some(segment) = calculate_standard_path(grid, current, waypoint, None, &mut cache) {
            // Skip the first hex (it's the last hex of the previous segment)
            if !full_path.is_empty() && !segment.is_empty() {
                full_path.extend_from_slice(&segment[1..]);
            } else {
                full_path.extend_from_slice(&segment);
            }
            current = waypoint;
        } else {
            return None; // Can't reach waypoint
        }
    }
    
    // Path back to start to complete the patrol loop
    if let Some(segment) = calculate_standard_path(grid, current, start, None, &mut cache) {
        if !segment.is_empty() {
            full_path.extend_from_slice(&segment[1..]);
        }
    }
    
    Some(full_path)
}

/// Update companion pathfinding
fn update_companion_paths(
    mut companions: Query<(&Transform, &mut Pathfinder, &crate::components::Companion)>,
    player: Query<&Transform, With<crate::components::Player>>,
    mut requests: EventWriter<PathfindingRequest>,
    grid: Res<PathfindingGrid>,
) {
    let Ok(player_transform) = player.get_single() else { return };
    let player_hex = grid.hex_layout.world_pos_to_hex(player_transform.translation.xz());
    
    for (transform, mut pathfinder, companion) in companions.iter_mut() {
        let companion_hex = grid.hex_layout.world_pos_to_hex(transform.translation.xz());
        let distance_to_player = companion_hex.unsigned_distance_to(player_hex) as f32;
        
        // Recalculate path if too far from player or path is empty
        if distance_to_player > pathfinder.recalculate_distance || pathfinder.current_path.is_empty() {
            // Companions with high trauma may flee instead of follow
            let pathfinding_type = if companion.trauma > 0.8 {
                PathfindingType::Flee(player_hex)
            } else {
                PathfindingType::Standard
            };
            
            requests.send(PathfindingRequest {
                entity: Entity::PLACEHOLDER, // Would be actual entity
                start: companion_hex,
                goal: player_hex,
                pathfinding_type,
                max_cost: Some(50.0),
            });
        }
    }
}

/// Update dragon stalking path
fn update_dragon_stalking_path(
    mut dragon: Query<(&Transform, &mut Pathfinder), With<crate::components::Dragon>>,
    player: Query<&Transform, With<crate::components::Player>>,
    mut requests: EventWriter<PathfindingRequest>,
    grid: Res<PathfindingGrid>,
    dread_state: Res<crate::resources::DreadState>,
) {
    // Dragon only stalks at high dread levels
    if dread_state.current_level < 3 {
        return;
    }
    
    let Ok(player_transform) = player.get_single() else { return };
    let Ok((dragon_transform, mut pathfinder)) = dragon.get_single_mut() else { return };
    
    let player_hex = grid.hex_layout.world_pos_to_hex(player_transform.translation.xz());
    let dragon_hex = grid.hex_layout.world_pos_to_hex(dragon_transform.translation.xz());
    
    // Recalculate stalking path periodically
    if pathfinder.current_path.is_empty() || pathfinder.path_index >= pathfinder.current_path.len() {
        requests.send(PathfindingRequest {
            entity: Entity::PLACEHOLDER,
            start: dragon_hex,
            goal: player_hex,
            pathfinding_type: PathfindingType::Stalking,
            max_cost: None,
        });
    }
}

/// Update NPC flee paths when dragon is near
fn update_npc_flee_paths(
    mut npcs: Query<(&Transform, &mut Pathfinder), With<crate::components::Npc>>,
    dragon: Query<&Transform, With<crate::components::Dragon>>,
    mut requests: EventWriter<PathfindingRequest>,
    grid: Res<PathfindingGrid>,
    dread_state: Res<crate::resources::DreadState>,
) {
    if dread_state.current_level < 2 {
        return; // NPCs don't flee at low dread
    }
    
    let Ok(dragon_transform) = dragon.get_single() else { return };
    let dragon_hex = grid.hex_layout.world_pos_to_hex(dragon_transform.translation.xz());
    
    for (transform, mut pathfinder) in npcs.iter_mut() {
        let npc_hex = grid.hex_layout.world_pos_to_hex(transform.translation.xz());
        let distance_to_dragon = npc_hex.unsigned_distance_to(dragon_hex) as f32;
        
        // Flee if dragon is too close
        if distance_to_dragon < 15.0 {
            requests.send(PathfindingRequest {
                entity: Entity::PLACEHOLDER,
                start: npc_hex,
                goal: npc_hex, // Not used for flee
                pathfinding_type: PathfindingType::Flee(dragon_hex),
                max_cost: Some(30.0),
            });
        }
    }
}

/// Visualize paths for debugging
fn visualize_paths(
    pathfinders: Query<(&Transform, &Pathfinder)>,
    mut gizmos: Gizmos,
    grid: Res<PathfindingGrid>,
) {
    for (transform, pathfinder) in pathfinders.iter() {
        if pathfinder.current_path.is_empty() {
            continue;
        }
        
        // Draw path as connected lines
        let mut prev_pos = transform.translation;
        
        for (i, &hex) in pathfinder.current_path.iter().enumerate() {
            let world_pos = grid.hex_layout.hex_to_world_pos(hex);
            let pos = Vec3::new(world_pos.x, transform.translation.y, world_pos.y);
            
            // Color based on progress
            let color = if i < pathfinder.path_index {
                Color::srgb(0.5, 0.5, 0.5) // Already traversed
            } else if i == pathfinder.path_index {
                Color::srgb(0.0, 1.0, 0.0) // Current target
            } else {
                Color::srgb(0.0, 0.5, 1.0) // Future path
            };
            
            gizmos.line(prev_pos, pos, color);
            gizmos.sphere(pos, 0.2, color);
            
            prev_pos = pos;
        }
    }
}

// Helper functions

/// BFS reach implementation
fn bfs_reach<F, P>(start: &Hex, successors: F, predicate: P) -> Vec<Hex>
where
    F: Fn(&Hex) -> Vec<Hex>,
    P: Fn(&Hex) -> bool,
{
    let mut visited = HashSet::new();
    let mut queue = Vec::new();
    let mut result = Vec::new();
    
    queue.push(*start);
    visited.insert(*start);
    
    while let Some(current) = queue.pop() {
        if !predicate(&current) {
            continue;
        }
        
        result.push(current);
        
        for neighbor in successors(&current) {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor);
                queue.push(neighbor);
            }
        }
    }
    
    result
}

/// Calculate total cost of a path
fn calculate_path_cost(grid: &PathfindingGrid, path: &[Hex]) -> f32 {
    path.windows(2)
        .map(|window| {
            grid.cost_map.get(&window[1]).copied().unwrap_or(1.0)
        })
        .sum()
}

/// Update pathfinding grid from game world
pub fn update_pathfinding_grid(
    mut grid: ResMut<PathfindingGrid>,
    tiles: Query<(&crate::maps::CorruptibleTile, &hexx::Hex)>,
    dread_state: Res<crate::resources::DreadState>,
) {
    grid.cost_map.clear();
    grid.walkable_map.clear();
    grid.zone_map.clear();
    
    for (tile, &hex_pos) in tiles.iter() {
        // Base cost from terrain
        let base_cost = match tile.base_biome {
            crate::maps::BiomeType::Grassland => 1.0,
            crate::maps::BiomeType::Forest => 1.5,
            crate::maps::BiomeType::Desert => 2.0,
            crate::maps::BiomeType::Swamp => 3.0,
            crate::maps::BiomeType::Mountain => 5.0,
            crate::maps::BiomeType::Volcanic => 10.0,
            crate::maps::BiomeType::Corrupted => 4.0,
        };
        
        // Modify cost based on corruption
        let corruption_modifier = 1.0 + tile.corruption_level * 2.0;
        let final_cost = base_cost * corruption_modifier;
        
        grid.cost_map.insert(hex_pos, final_cost);
        grid.walkable_map.insert(hex_pos, final_cost < 100.0);
        
        // Add zone if corrupted
        if tile.corruption_level > 0.5 {
            grid.zone_map.insert(
                hex_pos,
                PathfindingZone::Corrupted(tile.corruption_level),
            );
        }
        
        // Mark dragon territory at high dread
        if dread_state.current_level >= 3 && tile.corruption_level > 0.8 {
            grid.zone_map.insert(hex_pos, PathfindingZone::DragonTerritory);
        }
    }
}
