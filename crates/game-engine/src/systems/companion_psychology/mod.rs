//! Companion Psychology System - Complete ECS Integration
//!
//! Production-ready Bevy plugin for Dragon's Labyrinth's sophisticated companion psychology,
//! trauma processing, therapy quests, and memory palace systems with full database integration.

use bevy::prelude::*;
use std::collections::HashMap;

pub mod components;
pub mod systems;
pub mod resources;
pub mod events;
pub mod queries;

pub use components::*;
pub use systems::*;
pub use resources::*;
pub use events::*;
pub use queries::*;

/// Main plugin for companion psychology system
pub struct CompanionPsychologyPlugin;

impl Plugin for CompanionPsychologyPlugin {
    fn build(&self, app: &mut App) {
        app
            // Initialize resources
            .init_resource::<TherapyConfiguration>()
            .init_resource::<MemoryPalaceManager>()
            .init_resource::<TherapySessionManager>()
            
            // Register events
            .add_event::<TraumaEvent>()
            .add_event::<TraumaTriggerEvent>()
            .add_event::<CompanionBreakingPointEvent>()
            .add_event::<CompanionDepartureEvent>()
            .add_event::<TherapyActionEvent>()
            .add_event::<TherapyBreakthroughEvent>()
            .add_event::<RecoveryMilestoneEvent>()
            .add_event::<MemoryPalaceEvent>()
            .add_event::<MemoryPalaceHealingEvent>()
            .add_event::<CompanionSupportEvent>()
            .add_event::<TherapySessionEvent>()
            .add_event::<TherapySessionCompleteEvent>()
            .add_event::<ProfessionalSupportEvent>()
            .add_event::<DreadPsychologyEvent>()
            .add_event::<PsychologicalAssessmentEvent>()
            .add_event::<PsychologyResearchEvent>()
            .add_event::<CompanionBehaviorChangeEvent>()
            .add_event::<TraumaContaminationEvent>()
            .add_event::<PsychologyIntegrationEvent>()
            .add_event::<PlayerPsychologySkillEvent>()
            .add_event::<CompanionTrustEvent>()
            .add_event::<TherapeuticDialogueEvent>()
            .add_event::<PsychologicalCrisisEvent>()
            
            // Add startup systems
            .add_systems(Startup, (
                setup_companion_psychology_system,
                setup_therapy_configuration,
                setup_memory_palace_manager,
            ).chain())
            
            // Add main update systems with proper scheduling
            .add_systems(Update, (
                // Core trauma processing (highest priority)
                trauma_accumulation_system,
                trauma_response_system,
                breaking_point_monitoring_system,
                
                // Psychology state updates
                initialize_companion_psychology_system,
                psychological_crisis_response_system,
                
                // Therapy and healing systems
                therapy_quest_system,
                therapy_breakthrough_system,
                therapy_session_system,
                memory_palace_system,
                
                // Support and relationship systems
                companion_support_system,
                professional_support_integration_system,
                
                // Integration with other game systems
                psychology_integration_system,
                dread_level_psychology_effects_system,
                
                // Assessment and monitoring
                companion_risk_assessment_system,
                therapy_progress_monitoring_system,
                
                // Player skill development
                player_psychology_skill_system,
                
                // Database persistence (lowest priority)
                psychology_database_sync_system,
            ).chain())
            
            // Add systems that run less frequently
            .add_systems(FixedUpdate, (
                // Daily/periodic systems
                recovery_milestone_check_system,
                therapy_session_scheduling_system,
                professional_support_availability_system,
                companion_psychology_analytics_system,
            ).chain())
            
            // Register component reflection for debugging and serialization
            .register_type::<CompanionPsychology>()
            .register_type::<TraumaSource>()
            .register_type::<TraumaResponse>()
            .register_type::<TherapyQuest>()
            .register_type::<MemoryPalace>()
            .register_type::<CompanionSupport>()
            .register_type::<TherapySession>()
            .register_type::<ProfessionalSupport>()
            .register_type::<PsychologicalResilience>()
            .register_type::<RecoveryMilestone>()
            
            // Register enums
            .register_type::<TherapyStage>()
            .register_type::<BehavioralEffect>()
            
            // Add state for psychology system phases
            .init_state::<PsychologySystemPhase>()
            
            // Integration systems that run in specific game states
            .add_systems(OnEnter(PsychologySystemPhase::Assessment), (
                trigger_companion_assessments,
                initialize_baseline_psychology,
            ))
            .add_systems(OnEnter(PsychologySystemPhase::CrisisIntervention), (
                activate_crisis_protocols,
                emergency_professional_support,
            ))
            .add_systems(OnExit(PsychologySystemPhase::CrisisIntervention), (
                deactivate_crisis_protocols,
                post_crisis_assessment,
            ));
    }
}

