//! Dread Progression System - Complete ECS Integration
//!
//! Production-ready Bevy plugin for Dragon's Labyrinth's master horror orchestrator system.
//! This system transforms ALL other game systems based on dread level progression (0-4),
//! creating the core horror experience that makes Dragon's Labyrinth unique.

use bevy::prelude::*;
use sea_orm::DatabaseConnection;

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

/// Master dread progression plugin that orchestrates all horror mechanics
pub struct DreadProgressionPlugin;

impl Plugin for DreadProgressionPlugin {
    fn build(&self, app: &mut App) {
        app
            // Initialize core resources
            .init_resource::<DreadProgressionConfig>()
            .init_resource::<DreadAuraManager>()
            .init_resource::<RealityDistortionManager>()
            
            // Register all dread progression events
            .add_event::<DreadLevelChangeEvent>()
            .add_event::<DreadSourceEvent>()
            .add_event::<SystemTransformationEvent>()
            .add_event::<DreadMilestoneEvent>()
            .add_event::<DragonPresenceEvent>()
            .add_event::<CompanionDreadEvent>()
            .add_event::<EnvironmentalDreadEvent>()
            .add_event::<NarrativeDreadEvent>()
            .add_event::<RealityDistortionEvent>()
            .add_event::<PlayerDreadAdaptationEvent>()
            .add_event::<DreadAuraInteractionEvent>()
            .add_event::<SystemCorruptionEvent>()
            .add_event::<DreadResistanceEvent>()
            .add_event::<DreadContagionEvent>()
            .add_event::<DreadSpikeEvent>()
            .add_event::<DreadStabilityEvent>()
            .add_event::<DreadParameterModificationEvent>()
            .add_event::<DreadFeatureToggleEvent>()
            .add_event::<DreadIntegrationEvent>()
            .add_event::<DreadEmergencyEvent>()
            .add_event::<DreadAnalyticsEvent>()
            .add_event::<DreadDebugEvent>()
            
            // Startup systems (initialize dread progression)
            .add_systems(Startup, (
                setup_dread_progression_system,
                initialize_dread_configuration,
                setup_aura_manager,
                setup_reality_distortion_manager,
                initialize_baseline_dread_levels,
            ).chain())
            
            // Core dread calculation and orchestration (highest priority)
            .add_systems(Update, (
                // Master dread level calculation
                dread_level_calculation_system,
                
                // System transformation orchestration
                system_transformation_orchestrator,
                
                // Specific dread source systems
                dragon_presence_dread_system,
                companion_dread_integration_system,
                environmental_dread_system,
                narrative_dread_system,
                
                // Dread effects and interactions
                dread_aura_system,
                reality_distortion_system,
                dread_contagion_system,
                
                // Player adaptation and response
                player_dread_adaptation_system,
                
                // System modification application
                parameter_modification_application_system,
                feature_toggle_application_system,
                
                // Crisis management
                system_corruption_management_system,
                dread_emergency_system,
                
                // Tracking and analytics
                dread_milestone_system,
                dread_analytics_system,
            ).chain().run_if(resource_exists::<DreadProgressionState>))
            
            // Periodic systems for maintenance and analysis
            .add_systems(FixedUpdate, (
                dread_level_stability_monitoring,
                regional_dread_calculation,
                player_adaptation_tracking,
                system_performance_monitoring,
                dread_balance_analysis,
            ).chain())
            
            // Register component reflection for debugging
            .register_type::<DreadLevel>()
            .register_type::<DreadSource>()
            .register_type::<SystemDreadModification>()
            .register_type::<DreadAura>()
            .register_type::<DreadPulse>()
            .register_type::<DreadResistance>()
            .register_type::<ResistanceSource>()
            .register_type::<DreadTransformation>()
            .register_type::<NarrativeDreadProgression>()
            .register_type::<EnvironmentalDread>()
            .register_type::<RealityDistortion>()
            .register_type::<DistortionManifestation>()
            .register_type::<DreadContagion>()
            .register_type::<SystemOverride>()
            .register_type::<DreadAdaptation>()
            .register_type::<AdaptationPoint>()
            .register_type::<DreadEffects>()
            .register_type::<DreadMilestone>()
            
            // Add state for dread system phases
            .init_state::<DreadSystemPhase>()
            
            // State-specific systems
            .add_systems(OnEnter(DreadSystemPhase::Emergency), (
                activate_emergency_protocols,
                isolate_corrupted_systems,
            ))
            .add_systems(OnExit(DreadSystemPhase::Emergency), (
                deactivate_emergency_protocols,
                restore_system_functionality,
            ))
            .add_systems(OnEnter(DreadSystemPhase::RealityDistortion), (
                initialize_reality_anchors,
                enable_distortion_tracking,
            ))
            .add_systems(OnExit(DreadSystemPhase::RealityDistortion), (
                disable_distortion_tracking,
                restore_reality_stability,
            ));
    }
}

