//! Dread Progression System - ECS Resources
//!
//! Production-ready resources for managing dread level orchestration and system transformation
//! across ALL Dragon's Labyrinth systems based on horror progression (dread levels 0-4).

use bevy::prelude::*;
use sea_orm::DatabaseConnection;
use database_orm::hex_tiles;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;
use crate::systems::dread_progression::components::*;

/// Master dread progression state resource
#[derive(Resource, Debug)]
pub struct DreadProgressionState {
    pub db: DatabaseConnection,
    pub global_dread_level: u8,   // Global dread level (0-4)
    pub regional_dread_levels: HashMap<String, u8>, // Per-region dread levels
    pub active_dread_sources: HashMap<String, DreadSource>, // All active dread sources
    pub system_transformations: HashMap<String, SystemTransformation>, // How systems are transformed
    pub dread_history: Vec<DreadLevelChange>, // History of dread changes
    pub milestone_tracker: HashMap<String, bool>, // Which milestones have been achieved
    pub player_adaptation_data: HashMap<Uuid, PlayerDreadAdaptation>, // Per-player adaptation
}

/// Record of a dread level change
#[derive(Debug, Clone)]
pub struct DreadLevelChange {
    pub timestamp: i64,
    pub old_level: u8,
    pub new_level: u8,
    pub trigger_source: String,   // What caused this change
    pub affected_systems: Vec<String>, // Which systems were affected
    pub player_reaction_time: f32, // How long player took to adapt
}

/// How a specific system is transformed by dread
#[derive(Debug, Clone)]
pub struct SystemTransformation {
    pub system_name: String,
    pub base_parameters: HashMap<String, f32>, // Original parameters
    pub dread_transforms: [SystemTransformLevel; 5], // Transform for each dread level
    pub currently_applied: SystemTransformLevel, // Currently applied transformation
    pub transition_state: TransitionState, // Current transition state
    pub override_priority: u8,     // Priority for override resolution
}

/// System transformation for a specific dread level
#[derive(Debug, Clone)]
pub struct SystemTransformLevel {
    pub level: u8,                // Dread level this applies to
    pub parameter_multipliers: HashMap<String, f32>, // Multiply base parameters
    pub parameter_replacements: HashMap<String, f32>, // Replace parameters entirely
    pub feature_toggles: HashMap<String, bool>, // Enable/disable features
    pub visual_overrides: HashMap<String, String>, // Visual appearance changes
    pub behavioral_overrides: HashMap<String, String>, // Behavior changes
    pub audio_overrides: HashMap<String, String>, // Audio changes
}

/// Current state of system transformation
#[derive(Debug, Clone)]
pub enum TransitionState {
    Stable,                       // No transition in progress
    Transitioning {
        from_level: u8,
        to_level: u8,
        progress: f32,            // 0.0-1.0 transition progress
        duration: f32,            // Total transition duration
    },
    Corrupted {
        corruption_level: f32,    // How corrupted the transition is
        recovery_possible: bool,  // Can this be fixed?
    },
}

/// Player adaptation data for dread progression
#[derive(Debug, Clone)]
pub struct PlayerDreadAdaptation {
    pub player_id: Uuid,
    pub baseline_sensitivity: f32, // Base sensitivity to dread
    pub current_adaptation: [f32; 5], // Adaptation to each dread level 0-4
    pub habituation_curves: HashMap<String, HabituationCurve>, // Adaptation to specific sources
    pub sensitization_triggers: Vec<String>, // Things that make player more sensitive
    pub comfort_zones: Vec<DreadComfortZone>, // Dread levels player is comfortable with
    pub breakthrough_threshold: f32, // Dread level that always affects player
}

/// Habituation curve for a specific dread source
#[derive(Debug, Clone)]
pub struct HabituationCurve {
    pub source_id: String,
    pub exposure_time: f32,       // Total exposure time in seconds
    pub habituation_rate: f32,    // How fast habituation occurs
    pub maximum_habituation: f32, // Maximum habituation possible
    pub current_habituation: f32, // Current habituation level
    pub decay_rate: f32,          // How fast habituation decays without exposure
}

/// Comfort zone for specific dread level range
#[derive(Debug, Clone)]
pub struct DreadComfortZone {
    pub min_dread: f32,
    pub max_dread: f32,
    pub comfort_level: f32,       // 0.0-1.0 how comfortable in this range
    pub established_time: i64,    // When this comfort zone was established
    pub stability: f32,           // How stable this comfort zone is
}

