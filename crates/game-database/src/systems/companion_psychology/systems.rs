//! Companion Psychology System - ECS Systems
//!
//! Systems for processing companion trauma, therapy quests, and psychological healing
//! in Dragon's Labyrinth's horror-first RPG experience.

use bevy::prelude::*;
use sea_orm::DatabaseConnection;
use database_orm::{companions, psychology};
use crate::systems::companion_psychology::{
    components::*, events::*, resources::*, queries::*
};
use uuid::Uuid;
use rand::Rng;
use std::collections::HashMap;

/// Initialize companion psychology from database
pub fn initialize_companion_psychology_system(
    mut commands: Commands,
    db: Res<DatabaseConnection>,
    mut psychology_state: ResMut<CompanionPsychologyState>,
    companions_query: Query<Entity, Added<companions::Model>>,
) {
    for entity in companions_query.iter() {
        // This would typically load from database and spawn psychology components
        // For now, we'll set up basic psychology state
        
        // Add psychology component with default values
        commands.entity(entity).insert(CompanionPsychology {
            companion_id: Uuid::new_v4(), // Would come from companion model
            companion_type: "einar".to_string(), // Would come from companion model
            trauma_level: 0.0,
            breaking_point: 3.5, // Companions break at different trauma thresholds
            loyalty: 0.8,
            trust: 0.6,
            trauma_sources: Vec::new(),
            trauma_triggers: Vec::new(),
            active_trauma_responses: Vec::new(),
            recovery_progress: 0.0,
            therapy_readiness: 0.3,
            breakthrough_potential: 0.1,
            therapeutic_bond: 0.0,
            support_network_quality: 0.5,
            isolation_tendency: 0.2,
        });
        
        // Add resilience component
        commands.entity(entity).insert(PsychologicalResilience {
            companion_id: Uuid::new_v4(),
            emotional_regulation: 0.5,
            cognitive_flexibility: 0.6,
            social_connection: 0.7,
            meaning_making: 0.4,
            self_efficacy: 0.5,
            healthy_coping_strategies: vec!["exercise".to_string(), "meditation".to_string()],
            unhealthy_coping_strategies: vec!["avoidance".to_string()],
            coping_strategy_effectiveness: vec![0.7, 0.6, -0.3],
            post_traumatic_growth: 0.0,
            wisdom_gained: 0.0,
            empathy_development: 0.0,
        });
    }
}

/// Process trauma accumulation from various game events
pub fn trauma_accumulation_system(
    mut trauma_events: EventReader<TraumaEvent>,
    mut psychology_query: Query<&mut CompanionPsychology>,
    mut breaking_point_events: EventWriter<CompanionBreakingPointEvent>,
    time: Res<Time>,
) {
    for trauma_event in trauma_events.read() {
        if let Ok(mut psychology) = psychology_query.get_mut(trauma_event.companion_entity) {
            // Calculate trauma impact based on event severity and companion resilience
            let trauma_impact = calculate_trauma_impact(&trauma_event, &psychology);
            
            // Add trauma source if new
            if !psychology.trauma_sources.iter().any(|source| source.source_id == trauma_event.source_id) {
                psychology.trauma_sources.push(TraumaSource {
                    source_id: trauma_event.source_id.clone(),
                    severity: trauma_impact,
                    acquisition_context: trauma_event.context.clone(),
                    active_triggers: trauma_event.triggers.clone(),
                    healing_progress: 0.0,
                    requires_professional_help: trauma_impact > 0.8,
                });
            }
            
            // Increase overall trauma level
            psychology.trauma_level += trauma_impact;
            psychology.trauma_level = psychology.trauma_level.min(5.0);
            
            // Decrease trust and loyalty based on trauma type
            match trauma_event.trauma_type.as_str() {
                "betrayal" => {
                    psychology.trust -= trauma_impact * 0.5;
                    psychology.loyalty -= trauma_impact * 0.3;
                }
                "abandonment" => {
                    psychology.isolation_tendency += trauma_impact * 0.2;
                    psychology.trust -= trauma_impact * 0.3;
                }
                "combat_death" => {
                    psychology.therapy_readiness += trauma_impact * 0.1; // Combat trauma makes them more open to help
                }
                _ => {}
            }
            
            // Check for breaking point
            if psychology.trauma_level >= psychology.breaking_point {
                breaking_point_events.send(CompanionBreakingPointEvent {
                    companion_entity: trauma_event.companion_entity,
                    companion_id: psychology.companion_id,
                    trigger_trauma: trauma_event.source_id.clone(),
                    breaking_point_type: determine_breaking_point_type(&psychology),
                });
            }
        }
    }
}

