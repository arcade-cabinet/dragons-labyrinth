//! Sophisticated Companion Psychology System
//! 
//! This module implements the revolutionary companion trauma/therapy system discovered in vision integration:
//! - Trauma accumulation (0-5 levels) with specific triggers
//! - Therapy quest system with dialogue-heavy healing missions
//! - Personal story arcs (3-5 quests each) with romantic/platonic options
//! - Inter-companion relationships and support networks
//! - Companion breakdown and recovery mechanics

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

pub mod trauma;
pub mod therapy;
pub mod relationships;
pub mod breakdown;

pub use trauma::*;
pub use therapy::*;
pub use relationships::*;
pub use breakdown::*;

// ============================================================================
// PSYCHOLOGY SYSTEM PLUGIN
// ============================================================================

pub struct PsychologySystemPlugin;

impl Plugin for PsychologySystemPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<GlobalPsychologyState>()
            .init_resource::<TherapyQuestRegistry>()
            .init_resource::<CompanionRelationshipNetwork>()
            
            // Events
            .add_event::<TraumaEvent>()
            .add_event::<TherapySessionEvent>()
            .add_event::<CompanionBreakdownEvent>()
            .add_event::<RecoveryMilestoneEvent>()
            .add_event::<RelationshipChangeEvent>()
            .add_event::<TherapeuticBreakthroughEvent>()
            
            // Systems
            .add_systems(Startup, setup_psychology_system)
            .add_systems(Update, (
                // Core trauma systems
                trauma_accumulation_system,
                trauma_trigger_detection_system,
                trauma_progression_system,
                trauma_visual_update_system,
                
                // Therapy systems
                therapy_quest_progression_system,
                therapeutic_dialogue_system,
                healing_mission_system,
                breakthrough_detection_system,
                
                // Relationship systems
                inter_companion_support_system,
                relationship_development_system,
                group_therapy_system,
                peer_support_effectiveness_system,
                
                // Breakdown and recovery systems
                companion_breakdown_system,
                recovery_tracking_system,
                setback_detection_system,
                resilience_building_system,
                
                // Professional help systems
                professional_support_system,
                therapy_quality_assessment_system,
                long_term_healing_trajectory_system,
            ).chain());
    }
}

// ============================================================================
// CORE PSYCHOLOGY COMPONENTS
// ============================================================================

/// Component tracking companion trauma state with specific triggers
#[derive(Component, Reflect, Clone, Debug, Serialize, Deserialize)]
#[reflect(Component)]
pub struct CompanionTrauma {
    // Core trauma metrics
    pub trauma_level: f32,              // 0.0-5.0 trauma accumulation (0-5 scale from vision)
    pub trauma_categories: HashMap<TraumaCategory, f32>, // Specific trauma types
    pub breaking_point: f32,            // Level where companion breaks down
    pub current_stability: f32,         // 0.0-1.0 current psychological stability
    
    // Trauma triggers and patterns
    pub trauma_triggers: Vec<TraumaTrigger>, // Things that cause trauma
    pub trigger_sensitivity: HashMap<String, f32>, // How sensitive to each trigger
    pub trauma_accumulation_rate: f32,  // How fast trauma builds up
    pub trauma_recovery_rate: f32,      // Natural recovery rate
    
    // Coping and resilience
    pub coping_mechanisms: Vec<CopingMechanism>, // How companion deals with trauma
    pub psychological_strengths: Vec<String>, // Mental resilience factors
    pub emotional_vulnerabilities: Vec<String>, // Emotional weak points
    pub resilience_score: f32,          // 0.0-1.0 resistance to trauma
    
    // Visual and behavioral manifestation
    pub trauma_visual_level: u32,       // 0-4 visual trauma representation
    pub behavioral_changes: Vec<String>, // How trauma affects behavior
    pub dialogue_impact: TraumaDialogueImpact, // How trauma affects speech
    pub physical_manifestations: Vec<String>, // Physical signs of trauma
    
    // Recovery tracking
    pub recovery_progress: f32,         // 0.0-1.0 progress toward healing
    pub recovery_milestones: Vec<String>, // Healing achievements
    pub setback_count: u32,             // Number of setbacks experienced
    pub last_breakdown: Option<f64>,    // Game time of last breakdown
}

/// Component for therapy quest progression
#[derive(Component, Reflect, Clone, Debug, Serialize, Deserialize)]
#[reflect(Component)]
pub struct TherapyProgression {
    // Current therapy state
    pub therapy_quest_id: String,       // Current therapy quest
    pub therapy_stage: TherapyStage,    // Current stage of therapy
    pub therapy_progress: f32,          // 0.0-1.0 progress through current quest
    pub session_count: u32,             // Number of therapy sessions completed
    
    // Therapeutic relationship
    pub therapeutic_alliance: f32,      // 0.0-1.0 strength of therapy relationship
    pub trust_in_therapy: f32,          // 0.0-1.0 trust in therapeutic process
    pub resistance_to_therapy: f32,     // 0.0-1.0 resistance to healing
    pub therapy_readiness: f32,         // 0.0-1.0 readiness to engage in therapy
    
    // Healing missions
    pub active_healing_missions: Vec<HealingMission>, // Current therapeutic missions
    pub completed_healing_missions: Vec<String>, // Finished missions
    pub available_healing_options: Vec<String>, // Available therapeutic approaches
    
    // Therapeutic dialogue
    pub therapeutic_conversation_history: Vec<TherapeuticDialogue>, // Past conversations
    pub current_therapeutic_focus: String, // Current therapy topic
    pub dialogue_breakthrough_points: Vec<String>, // Moments of breakthrough
    pub therapeutic_insights: Vec<String>, // Insights gained through therapy
}

/// Component for inter-companion relationship tracking
#[derive(Component, Reflect, Clone, Debug, Serialize, Deserialize)]
#[reflect(Component)]
pub struct CompanionRelationships {
    // Relationship network
    pub relationships: HashMap<Entity, RelationshipData>, // Relationships with other companions
    pub support_provided: HashMap<Entity, SupportData>, // Support given to others
    pub support_received: HashMap<Entity, SupportData>, // Support received from others
    
    // Social dynamics
    pub social_role: SocialRole,        // Role in group dynamics
    pub influence_level: f32,           // 0.0-1.0 influence on group
    pub group_therapy_participation: f32, // 0.0-1.0 participation in group therapy
    pub peer_support_effectiveness: f32, // 0.0-1.0 effectiveness at helping others
    
    // Relationship quality metrics
    pub average_relationship_quality: f32, // 0.0-1.0 average relationship strength
    pub conflict_resolution_skill: f32, // 0.0-1.0 ability to resolve conflicts
    pub emotional_availability: f32,    // 0.0-1.0 availability to support others
    pub boundary_respect: f32,          // 0.0-1.0 respect for others' boundaries
}