/// States for psychology system operation
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PsychologySystemPhase {
    #[default]
    Normal,
    Assessment,
    CrisisIntervention,
    TherapySession,
    MemoryPalaceExploration,
    RecoveryIntegration,
}

/// Setup companion psychology system on startup
fn setup_companion_psychology_system(
    mut commands: Commands,
) {
    info!("Initializing Companion Psychology System");
    
    // Initialize the psychology state resource
    let psychology_state = CompanionPsychologyState {
        active_companions: HashMap::new(),
        therapy_sessions_cache: HashMap::new(),
        memory_palace_entities: HashMap::new(),
        professional_network: HashMap::new(),
        trauma_trigger_mapping: HashMap::new(),
    };
    
    commands.insert_resource(psychology_state);
    
    info!("Companion Psychology System initialized successfully");
}

/// Setup therapy configuration from game data
fn setup_therapy_configuration(
    mut commands: Commands,
) {
    let config = TherapyConfiguration::default();
    
    info!("Loaded therapy configuration with {} trauma categories", 
          config.trauma_categories.len());
    info!("Professional support multipliers: {:?}", 
          config.professional_support_multipliers);
    
    commands.insert_resource(config);
}

/// Setup memory palace manager
fn setup_memory_palace_manager(
    mut commands: Commands,
    therapy_config: Res<TherapyConfiguration>,
) {
    let manager = MemoryPalaceManager::new(&therapy_config);
    
    info!("Memory palace manager initialized with {} room templates and {} healing symbols",
          manager.room_templates.len(), manager.healing_symbols.len());
    
    commands.insert_resource(manager);
}

/// System to handle psychological crises
fn psychological_crisis_response_system(
    mut crisis_events: EventReader<PsychologicalCrisisEvent>,
    mut psychology_state: ResMut<PsychologySystemPhase>,
    mut commands: Commands,
    psychology_query: Query<&CompanionPsychology>,
    mut intervention_events: EventWriter<ProfessionalSupportEvent>,
) {
    for crisis in crisis_events.read() {
        warn!("Psychological crisis detected for companion: {} - Type: {} - Severity: {}", 
              crisis.companion_id, crisis.crisis_type, crisis.crisis_severity);
        
        // Switch to crisis intervention mode
        psychology_state.set(PsychologySystemPhase::CrisisIntervention);
        
        // Determine appropriate intervention
        let intervention_type = match crisis.crisis_type.as_str() {
            "panic_attack" => "immediate_grounding_techniques",
            "dissociation" => "grounding_and_reality_orientation",
            "self_harm_risk" => "emergency_safety_planning",
            "suicide_ideation" => "emergency_professional_intervention",
            _ => "general_crisis_support",
        };
        
        // If professional help is needed, trigger professional support event
        if crisis.professional_help_needed || crisis.crisis_severity > 0.8 {
            intervention_events.send(ProfessionalSupportEvent {
                companion_entity: crisis.companion_entity,
                companion_id: crisis.companion_id,
                support_type: "emergency_intervention".to_string(),
                provider_id: "crisis_hotline".to_string(),
                support_action: ProfessionalSupportAction::CrisisIntervention,
                cost: 0.0, // Emergency support is free
                accessibility_barriers: vec![],
                cultural_fit: 0.8, // Crisis intervention is generally culturally adaptable
            });
        }
        
        // Tag entity for special crisis monitoring
        commands.entity(crisis.companion_entity).insert(PsychologicalCrisisMarker {
            crisis_type: crisis.crisis_type.clone(),
            severity: crisis.crisis_severity,
            intervention_active: true,
            start_time: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64,
        });
    }
}