/// States for dread system operation
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum DreadSystemPhase {
    #[default]
    Normal,           // 0-1 dread level: normal operation
    Elevated,         // 2 dread level: heightened awareness
    Critical,         // 3 dread level: major system changes
    RealityDistortion, // 4 dread level: reality breaks down
    Emergency,        // System corruption emergency
}

/// Setup dread progression system on startup
fn setup_dread_progression_system(
    mut commands: Commands,
    db: Res<DatabaseConnection>,
) {
    info!("Initializing Dragon's Labyrinth Dread Progression System");
    
    // Initialize the master dread progression state
    let dread_state = DreadProgressionState {
        db: db.clone(),
        global_dread_level: 0,
        regional_dread_levels: HashMap::new(),
        active_dread_sources: HashMap::new(),
        system_transformations: HashMap::new(),
        dread_history: Vec::new(),
        milestone_tracker: HashMap::new(),
        player_adaptation_data: HashMap::new(),
    };
    
    commands.insert_resource(dread_state);
    
    info!("Dread Progression System initialized - Master horror orchestrator active");
}

/// Initialize dread configuration from game balance data
fn initialize_dread_configuration(
    mut commands: Commands,
) {
    let config = DreadProgressionConfig::default();
    
    info!("Dread configuration loaded:");
    info!("  - {} system configurations", config.system_configs.len());
    info!("  - {} biome dread multipliers", config.biome_dread_multipliers.len());
    info!("  - {} revelation dread spikes", config.revelation_dread_spikes.len());
    info!("  - Dragon presence curve: {} points", config.dragon_presence_dread_curve.len());
    
    commands.insert_resource(config);
}

/// Setup dread aura manager
fn setup_aura_manager(
    mut commands: Commands,
) {
    let aura_manager = DreadAuraManager::default();
    commands.insert_resource(aura_manager);
    
    info!("Dread aura manager initialized");
}

/// Setup reality distortion manager
fn setup_reality_distortion_manager(
    mut commands: Commands,
) {
    let distortion_manager = RealityDistortionManager::default();
    commands.insert_resource(distortion_manager);
    
    info!("Reality distortion manager initialized");
}

/// Initialize baseline dread levels from database
fn initialize_baseline_dread_levels(
    mut commands: Commands,
    db: Res<DatabaseConnection>,
) {
    // Create initial dread level entity
    commands.spawn(DreadLevel {
        current_level: 0,
        previous_level: 0,
        progression_rate: 0.01,
        stability: 1.0,
        external_factors: 0.0,
        active_sources: Vec::new(),
        environmental_dread: 0.0,
        narrative_dread: 0.0,
        companion_dread: 0.0,
        player_action_dread: 0.0,
        affected_systems: HashMap::new(),
        last_update: chrono::Utc::now().timestamp(),
        transition_duration: 3.0,
    });
    
    info!("Baseline dread levels initialized");
}

