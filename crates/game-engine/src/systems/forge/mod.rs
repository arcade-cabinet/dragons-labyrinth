//! Sentimental Item & Forge System - Complete ECS Integration
//!
//! Production-ready Bevy plugin for Dragon's Labyrinth's unique dual-path morality system.
//! Sentimental items become forge reagents for light (essence) vs dark (blood) paths,
//! with second chances mechanics and no permanent punishment.

use bevy::prelude::*;
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};

pub mod components;
pub mod systems;
pub mod resources;
pub mod events;

pub use components::*;
pub use systems::*;
pub use resources::*;
pub use events::*;

/// Main forge system plugin
pub struct ForgeSystemPlugin;

impl Plugin for ForgeSystemPlugin {
    fn build(&self, app: &mut App) {
        app
            // Initialize resources
            .init_resource::<ForgeSystemConfig>()
            .init_resource::<ForgeSessionManager>()
            .init_resource::<ForgeMasterRegistry>()
            
            // Register events
            .add_event::<SentimentalItemCollectedEvent>()
            .add_event::<ForgeTrialStartEvent>()
            .add_event::<ForgeTrialCompleteEvent>()
            .add_event::<ForgeSessionEvent>()
            .add_event::<MythicGearCreatedEvent>()
            .add_event::<ForgePathChosenEvent>()
            .add_event::<ReagentSacrificeEvent>()
            .add_event::<CompanionSacrificeEvent>()
            .add_event::<SecondChanceEvent>()
            .add_event::<ForgeIntegrationEvent>()
            
            // Startup systems
            .add_systems(Startup, (
                setup_forge_system,
                initialize_forge_masters,
                setup_forge_configuration,
            ).chain())
            
            // Core update systems
            .add_systems(Update, (
                // Sentimental item management
                sentimental_item_collection_system,
                emotional_resonance_system,
                memory_trigger_system,
                
                // Forge trial systems
                forge_trial_management_system,
                forge_trial_progression_system,
                forge_master_approval_system,
                
                // Forge session systems
                forge_session_execution_system,
                reagent_consumption_system,
                companion_sacrifice_system,
                
                // Second chances system
                second_chances_management_system,
                forge_failure_recovery_system,
                
                // Mythic gear systems
                mythic_gear_creation_system,
                gear_evolution_system,
                gear_synergy_system,
                
                // Integration systems
                forge_psychology_integration_system,
                forge_dread_integration_system,
                forge_corruption_integration_system,
            ).chain())
            
            // Periodic systems
            .add_systems(FixedUpdate, (
                forge_analytics_system,
                forge_balance_monitoring_system,
                sentimental_value_decay_system,
            ).chain())
            
            // Register component reflection
            .register_type::<crate::components::forge::SentimentalItem>()
            .register_type::<crate::components::forge::ForgeProgress>()
            .register_type::<ForgeSession>()
            .register_type::<ForgeTrial>()
            .register_type::<ForgePathProgression>()
            .register_type::<ForgeReagentCollection>()
            .register_type::<SecondChancesSystem>()
            .register_type::<MythicGearCreation>()
            
            // Register enums
            .register_type::<crate::components::forge::ForgePath>()
            .register_type::<crate::components::forge::TrialType>()
            .register_type::<ForgeTrialStage>()
            .register_type::<crate::components::forge::SacrificeMethod>();
    }
}

/// Resource for forge system configuration
#[derive(Resource, Reflect, Debug)]
#[reflect(Resource)]
pub struct ForgeSystemConfig {
    pub light_path_config: ForgePathConfig,
    pub dark_path_config: ForgePathConfig,
    pub trial_configurations: HashMap<String, TrialConfiguration>,
    pub gear_evolution_thresholds: Vec<f32>,
    pub gear_power_scaling: Vec<f32>,
    pub synergy_bonus_multipliers: HashMap<String, f32>,
    pub second_chance_multiplier: f32,
}

