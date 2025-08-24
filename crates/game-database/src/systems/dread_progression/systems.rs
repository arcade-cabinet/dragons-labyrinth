//! Dread Progression System - ECS Systems
//!
//! Production-ready systems for the master horror orchestrator that transforms ALL
//! Dragon's Labyrinth game systems based on dread level progression (0-4).

use bevy::prelude::*;
use sea_orm::DatabaseConnection;
use database_orm::hex_tiles;
use uuid::Uuid;
use std::collections::HashMap;
use crate::systems::dread_progression::{
    components::*, resources::*, events::*
};
use crate::systems::companion_psychology::events::{
    DreadPsychologyEvent, TraumaEvent
};

/// Master system that calculates current dread level from all sources
pub fn dread_level_calculation_system(
    mut dread_state: ResMut<DreadProgressionState>,
    mut dread_level_query: Query<&mut DreadLevel>,
    aura_manager: Res<DreadAuraManager>,
    dread_config: Res<DreadProgressionConfig>,
    mut dread_change_events: EventWriter<DreadLevelChangeEvent>,
    mut dread_source_events: EventReader<DreadSourceEvent>,
    time: Res<Time>,
) {
    // Process new dread source events
    for source_event in dread_source_events.read() {
        match &source_event.action {
            DreadSourceAction::Add => {
                dread_state.add_dread_source(source_event.source.clone());
                info!("Added dread source: {} with intensity {}", 
                      source_event.source.source_id, source_event.source.intensity);
            }
            DreadSourceAction::Remove(source_id) => {
                dread_state.remove_dread_source(source_id);
                info!("Removed dread source: {}", source_id);
            }
            DreadSourceAction::Modify(source_id) => {
                if let Some(existing_source) = dread_state.active_dread_sources.get_mut(source_id) {
                    // Update source with new data
                    *existing_source = source_event.source.clone();
                    debug!("Modified dread source: {}", source_id);
                }
            }
            DreadSourceAction::Amplify(source_id, multiplier) => {
                if let Some(existing_source) = dread_state.active_dread_sources.get_mut(source_id) {
                    existing_source.intensity *= multiplier;
                    debug!("Amplified dread source {} by {:.2}x", source_id, multiplier);
                }
            }
            DreadSourceAction::Decay(source_id, decay_amount) => {
                if let Some(existing_source) = dread_state.active_dread_sources.get_mut(source_id) {
                    existing_source.intensity -= decay_amount * time.delta_seconds();
                    if existing_source.intensity <= 0.0 {
                        dread_state.remove_dread_source(source_id);
                        debug!("Dread source {} decayed to zero and was removed", source_id);
                    }
                }
            }
            _ => {} // Handle other actions as needed
        }
    }

    // Calculate total dread from all sources
    let total_dread = dread_state.calculate_total_dread();
    let new_dread_level = dread_state.get_dread_level(total_dread, &dread_config);
    let old_dread_level = dread_state.global_dread_level;

    // Check if dread level has changed
    if new_dread_level != old_dread_level {
        warn!("Dread level changing from {} to {} (total dread: {:.2})", 
              old_dread_level, new_dread_level, total_dread);

        // Record the change
        dread_state.record_dread_change(
            old_dread_level,
            new_dread_level,
            "dread_accumulation".to_string(),
            vec!["all_systems".to_string()],
        );

        // Update global dread level
        dread_state.global_dread_level = new_dread_level;

        // Send dread level change event
        dread_change_events.send(DreadLevelChangeEvent {
            old_level: old_dread_level,
            new_level: new_dread_level,
            trigger_source: "dread_accumulation".to_string(),
            affected_regions: vec!["global".to_string()],
            transition_speed: dread_config.transition_speeds[new_dread_level as usize],
            forced_transition: false,
            player_adaptation_override: false,
            system_priority_overrides: HashMap::new(),
        });

        // Update all DreadLevel components
        for mut dread_level in dread_level_query.iter_mut() {
            dread_level.previous_level = dread_level.current_level;
            dread_level.current_level = new_dread_level;
            dread_level.last_update = chrono::Utc::now().timestamp();
        }
    }

    // Update active dread sources (decay over time)
    let sources_to_update: Vec<String> = dread_state.active_dread_sources.keys().cloned().collect();
    for source_id in sources_to_update {
        if let Some(source) = dread_state.active_dread_sources.get_mut(&source_id) {
            // Apply natural decay
            if source.decay_rate > 0.0 {
                source.intensity -= source.decay_rate * time.delta_seconds();
                if source.intensity <= 0.0 {
                    dread_state.active_dread_sources.remove(&source_id);
                    debug!("Dread source {} naturally decayed away", source_id);
                }
            }

            // Update duration
            if source.duration_remaining > 0.0 {
                source.duration_remaining -= time.delta_seconds();
                if source.duration_remaining <= 0.0 {
                    dread_state.active_dread_sources.remove(&source_id);
                    debug!("Dread source {} expired", source_id);
                }
            }
        }
    }
}