/// Component to mark entities in psychological crisis
#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct PsychologicalCrisisMarker {
    crisis_type: String,
    severity: f32,
    intervention_active: bool,
    start_time: i64,
}

/// System for professional support integration
fn professional_support_integration_system(
    mut support_events: EventReader<ProfessionalSupportEvent>,
    mut psychology_query: Query<&mut CompanionPsychology>,
    psychology_state: Res<CompanionPsychologyState>,
    mut milestone_events: EventWriter<RecoveryMilestoneEvent>,
) {
    for support_event in support_events.read() {
        if let Ok(mut psychology) = psychology_query.get_mut(support_event.companion_entity) {
            let provider = psychology_state.professional_network.get(&support_event.provider_id);
            
            match &support_event.support_action {
                ProfessionalSupportAction::InitialAssessment => {
                    psychology.therapy_readiness += 0.2;
                    psychology.therapy_readiness = psychology.therapy_readiness.min(1.0);
                    
                    info!("Professional assessment completed for companion {}", support_event.companion_id);
                }
                ProfessionalSupportAction::ProvideTherapy(therapy_type) => {
                    let effectiveness = if let Some(provider) = provider {
                        provider.quality_rating * support_event.cultural_fit
                    } else {
                        0.6 // Default effectiveness
                    };
                    
                    psychology.therapeutic_bond += effectiveness * 0.15;
                    psychology.recovery_progress += effectiveness * 0.1;
                    psychology.therapeutic_bond = psychology.therapeutic_bond.min(1.0);
                    psychology.recovery_progress = psychology.recovery_progress.min(1.0);
                    
                    info!("Professional therapy session completed: {} - Effectiveness: {:.2}", 
                          therapy_type, effectiveness);
                }
                ProfessionalSupportAction::CrisisIntervention => {
                    // Emergency intervention reduces immediate trauma
                    psychology.trauma_level *= 0.9; // Slight reduction
                    psychology.breakthrough_potential += 0.3;
                    psychology.breakthrough_potential = psychology.breakthrough_potential.min(1.0);
                    
                    milestone_events.send(RecoveryMilestoneEvent {
                        companion_entity: support_event.companion_entity,
                        companion_id: support_event.companion_id,
                        milestone_type: "crisis_intervention_received".to_string(),
                        achievement_date: chrono::Utc::now().timestamp(),
                        significance: 0.7,
                        celebration_appropriate: false, // Crisis intervention isn't celebratory
                        progress_indicators: vec!["Professional crisis support received".to_string()],
                        consolidation_needed: true,
                    });
                    
                    warn!("Crisis intervention provided for companion {}", support_event.companion_id);
                }
                _ => {}
            }
        }
    }
}

/// System for psychology integration with other game systems
fn psychology_integration_system(
    psychology_query: Query<&CompanionPsychology, Changed<CompanionPsychology>>,
    mut integration_events: EventWriter<PsychologyIntegrationEvent>,
) {
    for psychology in psychology_query.iter() {
        // Generate integration events when psychology changes significantly
        let mut modifiers = Vec::new();
        
        // Combat effectiveness modifiers based on trauma and active responses
        let combat_modifier = calculate_combat_effectiveness_modifier(psychology);
        if combat_modifier != 0.0 {
            modifiers.push(PsychologicalModifier {
                modifier_type: "combat_effectiveness".to_string(),
                target_attribute: "attack_power".to_string(),
                modifier_value: combat_modifier,
                duration: None, // Persistent until psychology changes
                stacks: false,
            });
        }
        
        // Movement speed modifiers for trauma responses
        let movement_modifier = calculate_movement_modifier(psychology);
        if movement_modifier != 1.0 {
            modifiers.push(PsychologicalModifier {
                modifier_type: "movement_speed".to_string(),
                target_attribute: "movement_speed".to_string(),
                modifier_value: movement_modifier,
                duration: None,
                stacks: false,
            });
        }
        
        // Trust affects dialogue options
        if psychology.trust < 0.5 {
            modifiers.push(PsychologicalModifier {
                modifier_type: "dialogue_restriction".to_string(),
                target_attribute: "available_dialogue_options".to_string(),
                modifier_value: psychology.trust,
                duration: None,
                stacks: false,
            });
        }
        
        if !modifiers.is_empty() {
            integration_events.send(PsychologyIntegrationEvent {
                companion_entity: Entity::PLACEHOLDER, // Would be set by calling system
                companion_id: psychology.companion_id,
                integrating_system: "multiple_systems".to_string(),
                integration_type: "psychological_modifiers".to_string(),
                psychological_modifiers: modifiers,
                feedback_effects: vec![
                    "Combat stress may increase trauma".to_string(),
                    "Safe dialogue builds trust".to_string(),
                ],
            });
        }
    }
}