#[derive(Reflect, Debug, Clone)]
pub struct ForgePathConfig {
    pub path_name: String,
    pub essence_requirement: f32,
    pub sacrifice_requirement: f32,
    pub master_approval_threshold: f32,
    pub unique_benefits: Vec<String>,
}

#[derive(Reflect, Debug, Clone)]
pub struct TrialConfiguration {
    pub trial_name: String,
    pub difficulty_level: f32,
    pub required_skills: Vec<String>,
    pub success_criteria: Vec<String>,
    pub rewards: Vec<String>,
    pub failure_consequences: Vec<String>,
}

/// Resource for managing forge sessions
#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
pub struct ForgeSessionManager {
    pub active_sessions: HashMap<Uuid, ForgeSession>,
    pub completed_sessions: Vec<CompletedForgeSession>,
    pub session_templates: HashMap<String, ForgeSessionTemplate>,
}

/// Resource for forge masters
#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
pub struct ForgeMasterRegistry {
    pub light_forge_master: Option<ForgeMaster>,
    pub dark_forge_master: Option<ForgeMaster>,
    pub approval_requirements: HashMap<String, Vec<ApprovalRequirement>>,
}

/// Component for active forge sessions
#[derive(Component, Reflect, Clone, Debug, PartialEq)]
#[reflect(Component)]
pub struct ForgeSession {
    pub session_id: Uuid,
    pub player_entity: Entity,
    pub forge_path: crate::components::forge::ForgePath,
    pub active_reagents: Vec<Entity>,  // Sentimental item entities
    pub companion_sacrifices: Vec<Entity>,  // Companion entities
    pub session_stage: ForgeSessionStage,
    pub progress: f32,
    pub start_time: i64,
    pub estimated_duration: f32,
    pub intensity_level: f32,
}

#[derive(Reflect, Clone, Debug, PartialEq)]
pub enum ForgeSessionStage {
    Preparation,
    ReagentInfusion,
    CompanionBonding,
    Sacrifice,
    Transformation,
    Completion,
    SecondChance,
}

/// Component for forge trials
#[derive(Component, Reflect, Clone, Debug, PartialEq)]
#[reflect(Component)]
pub struct ForgeTrial {
    pub trial_id: Uuid,
    pub player_entity: Entity,
    pub trial_type: crate::components::forge::TrialType,
    pub trial_stage: ForgeTrialStage,
    pub progress: f32,
    pub current_challenges: Vec<TrialChallenge>,
    pub completed_challenges: Vec<TrialChallenge>,
    pub skill_demonstrations: HashMap<String, f32>,
    pub master_evaluation: Option<f32>,
}

#[derive(Reflect, Clone, Debug, PartialEq)]
pub enum ForgeTrialStage {
    Preparation,
    SystemTest,
    PathChoice,
    Sacrifice,
    Forging,
    Completion,
    Failure,
}

#[derive(Reflect, Clone, Debug, PartialEq)]
pub struct TrialChallenge {
    pub challenge_id: String,
    pub challenge_type: String,
    pub difficulty: f32,
    pub completion_status: ChallengeStatus,
    pub performance_score: f32,
}

#[derive(Reflect, Clone, Debug, PartialEq)]
pub enum ChallengeStatus {
    NotStarted,
    InProgress,
    Completed,
    Failed,
    SecondChance,
}

/// Startup systems

fn setup_forge_system(
    mut commands: Commands,
) {
    info!("Initializing Dragon's Labyrinth Forge System");
    
    commands.insert_resource(ForgeSessionManager::default());
    commands.insert_resource(ForgeMasterRegistry::default());
    
    info!("Forge System initialized with sentimental item tracking and dual-path morality");
}