/// Process active trauma responses (flashbacks, panic attacks, etc.)
pub fn trauma_response_system(
    mut commands: Commands,
    mut psychology_query: Query<(Entity, &mut CompanionPsychology)>,
    mut trauma_trigger_events: EventReader<TraumaTriggerEvent>,
    time: Res<Time>,
) {
    // Process new trauma triggers
    for trigger_event in trauma_trigger_events.read() {
        if let Ok((entity, mut psychology)) = psychology_query.get_mut(trigger_event.companion_entity) {
            // Check if this trigger matches any trauma sources
            for trauma_source in &psychology.trauma_sources {
                if trauma_source.active_triggers.contains(&trigger_event.trigger_stimulus) {
                    // Create trauma response
                    let response_intensity = calculate_response_intensity(trauma_source, &psychology);
                    let response_duration = calculate_response_duration(trauma_source, response_intensity);
                    
                    let trauma_response = TraumaResponse {
                        response_type: determine_response_type(trauma_source, &psychology),
                        intensity: response_intensity,
                        duration_remaining: response_duration,
                        triggered_by: trigger_event.trigger_stimulus.clone(),
                        behavioral_effects: generate_behavioral_effects(&trauma_source, response_intensity),
                    };
                    
                    psychology.active_trauma_responses.push(trauma_response);
                }
            }
        }
    }
    
    // Update existing trauma responses
    for (entity, mut psychology) in psychology_query.iter_mut() {
        psychology.active_trauma_responses.retain_mut(|response| {
            response.duration_remaining -= time.delta_seconds();
            
            // Gradually reduce intensity over time
            if response.duration_remaining > 0.0 {
                response.intensity *= 0.99; // Slight decay
                true
            } else {
                false // Remove expired responses
            }
        });
    }
}

/// Process therapy quest progression
pub fn therapy_quest_system(
    mut therapy_query: Query<&mut TherapyQuest>,
    mut psychology_query: Query<&mut CompanionPsychology>,
    mut therapy_events: EventReader<TherapyActionEvent>,
    mut breakthrough_events: EventWriter<TherapyBreakthroughEvent>,
    mut therapy_resources: ResMut<TherapyResources>,
    time: Res<Time>,
) {
    for therapy_event in therapy_events.read() {
        if let Ok(mut quest) = therapy_query.get_mut(therapy_event.therapy_entity) {
            // Process therapy action
            let action_effectiveness = calculate_therapy_action_effectiveness(
                &therapy_event.action,
                &quest,
                &therapy_event,
            );
            
            // Update quest progress
            quest.progress += action_effectiveness * 0.1; // Progress increment
            quest.progress = quest.progress.min(1.0);
            
            // Check for stage progression
            if quest.progress >= 0.8 && matches!(quest.stage, TherapyStage::Processing) {
                // Check for breakthrough opportunity
                let breakthrough_chance = calculate_breakthrough_probability(&quest, action_effectiveness);
                if rand::thread_rng().gen::<f32>() < breakthrough_chance {
                    breakthrough_events.send(TherapyBreakthroughEvent {
                        companion_entity: therapy_event.companion_entity,
                        therapy_entity: therapy_event.therapy_entity,
                        breakthrough_type: "trauma_resolution".to_string(),
                        significance: breakthrough_chance,
                        insights_gained: generate_breakthrough_insights(&quest),
                    });
                    
                    quest.stage = TherapyStage::Breakthrough;
                }
            }
            
            // Stage transitions
            match quest.stage {
                TherapyStage::Assessment if quest.progress >= 1.0 => {
                    quest.stage = TherapyStage::Preparation;
                    quest.progress = 0.0;
                }
                TherapyStage::Preparation if quest.progress >= 1.0 => {
                    quest.stage = TherapyStage::Processing;
                    quest.progress = 0.0;
                }
                TherapyStage::Processing if quest.progress >= 1.0 => {
                    quest.stage = TherapyStage::Integration;
                    quest.progress = 0.0;
                }
                TherapyStage::Breakthrough => {
                    // After breakthrough, move to integration
                    quest.stage = TherapyStage::Integration;
                    quest.progress = 0.5; // Start integration with some progress
                }
                _ => {}
            }
        }
    }
}