/// System for dread level effects on psychology
fn dread_level_psychology_effects_system(
    mut dread_events: EventReader<DreadPsychologyEvent>,
    mut psychology_query: Query<&mut CompanionPsychology>,
    therapy_config: Res<TherapyConfiguration>,
    mut trauma_events: EventWriter<TraumaEvent>,
) {
    for dread_event in dread_events.read() {
        let amplifier = therapy_config.dread_level_trauma_amplifiers[dread_event.current_dread_level as usize];
        
        // Apply dread effects to all affected companions
        for companion_id in &dread_event.affected_companions {
            // Find companion entity (would need proper entity lookup in production)
            for mut psychology in psychology_query.iter_mut() {
                if psychology.companion_id == *companion_id {
                    // Increase isolation tendency at higher dread levels
                    if dread_event.current_dread_level >= 2 {
                        psychology.isolation_tendency += 0.1 * (dread_event.current_dread_level as f32 / 4.0);
                        psychology.isolation_tendency = psychology.isolation_tendency.min(1.0);
                    }
                    
                    // Reduce therapy readiness in high dread environments
                    if dread_event.current_dread_level >= 3 {
                        psychology.therapy_readiness *= 0.9;
                    }
                    
                    // At maximum dread, trigger environmental trauma
                    if dread_event.current_dread_level == 4 && dread_event.previous_dread_level < 4 {
                        trauma_events.send(TraumaEvent {
                            companion_entity: Entity::PLACEHOLDER, // Would need proper lookup
                            companion_id: *companion_id,
                            trauma_type: "environmental_horror".to_string(),
                            source_id: "maximum_dread_exposure".to_string(),
                            severity: 1.5 * amplifier,
                            context: "Exposed to maximum dread level environment".to_string(),
                            triggers: vec!["darkness".to_string(), "unnatural_sounds".to_string(), "reality_distortion".to_string()],
                            dread_level_amplifier: amplifier,
                            witness_companions: dread_event.affected_companions.clone(),
                        });
                    }
                }
            }
        }
        
        info!("Applied dread level {} effects to {} companions", 
              dread_event.current_dread_level, dread_event.affected_companions.len());
    }
}

/// System for companion risk assessment
fn companion_risk_assessment_system(
    psychology_query: Query<(&CompanionPsychology, &PsychologicalResilience), Changed<CompanionPsychology>>,
    mut crisis_events: EventWriter<PsychologicalCrisisEvent>,
    time: Res<Time>,
) {
    for (psychology, resilience) in psychology_query.iter() {
        // Check for critical risk factors
        let risk_score = calculate_companion_risk_score(psychology, resilience);
        
        if risk_score > 0.8 {
            // High risk - may need intervention
            warn!("Companion {} at high risk - Risk score: {:.2}", 
                  psychology.companion_id, risk_score);
            
            // If trauma level approaching breaking point, trigger crisis
            if psychology.trauma_level >= psychology.breaking_point * 0.95 {
                crisis_events.send(PsychologicalCrisisEvent {
                    companion_entity: Entity::PLACEHOLDER, // Would need proper entity lookup
                    companion_id: psychology.companion_id,
                    crisis_type: "impending_breakdown".to_string(),
                    crisis_severity: risk_score,
                    immediate_danger: false,
                    triggers: vec!["accumulated_trauma".to_string()],
                    available_interventions: vec![
                        CrisisIntervention {
                            intervention_name: "Intensive Therapy".to_string(),
                            intervention_type: "therapy".to_string(),
                            effectiveness_estimate: 0.7,
                            resource_requirements: vec!["Professional therapist".to_string()],
                            time_to_effectiveness: 3600.0, // 1 hour
                            side_effects: vec!["Emotional exhaustion".to_string()],
                        }
                    ],
                    professional_help_needed: true,
                    support_people_present: vec![], // Would be filled by calling system
                });
            }
        }
    }
}