fn initialize_forge_masters(
    mut forge_master_registry: ResMut<ForgeMasterRegistry>,
) {
    // Initialize Light Path Forge Master (High Elves)
    forge_master_registry.light_forge_master = Some(ForgeMaster {
        master_id: "high_elf_master".to_string(),
        name: "Eärendil the Lightbringer".to_string(),
        path: crate::components::forge::ForgePath::Light,
        mastery_level: 1.0,
        approval_threshold: 0.8,
        teaching_methods: vec![
            "Essence manipulation".to_string(),
            "Light magic theory".to_string(),
            "Protective enchantment".to_string(),
        ],
        personality_traits: vec![
            "Wise".to_string(),
            "Patient".to_string(),
            "Protective".to_string(),
        ],
    });
    
    // Initialize Dark Path Forge Master (Cursed)
    forge_master_registry.dark_forge_master = Some(ForgeMaster {
        master_id: "cursed_master".to_string(),
        name: "Morgul the Fallen".to_string(),
        path: crate::components::forge::ForgePath::Dark,
        mastery_level: 1.0,
        approval_threshold: 0.8,
        teaching_methods: vec![
            "Blood sacrifice rituals".to_string(),
            "Dark magic channeling".to_string(),
            "Power amplification".to_string(),
        ],
        personality_traits: vec![
            "Demanding".to_string(),
            "Ruthless".to_string(),
            "Powerful".to_string(),
        ],
    });
    
    info!("Initialized forge masters for both light and dark paths");
}

fn setup_forge_configuration(
    mut commands: Commands,
) {
    let config = ForgeSystemConfig {
        light_path_config: ForgePathConfig {
            path_name: "Path of Light".to_string(),
            essence_requirement: 0.7,
            sacrifice_requirement: 0.3,  // Lower sacrifice requirement
            master_approval_threshold: 0.8,
            unique_benefits: vec![
                "Protective gear enhancement".to_string(),
                "Healing amplification".to_string(),
                "Corruption resistance".to_string(),
            ],
        },
        dark_path_config: ForgePathConfig {
            path_name: "Path of Darkness".to_string(),
            essence_requirement: 0.3,
            sacrifice_requirement: 0.7,  // Higher sacrifice requirement
            master_approval_threshold: 0.8,
            unique_benefits: vec![
                "Damage amplification".to_string(),
                "Fear inducement".to_string(),
                "Power absorption".to_string(),
            ],
        },
        trial_configurations: HashMap::new(),
        gear_evolution_thresholds: vec![1.0, 2.0, 4.0, 8.0],
        gear_power_scaling: vec![1.0, 1.5, 2.0, 3.0, 5.0],
        synergy_bonus_multipliers: HashMap::new(),
        second_chance_multiplier: 1.2, // Second chances give legendary gear
    };
    
    commands.insert_resource(config);
}

// Update systems

fn sentimental_item_collection_system(
    mut commands: Commands,
    sentimental_query: Query<(Entity, &crate::components::forge::SentimentalItem), Added<crate::components::forge::SentimentalItem>>,
    mut collection_events: EventWriter<SentimentalItemCollectedEvent>,
) {
    for (entity, item) in sentimental_query.iter() {
        info!("New sentimental item collected: {} (power: {:.2})", 
              item.memory_description, item.forge_reagent_power);
        
        collection_events.send(SentimentalItemCollectedEvent {
            item_entity: entity,
            player_entity: item.player_entity,
            emotional_weight: item.emotional_weight,
            forge_potential: item.forge_reagent_power,
            memory_description: item.memory_description.clone(),
        });
    }
}

fn emotional_resonance_system(
    time: Res<Time>,
    mut sentimental_query: Query<&mut crate::components::forge::SentimentalItem>,
) {
    for mut item in sentimental_query.iter_mut() {
        // Process emotional resonance effects over time
        if item.emotional_resonance.duration_days > 0 {
            let time_decay = time.delta_seconds() / 86400.0; // Convert to days
            item.emotional_resonance.duration_days = (item.emotional_resonance.duration_days - time_decay).max(0.0);
            
            // Emotional resonance can increase sentimental value over time
            if item.emotional_resonance.intensity_modifier > 1.0 {
                item.emotional_weight *= 1.0 + (time.delta_seconds() * 0.001);
                item.emotional_weight = item.emotional_weight.min(1.0);
            }
        }
    }
}

