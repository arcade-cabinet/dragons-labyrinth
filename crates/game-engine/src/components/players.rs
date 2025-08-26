//! Player ECS Components - Top Priority Layer with Movement Rules
//!
//! Player layer has highest priority in the layer cake system and defines all movement,
//! day/night cycles, and interaction rules that override lower layers.

use bevy::prelude::*;
use hexx::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Player component - highest priority in layer cake system
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct Player {
    pub player_id: String,
    pub current_hex: Hex,
}

/// Player day/night cycle system with seasonal effects
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct DayNightCycle {
    /// Current remaining turns in day cycle before fatigue
    pub remaining_turns: u32,
    /// Maximum turns per day cycle (affected by seasons)
    pub max_turns_per_day: u32,
    /// Base turns per day (50 as per rules)
    pub base_turns_per_day: u32,
    /// Current season affects day cycle length
    pub current_season: Season,
    /// Whether player needs to rest (fatigue)
    pub must_rest: bool,
    /// Time since last rest
    pub time_since_rest: f32,
}

impl DayNightCycle {
    pub fn new() -> Self {
        let base_turns = 50;
        Self {
            remaining_turns: base_turns,
            max_turns_per_day: base_turns,
            base_turns_per_day: base_turns,
            current_season: Season::Spring,
            must_rest: false,
            time_since_rest: 0.0,
        }
    }
    
    /// Seasonal rule: Summer increases day cycle, Winter decreases
    pub fn update_for_season(&mut self, season: Season) {
        self.current_season = season;
        self.max_turns_per_day = match season {
            Season::Summer => (self.base_turns_per_day as f32 * 1.2) as u32, // +20% in summer
            Season::Winter => (self.base_turns_per_day as f32 * 0.8) as u32, // -20% in winter  
            Season::Spring | Season::Fall => self.base_turns_per_day,
        };
        
        // Adjust remaining turns if current exceeds new max
        self.remaining_turns = self.remaining_turns.min(self.max_turns_per_day);
    }
    
    /// Use turns for movement and check fatigue
    pub fn use_turns(&mut self, turns_used: u32) -> bool {
        if self.remaining_turns >= turns_used {
            self.remaining_turns -= turns_used;
            if self.remaining_turns == 0 {
                self.must_rest = true;
            }
            true
        } else {
            false // Not enough turns
        }
    }
    
    /// Rest to restore day cycle
    pub fn rest(&mut self) {
        self.remaining_turns = self.max_turns_per_day;
        self.must_rest = false;
        self.time_since_rest = 0.0;
    }
}

/// Player movement capabilities and rules
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct PlayerMovement {
    /// Current movement type
    pub movement_type: MovementType,
    /// Movement points available this turn
    pub movement_points: u32,
    /// Base movement speed
    pub base_speed: u32,
    /// Equipment affecting movement
    pub equipment_modifiers: HashMap<String, i32>,
}

impl PlayerMovement {
    pub fn new() -> Self {
        Self {
            movement_type: MovementType::Walk,
            movement_points: 1,
            base_speed: 1,
            equipment_modifiers: HashMap::new(),
        }
    }
    
    /// Movement rule: WALK 1 space, or RUN 2-3 spaces with decreased encounter chance
    pub fn set_movement_type(&mut self, movement_type: MovementType) {
        self.movement_type = movement_type;
        self.movement_points = match movement_type {
            MovementType::Walk => 1,
            MovementType::Run => 3, // Can move up to 3 spaces when running
        };
    }
    
    /// Check if player can move to target hex given tile rules
    pub fn can_move_to_hex(&self, current_hex: Hex, target_hex: Hex, world_tiles: &TileAccessibilityMap) -> MovementResult {
        let distance = current_hex.distance_to(target_hex) as u32;
        
        // Check if we have enough movement points
        if distance > self.movement_points {
            return MovementResult::InsufficientMovement;
        }
        
        // Get path to target and check each tile
        let path = current_hex.line_to(target_hex);
        
        for hex in path {
            if let Some(tile_info) = world_tiles.tiles.get(&hex) {
                // Priority system: Player > Path > Biome rules
                let final_modifier = self.calculate_final_movement_modifier(tile_info);
                
                if tile_info.is_impassable {
                    return MovementResult::Impassable(hex);
                }
                
                if final_modifier < -2 {
                    return MovementResult::TooSlow(hex);
                }
            }
        }
        
        // Check for running through intermediary spaces (decreased encounter chance)
        if self.movement_type == MovementType::Run && distance > 1 {
            return MovementResult::Success { 
                path,
                decreased_encounter_chance: true,
                turns_used: 1, // Running uses same turns as walking
            };
        }
        
        MovementResult::Success {
            path,
            decreased_encounter_chance: false, 
            turns_used: distance,
        }
    }
    
    /// Calculate final movement modifier using layer cake priority
    fn calculate_final_movement_modifier(&self, tile_info: &TileInfo) -> i32 {
        let mut modifier = 0;
        
        // Start with biome modifier (lowest priority)
        modifier += tile_info.biome_movement_modifier;
        
        // Path layer can override biome penalties (middle priority)
        if let Some(path_modifier) = tile_info.path_movement_modifier {
            modifier = tile_info.path.as_ref()
                .map(|path| path.get_movement_modifier(tile_info.biome_movement_modifier))
                .unwrap_or(modifier);
        }
        
        // Player equipment can override everything (highest priority)
        for (_, equipment_modifier) in &self.equipment_modifiers {
            modifier += equipment_modifier;
        }
        
        modifier
    }
}

/// Player equipment with specific interaction rules
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct PlayerEquipment {
    pub equipped_items: HashMap<EquipmentSlot, String>,
    pub inventory: Vec<String>,
}

