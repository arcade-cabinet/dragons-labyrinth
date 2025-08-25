//! Movement Validation System - Layer Cake Priority System
//!
//! Demonstrates the priority-based layer cake system in action:
//! Player > Path > Biome rules with real movement validation.

use bevy::prelude::*;
use hexx::*;
use crate::components::{
    players::*,
    hex_tiles::*,
};

/// System that validates movement using layer cake priority rules
pub fn movement_validation_system(
    mut movement_events: EventReader<MovementRequestEvent>,
    mut movement_results: EventWriter<MovementResultEvent>,
    player_query: Query<(&Player, &PlayerMovement, &PlayerEquipment, &DayNightCycle)>,
    tile_query: Query<(&HexTile, &Biome, &Path, &Feature)>,
    accessibility_map: Res<TileAccessibilityMap>,
) {
    for movement_request in movement_events.read() {
        if let Ok((player, movement, equipment, day_cycle)) = player_query.get(movement_request.player_entity) {
            // Check if player has enough turns remaining
            if day_cycle.must_rest {
                movement_results.send(MovementResultEvent {
                    player_entity: movement_request.player_entity,
                    result: MovementResult::InsufficientMovement,
                    message: "Must rest before moving".to_string(),
                });
                continue;
            }
            
            // Evaluate movement using layer cake priority system
            let result = evaluate_movement_with_layer_cake(
                player.current_hex,
                movement_request.target_hex,
                movement,
                equipment,
                &accessibility_map,
                &tile_query,
            );
            
            movement_results.send(MovementResultEvent {
                player_entity: movement_request.player_entity,
                result,
                message: "Movement evaluated".to_string(),
            });
        }
    }
}

/// Core layer cake evaluation function - shows priority system in action
fn evaluate_movement_with_layer_cake(
    current_hex: Hex,
    target_hex: Hex,
    movement: &PlayerMovement,
    equipment: &PlayerEquipment,
    accessibility_map: &TileAccessibilityMap,
    tile_query: &Query<(&HexTile, &Biome, &Path, &Feature)>,
) -> MovementResult {
    let distance = current_hex.distance_to(target_hex) as u32;
    
    // Check movement points
    if distance > movement.movement_points {
        return MovementResult::InsufficientMovement;
    }
    
    // Get path and evaluate each tile using layer cake system
    let path = current_hex.line_to(target_hex);
    let mut total_turns_used = 0;
    let mut path_analysis = Vec::new();
    
    for hex in &path {
        if let Some(tile_evaluation) = evaluate_single_tile(*hex, equipment, tile_query) {
            path_analysis.push(tile_evaluation.clone());
            
            // Check if tile is completely impassable (no layer can override)
            if tile_evaluation.final_result.is_impassable {
                return MovementResult::Impassable(*hex);
            }
            
            // Add movement cost
            total_turns_used += tile_evaluation.final_result.movement_cost;
            
            // Check if movement penalty is too severe
            if tile_evaluation.final_result.movement_modifier < -3 {
                return MovementResult::TooSlow(*hex);
            }
        }
    }
    
    // Running gives decreased encounter chance for intermediate tiles
    let decreased_encounter_chance = movement.movement_type == MovementType::Run && distance > 1;
    
    MovementResult::Success {
        path,
        decreased_encounter_chance,
        turns_used: total_turns_used,
    }
}

/// Evaluate a single tile using the layer cake priority system
fn evaluate_single_tile(
    hex: Hex,
    equipment: &PlayerEquipment,
    tile_query: &Query<(&HexTile, &Biome, &Path, &Feature)>,
) -> Option<TileEvaluation> {
    // Find the tile entity at this hex
    for (hex_tile, biome, path, feature) in tile_query.iter() {
        if hex_tile.coord == hex {
            return Some(evaluate_layer_cake_rules(hex, biome, path, feature, equipment));
        }
    }
    None
}

