use bevy::prelude::*;
use dragons_core::components::*;
use hexx::{Hex, HexLayout, HexOrientation};

/// Health component for AI entities
#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Health {
    pub fn percentage(&self) -> f32 {
        self.current / self.max
    }
}

/// AI behavior states
#[derive(Component, Clone, Debug)]
pub enum AIBehavior {
    Idle,
    Patrol { waypoints: Vec<Hex>, current_index: usize },
    Chase { target: Entity },
    Attack { target: Entity },
    Flee { from: Entity },
}

/// AI agent component
#[derive(Component)]
pub struct AIAgent {
    pub behavior: AIBehavior,
    pub detection_range: f32,
    pub attack_range: f32,
    pub flee_threshold: f32, // Health percentage to trigger flee
    pub speed: f32,
}

impl Default for AIAgent {
    fn default() -> Self {
        Self {
            behavior: AIBehavior::Idle,
            detection_range: 5.0,
            attack_range: 1.0,
            flee_threshold: 0.25,
            speed: 3.0,
        }
    }
}

/// System to update AI behaviors based on game state
pub fn update_ai_behaviors(
    mut ai_query: Query<(&mut AIAgent, &HexPosition, &Health, Entity), Without<Player>>,
    player_query: Query<(&HexPosition, Entity), With<Player>>,
    time: Res<Time>,
) {
    let Ok((player_pos, player_entity)) = player_query.get_single() else {
        return;
    };
    
    for (mut ai, ai_pos, health, ai_entity) in ai_query.iter_mut() {
        let distance_to_player = ai_pos.0.unsigned_distance_to(player_pos.0) as f32;
        
        // Check if should flee (low health)
        if health.percentage() < ai.flee_threshold {
            ai.behavior = AIBehavior::Flee { from: player_entity };
            continue;
        }
        
        // Update behavior based on current state
        match &ai.behavior {
            AIBehavior::Idle => {
                // Check if player is in detection range
                if distance_to_player <= ai.detection_range {
                    ai.behavior = AIBehavior::Chase { target: player_entity };
                }
            }
            AIBehavior::Patrol { waypoints, current_index } => {
                // Check if player is in detection range
                if distance_to_player <= ai.detection_range {
                    ai.behavior = AIBehavior::Chase { target: player_entity };
                } else {
                    // Continue patrolling
                    // TODO: Move towards current waypoint
                }
            }
            AIBehavior::Chase { target } => {
                if distance_to_player <= ai.attack_range {
                    // In attack range
                    ai.behavior = AIBehavior::Attack { target: *target };
                } else if distance_to_player > ai.detection_range * 1.5 {
                    // Lost target
                    ai.behavior = AIBehavior::Idle;
                }
                // TODO: Move towards target
            }
            AIBehavior::Attack { target } => {
                if distance_to_player > ai.attack_range {
                    // Out of attack range, chase again
                    ai.behavior = AIBehavior::Chase { target: *target };
                }
                // Attack is handled by combat system
            }
            AIBehavior::Flee { from } => {
                if distance_to_player > ai.detection_range * 2.0 {
                    // Far enough away
                    ai.behavior = AIBehavior::Idle;
                }
                // TODO: Move away from threat
            }
        }
    }
}

/// System for patrol behavior
pub fn patrol_behavior_system(
    mut ai_query: Query<(&mut AIAgent, &mut HexPosition, &mut Transform)>,
    hex_world: Res<HexWorld>,
    time: Res<Time>,
) {
    for (mut ai, mut hex_pos, mut transform) in ai_query.iter_mut() {
        if let AIBehavior::Patrol { waypoints, current_index } = &mut ai.behavior {
            if waypoints.is_empty() {
                continue;
            }
            
            let target_hex = waypoints[*current_index];
            let current_hex = hex_pos.0;
            
            if current_hex == target_hex {
                // Reached waypoint, move to next
                *current_index = (*current_index + 1) % waypoints.len();
            } else {
                // Move towards waypoint
                // Simple movement - find neighbor closest to target
                let neighbors = current_hex.all_neighbors();
                let mut best_neighbor = current_hex;
                let mut best_distance = current_hex.unsigned_distance_to(target_hex);
                
                for neighbor in neighbors {
                    if let Some(tile) = hex_world.tiles.get(&neighbor) {
                        if tile.passable {
                            let distance = neighbor.unsigned_distance_to(target_hex);
                            if distance < best_distance {
                                best_distance = distance;
                                best_neighbor = neighbor;
                            }
                        }
                    }
                }
                
                if best_neighbor != current_hex {
                    hex_pos.0 = best_neighbor;
                    // Update world position
                    let layout = HexLayout {
                        orientation: HexOrientation::Flat,
                        origin: Vec2::ZERO,
                        hex_size: Vec2::splat(1.0),
                        invert_x: false,
                        invert_y: false,
                    };
                    let world_pos = layout.hex_to_world_pos(best_neighbor);
                    transform.translation.x = world_pos.x;
                    transform.translation.z = world_pos.y;
                }
            }
        }
    }
}