/// Configuration for dread progression mechanics
#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct DreadProgressionConfig {
    // Basic dread mechanics
    pub dread_level_thresholds: [f32; 5], // Thresholds for dread levels 0-4
    pub base_progression_rate: f32, // Base rate of dread change per second
    pub stability_requirements: [f32; 5], // Stability needed to maintain each level
    
    // System transformation configuration
    pub system_configs: HashMap<String, SystemDreadConfig>, // Per-system configuration
    pub transition_speeds: [f32; 5], // How fast transitions occur for each level
    pub override_priorities: HashMap<String, u8>, // System override priorities
    
    // Environmental dread
    pub biome_dread_multipliers: HashMap<String, f32>, // Dread multipliers by biome
    pub corruption_dread_scaling: f32, // How corruption affects dread
    pub isolation_dread_factor: f32,   // How isolation increases dread
    
    // Companion interaction with dread
    pub companion_dread_contributions: HashMap<String, f32>, // How companions affect dread
    pub companion_dread_resistances: HashMap<String, f32>,   // How companions resist dread
    pub support_network_dread_reduction: f32, // How companion support reduces dread
    
    // Narrative dread progression
    pub act_dread_baselines: [f32; 3], // Base dread for story acts 1-3
    pub revelation_dread_spikes: Vec<DreadSpike>, // Dread spikes from story revelations
    pub player_agency_dread_relationship: f32, // How player agency affects dread
    
    // Dragon-specific dread mechanics (unique to Dragon's Labyrinth)
    pub dragon_presence_dread_curve: Vec<(f32, f32)>, // Distance to dread mapping
    pub dragon_intelligence_dread_factor: f32, // How dragon intelligence affects dread
    pub dragon_stalking_dread_buildup: f32,   // Dread buildup when being stalked
}

/// Configuration for how a specific system responds to dread
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemDreadConfig {
    pub system_name: String,
    pub dread_sensitivity: f32,   // 0.0-1.0 how sensitive system is to dread
    pub transformation_threshold: [f32; 5], // Dread thresholds for transformations
    pub parameter_scaling: HashMap<String, [f32; 5]>, // Parameter scaling for each dread level
    pub feature_availability: HashMap<String, [bool; 5]>, // Feature availability by dread level
    pub visual_corruption_levels: [f32; 5], // Visual corruption at each dread level
    pub audio_distortion_levels: [f32; 5],  // Audio distortion at each dread level
    pub interaction_modifications: HashMap<String, [f32; 5]>, // UI/interaction changes
}

/// Dread spike from narrative events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DreadSpike {
    pub trigger_event: String,    // What narrative event triggers this
    pub spike_intensity: f32,     // How much dread spikes
    pub spike_duration: f32,      // How long spike lasts
    pub decay_pattern: String,    // "linear", "exponential", "logarithmic"
    pub prerequisite_dread_level: u8, // Minimum dread level for this to trigger
    pub can_push_to_next_level: bool, // Can this spike push to next dread level?
}