/// The core layer cake evaluation - demonstrates priority system
fn evaluate_layer_cake_rules(
    hex: Hex,
    biome: &Biome,
    path: &Path,
    feature: &Feature,
    equipment: &PlayerEquipment,
) -> TileEvaluation {
    let mut evaluation = TileEvaluation {
        hex,
        layer_results: Vec::new(),
        final_result: TileRuleResult::default(),
    };
    
    // LAYER 1: BIOME (Lowest Priority)
    let biome_result = evaluate_biome_rules(biome, equipment);
    evaluation.layer_results.push(LayerEvaluation {
        layer_type: "Biome".to_string(),
        rules_applied: biome_result.rules_applied.clone(),
        result: biome_result.clone(),
    });
    evaluation.final_result = biome_result;
    
    // LAYER 2: PATH (Middle Priority - can override biome penalties)
    if path.path_type != PathType::None {
        let path_result = evaluate_path_rules(path, &evaluation.final_result, equipment);
        evaluation.layer_results.push(LayerEvaluation {
            layer_type: "Path".to_string(),
            rules_applied: path_result.rules_applied.clone(),
            result: path_result.clone(),
        });
        
        // Path can override biome movement penalties
        if evaluation.final_result.movement_modifier < 0 && path_result.movement_modifier > 0 {
            evaluation.final_result.movement_modifier = path.get_movement_modifier(evaluation.final_result.movement_modifier);
            evaluation.final_result.rules_applied.push("Path overrides biome movement penalty".to_string());
        }
    }
    
    // LAYER 3: FEATURE (Can add interactions but doesn't override movement)
    if feature.feature_type != FeatureType::None {
        let feature_result = evaluate_feature_rules(feature, equipment);
        evaluation.layer_results.push(LayerEvaluation {
            layer_type: "Feature".to_string(),
            rules_applied: feature_result.rules_applied.clone(),
            result: feature_result.clone(),
        });
        
        // Features add interactions but don't change movement
        evaluation.final_result.available_interactions.extend(feature_result.available_interactions);
    }
    
    // LAYER 4: PLAYER EQUIPMENT (Highest Priority - can override everything)
    let equipment_result = evaluate_equipment_overrides(equipment, &evaluation.final_result);
    if !equipment_result.rules_applied.is_empty() {
        evaluation.layer_results.push(LayerEvaluation {
            layer_type: "Equipment".to_string(), 
            rules_applied: equipment_result.rules_applied.clone(),
            result: equipment_result.clone(),
        });
        
        // Equipment can prevent terrain damage
        if !equipment_result.terrain_damage.is_empty() {
            evaluation.final_result.terrain_damage.retain(|damage| {
                !equipment_result.damage_prevented.contains(&damage.damage_type)
            });
        }
        
        // Equipment can provide movement bonuses
        evaluation.final_result.movement_modifier += equipment_result.movement_modifier;
    }
    
    evaluation
}

// Individual layer evaluation functions

fn evaluate_biome_rules(biome: &Biome, equipment: &PlayerEquipment) -> TileRuleResult {
    let mut result = TileRuleResult::default();
    
    match biome.biome_type {
        BiomeType::Mountain => {
            if biome.variant == "jagged_rock" {
                result.movement_modifier = -1;
                result.rules_applied.push("Jagged rock: -1 movement".to_string());
                
                if !equipment.has_shoes() {
                    result.terrain_damage.push(TerrainDamage {
                        damage_type: "health".to_string(),
                        damage_amount: 1,
                        prevented_by_equipment: vec!["shoes".to_string()],
                    });
                    result.rules_applied.push("Jagged rock: -1 health without shoes".to_string());
                }
            } else {
                // Regular mountains are impassable
                result.is_impassable = true;
                result.rules_applied.push("Mountain: impassable".to_string());
            }
        },
        BiomeType::Ocean => {
            result.is_impassable = true;
            result.rules_applied.push("Ocean: impassable".to_string());
        },
        BiomeType::Snow => {
            result.movement_modifier = -1;
            result.rules_applied.push("Snow: -1 movement".to_string());
        },
        BiomeType::Swamp => {
            result.movement_modifier = -1;
            result.rules_applied.push("Swamp: -1 movement".to_string());
        },
        _ => {
            result.rules_applied.push(format!("{:?}: no movement penalty", biome.biome_type));
        }
    }
    
    result.movement_cost = 1; // Base movement cost
    result
}