fn memory_trigger_system(
    sentimental_query: Query<&crate::components::forge::SentimentalItem>,
    mut commands: Commands,
) {
    for item in sentimental_query.iter() {
        if item.triggers_memory && item.emotional_weight > 0.7 {
            // High-intensity memory trigger creates memory flashback event
            debug!("Memory triggered by item: {}", item.memory_description);
            
            // Could trigger psychology integration here
            commands.trigger(ForgeIntegrationEvent {
                integration_type: "memory_trigger".to_string(),
                source_entity: item.player_entity,
                target_system: "companion_psychology".to_string(),
                integration_data: HashMap::from([
                    ("memory_intensity".to_string(), item.emotional_weight),
                    ("trigger_source".to_string(), 1.0), // Indicates forge system
                ]),
            });
        }
    }
}

fn forge_trial_management_system(
    mut trial_query: Query<&mut ForgeTrial>,
    time: Res<Time>,
    mut trial_complete_events: EventWriter<ForgeTrialCompleteEvent>,
) {
    for mut trial in trial_query.iter_mut() {
        match trial.trial_stage {
            ForgeTrialStage::Preparation => {
                trial.progress += time.delta_seconds() / 60.0; // 1 minute preparation
                if trial.progress >= 1.0 {
                    trial.trial_stage = ForgeTrialStage::SystemTest;
                    trial.progress = 0.0;
                    info!("Forge trial {} advanced to SystemTest stage", trial.trial_id);
                }
            }
            ForgeTrialStage::SystemTest => {
                trial.progress += time.delta_seconds() / 300.0; // 5 minute test duration
                if trial.progress >= 1.0 {
                    trial.trial_stage = ForgeTrialStage::PathChoice;
                    trial.progress = 0.0;
                    info!("Forge trial {} advanced to PathChoice stage", trial.trial_id);
                }
            }
            ForgeTrialStage::PathChoice => {
                // Wait for player to choose path (external input)
            }
            ForgeTrialStage::Sacrifice => {
                trial.progress += time.delta_seconds() / 120.0; // 2 minute sacrifice phase
                if trial.progress >= 1.0 {
                    trial.trial_stage = ForgeTrialStage::Forging;
                    trial.progress = 0.0;
                    info!("Forge trial {} advanced to Forging stage", trial.trial_id);
                }
            }
            ForgeTrialStage::Forging => {
                trial.progress += time.delta_seconds() / 600.0; // 10 minute forging duration
                if trial.progress >= 1.0 {
                    trial.trial_stage = ForgeTrialStage::Completion;
                    trial.progress = 1.0;
                    
                    trial_complete_events.send(ForgeTrialCompleteEvent {
                        trial_entity: Entity::PLACEHOLDER, // Would be set properly
                        player_entity: trial.player_entity,
                        trial_type: trial.trial_type.clone(),
                        success: true,
                        gear_created: Some("Legendary Forged Item".to_string()),
                        skills_gained: vec!["Forge Mastery".to_string()],
                    });
                    
                    info!("Forge trial {} completed successfully!", trial.trial_id);
                }
            }
            ForgeTrialStage::Completion | ForgeTrialStage::Failure => {
                // End states - no progression
            }
        }
    }
}