/// Component for professional therapy support integration
#[derive(Component, Reflect, Clone, Debug, Serialize, Deserialize)]
#[reflect(Component)]
pub struct ProfessionalSupport {
    pub has_professional_help: bool,    // Is professional involved?
    pub support_type: ProfessionalSupportType, // Type of professional help
    pub support_quality: f32,           // 0.0-1.0 quality of professional help
    pub professional_relationship: f32, // 0.0-1.0 relationship with professional
    pub treatment_approach: String,     // Therapeutic approach being used
    pub professional_assessments: Vec<String>, // Professional observations
    pub treatment_plan: Vec<String>,    // Structured treatment plan
    pub professional_recommendations: Vec<String>, // Professional advice
}

// ============================================================================
// PSYCHOLOGY ENUMS AND TYPES
// ============================================================================

#[derive(Reflect, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum TraumaCategory {
    CombatTrauma,       // Trauma from violence and combat
    LossTrauma,         // Trauma from losing loved ones
    BetrayalTrauma,     // Trauma from betrayal by trusted figures
    WitnessTrauma,      // Trauma from witnessing horrible events
    AbandonmentTrauma,  // Trauma from being abandoned
    CorruptionTrauma,   // Trauma from world corruption
    MoralTrauma,        // Trauma from moral compromises
    IdentityTrauma,     // Trauma from identity crisis
}

#[derive(Reflect, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum TherapyStage {
    Assessment,         // Understanding the trauma
    Stabilization,      // Building stability and safety
    Processing,         // Working through trauma memories
    Integration,        // Integrating healing into daily life
    PostTraumatic,      // Post-traumatic growth and wisdom
    Maintenance,        // Ongoing support and relapse prevention
}

#[derive(Reflect, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum CopingMechanism {
    HealthyCoping(String),      // Positive coping strategies
    UnhealthyCoping(String),    // Negative/harmful coping strategies
    AvoidanceCoping(String),    // Avoidance-based strategies
    SocialCoping(String),       // Social support strategies
    CognitiveCoping(String),    // Thought-based strategies
    BehavioralCoping(String),   // Action-based strategies
}

#[derive(Reflect, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum SocialRole {
    Supporter,          // Provides emotional support to others
    ProtectedMember,    // Receives protection from group
    Mediator,           // Helps resolve conflicts
    Isolator,           // Tends to withdraw from group
    Leader,             // Provides guidance and direction
    Follower,           // Looks to others for guidance
}

#[derive(Reflect, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum ProfessionalSupportType {
    TraumaTherapist,    // Specialized trauma therapy
    GroupTherapist,     // Group therapy facilitation
    PsychiatricSupport, // Medication and psychiatric care
    PeerCounselor,      // Peer support specialist
    SpiritualGuide,     // Spiritual/religious support
    HealingPractitioner, // Alternative healing approaches
}

#[derive(Reflect, Clone, Debug, Serialize, Deserialize)]
pub struct TraumaTrigger {
    pub trigger_name: String,
    pub trigger_category: TraumaCategory,
    pub sensitivity_level: f32,         // 0.0-1.0 how sensitive companion is
    pub trauma_impact: f32,             // How much trauma this trigger causes
    pub avoidance_behaviors: Vec<String>, // Behaviors to avoid trigger
    pub recovery_time: f32,             // Time to recover from trigger
}

#[derive(Reflect, Clone, Debug, Serialize, Deserialize)]
pub struct TraumaDialogueImpact {
    pub speech_patterns_changed: bool,  // Has trauma affected speech?
    pub emotional_expression_limited: bool, // Limited emotional expression?
    pub trigger_word_sensitivity: Vec<String>, // Words that trigger reactions
    pub dialogue_withdrawal_tendency: f32, // 0.0-1.0 tendency to withdraw from conversation
    pub emotional_numbing_level: f32,   // 0.0-1.0 emotional numbing
}

#[derive(Reflect, Clone, Debug, Serialize, Deserialize)]
pub struct HealingMission {
    pub mission_id: String,
    pub mission_type: HealingMissionType,
    pub target_trauma_category: TraumaCategory,
    pub required_therapeutic_alliance: f32, // Minimum alliance needed
    pub dialogue_trees_required: Vec<String>, // Specific dialogues needed
    pub companion_readiness_required: f32, // Readiness to engage
    pub expected_healing_outcome: f32,  // Expected healing progress
    pub mission_complexity: f32,        // 0.0-1.0 complexity level
}

#[derive(Reflect, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum HealingMissionType {
    ExposureTherapy,    // Gradual exposure to trauma triggers
    CognitiveReprocessing, // Reprocessing traumatic memories
    EmotionalRegulation, // Learning emotional regulation skills
    SocialReintegration, // Rebuilding social connections
    MeaningMaking,      // Finding meaning in trauma experience
    PostTraumaticGrowth, // Growing beyond pre-trauma baseline
}

#[derive(Reflect, Clone, Debug, Serialize, Deserialize)]
pub struct TherapeuticDialogue {
    pub dialogue_id: String,
    pub therapeutic_technique: String,  // Therapy technique used
    pub emotional_tone: String,         // Emotional tone of session
    pub breakthrough_achieved: bool,    // Was breakthrough achieved?
    pub healing_progress_made: f32,     // 0.0-1.0 progress made this session
    pub resistance_encountered: f32,    // 0.0-1.0 resistance during session
    pub insights_gained: Vec<String>,   // Insights from this dialogue
    pub follow_up_actions: Vec<String>, // Actions to take after session
}

#[derive(Reflect, Clone, Debug, Serialize, Deserialize)]
pub struct RelationshipData {
    pub companion_entity: Entity,
    pub relationship_type: RelationshipType,
    pub relationship_quality: f32,      // 0.0-1.0 quality of relationship
    pub support_capacity: f32,          // 0.0-1.0 ability to provide support
    pub conflict_level: f32,            // 0.0-1.0 current conflict level
    pub shared_experiences: Vec<String>, // Shared traumatic/healing experiences
    pub mutual_understanding: f32,      // 0.0-1.0 how well they understand each other
}

#[derive(Reflect, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum RelationshipType {
    PlatonicFriend,     // Close platonic friendship
    RomanticPartner,    // Romantic relationship
    Mentor,             // Mentoring relationship
    Mentee,             // Being mentored
    SupportBuddy,       // Mutual support partnership
    ConflictualRelationship, // Relationship with significant conflict
    DistantRelationship, // Minimal interaction
}

#[derive(Reflect, Clone, Debug, Serialize, Deserialize)]
pub struct SupportData {
    pub support_type: SupportType,
    pub effectiveness: f32,             // 0.0-1.0 how effective support is
    pub frequency: f32,                 // How often support is provided/received
    pub emotional_cost: f32,            // Cost to supporter
    pub healing_benefit: f32,           // Benefit to recipient
}

#[derive(Reflect, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum SupportType {
    EmotionalSupport,   // Emotional comfort and validation
    PracticalSupport,   // Practical help and assistance
    InformationalSupport, // Information and guidance
    SocialSupport,      // Social connection and inclusion
    SpiritualSupport,   // Spiritual/meaning-based support
    AdvocacySupport,    // Advocacy and protection
}

// ============================================================================
// PSYCHOLOGY RESOURCES
// ============================================================================

