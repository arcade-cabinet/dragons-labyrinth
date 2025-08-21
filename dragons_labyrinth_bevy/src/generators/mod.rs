// AI-powered content generators for specialized game formats
// Generates .cob (Cobweb UI), .yol (Yoleck levels), and ECS components

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use rand::Rng;

pub mod cob_generator;
pub mod yol_generator;
pub mod ecs_world_generator;

/// Core trait for all AI generators
pub trait ContentGenerator {
    type Output: Serialize;
    
    fn generate(&self, prompt: &str, dread_level: u8) -> Self::Output;
    fn validate(&self, content: &Self::Output) -> Result<(), String>;
}

/// Manages all content generation pipelines
pub struct GeneratorPipeline {
    pub cob_gen: cob_generator::CobGenerator,
    pub yol_gen: yol_generator::YolGenerator,
    pub ecs_gen: ecs_world_generator::EcsWorldGenerator,
}

impl GeneratorPipeline {
    pub fn new() -> Self {
        Self {
            cob_gen: cob_generator::CobGenerator::new(),
            yol_gen: yol_generator::YolGenerator::new(),
            ecs_gen: ecs_world_generator::EcsWorldGenerator::new(),
        }
    }
    
    /// Generate a complete zone with all required assets
    pub fn generate_zone(&self, biome: &str, dread_level: u8) -> ZoneContent {
        ZoneContent {
            ui_scenes: self.cob_gen.generate_biome_ui(biome, dread_level),
            level_data: self.yol_gen.generate_level(biome, dread_level),
            entities: self.ecs_gen.generate_encounters(biome, dread_level),
            dialogue: self.generate_dialogue(biome, dread_level),
        }
    }
    
    fn generate_dialogue(&self, biome: &str, dread_level: u8) -> Vec<DialogueNode> {
        // Generate Yarn-compatible dialogue based on biome and dread
        vec![
            DialogueNode {
                id: format!("{}_{}_intro", biome, dread_level),
                text: self.generate_dialogue_text(biome, dread_level),
                choices: self.generate_choices(dread_level),
            }
        ]
    }
    
    fn generate_dialogue_text(&self, biome: &str, dread_level: u8) -> String {
        match (biome, dread_level) {
            ("meadow", 0) => "The sun warms your face as you walk the familiar path...".to_string(),
            ("meadow", 1) => "Something feels different today. The birds are too quiet...".to_string(),
            ("forest", 2) => "The trees lean in, their shadows darker than they should be...".to_string(),
            ("swamp", 3) => "Each step squelches with a sound that isn't quite water...".to_string(),
            ("labyrinth", 4) => "Stone walls pulse with a heartbeat not your own...".to_string(),
            _ => format!("You find yourself in the {} at dread level {}", biome, dread_level),
        }
    }
    
    fn generate_choices(&self, dread_level: u8) -> Vec<DialogueChoice> {
        if dread_level < 2 {
            vec![
                DialogueChoice { text: "Continue forward".to_string(), next: "explore".to_string() },
                DialogueChoice { text: "Look around".to_string(), next: "investigate".to_string() },
            ]
        } else {
            vec![
                DialogueChoice { text: "Steel yourself".to_string(), next: "brave".to_string() },
                DialogueChoice { text: "Turn back".to_string(), next: "flee".to_string() },
                DialogueChoice { text: "Call out".to_string(), next: "desperate".to_string() },
            ]
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ZoneContent {
    pub ui_scenes: Vec<String>,  // .cob files
    pub level_data: String,       // .yol file
    pub entities: Vec<EntitySpawn>,
    pub dialogue: Vec<DialogueNode>,
}

#[derive(Serialize, Deserialize)]
pub struct EntitySpawn {
    pub entity_type: String,
    pub position: (f32, f32, f32),
    pub components: HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize)]
pub struct DialogueNode {
    pub id: String,
    pub text: String,
    pub choices: Vec<DialogueChoice>,
}

#[derive(Serialize, Deserialize)]
pub struct DialogueChoice {
    pub text: String,
    pub next: String,
}