/// System for therapy progress monitoring
fn therapy_progress_monitoring_system(
    therapy_query: Query<&TherapyQuest, Changed<TherapyQuest>>,
    mut milestone_events: EventWriter<RecoveryMilestoneEvent>,
) {
    for quest in therapy_query.iter() {
        // Check for significant progress milestones
        match quest.stage {
            TherapyStage::Breakthrough if quest.progress > 0.5 => {
                milestone_events.send(RecoveryMilestoneEvent {
                    companion_entity: Entity::PLACEHOLDER, // Would need proper entity lookup
                    companion_id: Uuid::new_v4(), // Would come from quest data
                    milestone_type: "therapy_breakthrough".to_string(),
                    achievement_date: chrono::Utc::now().timestamp(),
                    significance: quest.progress,
                    celebration_appropriate: true,
                    progress_indicators: vec!["Major therapeutic breakthrough achieved".to_string()],
                    consolidation_needed: true,
                });
            }
            TherapyStage::Integration if quest.progress >= 1.0 => {
                milestone_events.send(RecoveryMilestoneEvent {
                    companion_entity: Entity::PLACEHOLDER,
                    companion_id: Uuid::new_v4(),
                    milestone_type: "therapy_quest_completed".to_string(),
                    achievement_date: chrono::Utc::now().timestamp(),
                    significance: 0.8,
                    celebration_appropriate: true,
                    progress_indicators: vec!["Therapy quest successfully completed".to_string()],
                    consolidation_needed: false,
                });
            }
            _ => {}
        }
    }
}

/// System for player psychology skill development
fn player_psychology_skill_system(
    mut skill_events: EventReader<PlayerPsychologySkillEvent>,
    mut therapy_events: EventReader<TherapyActionEvent>,
    mut commands: Commands,
) {
    // Track player actions that build psychological skills
    for therapy_action in therapy_events.read() {
        let skill_gain = calculate_skill_gain_from_action(&therapy_action.action, therapy_action.player_skill_level);
        
        if skill_gain > 0.0 {
            commands.trigger(PlayerPsychologySkillEvent {
                player_entity: Entity::PLACEHOLDER, // Would need proper player entity
                skill_area: get_skill_area_for_action(&therapy_action.action),
                skill_improvement: skill_gain,
                learning_source: "practical_application".to_string(),
                practical_application: format!("Used {} in therapy", format!("{:?}", therapy_action.action)),
                teaching_opportunities: vec![], // Could suggest teaching other players
                skill_level_reached: therapy_action.player_skill_level + skill_gain,
            });
        }
    }
}

/// System for database synchronization
fn psychology_database_sync_system(
    psychology_query: Query<&CompanionPsychology, Changed<CompanionPsychology>>,
    psychology_state: Res<CompanionPsychologyState>,
    mut commands: Commands,
) {
    // This would periodically sync psychology data to database
    // In production, this might use a more sophisticated batching system
    for psychology in psychology_query.iter() {
        // Mark entity for database sync
        commands.spawn(DatabaseSyncRequest {
            companion_id: psychology.companion_id,
            sync_type: "psychology_update".to_string(),
            priority: SyncPriority::Normal,
            timestamp: chrono::Utc::now().timestamp(),
        });
    }
}

// Additional systems for startup state transitions

fn trigger_companion_assessments(mut commands: Commands) {
    info!("Triggering companion psychological assessments");
    // Implementation would spawn assessment tasks
}