/// Global psychology system state
#[derive(Resource, Reflect, Clone, Debug, Serialize, Deserialize)]
#[reflect(Resource)]
pub struct GlobalPsychologyState {
    // Overall companion mental health
    pub average_companion_trauma: f32,  // Average trauma across all companions
    pub companions_in_therapy: u32,     // Number currently in therapy
    pub companions_recovered: u32,      // Number who have recovered
    pub companions_broken: u32,         // Number who have broken down
    
    // Group dynamics
    pub group_cohesion: f32,            // 0.0-1.0 how well group supports each other
    pub collective_resilience: f32,     // 0.0-1.0 group resilience to trauma
    pub shared_trauma_processing: f32,  // 0.0-1.0 how well group processes shared trauma
    pub group_therapy_effectiveness: f32, // 0.0-1.0 effectiveness of group therapy
    
    // Professional support availability
    pub professional_support_available: bool, // Are professionals accessible?
    pub professional_support_quality: f32, // 0.0-1.0 quality of available help
    pub therapy_resources_abundance: f32, // 0.0-1.0 abundance of therapy resources
    
    // Healing environment
    pub healing_environment_quality: f32, // 0.0-1.0 how conducive environment is to healing
    pub safety_level: f32,              // 0.0-1.0 how safe companions feel
    pub social_support_availability: f32, // 0.0-1.0 availability of social support
}

impl Default for GlobalPsychologyState {
    fn default() -> Self {
        Self {
            average_companion_trauma: 0.0,
            companions_in_therapy: 0,
            companions_recovered: 0,
            companions_broken: 0,
            group_cohesion: 0.5,
            collective_resilience: 0.5,
            shared_trauma_processing: 0.3,
            group_therapy_effectiveness: 0.4,
            professional_support_available: false, // Starts unavailable
            professional_support_quality: 0.0,
            therapy_resources_abundance: 0.2,
            healing_environment_quality: 0.6,
            safety_level: 0.7,
            social_support_availability: 0.5,
        }
    }
}

/// Registry of therapy quest templates and progression
#[derive(Resource, Reflect, Clone, Debug, Serialize, Deserialize)]
#[reflect(Resource)]
pub struct TherapyQuestRegistry {
    // Available therapy quests by trauma category
    pub trauma_specific_quests: HashMap<TraumaCategory, Vec<TherapyQuest>>,
    pub general_healing_quests: Vec<TherapyQuest>,
    pub group_therapy_quests: Vec<GroupTherapyQuest>,
    
    // Quest progression tracking
    pub active_quests: HashMap<Entity, Vec<String>>, // Active quests per companion
    pub completed_quests: HashMap<Entity, Vec<String>>, // Completed quests per companion
    pub available_quests: HashMap<Entity, Vec<String>>, // Available quests per companion
    
    // Therapeutic dialogue trees
    pub dialogue_trees: HashMap<String, TherapeuticDialogueTree>,
    pub breakthrough_dialogues: HashMap<String, Vec<String>>, // Breakthrough dialogue options
    pub resistance_dialogues: HashMap<String, Vec<String>>, // Handling resistance
}

#[derive(Reflect, Clone, Debug, Serialize, Deserialize)]
pub struct TherapyQuest {
    pub quest_id: String,
    pub quest_name: String,
    pub quest_description: String,
    pub target_trauma_category: TraumaCategory,
    pub therapeutic_approach: String,   // Therapy technique used
    pub required_therapeutic_alliance: f32, // Minimum alliance to start
    pub expected_duration_sessions: u32, // Expected number of sessions
    pub healing_potential: f32,         // 0.0-1.0 potential healing from quest
    pub complexity_level: f32,          // 0.0-1.0 complexity of quest
    pub prerequisite_quests: Vec<String>, // Quests that must be completed first
    pub dialogue_requirements: Vec<String>, // Required dialogue interactions
}

#[derive(Reflect, Clone, Debug, Serialize, Deserialize)]
pub struct GroupTherapyQuest {
    pub quest_id: String,
    pub participants_required: Vec<Entity>, // Which companions must participate
    pub group_dynamic_focus: String,    // Focus of group therapy
    pub shared_trauma_category: Option<TraumaCategory>, // Shared trauma being addressed
    pub group_healing_potential: f32,   // 0.0-1.0 potential group healing
    pub individual_benefits: HashMap<Entity, f32>, // Individual benefits per companion
}

#[derive(Reflect, Clone, Debug, Serialize, Deserialize)]
pub struct TherapeuticDialogueTree {
    pub tree_id: String,
    pub therapeutic_technique: String,
    pub dialogue_branches: HashMap<String, DialogueBranch>,
    pub breakthrough_conditions: Vec<String>, // Conditions for breakthrough
    pub resistance_handling: HashMap<String, String>, // How to handle resistance
    pub healing_progressions: HashMap<String, f32>, // Healing per branch
}

#[derive(Reflect, Clone, Debug, Serialize, Deserialize)]
pub struct DialogueBranch {
    pub branch_id: String,
    pub therapeutic_content: String,    // The therapeutic dialogue content
    pub emotional_tone_required: String, // Required emotional state
    pub healing_impact: f32,            // 0.0-1.0 healing impact of this branch
    pub resistance_probability: f32,    // 0.0-1.0 probability of resistance
    pub breakthrough_potential: f32,    // 0.0-1.0 potential for breakthrough
    pub follow_up_branches: Vec<String>, // Possible next branches
}

/// Resource tracking companion relationship network
#[derive(Resource, Reflect, Clone, Debug, Serialize, Deserialize)]
#[reflect(Resource)]
pub struct CompanionRelationshipNetwork {
    // Network topology
    pub relationship_matrix: HashMap<(Entity, Entity), f32>, // Relationship strengths
    pub support_network_strength: f32,  // 0.0-1.0 overall network strength
    pub network_resilience: f32,        // 0.0-1.0 network's trauma resilience
    
    // Support flow tracking
    pub active_support_flows: Vec<SupportFlow>, // Currently active support
    pub support_effectiveness_scores: HashMap<Entity, f32>, // Support effectiveness per companion
    pub support_needs: HashMap<Entity, Vec<SupportType>>, // Support needs per companion
    
    // Group dynamics
    pub group_therapy_sessions: Vec<GroupTherapySession>, // Past group sessions
    pub group_conflict_level: f32,      // 0.0-1.0 current group conflict
    pub collective_healing_progress: f32, // 0.0-1.0 group healing progress
}

#[derive(Reflect, Clone, Debug, Serialize, Deserialize)]
pub struct SupportFlow {
    pub supporter: Entity,
    pub recipient: Entity,
    pub support_type: SupportType,
    pub intensity: f32,                 // 0.0-1.0 intensity of support
    pub duration: f32,                  // How long support has been active
    pub effectiveness: f32,             // 0.0-1.0 how effective support is
    pub emotional_cost_to_supporter: f32, // Cost to person providing support
}

#[derive(Reflect, Clone, Debug, Serialize, Deserialize)]
pub struct GroupTherapySession {
    pub session_id: String,
    pub participants: Vec<Entity>,
    pub session_focus: String,          // Focus of this group session
    pub facilitation_quality: f32,     // 0.0-1.0 quality of facilitation
    pub group_participation: HashMap<Entity, f32>, // Participation per companion
    pub breakthroughs_achieved: Vec<String>, // Breakthroughs in this session
    pub conflicts_resolved: Vec<String>, // Conflicts resolved
    pub healing_outcomes: HashMap<Entity, f32>, // Healing per participant
}