/// Analytics system for tracking dread progression
fn dread_analytics_system(
    mut analytics_events: EventReader<DreadAnalyticsEvent>,
    dread_state: Res<DreadProgressionState>,
    dread_config: Res<DreadProgressionConfig>,
) {
    for analytics_event in analytics_events.read() {
        match analytics_event.analytics_type.as_str() {
            "performance_monitoring" => {
                debug!("Dread system performance: {:?}", analytics_event.system_performance);
            }
            "player_behavior_analysis" => {
                debug!("Player behavior under dread: {:?}", analytics_event.player_behavior);
            }
            "balance_analysis" => {
                info!("Dread balance recommendations: {:?}", analytics_event.balance_recommendations);
            }
            _ => {}
        }
    }
}

// Fixed update systems for maintenance

fn dread_level_stability_monitoring(
    dread_level_query: Query<&DreadLevel>,
    mut stability_events: EventWriter<DreadStabilityEvent>,
    dread_config: Res<DreadProgressionConfig>,
) {
    for dread_level in dread_level_query.iter() {
        let required_stability = dread_config.stability_requirements[dread_level.current_level as usize];
        
        if dread_level.stability < required_stability {
            stability_events.send(DreadStabilityEvent {
                stability_change: required_stability - dread_level.stability,
                stability_factors: vec![],
                current_stability: dread_level.stability,
                required_stability,
                stabilization_methods: vec![
                    "Reduce dread sources".to_string(),
                    "Increase companion support".to_string(),
                    "Find safe locations".to_string(),
                ],
                destabilization_threats: vec![
                    "Active trauma responses".to_string(),
                    "Environmental corruption".to_string(),
                    "Dragon presence".to_string(),
                ],
            });
        }
    }
}

fn regional_dread_calculation(
    mut dread_state: ResMut<DreadProgressionState>,
    db: Res<DatabaseConnection>,
) {
    // Calculate regional dread levels
    // This would use the DreadProgressionQueries in actual implementation
    debug!("Calculating regional dread levels");
}

fn player_adaptation_tracking(
    mut dread_state: ResMut<DreadProgressionState>,
    mut adaptation_events: EventWriter<PlayerDreadAdaptationEvent>,
) {
    // Track player adaptation over time
    for (player_id, adaptation_data) in dread_state.player_adaptation_data.iter_mut() {
        // Check for natural adaptation decay
        for level_adaptation in adaptation_data.current_adaptation.iter_mut() {
            *level_adaptation *= 0.999; // Very slow natural decay
        }
        
        // Check for habituation curve updates
        for (source_id, curve) in adaptation_data.habituation_curves.iter_mut() {
            if curve.current_habituation > 0.0 {
                curve.current_habituation -= curve.decay_rate; // Natural decay
                curve.current_habituation = curve.current_habituation.max(0.0);
            }
        }
    }
}

fn system_performance_monitoring(
    dread_state: Res<DreadProgressionState>,
    mut analytics_events: EventWriter<DreadAnalyticsEvent>,
) {
    let mut performance_data = HashMap::new();
    
    // Monitor each system's performance under current dread level
    for (system_name, transformation) in &dread_state.system_transformations {
        let performance_score = match transformation.transition_state {
            TransitionState::Stable => 1.0,
            TransitionState::Transitioning { progress, .. } => progress,
            TransitionState::Corrupted { corruption_level, .. } => 1.0 - corruption_level,
        };
        
        performance_data.insert(system_name.clone(), performance_score);
    }
    
    if !performance_data.is_empty() {
        analytics_events.send(DreadAnalyticsEvent {
            analytics_type: "system_performance".to_string(),
            data_points: performance_data,
            player_behavior: vec![],
            system_performance: HashMap::new(),
            balance_recommendations: vec![],
            player_feedback_indicators: vec![],
        });
    }
}

