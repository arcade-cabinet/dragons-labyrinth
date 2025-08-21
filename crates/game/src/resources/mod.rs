use bevy::prelude::*;
use std::collections::HashMap;
use crate::components::*;

// Central dread state following design bible narrative arc
#[derive(Resource, Default)]
pub struct DreadState {
    pub current_level: u8,  // 0=Peace, 1=Unease, 2=Dread, 3=Terror, 4=Horror
    pub progression_points: f32,  // Accumulates to trigger next stage
    pub points_needed: f32,       // Points needed for next progression
}

impl DreadState {
    pub fn progress(&mut self, points: f32) {
        self.progression_points += points;
        if self.progression_points >= self.points_needed && self.current_level < 4 {
            self.current_level += 1;
            self.progression_points = 0.0;
            self.points_needed *= 1.5; // Each stage takes longer
            
            // Log narrative progression
            match self.current_level {
                1 => info!("Narrative progression: Peace → Unease"),
                2 => info!("Narrative progression: Unease → Dread"), 
                3 => info!("Narrative progression: Dread → Terror"),
                4 => info!("Narrative progression: Terror → Horror"),
                _ => {}
            }
        }
    }
    
    pub fn get_stage_name(&self) -> &'static str {
        match self.current_level {
            0 => "Peace",
            1 => "Unease", 
            2 => "Dread",
            3 => "Terror",
            4 => "Horror",
            _ => "Unknown"
        }
    }
}

// Hex world state management
#[derive(Resource, Default)]
pub struct HexWorld {
    pub tiles: HashMap<(i32, i32), HexTile>,
    pub player_position: (i32, i32),
    pub corruption_radius: f32,  // Grows with dread level
}

impl HexWorld {
    pub fn get_tile(&self, q: i32, r: i32) -> Option<&HexTile> {
        self.tiles.get(&(q, r))
    }
    
    pub fn set_tile(&mut self, q: i32, r: i32, tile: HexTile) {
        self.tiles.insert((q, r), tile);
    }
    
    pub fn corrupt_area(&mut self, center_q: i32, center_r: i32, radius: f32, dread_level: u8) {
        for ((q, r), tile) in self.tiles.iter_mut() {
            let distance = hex_distance(*q, *r, center_q, center_r);
            if distance <= radius as i32 {
                tile.dread_level = tile.dread_level.max(dread_level);
                // Transform tile types based on dread
                if dread_level >= 3 {
                    tile.tile_type = match tile.tile_type {
                        TileType::Grass => TileType::Corrupted,
                        TileType::Forest => TileType::Corrupted,
                        TileType::Village => TileType::Ruins,
                        _ => tile.tile_type.clone(),
                    };
                }
            }
        }
    }
}

// Narrative state tracking all story elements
#[derive(Resource, Default)]
pub struct NarrativeState {
    pub active_quests: Vec<Quest>,
    pub completed_quests: Vec<Quest>,
    pub companion_states: HashMap<CompanionType, CompanionState>,
    pub key_choices: Vec<String>,  // Track moral choices for endings
    pub dragon_proximity: f32,     // How close the dragon feels (audio cues)
}

impl NarrativeState {
    pub fn make_choice(&mut self, choice: String) {
        self.key_choices.push(choice);
        // Log important narrative choices
        info!("Player choice recorded: {}", self.key_choices.last().unwrap());
    }
    
    pub fn get_available_endings(&self) -> Vec<EndingType> {
        // Determine endings based on choices and companion states
        let mut endings = vec![EndingType::Acceptance]; // Always available
        
        if self.key_choices.iter().any(|c| c.contains("defiance")) {
            endings.push(EndingType::Defiance);
        }
        
        if self.companion_states.values().all(|state| 
            *state != CompanionState::Abandoned && *state != CompanionState::Hostile
        ) {
            endings.push(EndingType::Understanding);
        }
        
        endings
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum EndingType {
    Acceptance,    // Accept the dragon's nature
    Defiance,      // Fight the inevitable 
    Understanding, // Comprehend the true horror
}

// Audio state for proximity horror system
#[derive(Resource, Default)]
pub struct AudioState {
    pub ambient_tracks: HashMap<u8, Handle<AudioSource>>, // Per dread level
    pub dragon_proximity_audio: Vec<Handle<AudioSource>>, // Proximity horror
    pub current_ambient: Option<Handle<AudioSource>>,
}

// Utility function for hex distance calculation
pub fn hex_distance(q1: i32, r1: i32, q2: i32, r2: i32) -> i32 {
    ((q1 - q2).abs() + (q1 + r1 - q2 - r2).abs() + (r1 - r2).abs()) / 2
}