use bevy_ecs::prelude::*;
use crate::{BiomeType, DreadPhase};
use bevy_math::Vec3;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Component, Debug, Clone)]
pub struct Companion {
    pub name: String,
    pub companion_type: String,
    pub stress: f32,
    pub trust: f32,
    pub state: CompanionState,
    pub trauma_level: TraumaLevel,
    pub dialogue_flags: HashMap<String, bool>,
    pub state_changed_this_frame: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CompanionState {
    Loyal,      // High trust, low stress - will follow anywhere
    Content,    // Moderate trust, low stress - reliable companion
    Stable,     // Balanced state - normal behavior
    Nervous,    // Moderate stress - jumpy, needs reassurance
    Wary,       // Low trust - questions decisions
    Distressed, // High stress - panic reactions
    Hostile,    // Very low trust - may abandon or betray
    Broken,     // Extreme trauma - non-functional
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TraumaLevel {
    None,
    Mild,       // Occasional nightmares, slight anxiety
    Moderate,   // Regular panic attacks, avoidance behaviors
    Severe,     // Catatonic episodes, severe dissociation
    Critical,   // Complete breakdown, requires special care
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CompanionType {
    Scholar {
        expertise: Vec<String>,
        research_notes: HashMap<String, String>,
    },
    Warrior {
        combat_style: String,
        weapon_proficiency: Vec<String>,
    },
    Guide {
        known_regions: Vec<String>,
        survival_skills: u32,
    },
    Mystic {
        magic_school: String,
        ritual_knowledge: Vec<String>,
    },
    Merchant {
        trade_connections: Vec<String>,
        negotiation_skill: u32,
    },
    Refugee {
        homeland: String,
        trauma_triggers: Vec<String>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmotionalResponse {
    pub stress_modifier: f32,
    pub trust_modifier: f32,
    pub dialogue_trigger: Option<String>,
    pub behavioral_change: Option<String>,
}

impl Companion {
    pub fn new(name: String, companion_type: String) -> Self {
        Self {
            name,
            companion_type,
            stress: 10.0,
            trust: 50.0,
            state: CompanionState::Stable,
            trauma_level: TraumaLevel::None,
            dialogue_flags: HashMap::new(),
            state_changed_this_frame: false,
        }
    }
    
    pub fn react_to_biome(&mut self, biome_type: &BiomeType) {
        let stress_change = match biome_type {
            BiomeType::Grassland => -0.5,
            BiomeType::Forest => 0.0,
            BiomeType::Mountain => 1.0,
            BiomeType::Desert => 2.0,
            BiomeType::Swamp => 3.0,
            BiomeType::Water => 1.5,
            BiomeType::Lava => 8.0,
            BiomeType::Void => 15.0,
            // Corrupted variants cause high stress
            BiomeType::CorruptedGrassland | 
            BiomeType::CorruptedForest |
            BiomeType::CorruptedMountain |
            BiomeType::CorruptedDesert |
            BiomeType::CorruptedSwamp |
            BiomeType::CorruptedWater |
            BiomeType::CorruptedSnow => 10.0,
            
            // Void variants cause extreme stress
            BiomeType::VoidGrassland |
            BiomeType::VoidForest |
            BiomeType::VoidMountain |
            BiomeType::VoidDesert |
            BiomeType::VoidSwamp |
            BiomeType::VoidWater |
            BiomeType::VoidSnow |
            BiomeType::VoidLava => 20.0,
            
            // Transitional biomes cause mild stress
            BiomeType::ForestGrassland |
            BiomeType::MountainForest |
            BiomeType::DesertMountain |
            BiomeType::SwampWater |
            BiomeType::SnowMountain => 0.5,
            BiomeType::Snow => 1.5,
        };
        
        self.stress += stress_change;
        self.stress = self.stress.clamp(0.0, 100.0);
    }
    
    pub fn react_to_dread_phase(&mut self, dread_phase: &DreadPhase) {
        let (stress_change, trust_change) = match dread_phase {
            DreadPhase::Peace => (-2.0, 1.0),
            DreadPhase::Unease => (1.0, 0.0),
            DreadPhase::Dread => (3.0, -1.0),
            DreadPhase::Terror => (8.0, -3.0),
            DreadPhase::Void => (15.0, -8.0),
            DreadPhase::BeyondVoid => (25.0, -15.0),
        };
        
        self.stress += stress_change;
        self.trust += trust_change;
        
        self.stress = self.stress.clamp(0.0, 100.0);
        self.trust = self.trust.clamp(0.0, 100.0);
    }
    
    pub fn get_dialogue_options(&self) -> Vec<String> {
        let mut options = Vec::new();
        
        match self.state {
            CompanionState::Loyal => {
                options.push("ask_advice".to_string());
                options.push("share_thoughts".to_string());
            }
            CompanionState::Content => {
                options.push("chat".to_string());
                options.push("ask_about_area".to_string());
            }
            CompanionState::Nervous => {
                options.push("reassure".to_string());
                options.push("ask_concerns".to_string());
            }
            CompanionState::Wary => {
                options.push("explain_actions".to_string());
                options.push("build_trust".to_string());
            }
            CompanionState::Distressed => {
                options.push("comfort".to_string());
                options.push("suggest_rest".to_string());
            }
            CompanionState::Hostile => {
                options.push("apologize".to_string());
                options.push("negotiate".to_string());
            }
            CompanionState::Broken => {
                options.push("care_for".to_string());
            }
            _ => {}
        }
        
        options
    }
    
    pub fn process_dialogue_choice(&mut self, choice: &str) -> EmotionalResponse {
        match (choice, &self.state) {
            ("reassure", CompanionState::Nervous) => {
                self.stress -= 5.0;
                self.trust += 2.0;
                EmotionalResponse {
                    stress_modifier: -5.0,
                    trust_modifier: 2.0,
                    dialogue_trigger: Some("reassured_response".to_string()),
                    behavioral_change: None,
                }
            }
            ("comfort", CompanionState::Distressed) => {
                self.stress -= 10.0;
                self.trust += 5.0;
                EmotionalResponse {
                    stress_modifier: -10.0,
                    trust_modifier: 5.0,
                    dialogue_trigger: Some("comforted_response".to_string()),
                    behavioral_change: Some("calmer_behavior".to_string()),
                }
            }
            ("apologize", CompanionState::Hostile) => {
                self.trust += 8.0;
                EmotionalResponse {
                    stress_modifier: 0.0,
                    trust_modifier: 8.0,
                    dialogue_trigger: Some("apology_accepted".to_string()),
                    behavioral_change: Some("less_hostile".to_string()),
                }
            }
            _ => EmotionalResponse {
                stress_modifier: 0.0,
                trust_modifier: 0.0,
                dialogue_trigger: None,
                behavioral_change: None,
            }
        }
    }
}

#[derive(Component, Debug)]
pub struct CompanionAI {
    pub behavior_type: CompanionBehavior,
    pub decision_cooldown: f32,
    pub last_action: Option<String>,
    pub action_queue: Vec<CompanionAction>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CompanionBehavior {
    FollowPlayer,
    StayClose,
    Explore,
    Flee,
    Hide,
    Combat,
    Rest,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CompanionAction {
    MoveTo(Vec3),
    InteractWith(Entity),
    UseItem(String),
    Dialogue(String),
    Wait(f32),
    Panic,
}