/// System that transforms all game systems based on current dread level
pub fn system_transformation_orchestrator(
    mut transformation_events: EventReader<DreadLevelChangeEvent>,
    mut system_transformation_events: EventWriter<SystemTransformationEvent>,
    mut parameter_modification_events: EventWriter<DreadParameterModificationEvent>,
    mut feature_toggle_events: EventWriter<DreadFeatureToggleEvent>,
    dread_config: Res<DreadProgressionConfig>,
    mut dread_state: ResMut<DreadProgressionState>,
) {
    for change_event in transformation_events.read() {
        info!("Orchestrating system transformations for dread level change {} -> {}", 
              change_event.old_level, change_event.new_level);

        // Transform each system based on its configuration
        for (system_name, system_config) in &dread_config.system_configs {
            let target_dread = change_event.new_level as f32;
            
            // Check if this system should be transformed at this dread level
            if target_dread >= system_config.transformation_threshold[change_event.new_level as usize] {
                // Send system transformation event
                system_transformation_events.send(SystemTransformationEvent {
                    system_name: system_name.clone(),
                    target_dread_level: change_event.new_level,
                    transformation_reason: format!("Dread level increased to {}", change_event.new_level),
                    override_existing: false,
                    transformation_parameters: HashMap::new(),
                    rollback_conditions: vec![format!("dread_level < {}", change_event.new_level)],
                    affected_entities: vec![],
                });

                // Generate parameter modifications for this system
                let mut parameter_modifications = Vec::new();
                for (parameter_name, scaling_values) in &system_config.parameter_scaling {
                    let new_value = scaling_values[change_event.new_level as usize];
                    parameter_modifications.push(ParameterModification {
                        parameter_name: parameter_name.clone(),
                        modification_type: "multiply".to_string(),
                        modification_value: new_value,
                        original_value: Some(1.0), // Would store actual original value
                        transition_speed: change_event.transition_speed,
                    });
                }

                if !parameter_modifications.is_empty() {
                    parameter_modification_events.send(DreadParameterModificationEvent {
                        system_name: system_name.clone(),
                        parameter_modifications,
                        modification_reason: format!("Dread level {}", change_event.new_level),
                        priority: 5, // Standard priority
                        duration: None, // Permanent while dread level maintained
                        rollback_triggers: vec![format!("dread_level < {}", change_event.new_level)],
                    });
                }

                // Generate feature toggles for this system
                let mut feature_toggles = Vec::new();
                for (feature_name, availability_values) in &system_config.feature_availability {
                    let should_be_enabled = availability_values[change_event.new_level as usize];
                    feature_toggles.push(FeatureToggle {
                        feature_name: feature_name.clone(),
                        new_state: should_be_enabled,
                        toggle_conditions: vec![format!("dread_level >= {}", change_event.new_level)],
                        alternative_features: vec![], // Could specify alternative features
                        restoration_requirements: vec![format!("dread_level < {}", change_event.new_level)],
                    });
                }

                if !feature_toggles.is_empty() {
                    feature_toggle_events.send(DreadFeatureToggleEvent {
                        system_name: system_name.clone(),
                        feature_toggles,
                        toggle_reason: format!("Dread level {}", change_event.new_level),
                        affected_entities: vec![],
                        player_notification: change_event.new_level >= 2, // Notify at medium+ dread
                        companion_awareness: HashMap::new(), // Would be populated with companion data
                    });
                }
            }
        }
    }
}

/// System for processing dragon presence and its effect on dread
pub fn dragon_presence_dread_system(
    mut dragon_presence_events: EventReader<DragonPresenceEvent>,
    mut dread_source_events: EventWriter<DreadSourceEvent>,
    dread_config: Res<DreadProgressionConfig>,
    time: Res<Time>,
) {
    for dragon_event in dragon_presence_events.read() {
        // Calculate dread contribution based on distance
        let mut dread_intensity = 0.0;
        for (distance, dread_value) in &dread_config.dragon_presence_dread_curve {
            if dragon_event.player_distance <= *distance {
                dread_intensity = *dread_value;
                break;
            }
        }

        // Amplify based on dragon intelligence level
        dread_intensity *= dread_config.dragon_intelligence_dread_factor * dragon_event.dragon_intelligence_level;

        // Add stalking buildup if dragon is stalking
        if dragon_event.dragon_activity == "stalking" {
            dread_intensity += dread_config.dragon_stalking_dread_buildup * dragon_event.stalking_duration;
        }

        // Create or update dragon presence dread source
        let dragon_source = DreadSource {
            source_id: format!("dragon_presence_{:?}", dragon_event.dragon_entity),
            source_type: "supernatural".to_string(),
            intensity: dread_intensity,
            decay_rate: 0.1, // Dragon presence decays when dragon leaves
            radius: Some(1000.0), // Large radius for dragon presence
            duration_remaining: -1.0, // Infinite duration while dragon is present
            compounding_factor: 0.5, // Dragon presence compounds with other fears
            description: format!("The terrible presence of the dragon, {} units away", dragon_event.player_distance),
        };

        dread_source_events.send(DreadSourceEvent {
            action: DreadSourceAction::Add,
            source: dragon_source,
            spatial_position: Some(dragon_event.dragon_position),
            affected_entities: vec![],
            propagation_rules: vec![
                DreadPropagationRule {
                    propagation_type: "spatial".to_string(),
                    propagation_speed: 100.0, // Fast propagation
                    propagation_decay: 0.1,   // Some decay over distance
                    propagation_barriers: vec!["solid_walls".to_string(), "blessed_ground".to_string()],
                    propagation_amplifiers: vec!["open_spaces".to_string(), "darkness".to_string()],
                }
            ],
        });

        warn!("Dragon presence generating {:.2} dread at distance {:.1}", 
              dread_intensity, dragon_event.player_distance);
    }
}