/// Process therapy breakthroughs and their effects on companion psychology
pub fn therapy_breakthrough_system(
    mut breakthrough_events: EventReader<TherapyBreakthroughEvent>,
    mut psychology_query: Query<&mut CompanionPsychology>,
    mut milestone_events: EventWriter<RecoveryMilestoneEvent>,
    mut commands: Commands,
) {
    for breakthrough in breakthrough_events.read() {
        if let Ok(mut psychology) = psychology_query.get_mut(breakthrough.companion_entity) {
            // Apply breakthrough effects
            match breakthrough.breakthrough_type.as_str() {
                "trauma_resolution" => {
                    // Reduce trauma level
                    psychology.trauma_level -= breakthrough.significance * 0.5;
                    psychology.trauma_level = psychology.trauma_level.max(0.0);
                    
                    // Increase recovery progress
                    psychology.recovery_progress += breakthrough.significance * 0.3;
                    psychology.recovery_progress = psychology.recovery_progress.min(1.0);
                    
                    // Improve therapeutic bond
                    psychology.therapeutic_bond += breakthrough.significance * 0.2;
                    psychology.therapeutic_bond = psychology.therapeutic_bond.min(1.0);
                    
                    // Increase trust if player was involved
                    psychology.trust += breakthrough.significance * 0.1;
                    psychology.trust = psychology.trust.min(1.0);
                }
                "emotional_regulation" => {
                    psychology.isolation_tendency -= breakthrough.significance * 0.3;
                    psychology.isolation_tendency = psychology.isolation_tendency.max(0.0);
                }
                "narrative_coherence" => {
                    psychology.breakthrough_potential += breakthrough.significance * 0.2;
                }
                _ => {}
            }
            
            // Create recovery milestone
            milestone_events.send(RecoveryMilestoneEvent {
                companion_entity: breakthrough.companion_entity,
                milestone_type: format!("breakthrough_{}", breakthrough.breakthrough_type),
                significance: breakthrough.significance,
                celebration_appropriate: breakthrough.significance > 0.7,
            });
        }
    }
}