impl Default for CompanionRelationshipNetwork {
    fn default() -> Self {
        Self {
            relationship_matrix: HashMap::new(),
            support_network_strength: 0.3,
            network_resilience: 0.4,
            active_support_flows: Vec::new(),
            support_effectiveness_scores: HashMap::new(),
            support_needs: HashMap::new(),
            group_therapy_sessions: Vec::new(),
            group_conflict_level: 0.2,
            collective_healing_progress: 0.0,
        }
    }
}

// ============================================================================
// PSYCHOLOGY EVENTS
// ============================================================================

#[derive(Event, Reflect, Clone, Debug)]
#[reflect(Default)]
pub struct TraumaEvent {
    pub companion_entity: Entity,
    pub trauma_source: String,          // What caused the trauma
    pub trauma_category: TraumaCategory,
    pub trauma_severity: f32,           // 0.0-1.0 severity of traumatic event
    pub witnesses: Vec<Entity>,         // Other companions who witnessed
    pub immediate_impact: f32,          // Immediate trauma increase
    pub long_term_impact: f32,          // Long-term trauma accumulation
    pub triggers_breakdown_risk: bool,  // Does this risk immediate breakdown?
}

impl Default for TraumaEvent {
    fn default() -> Self {
        Self {
            companion_entity: Entity::PLACEHOLDER,
            trauma_source: String::new(),
            trauma_category: TraumaCategory::CombatTrauma,
            trauma_severity: 0.0,
            witnesses: Vec::new(),
            immediate_impact: 0.0,
            long_term_impact: 0.0,
            triggers_breakdown_risk: false,
        }
    }
}

#[derive(Event, Reflect, Clone, Debug)]
#[reflect(Default)]
pub struct TherapySessionEvent {
    pub companion_entity: Entity,
    pub session_type: String,           // Type of therapy session
    pub therapeutic_technique: String,  // Specific technique used
    pub session_quality: f32,           // 0.0-1.0 quality of session
    pub breakthrough_achieved: bool,    // Was breakthrough achieved?
    pub healing_progress: f32,          // 0.0-1.0 healing progress from session
    pub resistance_encountered: f32,    // 0.0-1.0 resistance during session
    pub follow_up_required: bool,       // Does this session require follow-up?
    pub insights_gained: Vec<String>,   // Insights from session
}

impl Default for TherapySessionEvent {
    fn default() -> Self {
        Self {
            companion_entity: Entity::PLACEHOLDER,
            session_type: String::new(),
            therapeutic_technique: String::new(),
            session_quality: 0.0,
            breakthrough_achieved: false,
            healing_progress: 0.0,
            resistance_encountered: 0.0,
            follow_up_required: false,
            insights_gained: Vec::new(),
        }
    }
}

#[derive(Event, Reflect, Clone, Debug)]
#[reflect(Default)]
pub struct CompanionBreakdownEvent {
    pub companion_entity: Entity,
    pub breakdown_trigger: String,      // What triggered the breakdown
    pub breakdown_severity: BreakdownSeverity,
    pub breakdown_type: BreakdownType,
    pub immediate_consequences: Vec<String>, // Immediate effects
    pub recovery_requirements: Vec<String>, // What's needed for recovery
    pub support_needed: Vec<SupportType>, // Types of support needed
    pub professional_intervention_required: bool,
}

#[derive(Reflect, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum BreakdownSeverity {
    Mild,       // Temporary distress, recovers quickly
    Moderate,   // Significant impact, requires support
    Severe,     // Major breakdown, requires intervention
    Critical,   // Life-threatening, requires immediate professional help
}

#[derive(Reflect, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum BreakdownType {
    EmotionalBreakdown,     // Emotional overwhelm
    CognitiveBreakdown,     // Cognitive dysfunction
    BehavioralBreakdown,    // Behavioral dysregulation
    SocialBreakdown,        // Social withdrawal
    PhysicalBreakdown,      // Physical symptoms
    SpiritualBreakdown,     // Loss of meaning/purpose
}

impl Default for CompanionBreakdownEvent {
    fn default() -> Self {
        Self {
            companion_entity: Entity::PLACEHOLDER,
            breakdown_trigger: String::new(),
            breakdown_severity: BreakdownSeverity::Mild,
            breakdown_type: BreakdownType::EmotionalBreakdown,
            immediate_consequences: Vec::new(),
            recovery_requirements: Vec::new(),
            support_needed: Vec::new(),
            professional_intervention_required: false,
        }
    }
}

#[derive(Event, Reflect, Clone, Debug)]
#[reflect(Default)]
pub struct RecoveryMilestoneEvent {
    pub companion_entity: Entity,
    pub milestone_type: RecoveryMilestoneType,
    pub healing_progress_achieved: f32, // 0.0-1.0 healing progress
    pub trauma_reduction: f32,          // Reduction in trauma level
    pub new_coping_skills: Vec<String>, // New coping skills learned
    pub relationship_improvements: Vec<Entity>, // Relationships improved
    pub celebration_worthy: bool,       // Should this be celebrated?
}

#[derive(Reflect, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum RecoveryMilestoneType {
    FirstTherapySession,    // Completed first therapy session
    TraumaProcessed,        // Successfully processed specific trauma
    CopingSkillLearned,     // Learned new coping skill
    RelationshipRepaired,   // Repaired damaged relationship
    SupportNetworkBuilt,    // Built strong support network
    PostTraumaticGrowth,    // Achieved growth beyond baseline
    TherapyCompletion,      // Completed therapy program
    LongTermStability,      // Achieved long-term stability
}

impl Default for RecoveryMilestoneEvent {
    fn default() -> Self {
        Self {
            companion_entity: Entity::PLACEHOLDER,
            milestone_type: RecoveryMilestoneType::FirstTherapySession,
            healing_progress_achieved: 0.0,
            trauma_reduction: 0.0,
            new_coping_skills: Vec::new(),
            relationship_improvements: Vec::new(),
            celebration_worthy: false,
        }
    }
}

#[derive(Event, Reflect, Clone, Debug)]
#[reflect(Default)]
pub struct RelationshipChangeEvent {
    pub companion_a: Entity,
    pub companion_b: Entity,
    pub relationship_change: RelationshipChange,
    pub change_magnitude: f32,          // How significant the change is
    pub change_reason: String,          // What caused the change
    pub impact_on_support: f32,         // How this affects mutual support
    pub impact_on_group: f32,           // How this affects group dynamics
}

#[derive(Reflect, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum RelationshipChange {
    Strengthened,       // Relationship became stronger
    Weakened,           // Relationship became weaker
    TypeChanged,        // Relationship type changed
    ConflictResolved,   // Previous conflict was resolved
    ConflictEmerged,    // New conflict emerged
    SupportIncreased,   // Support capacity increased
    SupportDecreased,   // Support capacity decreased
}

