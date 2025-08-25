//! Sentimental Item & Forge System - ECS Resources
//!
//! Production-ready resources for managing sentimental items, forge trials, and dual-path
//! morality system with full database integration using existing forge database models.

use bevy::prelude::*;
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait};
use database_orm::{forge, players};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;
use crate::systems::forge::components::*;

/// Master forge system state with full database backing
#[derive(Resource, Debug)]
pub struct ForgeSystemState {
    pub db: DatabaseConnection,
    pub active_players: HashMap<Uuid, PlayerForgeData>,
    pub sentimental_items_cache: HashMap<Uuid, forge::Model>, // Cache of sentimental items
    pub forge_progress_cache: HashMap<Uuid, forge::forge_progress::Model>, // Cache of forge progress
    pub active_trials: HashMap<String, Entity>, // Active forge trials
    pub forge_masters: HashMap<String, ForgeMaster>, // Available forge masters
    pub system_integration_hooks: HashMap<String, SystemIntegrationHook>, // How forge integrates with other systems
}

/// Player forge data loaded from database
#[derive(Debug, Clone)]
pub struct PlayerForgeData {
    pub player_entity: Entity,
    pub forge_progress: forge::forge_progress::Model,
    pub sentimental_items: Vec<forge::Model>,
    pub reagent_power_analysis: ReagentPowerAnalysis,
    pub current_forge_session: Option<ActiveForgeSession>,
    pub forge_readiness_assessment: ForgeReadinessAssessment,
}

/// Analysis of player's reagent power for forge paths
#[derive(Debug, Clone)]
pub struct ReagentPowerAnalysis {
    pub total_essence_power: f32,   // Total light path power available
    pub total_blood_power: f32,     // Total dark path power available
    pub optimal_path: Option<ForgePath>, // Optimal path based on reagents
    pub power_quality_scores: HashMap<String, f32>, // Quality scores by category
    pub reagent_synergies: Vec<ReagentSynergy>, // Detected synergies between reagents
    pub sacrifice_resistance_total: f32, // Total emotional resistance to overcome
}

/// Synergy between reagents
#[derive(Debug, Clone)]
pub struct ReagentSynergy {
    pub synergy_name: String,
    pub involved_reagents: Vec<Uuid>, // Reagents involved in synergy
    pub synergy_bonus: f32,         // Bonus power from synergy
    pub activation_requirements: Vec<String>, // Requirements to activate synergy
    pub emotional_coherence: f32,   // How emotionally coherent the synergy is
}

/// Active forge session in progress
#[derive(Debug, Clone)]
pub struct ActiveForgeSession {
    pub session_id: Uuid,
    pub forge_type: String,         // "light_forge", "dark_forge", "neutral_forge"
    pub target_gear: String,        // What gear is being forged
    pub start_time: i64,            // Session start timestamp
    pub estimated_duration: f32,    // Estimated session duration
    pub active_reagents: Vec<Uuid>, // Reagents being consumed
    pub participating_companions: Vec<Uuid>, // Companions helping
    pub current_complications: Vec<String>, // Current problems/complications
    pub progress_milestones: Vec<ProgressMilestone>, // Milestones within session
}

/// Milestone within forge session
#[derive(Debug, Clone)]
pub struct ProgressMilestone {
    pub milestone_name: String,
    pub completion_timestamp: Option<i64>, // When milestone was completed
    pub required_for_success: bool, // Is this milestone required?
    pub skill_demonstrated: Option<String>, // Skill demonstrated by completing milestone
    pub emotional_significance: f32, // Emotional significance of milestone
}

/// Assessment of player's readiness for forge use
#[derive(Debug, Clone)]
pub struct ForgeReadinessAssessment {
    pub overall_readiness: f32,     // 0.0-1.0 overall readiness
    pub light_path_readiness: f32,  // Readiness for light path specifically
    pub dark_path_readiness: f32,   // Readiness for dark path specifically
    pub missing_requirements: Vec<String>, // What's still needed
    pub recommended_preparations: Vec<String>, // Recommended next steps
    pub estimated_time_to_readiness: Option<f32>, // Time estimate to full readiness
    pub risk_factors: Vec<String>,  // Factors that could cause failure
    pub success_probability: f32,   // 0.0-1.0 estimated success probability
}

/// Forge master character with specific expertise
#[derive(Debug, Clone)]
pub struct ForgeMaster {
    pub master_id: String,          // "high_elf_master", "cursed_master", "neutral_master"
    pub master_type: String,        // Type of forge master
    pub path_alignment: ForgePath,  // Which path this master represents
    pub skill_level: f32,           // 0.0-1.0 master's skill level
    pub teaching_ability: f32,      // 0.0-1.0 ability to teach others
    pub approval_requirements: Vec<ApprovalRequirement>, // What master requires for approval
    pub available_techniques: Vec<ForgingTechnique>, // Techniques master can teach
    pub personality_traits: Vec<String>, // Master's personality
    pub dialogue_options: Vec<String>, // Available dialogue with master
    pub location: String,           // Where to find this master
}