/// Process memory palace navigation for 3D therapy visualization
pub fn memory_palace_system(
    mut palace_query: Query<&mut MemoryPalace>,
    mut psychology_query: Query<&mut CompanionPsychology>,
    mut palace_events: EventReader<MemoryPalaceEvent>,
    mut healing_events: EventWriter<MemoryPalaceHealingEvent>,
    time: Res<Time>,
) {
    for palace_event in palace_events.read() {
        if let Ok(mut palace) = palace_query.get_mut(palace_event.palace_entity) {
            match &palace_event.action {
                MemoryPalaceAction::EnterRoom(room_id) => {
                    if let Some(room) = palace.room_configurations.iter()
                        .find(|r| r.room_id == *room_id) {
                        
                        // Check accessibility
                        if room.accessibility > 0.5 {
                            if !palace.explored_areas.contains(room_id) {
                                palace.explored_areas.push(room_id.clone());
                                
                                // Potential for healing in this room
                                if room.therapeutic_value > 0.6 {
                                    healing_events.send(MemoryPalaceHealingEvent {
                                        palace_entity: palace_event.palace_entity,
                                        companion_entity: palace_event.companion_entity,
                                        room_id: room_id.clone(),
                                        healing_type: room.room_type.clone(),
                                        healing_power: room.therapeutic_value,
                                    });
                                }
                            }
                        }
                    }
                }
                MemoryPalaceAction::InteractWithTraumaObject(object_id) => {
                    if let Some(trauma_obj) = palace.trauma_representations.iter_mut()
                        .find(|t| t.object_id == *object_id) {
                        
                        // Process interaction based on trauma object properties
                        if trauma_obj.transformation_potential > 0.5 {
                            trauma_obj.emotional_charge *= 0.9; // Reduce emotional charge
                            
                            healing_events.send(MemoryPalaceHealingEvent {
                                palace_entity: palace_event.palace_entity,
                                companion_entity: palace_event.companion_entity,
                                room_id: "trauma_processing".to_string(),
                                healing_type: "trauma_transformation".to_string(),
                                healing_power: trauma_obj.transformation_potential,
                            });
                        }
                    }
                }
                MemoryPalaceAction::ActivateHealingSymbol(symbol_id) => {
                    if let Some(healing_symbol) = palace.healing_symbols.iter()
                        .find(|h| h.symbol_id == *symbol_id) {
                        
                        healing_events.send(MemoryPalaceHealingEvent {
                            palace_entity: palace_event.palace_entity,
                            companion_entity: palace_event.companion_entity,
                            room_id: "healing_activation".to_string(),
                            healing_type: healing_symbol.symbol_type.clone(),
                            healing_power: healing_symbol.healing_power,
                        });
                    }
                }
            }
        }
    }
}

/// Process companion support relationships
pub fn companion_support_system(
    support_query: Query<&CompanionSupport>,
    mut psychology_query: Query<&mut CompanionPsychology>,
    mut support_events: EventReader<CompanionSupportEvent>,
    time: Res<Time>,
) {
    for support_event in support_events.read() {
        // Find support relationships involving these companions
        for support in support_query.iter() {
            if support.supporter_id == support_event.supporter_id &&
               support.supported_id == support_event.supported_id {
                
                // Apply support effects
                if let Ok(mut psychology) = psychology_query.get_mut(support_event.supported_entity) {
                    match support.support_type.as_str() {
                        "peer_counseling" => {
                            psychology.support_network_quality += support.effectiveness * 0.1;
                            psychology.isolation_tendency -= support.effectiveness * 0.05;
                        }
                        "shared_experience" => {
                            psychology.therapeutic_bond += support.effectiveness * 0.05;
                            psychology.therapy_readiness += support.effectiveness * 0.03;
                        }
                        "protective" => {
                            psychology.trust += support.effectiveness * 0.02;
                            psychology.loyalty += support.effectiveness * 0.01;
                        }
                        _ => {}
                    }
                }
                
                // Mutual benefit for supporter
                if let Ok(mut supporter_psychology) = psychology_query.get_mut(support_event.supporter_entity) {
                    supporter_psychology.self_efficacy += support.mutual_benefit * 0.02;
                    supporter_psychology.empathy_development += support.mutual_benefit * 0.01;
                }
            }
        }
    }
}

/// Update therapy session progress and tracking
pub fn therapy_session_system(
    mut session_query: Query<&mut TherapySession>,
    psychology_query: Query<&CompanionPsychology>,
    mut session_events: EventReader<TherapySessionEvent>,
    mut session_complete_events: EventWriter<TherapySessionCompleteEvent>,
    time: Res<Time>,
) {
    for session_event in session_events.read() {
        if let Ok(mut session) = session_query.get_mut(session_event.session_entity) {
            match &session_event.session_action {
                TherapySessionAction::Start => {
                    // Initialize session state
                    if let Ok(psychology) = psychology_query.get(session_event.companion_entity) {
                        session.emotional_state_start = calculate_emotional_state(psychology);
                    }
                }
                TherapySessionAction::Progress(activity) => {
                    // Process therapeutic activity
                    session.insights_gained.push(format!("Insight from {}", activity));
                }
                TherapySessionAction::Complete => {
                    // Finalize session
                    if let Ok(psychology) = psychology_query.get(session_event.companion_entity) {
                        session.emotional_state_end = calculate_emotional_state(psychology);
                        
                        session_complete_events.send(TherapySessionCompleteEvent {
                            session_entity: session_event.session_entity,
                            companion_entity: session_event.companion_entity,
                            emotional_progress: session.emotional_state_end - session.emotional_state_start,
                            insights_count: session.insights_gained.len(),
                            breakthrough_occurred: session.breakthrough_potential > 0.8,
                        });
                    }
                }
            }
        }
    }
}