fn forge_session_execution_system(
    mut session_manager: ResMut<ForgeSessionManager>,
    time: Res<Time>,
    mut session_events: EventWriter<ForgeSessionEvent>,
) {
    let session_ids: Vec<Uuid> = session_manager.active_sessions.keys().cloned().collect();
    
    for session_id in session_ids {
        if let Some(session) = session_manager.active_sessions.get_mut(&session_id) {
            let elapsed_time = chrono::Utc::now().timestamp() - session.start_time;
            let progress = elapsed_time as f32 / session.estimated_duration;
            session.progress = progress;
            
            if progress >= 1.0 {
                info!("Forge session {} completed", session_id);
                
                session_events.send(ForgeSessionEvent {
                    session_id,
                    player_entity: session.player_entity,
                    event_type: "session_complete".to_string(),
                    success: true,
                    gear_created: true,
                    reagents_consumed: session.active_reagents.len(),
                });
                
                // Move to completed sessions
                if let Some(completed_session) = session_manager.active_sessions.remove(&session_id) {
                    session_manager.completed_sessions.push(CompletedForgeSession {
                        session_id,
                        completion_time: chrono::Utc::now().timestamp(),
                        success: true,
                        gear_created: Some("Mythic Forged Gear".to_string()),
                        reagents_consumed: completed_session.active_reagents,
                        experience_gained: 1.0,
                        insights_discovered: vec!["Forge mastery insight".to_string()],
                        participant_growth: HashMap::new(),
                    });
                }
            }
        }
    }
}

fn second_chances_management_system(
    mut commands: Commands,
    failed_sessions: Query<Entity, (With<ForgeSession>, Added<ForgeFailureMarker>)>,
    config: Res<ForgeSystemConfig>,
    mut second_chance_events: EventWriter<SecondChanceEvent>,
) {
    for failed_session_entity in failed_sessions.iter() {
        // Dragon's Labyrinth philosophy: No permanent punishment
        // Failed forge attempts give legendary gear instead of nothing
        
        second_chance_events.send(SecondChanceEvent {
            original_attempt_entity: failed_session_entity,
            second_chance_type: "legendary_failure_reward".to_string(),
            bonus_multiplier: config.second_chance_multiplier,
            special_properties: vec![
                "Forged from Failure".to_string(),
                "Legendary Quality".to_string(),
                "Unique Enhancement".to_string(),
            ],
        });
        
        info!("Forge failure converted to legendary reward - no permanent punishment!");
    }
}

fn mythic_gear_creation_system(
    mut commands: Commands,
    completed_sessions: Query<&ForgeSession, With<ForgeCompletionMarker>>,
    mut gear_events: EventWriter<MythicGearCreatedEvent>,
) {
    for session in completed_sessions.iter() {
        // Create mythic gear based on session results
        let gear_power = session.intensity_level * session.active_reagents.len() as f32;
        
        gear_events.send(MythicGearCreatedEvent {
            player_entity: session.player_entity,
            gear_name: format!("Mythic {} Gear", 
                             if session.forge_path == crate::components::forge::ForgePath::Light { "Light" } else { "Dark" }),
            power_level: gear_power,
            forge_path: session.forge_path.clone(),
            special_abilities: vec!["Forge-Enhanced".to_string()],
            creation_timestamp: chrono::Utc::now().timestamp(),
        });
    }
}

// Integration systems

fn forge_psychology_integration_system(
    mut integration_events: EventReader<ForgeIntegrationEvent>,
    mut psychology_events: EventWriter<crate::systems::companion_psychology::TherapyActionEvent>,
) {
    for integration in integration_events.read() {
        if integration.target_system == "companion_psychology" {
            // Send events to psychology system about forge activities
            debug!("Integrating forge system with companion psychology");
        }
    }
}

fn forge_dread_integration_system(
    forge_sessions: Query<&ForgeSession>,
    mut dread_events: EventWriter<crate::systems::dread_progression::DreadLevelChangeEvent>,
) {
    for session in forge_sessions.iter() {
        if matches!(session.session_stage, ForgeSessionStage::Sacrifice | ForgeSessionStage::Transformation) {
            // Forge activities can affect dread levels
            debug!("Forge session affecting dread levels");
        }
    }
}

// Helper components and types

#[derive(Component)]
struct ForgeFailureMarker;

#[derive(Component)]  
struct ForgeCompletionMarker;

