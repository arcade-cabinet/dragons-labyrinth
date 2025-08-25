//! Companion ECS Components - Sophisticated Psychology System
//!
//! Pure Bevy ECS components for companion psychology, trauma processing, and betrayal mechanics.
//! Integrates with the companion psychology system for therapy, recovery, and relationship dynamics.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Core companion identity and character information
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct Companion {
    pub companion_id: String, // "einar", "mira", "sorin", "tamara"
    pub name: String,
    pub display_name: String,
    pub companion_type: CompanionType,
    pub backstory: String,
}

/// Combat and gameplay stats for companions
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct CompanionStats {
    pub level: u32,
    pub health: f32,
    pub max_health: f32,
    pub attack: f32,
    pub defense: f32,
    pub movement_speed: f32,
}

/// Core psychology component - integrates with psychology system
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct CompanionPsychology {
    /// Current trauma level (0.0-5.0)
    pub trauma_level: f32,
    /// Point at which companion will break/leave (unique per companion)
    pub breaking_point: f32,
    /// Loyalty to player (0.0-1.0)
    pub loyalty: f32,
    /// Trust in player (0.0-1.0)
    pub trust: f32,
    /// Isolation tendency (increases with dread)
    pub isolation_tendency: f32,
    /// Current psychological state
    pub psychological_state: PsychologicalState,
    /// Active trauma responses affecting behavior
    pub active_trauma_responses: Vec<TraumaResponse>,
    /// Companion's unique coping mechanisms
    pub coping_mechanisms: Vec<CopingMechanism>,
    /// Therapy readiness (0.0-1.0)
    pub therapy_readiness: f32,
    /// Therapeutic bond strength (0.0-1.0)
    pub therapeutic_bond: f32,
    /// Recovery progress (0.0-1.0)
    pub recovery_progress: f32,
    /// Breakthrough potential (0.0-1.0)
    pub breakthrough_potential: f32,
    /// Companion's unique ID for psychology tracking
    pub companion_id: String,
}

/// Trauma sources and processing for sophisticated trauma mechanics
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct TraumaSources {
    /// All trauma events experienced by this companion
    pub trauma_events: Vec<TraumaEvent>,
    /// Specific triggers that cause trauma responses
    pub trauma_triggers: HashMap<String, f32>, // trigger -> sensitivity
    /// Environmental trauma accumulation
    pub environmental_trauma: f32,
    /// Witness trauma from seeing horrible events
    pub witness_trauma: f32,
    /// Combat-related trauma
    pub combat_trauma: f32,
    /// Betrayal/abandonment trauma
    pub relationship_trauma: f32,
}

/// Relationship progression and story mechanics
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct CompanionRelationship {
    /// Current story arc stage
    pub current_arc_stage: String,
    /// Progress on personal quests
    pub personal_quest_progress: HashMap<String, f32>,
    /// Story flags and player choices affecting relationship
    pub relationship_flags: HashMap<String, bool>,
    /// Whether companion has betrayed player
    pub has_betrayed: bool,
    /// Reason for betrayal (if applicable)
    pub betrayal_reason: Option<BetrayalReason>,
    /// Whether companion can return after betrayal
    pub can_return: bool,
    /// Redemption arc progress (0.0-1.0)
    pub redemption_progress: f32,
}

/// Companion availability and world presence
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct CompanionAvailability {
    /// Currently traveling with player
    pub is_active: bool,
    /// Can be recruited/found
    pub is_available: bool,
    /// Current world location (if not active)
    pub current_location: Option<Entity>, // Reference to hex tile entity
    /// Reason for departure
    pub departure_reason: Option<DepartureReason>,
    /// When they departed (for tracking return conditions)
    pub departure_time: Option<f64>,
    /// Conditions required for return
    pub return_conditions: Vec<ReturnCondition>,
}

/// Mount system integration for companions who can be mounts
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct CompanionMount {
    /// Can be used as mount
    pub is_mountable: bool,
    /// Movement speed bonus when used as mount
    pub mount_speed: f32,
    /// Carrying capacity bonus
    pub mount_capacity: f32,
    /// Stamina system for mount usage
    pub mount_stamina: f32,
    pub max_mount_stamina: f32,
    /// Whether companion likes being used as mount
    pub mount_comfort_level: f32,
}

/// Visual progression through AI-generated trauma states
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct CompanionVisuals {
    /// Base visual state (peaceful/healthy appearance)
    pub base_visual: Handle<Scene>,
    /// Current trauma visual level (0-4)
    pub trauma_visual_level: u8,
    /// AI-generated visual variants for each trauma level
    pub trauma_visuals: HashMap<u8, Handle<Scene>>,
    /// Current active visual state
    pub current_visual: Handle<Scene>,
    /// Transition progress when changing visual states
    pub visual_transition_progress: f32,
}