/// Requirement for forge master approval
#[derive(Debug, Clone)]
pub struct ApprovalRequirement {
    pub requirement_type: String,   // Type of requirement
    pub requirement_description: String, // Detailed description
    pub current_progress: f32,      // 0.0-1.0 current progress toward requirement
    pub completion_methods: Vec<String>, // Ways to complete this requirement
    pub difficulty_level: f32,      // 0.0-1.0 difficulty of completing
}

/// Forging technique available from master
#[derive(Debug, Clone)]
pub struct ForgingTechnique {
    pub technique_name: String,
    pub technique_type: String,     // "essence_infusion", "blood_binding", "memory_weaving"
    pub power_multiplier: f32,      // How much this technique amplifies power
    pub skill_requirement: f32,     // Required skill to use technique
    pub reagent_efficiency: f32,    // How efficiently technique uses reagents
    pub unique_effects: Vec<String>, // Special effects of this technique
    pub learning_time: f32,         // Time to learn technique (seconds)
    pub mastery_benefits: Vec<String>, // Benefits of mastering technique
}

/// Hook for integrating forge system with other systems
#[derive(Debug, Clone)]
pub struct SystemIntegrationHook {
    pub target_system: String,     // Which system to integrate with
    pub integration_points: Vec<IntegrationPoint>, // Specific integration points
    pub data_sharing_protocol: String, // How data is shared
    pub event_triggers: Vec<String>, // Events that trigger integration
    pub feedback_mechanisms: Vec<String>, // How systems provide feedback
}

/// Specific point of integration between systems
#[derive(Debug, Clone)]
pub struct IntegrationPoint {
    pub integration_name: String,
    pub trigger_conditions: Vec<String>, // When integration activates
    pub data_exchanged: Vec<String>, // What data is shared
    pub system_modifications: Vec<String>, // How target system is modified
    pub success_metrics: Vec<String>, // How to measure integration success
}

