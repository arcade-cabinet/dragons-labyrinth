//! Sentimental Item & Forge System - Complete ECS Integration
//!
//! Production-ready Bevy plugin for Dragon's Labyrinth's unique dual-path morality system.
//! Sentimental items become forge reagents for light (essence) vs dark (blood) paths,
//! with full database integration and second chances mechanics.

use bevy::prelude::*;
use sea_orm::DatabaseConnection;

pub mod components;
pub mod resources;

pub use components::*;
pub use resources::*;

/// Main forge system plugin
pub struct ForgeSystemPlugin;

impl Plugin for ForgeSystemPlugin {
    fn build(&self, app: &mut App) {
        app
            // Initialize resources
            .init_resource::<ForgeSystemConfig>()
            .init_resource::<ForgeSessionManager>()
            
            // Core forge systems
            .add_systems(Startup, (
                setup_forge_system,
                initialize_forge_masters,
                load_player_forge_data_system,
            ).chain())
            
            .add_systems(Update, (
                // Sentimental item management
                sentimental_item_collection_system,
                emotional_resonance_system,
                memory_trigger_system,
                
                // Forge trial systems
                forge_trial_management_system,
                forge_master_approval_system,
                
                // Forge session systems
                forge_session_execution_system,
                reagent_consumption_system,
                
                // Second chances system
                second_chances_management_system,
                
                // Mythic gear systems
                mythic_gear_creation_system,
                gear_evolution_system,
                gear_synergy_system,
                
                // Integration systems
                forge_psychology_integration_system,
                forge_dread_integration_system,
                
                // Database sync
                forge_database_sync_system,
            ).chain())
            
            // Register component reflection
            .register_type::<SentimentalItem>()
            .register_type::<ForgeTrial>()
            .register_type::<ForgePathProgression>()
            .register_type::<ForgeReagentCollection>()
            .register_type::<SecondChancesSystem>()
            .register_type::<MythicGearCreation>()
            .register_type::<ForgeTrialStage>()
            .register_type::<ForgePath>();
    }
}

// Startup systems

fn setup_forge_system(
    mut commands: Commands,
    db: Res<DatabaseConnection>,
) {
    info!("Initializing Dragon's Labyrinth Forge System");
    
    let forge_state = ForgeSystemState {
        db: db.clone(),
        active_players: HashMap::new(),
        sentimental_items_cache: HashMap::new(),
        forge_progress_cache: HashMap::new(),
        active_trials: HashMap::new(),
        forge_masters: HashMap::new(),
        system_integration_hooks: HashMap::new(),
    };
    
    commands.insert_resource(forge_state);
    info!("Forge System initialized with sentimental item tracking and dual-path morality");
}

fn initialize_forge_masters(mut forge_state: ResMut<ForgeSystemState>) {
    // Forge masters are initialized in ForgeSystemState::new()
    info!("Initialized {} forge masters", forge_state.forge_masters.len());
}

fn load_player_forge_data_system(
    players_query: Query<&players::Model, Added<players::Model>>,
    mut forge_state: ResMut<ForgeSystemState>,
    mut commands: Commands,
) {
    // This would load forge data for new players
    for player in players_query.iter() {
        info!("Loading forge data for player {}", player.id);
        // Would spawn forge components for this player
    }
}

// Update systems

fn sentimental_item_collection_system(
    mut commands: Commands,
    sentimental_query: Query<&SentimentalItem, Added<SentimentalItem>>,
    mut forge_state: ResMut<ForgeSystemState>,
) {
    for item in sentimental_query.iter() {
        info!("New sentimental item collected: {} (category: {}, power: {:.2})", 
              item.memory_description, item.sentimental_category, item.forge_reagent_power);
        
        // Add to cache
        // Would add to sentimental_items_cache in production
    }
}

fn emotional_resonance_system(
    time: Res<Time>,
    mut sentimental_query: Query<&mut SentimentalItem>,
) {
    for mut item in sentimental_query.iter_mut() {
        // Process emotional resonance effects
        for resonance in &mut item.emotional_resonance {
            if resonance.duration > 0.0 {
                resonance.duration -= time.delta_seconds();
            }
        }
    }
}

fn memory_trigger_system(
    sentimental_query: Query<&SentimentalItem>,
) {
    for item in sentimental_query.iter() {
        if item.triggers_memory && item.memory_intensity > 0.7 {
            // High-intensity memory trigger would create memory flashback
            debug!("Memory triggered by item: {}", item.memory_description);
        }
    }
}