impl Default for RelationshipChangeEvent {
    fn default() -> Self {
        Self {
            companion_a: Entity::PLACEHOLDER,
            companion_b: Entity::PLACEHOLDER,
            relationship_change: RelationshipChange::Strengthened,
            change_magnitude: 0.0,
            change_reason: String::new(),
            impact_on_support: 0.0,
            impact_on_group: 0.0,
        }
    }
}

#[derive(Event, Reflect, Clone, Debug)]
#[reflect(Default)]
pub struct TherapeuticBreakthroughEvent {
    pub companion_entity: Entity,
    pub breakthrough_type: BreakthroughType,
    pub therapy_technique_used: String,
    pub trauma_category_addressed: TraumaCategory,
    pub healing_magnitude: f32,         // 0.0-1.0 magnitude of breakthrough
    pub insights_gained: Vec<String>,   // Specific insights from breakthrough
    pub new_coping_skills: Vec<String>, // New coping skills developed
    pub relationship_impacts: Vec<(Entity, f32)>, // Impact on relationships
    pub long_term_implications: Vec<String>, // Long-term effects
}

#[derive(Reflect, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum BreakthroughType {
    MemoryProcessing,   // Successfully processed traumatic memory
    EmotionalRelease,   // Released suppressed emotions
    CognitiveReframe,   // Reframed traumatic experience
    SelfCompassion,     // Developed self-compassion
    MeaningMaking,      // Found meaning in trauma
    ForgivenessSelf,    // Forgave themselves
    ForgivenessOther,   // Forgave others
    IdentityIntegration, // Integrated trauma into identity
}

impl Default for TherapeuticBreakthroughEvent {
    fn default() -> Self {
        Self {
            companion_entity: Entity::PLACEHOLDER,
            breakthrough_type: BreakthroughType::EmotionalRelease,
            therapy_technique_used: String::new(),
            trauma_category_addressed: TraumaCategory::CombatTrauma,
            healing_magnitude: 0.0,
            insights_gained: Vec::new(),
            new_coping_skills: Vec::new(),
            relationship_impacts: Vec::new(),
            long_term_implications: Vec::new(),
        }
    }
}

// ============================================================================
// PSYCHOLOGY SYSTEM SETUP
// ============================================================================

fn setup_psychology_system(
    mut commands: Commands,
    mut therapy_registry: ResMut<TherapyQuestRegistry>,
) {
    info!("Initializing Sophisticated Companion Psychology System");
    
    // Initialize therapy quest templates for each trauma category
    initialize_therapy_quests(&mut therapy_registry);
    
    // Set up therapeutic dialogue trees
    initialize_therapeutic_dialogues(&mut therapy_registry);
    
    info!("Psychology system initialized - therapy quests ready");
}

fn initialize_therapy_quests(registry: &mut TherapyQuestRegistry) {
    // Combat trauma therapy quests
    let combat_trauma_quests = vec![
        TherapyQuest {
            quest_id: "combat_trauma_processing".to_string(),
            quest_name: "Processing Combat Trauma".to_string(),
            quest_description: "Work through traumatic combat experiences with specialized techniques".to_string(),
            target_trauma_category: TraumaCategory::CombatTrauma,
            therapeutic_approach: "Trauma-Focused Cognitive Behavioral Therapy".to_string(),
            required_therapeutic_alliance: 0.6,
            expected_duration_sessions: 8,
            healing_potential: 0.7,
            complexity_level: 0.8,
            prerequisite_quests: vec!["basic_stabilization".to_string()],
            dialogue_requirements: vec!["combat_trauma_assessment".to_string()],
        },
        TherapyQuest {
            quest_id: "combat_exposure_therapy".to_string(),
            quest_name: "Gradual Combat Re-exposure".to_string(),
            quest_description: "Gradually re-expose to combat situations in controlled way".to_string(),
            target_trauma_category: TraumaCategory::CombatTrauma,
            therapeutic_approach: "Exposure Therapy".to_string(),
            required_therapeutic_alliance: 0.8,
            expected_duration_sessions: 12,
            healing_potential: 0.9,
            complexity_level: 0.9,
            prerequisite_quests: vec!["combat_trauma_processing".to_string()],
            dialogue_requirements: vec!["combat_readiness_assessment".to_string()],
        },
    ];
    
    registry.trauma_specific_quests.insert(TraumaCategory::CombatTrauma, combat_trauma_quests);
    
    // Loss trauma therapy quests
    let loss_trauma_quests = vec![
        TherapyQuest {
            quest_id: "grief_processing".to_string(),
            quest_name: "Processing Grief and Loss".to_string(),
            quest_description: "Work through grief using evidence-based grief therapy".to_string(),
            target_trauma_category: TraumaCategory::LossTrauma,
            therapeutic_approach: "Grief Therapy".to_string(),
            required_therapeutic_alliance: 0.5,
            expected_duration_sessions: 10,
            healing_potential: 0.8,
            complexity_level: 0.7,
            prerequisite_quests: vec![],
            dialogue_requirements: vec!["loss_narrative_creation".to_string()],
        },
    ];
    
    registry.trauma_specific_quests.insert(TraumaCategory::LossTrauma, loss_trauma_quests);
    
    // Add other trauma category quests...
    // TODO: Add betrayal, witness, abandonment, corruption, moral, identity trauma quests
}

fn initialize_therapeutic_dialogues(registry: &mut TherapyQuestRegistry) {
    // Create therapeutic dialogue trees for different techniques
    let cognitive_reprocessing_tree = TherapeuticDialogueTree {
        tree_id: "cognitive_reprocessing".to_string(),
        therapeutic_technique: "Cognitive Processing Therapy".to_string(),
        dialogue_branches: HashMap::from([
            ("initial_assessment".to_string(), DialogueBranch {
                branch_id: "initial_assessment".to_string(),
                therapeutic_content: "Let's start by understanding what happened and how it affected you.".to_string(),
                emotional_tone_required: "calm_supportive".to_string(),
                healing_impact: 0.1,
                resistance_probability: 0.3,
                breakthrough_potential: 0.2,
                follow_up_branches: vec!["memory_exploration".to_string()],
            }),
            ("memory_exploration".to_string(), DialogueBranch {
                branch_id: "memory_exploration".to_string(),
                therapeutic_content: "Can you tell me about that memory? Take your time.".to_string(),
                emotional_tone_required: "gentle_encouraging".to_string(),
                healing_impact: 0.3,
                resistance_probability: 0.6,
                breakthrough_potential: 0.5,
                follow_up_branches: vec!["cognitive_restructuring".to_string()],
            }),
        ]),
        breakthrough_conditions: vec![
            "emotional_openness > 0.7".to_string(),
            "therapeutic_alliance > 0.6".to_string(),
            "resistance < 0.4".to_string(),
        ],
        resistance_handling: HashMap::from([
            ("high_resistance".to_string(), "Let's slow down and focus on feeling safe first.".to_string()),
            ("moderate_resistance".to_string(), "It's okay to feel hesitant. We can take this at your pace.".to_string()),
        ]),
        healing_progressions: HashMap::from([
            ("breakthrough_path".to_string(), 0.8),
            ("gradual_path".to_string(), 0.4),
            ("resistance_path".to_string(), 0.1),
        ]),
    };
    
    registry.dialogue_trees.insert("cognitive_reprocessing".to_string(), cognitive_reprocessing_tree);
    
    // TODO: Add more therapeutic dialogue trees
}