/// System for processing companion psychology effects on dread
pub fn companion_dread_integration_system(
    mut companion_dread_events: EventReader<CompanionDreadEvent>,
    mut dread_source_events: EventWriter<DreadSourceEvent>,
    mut dread_psychology_events: EventWriter<DreadPsychologyEvent>,
    dread_config: Res<DreadProgressionConfig>,
    dread_state: Res<DreadProgressionState>,
) {
    for companion_event in companion_dread_events.read() {
        match &companion_event.dread_effect {
            CompanionDreadEffect::ReduceDread(reduction) => {
                // Companion provides dread resistance
                let resistance_source_id = format!("{}_support", companion_event.companion_type);
                let resistance_amount = dread_config.companion_dread_resistances
                    .get(&resistance_source_id)
                    .copied()
                    .unwrap_or(*reduction);

                debug!("Companion {} providing {:.2} dread resistance", 
                       companion_event.companion_type, resistance_amount);
            }
            
            CompanionDreadEffect::IncreaseDread(increase) => {
                // Companion is adding to dread (breakdown, trauma, etc.)
                let source_id = format!("{}_breakdown", companion_event.companion_type);
                let contribution = dread_config.companion_dread_contributions
                    .get(&source_id)
                    .copied()
                    .unwrap_or(*increase);

                let companion_dread_source = DreadSource {
                    source_id: source_id.clone(),
                    source_type: "psychological".to_string(),
                    intensity: contribution * companion_event.trauma_level,
                    decay_rate: 0.05, // Slowly improves with support
                    radius: Some(50.0), // Close proximity effect
                    duration_remaining: -1.0, // Ongoing while companion traumatized
                    compounding_factor: 0.3,   // Trauma spreads to others
                    description: format!("{} is struggling with trauma and adding to party stress", 
                                        companion_event.companion_type),
                };

                dread_source_events.send(DreadSourceEvent {
                    action: DreadSourceAction::Add,
                    source: companion_dread_source,
                    spatial_position: None, // Travels with party
                    affected_entities: vec![companion_event.companion_entity],
                    propagation_rules: vec![
                        DreadPropagationRule {
                            propagation_type: "social".to_string(),
                            propagation_speed: 10.0,
                            propagation_decay: 0.2,
                            propagation_barriers: vec!["emotional_walls".to_string()],
                            propagation_amplifiers: vec!["empathy".to_string(), "shared_trauma".to_string()],
                        }
                    ],
                });

                warn!("Companion {} adding {:.2} dread due to trauma level {:.2}", 
                      companion_event.companion_type, contribution, companion_event.trauma_level);
            }

            CompanionDreadEffect::AbsorbDread(absorption) => {
                // Companion is taking on dread from others (protective behavior)
                debug!("Companion {} absorbing {:.2} dread from party", 
                       companion_event.companion_type, absorption);
                
                // This would generate trauma for the companion
                // Send to companion psychology system
                dread_psychology_events.send(DreadPsychologyEvent {
                    current_dread_level: dread_state.global_dread_level,
                    previous_dread_level: dread_state.global_dread_level,
                    affected_companions: vec![companion_event.companion_id],
                    psychological_effects: vec![],
                    system_modifications: vec!["companion_psychology".to_string()],
                });
            }

            _ => {} // Handle other companion dread effects
        }
    }
}

/// System for environmental dread calculation and application
pub fn environmental_dread_system(
    mut environmental_events: EventReader<EnvironmentalDreadEvent>,
    mut dread_source_events: EventWriter<DreadSourceEvent>,
    dread_config: Res<DreadProgressionConfig>,
    hex_tiles_query: Query<&hex_tiles::Model>,
) {
    for env_event in environmental_events.read() {
        let mut total_environmental_dread = 0.0;

        // Calculate base environmental dread
        total_environmental_dread += env_event.corruption_level * dread_config.corruption_dread_scaling;
        total_environmental_dread += env_event.isolation_factor * dread_config.isolation_dread_factor;
        total_environmental_dread += env_event.historical_trauma;

        // Reduce dread based on available escape routes and safe zones
        let escape_factor = 1.0 - (env_event.escape_routes_available as f32 * 0.1).min(0.5);
        let safety_factor = 1.0 - (env_event.safe_zones_present as f32 * 0.15).min(0.3);
        total_environmental_dread *= escape_factor * safety_factor;

        // Apply environmental factors
        for factor in &env_event.environmental_factors {
            match factor.factor_type.as_str() {
                "lighting" => {
                    if factor.intensity > 0.7 { // Very dark
                        total_environmental_dread += 0.5;
                    }
                }
                "sounds" => {
                    if factor.intensity > 0.6 { // Unnatural/threatening sounds
                        total_environmental_dread += 0.3;
                    }
                }
                "temperature" => {
                    if factor.intensity > 0.8 || factor.intensity < 0.2 { // Extreme temperatures
                        total_environmental_dread += 0.2;
                    }
                }
                _ => {}
            }
        }

        // Create environmental dread source
        if total_environmental_dread > 0.1 {
            let environmental_source = DreadSource {
                source_id: format!("environment_{}", env_event.location_id),
                source_type: "environmental".to_string(),
                intensity: total_environmental_dread,
                decay_rate: 0.02, // Slowly fades as you get used to environment
                radius: Some(200.0), // Local area effect
                duration_remaining: -1.0, // Permanent for this location
                compounding_factor: 0.2, // Environmental dread compounds with others
                description: format!("The oppressive atmosphere of {}", env_event.location_id),
            };

            dread_source_events.send(DreadSourceEvent {
                action: DreadSourceAction::Add,
                source: environmental_source,
                spatial_position: None, // Location-based
                affected_entities: vec![],
                propagation_rules: vec![
                    DreadPropagationRule {
                        propagation_type: "spatial".to_string(),
                        propagation_speed: 5.0, // Slow environmental spread
                        propagation_decay: 0.3, // Significant decay over distance
                        propagation_barriers: vec!["terrain_barriers".to_string()],
                        propagation_amplifiers: vec!["wind".to_string(), "echoing_spaces".to_string()],
                    }
                ],
            });

            debug!("Environmental dread {:.2} at location {}", 
                   total_environmental_dread, env_event.location_id);
        }
    }
}

