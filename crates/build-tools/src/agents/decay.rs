use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::path::PathBuf;
use std::collections::HashMap;

use crate::mcp_client::MCPClient;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecayConfig {
    pub dread_level: u8,
    pub world_corruption_level: f32,
    pub environmental_effects: Vec<EnvironmentalEffect>,
    pub npc_behaviors: HashMap<String, NPCBehaviorConfig>,
    pub economic_parameters: EconomicDecayConfig,
    pub reality_distortions: Vec<RealityDistortion>,
    pub corruption_progression: CorruptionProgression,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentalEffect {
    pub effect_type: String,
    pub intensity: f32,
    pub affected_biomes: Vec<String>,
    pub visual_changes: Vec<String>,
    pub audio_changes: Vec<String>,
    pub spread_rate: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NPCBehaviorConfig {
    pub npc_type: String,
    pub base_behavior: String,
    pub fear_threshold: f32,
    pub flee_probability: f32,
    pub aggression_modifier: f32,
    pub dialogue_changes: Vec<String>,
    pub visual_state_changes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EconomicDecayConfig {
    pub trade_disruption: f32,
    pub price_inflation: f32,
    pub shop_closures: Vec<String>,
    pub resource_scarcity: HashMap<String, f32>,
    pub unemployment_rate: f32,
    pub social_unrest_level: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealityDistortion {
    pub distortion_type: String,
    pub probability: f32,
    pub duration_range: (f32, f32),
    pub affected_systems: Vec<String>,
    pub player_perception_change: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorruptionProgression {
    pub current_epicenters: Vec<CorruptionEpicenter>,
    pub spread_patterns: Vec<SpreadPattern>,
    pub resistance_factors: HashMap<String, f32>,
    pub accelerators: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorruptionEpicenter {
    pub location_type: String,
    pub corruption_strength: f32,
    pub radius_of_influence: f32,
    pub growth_rate: f32,
    pub unique_effects: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpreadPattern {
    pub pattern_type: String,
    pub speed_multiplier: f32,
    pub preferred_paths: Vec<String>,
    pub barriers: Vec<String>,
}

pub struct DecayAgent {
    dread_level: u8,
    out_dir: PathBuf,
    mcp_client: MCPClient,
}

impl DecayAgent {
    pub fn new(dread_level: u8, out_dir: &std::path::Path, mcp_client: MCPClient) -> Self {
        Self {
            dread_level,
            out_dir: out_dir.to_path_buf(),
            mcp_client,
        }
    }
    
    pub async fn generate(&self) -> Result<DecayConfig> {
        println!("DecayAgent: Generating decay configuration for dread level {}", self.dread_level);
        
        // Query current world state
        let world_corruption = self.query_world_corruption().await?;
        let npc_states = self.query_npc_fear_levels().await?;
        
        // Generate decay configuration
        let decay_config = self.generate_decay_rules(world_corruption, npc_states)?;
        
        // Save decay rules to OUT_DIR
        let decay_dir = self.out_dir.join("decay");
        std::fs::create_dir_all(&decay_dir)?;
        
        // Save different types of decay data as separate files
        let corruption_path = decay_dir.join("corruption_rules.json");
        std::fs::write(
            corruption_path,
            serde_json::to_string_pretty(&decay_config.environmental_effects)?
        )?;
        
        let npc_path = decay_dir.join("npc_behaviors.json");
        std::fs::write(
            npc_path,
            serde_json::to_string_pretty(&decay_config.npc_behaviors)?
        )?;
        
        let economic_path = decay_dir.join("economic_collapse.json");
        std::fs::write(
            economic_path,
            serde_json::to_string_pretty(&decay_config.economic_parameters)?
        )?;
        
        let distortion_path = decay_dir.join("reality_distortions.json");
        std::fs::write(
            distortion_path,
            serde_json::to_string_pretty(&decay_config.reality_distortions)?
        )?;
        
        println!("DecayAgent: Generated {} environmental effects, {} NPC behavior configs, {} reality distortions",
                decay_config.environmental_effects.len(),
                decay_config.npc_behaviors.len(),
                decay_config.reality_distortions.len());
        
        Ok(decay_config)
    }
    
    async fn query_world_corruption(&self) -> Result<f32> {
        match self.mcp_client.query_world_corruption_level().await {
            Ok(level) => Ok(level),
            Err(_) => {
                // Fallback: calculate corruption based on dread level
                Ok((self.dread_level as f32 * 0.2).min(1.0))
            }
        }
    }
    
    async fn query_npc_fear_levels(&self) -> Result<HashMap<String, f32>> {
        match self.mcp_client.query_npc_fear_states().await {
            Ok(states) => Ok(states),
            Err(_) => {
                // Fallback: generate NPC fear states based on dread level
                Ok(self.generate_fallback_npc_states())
            }
        }
    }
    
    fn generate_fallback_npc_states(&self) -> HashMap<String, f32> {
        let mut states = HashMap::new();
        
        // Different NPC types react differently to dread
        let base_fear = self.dread_level as f32 * 0.2;
        
        states.insert("villager".to_string(), base_fear);
        states.insert("merchant".to_string(), base_fear * 1.2); // More sensitive
        states.insert("guard".to_string(), base_fear * 0.8);   // Less sensitive
        states.insert("child".to_string(), base_fear * 1.5);   // Very sensitive
        states.insert("elder".to_string(), base_fear * 0.9);   // Slightly less
        states.insert("priest".to_string(), base_fear * 0.7);  // More resistant
        
        states
    }
    
    fn generate_decay_rules(&self, world_corruption: f32, npc_states: HashMap<String, f32>) -> Result<DecayConfig> {
        // Generate environmental effects based on dread level
        let environmental_effects = self.generate_environmental_effects(world_corruption);
        
        // Generate NPC behavior changes
        let npc_behaviors = self.generate_npc_behaviors(&npc_states);
        
        // Generate economic decay parameters
        let economic_parameters = self.generate_economic_decay();
        
        // Generate reality distortions
        let reality_distortions = self.generate_reality_distortions(world_corruption);
        
        // Generate corruption progression patterns
        let corruption_progression = self.generate_corruption_progression(world_corruption);
        
        Ok(DecayConfig {
            dread_level: self.dread_level,
            world_corruption_level: world_corruption,
            environmental_effects,
            npc_behaviors,
            economic_parameters,
            reality_distortions,
            corruption_progression,
        })
    }
    
    fn generate_environmental_effects(&self, corruption: f32) -> Vec<EnvironmentalEffect> {
        let mut effects = Vec::new();
        
        match self.dread_level {
            0 => {
                // No corruption effects in peaceful stage
            },
            1 => {
                effects.push(EnvironmentalEffect {
                    effect_type: "subtle_shadow_lengthening".to_string(),
                    intensity: 0.2,
                    affected_biomes: vec!["forest".to_string(), "plains".to_string()],
                    visual_changes: vec!["longer_shadows".to_string(), "dimmer_lighting".to_string()],
                    audio_changes: vec!["reduced_bird_songs".to_string()],
                    spread_rate: 0.1,
                });
            },
            2 => {
                effects.push(EnvironmentalEffect {
                    effect_type: "vegetation_withering".to_string(),
                    intensity: 0.4,
                    affected_biomes: vec!["forest".to_string(), "swamp".to_string(), "plains".to_string()],
                    visual_changes: vec![
                        "brown_leaves".to_string(),
                        "dead_grass_patches".to_string(),
                        "wilted_flowers".to_string()
                    ],
                    audio_changes: vec![
                        "wind_through_dead_leaves".to_string(),
                        "distant_animal_distress".to_string()
                    ],
                    spread_rate: 0.3,
                });
                
                effects.push(EnvironmentalEffect {
                    effect_type: "water_contamination".to_string(),
                    intensity: 0.3,
                    affected_biomes: vec!["swamp".to_string(), "river".to_string()],
                    visual_changes: vec!["murky_water".to_string(), "floating_debris".to_string()],
                    audio_changes: vec!["bubbling_sounds".to_string(), "unnatural_splashing".to_string()],
                    spread_rate: 0.4,
                });
            },
            3 => {
                effects.push(EnvironmentalEffect {
                    effect_type: "structural_decay".to_string(),
                    intensity: 0.6,
                    affected_biomes: vec!["village".to_string(), "ruins".to_string()],
                    visual_changes: vec![
                        "cracked_walls".to_string(),
                        "broken_windows".to_string(),
                        "collapsed_roofs".to_string(),
                        "rust_stains".to_string()
                    ],
                    audio_changes: vec![
                        "creaking_wood".to_string(),
                        "settling_sounds".to_string(),
                        "distant_collapses".to_string()
                    ],
                    spread_rate: 0.5,
                });
                
                effects.push(EnvironmentalEffect {
                    effect_type: "atmospheric_oppression".to_string(),
                    intensity: 0.7,
                    affected_biomes: vec!["all".to_string()],
                    visual_changes: vec![
                        "heavy_fog".to_string(),
                        "reduced_visibility".to_string(),
                        "unnatural_darkness".to_string()
                    ],
                    audio_changes: vec![
                        "oppressive_silence".to_string(),
                        "distant_rumbling".to_string(),
                        "echo_distortion".to_string()
                    ],
                    spread_rate: 0.8,
                });
            },
            4 => {
                effects.push(EnvironmentalEffect {
                    effect_type: "reality_breakdown".to_string(),
                    intensity: 0.9,
                    affected_biomes: vec!["all".to_string()],
                    visual_changes: vec![
                        "geometry_distortion".to_string(),
                        "texture_corruption".to_string(),
                        "impossible_architecture".to_string(),
                        "shifting_landscapes".to_string()
                    ],
                    audio_changes: vec![
                        "reality_tearing_sounds".to_string(),
                        "impossible_echoes".to_string(),
                        "non_euclidean_acoustics".to_string()
                    ],
                    spread_rate: 1.0,
                });
            },
            _ => {}
        }
        
        effects
    }
    
    fn generate_npc_behaviors(&self, npc_states: &HashMap<String, f32>) -> HashMap<String, NPCBehaviorConfig> {
        let mut behaviors = HashMap::new();
        
        for (npc_type, fear_level) in npc_states {
            let behavior_config = match self.dread_level {
                0 => NPCBehaviorConfig {
                    npc_type: npc_type.clone(),
                    base_behavior: "normal".to_string(),
                    fear_threshold: 1.0,
                    flee_probability: 0.0,
                    aggression_modifier: 1.0,
                    dialogue_changes: vec!["friendly".to_string(), "helpful".to_string()],
                    visual_state_changes: vec!["normal".to_string()],
                },
                1 => NPCBehaviorConfig {
                    npc_type: npc_type.clone(),
                    base_behavior: "cautious".to_string(),
                    fear_threshold: 0.8,
                    flee_probability: 0.1,
                    aggression_modifier: 0.9,
                    dialogue_changes: vec!["nervous".to_string(), "worried".to_string()],
                    visual_state_changes: vec!["tense_posture".to_string(), "frequent_looking_around".to_string()],
                },
                2 => NPCBehaviorConfig {
                    npc_type: npc_type.clone(),
                    base_behavior: "fearful".to_string(),
                    fear_threshold: 0.6,
                    flee_probability: 0.3,
                    aggression_modifier: 0.7,
                    dialogue_changes: vec![
                        "fearful".to_string(),
                        "seeking_reassurance".to_string(),
                        "rumors_and_warnings".to_string()
                    ],
                    visual_state_changes: vec![
                        "hunched_shoulders".to_string(),
                        "darting_eyes".to_string(),
                        "protective_gestures".to_string()
                    ],
                },
                3 => NPCBehaviorConfig {
                    npc_type: npc_type.clone(),
                    base_behavior: "panicked".to_string(),
                    fear_threshold: 0.4,
                    flee_probability: 0.6,
                    aggression_modifier: match npc_type.as_str() {
                        "guard" => 1.2, // Guards become more aggressive
                        _ => 0.5,
                    },
                    dialogue_changes: vec![
                        "panicked".to_string(),
                        "desperate".to_string(),
                        "irrational".to_string(),
                        "begging_for_help".to_string()
                    ],
                    visual_state_changes: vec![
                        "shaking".to_string(),
                        "wild_eyes".to_string(),
                        "defensive_postures".to_string(),
                        "rapid_movements".to_string()
                    ],
                },
                4 => NPCBehaviorConfig {
                    npc_type: npc_type.clone(),
                    base_behavior: "broken".to_string(),
                    fear_threshold: 0.2,
                    flee_probability: match npc_type.as_str() {
                        "guard" => 0.4, // Some guards still try to fight
                        _ => 0.9,       // Everyone else flees
                    },
                    aggression_modifier: match npc_type.as_str() {
                        "guard" => 1.5, // Desperate aggression
                        _ => 0.2,       // Broken and helpless
                    },
                    dialogue_changes: vec![
                        "broken".to_string(),
                        "nonsensical".to_string(),
                        "traumatized".to_string(),
                        "silent".to_string()
                    ],
                    visual_state_changes: vec![
                        "catatonic_episodes".to_string(),
                        "thousand_yard_stare".to_string(),
                        "uncontrollable_shaking".to_string(),
                        "fetal_position".to_string()
                    ],
                },
                _ => NPCBehaviorConfig {
                    npc_type: npc_type.clone(),
                    base_behavior: "normal".to_string(),
                    fear_threshold: 1.0,
                    flee_probability: 0.0,
                    aggression_modifier: 1.0,
                    dialogue_changes: vec!["normal".to_string()],
                    visual_state_changes: vec!["normal".to_string()],
                }
            };
            
            behaviors.insert(npc_type.clone(), behavior_config);
        }
        
        behaviors
    }
    
    fn generate_economic_decay(&self) -> EconomicDecayConfig {
        match self.dread_level {
            0 => EconomicDecayConfig {
                trade_disruption: 0.0,
                price_inflation: 1.0,
                shop_closures: vec![],
                resource_scarcity: HashMap::new(),
                unemployment_rate: 0.05,
                social_unrest_level: 0.0,
            },
            1 => EconomicDecayConfig {
                trade_disruption: 0.1,
                price_inflation: 1.1,
                shop_closures: vec![],
                resource_scarcity: {
                    let mut scarcity = HashMap::new();
                    scarcity.insert("exotic_goods".to_string(), 0.2);
                    scarcity
                },
                unemployment_rate: 0.08,
                social_unrest_level: 0.1,
            },
            2 => EconomicDecayConfig {
                trade_disruption: 0.3,
                price_inflation: 1.3,
                shop_closures: vec!["luxury_goods".to_string()],
                resource_scarcity: {
                    let mut scarcity = HashMap::new();
                    scarcity.insert("exotic_goods".to_string(), 0.5);
                    scarcity.insert("medicine".to_string(), 0.3);
                    scarcity
                },
                unemployment_rate: 0.15,
                social_unrest_level: 0.3,
            },
            3 => EconomicDecayConfig {
                trade_disruption: 0.6,
                price_inflation: 1.8,
                shop_closures: vec![
                    "luxury_goods".to_string(),
                    "entertainment".to_string(),
                    "non_essential_crafts".to_string()
                ],
                resource_scarcity: {
                    let mut scarcity = HashMap::new();
                    scarcity.insert("food".to_string(), 0.4);
                    scarcity.insert("medicine".to_string(), 0.6);
                    scarcity.insert("tools".to_string(), 0.3);
                    scarcity
                },
                unemployment_rate: 0.3,
                social_unrest_level: 0.6,
            },
            4 => EconomicDecayConfig {
                trade_disruption: 0.9,
                price_inflation: 3.0,
                shop_closures: vec![
                    "all_non_essential".to_string(),
                    "most_essential".to_string()
                ],
                resource_scarcity: {
                    let mut scarcity = HashMap::new();
                    scarcity.insert("food".to_string(), 0.8);
                    scarcity.insert("medicine".to_string(), 0.9);
                    scarcity.insert("tools".to_string(), 0.7);
                    scarcity.insert("water".to_string(), 0.5);
                    scarcity
                },
                unemployment_rate: 0.7,
                social_unrest_level: 0.9,
            },
            _ => EconomicDecayConfig {
                trade_disruption: 0.0,
                price_inflation: 1.0,
                shop_closures: vec![],
                resource_scarcity: HashMap::new(),
                unemployment_rate: 0.05,
                social_unrest_level: 0.0,
            }
        }
    }
    
    fn generate_reality_distortions(&self, corruption: f32) -> Vec<RealityDistortion> {
        let mut distortions = Vec::new();
        
        if self.dread_level >= 2 {
            distortions.push(RealityDistortion {
                distortion_type: "false_audio".to_string(),
                probability: 0.1 * corruption,
                duration_range: (2.0, 8.0),
                affected_systems: vec!["audio".to_string()],
                player_perception_change: "hears sounds that aren't there".to_string(),
            });
        }
        
        if self.dread_level >= 3 {
            distortions.push(RealityDistortion {
                distortion_type: "phantom_movement".to_string(),
                probability: 0.15 * corruption,
                duration_range: (1.0, 3.0),
                affected_systems: vec!["visual".to_string(), "movement".to_string()],
                player_perception_change: "sees movement in peripheral vision".to_string(),
            });
            
            distortions.push(RealityDistortion {
                distortion_type: "time_distortion".to_string(),
                probability: 0.05 * corruption,
                duration_range: (5.0, 15.0),
                affected_systems: vec!["time".to_string(), "progression".to_string()],
                player_perception_change: "time seems to move strangely".to_string(),
            });
        }
        
        if self.dread_level >= 4 {
            distortions.push(RealityDistortion {
                distortion_type: "false_companions".to_string(),
                probability: 0.2 * corruption,
                duration_range: (10.0, 30.0),
                affected_systems: vec!["companions".to_string(), "dialogue".to_string()],
                player_perception_change: "sees companions who aren't there".to_string(),
            });
            
            distortions.push(RealityDistortion {
                distortion_type: "spatial_distortion".to_string(),
                probability: 0.3 * corruption,
                duration_range: (3.0, 12.0),
                affected_systems: vec!["navigation".to_string(), "world".to_string()],
                player_perception_change: "world geometry becomes impossible".to_string(),
            });
        }
        
        distortions
    }
    
    fn generate_corruption_progression(&self, current_corruption: f32) -> CorruptionProgression {
        let epicenters = match self.dread_level {
            0 => vec![],
            1 => vec![
                CorruptionEpicenter {
                    location_type: "ancient_ruins".to_string(),
                    corruption_strength: 0.3,
                    radius_of_influence: 2.0,
                    growth_rate: 0.1,
                    unique_effects: vec!["whispers".to_string(), "cold_spots".to_string()],
                }
            ],
            2 => vec![
                CorruptionEpicenter {
                    location_type: "ancient_ruins".to_string(),
                    corruption_strength: 0.6,
                    radius_of_influence: 4.0,
                    growth_rate: 0.2,
                    unique_effects: vec!["whispers".to_string(), "cold_spots".to_string(), "shadow_movement".to_string()],
                },
                CorruptionEpicenter {
                    location_type: "abandoned_village".to_string(),
                    corruption_strength: 0.4,
                    radius_of_influence: 3.0,
                    growth_rate: 0.15,
                    unique_effects: vec!["phantom_lights".to_string(), "false_voices".to_string()],
                }
            ],
            3 => vec![
                CorruptionEpicenter {
                    location_type: "dragon_lair_approach".to_string(),
                    corruption_strength: 0.9,
                    radius_of_influence: 8.0,
                    growth_rate: 0.4,
                    unique_effects: vec![
                        "reality_tears".to_string(),
                        "time_distortion".to_string(),
                        "nightmare_manifestations".to_string()
                    ],
                }
            ],
            4 => vec![
                CorruptionEpicenter {
                    location_type: "labyrinth_center".to_string(),
                    corruption_strength: 1.0,
                    radius_of_influence: 20.0,
                    growth_rate: 0.8,
                    unique_effects: vec![
                        "complete_reality_breakdown".to_string(),
                        "dragon_presence".to_string(),
                        "psychological_assault".to_string()
                    ],
                }
            ],
            _ => vec![]
        };
        
        let spread_patterns = vec![
            SpreadPattern {
                pattern_type: "along_paths".to_string(),
                speed_multiplier: 1.5,
                preferred_paths: vec!["roads".to_string(), "rivers".to_string()],
                barriers: vec!["holy_sites".to_string(), "bright_areas".to_string()],
            },
            SpreadPattern {
                pattern_type: "through_fear".to_string(),
                speed_multiplier: 2.0,
                preferred_paths: vec!["populated_areas".to_string()],
                barriers: vec!["courageous_npcs".to_string(), "companions".to_string()],
            },
        ];
        
        let mut resistance_factors = HashMap::new();
        resistance_factors.insert("companion_presence".to_string(), 0.5);
        resistance_factors.insert("holy_blessing".to_string(), 0.3);
        resistance_factors.insert("player_courage".to_string(), 0.2);
        
        CorruptionProgression {
            current_epicenters: epicenters,
            spread_patterns,
            resistance_factors,
            accelerators: vec![
                "companion_loss".to_string(),
                "moral_compromise".to_string(),
                "dragon_proximity".to_string(),
            ],
        }
    }
}