/// Monitor and process companion breaking points
pub fn breaking_point_monitoring_system(
    mut breaking_point_events: EventReader<CompanionBreakingPointEvent>,
    mut psychology_query: Query<&mut CompanionPsychology>,
    mut companion_departure_events: EventWriter<CompanionDepartureEvent>,
    mut commands: Commands,
) {
    for breaking_point in breaking_point_events.read() {
        if let Ok(mut psychology) = psychology_query.get_mut(breaking_point.companion_entity) {
            match breaking_point.breaking_point_type.as_str() {
                "traumatic_breakdown" => {
                    // Companion temporarily leaves for self-care
                    companion_departure_events.send(CompanionDepartureEvent {
                        companion_entity: breaking_point.companion_entity,
                        departure_type: "temporary_breakdown".to_string(),
                        return_conditions: vec![
                            "therapy_progress > 0.5".to_string(),
                            "player_support_demonstrated".to_string(),
                        ],
                        estimated_departure_duration: 30.0 * 24.0 * 3600.0, // 30 days in seconds
                    });
                }
                "trust_betrayal" => {
                    // Companion leaves due to trust issues
                    psychology.trust = 0.0;
                    psychology.loyalty *= 0.3;
                    
                    companion_departure_events.send(CompanionDepartureEvent {
                        companion_entity: breaking_point.companion_entity,
                        departure_type: "trust_betrayal".to_string(),
                        return_conditions: vec![
                            "major_trust_rebuilding_quest".to_string(),
                            "public_vindication".to_string(),
                        ],
                        estimated_departure_duration: -1.0, // Indefinite
                    });
                }
                "overwhelmed_shutdown" => {
                    // Companion becomes unresponsive but doesn't leave
                    psychology.isolation_tendency = 1.0;
                    psychology.therapy_readiness = 0.0;
                }
                _ => {}
            }
        }
    }
}

// Helper functions for calculations

fn calculate_trauma_impact(trauma_event: &TraumaEvent, psychology: &CompanionPsychology) -> f32 {
    let base_impact = trauma_event.severity;
    
    // Adjust based on companion's resilience and current state
    let resilience_modifier = 1.0 - (psychology.trust * 0.3 + psychology.loyalty * 0.2);
    let current_trauma_modifier = 1.0 + (psychology.trauma_level * 0.1);
    
    (base_impact * resilience_modifier * current_trauma_modifier).min(2.0)
}

fn determine_breaking_point_type(psychology: &CompanionPsychology) -> String {
    if psychology.trust < 0.2 {
        "trust_betrayal".to_string()
    } else if psychology.isolation_tendency > 0.8 {
        "overwhelmed_shutdown".to_string()
    } else {
        "traumatic_breakdown".to_string()
    }
}

fn calculate_response_intensity(trauma_source: &TraumaSource, psychology: &CompanionPsychology) -> f32 {
    let base_intensity = trauma_source.severity;
    let healing_reduction = trauma_source.healing_progress * 0.5;
    let current_trauma_amplification = psychology.trauma_level * 0.1;
    
    ((base_intensity - healing_reduction) * (1.0 + current_trauma_amplification)).max(0.1).min(1.0)
}

fn calculate_response_duration(trauma_source: &TraumaSource, intensity: f32) -> f32 {
    // Duration in seconds, based on trauma severity and current intensity
    let base_duration = 30.0 + (trauma_source.severity * 300.0); // 30 seconds to 5 minutes
    base_duration * intensity
}