/// Dialogue system integration for dynamic conversation
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct CompanionDialogue {
    /// Base dialogue tree reference
    pub base_dialogue_tree: String,
    /// Trauma-affected dialogue variants
    pub trauma_dialogue_variants: HashMap<u8, String>,
    /// Trust-based dialogue options
    pub trust_dialogue_tiers: HashMap<String, f32>, // dialogue_id -> required_trust
    /// Available therapeutic dialogue options
    pub therapeutic_dialogue: Vec<String>,
    /// Last conversation timestamp
    pub last_conversation_time: Option<f64>,
}

/// Therapy system integration
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct TherapyParticipant {
    /// Active therapy sessions
    pub active_sessions: Vec<Entity>, // References to therapy session entities
    /// Therapy effectiveness for this companion
    pub therapy_effectiveness: f32,
    /// Preferred therapy methods
    pub preferred_methods: Vec<TherapyMethod>,
    /// Therapy resistance/barriers
    pub therapy_barriers: Vec<String>,
    /// Progress towards breakthroughs
    pub breakthrough_progress: HashMap<String, f32>,
    /// Professional support access
    pub professional_support_access: f32,
}

/// Psychological resilience and recovery factors
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct PsychologicalResilience {
    /// Emotional regulation ability
    pub emotional_regulation: f32,
    /// Social connection strength
    pub social_connection: f32,
    /// Meaning-making ability
    pub meaning_making: f32,
    /// Self-efficacy
    pub self_efficacy: f32,
    /// Hope and optimism
    pub hope_level: f32,
    /// Adaptive coping strategies
    pub adaptive_coping: f32,
}

// Supporting data structures

#[derive(Debug, Clone, PartialEq, Reflect, Serialize, Deserialize)]
pub enum CompanionType {
    Warrior, // Einar - combat-focused, high loyalty but trauma-prone
    Healer,  // Mira - support-focused, high therapy effectiveness
    Rogue,   // Sorin - stealth-focused, trust issues but adaptable
    Mage,    // Tamara - magic-focused, reality distortion sensitivity
}

#[derive(Debug, Clone, PartialEq, Reflect, Serialize, Deserialize)]
pub enum PsychologicalState {
    Stable,
    Anxious,
    Depressed,
    Traumatized,
    Dissociative,
    Hypervigilant,
    InRecovery,
    Breakthrough,
}

#[derive(Debug, Clone, Reflect, Serialize, Deserialize)]
pub struct TraumaResponse {
    pub response_type: String, // "flashback", "panic", "dissociation", "hypervigilance"
    pub intensity: f32, // 0.0-1.0
    pub duration: f32, // How long this response lasts
    pub triggers: Vec<String>, // What can trigger this response
    pub behavioral_effects: HashMap<String, f32>, // Effect on different behaviors
}

#[derive(Debug, Clone, Reflect, Serialize, Deserialize)]
pub struct CopingMechanism {
    pub mechanism_type: String, // "avoidance", "humor", "problem_solving", "seeking_support"
    pub effectiveness: f32, // How well this works for this companion
    pub conditions: Vec<String>, // When this coping mechanism is used
    pub side_effects: Vec<String>, // Negative effects of this coping
}

#[derive(Debug, Clone, Reflect, Serialize, Deserialize)]
pub struct TraumaEvent {
    pub event_id: String,
    pub event_type: String, // "combat", "witnessing_death", "betrayal", "environmental"
    pub severity: f32, // 0.0-1.0
    pub context: String,
    pub timestamp: f64,
    pub processed: bool, // Whether this has been processed through therapy
    pub impact_areas: Vec<String>, // Which areas of psychology this affects
}

#[derive(Debug, Clone, PartialEq, Reflect, Serialize, Deserialize)]
pub enum BetrayalReason {
    TraumaOverload,
    LostTrust,
    FearForSafety,
    PhilosophicalDifference,
    ProtectingOthers,
    Corruption,
}

#[derive(Debug, Clone, PartialEq, Reflect, Serialize, Deserialize)]
pub enum DepartureReason {
    Trauma,
    Safety,
    PersonalQuest,
    IdeologicalDifference,
    TemporaryLeave,
    Death,
}

#[derive(Debug, Clone, Reflect, Serialize, Deserialize)]
pub struct ReturnCondition {
    pub condition_type: String, // "trauma_recovery", "trust_rebuilding", "story_milestone"
    pub threshold: f32, // Required value to meet condition
    pub current_progress: f32,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Reflect, Serialize, Deserialize)]
pub enum TherapyMethod {
    CognitiveBehavioral,
    TraumaFocused,
    Somatic,
    Narrative,
    GroupTherapy,
    PeerSupport,
}

/// Bundle for spawning complete companion entities
#[derive(Bundle)]
pub struct CompanionBundle {
    pub companion: Companion,
    pub stats: CompanionStats,
    pub psychology: CompanionPsychology,
    pub trauma_sources: TraumaSources,
    pub relationship: CompanionRelationship,
    pub availability: CompanionAvailability,
    pub mount: CompanionMount,
    pub visuals: CompanionVisuals,
    pub dialogue: CompanionDialogue,
    pub therapy_participant: TherapyParticipant,
    pub resilience: PsychologicalResilience,
    
    // Bevy rendering components
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}