fn evaluate_path_rules(path: &Path, base_result: &TileRuleResult, equipment: &PlayerEquipment) -> TileRuleResult {
    let mut result = TileRuleResult::default();
    
    if path.path_type != PathType::None {
        // Path rule: +1 modifier on tiles with negative movement
        if base_result.movement_modifier < 0 {
            result.movement_modifier = path.speed_modifier;
            result.rules_applied.push(format!("{:?} path cancels terrain penalty", path.path_type));
        }
        
        // Path connections for pathfinding
        result.available_connections = path.connections.clone();
        result.rules_applied.push(format!("{:?} {:?} path provides connections", path.material, path.path_type));
    }
    
    result
}

fn evaluate_feature_rules(feature: &Feature, equipment: &PlayerEquipment) -> TileRuleResult {
    let mut result = TileRuleResult::default();
    
    match feature.feature_type {
        FeatureType::Tavern => {
            result.available_interactions.push("Rest".to_string());
            result.available_interactions.push("Buy Food".to_string());
            result.rules_applied.push("Tavern: rest and trade available".to_string());
        },
        FeatureType::Dungeon => {
            result.available_interactions.push("Explore".to_string());
            result.rules_applied.push("Dungeon: exploration available".to_string());
        },
        FeatureType::Village => {
            result.available_interactions.push("Rest".to_string());
            result.available_interactions.push("Trade".to_string());
            result.rules_applied.push("Village: rest and trade available".to_string());
        },
        _ => {}
    }
    
    result
}

fn evaluate_equipment_overrides(equipment: &PlayerEquipment, base_result: &TileRuleResult) -> TileRuleResult {
    let mut result = TileRuleResult::default();
    
    // Equipment can prevent terrain damage
    for damage in &base_result.terrain_damage {
        if equipment.prevents_terrain_damage(&damage.damage_type) {
            result.damage_prevented.push(damage.damage_type.clone());
            result.rules_applied.push(format!("Equipment prevents {} damage", damage.damage_type));
        }
    }
    
    // Equipment can provide movement bonuses
    if equipment.equipped_items.contains_key(&EquipmentSlot::Feet) {
        result.movement_modifier = 0; // Shoes can negate some penalties
        result.rules_applied.push("Shoes provide stable footing".to_string());
    }
    
    result
}

// Supporting types for the layer cake evaluation

#[derive(Debug, Clone)]
pub struct TileEvaluation {
    pub hex: Hex,
    pub layer_results: Vec<LayerEvaluation>,
    pub final_result: TileRuleResult,
}

#[derive(Debug, Clone)]
pub struct LayerEvaluation {
    pub layer_type: String,
    pub rules_applied: Vec<String>,
    pub result: TileRuleResult,
}

#[derive(Debug, Clone, Default)]
pub struct TileRuleResult {
    pub is_impassable: bool,
    pub movement_modifier: i32,
    pub movement_cost: u32,
    pub terrain_damage: Vec<TerrainDamage>,
    pub available_interactions: Vec<String>,
    pub available_connections: Vec<Hex>,
    pub rules_applied: Vec<String>,
    pub damage_prevented: Vec<String>,
}

// Events for the movement system

#[derive(Event)]
pub struct MovementRequestEvent {
    pub player_entity: Entity,
    pub target_hex: Hex,
}

#[derive(Event)]
pub struct MovementResultEvent {
    pub player_entity: Entity,
    pub result: MovementResult,
    pub message: String,
}