impl Default for DreadProgressionConfig {
    fn default() -> Self {
        let mut system_configs = HashMap::new();
        
        // Combat system dread configuration
        system_configs.insert("combat".to_string(), SystemDreadConfig {
            system_name: "combat".to_string(),
            dread_sensitivity: 0.8,
            transformation_threshold: [0.0, 1.0, 2.0, 3.5, 4.2],
            parameter_scaling: {
                let mut scaling = HashMap::new();
                scaling.insert("enemy_aggression".to_string(), [1.0, 1.2, 1.5, 2.0, 3.0]);
                scaling.insert("player_accuracy".to_string(), [1.0, 0.9, 0.8, 0.6, 0.4]);
                scaling.insert("healing_effectiveness".to_string(), [1.0, 0.95, 0.9, 0.8, 0.6]);
                scaling.insert("companion_coordination".to_string(), [1.0, 0.9, 0.7, 0.5, 0.3]);
                scaling
            },
            feature_availability: {
                let mut features = HashMap::new();
                features.insert("tactical_pause".to_string(), [true, true, true, false, false]);
                features.insert("retreat_option".to_string(), [true, true, false, false, false]);
                features.insert("companion_commands".to_string(), [true, true, true, false, false]);
                features
            },
            visual_corruption_levels: [0.0, 0.1, 0.3, 0.6, 1.0],
            audio_distortion_levels: [0.0, 0.05, 0.2, 0.5, 0.8],
            interaction_modifications: {
                let mut interactions = HashMap::new();
                interactions.insert("ui_responsiveness".to_string(), [1.0, 0.95, 0.9, 0.8, 0.6]);
                interactions.insert("menu_clarity".to_string(), [1.0, 0.9, 0.8, 0.6, 0.4]);
                interactions
            },
        });
        
        // Hex rendering system dread configuration
        system_configs.insert("hex_rendering".to_string(), SystemDreadConfig {
            system_name: "hex_rendering".to_string(),
            dread_sensitivity: 1.0, // Very sensitive to create visual horror
            transformation_threshold: [0.0, 0.5, 1.5, 2.5, 3.5],
            parameter_scaling: {
                let mut scaling = HashMap::new();
                scaling.insert("visibility_range".to_string(), [1.0, 0.9, 0.7, 0.5, 0.3]);
                scaling.insert("color_saturation".to_string(), [1.0, 0.8, 0.6, 0.3, 0.1]);
                scaling.insert("shadow_depth".to_string(), [1.0, 1.3, 1.7, 2.5, 4.0]);
                scaling.insert("texture_corruption".to_string(), [0.0, 0.1, 0.3, 0.7, 1.0]);
                scaling
            },
            feature_availability: {
                let mut features = HashMap::new();
                features.insert("minimap".to_string(), [true, true, true, false, false]);
                features.insert("waypoint_markers".to_string(), [true, true, false, false, false]);
                features.insert("distance_indicators".to_string(), [true, false, false, false, false]);
                features
            },
            visual_corruption_levels: [0.0, 0.2, 0.5, 0.8, 1.0],
            audio_distortion_levels: [0.0, 0.1, 0.3, 0.6, 0.9],
            interaction_modifications: HashMap::new(),
        });
        
        // Dialogue system dread configuration
        system_configs.insert("dialogue".to_string(), SystemDreadConfig {
            system_name: "dialogue".to_string(),
            dread_sensitivity: 0.6,
            transformation_threshold: [0.0, 1.0, 2.0, 3.0, 4.0],
            parameter_scaling: {
                let mut scaling = HashMap::new();
                scaling.insert("dialogue_clarity".to_string(), [1.0, 0.9, 0.7, 0.5, 0.2]);
                scaling.insert("emotional_range".to_string(), [1.0, 0.8, 0.6, 0.4, 0.2]);
                scaling.insert("trust_options_availability".to_string(), [1.0, 0.9, 0.7, 0.4, 0.1]);
                scaling
            },
            feature_availability: {
                let mut features = HashMap::new();
                features.insert("save_conversation".to_string(), [true, true, false, false, false]);
                features.insert("dialogue_history".to_string(), [true, true, true, false, false]);
                features.insert("emotional_indicators".to_string(), [true, true, false, false, false]);
                features
            },
            visual_corruption_levels: [0.0, 0.1, 0.2, 0.4, 0.7],
            audio_distortion_levels: [0.0, 0.15, 0.35, 0.6, 0.85],
            interaction_modifications: {
                let mut interactions = HashMap::new();
                interactions.insert("response_time_pressure".to_string(), [1.0, 1.1, 1.3, 1.7, 2.5]);
                interactions
            },
        });
        
        // Companion psychology system dread configuration
        system_configs.insert("companion_psychology".to_string(), SystemDreadConfig {
            system_name: "companion_psychology".to_string(),
            dread_sensitivity: 0.9, // Highly sensitive to dread
            transformation_threshold: [0.0, 0.8, 1.8, 2.8, 3.8],
            parameter_scaling: {
                let mut scaling = HashMap::new();
                scaling.insert("trauma_accumulation_rate".to_string(), [1.0, 1.3, 1.7, 2.2, 3.0]);
                scaling.insert("therapy_effectiveness".to_string(), [1.0, 0.9, 0.7, 0.5, 0.2]);
                scaling.insert("breakdown_resistance".to_string(), [1.0, 0.8, 0.6, 0.4, 0.1]);
                scaling.insert("recovery_rate".to_string(), [1.0, 0.85, 0.7, 0.5, 0.2]);
                scaling
            },
            feature_availability: {
                let mut features = HashMap::new();
                features.insert("memory_palace_access".to_string(), [true, true, true, false, false]);
                features.insert("professional_support".to_string(), [true, true, false, false, false]);
                features.insert("peer_support_effectiveness".to_string(), [true, true, true, false, false]);
                features
            },
            visual_corruption_levels: [0.0, 0.15, 0.4, 0.7, 1.0],
            audio_distortion_levels: [0.0, 0.2, 0.4, 0.7, 1.0],
            interaction_modifications: HashMap::new(),
        });
        
        Self {
            dread_level_thresholds: [0.0, 1.0, 2.0, 3.0, 4.0],
            base_progression_rate: 0.01, // Very slow base progression
            stability_requirements: [0.0, 0.2, 0.4, 0.6, 0.8], // Higher levels need more stability
            system_configs,
            transition_speeds: [2.0, 3.0, 4.0, 6.0, 8.0], // Faster transitions at higher levels
            override_priorities: {
                let mut priorities = HashMap::new();
                priorities.insert("dragon_presence".to_string(), 10); // Highest priority
                priorities.insert("companion_breakdown".to_string(), 8);
                priorities.insert("reality_distortion".to_string(), 7);
                priorities.insert("narrative_revelation".to_string(), 6);
                priorities.insert("environmental_corruption".to_string(), 5);
                priorities.insert("player_choice".to_string(), 3);
                priorities
            },
            biome_dread_multipliers: {
                let mut multipliers = HashMap::new();
                multipliers.insert("swamp".to_string(), 1.4);
                multipliers.insert("mountain".to_string(), 1.2);
                multipliers.insert("jungle".to_string(), 1.3);
                multipliers.insert("forest".to_string(), 1.0);
                multipliers.insert("plains".to_string(), 0.8);
                multipliers.insert("desert".to_string(), 1.1);
                multipliers.insert("tundra".to_string(), 1.1);
                multipliers.insert("corruption_zone".to_string(), 2.0);
                multipliers.insert("void_touched".to_string(), 3.0);
                multipliers
            },
            corruption_dread_scaling: 1.5,
            isolation_dread_factor: 0.3,
            companion_dread_contributions: {
                let mut contributions = HashMap::new();
                contributions.insert("einar_breakdown".to_string(), 0.8);
                contributions.insert("mira_despair".to_string(), 1.0);
                contributions.insert("sorin_paranoia".to_string(), 0.6);
                contributions.insert("tamara_madness".to_string(), 1.2);
                contributions
            },
            companion_dread_resistances: {
                let mut resistances = HashMap::new();
                resistances.insert("einar_presence".to_string(), 0.2);
                resistances.insert("mira_healing".to_string(), 0.3);
                resistances.insert("sorin_watchfulness".to_string(), 0.15);
                resistances.insert("tamara_knowledge".to_string(), 0.25);
                resistances
            },
            support_network_dread_reduction: 0.4,
            act_dread_baselines: [0.5, 2.0, 3.5], // Act 1: low, Act 2: medium, Act 3: high
            revelation_dread_spikes: vec![
                DreadSpike {
                    trigger_event: "dragon_first_sighting".to_string(),
                    spike_intensity: 1.5,
                    spike_duration: 300.0, // 5 minutes
                    decay_pattern: "exponential".to_string(),
                    prerequisite_dread_level: 0,
                    can_push_to_next_level: true,
                },
                DreadSpike {
                    trigger_event: "companion_first_death".to_string(),
                    spike_intensity: 2.0,
                    spike_duration: 600.0, // 10 minutes
                    decay_pattern: "logarithmic".to_string(),
                    prerequisite_dread_level: 1,
                    can_push_to_next_level: true,
                },
                DreadSpike {
                    trigger_event: "reality_first_break".to_string(),
                    spike_intensity: 1.8,
                    spike_duration: 900.0, // 15 minutes
                    decay_pattern: "linear".to_string(),
                    prerequisite_dread_level: 2,
                    can_push_to_next_level: true,
                },
                DreadSpike {
                    trigger_event: "dragon_intelligence_revealed".to_string(),
                    spike_intensity: 2.5,
                    spike_duration: 1200.0, // 20 minutes
                    decay_pattern: "exponential".to_string(),
                    prerequisite_dread_level: 2,
                    can_push_to_next_level: true,
                },
                DreadSpike {
                    trigger_event: "final_truth_revelation".to_string(),
                    spike_intensity: 3.0,
                    spike_duration: 0.0, // Permanent
                    decay_pattern: "none".to_string(),
                    prerequisite_dread_level: 3,
                    can_push_to_next_level: true,
                },
            ],
            player_agency_dread_relationship: -0.3, // More agency = less dread
            dragon_presence_dread_curve: vec![
                (1000.0, 0.0),  // >1000 units: no dread
                (500.0, 0.5),   // 500 units: slight dread
                (200.0, 1.0),   // 200 units: noticeable dread
                (100.0, 2.0),   // 100 units: significant dread
                (50.0, 3.0),    // 50 units: major dread
                (20.0, 4.0),    // 20 units: maximum dread
                (0.0, 5.0),     // 0 units: overwhelming dread
            ],
            dragon_intelligence_dread_factor: 1.5, // Dragon intelligence multiplies dread
            dragon_stalking_dread_buildup: 0.02,   // Dread per second when being stalked
        }
    }
}