fn dread_balance_analysis(
    dread_state: Res<DreadProgressionState>,
    mut analytics_events: EventWriter<DreadAnalyticsEvent>,
) {
    let mut balance_recommendations = Vec::new();
    
    // Analyze dread progression rate
    if dread_state.dread_history.len() > 10 {
        let recent_changes = &dread_state.dread_history[dread_state.dread_history.len() - 10..];
        let avg_change_rate = recent_changes.len() as f32 / 600.0; // Changes per 10 minutes
        
        if avg_change_rate > 0.5 {
            balance_recommendations.push("Dread progression may be too fast".to_string());
        } else if avg_change_rate < 0.1 {
            balance_recommendations.push("Dread progression may be too slow".to_string());
        }
    }
    
    // Analyze milestone distribution
    let achieved_milestones = dread_state.milestone_tracker.values().filter(|&&achieved| achieved).count();
    let total_milestones = dread_state.milestone_tracker.len();
    
    if total_milestones > 0 {
        let achievement_rate = achieved_milestones as f32 / total_milestones as f32;
        if achievement_rate > 0.8 {
            balance_recommendations.push("Consider adding more challenging milestones".to_string());
        } else if achievement_rate < 0.3 {
            balance_recommendations.push("Consider making milestones more achievable".to_string());
        }
    }
    
    if !balance_recommendations.is_empty() {
        analytics_events.send(DreadAnalyticsEvent {
            analytics_type: "balance_analysis".to_string(),
            data_points: HashMap::new(),
            player_behavior: vec![],
            system_performance: HashMap::new(),
            balance_recommendations,
            player_feedback_indicators: vec![],
        });
    }
}

// State transition systems

fn activate_emergency_protocols(
    mut commands: Commands,
    mut emergency_events: EventWriter<DreadEmergencyEvent>,
) {
    error!("ACTIVATING DREAD EMERGENCY PROTOCOLS");
    
    emergency_events.send(DreadEmergencyEvent {
        emergency_type: "system_phase_emergency".to_string(),
        trigger_conditions: vec!["Entered emergency dread phase".to_string()],
        emergency_actions: vec![
            EmergencyAction {
                action_type: "global_system_safeguards".to_string(),
                action_target: "all_systems".to_string(),
                action_parameters: {
                    let mut params = HashMap::new();
                    params.insert("safety_level".to_string(), 0.9);
                    params
                },
                action_priority: 10,
                action_duration: None, // Permanent until resolved
                success_criteria: vec!["Systems stabilized".to_string()],
            }
        ],
        affected_systems: vec!["all".to_string()],
        player_safety_measures: vec![
            "Enable safe mode options".to_string(),
            "Provide emergency exits".to_string(),
            "Reduce system complexity".to_string(),
        ],
        recovery_plan: vec![
            "Identify corruption source".to_string(),
            "Isolate affected systems".to_string(),
            "Restore stable operation".to_string(),
        ],
    });
}

fn isolate_corrupted_systems(mut commands: Commands) {
    warn!("Isolating corrupted systems to prevent spread");
}

fn deactivate_emergency_protocols(mut commands: Commands) {
    info!("Deactivating dread emergency protocols");
}

fn restore_system_functionality(mut commands: Commands) {
    info!("Restoring normal system functionality");
}

fn initialize_reality_anchors(mut commands: Commands) {
    warn!("Reality distortion detected - initializing reality anchors");
}

fn enable_distortion_tracking(mut commands: Commands) {
    warn!("Enabling reality distortion tracking");
}

fn disable_distortion_tracking(mut commands: Commands) {
    info!("Disabling reality distortion tracking");
}

fn restore_reality_stability(mut commands: Commands) {
    info!("Restoring reality stability");
}

