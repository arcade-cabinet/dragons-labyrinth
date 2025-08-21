// Yoleck level editor .yol file generator
// Generates level data for AI-driven content

use super::ContentGenerator;
use serde::{Deserialize, Serialize};
use rand::Rng;

pub struct YolGenerator;

impl YolGenerator {
    pub fn new() -> Self {
        Self
    }
    
    pub fn generate_level(&self, biome: &str, dread_level: u8) -> String {
        let level = YolLevel {
            format_version: 3,
            app_format_version: 1,
            entities: self.generate_entities(biome, dread_level),
        };
        
        serde_json::to_string_pretty(&level).unwrap()
    }
    
    fn generate_entities(&self, biome: &str, dread_level: u8) -> Vec<YolEntity> {
        let mut entities = Vec::new();
        let mut rng = rand::thread_rng();
        
        // Generate hex tiles
        for q in -10..=10 {
            for r in -10..=10 {
                if rng.gen_bool(0.8) {  // 80% chance of tile existing
                    entities.push(self.create_hex_tile(q, r, biome, dread_level));
                }
            }
        }
        
        // Add NPCs based on dread level
        let npc_count = match dread_level {
            0 => 8,  // Many friendly NPCs
            1 => 6,
            2 => 4,
            3 => 2,  // Few desperate survivors
            _ => 0,  // No NPCs in horror stage
        };
        
        for _ in 0..npc_count {
            entities.push(self.create_npc(
                rng.gen_range(-8..8),
                rng.gen_range(-8..8),
                dread_level
            ));
        }
        
        // Add monsters
        let monster_count = dread_level as usize * 3;
        for _ in 0..monster_count {
            entities.push(self.create_monster(
                rng.gen_range(-10..10),
                rng.gen_range(-10..10),
                biome,
                dread_level
            ));
        }
        
        entities
    }
    
    fn create_hex_tile(&self, q: i32, r: i32, biome: &str, dread_level: u8) -> YolEntity {
        let mut rng = rand::thread_rng();
        let corruption = (dread_level as f32 / 4.0) + rng.gen_range(-0.1..0.1);
        
        YolEntity {
            format_version: 1,
            name: "HexTile".to_string(),
            components: serde_json::json!({
                "Position": {
                    "q": q,
                    "r": r,
                },
                "TileType": self.get_tile_type(biome, corruption),
                "Corruption": corruption,
                "Passable": rng.gen_bool(0.9 - (dread_level as f64 * 0.1)),
                "Height": rng.gen_range(-0.5..2.0) * (1.0 - corruption),
            }),
        }
    }
    
    fn create_npc(&self, q: i32, r: i32, dread_level: u8) -> YolEntity {
        let npc_types = match dread_level {
            0 => vec!["Merchant", "Guard", "Villager", "Child"],
            1 => vec!["Nervous_Guard", "Worried_Merchant", "Hiding_Villager"],
            2 => vec!["Desperate_Survivor", "Mad_Prophet", "Fleeing_Guard"],
            3 => vec!["Gibbering_Madman", "Shadow_Person"],
            _ => vec!["Echo"],
        };
        
        let mut rng = rand::thread_rng();
        YolEntity {
            format_version: 1,
            name: "NPC".to_string(),
            components: serde_json::json!({
                "Position": { "q": q, "r": r },
                "NPCType": npc_types[rng.gen_range(0..npc_types.len())],
                "SanityLevel": 100 - (dread_level * 25),
                "DialogueTree": format!("npc_{}_{}", dread_level, rng.gen_range(0..3)),
                "FleeThreshold": dread_level * 20,
            }),
        }
    }
    
    fn create_monster(&self, q: i32, r: i32, biome: &str, dread_level: u8) -> YolEntity {
        let monster_type = match (biome, dread_level) {
            (_, 0) => "Shadow_Rabbit",  // Harmless but unsettling
            ("forest", 1) => "Whispering_Tree",
            ("meadow", 1) => "Wrong_Bird",
            ("swamp", 2) => "Bog_Wraith",
            (_, 2) => "Faceless_Walker",
            (_, 3) => "Screaming_Void",
            ("labyrinth", 4) => "Dragon_Echo",
            _ => "Nightmare_Fragment",
        };
        
        let mut rng = rand::thread_rng();
        YolEntity {
            format_version: 1,
            name: "Monster".to_string(),
            components: serde_json::json!({
                "Position": { "q": q, "r": r },
                "MonsterType": monster_type,
                "Health": 50 + (dread_level * 30),
                "Damage": 5 + (dread_level * 10),
                "DetectionRadius": 3 + dread_level,
                "MoveSpeed": 0.5 + (dread_level as f32 * 0.2),
                "Behavior": self.get_monster_behavior(dread_level),
            }),
        }
    }
    
    fn get_tile_type(&self, biome: &str, corruption: f32) -> String {
        if corruption > 0.7 {
            "Corrupted".to_string()
        } else {
            match biome {
                "meadow" => "Grass",
                "forest" => "Forest_Floor",
                "swamp" => "Murky_Water",
                "ruins" => "Cracked_Stone",
                "labyrinth" => "Ancient_Stone",
                _ => "Default",
            }.to_string()
        }
    }
    
    fn get_monster_behavior(&self, dread_level: u8) -> String {
        match dread_level {
            0 => "Observe",      // Just watches
            1 => "Follow",       // Follows at distance
            2 => "Stalk",        // Gets closer
            3 => "Hunt",         // Actively pursues
            _ => "Relentless",   // Never stops
        }.to_string()
    }
}

#[derive(Serialize, Deserialize)]
struct YolLevel {
    format_version: u32,
    app_format_version: u32,
    entities: Vec<YolEntity>,
}

#[derive(Serialize, Deserialize)]
struct YolEntity {
    format_version: u32,
    name: String,
    components: serde_json::Value,
}

impl ContentGenerator for YolGenerator {
    type Output = String;
    
    fn generate(&self, prompt: &str, dread_level: u8) -> Self::Output {
        self.generate_level(prompt, dread_level)
    }
    
    fn validate(&self, content: &Self::Output) -> Result<(), String> {
        match serde_json::from_str::<YolLevel>(content) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Invalid .yol format: {}", e)),
        }
    }
}