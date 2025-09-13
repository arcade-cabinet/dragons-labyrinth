use bevy_ecs::prelude::*;
use crate::BiomeType;
use crate::world::companions::EmotionalResponse;
use serde::{Deserialize, Serialize};

#[derive(Component, Debug, Clone)]
pub struct DreadSource {
    pub intensity: f32,
    pub range: f32,
    pub source_type: String,
    pub is_permanent: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DreadPhase {
    Peace,      // 0-20: World feels safe, companions relaxed
    Unease,     // 21-40: Something feels off, subtle wrongness
    Dread,      // 41-60: Horror manifesting, visible corruption
    Terror,     // 61-80: Reality breaking down, companion panic
    Void,       // 81-100: Void consuming everything, alien geometry
    BeyondVoid, // 100+: Complete reality breakdown, final encounters
}

impl DreadPhase {
    pub fn from_level(level: f32) -> Self {
        match level as i32 {
            0..=20 => DreadPhase::Peace,
            21..=40 => DreadPhase::Unease,
            41..=60 => DreadPhase::Dread,
            61..=80 => DreadPhase::Terror,
            81..=100 => DreadPhase::Void,
            _ => DreadPhase::BeyondVoid,
        }
    }
    
    pub fn get_world_corruption_multiplier(&self) -> f32 {
        match self {
            DreadPhase::Peace => 0.0,
            DreadPhase::Unease => 0.1,
            DreadPhase::Dread => 0.3,
            DreadPhase::Terror => 0.6,
            DreadPhase::Void => 1.0,
            DreadPhase::BeyondVoid => 2.0,
        }
    }
    
    pub fn get_companion_stress_multiplier(&self) -> f32 {
        match self {
            DreadPhase::Peace => 0.5,
            DreadPhase::Unease => 1.0,
            DreadPhase::Dread => 1.5,
            DreadPhase::Terror => 2.5,
            DreadPhase::Void => 4.0,
            DreadPhase::BeyondVoid => 6.0,
        }
    }
    
    pub fn get_encounter_spawn_rate(&self) -> f32 {
        match self {
            DreadPhase::Peace => 0.1,
            DreadPhase::Unease => 0.2,
            DreadPhase::Dread => 0.4,
            DreadPhase::Terror => 0.7,
            DreadPhase::Void => 1.0,
            DreadPhase::BeyondVoid => 1.5,
        }
    }
    
    pub fn allows_boss_encounters(&self) -> bool {
        matches!(self, DreadPhase::Terror | DreadPhase::Void | DreadPhase::BeyondVoid)
    }
    
    pub fn get_labyrinth_complexity(&self) -> u32 {
        match self {
            DreadPhase::Peace => 1,
            DreadPhase::Unease => 2,
            DreadPhase::Dread => 3,
            DreadPhase::Terror => 4,
            DreadPhase::Void => 5,
            DreadPhase::BeyondVoid => 6,
        }
    }
}

#[derive(Component, Debug)]
pub struct CorruptionNode {
    pub corruption_level: f32,
    pub spread_rate: f32,
    pub affected_radius: f32,
    pub corruption_type: CorruptionType,
    pub is_spreading: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CorruptionType {
    VoidTaint,      // Reality erosion
    BloodCurse,     // Organic corruption
    ShadowBlight,   // Darkness spreading
    TimeFracture,   // Temporal distortion
    MindRot,        // Psychological corruption
    ElementalFlux,  // Magical instability
}

impl CorruptionType {
    pub fn get_biome_effect(&self, base_biome: &BiomeType) -> BiomeType {
        match self {
            CorruptionType::VoidTaint => BiomeType::Void,
            CorruptionType::BloodCurse => {
                match base_biome {
                    BiomeType::Grassland => BiomeType::CorruptedGrassland,
                    BiomeType::Forest => BiomeType::CorruptedForest,
                    BiomeType::Mountain => BiomeType::CorruptedMountain,
                    BiomeType::Desert => BiomeType::CorruptedDesert,
                    BiomeType::Swamp => BiomeType::CorruptedSwamp,
                    BiomeType::Water => BiomeType::CorruptedWater,
                    BiomeType::Snow => BiomeType::CorruptedSnow,
                    _ => BiomeType::CorruptedGrassland, // Default fallback
                }
            }
            CorruptionType::ShadowBlight => {
                match base_biome {
                    BiomeType::Grassland => BiomeType::CorruptedGrassland,
                    BiomeType::Forest => BiomeType::CorruptedForest,
                    BiomeType::Mountain => BiomeType::CorruptedMountain,
                    BiomeType::Desert => BiomeType::CorruptedDesert,
                    BiomeType::Swamp => BiomeType::CorruptedSwamp,
                    BiomeType::Water => BiomeType::CorruptedWater,
                    BiomeType::Snow => BiomeType::CorruptedSnow,
                    _ => BiomeType::CorruptedForest, // Default to corrupted forest for shadow
                }
            }
            CorruptionType::TimeFracture => BiomeType::Void,
            CorruptionType::MindRot => {
                match base_biome {
                    BiomeType::Grassland => BiomeType::VoidGrassland,
                    BiomeType::Forest => BiomeType::VoidForest,
                    BiomeType::Mountain => BiomeType::VoidMountain,
                    BiomeType::Desert => BiomeType::VoidDesert,
                    BiomeType::Swamp => BiomeType::VoidSwamp,
                    BiomeType::Water => BiomeType::VoidWater,
                    BiomeType::Snow => BiomeType::VoidSnow,
                    _ => BiomeType::Void, // Default to pure void for mind rot
                }
            }
            CorruptionType::ElementalFlux => BiomeType::Lava,
        }
    }
    