/// System for reality distortion at high dread levels (3-4)
pub fn reality_distortion_system(
    mut distortion_events: EventReader<RealityDistortionEvent>,
    mut system_corruption_events: EventWriter<SystemCorruptionEvent>,
    mut parameter_modification_events: EventWriter<DreadParameterModificationEvent>,
    dread_state: Res<DreadProgressionState>,
    mut distortion_manager: ResMut<RealityDistortionManager>,
) {
    // Only process reality distortions at dread level 3+
    if dread_state.global_dread_level < 3 {
        return;
    }

    for distortion_event in distortion_events.read() {
        warn!("Reality distortion manifesting: {} at intensity {:.2}", 
              distortion_event.distortion_type, distortion_event.manifestation.requires_dread_level);

        // Apply gameplay effects from reality distortion
        for effect in &distortion_event.gameplay_effects {
            match effect.effect_type.as_str() {
                "navigation_interference" => {
                    // Corrupt navigation systems
                    system_corruption_events.send(SystemCorruptionEvent {
                        system_name: "hex_rendering".to_string(),
                        corruption_type: "navigation_distortion".to_string(),
                        corruption_level: effect.effect_strength,
                        corruption_source: distortion_event.distortion_type.clone(),
                        affected_parameters: vec!["waypoint_accuracy".to_string(), "distance_calculation".to_string()],
                        recovery_possible: true,
                        recovery_requirements: vec!["dread_level < 3".to_string(), "reality_anchor".to_string()],
                        cascading_effects: vec!["movement_system".to_string()],
                    });
                }
                "combat_modifier" => {
                    // Modify combat system parameters
                    parameter_modification_events.send(DreadParameterModificationEvent {
                        system_name: "combat".to_string(),
                        parameter_modifications: vec![
                            ParameterModification {
                                parameter_name: "reality_stability".to_string(),
                                modification_type: "multiply".to_string(),
                                modification_value: 1.0 - effect.effect_strength,
                                original_value: Some(1.0),
                                transition_speed: 2.0,
                            }
                        ],
                        modification_reason: format!("Reality distortion: {}", distortion_event.distortion_type),
                        priority: 8, // High priority for reality distortion
                        duration: effect.duration,
                        rollback_triggers: vec!["reality_distortion_ends".to_string()],
                    });
                }
                "dialogue_corruption" => {
                    // Corrupt dialogue system
                    system_corruption_events.send(SystemCorruptionEvent {
                        system_name: "dialogue".to_string(),
                        corruption_type: "perceptual_distortion".to_string(),
                        corruption_level: effect.effect_strength,
                        corruption_source: distortion_event.distortion_type.clone(),
                        affected_parameters: vec!["text_coherence".to_string(), "emotional_clarity".to_string()],
                        recovery_possible: true,
                        recovery_requirements: vec!["reality_anchor".to_string()],
                        cascading_effects: vec!["companion_psychology".to_string()],
                    });
                }
                _ => {}
            }
        }

        info!("Applied {} reality distortion effects for {}", 
              distortion_event.gameplay_effects.len(), distortion_event.distortion_type);
    }
}