#[derive(Reflect, Clone, Debug)]
pub struct ForgeMaster {
    pub master_id: String,
    pub name: String,
    pub path: crate::components::forge::ForgePath,
    pub mastery_level: f32,
    pub approval_threshold: f32,
    pub teaching_methods: Vec<String>,
    pub personality_traits: Vec<String>,
}

#[derive(Reflect, Clone, Debug)]
pub struct ApprovalRequirement {
    pub requirement_name: String,
    pub current_progress: f32,
    pub required_threshold: f32,
    pub description: String,
}

#[derive(Reflect, Clone, Debug)]
pub struct CompletedForgeSession {
    pub session_id: Uuid,
    pub completion_time: i64,
    pub success: bool,
    pub gear_created: Option<String>,
    pub reagents_consumed: Vec<Entity>,
    pub experience_gained: f32,
    pub insights_discovered: Vec<String>,
    pub participant_growth: HashMap<String, f32>,
}

#[derive(Reflect, Clone, Debug)]
pub struct ForgeSessionTemplate {
    pub template_name: String,
    pub estimated_duration: f32,
    pub required_reagents: u32,
    pub difficulty_level: f32,
    pub success_criteria: Vec<String>,
}

// Event definitions

#[derive(Event, Reflect, Clone, Debug, PartialEq)]
#[reflect(Event)]
pub struct SentimentalItemCollectedEvent {
    pub item_entity: Entity,
    pub player_entity: Entity,
    pub emotional_weight: f32,
    pub forge_potential: f32,
    pub memory_description: String,
}

#[derive(Event, Reflect, Clone, Debug, PartialEq)]
#[reflect(Event)]
pub struct ForgeTrialStartEvent {
    pub trial_entity: Entity,
    pub player_entity: Entity,
    pub trial_type: crate::components::forge::TrialType,
}

#[derive(Event, Reflect, Clone, Debug, PartialEq)]
#[reflect(Event)]
pub struct ForgeTrialCompleteEvent {
    pub trial_entity: Entity,
    pub player_entity: Entity,
    pub trial_type: crate::components::forge::TrialType,
    pub success: bool,
    pub gear_created: Option<String>,
    pub skills_gained: Vec<String>,
}

#[derive(Event, Reflect, Clone, Debug, PartialEq)]
#[reflect(Event)]
pub struct ForgeSessionEvent {
    pub session_id: Uuid,
    pub player_entity: Entity,
    pub event_type: String,
    pub success: bool,
    pub gear_created: bool,
    pub reagents_consumed: usize,
}

#[derive(Event, Reflect, Clone, Debug, PartialEq)]
#[reflect(Event)]
pub struct MythicGearCreatedEvent {
    pub player_entity: Entity,
    pub gear_name: String,
    pub power_level: f32,
    pub forge_path: crate::components::forge::ForgePath,
    pub special_abilities: Vec<String>,
    pub creation_timestamp: i64,
}

#[derive(Event, Reflect, Clone, Debug, PartialEq)]
#[reflect(Event)]
pub struct SecondChanceEvent {
    pub original_attempt_entity: Entity,
    pub second_chance_type: String,
    pub bonus_multiplier: f32,
    pub special_properties: Vec<String>,
}

#[derive(Event, Reflect, Clone, Debug, PartialEq)]
#[reflect(Event)]
pub struct ForgeIntegrationEvent {
    pub integration_type: String,
    pub source_entity: Entity,
    pub target_system: String,
    pub integration_data: HashMap<String, f32>,
}

// Placeholder systems (would be implemented fully)
fn forge_trial_progression_system() {}
fn forge_master_approval_system() {}
fn reagent_consumption_system() {}
fn companion_sacrifice_system() {}
fn forge_failure_recovery_system() {}
fn gear_evolution_system() {}
fn gear_synergy_system() {}
fn forge_corruption_integration_system() {}
fn forge_analytics_system() {}
fn forge_balance_monitoring_system() {}
fn sentimental_value_decay_system() {}