/// System for chase behavior
pub fn chase_behavior_system(
    mut ai_query: Query<(&AIAgent, &mut HexPosition, &mut Transform), Without<Player>>,
    player_query: Query<&HexPosition, With<Player>>,
    hex_world: Res<HexWorld>,
    time: Res<Time>,
) {
    let Ok(player_pos) = player_query.get_single() else {
        return;
    };
    
    for (ai, mut hex_pos, mut transform) in ai_query.iter_mut() {
        if let AIBehavior::Chase { .. } = ai.behavior {
            let current_hex = hex_pos.0;
            let target_hex = player_pos.0;
            
            // Simple pathfinding - move towards player
            let neighbors = current_hex.all_neighbors();
            let mut best_neighbor = current_hex;
            let mut best_distance = current_hex.unsigned_distance_to(target_hex);
            
            for neighbor in neighbors {
                if let Some(tile) = hex_world.tiles.get(&neighbor) {
                    if tile.passable {
                        let distance = neighbor.unsigned_distance_to(target_hex);
                        if distance < best_distance {
                            best_distance = distance;
                            best_neighbor = neighbor;
                        }
                    }
                }
            }
            
            if best_neighbor != current_hex {
                hex_pos.0 = best_neighbor;
                // Update world position
                let layout = HexLayout {
                    orientation: HexOrientation::Flat,
                    origin: Vec2::ZERO,
                    hex_size: Vec2::splat(1.0),
                    invert_x: false,
                    invert_y: false,
                };
                let world_pos = layout.hex_to_world_pos(best_neighbor);
                transform.translation.x = world_pos.x;
                transform.translation.z = world_pos.y;
            }
        }
    }
}

/// System for flee behavior
pub fn flee_behavior_system(
    mut ai_query: Query<(&AIAgent, &mut HexPosition, &mut Transform), Without<Player>>,
    player_query: Query<&HexPosition, With<Player>>,
    hex_world: Res<HexWorld>,
    time: Res<Time>,
) {
    let Ok(player_pos) = player_query.get_single() else {
        return;
    };
    
    for (ai, mut hex_pos, mut transform) in ai_query.iter_mut() {
        if let AIBehavior::Flee { .. } = ai.behavior {
            let current_hex = hex_pos.0;
            let threat_hex = player_pos.0;
            
            // Move away from threat
            let neighbors = current_hex.all_neighbors();
            let mut best_neighbor = current_hex;
            let mut best_distance = 0u32;
            
            for neighbor in neighbors {
                if let Some(tile) = hex_world.tiles.get(&neighbor) {
                    if tile.passable {
                        let distance = neighbor.unsigned_distance_to(threat_hex);
                        if distance > best_distance {
                            best_distance = distance;
                            best_neighbor = neighbor;
                        }
                    }
                }
            }
            
            if best_neighbor != current_hex {
                hex_pos.0 = best_neighbor;
                // Update world position
                let layout = HexLayout {
                    orientation: HexOrientation::Flat,
                    origin: Vec2::ZERO,
                    hex_size: Vec2::splat(1.0),
                    invert_x: false,
                    invert_y: false,
                };
                let world_pos = layout.hex_to_world_pos(best_neighbor);
                transform.translation.x = world_pos.x;
                transform.translation.z = world_pos.y;
            }
        }
    }
}