/// System for processing narrative dread progression
pub fn narrative_dread_system(
    mut narrative_events: EventReader<NarrativeDreadEvent>,
    mut dread_source_events: EventWriter<DreadSourceEvent>,
    mut milestone_events: EventWriter<DreadMilestoneEvent>,
    dread_config: Res<DreadProgressionConfig>,
    dread_state: Res<DreadProgressionState>,
) {
    for narrative_event in narrative_events.read() {
        // Calculate narrative dread contribution
        let mut narrative_dread = 0.0;
        
        // Base dread from current act
        let act_index = (narrative_event.narrative_progression.act - 1) as usize;
        if act_index < dread_config.act_dread_baselines.len() {
            narrative_dread += dread_config.act_dread_baselines[act_index];
        }

        // Add dread from narrative tension and dramatic irony
        narrative_dread += narrative_event.narrative_progression.narrative_tension * 0.5;
        narrative_dread += narrative_event.narrative_progression.dramatic_irony_level * 0.3;
        narrative_dread += narrative_event.narrative_progression.foreshadowing_intensity * 0.4;

        // Reduce dread based on player agency
        narrative_dread *= 1.0 + (dread_config.player_agency_dread_relationship * 
                                  narrative_event.narrative_progression.character_agency);

        // Create narrative dread source
        let narrative_source = DreadSource {
            source_id: format!("narrative_{}", narrative_event.story_beat_id),
            source_type: "narrative".to_string(),
            intensity: narrative_dread,
            decay_rate: 0.01, // Very slow decay for narrative dread
            radius: None,      // Global effect
            duration_remaining: -1.0, // Permanent for this story beat
            compounding_factor: 0.4,   // Narrative dread strongly compounds
            description: format!("Growing tension from story progression: {}", 
                                narrative_event.story_beat_id),
        };

        dread_source_events.send(DreadSourceEvent {
            action: DreadSourceAction::Add,
            source: narrative_source,
            spatial_position: None,
            affected_entities: vec![],
            propagation_rules: vec![
                DreadPropagationRule {
                    propagation_type: "narrative".to_string(),
                    propagation_speed: 1.0, // Instant narrative propagation
                    propagation_decay: 0.0, // No decay for narrative effects
                    propagation_barriers: vec![],
                    propagation_amplifiers: vec!["player_attention".to_string(), "companion_awareness".to_string()],
                }
            ],
        });

        // Check for narrative dread spikes
        for spike in &dread_config.revelation_dread_spikes {
            if spike.trigger_event == narrative_event.revelation_type &&
               dread_state.global_dread_level >= spike.prerequisite_dread_level {
                
                debug!("Narrative dread spike triggered: {} with intensity {}", 
                       spike.trigger_event, spike.spike_intensity);
                
                // This would trigger a DreadSpikeEvent
                // Implementation would depend on how spikes are processed
            }
        }

        info!("Narrative dread {:.2} from story beat {}", 
              narrative_dread, narrative_event.story_beat_id);
    }
}

/// System for player adaptation to dread levels
pub fn player_dread_adaptation_system(
    mut adaptation_events: EventReader<PlayerDreadAdaptationEvent>,
    mut dread_state: ResMut<DreadProgressionState>,
    time: Res<Time>,
) {
    for adaptation_event in adaptation_events.read() {
        let player_adaptation = dread_state.get_player_adaptation(adaptation_event.player_id);

        match adaptation_event.adaptation_type {
            AdaptationType::Habituation => {
                // Player is getting used to this dread level
                let adaptation_gained = adaptation_event.adaptation_gained;
                let dread_index = adaptation_event.dread_level_exposed as usize;
                
                if dread_index < 5 {
                    player_adaptation.current_adaptation[dread_index] += adaptation_gained;
                    player_adaptation.current_adaptation[dread_index] = 
                        player_adaptation.current_adaptation[dread_index].min(1.0);
                    
                    debug!("Player {} gained {:.2} adaptation to dread level {}", 
                           adaptation_event.player_id, adaptation_gained, adaptation_event.dread_level_exposed);
                }

                // Update habituation curves for specific sources
                for source_id in &adaptation_event.habituation_sources {
                    let habituation_curve = player_adaptation.habituation_curves
                        .entry(source_id.clone())
                        .or_insert_with(|| HabituationCurve {
                            source_id: source_id.clone(),
                            exposure_time: 0.0,
                            habituation_rate: 0.01,
                            maximum_habituation: 0.8,
                            current_habituation: 0.0,
                            decay_rate: 0.001,
                        });
                    
                    habituation_curve.exposure_time += adaptation_event.exposure_duration;
                    habituation_curve.current_habituation += adaptation_gained;
                    habituation_curve.current_habituation = 
                        habituation_curve.current_habituation.min(habituation_curve.maximum_habituation);
                }
            }
            
            AdaptationType::Sensitization => {
                // Player is becoming more sensitive to dread
                let sensitization_amount = adaptation_event.adaptation_gained;
                player_adaptation.baseline_sensitivity += sensitization_amount;
                
                for trigger in &adaptation_event.sensitization_triggers {
                    if !player_adaptation.sensitization_triggers.contains(trigger) {
                        player_adaptation.sensitization_triggers.push(trigger.clone());
                    }
                }
                
                warn!("Player {} became more sensitive to dread (+{:.2} sensitivity)", 
                      adaptation_event.player_id, sensitization_amount);
            }

            AdaptationType::Breakthrough => {
                // Dread overcame player's adaptation defenses
                let breakthrough_impact = adaptation_event.adaptation_gained;
                
                // Reset adaptation for this dread level (breakthrough burns out adaptation)
                let dread_index = adaptation_event.dread_level_exposed as usize;
                if dread_index < 5 {
                    player_adaptation.current_adaptation[dread_index] *= 0.5; // Significant reduction
                }
                
                // Add sensitization triggers
                for trigger in &adaptation_event.sensitization_triggers {
                    if !player_adaptation.sensitization_triggers.contains(trigger) {
                        player_adaptation.sensitization_triggers.push(trigger.clone());
                    }
                }
                
                error!("Player {} experienced dread breakthrough at level {} (impact: {:.2})", 
                       adaptation_event.player_id, adaptation_event.dread_level_exposed, breakthrough_impact);
            }

            AdaptationType::Recovery => {
                // Player is recovering from dread exposure
                let recovery_amount = adaptation_event.adaptation_gained;
                
                // Restore some adaptation but not all (trauma leaves lasting effects)
                let dread_index = adaptation_event.dread_level_exposed as usize;
                if dread_index < 5 {
                    player_adaptation.current_adaptation[dread_index] += recovery_amount * 0.7;
                    player_adaptation.current_adaptation[dread_index] = 
                        player_adaptation.current_adaptation[dread_index].min(0.9); // Never fully recovers
                }
                
                info!("Player {} recovering from dread level {} (+{:.2} adaptation)", 
                      adaptation_event.player_id, adaptation_event.dread_level_exposed, recovery_amount);
            }

            _ => {} // Handle other adaptation types
        }
    }
}