// ============================================================================
// CORE PSYCHOLOGY SYSTEMS
// ============================================================================

/// Trauma accumulation system with sophisticated algorithms
fn trauma_accumulation_system(
    mut trauma_events: EventReader<TraumaEvent>,
    mut companions: Query<(Entity, &mut CompanionTrauma)>,
    mut global_state: ResMut<GlobalPsychologyState>,
) {
    for event in trauma_events.read() {
        if let Ok((entity, mut trauma)) = companions.get_mut(event.companion_entity) {
            // Calculate trauma impact based on current state and resilience
            let resilience_factor = 1.0 - trauma.resilience_score;
            let current_vulnerability = trauma.trauma_level / 5.0; // Normalize to 0-1
            let accumulated_trauma_factor = 1.0 + (current_vulnerability * 0.5); // More trauma = more vulnerable
            
            let total_trauma_impact = event.immediate_impact * resilience_factor * accumulated_trauma_factor;
            
            // Apply trauma with category-specific processing
            if let Some(category_trauma) = trauma.trauma_categories.get_mut(&event.trauma_category) {
                *category_trauma += total_trauma_impact;
            } else {
                trauma.trauma_categories.insert(event.trauma_category.clone(), total_trauma_impact);
            }
            
            // Update overall trauma level
            trauma.trauma_level = trauma.trauma_categories.values().sum::<f32>().min(5.0);
            
            // Update stability based on trauma level
            trauma.current_stability = ((5.0 - trauma.trauma_level) / 5.0).max(0.0);
            
            // Check for breakdown risk
            if trauma.trauma_level >= trauma.breaking_point || event.triggers_breakdown_risk {
                // TODO: Trigger breakdown event
                warn!("Companion {} at breakdown risk (trauma: {:.2}/{:.2})", 
                      entity.index(), trauma.trauma_level, trauma.breaking_point);
            }
            
            // Update global state
            let total_companions = companions.iter().count() as f32;
            global_state.average_companion_trauma = companions.iter()
                .map(|(_, trauma)| trauma.trauma_level)
                .sum::<f32>() / total_companions;
            
            info!("Trauma accumulated for companion {}: +{:.2} (total: {:.2}/5.0)", 
                  entity.index(), total_trauma_impact, trauma.trauma_level);
        }
    }
}

/// Detect and respond to trauma triggers in environment
fn trauma_trigger_detection_system(
    companions: Query<(Entity, &CompanionTrauma)>,
    mut trauma_events: EventWriter<TraumaEvent>,
    // TODO: Add queries for environmental triggers, player actions, etc.
) {
    for (entity, trauma) in companions.iter() {
        // Check each trauma trigger for activation
        for trigger in &trauma.trauma_triggers {
            // TODO: Check if trigger conditions are met in current environment
            let trigger_activated = false; // Placeholder
            
            if trigger_activated {
                let trauma_impact = trigger.trauma_impact * trigger.sensitivity_level;
                
                trauma_events.send(TraumaEvent {
                    companion_entity: entity,
                    trauma_source: trigger.trigger_name.clone(),
                    trauma_category: trigger.trigger_category.clone(),
                    trauma_severity: trigger.sensitivity_level,
                    witnesses: vec![], // TODO: Get nearby companions
                    immediate_impact: trauma_impact,
                    long_term_impact: trauma_impact * 0.5,
                    triggers_breakdown_risk: trauma_impact > 0.8,
                });
                
                info!("Trauma trigger activated: {} for companion {}", 
                      trigger.trigger_name, entity.index());
            }
        }
    }
}

/// Progress trauma over time with natural recovery
fn trauma_progression_system(
    mut companions: Query<&mut CompanionTrauma>,
    time: Res<Time>,
) {
    for mut trauma in companions.iter_mut() {
        // Natural recovery over time (if not being re-traumatized)
        let recovery_rate = trauma.trauma_recovery_rate * time.delta_seconds();
        
        // Apply recovery to each trauma category
        for category_trauma in trauma.trauma_categories.values_mut() {
            *category_trauma = (*category_trauma - recovery_rate).max(0.0);
        }
        
        // Update overall trauma level
        trauma.trauma_level = trauma.trauma_categories.values().sum::<f32>().min(5.0);
        
        // Update stability
        trauma.current_stability = ((5.0 - trauma.trauma_level) / 5.0).max(0.0);
    }
}

/// Update visual representation based on trauma level
fn trauma_visual_update_system(
    mut companions: Query<(&mut CompanionTrauma, &mut Transform), Changed<CompanionTrauma>>,
    // TODO: Add visual asset queries for trauma states
) {
    for (mut trauma, mut transform) in companions.iter_mut() {
        // Update visual trauma level based on current trauma
        let new_visual_level = (trauma.trauma_level as u32).min(4);
        
        if trauma.trauma_visual_level != new_visual_level {
            trauma.trauma_visual_level = new_visual_level;
            
            // TODO: Update visual assets, animations, materials based on trauma level
            info!("Updating companion visual trauma level to {}", new_visual_level);
        }
    }
}

/// Therapy quest progression system
fn therapy_quest_progression_system(
    mut companions: Query<(Entity, &mut TherapyProgression)>,
    mut therapy_events: EventWriter<TherapySessionEvent>,
    therapy_registry: Res<TherapyQuestRegistry>,
) {
    for (entity, mut therapy) in companions.iter_mut() {
        if !therapy.therapy_quest_id.is_empty() {
            // Progress current therapy quest
            let session_progress = 0.1; // TODO: Calculate based on actual therapy interaction
            therapy.therapy_progress += session_progress;
            therapy.session_count += 1;
            
            // Check for therapy completion
            if therapy.therapy_progress >= 1.0 {
                therapy_events.send(TherapySessionEvent {
                    companion_entity: entity,
                    session_type: "quest_completion".to_string(),
                    therapeutic_technique: therapy.therapy_quest_id.clone(),
                    session_quality: 0.8, // TODO: Calculate based on actual performance
                    breakthrough_achieved: true,
                    healing_progress: 0.2,
                    resistance_encountered: 0.1,
                    follow_up_required: false,
                    insights_gained: vec!["Completed therapy quest successfully".to_string()],
                });
                
                info!("Therapy quest completed for companion {}: {}", 
                      entity.index(), therapy.therapy_quest_id);
                
                // Reset for next quest
                therapy.therapy_quest_id.clear();
                therapy.therapy_progress = 0.0;
                therapy.therapy_stage = TherapyStage::Assessment;
            }
        }
    }
}