fn determine_response_type(trauma_source: &TraumaSource, psychology: &CompanionPsychology) -> String {
    match trauma_source.source_id.as_str() {
        "combat_death" => "flashback".to_string(),
        "dragon_encounter" => "panic".to_string(),
        "betrayal" => "anger".to_string(),
        "abandonment" => "dissociation".to_string(),
        _ => {
            if psychology.isolation_tendency > 0.6 {
                "withdrawal".to_string()
            } else {
                "flashback".to_string()
            }
        }
    }
}

fn generate_behavioral_effects(trauma_source: &TraumaSource, intensity: f32) -> Vec<BehavioralEffect> {
    let mut effects = Vec::new();
    
    // Combat effectiveness reduction
    effects.push(BehavioralEffect::CombatEffectiveness(-intensity * 0.5));
    
    // Movement hesitation for severe trauma
    if intensity > 0.7 {
        effects.push(BehavioralEffect::MovementHesitation(intensity * 0.3));
    }
    
    // Dialogue restrictions based on trauma type
    match trauma_source.source_id.as_str() {
        "betrayal" => {
            effects.push(BehavioralEffect::DialogueRestriction(vec![
                "trust_building".to_string(),
                "future_planning".to_string(),
            ]));
        }
        "combat_death" => {
            effects.push(BehavioralEffect::DialogueRestriction(vec![
                "combat_strategy".to_string(),
                "casualty_discussion".to_string(),
            ]));
        }
        _ => {}
    }
    
    effects
}

fn calculate_therapy_action_effectiveness(
    action: &TherapyAction,
    quest: &TherapyQuest,
    event: &TherapyActionEvent,
) -> f32 {
    match action {
        TherapyAction::ActiveListening => 0.8, // Almost always effective
        TherapyAction::ValidateExperience => 0.9, // Validation is crucial
        TherapyAction::AskGentleQuestion(_) => 0.6, // Depends on context
        TherapyAction::SharePersonalExperience => 0.7, // Can be very effective
        TherapyAction::PracticeGrounding => 0.8, // Grounding techniques work well
        _ => 0.5, // Default effectiveness
    }
}

fn calculate_breakthrough_probability(quest: &TherapyQuest, action_effectiveness: f32) -> f32 {
    let base_probability = 0.05; // 5% base chance
    let progress_modifier = quest.progress * 0.3; // Up to 30% from progress
    let effectiveness_modifier = action_effectiveness * 0.2; // Up to 20% from effective actions
    let stage_modifier = match quest.stage {
        TherapyStage::Processing => 0.4, // Higher chance during processing
        TherapyStage::Integration => 0.2,
        _ => 0.0,
    };
    
    (base_probability + progress_modifier + effectiveness_modifier + stage_modifier).min(0.8)
}

fn generate_breakthrough_insights(quest: &TherapyQuest) -> Vec<String> {
    match quest.trauma_focus.as_str() {
        "combat_death" => vec![
            "I understand that I couldn't have prevented what happened".to_string(),
            "Surviving doesn't make me responsible for others' deaths".to_string(),
            "I can honor the fallen by living well".to_string(),
        ],
        "betrayal" => vec![
            "Not everyone who seems trustworthy will betray me".to_string(),
            "I can rebuild trust gradually and safely".to_string(),
            "My worth isn't determined by others' actions".to_string(),
        ],
        _ => vec![
            "I have the strength to heal from this".to_string(),
            "My experiences, painful as they are, have value".to_string(),
        ],
    }
}

fn calculate_emotional_state(psychology: &CompanionPsychology) -> f32 {
    let trauma_impact = -psychology.trauma_level * 0.2;
    let recovery_benefit = psychology.recovery_progress * 0.5;
    let trust_benefit = psychology.trust * 0.3;
    let support_benefit = psychology.support_network_quality * 0.2;
    
    (0.5 + trauma_impact + recovery_benefit + trust_benefit + support_benefit).max(0.0).min(1.0)
}