impl CompanionBundle {
    pub fn new_einar(asset_server: &AssetServer, position: Vec3) -> Self {
        Self {
            companion: Companion {
                companion_id: "einar".to_string(),
                name: "Einar".to_string(),
                display_name: "Einar the Steadfast".to_string(),
                companion_type: CompanionType::Warrior,
                backstory: "A loyal warrior with unshakeable dedication".to_string(),
            },
            stats: CompanionStats {
                level: 1,
                health: 120.0,
                max_health: 120.0,
                attack: 15.0,
                defense: 12.0,
                movement_speed: 1.0,
            },
            psychology: CompanionPsychology {
                trauma_level: 0.0,
                breaking_point: 4.2, // Higher than average - very loyal
                loyalty: 0.9, // Starts very loyal
                trust: 0.7, // Good initial trust
                isolation_tendency: 0.1, // Low isolation tendency
                psychological_state: PsychologicalState::Stable,
                active_trauma_responses: Vec::new(),
                coping_mechanisms: vec![
                    CopingMechanism {
                        mechanism_type: "protective_duty".to_string(),
                        effectiveness: 0.8,
                        conditions: vec!["protecting_others".to_string()],
                        side_effects: vec!["self_neglect".to_string()],
                    }
                ],
                therapy_readiness: 0.6, // Moderate therapy openness
                therapeutic_bond: 0.0,
                recovery_progress: 0.0,
                breakthrough_potential: 0.3,
                companion_id: "einar".to_string(),
            },
            trauma_sources: TraumaSources {
                trauma_events: Vec::new(),
                trauma_triggers: HashMap::from([
                    ("ally_death".to_string(), 0.9), // Very sensitive to losing allies
                    ("failure_to_protect".to_string(), 0.8),
                ]),
                environmental_trauma: 0.0,
                witness_trauma: 0.0,
                combat_trauma: 0.0,
                relationship_trauma: 0.0,
            },
            relationship: CompanionRelationship {
                current_arc_stage: "introduction".to_string(),
                personal_quest_progress: HashMap::new(),
                relationship_flags: HashMap::new(),
                has_betrayed: false,
                betrayal_reason: None,
                can_return: true,
                redemption_progress: 0.0,
            },
            availability: CompanionAvailability {
                is_active: true,
                is_available: true,
                current_location: None,
                departure_reason: None,
                departure_time: None,
                return_conditions: Vec::new(),
            },
            mount: CompanionMount {
                is_mountable: false, // Einar is not a mount
                mount_speed: 0.0,
                mount_capacity: 0.0,
                mount_stamina: 0.0,
                max_mount_stamina: 0.0,
                mount_comfort_level: 0.0,
            },
            visuals: CompanionVisuals {
                base_visual: asset_server.load("models/companions/einar_base.glb#Scene0"),
                trauma_visual_level: 0,
                trauma_visuals: HashMap::from([
                    (0, asset_server.load("models/companions/einar_base.glb#Scene0")),
                    (1, asset_server.load("models/companions/einar_stressed.glb#Scene0")),
                    (2, asset_server.load("models/companions/einar_traumatized.glb#Scene0")),
                    (3, asset_server.load("models/companions/einar_breaking.glb#Scene0")),
                    (4, asset_server.load("models/companions/einar_broken.glb#Scene0")),
                ]),
                current_visual: asset_server.load("models/companions/einar_base.glb#Scene0"),
                visual_transition_progress: 0.0,
            },
            dialogue: CompanionDialogue {
                base_dialogue_tree: "einar_base_dialogue".to_string(),
                trauma_dialogue_variants: HashMap::from([
                    (1, "einar_stressed_dialogue".to_string()),
                    (2, "einar_traumatized_dialogue".to_string()),
                    (3, "einar_breaking_dialogue".to_string()),
                    (4, "einar_broken_dialogue".to_string()),
                ]),
                trust_dialogue_tiers: HashMap::new(),
                therapeutic_dialogue: vec!["einar_therapy_dialogue".to_string()],
                last_conversation_time: None,
            },
            therapy_participant: TherapyParticipant {
                active_sessions: Vec::new(),
                therapy_effectiveness: 0.7, // Good response to therapy
                preferred_methods: vec![TherapyMethod::CognitiveBehavioral, TherapyMethod::PeerSupport],
                therapy_barriers: vec!["stoicism".to_string(), "duty_before_self".to_string()],
                breakthrough_progress: HashMap::new(),
                professional_support_access: 0.8,
            },
            resilience: PsychologicalResilience {
                emotional_regulation: 0.7,
                social_connection: 0.8, // Strong social bonds
                meaning_making: 0.8, // Strong sense of duty provides meaning
                self_efficacy: 0.9, // High confidence in abilities
                hope_level: 0.7,
                adaptive_coping: 0.6,
            },
            transform: Transform::from_translation(position),
            global_transform: GlobalTransform::default(),
            visibility: Visibility::default(),
            inherited_visibility: InheritedVisibility::default(),
            view_visibility: ViewVisibility::default(),
        }
    }
}