impl ForgeSystemState {
    pub async fn new(db: DatabaseConnection) -> Result<Self, Box<dyn std::error::Error>> {
        // Initialize forge masters
        let mut forge_masters = HashMap::new();
        
        // High Elf Forge Master (Light Path)
        forge_masters.insert("high_elf_master".to_string(), ForgeMaster {
            master_id: "high_elf_master".to_string(),
            master_type: "light_path_master".to_string(),
            path_alignment: ForgePath::Light,
            skill_level: 0.95,
            teaching_ability: 0.9,
            approval_requirements: vec![
                ApprovalRequirement {
                    requirement_type: "compassionate_choices".to_string(),
                    requirement_description: "Demonstrate consistent compassion in difficult situations".to_string(),
                    current_progress: 0.0,
                    completion_methods: vec![
                        "Help companions without expecting reward".to_string(),
                        "Choose merciful options in combat".to_string(),
                        "Prioritize healing over harm".to_string(),
                    ],
                    difficulty_level: 0.7,
                },
                ApprovalRequirement {
                    requirement_type: "essence_mastery".to_string(),
                    requirement_description: "Master essence-based forging techniques".to_string(),
                    current_progress: 0.0,
                    completion_methods: vec![
                        "Complete essence manipulation trials".to_string(),
                        "Successfully forge essence-based items".to_string(),
                    ],
                    difficulty_level: 0.8,
                },
            ],
            available_techniques: vec![
                ForgingTechnique {
                    technique_name: "Essence Infusion".to_string(),
                    technique_type: "essence_manipulation".to_string(),
                    power_multiplier: 1.5,
                    skill_requirement: 0.6,
                    reagent_efficiency: 0.9,
                    unique_effects: vec!["Painless sacrifice".to_string(), "Memory preservation".to_string()],
                    learning_time: 3600.0, // 1 hour
                    mastery_benefits: vec!["Increased reagent efficiency".to_string()],
                },
                ForgingTechnique {
                    technique_name: "Memory Crystallization".to_string(),
                    technique_type: "memory_preservation".to_string(),
                    power_multiplier: 1.3,
                    skill_requirement: 0.8,
                    reagent_efficiency: 0.85,
                    unique_effects: vec!["Preserve memories in gear".to_string()],
                    learning_time: 7200.0, // 2 hours
                    mastery_benefits: vec!["Enhanced memory preservation".to_string()],
                },
            ],
            personality_traits: vec!["wise".to_string(), "patient".to_string(), "compassionate".to_string()],
            dialogue_options: vec![
                "Ask about essence manipulation".to_string(),
                "Request approval assessment".to_string(),
                "Learn about light path philosophy".to_string(),
            ],
            location: "high_elf_sanctuary".to_string(),
        });
        
        // Cursed Forge Master (Dark Path)
        forge_masters.insert("cursed_master".to_string(), ForgeMaster {
            master_id: "cursed_master".to_string(),
            master_type: "dark_path_master".to_string(),
            path_alignment: ForgePath::Dark,
            skill_level: 0.98, // Higher skill but more dangerous
            teaching_ability: 0.7, // Less willing to teach
            approval_requirements: vec![
                ApprovalRequirement {
                    requirement_type: "ruthless_pragmatism".to_string(),
                    requirement_description: "Demonstrate willingness to make hard choices for power".to_string(),
                    current_progress: 0.0,
                    completion_methods: vec![
                        "Sacrifice valuable things for greater power".to_string(),
                        "Choose efficient solutions over comfortable ones".to_string(),
                        "Demonstrate understanding of necessary suffering".to_string(),
                    ],
                    difficulty_level: 0.9,
                },
                ApprovalRequirement {
                    requirement_type: "blood_mastery".to_string(),
                    requirement_description: "Master blood-based forging techniques".to_string(),
                    current_progress: 0.0,
                    completion_methods: vec![
                        "Complete blood manipulation trials".to_string(),
                        "Successfully forge blood-based items".to_string(),
                        "Demonstrate pain tolerance".to_string(),
                    ],
                    difficulty_level: 0.95,
                },
            ],
            available_techniques: vec![
                ForgingTechnique {
                    technique_name: "Blood Binding".to_string(),
                    technique_type: "blood_manipulation".to_string(),
                    power_multiplier: 2.0, // More powerful but painful
                    skill_requirement: 0.7,
                    reagent_efficiency: 0.8, // Less efficient due to waste
                    unique_effects: vec!["Painful sacrifice".to_string(), "Increased power".to_string()],
                    learning_time: 1800.0, // 30 minutes (faster but harder)
                    mastery_benefits: vec!["Pain resistance".to_string()],
                },
                ForgingTechnique {
                    technique_name: "Soul Scarring".to_string(),
                    technique_type: "permanent_enhancement".to_string(),
                    power_multiplier: 2.5,
                    skill_requirement: 0.9,
                    reagent_efficiency: 0.7,
                    unique_effects: vec!["Permanent power increase".to_string(), "Emotional scarring".to_string()],
                    learning_time: 10800.0, // 3 hours
                    mastery_benefits: vec!["Resistance to emotional damage".to_string()],
                },
            ],
            personality_traits: vec!["cunning".to_string(), "demanding".to_string(), "powerful".to_string()],
            dialogue_options: vec![
                "Ask about blood manipulation".to_string(),
                "Request power assessment".to_string(),
                "Learn about dark path philosophy".to_string(),
                "Inquire about pain tolerance training".to_string(),
            ],
            location: "cursed_forge_chamber".to_string(),
        });
        
        // Initialize system integration hooks
        let mut integration_hooks = HashMap::new();
        
        // Integration with companion psychology system
        integration_hooks.insert("companion_psychology".to_string(), SystemIntegrationHook {
            target_system: "companion_psychology".to_string(),
            integration_points: vec![
                IntegrationPoint {
                    integration_name: "sacrifice_trauma_integration".to_string(),
                    trigger_conditions: vec!["companion_sacrifice_offered".to_string()],
                    data_exchanged: vec!["trauma_level".to_string(), "sacrifice_resistance".to_string()],
                    system_modifications: vec!["trauma_accumulation".to_string()],
                    success_metrics: vec!["trauma_handled_appropriately".to_string()],
                },
                IntegrationPoint {
                    integration_name: "forge_therapy_integration".to_string(),
                    trigger_conditions: vec!["forge_session_active".to_string()],
                    data_exchanged: vec!["emotional_state".to_string(), "therapy_progress".to_string()],
                    system_modifications: vec!["forge_success_probability".to_string()],
                    success_metrics: vec!["positive_therapeutic_outcome".to_string()],
                },
            ],
            data_sharing_protocol: "event_driven".to_string(),
            event_triggers: vec!["forge_trial_start".to_string(), "sacrifice_offered".to_string()],
            feedback_mechanisms: vec!["trauma_event_feedback".to_string(), "therapy_progress_feedback".to_string()],
        });
        
        // Integration with dread progression system
        integration_hooks.insert("dread_progression".to_string(), SystemIntegrationHook {
            target_system: "dread_progression".to_string(),
            integration_points: vec![
                IntegrationPoint {
                    integration_name: "forge_dread_interaction".to_string(),
                    trigger_conditions: vec!["forge_trial_active".to_string(), "dread_level_change".to_string()],
                    data_exchanged: vec!["current_dread_level".to_string(), "forge_difficulty".to_string()],
                    system_modifications: vec!["trial_success_probability".to_string()],
                    success_metrics: vec!["trial_completion_rate".to_string()],
                },
            ],
            data_sharing_protocol: "continuous_monitoring".to_string(),
            event_triggers: vec!["dread_level_change".to_string()],
            feedback_mechanisms: vec!["forge_difficulty_adjustment".to_string()],
        });
        
        Ok(Self {
            db,
            active_players: HashMap::new(),
            sentimental_items_cache: HashMap::new(),
            forge_progress_cache: HashMap::new(),
            active_trials: HashMap::new(),
            forge_masters,
            system_integration_hooks: integration_hooks,
        })
    }
    
