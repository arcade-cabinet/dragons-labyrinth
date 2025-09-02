use bevy::prelude::*;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use std::collections::HashMap;
use crate::world::components::tiles::{BiomeType, HexCoord};
use crate::world::state::WorldState;

#[derive(Component, Debug, Clone)]
pub struct RegionalMilestone {
    pub milestone_type: MilestoneType,
    pub level_requirement: u32,
    pub narrative_weight: f32,
    pub spawned: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MilestoneType {
    Village,
    Inn,
    Dungeon,
    Shrine,
    QuestGiver,
    Merchant,
    Transition,
}

#[derive(Resource, Debug)]
pub struct RegionalProgression {
    pub current_band: u32,
    pub player_level: u32,
    pub regions_generated: HashMap<u32, RegionData>,
    pub milestone_tracker: Vec<RegionalMilestone>,
    pub transition_buffer: f32, // Distance from band edge to start showing transition biomes
}

#[derive(Debug, Clone)]
pub struct RegionData {
    pub band: u32,
    pub name: String,
    pub biome_composition: HashMap<BiomeType, f32>, // Biome type -> probability
    pub emotional_arc: EmotionalState,
    pub weather_intensity: f32,
    pub corruption_level: f32,
    pub milestone_density: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EmotionalState {
    Peace,      // Bands 1-20
    Unease,     // Bands 21-40
    Dread,      // Bands 41-60
    Terror,     // Bands 61-120
    Void,       // Bands 121+
}

impl Default for RegionalProgression {
    fn default() -> Self {
        Self {
            current_band: 1,
            player_level: 1,
            regions_generated: HashMap::new(),
            milestone_tracker: Vec::new(),
            transition_buffer: 5.0,
        }
    }
}

pub fn generate_dynamic_region(
    band: u32,
    seed: u64,
) -> RegionData {
    let mut rng = StdRng::seed_from_u64(seed + band as u64);
    
    let emotional_state = match band {
        1..=20 => EmotionalState::Peace,
        21..=40 => EmotionalState::Unease,
        41..=60 => EmotionalState::Dread,
        61..=120 => EmotionalState::Terror,
        _ => EmotionalState::Void,
    };
    
    let name = generate_region_name(&emotional_state, &mut rng);
    let biome_composition = generate_biome_composition(&emotional_state, &mut rng);
    
    RegionData {
        band,
        name,
        biome_composition,
        emotional_arc: emotional_state.clone(),
        weather_intensity: calculate_weather_intensity(&emotional_state),
        corruption_level: calculate_corruption_level(band),
        milestone_density: calculate_milestone_density(&emotional_state),
    }
}

fn generate_region_name(emotional_state: &EmotionalState, rng: &mut StdRng) -> String {
    let prefixes = match emotional_state {
        EmotionalState::Peace => vec!["Green", "Golden", "Gentle", "Blessed", "Fair"],
        EmotionalState::Unease => vec!["Grey", "Troubled", "Restless", "Shadowed", "Weary"],
        EmotionalState::Dread => vec!["Dark", "Cursed", "Blighted", "Forsaken", "Withered"],
        EmotionalState::Terror => vec!["Burning", "Screaming", "Broken", "Mad", "Dying"],
        EmotionalState::Void => vec!["Void", "Lost", "Forgotten", "Null", "Empty"],
    };
    
    let suffixes = match emotional_state {
        EmotionalState::Peace => vec!["Meadows", "Hills", "Vale", "Fields", "Grove"],
        EmotionalState::Unease => vec!["Moors", "Marshes", "Reaches", "Borderlands", "Wastes"],
        EmotionalState::Dread => vec!["Barrens", "Desolation", "Ruins", "Wastes", "Depths"],
        EmotionalState::Terror => vec!["Hellscape", "Nightmare", "Ashes", "Crater", "Inferno"],
        EmotionalState::Void => vec!["Nothing", "Expanse", "Rift", "Absence", "Silence"],
    };
    
    let prefix = prefixes[rng.gen_range(0..prefixes.len())];
    let suffix = suffixes[rng.gen_range(0..suffixes.len())];
    
    format!("{} {}", prefix, suffix)
}

fn generate_biome_composition(emotional_state: &EmotionalState, rng: &mut StdRng) -> HashMap<BiomeType, f32> {
    let mut composition = HashMap::new();
    
    match emotional_state {
        EmotionalState::Peace => {
            composition.insert(BiomeType::Grassland, 0.4);
            composition.insert(BiomeType::Forest, 0.3);
            composition.insert(BiomeType::Water, 0.2);
            composition.insert(BiomeType::Mountain, 0.1);
        },
        EmotionalState::Unease => {
            composition.insert(BiomeType::Forest, 0.3);
            composition.insert(BiomeType::Swamp, 0.25);
            composition.insert(BiomeType::Mountain, 0.2);
            composition.insert(BiomeType::Desert, 0.15);
            composition.insert(BiomeType::Corrupted(Box::new(BiomeType::Grassland)), 0.1);
        },
        EmotionalState::Dread => {
            composition.insert(BiomeType::Desert, 0.3);
            composition.insert(BiomeType::Mountain, 0.25);
            composition.insert(BiomeType::Corrupted(Box::new(BiomeType::Forest)), 0.2);
            composition.insert(BiomeType::Lava, 0.15);
            composition.insert(BiomeType::Swamp, 0.1);
        },
        EmotionalState::Terror => {
            composition.insert(BiomeType::Lava, 0.4);
            composition.insert(BiomeType::Corrupted(Box::new(BiomeType::Desert)), 0.3);
            composition.insert(BiomeType::Void, 0.2);
            composition.insert(BiomeType::Mountain, 0.1);
        },
        EmotionalState::Void => {
            composition.insert(BiomeType::Void, 0.6);
            composition.insert(BiomeType::Corrupted(Box::new(BiomeType::Void)), 0.4);
        },
    }
    
    composition
}

fn calculate_weather_intensity(emotional_state: &EmotionalState) -> f32 {
    match emotional_state {
        EmotionalState::Peace => 0.1,
        EmotionalState::Unease => 0.3,
        EmotionalState::Dread => 0.5,
        EmotionalState::Terror => 0.8,
        EmotionalState::Void => 1.0,
    }
}

fn calculate_corruption_level(band: u32) -> f32 {
    (band as f32 / 120.0).clamp(0.0, 1.0)
}

fn calculate_milestone_density(emotional_state: &EmotionalState) -> f32 {
    match emotional_state {
        EmotionalState::Peace => 0.8,      // Many settlements and friendly encounters
        EmotionalState::Unease => 0.6,     // Fewer, more scattered
        EmotionalState::Dread => 0.4,      // Rare safe havens
        EmotionalState::Terror => 0.2,     // Almost nothing survives
        EmotionalState::Void => 0.05,      // Reality itself is breaking down
    }
}

pub fn should_spawn_milestone(
    player_level: u32,
    distance_from_center: f32,
    region_data: &RegionData,
    rng: &mut StdRng,
) -> Option<MilestoneType> {
    let base_chance = region_data.milestone_density;
    let level_modifier = (player_level as f32 / 10.0).clamp(0.1, 2.0);
    let distance_modifier = 1.0 - (distance_from_center / 100.0).clamp(0.0, 0.8);
    
    let spawn_chance = base_chance * level_modifier * distance_modifier;
    
    if rng.gen::<f32>() < spawn_chance {
        let milestone_types = get_available_milestones(&region_data.emotional_arc);
        if !milestone_types.is_empty() {
            return Some(milestone_types[rng.gen_range(0..milestone_types.len())].clone());
        }
    }
    
    None
}

fn get_available_milestones(emotional_state: &EmotionalState) -> Vec<MilestoneType> {
    match emotional_state {
        EmotionalState::Peace => vec![
            MilestoneType::Village,
            MilestoneType::Inn,
            MilestoneType::Shrine,
            MilestoneType::QuestGiver,
            MilestoneType::Merchant,
        ],
        EmotionalState::Unease => vec![
            MilestoneType::Village,
            MilestoneType::Inn,
            MilestoneType::Dungeon,
            MilestoneType::QuestGiver,
        ],
        EmotionalState::Dread => vec![
            MilestoneType::Dungeon,
            MilestoneType::Shrine,
            MilestoneType::QuestGiver,
        ],
        EmotionalState::Terror => vec![
            MilestoneType::Dungeon,
            MilestoneType::Shrine,
        ],
        EmotionalState::Void => vec![
            MilestoneType::Transition,
        ],
    }
}

pub fn update_regional_progression(
    mut progression: ResMut<RegionalProgression>,
    world_state: Res<WorldState>,
    // TODO: Add player position component query
) {
    // This system will track player movement and update regional progression
    // Generate new regions as needed based on player advancement
}

pub fn generate_transitional_biomes(
    current_region: &RegionData,
    next_region: &RegionData,
    transition_progress: f32, // 0.0 = current region, 1.0 = next region
) -> HashMap<BiomeType, f32> {
    let mut transitional_composition = HashMap::new();
    
    for (biome, current_weight) in &current_region.biome_composition {
        let next_weight = next_region.biome_composition.get(biome).unwrap_or(&0.0);
        let interpolated_weight = current_weight * (1.0 - transition_progress) + next_weight * transition_progress;
        
        if interpolated_weight > 0.05 { // Only include biomes with significant presence
            transitional_composition.insert(biome.clone(), interpolated_weight);
        }
    }
    
    // Add biomes from next region that aren't in current
    for (biome, next_weight) in &next_region.biome_composition {
        if !current_region.biome_composition.contains_key(biome) {
            let interpolated_weight = next_weight * transition_progress;
            if interpolated_weight > 0.05 {
                transitional_composition.insert(biome.clone(), interpolated_weight);
            }
        }
    }
    
    transitional_composition
}