/// System for dread aura management and interactions
pub fn dread_aura_system(
    mut aura_manager: ResMut<DreadAuraManager>,
    mut aura_query: Query<(Entity, &mut DreadAura), Changed<DreadAura>>,
    mut aura_interaction_events: EventWriter<DreadAuraInteractionEvent>,
    time: Res<Time>,
) {
    // Update aura manager with changed auras
    for (entity, aura) in aura_query.iter() {
        aura_manager.add_aura(entity, aura.clone());
    }

    // Process aura pulsing
    for (entity, mut aura) in aura_query.iter_mut() {
        if let Some(pulse) = &mut aura.pulsing {
            pulse.current_phase += time.delta_seconds() / pulse.pulse_period;
            if pulse.current_phase >= 1.0 {
                pulse.current_phase -= 1.0; // Reset cycle
            }

            // Update aura intensity based on pulse
            let pulse_modifier = match pulse.pulse_shape.as_str() {
                "sine" => (pulse.current_phase * 2.0 * std::f32::consts::PI).sin(),
                "square" => if pulse.current_phase < 0.5 { 1.0 } else { -1.0 },
                "sawtooth" => 2.0 * pulse.current_phase - 1.0,
                "random" => fastrand::f32() * 2.0 - 1.0,
                _ => 0.0,
            };

            aura.current_intensity = aura.base_intensity * (1.0 + pulse_modifier * pulse.pulse_amplitude);
            aura.current_intensity = aura.current_intensity.max(0.0);
        }
    }

    // Check for aura interactions
    let aura_entities: Vec<Entity> = aura_manager.active_auras.keys().cloned().collect();
    for i in 0..aura_entities.len() {
        for j in (i + 1)..aura_entities.len() {
            let entity1 = aura_entities[i];
            let entity2 = aura_entities[j];
            
            if let (Some(aura1), Some(aura2)) = (
                aura_manager.active_auras.get(&entity1),
                aura_manager.active_auras.get(&entity2)
            ) {
                // Check if auras are close enough to interact
                let interaction_radius = (aura1.effective_radius + aura2.effective_radius) * 0.5;
                // Distance calculation would be done here
                let distance = 100.0; // Placeholder
                
                if distance <= interaction_radius {
                    let interaction_type = determine_aura_interaction_type(aura1, aura2);
                    let interaction_strength = calculate_aura_interaction_strength(aura1, aura2, distance);
                    
                    if interaction_strength > 0.1 {
                        aura_interaction_events.send(DreadAuraInteractionEvent {
                            primary_aura_entity: entity1,
                            secondary_aura_entity: entity2,
                            interaction_type: interaction_type.clone(),
                            interaction_strength,
                            resulting_dread_change: calculate_dread_change_from_interaction(&interaction_type, interaction_strength),
                            affected_radius: interaction_radius,
                            duration: 60.0, // Default interaction duration
                            stability: 0.7,
                        });
                    }
                }
            }
        }
    }
}

/// System for applying parameter modifications to target systems
pub fn parameter_modification_application_system(
    mut modification_events: EventReader<DreadParameterModificationEvent>,
    mut dread_state: ResMut<DreadProgressionState>,
) {
    for modification_event in modification_events.read() {
        // Apply parameter modifications to the target system
        let system_name = &modification_event.system_name;
        
        // This is where the actual system parameter modification would occur
        // Each system would need to listen for its own modifications
        match system_name.as_str() {
            "combat" => {
                // Combat system would read these events and apply modifications
                debug!("Applying {} parameter modifications to combat system", 
                       modification_event.parameter_modifications.len());
            }
            "hex_rendering" => {
                // Hex rendering system would read these events
                debug!("Applying {} parameter modifications to hex rendering system", 
                       modification_event.parameter_modifications.len());
            }
            "dialogue" => {
                // Dialogue system would read these events
                debug!("Applying {} parameter modifications to dialogue system", 
                       modification_event.parameter_modifications.len());
            }
            "companion_psychology" => {
                // Companion psychology system would read these events
                debug!("Applying {} parameter modifications to companion psychology system", 
                       modification_event.parameter_modifications.len());
            }
            _ => {
                warn!("Unknown system for parameter modification: {}", system_name);
            }
        }
        
        info!("Applied dread modifications to {} system (priority: {})", 
              system_name, modification_event.priority);
    }
}