    /// Load player forge data from database
    pub async fn load_player_forge_data(&mut self, player_id: Uuid) -> Result<Option<PlayerForgeData>, Box<dyn std::error::Error>> {
        // Load forge progress
        let forge_progress = forge::forge_progress::Entity::find()
            .filter(forge::forge_progress::Column::PlayerId.eq(player_id))
            .one(&self.db)
            .await?;
        
        let forge_progress = match forge_progress {
            Some(fp) => fp,
            None => {
                // Create default forge progress for new player
                let default_progress = forge::forge_progress::ActiveModel {
                    id: sea_orm::Set(Uuid::new_v4()),
                    player_id: sea_orm::Set(player_id),
                    chosen_forge_path: sea_orm::Set(None),
                    path_commitment_level: sea_orm::Set(0.0),
                    can_still_switch_paths: sea_orm::Set(true),
                    trials_completed: sea_orm::Set(serde_json::Value::Array(vec![])),
                    trial_scores: sea_orm::Set(serde_json::Value::Object(serde_json::Map::new())),
                    current_trial: sea_orm::Set(None),
                    trial_failures: sea_orm::Set(serde_json::Value::Array(vec![])),
                    readiness_score: sea_orm::Set(0.0),
                    missing_requirements: sea_orm::Set(serde_json::Value::Array(vec![])),
                    forge_master_approval: sea_orm::Set(false),
                    created_at: sea_orm::Set(chrono::Utc::now()),
                    updated_at: sea_orm::Set(chrono::Utc::now()),
                    ..Default::default()
                };
                
                default_progress.insert(&self.db).await?
            }
        };
        
        // Load sentimental items
        let sentimental_items = forge::Entity::find()
            .filter(forge::Column::PlayerId.eq(player_id))
            .all(&self.db)
            .await?;
        
        // Analyze reagent power
        let reagent_analysis = self.analyze_reagent_power(&sentimental_items)?;
        
        // Assess forge readiness
        let readiness_assessment = self.assess_forge_readiness(&forge_progress, &reagent_analysis)?;
        
        let player_data = PlayerForgeData {
            player_entity: Entity::PLACEHOLDER, // Would be set when creating ECS entity
            forge_progress,
            sentimental_items: sentimental_items.clone(),
            reagent_power_analysis: reagent_analysis,
            current_forge_session: None,
            forge_readiness_assessment: readiness_assessment,
        };
        
        // Cache the data
        self.forge_progress_cache.insert(player_id, player_data.forge_progress.clone());
        for item in &sentimental_items {
            self.sentimental_items_cache.insert(item.id, item.clone());
        }
        self.active_players.insert(player_id, player_data.clone());
        
        Ok(Some(player_data))
    }
    
    /// Analyze reagent power from sentimental items
    fn analyze_reagent_power(&self, items: &[forge::Model]) -> Result<ReagentPowerAnalysis, Box<dyn std::error::Error>> {
        let mut total_essence_power = 0.0;
        let mut total_blood_power = 0.0;
        let mut quality_scores = HashMap::new();
        let mut reagent_synergies = Vec::new();
        let mut sacrifice_resistance_total = 0.0;
        
        for item in items {
            total_essence_power += item.light_path_compatibility * item.forge_reagent_power;
            total_blood_power += item.dark_path_compatibility * item.forge_reagent_power;
            sacrifice_resistance_total += item.sacrifice_resistance;
            
            // Track quality by category
            let current_quality = quality_scores.get(&item.sentimental_category).copied().unwrap_or(0.0);
            quality_scores.insert(
                item.sentimental_category.clone(),
                current_quality + item.emotional_weight
            );
        }
        
        // Determine optimal path
        let optimal_path = if total_essence_power > total_blood_power * 1.2 {
            Some(ForgePath::Light)
        } else if total_blood_power > total_essence_power * 1.2 {
            Some(ForgePath::Dark)
        } else {
            Some(ForgePath::Balanced) // If powers are close, balanced might be optimal
        };
        
        // Detect reagent synergies (items that work well together)
        reagent_synergies = self.detect_reagent_synergies(items)?;
        
        Ok(ReagentPowerAnalysis {
            total_essence_power,
            total_blood_power,
            optimal_path,
            power_quality_scores: quality_scores,
            reagent_synergies,
            sacrifice_resistance_total,
        })
    }
    