/// Therapeutic dialogue system with breakthrough detection
fn therapeutic_dialogue_system(
    mut therapy_sessions: EventReader<TherapySessionEvent>,
    mut breakthrough_events: EventWriter<TherapeuticBreakthroughEvent>,
    mut companions: Query<&mut TherapyProgression>,
) {
    for session in therapy_sessions.read() {
        if let Ok(mut therapy) = companions.get_mut(session.companion_entity) {
            // Record therapeutic dialogue
            let dialogue = TherapeuticDialogue {
                dialogue_id: format!("session_{}", therapy.session_count),
                therapeutic_technique: session.therapeutic_technique.clone(),
                emotional_tone: "supportive".to_string(), // TODO: Detect from actual dialogue
                breakthrough_achieved: session.breakthrough_achieved,
                healing_progress_made: session.healing_progress,
                resistance_encountered: session.resistance_encountered,
                insights_gained: session.insights_gained.clone(),
                follow_up_actions: vec![], // TODO: Generate follow-up actions
            };
            
            therapy.therapeutic_conversation_history.push(dialogue);
            
            // Check for breakthrough conditions
            if session.breakthrough_achieved {
                breakthrough_events.send(TherapeuticBreakthroughEvent {
                    companion_entity: session.companion_entity,
                    breakthrough_type: BreakthroughType::EmotionalRelease, // TODO: Determine type
                    therapy_technique_used: session.therapeutic_technique.clone(),
                    trauma_category_addressed: TraumaCategory::CombatTrauma, // TODO: Determine category
                    healing_magnitude: session.healing_progress,
                    insights_gained: session.insights_gained.clone(),
                    new_coping_skills: vec![], // TODO: Extract coping skills
                    relationship_impacts: vec![], // TODO: Calculate relationship impacts
                    long_term_implications: vec![], // TODO: Assess long-term effects
                });
            }
        }
    }
}

/// Healing mission system for complex therapeutic work
fn healing_mission_system(
    mut companions: Query<(Entity, &mut TherapyProgression, &CompanionTrauma)>,
    mut recovery_events: EventWriter<RecoveryMilestoneEvent>,
) {
    for (entity, mut therapy, trauma) in companions.iter_mut() {
        // Process active healing missions
        for mission in &mut therapy.active_healing_missions {
            // TODO: Progress mission based on companion engagement and readiness
            let mission_progress = 0.1; // Placeholder
            
            if mission_progress >= 1.0 {
                // Mission completed - calculate healing outcomes
                let healing_achieved = mission.expected_healing_outcome;
                
                recovery_events.send(RecoveryMilestoneEvent {
                    companion_entity: entity,
                    milestone_type: RecoveryMilestoneType::TraumaProcessed,
                    healing_progress_achieved: healing_achieved,
                    trauma_reduction: healing_achieved * 0.5,
                    new_coping_skills: vec![format!("Skill from {}", mission.mission_id)],
                    relationship_improvements: vec![],
                    celebration_worthy: healing_achieved > 0.5,
                });
                
                therapy.completed_healing_missions.push(mission.mission_id.clone());
                info!("Healing mission completed: {} for companion {}", 
                      mission.mission_id, entity.index());
            }
        }
        
        // Remove completed missions
        therapy.active_healing_missions.retain(|mission| {
            !therapy.completed_healing_missions.contains(&mission.mission_id)
        });
    }
}

/// Breakthrough detection system
fn breakthrough_detection_system(
    mut breakthrough_events: EventReader<TherapeuticBreakthroughEvent>,
    mut companions: Query<(Entity, &mut CompanionTrauma, &mut TherapyProgression)>,
    mut recovery_events: EventWriter<RecoveryMilestoneEvent>,
) {
    for breakthrough in breakthrough_events.read() {
        if let Ok((entity, mut trauma, mut therapy)) = companions.get_mut(breakthrough.companion_entity) {
            // Apply breakthrough healing
            let healing_amount = breakthrough.healing_magnitude;
            
            // Reduce trauma in relevant category
            for (category, category_trauma) in trauma.trauma_categories.iter_mut() {
                if *category == breakthrough.trauma_category_addressed {
                    *category_trauma = (*category_trauma - healing_amount).max(0.0);
                    break;
                }
            }
            
            // Update overall trauma level
            trauma.trauma_level = trauma.trauma_categories.values().sum::<f32>().min(5.0);
            
            // Record insights and coping skills
            therapy.therapeutic_insights.extend(breakthrough.insights_gained.clone());
            
            // Add new coping mechanisms
            for skill in &breakthrough.new_coping_skills {
                trauma.coping_mechanisms.push(CopingMechanism::HealthyCoping(skill.clone()));
            }
            
            info!("Therapeutic breakthrough processed for companion {}: {:?}", 
                  entity.index(), breakthrough.breakthrough_type);
        }
    }
}

/// Inter-companion support system
fn inter_companion_support_system(
    companions: Query<(Entity, &CompanionTrauma, &CompanionRelationships)>,
    mut relationship_network: ResMut<CompanionRelationshipNetwork>,
    mut relationship_events: EventWriter<RelationshipChangeEvent>,
) {
    // Process active support flows
    for support_flow in &mut relationship_network.active_support_flows {
        if let (Ok((_, trauma_supporter, relationships_supporter)), 
               Ok((_, trauma_recipient, _))) = 
            (companions.get(support_flow.supporter), companions.get(support_flow.recipient)) {
            
            // Calculate support effectiveness based on multiple factors
            let supporter_capacity = 1.0 - (trauma_supporter.trauma_level / 5.0); // Less trauma = more capacity
            let relationship_quality = relationships_supporter.relationships
                .get(&support_flow.recipient)
                .map(|rel| rel.relationship_quality)
                .unwrap_or(0.5);
            
            let base_effectiveness = supporter_capacity * relationship_quality;
            support_flow.effectiveness = base_effectiveness;
            
            // Calculate emotional cost to supporter
            support_flow.emotional_cost_to_supporter = (1.0 - supporter_capacity) * support_flow.intensity;
            
            // TODO: Apply healing benefits to recipient
            // TODO: Apply emotional costs to supporter
            
            // Check if support flow should strengthen relationship
            if support_flow.effectiveness > 0.7 && support_flow.duration > 5.0 {
                relationship_events.send(RelationshipChangeEvent {
                    companion_a: support_flow.supporter,
                    companion_b: support_flow.recipient,
                    relationship_change: RelationshipChange::Strengthened,
                    change_magnitude: 0.1,
                    change_reason: "Effective support provided".to_string(),
                    impact_on_support: 0.1,
                    impact_on_group: 0.05,
                });
            }
        }
    }
}

/// Relationship development system
fn relationship_development_system(
    mut relationship_events: EventReader<RelationshipChangeEvent>,
    mut companions: Query<&mut CompanionRelationships>,
    mut network: ResMut<CompanionRelationshipNetwork>,
) {
    for event in relationship_events.read() {
        // Update relationship matrix
        let relationship_key = (event.companion_a, event.companion_b);
        let current_strength = network.relationship_matrix.get(&relationship_key).unwrap_or(&0.5);
        
        let new_strength = match event.relationship_change {
            RelationshipChange::Strengthened => (current_strength + event.change_magnitude).min(1.0),
            RelationshipChange::Weakened => (current_strength - event.change_magnitude).max(0.0),
            _ => *current_strength,
        };
        
        network.relationship_matrix.insert(relationship_key, new_strength);
        
        // Update individual relationship components
        if let Ok(mut relationships_a) = companions.get_mut(event.companion_a) {
            if let Some(rel_data) = relationships_a.relationships.get_mut(&event.companion_b) {
                rel_data.relationship_quality = new_strength;
            }
        }
        
        info!("Relationship updated between companions: {:.2} -> {:.2}", 
              current_strength, new_strength);
    }
}