/// System for processing feature toggles based on dread level
pub fn feature_toggle_application_system(
    mut feature_toggle_events: EventReader<DreadFeatureToggleEvent>,
) {
    for toggle_event in feature_toggle_events.read() {
        // Apply feature toggles to the target system
        let system_name = &toggle_event.system_name;
        
        for feature_toggle in &toggle_event.feature_toggles {
            // Target systems would read these events and toggle features accordingly
            match system_name.as_str() {
                "combat" => {
                    debug!("Combat system: {} -> {}", 
                           feature_toggle.feature_name, feature_toggle.new_state);
                }
                "hex_rendering" => {
                    debug!("Hex rendering: {} -> {}", 
                           feature_toggle.feature_name, feature_toggle.new_state);
                }
                "dialogue" => {
                    debug!("Dialogue system: {} -> {}", 
                           feature_toggle.feature_name, feature_toggle.new_state);
                }
                _ => {}
            }
        }
        
        info!("Applied {} feature toggles to {} system", 
              toggle_event.feature_toggles.len(), system_name);
    }
}

/// System for processing system corruption at high dread levels
pub fn system_corruption_management_system(
    mut corruption_events: EventReader<SystemCorruptionEvent>,
    mut emergency_events: EventWriter<DreadEmergencyEvent>,
    dread_state: Res<DreadProgressionState>,
) {
    for corruption_event in corruption_events.read() {
        warn!("System corruption detected: {} - Type: {} - Level: {:.2}", 
              corruption_event.system_name, corruption_event.corruption_type, corruption_event.corruption_level);

        // Handle severe corruption (0.8+) as emergency
        if corruption_event.corruption_level >= 0.8 {
            emergency_events.send(DreadEmergencyEvent {
                emergency_type: "severe_system_corruption".to_string(),
                trigger_conditions: vec![
                    format!("corruption_level >= 0.8 in {}", corruption_event.system_name),
                ],
                emergency_actions: vec![
                    EmergencyAction {
                        action_type: "system_isolation".to_string(),
                        action_target: corruption_event.system_name.clone(),
                        action_parameters: {
                            let mut params = HashMap::new();
                            params.insert("isolation_level".to_string(), 0.8);
                            params
                        },
                        action_priority: 9,
                        action_duration: Some(300.0), // 5 minute isolation
                        success_criteria: vec!["corruption_contained".to_string()],
                    }
                ],
                affected_systems: corruption_event.cascading_effects.clone(),
                player_safety_measures: vec!["disable_corrupted_features".to_string()],
                recovery_plan: corruption_event.recovery_requirements.clone(),
            });
        }

        // Log corruption for analysis
        info!("System {} corrupted by {} (level: {:.2}, recoverable: {})", 
              corruption_event.system_name, corruption_event.corruption_source, 
              corruption_event.corruption_level, corruption_event.recovery_possible);
    }
}

/// System for emergency dread intervention
pub fn dread_emergency_system(
    mut emergency_events: EventReader<DreadEmergencyEvent>,
    mut parameter_modification_events: EventWriter<DreadParameterModificationEvent>,
    mut feature_toggle_events: EventWriter<DreadFeatureToggleEvent>,
    mut dread_state: ResMut<DreadProgressionState>,
) {
    for emergency in emergency_events.read() {
        error!("Dread emergency activated: {} - Affected systems: {:?}", 
               emergency.emergency_type, emergency.affected_systems);

        // Execute emergency actions
        for action in &emergency.emergency_actions {
            match action.action_type.as_str() {
                "system_isolation" => {
                    // Isolate system to prevent corruption spread
                    let isolation_level = action.action_parameters.get("isolation_level").copied().unwrap_or(1.0);
                    
                    parameter_modification_events.send(DreadParameterModificationEvent {
                        system_name: action.action_target.clone(),
                        parameter_modifications: vec![
                            ParameterModification {
                                parameter_name: "system_isolation".to_string(),
                                modification_type: "replace".to_string(),
                                modification_value: isolation_level,
                                original_value: Some(0.0),
                                transition_speed: 10.0, // Fast emergency application
                            }
                        ],
                        modification_reason: format!("Emergency: {}", emergency.emergency_type),
                        priority: 10, // Maximum priority for emergencies
                        duration: action.action_duration,
                        rollback_triggers: vec!["emergency_resolved".to_string()],
                    });
                }
                "feature_emergency_disable" => {
                    // Emergency disable of features
                    feature_toggle_events.send(DreadFeatureToggleEvent {
                        system_name: action.action_target.clone(),
                        feature_toggles: vec![
                            FeatureToggle {
                                feature_name: "all_features".to_string(),
                                new_state: false,
                                toggle_conditions: vec![format!("emergency: {}", emergency.emergency_type)],
                                alternative_features: vec!["safe_mode".to_string()],
                                restoration_requirements: emergency.recovery_plan.clone(),
                            }
                        ],
                        toggle_reason: format!("Emergency: {}", emergency.emergency_type),
                        affected_entities: vec![],
                        player_notification: true, // Always notify on emergency
                        companion_awareness: HashMap::new(),
                    });
                }
                _ => {}
            }

            info!("Executed emergency action: {} on {}", action.action_type, action.action_target);
        }
    }
}

