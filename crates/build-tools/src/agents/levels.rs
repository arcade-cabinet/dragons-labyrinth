use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::path::PathBuf;
use std::collections::HashMap;

use crate::mcp_client::MCPClient;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelsConfig {
    pub dread_level: u8,
    pub encounter_placements: Vec<EncounterPlacement>,
    pub sentimental_items: Vec<SentimentalItem>,
    pub philosophical_path_variations: HashMap<String, PathVariation>,
    pub narrative_triggers: Vec<NarrativeTrigger>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncounterPlacement {
    pub encounter_id: String,
    pub encounter_type: String,
    pub hex_position: (i32, i32),
    pub min_dread_level: u8,
    pub max_dread_level: u8,
    pub moral_complexity: String,
    pub companion_reactions: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentimentalItem {
    pub item_id: String,
    pub item_name: String,
    pub item_type: String,
    pub hex_position: (i32, i32),
    pub associated_companion: Option<String>,
    pub emotional_significance: String,
    pub forge_trial_relevance: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathVariation {
    pub path_name: String,
    pub encounters_modified: Vec<String>,
    pub unique_encounters: Vec<String>,
    pub companion_interactions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarrativeTrigger {
    pub trigger_id: String,
    pub trigger_type: String,
    pub hex_position: (i32, i32),
    pub activation_conditions: Vec<String>,
    pub dread_progression_impact: f32,
}

pub struct LevelsAgent {
    dread_level: u8,
    out_dir: PathBuf,
    mcp_client: MCPClient,
}

impl LevelsAgent {
    pub fn new(dread_level: u8, out_dir: &std::path::Path, mcp_client: MCPClient) -> Self {
        Self {
            dread_level,
            out_dir: out_dir.to_path_buf(),
            mcp_client,
        }
    }
    
    pub async fn generate(&self) -> Result<LevelsConfig> {
        println!("LevelsAgent: Generating level content for dread level {}", self.dread_level);
        
        // Query game state for informed placement decisions
        let philosophical_states = self.query_philosophical_progression().await?;
        
        // Generate level content
        let levels_config = self.generate_level_content(philosophical_states)?;
        
        // Save level data to OUT_DIR
        let levels_dir = self.out_dir.join("levels");
        std::fs::create_dir_all(&levels_dir)?;
        
        let encounters_path = levels_dir.join("encounters.json");
        std::fs::write(
            encounters_path,
            serde_json::to_string_pretty(&levels_config.encounter_placements)?
        )?;
        
        let items_path = levels_dir.join("sentimental_items.json");
        std::fs::write(
            items_path,
            serde_json::to_string_pretty(&levels_config.sentimental_items)?
        )?;
        
        println!("LevelsAgent: Generated {} encounters, {} sentimental items",
                levels_config.encounter_placements.len(),
                levels_config.sentimental_items.len());
        
        Ok(levels_config)
    }
    
    async fn query_philosophical_progression(&self) -> Result<HashMap<String, f32>> {
        match self.mcp_client.query_philosophical_progression().await {
            Ok(progression) => Ok(progression),
            Err(_) => {
                let mut progression = HashMap::new();
                progression.insert("compassion".to_string(), 0.2);
                progression.insert("justice".to_string(), 0.2);
                progression.insert("wisdom".to_string(), 0.2);
                progression.insert("courage".to_string(), 0.2);
                Ok(progression)
            }
        }
    }
    
    fn generate_level_content(&self, philosophical_states: HashMap<String, f32>) -> Result<LevelsConfig> {
        let encounter_placements = self.generate_encounters()?;
        let sentimental_items = self.generate_sentimental_items()?;
        let philosophical_path_variations = self.generate_path_variations(&philosophical_states)?;
        let narrative_triggers = self.generate_narrative_triggers()?;
        
        Ok(LevelsConfig {
            dread_level: self.dread_level,
            encounter_placements,
            sentimental_items,
            philosophical_path_variations,
            narrative_triggers,
        })
    }
    
    fn generate_encounters(&self) -> Result<Vec<EncounterPlacement>> {
        let mut encounters = Vec::new();
        
        match self.dread_level {
            0 => {
                encounters.push(EncounterPlacement {
                    encounter_id: "village_welcome".to_string(),
                    encounter_type: "social_introduction".to_string(),
                    hex_position: (0, 0),
                    min_dread_level: 0,
                    max_dread_level: 0,
                    moral_complexity: "simple_kindness".to_string(),
                    companion_reactions: {
                        let mut reactions = HashMap::new();
                        reactions.insert("mira".to_string(), "encouraging".to_string());
                        reactions.insert("einar".to_string(), "supportive".to_string());
                        reactions
                    },
                });
            },
            1 => {
                encounters.push(EncounterPlacement {
                    encounter_id: "hollow_caretaker_boss".to_string(),
                    encounter_type: "boss_moral_choice".to_string(),
                    hex_position: (5, 3),
                    min_dread_level: 1,
                    max_dread_level: 1,
                    moral_complexity: "mercy_vs_justice".to_string(),
                    companion_reactions: {
                        let mut reactions = HashMap::new();
                        reactions.insert("mira".to_string(), "prefers_mercy".to_string());
                        reactions.insert("einar".to_string(), "accepts_justice".to_string());
                        reactions
                    },
                });
            },
            2 => {
                encounters.push(EncounterPlacement {
                    encounter_id: "companion_doubt_crisis".to_string(),
                    encounter_type: "social_crisis".to_string(),
                    hex_position: (8, 5),
                    min_dread_level: 2,
                    max_dread_level: 2,
                    moral_complexity: "leadership_crisis".to_string(),
                    companion_reactions: {
                        let mut reactions = HashMap::new();
                        reactions.insert("mira".to_string(), "questioning_everything".to_string());
                        reactions.insert("sorin".to_string(), "analyzing_situation".to_string());
                        reactions
                    },
                });
            },
            3 => {
                encounters.push(EncounterPlacement {
                    encounter_id: "companion_breaking_point".to_string(),
                    encounter_type: "companion_crisis".to_string(),
                    hex_position: (12, 8),
                    min_dread_level: 3,
                    max_dread_level: 3,
                    moral_complexity: "sacrifice_vs_protection".to_string(),
                    companion_reactions: {
                        let mut reactions = HashMap::new();
                        reactions.insert("mira".to_string(), "begging_to_stop".to_string());
                        reactions.insert("tamara".to_string(), "breaking_down".to_string());
                        reactions
                    },
                });
            },
            4 => {
                encounters.push(EncounterPlacement {
                    encounter_id: "labyrinth_entrance".to_string(),
                    encounter_type: "point_of_no_return".to_string(),
                    hex_position: (15, 10),
                    min_dread_level: 4,
                    max_dread_level: 4,
                    moral_complexity: "final_choice".to_string(),
                    companion_reactions: {
                        let mut reactions = HashMap::new();
                        reactions.insert("any_remaining".to_string(), "final_farewell".to_string());
                        reactions
                    },
                });
            },
            _ => {}
        }
        
        Ok(encounters)
    }
    
    fn generate_sentimental_items(&self) -> Result<Vec<SentimentalItem>> {
        let mut items = Vec::new();
        
        items.push(SentimentalItem {
            item_id: "mira_locket".to_string(),
            item_name: "Mira's Family Locket".to_string(),
            item_type: "jewelry".to_string(),
            hex_position: (2, 1),
            associated_companion: Some("mira".to_string()),
            emotional_significance: "reminder of innocence and hope".to_string(),
            forge_trial_relevance: 0.8,
        });
        
        items.push(SentimentalItem {
            item_id: "einar_sword_fragment".to_string(),
            item_name: "Fragment of Einar's Father's Sword".to_string(),
            item_type: "weapon_fragment".to_string(),
            hex_position: (6, 4),
            associated_companion: Some("einar".to_string()),
            emotional_significance: "legacy of duty and sacrifice".to_string(),
            forge_trial_relevance: 0.7,
        });
        
        items.push(SentimentalItem {
            item_id: "group_campfire_stone".to_string(),
            item_name: "Stone from First Campfire Together".to_string(),
            item_type: "memento".to_string(),
            hex_position: (1, 0),
            associated_companion: None,
            emotional_significance: "bond formed through shared journey".to_string(),
            forge_trial_relevance: 1.0,
        });
        
        Ok(items)
    }
    
    fn generate_path_variations(&self, philosophical_states: &HashMap<String, f32>) -> Result<HashMap<String, PathVariation>> {
        let mut variations = HashMap::new();
        
        variations.insert("compassion".to_string(), PathVariation {
            path_name: "Path of Compassion".to_string(),
            encounters_modified: vec!["hollow_caretaker_boss".to_string()],
            unique_encounters: vec!["healing_wounded_enemy".to_string()],
            companion_interactions: vec!["mira_feels_understood".to_string()],
        });
        
        variations.insert("justice".to_string(), PathVariation {
            path_name: "Path of Justice".to_string(),
            encounters_modified: vec!["hollow_caretaker_boss".to_string()],
            unique_encounters: vec!["judging_corrupt_official".to_string()],
            companion_interactions: vec!["einar_respects_decisiveness".to_string()],
        });
        
        variations.insert("wisdom".to_string(), PathVariation {
            path_name: "Path of Wisdom".to_string(),
            encounters_modified: vec!["companion_doubt_crisis".to_string()],
            unique_encounters: vec!["ancient_knowledge_choice".to_string()],
            companion_interactions: vec!["sorin_finds_intellectual_equal".to_string()],
        });
        
        variations.insert("courage".to_string(), PathVariation {
            path_name: "Path of Courage".to_string(),
            encounters_modified: vec!["labyrinth_entrance".to_string()],
            unique_encounters: vec!["facing_greatest_fear".to_string()],
            companion_interactions: vec!["einar_feels_inspired".to_string()],
        });
        
        Ok(variations)
    }
    
    fn generate_narrative_triggers(&self) -> Result<Vec<NarrativeTrigger>> {
        let mut triggers = Vec::new();
        
        triggers.push(NarrativeTrigger {
            trigger_id: format!("dread_transition_{}", self.dread_level),
            trigger_type: "dread_progression".to_string(),
            hex_position: (self.dread_level as i32 * 3, self.dread_level as i32 * 2),
            activation_conditions: vec![
                format!("dread_level_{}_conditions_met", self.dread_level),
                "player_in_area".to_string()
            ],
            dread_progression_impact: 0.2,
        });
        
        if self.dread_level >= 2 {
            triggers.push(NarrativeTrigger {
                trigger_id: "first_companion_questioning".to_string(),
                trigger_type: "companion_development".to_string(),
                hex_position: (7, 5),
                activation_conditions: vec![
                    "moral_choices_made".to_string(),
                    "world_changes_noticed".to_string()
                ],
                dread_progression_impact: 0.1,
            });
        }
        
        Ok(triggers)
    }
}