impl PlayerEquipment {
    /// Equipment rule: shoes prevent health loss on jagged rock
    pub fn prevents_terrain_damage(&self, terrain_type: &str) -> bool {
        match terrain_type {
            "jagged_rock" => self.has_shoes(),
            "lava" => self.has_heat_protection(),
            "ice" => self.has_cold_protection(),
            _ => false,
        }
    }
    
    pub fn has_shoes(&self) -> bool {
        self.equipped_items.contains_key(&EquipmentSlot::Feet)
    }
    
    pub fn has_heat_protection(&self) -> bool {
        self.equipped_items.get(&EquipmentSlot::Torso)
            .map(|armor| armor.contains("heat_resistant"))
            .unwrap_or(false)
    }
    
    pub fn has_cold_protection(&self) -> bool {
        self.equipped_items.get(&EquipmentSlot::Torso)
            .map(|armor| armor.contains("warm") || armor.contains("fur"))
            .unwrap_or(false)
    }
}

// Supporting types and systems

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect, Serialize, Deserialize)]
pub enum Season {
    Spring,
    Summer,
    Fall,  
    Winter,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect, Serialize, Deserialize)]
pub enum MovementType {
    Walk, // 1 space
    Run,  // 2-3 spaces with decreased encounter chance
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Reflect, Serialize, Deserialize)]
pub enum EquipmentSlot {
    Head,
    Torso,
    Legs,
    Feet,
    MainHand,
    OffHand,
}

#[derive(Debug, Clone)]
pub enum MovementResult {
    Success {
        path: Vec<Hex>,
        decreased_encounter_chance: bool,
        turns_used: u32,
    },
    InsufficientMovement,
    Impassable(Hex), // Which hex blocked the path
    TooSlow(Hex),    // Movement penalty too severe
}

/// World tile accessibility map for movement validation
#[derive(Resource)]
pub struct TileAccessibilityMap {
    pub tiles: HashMap<Hex, TileInfo>,
}

#[derive(Debug, Clone)]
pub struct TileInfo {
    pub hex: Hex,
    pub is_impassable: bool,
    pub biome_movement_modifier: i32,
    pub path_movement_modifier: Option<i32>,
    pub requires_equipment: Vec<String>,
    pub terrain_damage: Option<TerrainDamage>,
    pub path: Option<super::hex_tiles::Path>,
}

#[derive(Debug, Clone)]
pub struct TerrainDamage {
    pub damage_type: String, // "health", "stamina", "equipment"
    pub damage_amount: i32,
    pub prevented_by_equipment: Vec<String>,
}

/// Bundle for spawning player with all movement systems
#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub day_night_cycle: DayNightCycle,
    pub movement: PlayerMovement,
    pub equipment: PlayerEquipment,
    
    // Bevy components
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}

impl PlayerBundle {
    pub fn new(player_id: String, starting_hex: Hex, hex_layout: &HexLayout) -> Self {
        let world_pos = hex_layout.hex_to_world_pos(starting_hex).extend(1.0); // Slightly above tiles
        
        Self {
            player: Player {
                player_id,
                current_hex: starting_hex,
            },
            day_night_cycle: DayNightCycle::new(),
            movement: PlayerMovement::new(),
            equipment: PlayerEquipment {
                equipped_items: HashMap::new(),
                inventory: Vec::new(),
            },
            transform: Transform::from_translation(world_pos),
            global_transform: GlobalTransform::default(),
            visibility: Visibility::default(),
            inherited_visibility: InheritedVisibility::default(),
            view_visibility: ViewVisibility::default(),
        }
    }
    
    /// Example: Create player with shoes (prevents jagged rock damage)
    pub fn new_with_shoes(player_id: String, starting_hex: Hex, hex_layout: &HexLayout) -> Self {
        let mut bundle = Self::new(player_id, starting_hex, hex_layout);
        bundle.equipment.equipped_items.insert(
            EquipmentSlot::Feet, 
            "leather_boots".to_string()
        );
        bundle
    }
}

// Example biome implementations

impl super::hex_tiles::Biome {
    /// Biome rule: jagged rock gives -1 movement, -1 health unless wearing shoes
    pub fn get_movement_modifier(&self) -> i32 {
        match self.biome_type {
            super::hex_tiles::BiomeType::Mountain if self.variant == "jagged_rock" => -1,
            super::hex_tiles::BiomeType::Swamp => -1,
            super::hex_tiles::BiomeType::Desert => -1, // Hot sand is slow
            super::hex_tiles::BiomeType::Snow => -1,   // Deep snow is slow
            _ => 0,
        }
    }
    
    pub fn get_terrain_damage(&self) -> Option<TerrainDamage> {
        match self.biome_type {
            super::hex_tiles::BiomeType::Mountain if self.variant == "jagged_rock" => {
                Some(TerrainDamage {
                    damage_type: "health".to_string(),
                    damage_amount: 1,
                    prevented_by_equipment: vec!["shoes".to_string(), "boots".to_string()],
                })
            },
            super::hex_tiles::BiomeType::Lava => {
                Some(TerrainDamage {
                    damage_type: "health".to_string(),
                    damage_amount: 5,
                    prevented_by_equipment: vec!["heat_resistant_armor".to_string()],
                })
            },
            _ => None,
        }
    }
    
    /// Some biomes are completely impassable
    pub fn is_impassable(&self) -> bool {
        match self.biome_type {
            super::hex_tiles::BiomeType::Mountain if self.variant != "jagged_rock" => true,
            super::hex_tiles::BiomeType::Ocean => true,
            _ => false,
        }
    }
}

impl Default for TileAccessibilityMap {
    fn default() -> Self {
        Self {
            tiles: HashMap::new(),
        }
    }
}
