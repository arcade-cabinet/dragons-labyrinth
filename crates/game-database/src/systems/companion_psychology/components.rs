//! Companion Psychology System - ECS Components
//!
//! Components for sophisticated companion trauma tracking, therapy quests,
//! and psychological authenticity in Dragon's Labyrinth.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Companion psychological state component
#[derive(Component, Reflect, Debug, Clone, Serialize, Deserialize)]
#[reflect(Component)]
pub struct CompanionPsychology {
    pub companion_id: Uuid,
    pub companion_type: String, // "einar", "mira", "sorin", "tamara"
    
    // Core trauma system (0-5 scale from Dragon's Labyrinth vision)
    pub trauma_level: f32,        // 0.0-5.0 trauma accumulation 
    pub breaking_point: f32,      // When companion breaks/leaves
    pub loyalty: f32,             // 0.0-1.0 loyalty to player
    pub trust: f32,               // 0.0-1.0 trust level
    
    // Trauma sources and triggers
    pub trauma_sources: Vec<TraumaSource>,
    pub trauma_triggers: Vec<String>,
    pub active_trauma_responses: Vec<TraumaResponse>,
    
    // Recovery and healing
    pub recovery_progress: f32,   // 0.0-1.0 overall healing progress
    pub therapy_readiness: f32,   // 0.0-1.0 willingness to engage in therapy
    pub breakthrough_potential: f32, // 0.0-1.0 chance of breakthrough
    
    // Relationship dynamics
    pub therapeutic_bond: f32,    // 0.0-1.0 strength of therapy relationship
    pub support_network_quality: f32, // 0.0-1.0 quality of peer support
    pub isolation_tendency: f32,  // 0.0-1.0 tendency to withdraw
}

/// Individual trauma source with specific triggers and effects
#[derive(Component, Reflect, Debug, Clone, Serialize, Deserialize)]
#[reflect(Component)]
pub struct TraumaSource {
    pub source_id: String,        // "combat_death", "dragon_encounter", "betrayal", etc.
    pub severity: f32,            // 0.0-1.0 severity of this trauma
    pub acquisition_context: String, // When/where this trauma occurred
    pub active_triggers: Vec<String>, // What currently triggers this trauma
    pub healing_progress: f32,    // 0.0-1.0 healing progress for this specific trauma
    pub requires_professional_help: bool, // Needs specialized intervention
}

/// Current trauma response affecting companion behavior
#[derive(Component, Reflect, Debug, Clone, Serialize, Deserialize)]
#[reflect(Component)]
pub struct TraumaResponse {
    pub response_type: String,    // "flashback", "panic", "dissociation", "anger", "withdrawal"
    pub intensity: f32,           // 0.0-1.0 current intensity
    pub duration_remaining: f32,  // Seconds until response fades
    pub triggered_by: String,     // What triggered this response
    pub behavioral_effects: Vec<BehavioralEffect>,
}

/// Behavioral effects of trauma on companion actions
#[derive(Reflect, Debug, Clone, Serialize, Deserialize)]
pub enum BehavioralEffect {
    CombatEffectiveness(f32),     // Modifier to combat performance
    DialogueRestriction(Vec<String>), // Dialogue options unavailable
    MovementHesitation(f32),      // Movement speed penalty
    DecisionParalysis(f32),       // Delay in decision-making
    Hypervigilance(f32),          // Increased awareness but also anxiety
    Dissociation(f32),            // Reduced responsiveness
}

/// Active therapy quest component
#[derive(Component, Reflect, Debug, Clone, Serialize, Deserialize)]
#[reflect(Component)]
pub struct TherapyQuest {
    pub quest_id: String,         // Unique therapy quest identifier
    pub therapy_type: String,     // "exposure", "cognitive", "narrative", "somatic"
    pub stage: TherapyStage,      // Current stage of therapy
    pub progress: f32,            // 0.0-1.0 progress through current stage
    
    // Quest objectives and structure
    pub primary_objective: String, // Main therapeutic goal
    pub secondary_objectives: Vec<String>, // Supporting goals
    pub required_actions: Vec<TherapyAction>, // Actions needed to progress
    pub completion_criteria: Vec<String>, // How to complete this therapy
    
    // Therapeutic context
    pub trauma_focus: String,     // Which trauma this addresses
    pub therapeutic_approach: String, // Method being used
    pub estimated_sessions: i32,  // Expected number of therapy sessions
    pub breakthrough_opportunities: Vec<String>, // Potential breakthrough moments
}