    pub fn get_dread_intensity(&self) -> f32 {
        match self {
            CorruptionType::VoidTaint => 10.0,
            CorruptionType::BloodCurse => 8.0,
            CorruptionType::ShadowBlight => 6.0,
            CorruptionType::TimeFracture => 12.0,
            CorruptionType::MindRot => 15.0,
            CorruptionType::ElementalFlux => 5.0,
        }
    }
    
    pub fn get_companion_effects(&self) -> EmotionalResponse {
        match self {
            CorruptionType::VoidTaint => EmotionalResponse {
                stress_modifier: 20.0,
                trust_modifier: -10.0,
                dialogue_trigger: Some("void_terror".to_string()),
                behavioral_change: Some("panic_flee".to_string()),
            },
            CorruptionType::BloodCurse => EmotionalResponse {
                stress_modifier: 15.0,
                trust_modifier: -5.0,
                dialogue_trigger: Some("blood_revulsion".to_string()),
                behavioral_change: Some("avoid_area".to_string()),
            },
            CorruptionType::MindRot => EmotionalResponse {
                stress_modifier: 25.0,
                trust_modifier: -15.0,
                dialogue_trigger: Some("mind_confusion".to_string()),
                behavioral_change: Some("disoriented".to_string()),
            },
            _ => EmotionalResponse {
                stress_modifier: 10.0,
                trust_modifier: -3.0,
                dialogue_trigger: None,
                behavioral_change: None,
            }
        }
    }
}

#[derive(Component, Debug)]
pub struct DreadVisualization {
    pub particle_system: Option<Entity>,
    pub shader_intensity: f32,
    pub audio_layer: Option<String>,
    pub fog_density: f32,
}

impl DreadVisualization {
    pub fn update_for_phase(&mut self, phase: &DreadPhase) {
        self.shader_intensity = match phase {
            DreadPhase::Peace => 0.0,
            DreadPhase::Unease => 0.1,
            DreadPhase::Dread => 0.3,
            DreadPhase::Terror => 0.6,
            DreadPhase::Void => 0.9,
            DreadPhase::BeyondVoid => 1.0,
        };
        
        self.fog_density = match phase {
            DreadPhase::Peace => 0.0,
            DreadPhase::Unease => 0.05,
            DreadPhase::Dread => 0.15,
            DreadPhase::Terror => 0.3,
            DreadPhase::Void => 0.6,
            DreadPhase::BeyondVoid => 0.9,
        };
        
        self.audio_layer = match phase {
            DreadPhase::Peace => Some("ambient_peace".to_string()),
            DreadPhase::Unease => Some("ambient_unease".to_string()),
            DreadPhase::Dread => Some("ambient_dread".to_string()),
            DreadPhase::Terror => Some("ambient_terror".to_string()),
            DreadPhase::Void => Some("ambient_void".to_string()),
            DreadPhase::BeyondVoid => Some("ambient_beyond".to_string()),
        };
    }
}