/// Group therapy system for multi-companion healing
fn group_therapy_system(
    companions: Query<(Entity, &CompanionTrauma, &TherapyProgression)>,
    mut network: ResMut<CompanionRelationshipNetwork>,
    mut therapy_events: EventWriter<TherapySessionEvent>,
) {
    // TODO: Implement group therapy session logic
    // This should facilitate healing between multiple companions simultaneously
    info!("Group therapy system running");
}

/// Peer support effectiveness system
fn peer_support_effectiveness_system(
    companions: Query<(Entity, &CompanionRelationships)>,
    mut network: ResMut<CompanionRelationshipNetwork>,
) {
    // Calculate peer support effectiveness for each companion
    for (entity, relationships) in companions.iter() {
        let support_effectiveness = relationships.peer_support_effectiveness;
        network.support_effectiveness_scores.insert(entity, support_effectiveness);
    }
}

/// Companion breakdown system
fn companion_breakdown_system(
    mut breakdown_events: EventReader<CompanionBreakdownEvent>,
    mut companions: Query<&mut CompanionTrauma>,
    mut global_state: ResMut<GlobalPsychologyState>,
) {
    for event in breakdown_events.read() {
        if let Ok(mut trauma) = companions.get_mut(event.companion_entity) {
            // Handle breakdown consequences
            match event.breakdown_severity {
                BreakdownSeverity::Critical => {
                    trauma.trauma_level = 5.0; // Maximum trauma
                    trauma.current_stability = 0.0;
                    global_state.companions_broken += 1;
                },
                BreakdownSeverity::Severe => {
                    trauma.trauma_level += 1.0;
                    trauma.current_stability *= 0.5;
                },
                BreakdownSeverity::Moderate => {
                    trauma.trauma_level += 0.5;
                    trauma.current_stability *= 0.7;
                },
                BreakdownSeverity::Mild => {
                    trauma.current_stability *= 0.9;
                },
            }
            
            warn!("Companion breakdown: {:?} severity for companion {}", 
                  event.breakdown_severity, event.companion_entity.index());
        }
    }
}

/// Recovery tracking system
fn recovery_tracking_system(
    mut recovery_events: EventReader<RecoveryMilestoneEvent>,
    mut companions: Query<&mut CompanionTrauma>,
    mut global_state: ResMut<GlobalPsychologyState>,
) {
    for event in recovery_events.read() {
        if let Ok(mut trauma) = companions.get_mut(event.companion_entity) {
            // Apply recovery benefits
            trauma.trauma_level = (trauma.trauma_level - event.trauma_reduction).max(0.0);
            trauma.recovery_progress += event.healing_progress_achieved;
            trauma.recovery_milestones.push(format!("{:?}", event.milestone_type));
            
            // Check if this represents full recovery
            if trauma.trauma_level < 1.0 && trauma.recovery_progress > 0.8 {
                global_state.companions_recovered += 1;
                info!("Companion {} achieved significant recovery!", event.companion_entity.index());
            }
            
            info!("Recovery milestone achieved: {:?} for companion {}", 
                  event.milestone_type, event.companion_entity.index());
        }
    }
}

/// Setback detection system
fn setback_detection_system(
    mut companions: Query<&mut CompanionTrauma>,
    // TODO: Add queries for setback triggers
) {
    for mut trauma in companions.iter_mut() {
        // TODO: Detect conditions that could cause therapy setbacks
        // Examples: re-traumatization, relationship conflicts, environmental stressors
        
        let setback_detected = false; // Placeholder
        
        if setback_detected {
            trauma.setback_count += 1;
            trauma.recovery_progress *= 0.8; // Setback reduces progress
            trauma.current_stability *= 0.9;
            
            warn!("Therapy setback detected for companion (setbacks: {})", trauma.setback_count);
        }
    }
}

/// Resilience building system
fn resilience_building_system(
    mut companions: Query<&mut CompanionTrauma>,
    // TODO: Add queries for resilience-building activities
) {
    for mut trauma in companions.iter_mut() {
        // TODO: Implement resilience building through:
        // - Successful coping skill usage
        // - Positive relationship experiences  
        // - Therapy progress
        // - Support network strength
        
        let resilience_growth = 0.001; // Small gradual growth
        trauma.resilience_score = (trauma.resilience_score + resilience_growth).min(1.0);
    }
}

/// Professional support system
fn professional_support_system(
    companions: Query<(Entity, &ProfessionalSupport, &CompanionTrauma)>,
    mut therapy_events: EventWriter<TherapySessionEvent>,
) {
    for (entity, professional, trauma) in companions.iter() {
        if professional.has_professional_help {
            // Professional support provides higher quality therapy
            let session_quality = professional.support_quality * professional.professional_relationship;
            
            if session_quality > 0.6 {
                therapy_events.send(TherapySessionEvent {
                    companion_entity: entity,
                    session_type: format!("{:?}", professional.support_type),
                    therapeutic_technique: professional.treatment_approach.clone(),
                    session_quality,
                    breakthrough_achieved: session_quality > 0.8,
                    healing_progress: session_quality * 0.3,
                    resistance_encountered: (1.0 - session_quality) * 0.5,
                    follow_up_required: true,
                    insights_gained: professional.professional_assessments.clone(),
                });
            }
        }
    }
}

/// Therapy quality assessment system
fn therapy_quality_assessment_system(
    mut global_state: ResMut<GlobalPsychologyState>,
    companions: Query<&TherapyProgression>,
) {
    let companions_in_therapy = companions.iter()
        .filter(|therapy| !therapy.therapy_quest_id.is_empty())
        .count() as u32;
    
    global_state.companions_in_therapy = companions_in_therapy;
    
    // Calculate average therapy effectiveness
    let total_therapeutic_alliance: f32 = companions.iter()
        .map(|therapy| therapy.therapeutic_alliance)
        .sum();
    
    let companion_count = companions.iter().count() as f32;
    if companion_count > 0.0 {
        global_state.group_therapy_effectiveness = total_therapeutic_alliance / companion_count;
    }
}

/// Long-term healing trajectory system
fn long_term_healing_trajectory_system(
    companions: Query<&CompanionTrauma>,
    mut global_state: ResMut<GlobalPsychologyState>,
) {
    // Track long-term healing patterns across all companions
    let total_recovery_progress: f32 = companions.iter()
        .map(|trauma| trauma.recovery_progress)
        .sum();
    
    let companion_count = companions.iter().count() as f32;
    if companion_count > 0.0 {
        global_state.collective_healing_progress = total_recovery_progress / companion_count;
    }
    
    // Assess healing environment quality based on collective progress
    global_state.healing_environment_quality = (
        global_state.collective_healing_progress * 0.4 +
        global_state.group_cohesion * 0.3 +
        global_state.safety_level * 0.3
    ).min(1.0);
}