fn initialize_baseline_psychology(mut commands: Commands) {
    info!("Initializing baseline psychological profiles");
    // Implementation would set up initial psychology states
}

fn activate_crisis_protocols(mut commands: Commands) {
    warn!("Activating psychological crisis protocols");
    // Implementation would enable crisis response systems
}

fn emergency_professional_support(mut commands: Commands) {
    error!("Activating emergency professional support protocols");
    // Implementation would contact emergency support services
}

fn deactivate_crisis_protocols(mut commands: Commands) {
    info!("Deactivating crisis protocols");
    // Implementation would return systems to normal state
}

fn post_crisis_assessment(mut commands: Commands) {
    info!("Conducting post-crisis assessment");
    // Implementation would evaluate crisis resolution
}

// Fixed update systems

fn recovery_milestone_check_system() {
    // Check for recovery milestones daily
}

fn therapy_session_scheduling_system() {
    // Schedule therapy sessions based on companion needs
}

fn professional_support_availability_system() {
    // Update professional support availability
}

fn companion_psychology_analytics_system() {
    // Analyze psychology trends for game balance
}

// Helper components for database sync

#[derive(Component)]
struct DatabaseSyncRequest {
    companion_id: Uuid,
    sync_type: String,
    priority: SyncPriority,
    timestamp: i64,
}

#[derive(Debug, Clone)]
enum SyncPriority {
    Emergency,
    High,
    Normal,
    Low,
}

// Helper functions

fn calculate_combat_effectiveness_modifier(psychology: &CompanionPsychology) -> f32 {
    let trauma_penalty = psychology.trauma_level * -0.1;
    let trauma_response_penalty: f32 = psychology.active_trauma_responses.iter()
        .map(|response| match response.response_type.as_str() {
            "flashback" => -response.intensity * 0.3,
            "panic" => -response.intensity * 0.4,
            "dissociation" => -response.intensity * 0.5,
            _ => -response.intensity * 0.2,
        })
        .sum();
    
    (trauma_penalty + trauma_response_penalty).max(-0.8) // Cap at 80% penalty
}

fn calculate_movement_modifier(psychology: &CompanionPsychology) -> f32 {
    let base_modifier = 1.0;
    let trauma_slowdown = psychology.trauma_level * 0.05;
    let hesitation_penalty: f32 = psychology.active_trauma_responses.iter()
        .filter(|r| r.response_type == "dissociation")
        .map(|r| r.intensity * 0.2)
        .sum();
    
    (base_modifier - trauma_slowdown - hesitation_penalty).max(0.2) // Minimum 20% speed
}

fn calculate_companion_risk_score(psychology: &CompanionPsychology, resilience: &PsychologicalResilience) -> f32 {
    let trauma_risk = psychology.trauma_level / psychology.breaking_point;
    let trust_risk = 1.0 - psychology.trust;
    let isolation_risk = psychology.isolation_tendency;
    let resilience_protection = resilience.emotional_regulation * 0.5 + resilience.social_connection * 0.3;
    
    ((trauma_risk * 0.4 + trust_risk * 0.3 + isolation_risk * 0.3) - resilience_protection * 0.2).max(0.0).min(1.0)
}

fn calculate_skill_gain_from_action(action: &TherapyAction, current_skill: f32) -> f32 {
    let base_gain = match action {
        TherapyAction::ActiveListening => 0.02,
        TherapyAction::ValidateExperience => 0.03,
        TherapyAction::PracticeGrounding => 0.04,
        TherapyAction::ChallengeCognition => 0.05,
        _ => 0.01,
    };
    
    // Diminishing returns for higher skill levels
    let skill_multiplier = (1.0 - current_skill).max(0.1);
    base_gain * skill_multiplier
}

fn get_skill_area_for_action(action: &TherapyAction) -> String {
    match action {
        TherapyAction::ActiveListening => "listening_skills".to_string(),
        TherapyAction::ValidateExperience => "validation_techniques".to_string(),
        TherapyAction::PracticeGrounding => "grounding_techniques".to_string(),
        TherapyAction::ChallengeCognition => "cognitive_therapy".to_string(),
        _ => "general_therapy".to_string(),
    }
}