    /// Detect synergies between sentimental items
    fn detect_reagent_synergies(&self, items: &[forge::Model]) -> Result<Vec<ReagentSynergy>, Box<dyn std::error::Error>> {
        let mut synergies = Vec::new();
        
        // Group items by category to find synergies
        let mut category_groups: HashMap<String, Vec<&forge::Model>> = HashMap::new();
        for item in items {
            category_groups.entry(item.sentimental_category.clone())
                .or_insert_with(Vec::new)
                .push(item);
        }
        
        // Look for synergies within categories
        for (category, category_items) in &category_groups {
            if category_items.len() >= 2 {
                // Multiple items of same category create synergy
                let total_power: f32 = category_items.iter().map(|item| item.forge_reagent_power).sum();
                let average_weight: f32 = category_items.iter().map(|item| item.emotional_weight).sum::<f32>() / category_items.len() as f32;
                
                let synergy_bonus = (total_power * 0.2) * average_weight; // Bonus based on emotional coherence
                
                if synergy_bonus > 0.1 {
                    synergies.push(ReagentSynergy {
                        synergy_name: format!("{}_resonance", category),
                        involved_reagents: category_items.iter().map(|item| item.id).collect(),
                        synergy_bonus,
                        activation_requirements: vec![
                            format!("Use all {} items together", category),
                            "Maintain emotional coherence".to_string(),
                        ],
                        emotional_coherence: average_weight,
                    });
                }
            }
        }
        
        // Look for cross-category synergies (complementary emotions)
        if let (Some(love_items), Some(loss_items)) = (
            category_groups.get("love"),
            category_groups.get("loss")
        ) {
            if !love_items.is_empty() && !loss_items.is_empty() {
                synergies.push(ReagentSynergy {
                    synergy_name: "bittersweet_resonance".to_string(),
                    involved_reagents: love_items.iter().chain(loss_items.iter()).map(|item| item.id).collect(),
                    synergy_bonus: 0.5, // Powerful emotional synergy
                    activation_requirements: vec![
                        "Combine love and loss items".to_string(),
                        "Accept bittersweet nature of memory".to_string(),
                    ],
                    emotional_coherence: 0.9, // Very coherent combination
                });
            }
        }
        
        Ok(synergies)
    }
    
    /// Assess player's readiness for forge use
    fn assess_forge_readiness(&self, forge_progress: &forge::forge_progress::Model, reagent_analysis: &ReagentPowerAnalysis) -> Result<ForgeReadinessAssessment, Box<dyn std::error::Error>> {
        let mut readiness_factors = Vec::new();
        let mut missing_requirements = Vec::new();
        let mut risk_factors = Vec::new();
        
        // Check trial completion
        let trials_completed: Vec<serde_json::Value> = serde_json::from_value(forge_progress.trials_completed.clone())
            .unwrap_or_default();
        let trial_completion_score = trials_completed.len() as f32 / 10.0; // Assume 10 trials total
        readiness_factors.push(trial_completion_score.min(1.0));
        
        if trial_completion_score < 0.8 {
            missing_requirements.push("Complete more forge trials".to_string());
        }
        
        // Check reagent power sufficiency
        let min_required_power = 10.0; // Minimum power for forge use
        let reagent_power_score = (reagent_analysis.total_essence_power + reagent_analysis.total_blood_power) / min_required_power;
        readiness_factors.push(reagent_power_score.min(1.0));
        
        if reagent_power_score < 1.0 {
            missing_requirements.push("Collect more powerful sentimental items".to_string());
        }
        
        // Check forge master approval
        let master_approval_score = if forge_progress.forge_master_approval { 1.0 } else { 0.0 };
        readiness_factors.push(master_approval_score);
        
        if !forge_progress.forge_master_approval {
            missing_requirements.push("Gain forge master approval".to_string());
        }
        
        // Check path commitment
        let path_commitment_score = forge_progress.path_commitment_level;
        readiness_factors.push(path_commitment_score);
        
        if path_commitment_score < 0.7 {
            missing_requirements.push("Demonstrate stronger path commitment".to_string());
        }
        
        // Identify risk factors
        if reagent_analysis.sacrifice_resistance_total > 5.0 {
            risk_factors.push("High emotional resistance to sacrifice".to_string());
        }
        
        if forge_progress.path_commitment_level < 0.5 && forge_progress.can_still_switch_paths {
            risk_factors.push("Uncertain path commitment".to_string());
        }
        
        // Calculate overall readiness
        let overall_readiness = readiness_factors.iter().sum::<f32>() / readiness_factors.len() as f32;
        
        // Calculate path-specific readiness
        let light_path_readiness = if reagent_analysis.optimal_path == Some(ForgePath::Light) {
            overall_readiness * 1.2
        } else {
            overall_readiness * 0.8
        }.min(1.0);
        
        let dark_path_readiness = if reagent_analysis.optimal_path == Some(ForgePath::Dark) {
            overall_readiness * 1.2
        } else {
            overall_readiness * 0.8
        }.min(1.0);
        
        // Estimate time to readiness
        let estimated_time = if overall_readiness >= 0.9 {
            Some(0.0) // Ready now
        } else if overall_readiness >= 0.7 {
            Some(3600.0 * (1.0 - overall_readiness) * 10.0) // Hours to readiness
        } else {
            None // Too far from readiness to estimate
        };
        
        Ok(ForgeReadinessAssessment {
            overall_readiness,
            light_path_readiness,
            dark_path_readiness,
            missing_requirements,
            recommended_preparations: vec![
                if trial_completion_score < 0.8 {
                    "Focus on completing forge trials".to_string()
                } else {
                    "Maintain trial mastery".to_string()
                },
                if reagent_power_score < 1.0 {
                    "Collect more emotionally significant items".to_string()
                } else {
                    "Optimize reagent selection".to_string()
                },
                if !forge_progress.forge_master_approval {
                    "Work on gaining forge master approval".to_string()
                } else {
                    "Maintain forge master relationship".to_string()
                },
            ],
            estimated_time_to_readiness: estimated_time,
            risk_factors,
            success_probability: overall_readiness * 0.8, // Conservative estimate
        })
    }
    