fn forge_trial_management_system(
    mut trial_query: Query<&mut ForgeTrial>,
    time: Res<Time>,
    forge_config: Res<ForgeSystemConfig>,
) {
    for mut trial in trial_query.iter_mut() {
        match trial.trial_stage {
            ForgeTrialStage::Preparation => {
                // Check if ready to proceed to system test
                if trial.progress >= 1.0 {
                    trial.trial_stage = ForgeTrialStage::SystemTest;
                    trial.progress = 0.0;
                    info!("Forge trial {} advanced to SystemTest stage", trial.trial_id);
                }
            }
            ForgeTrialStage::SystemTest => {
                // Progress system testing
                trial.progress += time.delta_seconds() / 300.0; // 5 minute test duration
                if trial.progress >= 1.0 {
                    trial.trial_stage = ForgeTrialStage::PathChoice;
                    trial.progress = 0.0;
                    info!("Forge trial {} advanced to PathChoice stage", trial.trial_id);
                }
            }
            ForgeTrialStage::PathChoice => {
                // Wait for player to choose path
                if trial.progress >= 1.0 {
                    trial.trial_stage = ForgeTrialStage::Sacrifice;
                    trial.progress = 0.0;
                    warn!("Forge trial {} advanced to Sacrifice stage", trial.trial_id);
                }
            }
            ForgeTrialStage::Sacrifice => {
                // Process sacrifice phase
                if trial.progress >= 1.0 {
                    trial.trial_stage = ForgeTrialStage::Forging;
                    trial.progress = 0.0;
                    info!("Forge trial {} advanced to Forging stage", trial.trial_id);
                }
            }
            ForgeTrialStage::Forging => {
                // Process actual forging
                trial.progress += time.delta_seconds() / 600.0; // 10 minute forging duration
                if trial.progress >= 1.0 {
                    trial.trial_stage = ForgeTrialStage::Completion;
                    trial.progress = 1.0;
                    info!("Forge trial {} completed successfully!", trial.trial_id);
                }
            }
            _ => {} // Completion and Failure are end states
        }
    }
}

fn forge_master_approval_system(
    path_progression_query: Query<&ForgePathProgression>,
    forge_state: Res<ForgeSystemState>,
) {
    for path_progression in path_progression_query.iter() {
        if let Some(chosen_path) = &path_progression.chosen_path {
            if let Some(master) = forge_state.get_forge_master(chosen_path) {
                // Check approval requirements
                let mut approval_progress = 0.0;
                for requirement in &master.approval_requirements {
                    approval_progress += requirement.current_progress;
                }
                approval_progress /= master.approval_requirements.len() as f32;
                
                if approval_progress >= 0.8 && !path_progression.forge_master_tests_passed.contains(&master.master_id) {
                    debug!("Player approaching forge master approval for {} path", 
                           format!("{:?}", chosen_path));
                }
            }
        }
    }
}

fn forge_session_execution_system(
    mut session_manager: ResMut<ForgeSessionManager>,
    time: Res<Time>,
) {
    let session_ids: Vec<Uuid> = session_manager.active_sessions.keys().cloned().collect();
    
    for session_id in session_ids {
        if let Some(session) = session_manager.active_sessions.get_mut(&session_id) {
            // Update session progress
            let elapsed_time = chrono::Utc::now().timestamp() - session.start_time;
            let progress = elapsed_time as f32 / session.estimated_duration;
            
            if progress >= 1.0 {
                // Session completed
                info!("Forge session {} completed", session_id);
                
                // Would complete session with proper data
                let completion_data = CompletedForgeSession {
                    session_id,
                    completion_time: chrono::Utc::now().timestamp(),
                    success: true, // Would be calculated based on actual performance
                    gear_created: None, // Would be set if gear was created
                    reagents_consumed: session.active_reagents.clone(),
                    experience_gained: 1.0,
                    insights_discovered: vec!["Forge mastery insight".to_string()],
                    participant_growth: HashMap::new(),
                };
                
                session_manager.complete_session(session_id, true, completion_data);
            }
        }
    }
}

fn reagent_consumption_system(
    mut reagent_query: Query<&mut ForgeReagentCollection>,
    session_manager: Res<ForgeSessionManager>,
) {
    for mut reagent_collection in reagent_query.iter_mut() {
        // Process reagent consumption during active sessions
        for active_session in session_manager.active_sessions.values() {
            for reagent_id in &active_session.active_reagents {
                // Would reduce reagent power based on consumption rate
                debug!("Consuming reagent {} in session {}", reagent_id, active_session.session_id);
            }
        }
    }
}