impl DreadProgressionState {
    pub async fn new(db: DatabaseConnection) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            db,
            global_dread_level: 0,
            regional_dread_levels: HashMap::new(),
            active_dread_sources: HashMap::new(),
            system_transformations: HashMap::new(),
            dread_history: Vec::new(),
            milestone_tracker: HashMap::new(),
            player_adaptation_data: HashMap::new(),
        })
    }
    
    /// Add a new dread source to the system
    pub fn add_dread_source(&mut self, source: DreadSource) {
        self.active_dread_sources.insert(source.source_id.clone(), source);
    }
    
    /// Remove a dread source from the system
    pub fn remove_dread_source(&mut self, source_id: &str) {
        self.active_dread_sources.remove(source_id);
    }
    
    /// Calculate total dread from all active sources
    pub fn calculate_total_dread(&self) -> f32 {
        let mut total_dread = 0.0;
        let mut compounding_factor = 1.0;
        
        // Sum base dread from all sources
        for source in self.active_dread_sources.values() {
            total_dread += source.intensity;
            compounding_factor += source.compounding_factor;
        }
        
        // Apply compounding (multiple sources amplify each other)
        total_dread *= compounding_factor;
        
        total_dread.min(5.0) // Cap at maximum dread level
    }
    
    /// Get the appropriate dread level (0-4) for a given dread value
    pub fn get_dread_level(&self, dread_value: f32, config: &DreadProgressionConfig) -> u8 {
        for (level, &threshold) in config.dread_level_thresholds.iter().enumerate().rev() {
            if dread_value >= threshold {
                return level as u8;
            }
        }
        0
    }
    
    /// Record a change in dread level
    pub fn record_dread_change(&mut self, old_level: u8, new_level: u8, trigger: String, affected_systems: Vec<String>) {
        self.dread_history.push(DreadLevelChange {
            timestamp: chrono::Utc::now().timestamp(),
            old_level,
            new_level,
            trigger_source: trigger,
            affected_systems,
            player_reaction_time: 0.0, // Would be measured in actual implementation
        });
        
        // Keep history manageable
        if self.dread_history.len() > 1000 {
            self.dread_history.drain(0..100); // Remove oldest 100 entries
        }
    }
    
    /// Get player adaptation data or create default
    pub fn get_player_adaptation(&mut self, player_id: Uuid) -> &mut PlayerDreadAdaptation {
        self.player_adaptation_data.entry(player_id).or_insert_with(|| {
            PlayerDreadAdaptation {
                player_id,
                baseline_sensitivity: 1.0,
                current_adaptation: [0.0, 0.0, 0.0, 0.0, 0.0],
                habituation_curves: HashMap::new(),
                sensitization_triggers: Vec::new(),
                comfort_zones: vec![
                    DreadComfortZone {
                        min_dread: 0.0,
                        max_dread: 0.5,
                        comfort_level: 1.0,
                        established_time: chrono::Utc::now().timestamp(),
                        stability: 1.0,
                    }
                ],
                breakthrough_threshold: 4.0, // Max dread always affects player
            }
        })
    }
    
    /// Check if a milestone has been achieved
    pub fn is_milestone_achieved(&self, milestone_id: &str) -> bool {
        self.milestone_tracker.get(milestone_id).copied().unwrap_or(false)
    }
    
    /// Mark a milestone as achieved
    pub fn achieve_milestone(&mut self, milestone_id: String) {
        self.milestone_tracker.insert(milestone_id, true);
    }
}