/// System for milestone tracking and achievement
pub fn dread_milestone_system(
    milestone_query: Query<&DreadMilestone>,
    mut milestone_events: EventReader<DreadMilestoneEvent>,
    mut dread_state: ResMut<DreadProgressionState>,
    dread_config: Res<DreadProgressionConfig>,
) {
    // Check for milestone achievements based on current dread level
    for milestone in milestone_query.iter() {
        if !milestone.achieved && 
           dread_state.global_dread_level >= milestone.dread_level_required &&
           !dread_state.is_milestone_achieved(&milestone.milestone_id) {
            
            // Check additional unlock conditions
            let mut conditions_met = true;
            for condition in &milestone.unlock_conditions {
                // This would check actual game state conditions
                // For now, assume conditions are met
            }
            
            if conditions_met {
                dread_state.achieve_milestone(milestone.milestone_id.clone());
                
                info!("Dread milestone achieved: {} at dread level {}", 
                      milestone.milestone_id, dread_state.global_dread_level);
                
                // This would trigger milestone effects
                // Implementation depends on specific milestone systems
            }
        }
    }

    // Process milestone achievement events
    for milestone_event in milestone_events.read() {
        // Apply milestone effects
        for effect in &milestone_event.milestone.milestone_effects {
            match effect.effect_type.as_str() {
                "system_unlock" => {
                    info!("Unlocking system: {} due to milestone {}", 
                          effect.effect_target, milestone_event.milestone.milestone_id);
                }
                "narrative_branch" => {
                    info!("Narrative branch unlocked: {} due to milestone {}", 
                          effect.effect_target, milestone_event.milestone.milestone_id);
                }
                "companion_event" => {
                    info!("Companion event triggered: {} due to milestone {}", 
                          effect.effect_target, milestone_event.milestone.milestone_id);
                }
                _ => {}
            }
        }

        // Record world state changes
        for world_change in &milestone_event.world_state_changes {
            info!("World state change: {} - {} -> {}", 
                  world_change.change_type, world_change.target, world_change.change_value);
        }
    }
}

/// System for dread contagion spread
pub fn dread_contagion_system(
    mut contagion_events: EventReader<DreadContagionEvent>,
    mut dread_source_events: EventWriter<DreadSourceEvent>,
    contagion_query: Query<&DreadContagion>,
    time: Res<Time>,
) {
    for contagion_event in contagion_events.read() {
        debug!("Dread contagion spreading from {:?} to {} entities", 
               contagion_event.source_entity, contagion_event.affected_entities.len());

        // Create dread sources for each affected entity
        for (index, affected_entity) in contagion_event.affected_entities.iter().enumerate() {
            let contagion_source = DreadSource {
                source_id: format!("contagion_{}_{}", 
                                  contagion_event.contagion_type, index),
                source_type: "contagion".to_string(),
                intensity: contagion_event.contagion_strength * 0.8, // Slightly reduced
                decay_rate: 0.1, // Contagion fades over time
                radius: Some(30.0), // Small radius for contagion
                duration_remaining: 300.0, // 5 minute duration
                compounding_factor: 0.1, // Small compounding
                description: format!("Dread contagion spread via {}", 
                                    contagion_event.transmission_method),
            };

            dread_source_events.send(DreadSourceEvent {
                action: DreadSourceAction::Add,
                source: contagion_source,
                spatial_position: None, // Attached to entity
                affected_entities: vec![*affected_entity],
                propagation_rules: vec![
                    DreadPropagationRule {
                        propagation_type: "social".to_string(),
                        propagation_speed: 5.0,
                        propagation_decay: 0.4, // Significant decay for contagion
                        propagation_barriers: contagion_event.resistance_factors.clone(),
                        propagation_amplifiers: contagion_event.amplification_factors.clone(),
                    }
                ],
            });
        }

        warn!("Dread contagion {} spread to {} entities", 
              contagion_event.contagion_type, contagion_event.affected_entities.len());
    }
}

// Helper functions for aura interactions

fn determine_aura_interaction_type(aura1: &DreadAura, aura2: &DreadAura) -> String {
    match (aura1.aura_type.as_str(), aura2.aura_type.as_str()) {
        ("dragon", "corruption") => "amplification".to_string(),
        ("dragon", "madness") => "resonance".to_string(),
        ("corruption", "void") => "amplification".to_string(),
        ("madness", "madness") => "resonance".to_string(),
        _ => {
            // Check resonance frequency
            if (aura1.resonance_frequency - aura2.resonance_frequency).abs() < 0.1 {
                "resonance".to_string()
            } else if aura1.resonance_frequency > aura2.resonance_frequency {
                "interference".to_string()
            } else {
                "neutral".to_string()
            }
        }
    }
}

fn calculate_aura_interaction_strength(aura1: &DreadAura, aura2: &DreadAura, distance: f32) -> f32 {
    let base_strength = (aura1.current_intensity + aura2.current_intensity) * 0.5;
    let distance_falloff = 1.0 - (distance / (aura1.effective_radius + aura2.effective_radius));
    let penetration_factor = (aura1.penetration + aura2.penetration) * 0.5;
    
    base_strength * distance_falloff * penetration_factor
}

fn calculate_dread_change_from_interaction(interaction_type: &str, strength: f32) -> f32 {
    match interaction_type {
        "amplification" => strength * 0.5,  // Positive dread change
        "resonance" => strength * 0.7,      // Strong positive change
        "interference" => -strength * 0.3,  // Negative dread change
        "cancellation" => -strength * 0.6,  // Strong negative change
        _ => 0.0,
    }
}