/// Example scenario system - demonstrates the layer cake system with the exact example given
pub fn example_scenario_system(
    mut commands: Commands,
    hex_layout: Res<crate::world::HexWorldLayout>,
) {
    // Create the exact scenario: player on jagged rock with wood path, surrounded by impassable mountains
    let center_hex = Hex::ZERO;
    
    // Spawn jagged rock tile with wood path
    commands.spawn(LayerCakeHexTileBundle {
        hex_tile: HexTile::new(center_hex, &hex_layout.layout),
        biome: Biome {
            biome_type: BiomeType::Mountain,
            variant: "jagged_rock".to_string(),
            elevation: 100.0,
            moisture: 0.2,
            temperature: 0.3,
            adjacency_valid: true,
        },
        path: Path {
            path_type: PathType::Road,
            material: PathMaterial::Wood,
            connections: vec![Hex::new(1, 0)], // Connects to east
            speed_modifier: 1, // +1 to cancel the -1 from jagged rock
            quality: 0.8,
            overlay_asset: None,
        },
        feature: Feature {
            feature_type: FeatureType::None,
            name: "Rocky Path".to_string(),
            interaction_type: InteractionType::None,
            requirements: vec![],
            rewards: vec![],
            feature_asset: None,
        },
        // ... other components with defaults
        ..LayerCakeHexTileBundle::new(center_hex, &hex_layout.layout, BiomeType::Mountain)
    });
    
    // Spawn connecting wood path tile
    let path_hex = Hex::new(1, 0);
    commands.spawn(LayerCakeHexTileBundle {
        hex_tile: HexTile::new(path_hex, &hex_layout.layout),
        biome: Biome {
            biome_type: BiomeType::Mountain,
            variant: "jagged_rock".to_string(),
            elevation: 100.0,
            moisture: 0.2,
            temperature: 0.3,
            adjacency_valid: true,
        },
        path: Path {
            path_type: PathType::Road,
            material: PathMaterial::Wood,
            connections: vec![center_hex], // Connects back to center
            speed_modifier: 1,
            quality: 0.8,
            overlay_asset: None,
        },
        ..LayerCakeHexTileBundle::new(path_hex, &hex_layout.layout, BiomeType::Mountain)
    });
    
    // Surround with impassable mountains
    for neighbor in center_hex.all_neighbors() {
        if neighbor != path_hex {
            commands.spawn(LayerCakeHexTileBundle {
                hex_tile: HexTile::new(neighbor, &hex_layout.layout),
                biome: Biome {
                    biome_type: BiomeType::Mountain,
                    variant: "impassable".to_string(), // NOT jagged rock, so impassable
                    elevation: 200.0,
                    moisture: 0.1,
                    temperature: 0.2,
                    adjacency_valid: true,
                },
                path: Path {
                    path_type: PathType::None,
                    material: PathMaterial::None,
                    connections: vec![],
                    speed_modifier: 0,
                    quality: 0.0,
                    overlay_asset: None,
                },
                ..LayerCakeHexTileBundle::new(neighbor, &hex_layout.layout, BiomeType::Mountain)
            });
        }
    }
    
    // Spawn player with shoes to demonstrate equipment override
    commands.spawn(PlayerBundle::new_with_shoes(
        "player1".to_string(),
        center_hex,
        &hex_layout.layout,
    ));
    
    info!("Example scenario created: Player on jagged rock with wood path, surrounded by impassable mountains");
    info!("Layer cake evaluation will show:");
    info!("  Biome: Jagged rock (-1 movement, -1 health without shoes)");
    info!("  Path: Wood road (+1 movement modifier, cancels biome penalty)");
    info!("  Equipment: Shoes prevent health damage");
    info!("  Final result: Normal movement cost, no health damage, can only move along wood path");
}

/// Plugin to add the movement validation system
pub struct MovementValidationPlugin;

impl Plugin for MovementValidationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<MovementRequestEvent>()
            .add_event::<MovementResultEvent>()
            .add_systems(Update, movement_validation_system)
            .add_systems(Startup, example_scenario_system);
    }
}