/// Resource for tracking dread auras in the world
#[derive(Resource, Debug, Default)]
pub struct DreadAuraManager {
    pub active_auras: HashMap<Entity, DreadAura>,
    pub aura_interactions: Vec<AuraInteraction>, // How auras interact with each other
    pub spatial_index: HashMap<String, Vec<Entity>>, // Spatial indexing for performance
}

/// Interaction between two dread auras
#[derive(Debug, Clone)]
pub struct AuraInteraction {
    pub aura1_entity: Entity,
    pub aura2_entity: Entity,
    pub interaction_type: String, // "amplification", "interference", "resonance"
    pub interaction_strength: f32, // How strong the interaction is
    pub interaction_radius: f32,  // Radius where interaction occurs
}

impl DreadAuraManager {
    /// Add a dread aura to management
    pub fn add_aura(&mut self, entity: Entity, aura: DreadAura) {
        self.active_auras.insert(entity, aura);
        // Update spatial indexing if needed
    }
    
    /// Remove a dread aura
    pub fn remove_aura(&mut self, entity: Entity) {
        self.active_auras.remove(&entity);
        // Update spatial indexing
    }
    
    /// Calculate total dread influence at a position
    pub fn calculate_dread_at_position(&self, position: Vec3) -> f32 {
        let mut total_dread = 0.0;
        
        for (entity, aura) in &self.active_auras {
            // Would calculate distance and apply falloff curve
            // For now, simplified calculation
            let distance = 100.0; // Would be actual distance calculation
            
            if distance <= aura.effective_radius {
                let falloff = match aura.falloff_curve.as_str() {
                    "linear" => 1.0 - (distance / aura.effective_radius),
                    "exponential" => (-distance / (aura.effective_radius * 0.3)).exp(),
                    "inverse_square" => 1.0 / (1.0 + distance * distance / (aura.effective_radius * aura.effective_radius)),
                    _ => 1.0 - (distance / aura.effective_radius),
                };
                
                total_dread += aura.current_intensity * falloff;
            }
        }
        
        total_dread
    }
}