/// Stages of therapy progression
#[derive(Reflect, Debug, Clone, Serialize, Deserialize)]
pub enum TherapyStage {
    Assessment,      // Understanding the trauma
    Preparation,     // Building coping skills
    Processing,      // Working through trauma
    Integration,     // Incorporating healing
    Maintenance,     // Preventing relapse
    Breakthrough,    // Major healing moment
    Relapse,         // Temporary setback
}

/// Therapy actions player can take
#[derive(Reflect, Debug, Clone, Serialize, Deserialize)]
pub enum TherapyAction {
    ActiveListening,            // Listen without judgment
    AskGentleQuestion(String), // Ask specific supportive question
    SharePersonalExperience,   // Player shares similar experience
    OfferPhysicalComfort,     // Hug, hand on shoulder, etc.
    CreateSafeSpace,          // Ensure environment feels safe
    ValidateExperience,       // Acknowledge trauma as real and valid
    ChallengeCognition,       // Gently challenge negative thoughts
    PracticeGrounding,        // Grounding techniques for flashbacks
    EncourageExpression,      // Encourage emotional expression
    SetHealthyBoundaries,     // Help establish personal boundaries
}

/// Memory palace component for 3D therapy visualization
#[derive(Component, Reflect, Debug, Clone, Serialize, Deserialize)]
#[reflect(Component)]
pub struct MemoryPalace {
    pub companion_id: Uuid,
    pub palace_id: String,        // Unique memory palace identifier
    
    // 3D space configuration
    pub spatial_layout: String,   // "childhood_home", "safe_retreat", "trauma_site"
    pub room_configurations: Vec<MemoryRoom>, // Rooms within the palace
    pub navigation_mode: String,  // "guided", "free_exploration", "structured"
    
    // Therapeutic elements
    pub trauma_representations: Vec<TraumaObject>, // Objects representing trauma
    pub healing_symbols: Vec<HealingSymbol>,      // Symbols of recovery
    pub safe_zones: Vec<SafeZone>,                // Areas of comfort and safety
    
    // Progress tracking
    pub explored_areas: Vec<String>, // Areas companion has explored
    pub resolved_areas: Vec<String>, // Areas where healing has occurred
    pub locked_areas: Vec<String>,   // Areas too traumatic to access yet
}

/// Room within a memory palace
#[derive(Reflect, Debug, Clone, Serialize, Deserialize)]
pub struct MemoryRoom {
    pub room_id: String,
    pub room_type: String,        // "trauma_site", "safe_haven", "processing_space"
    pub emotional_tone: String,   // "fear", "sadness", "hope", "anger", "peace"
    pub accessibility: f32,       // 0.0-1.0 how easily companion can enter
    pub therapeutic_value: f32,   // 0.0-1.0 potential for healing
    pub required_preparation: Vec<String>, // What's needed before entering
}

/// Object representing trauma in memory palace
#[derive(Reflect, Debug, Clone, Serialize, Deserialize)]
pub struct TraumaObject {
    pub object_id: String,
    pub visual_representation: String, // How trauma appears in 3D space
    pub emotional_charge: f32,    // 0.0-1.0 emotional intensity
    pub interaction_options: Vec<String>, // Ways to interact therapeutically
    pub transformation_potential: f32, // 0.0-1.0 can this be transformed/healed?
}

/// Healing symbol in memory palace
#[derive(Reflect, Debug, Clone, Serialize, Deserialize)]
pub struct HealingSymbol {
    pub symbol_id: String,
    pub symbol_type: String,      // "light", "growth", "connection", "strength"
    pub manifestation: String,    // How it appears in 3D space
    pub healing_power: f32,       // 0.0-1.0 therapeutic effectiveness
    pub activation_requirements: Vec<String>, // How to activate this symbol
}

/// Safe zone within memory palace
#[derive(Reflect, Debug, Clone, Serialize, Deserialize)]
pub struct SafeZone {
    pub zone_id: String,
    pub comfort_level: f32,       // 0.0-1.0 how safe companion feels here
    pub recovery_rate: f32,       // Rate of emotional recovery in this zone
    pub accessible_from: Vec<String>, // Which rooms can access this zone
    pub special_properties: Vec<String>, // Unique healing properties
}

