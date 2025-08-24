use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::path::PathBuf;
use std::collections::HashMap;

use crate::mcp_client::MCPClient;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MountConfig {
    pub dread_level: u8,
    pub mount_personalities: HashMap<String, MountPersonality>,
    pub bonding_mechanics: BondingMechanics,
    pub protection_abilities: HashMap<String, ProtectionAbility>,
    pub trauma_responses: HashMap<String, TraumaResponse>,
    pub environmental_adaptations: Vec<EnvironmentalAdaptation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MountPersonality {
    pub mount_type: String,
    pub base_temperament: String,
    pub loyalty_capacity: f32,
    pub fear_resistance: f32,
    pub protective_instinct: f32,
    pub intelligence_level: f32,
    pub empathy_sensitivity: f32,
    pub breakdown_threshold: f32,
    pub unique_traits: Vec<String>,
    pub dread_reactions: HashMap<u8, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BondingMechanics {
    pub initial_trust: f32,
    pub trust_building_factors: HashMap<String, f32>,
    pub trust_damaging_factors: HashMap<String, f32>,
    pub bond_strength_thresholds: HashMap<String, f32>,
    pub bonding_activities: Vec<BondingActivity>,
    pub abandonment_conditions: Vec<String>,
    pub loyalty_tests: Vec<LoyaltyTest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BondingActivity {
    pub activity_type: String,
    pub trust_gain: f32,
    pub time_required: f32,
    pub dread_level_availability: Vec<u8>,
    pub success_conditions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoyaltyTest {
    pub test_scenario: String,
    pub required_bond_strength: f32,
    pub success_outcomes: Vec<String>,
    pub failure_outcomes: Vec<String>,
    pub trauma_impact: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtectionAbility {
    pub ability_name: String,
    pub protection_type: String,
    pub effectiveness: f32,
    pub energy_cost: f32,
    pub cooldown_time: f32,
    pub bond_requirement: f32,
    pub environmental_conditions: Vec<String>,
    pub dread_level_scaling: HashMap<u8, f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraumaResponse {
    pub trauma_trigger: String,
    pub immediate_reaction: String,
    pub behavioral_changes: Vec<String>,
    pub performance_impact: f32,
    pub recovery_time: f32,
    pub recovery_requirements: Vec<String>,
    pub permanent_changes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentalAdaptation {
    pub environment_type: String,
    pub adaptation_bonus: f32,
    pub special_abilities: Vec<String>,
    pub stress_factors: Vec<String>,
    pub comfort_level: f32,
}

pub struct MountAgent {
    dread_level: u8,
    out_dir: PathBuf,
    mcp_client: MCPClient,
}

impl MountAgent {
    pub fn new(dread_level: u8, out_dir: &std::path::Path, mcp_client: MCPClient) -> Self {
        Self {
            dread_level,
            out_dir: out_dir.to_path_buf(),
            mcp_client,
        }
    }
    
    pub async fn generate(&self) -> Result<MountConfig> {
        println!("MountAgent: Generating mount configuration for dread level {}", self.dread_level);
        
        // Query companion states to understand trauma context
        let companion_states = self.query_companion_trauma_states().await?;
        let world_corruption = self.query_world_corruption().await?;
        
        // Generate mount configuration
        let mount_config = self.generate_mount_systems(companion_states, world_corruption)?;
        
        // Save mount data to OUT_DIR
        let mount_dir = self.out_dir.join("mounts");
        std::fs::create_dir_all(&mount_dir)?;
        
        // Save different aspects as separate TOML files
        let personalities_path = mount_dir.join("personalities.toml");
        std::fs::write(
            personalities_path,
            toml::to_string(&mount_config.mount_personalities)?
        )?;
        
        let bonding_path = mount_dir.join("bonding_mechanics.toml");
        std::fs::write(
            bonding_path,
            toml::to_string(&mount_config.bonding_mechanics)?
        )?;
        
        let protection_path = mount_dir.join("protection_abilities.toml");
        std::fs::write(
            protection_path,
            toml::to_string(&mount_config.protection_abilities)?
        )?;
        
        let trauma_path = mount_dir.join("trauma_responses.toml");
        std::fs::write(
            trauma_path,
            toml::to_string(&mount_config.trauma_responses)?
        )?;
        
        println!("MountAgent: Generated {} mount personalities, {} protection abilities, {} trauma responses",
                mount_config.mount_personalities.len(),
                mount_config.protection_abilities.len(),
                mount_config.trauma_responses.len());
        
        Ok(mount_config)
    }
    
    async fn query_companion_trauma_states(&self) -> Result<HashMap<String, f32>> {
        match self.mcp_client.query_companion_trauma_states().await {
            Ok(states) => {
                // Convert complex trauma states to simple map for mount calculations
                let mut simple_states = HashMap::new();
                // This would be properly implemented based on the actual MCP response format
                simple_states.insert("average_trauma".to_string(), self.dread_level as f32 * 0.2);
                Ok(simple_states)
            },
            Err(_) => {
                // Fallback
                let mut states = HashMap::new();
                states.insert("average_trauma".to_string(), self.dread_level as f32 * 0.2);
                Ok(states)
            }
        }
    }
    
    async fn query_world_corruption(&self) -> Result<f32> {
        match self.mcp_client.query_world_corruption_level().await {
            Ok(level) => Ok(level),
            Err(_) => Ok(self.dread_level as f32 * 0.2)
        }
    }
    
    fn generate_mount_systems(&self, _companion_states: HashMap<String, f32>, world_corruption: f32) -> Result<MountConfig> {
        // Generate different mount personality types
        let mount_personalities = self.generate_mount_personalities();
        
        // Generate bonding mechanics that respond to dread
        let bonding_mechanics = self.generate_bonding_mechanics();
        
        // Generate protection abilities
        let protection_abilities = self.generate_protection_abilities();
        
        // Generate trauma response patterns
        let trauma_responses = self.generate_trauma_responses();
        
        // Generate environmental adaptations
        let environmental_adaptations = self.generate_environmental_adaptations(world_corruption);
        
        Ok(MountConfig {
            dread_level: self.dread_level,
            mount_personalities,
            bonding_mechanics,
            protection_abilities,
            trauma_responses,
            environmental_adaptations,
        })
    }
    
    fn generate_mount_personalities(&self) -> HashMap<String, MountPersonality> {
        let mut personalities = HashMap::new();
        
        // War Horse - Loyal and brave
        personalities.insert("war_horse".to_string(), MountPersonality {
            mount_type: "war_horse".to_string(),
            base_temperament: "brave".to_string(),
            loyalty_capacity: 0.9,
            fear_resistance: 0.8,
            protective_instinct: 0.9,
            intelligence_level: 0.6,
            empathy_sensitivity: 0.7,
            breakdown_threshold: 0.8,
            unique_traits: vec![
                "battle_trained".to_string(),
                "rider_protective".to_string(),
                "noise_resistant".to_string()
            ],
            dread_reactions: {
                let mut reactions = HashMap::new();
                reactions.insert(0, "confident".to_string());
                reactions.insert(1, "alert".to_string());
                reactions.insert(2, "nervous_but_steady".to_string());
                reactions.insert(3, "fearful_but_loyal".to_string());
                reactions.insert(4, "terrified_but_bonded".to_string());
                reactions
            },
        });
        
        // Forest Stag - Wise but easily spooked
        personalities.insert("forest_stag".to_string(), MountPersonality {
            mount_type: "forest_stag".to_string(),
            base_temperament: "noble".to_string(),
            loyalty_capacity: 0.7,
            fear_resistance: 0.4,
            protective_instinct: 0.6,
            intelligence_level: 0.8,
            empathy_sensitivity: 0.9,
            breakdown_threshold: 0.5,
            unique_traits: vec![
                "forest_wisdom".to_string(),
                "supernatural_senses".to_string(),
                "easily_startled".to_string()
            ],
            dread_reactions: {
                let mut reactions = HashMap::new();
                reactions.insert(0, "serene".to_string());
                reactions.insert(1, "sensing_wrongness".to_string());
                reactions.insert(2, "agitated".to_string());
                reactions.insert(3, "panic_prone".to_string());
                reactions.insert(4, "fled_or_catatonic".to_string());
                reactions
            },
        });
        
        // Mountain Wolf - Pack loyal but wild
        personalities.insert("mountain_wolf".to_string(), MountPersonality {
            mount_type: "mountain_wolf".to_string(),
            base_temperament: "wild".to_string(),
            loyalty_capacity: 0.8,
            fear_resistance: 0.6,
            protective_instinct: 0.9,
            intelligence_level: 0.7,
            empathy_sensitivity: 0.6,
            breakdown_threshold: 0.7,
            unique_traits: vec![
                "pack_instinct".to_string(),
                "territorial".to_string(),
                "fierce_loyalty".to_string()
            ],
            dread_reactions: {
                let mut reactions = HashMap::new();
                reactions.insert(0, "alert".to_string());
                reactions.insert(1, "hackles_raised".to_string());
                reactions.insert(2, "growling_protective".to_string());
                reactions.insert(3, "aggressive_defense".to_string());
                reactions.insert(4, "berserker_loyalty".to_string());
                reactions
            },
        });
        
        // Crystal Drake - Magical but fragile bond
        personalities.insert("crystal_drake".to_string(), MountPersonality {
            mount_type: "crystal_drake".to_string(),
            base_temperament: "mystical".to_string(),
            loyalty_capacity: 0.6,
            fear_resistance: 0.5,
            protective_instinct: 0.7,
            intelligence_level: 0.9,
            empathy_sensitivity: 1.0,
            breakdown_threshold: 0.4,
            unique_traits: vec![
                "magical_resonance".to_string(),
                "corruption_sensitive".to_string(),
                "telepathic_bond".to_string()
            ],
            dread_reactions: {
                let mut reactions = HashMap::new();
                reactions.insert(0, "harmonious".to_string());
                reactions.insert(1, "magical_disturbance".to_string());
                reactions.insert(2, "resonance_pain".to_string());
                reactions.insert(3, "magical_feedback".to_string());
                reactions.insert(4, "bond_severed".to_string());
                reactions
            },
        });
        
        personalities
    }
    
    fn generate_bonding_mechanics(&self) -> BondingMechanics {
        let mut trust_building = HashMap::new();
        trust_building.insert("daily_care".to_string(), 0.05);
        trust_building.insert("shared_danger".to_string(), 0.15);
        trust_building.insert("gentle_treatment".to_string(), 0.08);
        trust_building.insert("feeding".to_string(), 0.03);
        trust_building.insert("healing_injuries".to_string(), 0.20);
        trust_building.insert("defending_mount".to_string(), 0.25);
        
        let mut trust_damaging = HashMap::new();
        trust_damaging.insert("harsh_treatment".to_string(), -0.15);
        trust_damaging.insert("abandoning_in_danger".to_string(), -0.40);
        trust_damaging.insert("forcing_into_corruption".to_string(), -0.30);
        trust_damaging.insert("ignoring_distress".to_string(), -0.10);
        trust_damaging.insert("companion_betrayal_witness".to_string(), -0.20);
        
        let mut bond_thresholds = HashMap::new();
        bond_thresholds.insert("stranger".to_string(), 0.0);
        bond_thresholds.insert("acquaintance".to_string(), 0.2);
        bond_thresholds.insert("trusted".to_string(), 0.4);
        bond_thresholds.insert("bonded".to_string(), 0.6);
        bond_thresholds.insert("soul_bonded".to_string(), 0.8);
        
        let bonding_activities = vec![
            BondingActivity {
                activity_type: "grooming".to_string(),
                trust_gain: 0.05,
                time_required: 10.0,
                dread_level_availability: vec![0, 1, 2],
                success_conditions: vec!["mount_calm".to_string(), "safe_environment".to_string()],
            },
            BondingActivity {
                activity_type: "training_together".to_string(),
                trust_gain: 0.10,
                time_required: 30.0,
                dread_level_availability: vec![0, 1, 2],
                success_conditions: vec!["mount_healthy".to_string(), "player_patience".to_string()],
            },
            BondingActivity {
                activity_type: "surviving_danger".to_string(),
                trust_gain: 0.20,
                time_required: 0.0, // Immediate
                dread_level_availability: vec![1, 2, 3, 4],
                success_conditions: vec!["both_survive".to_string(), "player_protective".to_string()],
            },
            BondingActivity {
                activity_type: "sharing_food".to_string(),
                trust_gain: 0.03,
                time_required: 5.0,
                dread_level_availability: vec![0, 1, 2, 3],
                success_conditions: vec!["food_available".to_string()],
            },
        ];
        
        let loyalty_tests = vec![
            LoyaltyTest {
                test_scenario: "companion_betrayal_witnessed".to_string(),
                required_bond_strength: 0.6,
                success_outcomes: vec!["mount_stays_loyal".to_string(), "bond_strengthened".to_string()],
                failure_outcomes: vec!["mount_becomes_wary".to_string(), "trust_damaged".to_string()],
                trauma_impact: 0.3,
            },
            LoyaltyTest {
                test_scenario: "corruption_exposure".to_string(),
                required_bond_strength: 0.4,
                success_outcomes: vec!["mount_trusts_rider".to_string(), "follows_despite_fear".to_string()],
                failure_outcomes: vec!["mount_refuses".to_string(), "attempts_to_flee".to_string()],
                trauma_impact: 0.4,
            },
            LoyaltyTest {
                test_scenario: "dragon_proximity".to_string(),
                required_bond_strength: 0.8,
                success_outcomes: vec!["mount_stands_ground".to_string(), "ultimate_loyalty_proven".to_string()],
                failure_outcomes: vec!["mount_panics".to_string(), "abandons_rider".to_string()],
                trauma_impact: 0.8,
            },
        ];
        
        BondingMechanics {
            initial_trust: 0.1,
            trust_building_factors: trust_building,
            trust_damaging_factors: trust_damaging,
            bond_strength_thresholds: bond_thresholds,
            bonding_activities,
            abandonment_conditions: vec![
                "trust_below_zero".to_string(),
                "severe_trauma".to_string(),
                "corruption_overwhelm".to_string(),
                "rider_moral_betrayal".to_string(),
            ],
            loyalty_tests,
        }
    }
    
    fn generate_protection_abilities(&self) -> HashMap<String, ProtectionAbility> {
        let mut abilities = HashMap::new();
        
        // Environmental protection
        abilities.insert("corruption_resistance".to_string(), ProtectionAbility {
            ability_name: "corruption_resistance".to_string(),
            protection_type: "environmental".to_string(),
            effectiveness: 0.5,
            energy_cost: 0.2,
            cooldown_time: 0.0,
            bond_requirement: 0.3,
            environmental_conditions: vec!["corrupted_areas".to_string()],
            dread_level_scaling: {
                let mut scaling = HashMap::new();
                scaling.insert(0, 1.0);
                scaling.insert(1, 0.9);
                scaling.insert(2, 0.7);
                scaling.insert(3, 0.5);
                scaling.insert(4, 0.3);
                scaling
            },
        });
        
        // Combat protection
        abilities.insert("battle_partnership".to_string(), ProtectionAbility {
            ability_name: "battle_partnership".to_string(),
            protection_type: "combat".to_string(),
            effectiveness: 0.7,
            energy_cost: 0.4,
            cooldown_time: 5.0,
            bond_requirement: 0.5,
            environmental_conditions: vec!["combat_situations".to_string()],
            dread_level_scaling: {
                let mut scaling = HashMap::new();
                scaling.insert(0, 1.0);
                scaling.insert(1, 1.1); // Mounts fight better when alert
                scaling.insert(2, 1.0);
                scaling.insert(3, 0.8);
                scaling.insert(4, 0.6);
                scaling
            },
        });
        
        // Psychological protection
        abilities.insert("companionship_comfort".to_string(), ProtectionAbility {
            ability_name: "companionship_comfort".to_string(),
            protection_type: "psychological".to_string(),
            effectiveness: 0.6,
            energy_cost: 0.1,
            cooldown_time: 0.0,
            bond_requirement: 0.4,
            environmental_conditions: vec!["any".to_string()],
            dread_level_scaling: {
                let mut scaling = HashMap::new();
                scaling.insert(0, 0.3); // Less needed in peace
                scaling.insert(1, 0.6);
                scaling.insert(2, 0.8);
                scaling.insert(3, 1.0);
                scaling.insert(4, 0.9); // Slightly less effective in horror
                scaling
            },
        });
        
        // Navigation assistance
        abilities.insert("pathfinding_aid".to_string(), ProtectionAbility {
            ability_name: "pathfinding_aid".to_string(),
            protection_type: "navigation".to_string(),
            effectiveness: 0.8,
            energy_cost: 0.1,
            cooldown_time: 0.0,
            bond_requirement: 0.2,
            environmental_conditions: vec!["wilderness".to_string(), "unknown_terrain".to_string()],
            dread_level_scaling: {
                let mut scaling = HashMap::new();
                scaling.insert(0, 1.0);
                scaling.insert(1, 1.0);
                scaling.insert(2, 0.9);
                scaling.insert(3, 0.7); // Harder to navigate when scared
                scaling.insert(4, 0.4); // Very unreliable in horror
                scaling
            },
        });
        
        abilities
    }
    
    fn generate_trauma_responses(&self) -> HashMap<String, TraumaResponse> {
        let mut responses = HashMap::new();
        
        responses.insert("witness_companion_death".to_string(), TraumaResponse {
            trauma_trigger: "witness_companion_death".to_string(),
            immediate_reaction: "panic_and_grief".to_string(),
            behavioral_changes: vec![
                "avoids_other_companions".to_string(),
                "hypervigilant".to_string(),
                "protective_of_rider".to_string()
            ],
            performance_impact: -0.4,
            recovery_time: 72.0, // Hours
            recovery_requirements: vec![
                "gentle_care".to_string(),
                "safe_environment".to_string(),
                "time_to_mourn".to_string()
            ],
            permanent_changes: vec!["increased_loyalty".to_string(), "decreased_trust_in_others".to_string()],
        });
        
        responses.insert("corruption_exposure".to_string(), TraumaResponse {
            trauma_trigger: "corruption_exposure".to_string(),
            immediate_reaction: "fear_and_disgust".to_string(),
            behavioral_changes: vec![
                "reluctant_to_enter_corrupted_areas".to_string(),
                "increased_anxiety".to_string(),
                "seeks_cleansing".to_string()
            ],
            performance_impact: -0.2,
            recovery_time: 24.0,
            recovery_requirements: vec![
                "cleansing_ritual".to_string(),
                "pure_environment".to_string(),
                "rider_reassurance".to_string()
            ],
            permanent_changes: vec!["corruption_sensitivity".to_string()],
        });
        
        responses.insert("rider_betrayal".to_string(), TraumaResponse {
            trauma_trigger: "rider_betrayal".to_string(),
            immediate_reaction: "confusion_and_hurt".to_string(),
            behavioral_changes: vec![
                "tests_rider_loyalty".to_string(),
                "slower_to_trust".to_string(),
                "independent_behavior".to_string()
            ],
            performance_impact: -0.6,
            recovery_time: 168.0, // A week
            recovery_requirements: vec![
                "consistent_good_treatment".to_string(),
                "proof_of_loyalty".to_string(),
                "time_to_rebuild_trust".to_string()
            ],
            permanent_changes: vec![
                "trust_issues".to_string(),
                "conditional_loyalty".to_string()
            ],
        });
        
        responses.insert("dragon_encounter".to_string(), TraumaResponse {
            trauma_trigger: "dragon_encounter".to_string(),
            immediate_reaction: "primal_terror".to_string(),
            behavioral_changes: vec![
                "startles_at_loud_noises".to_string(),
                "refuses_certain_areas".to_string(),
                "clings_to_rider".to_string()
            ],
            performance_impact: -0.8,
            recovery_time: 336.0, // Two weeks
            recovery_requirements: vec![
                "complete_safety".to_string(),
                "constant_reassurance".to_string(),
                "gradual_re_exposure".to_string()
            ],
            permanent_changes: vec![
                "dragon_phobia".to_string(),
                "heightened_protective_instincts".to_string()
            ],
        });
        
        responses
    }
    
    fn generate_environmental_adaptations(&self, world_corruption: f32) -> Vec<EnvironmentalAdaptation> {
        vec![
            EnvironmentalAdaptation {
                environment_type: "forest".to_string(),
                adaptation_bonus: 0.3,
                special_abilities: vec!["stealth".to_string(), "foraging".to_string()],
                stress_factors: vec!["corruption".to_string(), "unnatural_sounds".to_string()],
                comfort_level: 1.0 - world_corruption * 0.5,
            },
            EnvironmentalAdaptation {
                environment_type: "mountains".to_string(),
                adaptation_bonus: 0.4,
                special_abilities: vec!["sure_footing".to_string(), "cold_resistance".to_string()],
                stress_factors: vec!["dragon_proximity".to_string(), "avalanche_risk".to_string()],
                comfort_level: 0.8 - world_corruption * 0.3,
            },
            EnvironmentalAdaptation {
                environment_type: "swamp".to_string(),
                adaptation_bonus: 0.2,
                special_abilities: vec!["disease_resistance".to_string(), "water_navigation".to_string()],
                stress_factors: vec![
                    "corruption_concentration".to_string(), 
                    "toxic_fumes".to_string(),
                    "false_lights".to_string()
                ],
                comfort_level: 0.5 - world_corruption * 0.8,
            },
            EnvironmentalAdaptation {
                environment_type: "village".to_string(),
                adaptation_bonus: 0.1,
                special_abilities: vec!["social_navigation".to_string(), "resource_finding".to_string()],
                stress_factors: vec![
                    "panicked_npcs".to_string(),
                    "economic_collapse".to_string(),
                    "social_unrest".to_string()
                ],
                comfort_level: 0.7 - world_corruption * 0.6,
            },
            EnvironmentalAdaptation {
                environment_type: "labyrinth".to_string(),
                adaptation_bonus: -0.5, // Penalty - unnatural environment
                special_abilities: vec![], // No special abilities here
                stress_factors: vec![
                    "reality_distortion".to_string(),
                    "dragon_presence".to_string(),
                    "psychological_pressure".to_string(),
                    "impossible_geometry".to_string()
                ],
                comfort_level: 0.1, // Always very uncomfortable
            },
        ]
    }
}