/// Resource for managing reality distortions at high dread levels
#[derive(Resource, Debug, Default)]
pub struct RealityDistortionManager {
    pub active_distortions: HashMap<Entity, RealityDistortion>,
    pub distortion_zones: Vec<DistortionZone>, // Geographic areas affected by distortion
    pub player_perception_filters: HashMap<Uuid, PerceptionFilter>, // How each player perceives distortions
}

/// Geographic zone affected by reality distortion
#[derive(Debug, Clone)]
pub struct DistortionZone {
    pub zone_id: String,
    pub center_position: Vec3,
    pub affected_radius: f32,
    pub distortion_types: Vec<String>, // Types of distortion in this zone
    pub intensity_gradient: f32,      // How intensity changes from center to edge
    pub stability: f32,               // How stable the distortion is
    pub manifestation_triggers: Vec<String>, // What triggers manifestations
}

/// How a player perceives reality distortions
#[derive(Debug, Clone)]
pub struct PerceptionFilter {
    pub player_id: Uuid,
    pub distortion_sensitivity: f32, // 0.0-1.0 how much player notices distortions
    pub adaptation_level: f32,       // 0.0-1.0 how adapted player is to distortions
    pub cognitive_filters: Vec<String>, // Mental filters that affect perception
    pub sanity_threshold: f32,       // Threshold for maintaining rational perception
    pub reality_anchor_strength: f32, // How well player maintains grip on reality
}

impl RealityDistortionManager {
    /// Add a reality distortion to the system
    pub fn add_distortion(&mut self, entity: Entity, distortion: RealityDistortion) {
        self.active_distortions.insert(entity, distortion);
    }
    
    /// Remove a reality distortion
    pub fn remove_distortion(&mut self, entity: Entity) {
        self.active_distortions.remove(&entity);
    }
    
    /// Get distortion intensity at a position for a specific player
    pub fn get_distortion_for_player(&self, position: Vec3, player_id: Uuid) -> f32 {
        let base_distortion = self.calculate_base_distortion_at_position(position);
        
        if let Some(filter) = self.player_perception_filters.get(&player_id) {
            // Apply player's perception filter
            base_distortion * filter.distortion_sensitivity * (1.0 - filter.adaptation_level)
        } else {
            base_distortion
        }
    }
    
    /// Calculate base reality distortion at a position
    fn calculate_base_distortion_at_position(&self, position: Vec3) -> f32 {
        let mut total_distortion = 0.0;
        
        for distortion in self.active_distortions.values() {
            let distance = (position - Vec3::ZERO).length(); // Would use actual position calculation
            if distance <= distortion.affected_radius {
                let falloff = 1.0 - (distance / distortion.affected_radius);
                total_distortion += distortion.intensity * falloff;
            }
        }
        
        // Check distortion zones
        for zone in &self.distortion_zones {
            let distance = (position - zone.center_position).length();
            if distance <= zone.affected_radius {
                let falloff = 1.0 - (distance / zone.affected_radius) * zone.intensity_gradient;
                total_distortion += falloff;
            }
        }
        
        total_distortion.min(1.0)
    }
}