/// Inter-companion support relationship
#[derive(Component, Reflect, Debug, Clone, Serialize, Deserialize)]
#[reflect(Component)]
pub struct CompanionSupport {
    pub supporter_id: Uuid,       // Companion providing support
    pub supported_id: Uuid,       // Companion receiving support
    pub support_type: String,     // "peer_counseling", "shared_experience", "protective"
    pub effectiveness: f32,       // 0.0-1.0 how helpful this relationship is
    pub mutual_benefit: f32,      // 0.0-1.0 how much supporter also benefits
    pub relationship_stage: String, // "forming", "working", "healing", "thriving"
}

/// Therapy session tracking component
#[derive(Component, Reflect, Debug, Clone, Serialize, Deserialize)]
#[reflect(Component)]
pub struct TherapySession {
    pub session_id: String,
    pub companion_id: Uuid,
    pub session_type: String,     // "individual", "group", "couples", "family"
    
    // Session structure
    pub duration_minutes: f32,    // How long session lasts
    pub therapeutic_focus: String, // Main focus of this session
    pub planned_activities: Vec<String>, // Planned therapeutic activities
    pub breakthrough_potential: f32, // 0.0-1.0 chance of breakthrough
    
    // Progress tracking
    pub emotional_state_start: f32, // 0.0-1.0 emotional state at start
    pub emotional_state_end: f32,   // 0.0-1.0 emotional state at end
    pub insights_gained: Vec<String>, // Therapeutic insights from session
    pub homework_assigned: Vec<String>, // Take-home therapeutic work
    pub next_session_focus: String,     // Focus for next session
}

/// Professional support integration (when player connects companion with professionals)
#[derive(Component, Reflect, Debug, Clone, Serialize, Deserialize)]
#[reflect(Component)]
pub struct ProfessionalSupport {
    pub companion_id: Uuid,
    pub support_type: String,     // "therapy", "psychiatry", "support_group", "medical"
    pub quality_level: f32,       // 0.0-1.0 quality of professional support
    pub accessibility: f32,       // 0.0-1.0 how easily companion can access
    pub cost_burden: f32,         // 0.0-1.0 financial/logistical burden
    
    // Integration with player relationship
    pub player_involvement: f32,  // 0.0-1.0 how involved player is
    pub professional_player_collaboration: f32, // How well they work together
    pub treatment_plan_transparency: f32, // How much player knows about treatment
}

/// Companion psychological resilience factors
#[derive(Component, Reflect, Debug, Clone, Serialize, Deserialize)]
#[reflect(Component)]
pub struct PsychologicalResilience {
    pub companion_id: Uuid,
    
    // Resilience factors
    pub emotional_regulation: f32, // 0.0-1.0 ability to manage emotions
    pub cognitive_flexibility: f32, // 0.0-1.0 ability to adapt thinking
    pub social_connection: f32,    // 0.0-1.0 quality of relationships
    pub meaning_making: f32,       // 0.0-1.0 ability to find purpose
    pub self_efficacy: f32,        // 0.0-1.0 belief in own capabilities
    
    // Coping strategies
    pub healthy_coping_strategies: Vec<String>, // Adaptive coping methods
    pub unhealthy_coping_strategies: Vec<String>, // Maladaptive methods
    pub coping_strategy_effectiveness: Vec<f32>, // How well each strategy works
    
    // Growth and adaptation
    pub post_traumatic_growth: f32, // 0.0-1.0 positive changes from trauma
    pub wisdom_gained: f32,        // 0.0-1.0 wisdom from difficult experiences
    pub empathy_development: f32,  // 0.0-1.0 increased empathy for others
}

/// Trauma recovery milestone tracking
#[derive(Component, Reflect, Debug, Clone, Serialize, Deserialize)]
#[reflect(Component)]
pub struct RecoveryMilestone {
    pub milestone_id: String,
    pub companion_id: Uuid,
    pub milestone_type: String,   // "first_disclosure", "first_nightmare_free_night", etc.
    pub achievement_date: i64,    // Timestamp of achievement
    pub significance: f32,        // 0.0-1.0 importance of this milestone
    pub celebration_enacted: bool, // Did player/party celebrate this?
    pub relapse_risk_factors: Vec<String>, // What could trigger setback
    pub consolidation_activities: Vec<String>, // Activities to strengthen progress
}