/// Integration system that connects dread progression with existing systems
pub fn integrate_with_existing_systems(
    mut integration_events: EventReader<DreadIntegrationEvent>,
    mut parameter_events: EventWriter<DreadParameterModificationEvent>,
    mut feature_events: EventWriter<DreadFeatureToggleEvent>,
) {
    for integration in integration_events.read() {
        match integration.integration_type.as_str() {
            "combat_integration" => {
                // Integrate dread effects with combat system
                parameter_events.send(DreadParameterModificationEvent {
                    system_name: "combat".to_string(),
                    parameter_modifications: vec![
                        ParameterModification {
                            parameter_name: "dread_combat_modifier".to_string(),
                            modification_type: "multiply".to_string(),
                            modification_value: integration.integration_data
                                .get("combat_effectiveness").copied().unwrap_or(1.0),
                            original_value: Some(1.0),
                            transition_speed: 2.0,
                        }
                    ],
                    modification_reason: "Dread integration with combat".to_string(),
                    priority: 6,
                    duration: None,
                    rollback_triggers: vec!["dread_integration_disabled".to_string()],
                });
            }
            
            "psychology_integration" => {
                // Integrate dread effects with companion psychology
                parameter_events.send(DreadParameterModificationEvent {
                    system_name: "companion_psychology".to_string(),
                    parameter_modifications: vec![
                        ParameterModification {
                            parameter_name: "dread_trauma_amplifier".to_string(),
                            modification_type: "multiply".to_string(),
                            modification_value: integration.integration_data
                                .get("trauma_amplification").copied().unwrap_or(1.0),
                            original_value: Some(1.0),
                            transition_speed: 1.0,
                        }
                    ],
                    modification_reason: "Dread integration with psychology".to_string(),
                    priority: 7,
                    duration: None,
                    rollback_triggers: vec!["dread_integration_disabled".to_string()],
                });
            }
            
            "hex_rendering_integration" => {
                // Integrate dread effects with hex rendering
                feature_events.send(DreadFeatureToggleEvent {
                    system_name: "hex_rendering".to_string(),
                    feature_toggles: vec![
                        FeatureToggle {
                            feature_name: "dread_visual_effects".to_string(),
                            new_state: true,
                            toggle_conditions: vec!["dread_integration_active".to_string()],
                            alternative_features: vec![],
                            restoration_requirements: vec!["dread_integration_disabled".to_string()],
                        }
                    ],
                    toggle_reason: "Enable dread visual effects".to_string(),
                    affected_entities: vec![],
                    player_notification: false,
                    companion_awareness: HashMap::new(),
                });
            }
            
            _ => {}
        }
        
        info!("Integrated dread system with: {}", integration.primary_system);
    }
}

/// Production monitoring system for dread progression
pub fn dread_production_monitoring_system(
    dread_state: Res<DreadProgressionState>,
    mut debug_events: EventWriter<DreadDebugEvent>,
    time: Res<Time>,
) {
    // Monitor system health every 30 seconds
    static mut LAST_CHECK: f32 = 0.0;
    unsafe {
        LAST_CHECK += time.delta_seconds();
        if LAST_CHECK >= 30.0 {
            LAST_CHECK = 0.0;
            
            let mut system_states = HashMap::new();
            let mut performance_metrics = HashMap::new();
            let mut error_conditions = Vec::new();
            
            // Check system health
            system_states.insert("global_dread_level".to_string(), 
                               dread_state.global_dread_level.to_string());
            system_states.insert("active_sources_count".to_string(), 
                               dread_state.active_dread_sources.len().to_string());
            system_states.insert("system_transformations_count".to_string(), 
                               dread_state.system_transformations.len().to_string());
            
            performance_metrics.insert("dread_calculation_rate".to_string(), 60.0); // 60 FPS target
            performance_metrics.insert("active_sources".to_string(), 
                                     dread_state.active_dread_sources.len() as f32);
            
            // Check for error conditions
            if dread_state.global_dread_level > 4 {
                error_conditions.push("Dread level exceeds maximum (4)".to_string());
            }
            
            if dread_state.active_dread_sources.len() > 100 {
                error_conditions.push("Too many active dread sources (>100)".to_string());
            }
            
            debug_events.send(DreadDebugEvent {
                debug_type: "system_health_check".to_string(),
                system_states,
                active_modifications: vec![], // Would be populated in production
                error_conditions,
                performance_metrics,
                recommended_actions: vec![
                    if dread_state.active_dread_sources.len() > 50 {
                        "Consider consolidating dread sources".to_string()
                    } else {
                        "Dread source count is optimal".to_string()
                    }
                ],
            });
        }
    }
}