    /// Save forge progress to database
    pub async fn save_forge_progress(&self, player_id: Uuid, player_data: &PlayerForgeData) -> Result<(), Box<dyn std::error::Error>> {
        use sea_orm::*;
        
        // Update forge progress
        let mut forge_progress: forge::forge_progress::ActiveModel = player_data.forge_progress.clone().into();
        forge_progress.updated_at = Set(chrono::Utc::now());
        forge_progress.update(&self.db).await?;
        
        // Update sentimental items
        for item in &player_data.sentimental_items {
            let mut sentimental_item: forge::ActiveModel = item.clone().into();
            sentimental_item.updated_at = Set(chrono::Utc::now());
            sentimental_item.update(&self.db).await?;
        }
        
        Ok(())
    }
    
    /// Get forge master by path
    pub fn get_forge_master(&self, path: &ForgePath) -> Option<&ForgeMaster> {
        match path {
            ForgePath::Light => self.forge_masters.get("high_elf_master"),
            ForgePath::Dark => self.forge_masters.get("cursed_master"),
            ForgePath::Balanced => self.forge_masters.get("neutral_master"), // Would add if implemented
        }
    }
}

/// Configuration for forge system mechanics
#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct ForgeSystemConfig {
    // Basic forge mechanics
    pub min_reagent_power_for_trials: f32, // Minimum power needed for trials
    pub min_reagent_power_for_ultimate: f32, // Minimum power for ultimate forge
    pub sacrifice_resistance_threshold: f32, // Max resistance before requiring special handling
    
    // Path-specific configuration
    pub light_path_efficiency_bonus: f32, // Efficiency bonus for light path
    pub dark_path_power_bonus: f32,       // Power bonus for dark path
    pub balanced_path_difficulty_multiplier: f32, // Difficulty multiplier for balanced path
    
    // Trial configuration
    pub forge_trial_types: Vec<ForgeTrialConfig>, // Available trial types
    pub trial_difficulty_scaling: [f32; 5], // Difficulty by player progression level
    pub trial_system_integration: HashMap<String, f32>, // How trials integrate with systems
    
    // Second chances system
    pub max_second_chances: u8,           // Maximum second chances possible
    pub second_chance_sources: Vec<SecondChanceSourceConfig>, // Sources of second chances
    pub second_chance_power_scaling: f32, // How second chances scale with usage
    
    // Mythic gear configuration
    pub gear_power_scaling: Vec<f32>,     // Power scaling for gear levels
    pub gear_evolution_thresholds: Vec<f32>, // Thresholds for gear evolution
    pub synergy_bonus_multipliers: HashMap<String, f32>, // Bonuses for gear synergies
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForgeTrialConfig {
    pub trial_type: String,
    pub required_systems: Vec<String>, // Systems this trial tests
    pub base_difficulty: f32,
    pub success_criteria: Vec<String>,
    pub rewards: Vec<String>,
    pub failure_consequences: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecondChanceSourceConfig {
    pub source_type: String,
    pub acquisition_method: String,
    pub power_level: f32,
    pub rarity: f32,
    pub requirements: Vec<String>,
}

impl Default for ForgeSystemConfig {
    fn default() -> Self {
        let forge_trial_types = vec![
            ForgeTrialConfig {
                trial_type: "hex_navigation_mastery".to_string(),
                required_systems: vec!["hex_rendering".to_string(), "movement".to_string()],
                base_difficulty: 0.6,
                success_criteria: vec!["Navigate complex hex maze".to_string(), "Reach target in time limit".to_string()],
                rewards: vec!["Navigation mastery".to_string(), "Spatial awareness bonus".to_string()],
                failure_consequences: vec!["Disorientation".to_string(), "Time penalty".to_string()],
            },
            ForgeTrialConfig {
                trial_type: "mounted_combat_excellence".to_string(),
                required_systems: vec!["combat".to_string(), "mount_system".to_string()],
                base_difficulty: 0.7,
                success_criteria: vec!["Defeat enemies while mounted".to_string(), "Protect mount from harm".to_string()],
                rewards: vec!["Combat mastery".to_string(), "Mount bond strengthening".to_string()],
                failure_consequences: vec!["Mount trauma".to_string(), "Combat effectiveness reduction".to_string()],
            },
            ForgeTrialConfig {
                trial_type: "first_person_dungeon_mastery".to_string(),
                required_systems: vec!["dungeon_3d".to_string(), "sound_navigation".to_string()],
                base_difficulty: 0.8,
                success_criteria: vec!["Navigate 3D dungeon".to_string(), "Solve spatial puzzles".to_string()],
                rewards: vec!["Spatial mastery".to_string(), "3D navigation skill".to_string()],
                failure_consequences: vec!["Spatial disorientation".to_string(), "Claustrophobia".to_string()],
            },
            ForgeTrialConfig {
                trial_type: "party_coordination_excellence".to_string(),
                required_systems: vec!["companion_psychology".to_string(), "group_dynamics".to_string()],
                base_difficulty: 0.9,
                success_criteria: vec!["Lead party through crisis".to_string(), "Maintain party cohesion".to_string()],
                rewards: vec!["Leadership mastery".to_string(), "Party synergy bonus".to_string()],
                failure_consequences: vec!["Party discord".to_string(), "Leadership confidence loss".to_string()],
            },
        ];
        
        let second_chance_sources = vec![
            SecondChanceSourceConfig {
                source_type: "companion_ultimate_sacrifice".to_string(),
                acquisition_method: "Companion offers their life for player's second chance".to_string(),
                power_level: 1.0,
                rarity: 0.1, // Very rare
                requirements: vec!["Maximum companion loyalty".to_string(), "Life-threatening situation".to_string()],
            },
            SecondChanceSourceConfig {
                source_type: "perfect_trial_completion".to_string(),
                acquisition_method: "Complete forge trial with perfect score".to_string(),
                power_level: 0.5,
                rarity: 0.3,
                requirements: vec!["100% trial completion".to_string(), "No mistakes".to_string()],
            },
            SecondChanceSourceConfig {
                source_type: "forge_master_gift".to_string(),
                acquisition_method: "Forge master grants second chance for exceptional demonstration".to_string(),
                power_level: 0.7,
                rarity: 0.2,
                requirements: vec!["Exceptional forge master approval".to_string(), "Unique achievement".to_string()],
            },
        ];
        
        Self {
            min_reagent_power_for_trials: 5.0,
            min_reagent_power_for_ultimate: 15.0,
            sacrifice_resistance_threshold: 8.0,
            light_path_efficiency_bonus: 0.2,
            dark_path_power_bonus: 0.3,
            balanced_path_difficulty_multiplier: 1.5,
            forge_trial_types,
            trial_difficulty_scaling: [0.5, 0.7, 0.8, 0.9, 1.0],
            trial_system_integration: {
                let mut integration = HashMap::new();
                integration.insert("combat".to_string(), 0.8);
                integration.insert("hex_rendering".to_string(), 0.7);
                integration.insert("companion_psychology".to_string(), 0.9);
                integration.insert("dread_progression".to_string(), 0.6);
                integration
            },
            max_second_chances: 3,
            second_chance_sources,
            second_chance_power_scaling: 0.8, // Each use reduces power
            gear_power_scaling: vec![1.0, 1.5, 2.2, 3.2, 4.5, 6.0], // Exponential scaling
            gear_evolution_thresholds: vec![10.0, 25.0, 50.0, 100.0, 200.0], // Power needed for evolution
            synergy_bonus_multipliers: {
                let mut multipliers = HashMap::new();
                multipliers.insert("emotional_coherence".to_string(), 1.3);
                multipliers.insert("path_alignment".to_string(), 1.2);
                multipliers.insert("memory_resonance".to_string(), 1.4);
                multipliers.insert("companion_bond".to_string(), 1.1);
                multipliers
            },
        }
    }
}

/// Resource for managing active forge sessions
#[derive(Resource, Debug, Default)]
pub struct ForgeSessionManager {
    pub active_sessions: HashMap<Uuid, ActiveForgeSession>,
    pub session_templates: HashMap<String, ForgeSessionTemplate>,
    pub completed_sessions: Vec<CompletedForgeSession>,
    pub session_analytics: ForgeSessionAnalytics,
}

#[derive(Debug, Clone)]
pub struct ForgeSessionTemplate {
    pub template_name: String,
    pub forge_type: String,         // Type of forge session
    pub estimated_duration: f32,    // Estimated session length
    pub required_reagent_categories: Vec<String>, // Required reagent types
    pub companion_roles: HashMap<String, String>, // Roles companions can play
    pub success_factors: Vec<String>, // Factors that contribute to success
    pub common_complications: Vec<String>, // Typical problems that arise
}

#[derive(Debug, Clone, Default)]
pub struct ForgeSessionAnalytics {
    pub total_sessions_attempted: usize,
    pub successful_sessions: usize,
    pub average_session_duration: f32,
    pub most_common_failure_reasons: Vec<String>,
    pub most_effective_techniques: Vec<String>,
    pub reagent_efficiency_statistics: HashMap<String, f32>,
}

impl ForgeSessionManager {
    pub fn new() -> Self {
        let mut session_templates = HashMap::new();
        
        // Light path forge session template
        session_templates.insert("light_forge_session".to_string(), ForgeSessionTemplate {
            template_name: "Light Path Forging".to_string(),
            forge_type: "essence_based".to_string(),
            estimated_duration: 3600.0, // 1 hour
            required_reagent_categories: vec!["love".to_string(), "hope".to_string(), "friendship".to_string()],
            companion_roles: {
                let mut roles = HashMap::new();
                roles.insert("emotional_support".to_string(), "Provide emotional stability".to_string());
                roles.insert("essence_channeling".to_string(), "Help channel essence energy".to_string());
                roles.insert("memory_keeper".to_string(), "Preserve important memories".to_string());
                roles
            },
            success_factors: vec!["Emotional stability".to_string(), "Clear intention".to_string(), "Compassionate focus".to_string()],
            common_complications: vec!["Memory interference".to_string(), "Essence instability".to_string(), "Emotional overwhelm".to_string()],
        });
        
        // Dark path forge session template
        session_templates.insert("dark_forge_session".to_string(), ForgeSessionTemplate {
            template_name: "Dark Path Forging".to_string(),
            forge_type: "blood_based".to_string(),
            estimated_duration: 1800.0, // 30 minutes (faster but more intense)
            required_reagent_categories: vec!["loss".to_string(), "pain".to_string(), "sacrifice".to_string()],
            companion_roles: {
                let mut roles = HashMap::new();
                roles.insert("pain_sharing".to_string(), "Share burden of painful process".to_string());
                roles.insert("will_reinforcement".to_string(), "Reinforce determination".to_string());
                roles.insert("power_stabilization".to_string(), "Help control unleashed power".to_string());
                roles
            },
            success_factors: vec!["Unwavering determination".to_string(), "Pain tolerance".to_string(), "Power control".to_string()],
            common_complications: vec!["Power overflow".to_string(), "Pain feedback".to_string(), "Emotional scarring".to_string()],
        });
        
        Self {
            active_sessions: HashMap::new(),
            session_templates,
            completed_sessions: Vec::new(),
            session_analytics: ForgeSessionAnalytics::default(),
        }
    }
    
    /// Start a new forge session
    pub fn start_session(&mut self, session_data: ActiveForgeSession) -> Result<(), String> {
        let session_id = session_data.session_id;
        
        if self.active_sessions.contains_key(&session_id) {
            return Err("Session already active".to_string());
        }
        
        self.active_sessions.insert(session_id, session_data);
        self.session_analytics.total_sessions_attempted += 1;
        
        Ok(())
    }
    
    /// Complete a forge session
    pub fn complete_session(&mut self, session_id: Uuid, success: bool, completion_data: CompletedForgeSession) {
        self.active_sessions.remove(&session_id);
        
        if success {
            self.session_analytics.successful_sessions += 1;
        }
        
        // Update analytics
        self.session_analytics.average_session_duration = 
            (self.session_analytics.average_session_duration * (self.completed_sessions.len() as f32) + 
             (completion_data.completion_time - completion_data.session_id.as_u128() as i64) as f32) / 
            (self.completed_sessions.len() + 1) as f32;
        
        self.completed_sessions.push(completion_data);
    }
}