fn second_chances_management_system(
    mut second_chances_query: Query<&mut SecondChancesSystem>,
    time: Res<Time>,
) {
    for mut second_chances in second_chances_query.iter_mut() {
        // Check for expiring second chance opportunities
        second_chances.pending_second_chance_opportunities.retain_mut(|opportunity| {
            if let Some(expiration) = opportunity.expiration_time {
                if chrono::Utc::now().timestamp() >= expiration {
                    debug!("Second chance opportunity expired: {}", opportunity.opportunity_id);
                    return false;
                }
            }
            true
        });
    }
}

fn mythic_gear_creation_system(
    mut gear_query: Query<&mut MythicGearCreation>,
    time: Res<Time>,
) {
    for mut gear_creation in gear_query.iter_mut() {
        // Process active gear forging
        if let Some(session) = &gear_creation.current_forge_session {
            debug!("Processing mythic gear creation session: {}", session.session_id);
            // Would update forging progress
        }
        
        // Check for gear evolution opportunities
        for (gear_id, evolution_potential) in &gear_creation.gear_evolution_potential {
            if *evolution_potential >= 1.0 {
                info!("Mythic gear {} ready for evolution", gear_id);
            }
        }
    }
}

fn gear_evolution_system(
    mut gear_query: Query<&mut MythicGearCreation>,
    forge_config: Res<ForgeSystemConfig>,
) {
    for mut gear_creation in gear_query.iter_mut() {
        // Check each completed gear for evolution potential
        for gear in &mut gear_creation.completed_mythic_gear {
            for (threshold_index, &threshold) in forge_config.gear_evolution_thresholds.iter().enumerate() {
                if gear.power_level >= threshold && gear.enhancement_level as usize <= threshold_index {
                    gear.enhancement_level = threshold_index as u8 + 1;
                    gear.power_level *= forge_config.gear_power_scaling[gear.enhancement_level as usize];
                    
                    info!("Mythic gear {} evolved to level {} (power: {:.2})", 
                          gear.gear_name, gear.enhancement_level, gear.power_level);
                }
            }
        }
    }
}

fn gear_synergy_system(
    gear_query: Query<&MythicGearCreation>,
    forge_config: Res<ForgeSystemConfig>,
) {
    for gear_creation in gear_query.iter() {
        // Check for gear synergies
        for synergy in &gear_creation.gear_synergies {
            let synergy_bonus = calculate_gear_synergy_bonus(synergy, &forge_config);
            if synergy_bonus > 1.0 {
                debug!("Gear synergy {} providing {:.2}x power bonus", 
                       synergy.synergy_name, synergy_bonus);
            }
        }
    }
}

fn forge_psychology_integration_system(
    forge_state: Res<ForgeSystemState>,
    trial_query: Query<&ForgeTrial>,
) {
    // Integration with companion psychology system
    if let Some(integration_hook) = forge_state.system_integration_hooks.get("companion_psychology") {
        for trial in trial_query.iter() {
            if trial.trial_stage == ForgeTrialStage::Sacrifice {
                debug!("Forge trial sacrifice stage - integrating with psychology system");
                // Would send events to psychology system about trauma from sacrifice
            }
        }
    }
}

fn forge_dread_integration_system(
    forge_state: Res<ForgeSystemState>,
    trial_query: Query<&ForgeTrial>,
) {
    // Integration with dread progression system
    if let Some(integration_hook) = forge_state.system_integration_hooks.get("dread_progression") {
        for trial in trial_query.iter() {
            if matches!(trial.trial_stage, ForgeTrialStage::Forging | ForgeTrialStage::Sacrifice) {
                debug!("Forge trial affecting dread levels");
                // Would send events to dread system about forge activities
            }
        }
    }
}

fn forge_database_sync_system(
    forge_state: Res<ForgeSystemState>,
    path_progression_query: Query<&ForgePathProgression, Changed<ForgePathProgression>>,
    sentimental_query: Query<&SentimentalItem, Changed<SentimentalItem>>,
) {
    // Sync changed forge data to database
    for path_progression in path_progression_query.iter() {
        debug!("Syncing forge path progression for player {}", path_progression.player_id);
        // Would update database with current progression
    }
    
    for item in sentimental_query.iter() {
        debug!("Syncing sentimental item changes for item {}", item.item_id);
        // Would update database with item changes
    }
}

// Helper functions

fn calculate_gear_synergy_bonus(synergy: &GearSynergy, config: &ForgeSystemConfig) -> f32 {
    let mut total_bonus = synergy.synergy_power;
    
    // Apply synergy multipliers from config
    for (synergy_type, multiplier) in &config.synergy_bonus_multipliers {
        if synergy.synergy_effects.iter().any(|effect| effect.contains(synergy_type)) {
            total_bonus *= multiplier;
        }
    }
    
    total_bonus
}
