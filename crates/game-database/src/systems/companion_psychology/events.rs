//! Companion Psychology System - ECS Events
//!
//! Production-ready events for companion trauma, therapy progression, and psychological healing
//! with full database integration and horror progression mechanics.

use bevy::prelude::*;
use uuid::Uuid;
use crate::systems::companion_psychology::components::*;

/// Core trauma event - triggers trauma accumulation and responses
#[derive(Event, Debug, Clone)]
pub struct TraumaEvent {
    pub companion_entity: Entity,
    pub companion_id: Uuid,
    pub trauma_type: String, // "dragon_encounter", "companion_death", "betrayal", "corruption_exposure"
    pub source_id: String,   // Specific identifier for this trauma source
    pub severity: f32,       // 0.0-5.0 trauma severity on Dragon's Labyrinth scale
    pub context: String,     // Where/when/how this trauma occurred
    pub triggers: Vec<String>, // What environmental cues trigger this trauma
    pub dread_level_amplifier: f32, // Current dread level amplifies trauma
    pub witness_companions: Vec<Uuid>, // Other companions who witnessed this
}

/// Event for triggering trauma responses (flashbacks, panic, etc.)
#[derive(Event, Debug, Clone)]
pub struct TraumaTriggerEvent {
    pub companion_entity: Entity,
    pub companion_id: Uuid,
    pub trigger_stimulus: String, // What triggered the response
    pub trigger_intensity: f32,   // How intense the trigger was
    pub environmental_context: String, // Current environment/situation
    pub player_proximity: f32,    // How close player is (affects response)
    pub other_companions_present: Vec<Uuid>, // Who else is present
}

/// Event when companion reaches breaking point
#[derive(Event, Debug, Clone)]
pub struct CompanionBreakingPointEvent {
    pub companion_entity: Entity,
    pub companion_id: Uuid,
    pub trigger_trauma: String,   // Final trauma that broke them
    pub breaking_point_type: String, // "traumatic_breakdown", "trust_betrayal", "overwhelmed_shutdown"
    pub severity_level: f32,      // 0.0-1.0 severity of breakdown
    pub recovery_potential: f32,  // 0.0-1.0 likelihood of recovery
    pub immediate_needs: Vec<String>, // What companion needs right now
}

/// Event for companion departing party (temporary or permanent)
#[derive(Event, Debug, Clone)]
pub struct CompanionDepartureEvent {
    pub companion_entity: Entity,
    pub companion_id: Uuid,
    pub departure_type: String,   // "temporary_breakdown", "trust_betrayal", "permanent_leaving"
    pub departure_reason: String, // Detailed explanation
    pub return_conditions: Vec<String>, // What needs to happen for them to return
    pub estimated_departure_duration: f32, // Seconds (-1 for indefinite)
    pub departure_location: Option<String>, // Where they go
    pub final_message: Option<String>,     // Last thing they say
}

/// Event for therapy action taken by player
#[derive(Event, Debug, Clone)]
pub struct TherapyActionEvent {
    pub companion_entity: Entity,
    pub therapy_entity: Entity,
    pub companion_id: Uuid,
    pub action: TherapyAction,
    pub action_context: String,   // Situation/environment where action was taken
    pub player_skill_level: f32,  // 0.0-1.0 player's therapeutic skill
    pub timing_quality: f32,      // 0.0-1.0 how well-timed the action was
    pub companion_receptivity: f32, // 0.0-1.0 how receptive companion is right now
    pub environmental_support: f32, // 0.0-1.0 how supportive the environment is
}

/// Event for therapy breakthrough moments
#[derive(Event, Debug, Clone)]
pub struct TherapyBreakthroughEvent {
    pub companion_entity: Entity,
    pub therapy_entity: Entity,
    pub companion_id: Uuid,
    pub breakthrough_type: String, // "trauma_resolution", "emotional_regulation", "narrative_coherence"
    pub significance: f32,         // 0.0-1.0 significance of breakthrough
    pub insights_gained: Vec<String>, // Specific insights from breakthrough
    pub emotional_release: f32,    // 0.0-1.0 emotional catharsis level
    pub integration_potential: f32, // 0.0-1.0 likelihood of lasting change
    pub celebration_appropriate: bool, // Should this be celebrated?
}

/// Event for recovery milestone achievements
#[derive(Event, Debug, Clone)]
pub struct RecoveryMilestoneEvent {
    pub companion_entity: Entity,
    pub companion_id: Uuid,
    pub milestone_type: String,    // "first_disclosure", "first_nightmare_free_night", "trust_rebuilding"
    pub achievement_date: i64,     // Timestamp of achievement
    pub significance: f32,         // 0.0-1.0 importance of milestone
    pub celebration_appropriate: bool, // Should party celebrate this?
    pub progress_indicators: Vec<String>, // Signs of this progress
    pub consolidation_needed: bool, // Does this need reinforcement?
}

/// Event for memory palace interactions
#[derive(Event, Debug, Clone)]
pub struct MemoryPalaceEvent {
    pub palace_entity: Entity,
    pub companion_entity: Entity,
    pub companion_id: Uuid,
    pub action: MemoryPalaceAction,
    pub player_guidance_quality: f32, // 0.0-1.0 how well player is guiding
    pub companion_readiness: f32,     // 0.0-1.0 how ready companion is for this
    pub therapeutic_support_present: bool, // Is professional help available?
}

/// Actions that can be taken in memory palace
#[derive(Debug, Clone)]
pub enum MemoryPalaceAction {
    EnterRoom(String),                    // Enter a specific room
    InteractWithTraumaObject(String),     // Interact with trauma representation
    ActivateHealingSymbol(String),        // Activate healing symbol
    CreateSafeZone(String),              // Create new safe zone
    GuideCompanionTo(String),            // Guide companion to specific area
    ProcessTraumaMemory(String),         // Work through specific memory
    IntegrateHealedTrauma(String),       // Integrate resolved trauma
    ExitToSafety,                        // Emergency exit to safety
}

/// Event for healing within memory palace
#[derive(Event, Debug, Clone)]
pub struct MemoryPalaceHealingEvent {
    pub palace_entity: Entity,
    pub companion_entity: Entity,
    pub companion_id: Uuid,
    pub room_id: String,
    pub healing_type: String,     // Type of healing occurring
    pub healing_power: f32,       // 0.0-1.0 strength of healing
    pub trauma_addressed: Vec<String>, // Which traumas are being healed
    pub emotional_breakthrough: bool,  // Major emotional breakthrough?
    pub integration_progress: f32, // 0.0-1.0 integration of healing
}

/// Event for companion support interactions
#[derive(Event, Debug, Clone)]
pub struct CompanionSupportEvent {
    pub supporter_entity: Entity,
    pub supported_entity: Entity,
    pub supporter_id: Uuid,
    pub supported_id: Uuid,
    pub support_type: String,     // "peer_counseling", "shared_experience", "protective"
    pub support_quality: f32,     // 0.0-1.0 quality of support provided
    pub mutual_benefit: f32,      // 0.0-1.0 benefit to supporter as well
    pub context: String,          // Situation where support was provided
    pub effectiveness: f32,       // 0.0-1.0 how effective the support was
    pub emotional_resonance: f32, // 0.0-1.0 emotional connection achieved
}

/// Event for therapy session management
#[derive(Event, Debug, Clone)]
pub struct TherapySessionEvent {
    pub session_entity: Entity,
    pub companion_entity: Entity,
    pub companion_id: Uuid,
    pub session_action: TherapySessionAction,
    pub professional_support_present: bool, // Is professional helping?
    pub session_quality_factors: Vec<SessionQualityFactor>, // Factors affecting quality
}

/// Actions within therapy session
#[derive(Debug, Clone)]
pub enum TherapySessionAction {
    Start,                           // Begin therapy session
    Progress(String),                // Progress with specific activity
    HandleBreakthrough,              // Manage breakthrough moment
    AddressResistance(String),       // Handle therapeutic resistance
    ProcessEmotionalRelease,         // Process emotional catharsis
    IntegrateInsights(Vec<String>),  // Integrate therapeutic insights
    Complete,                        // End therapy session
    EmergencyIntervention(String),   // Handle crisis during session
}

/// Factors affecting therapy session quality
#[derive(Debug, Clone)]
pub struct SessionQualityFactor {
    pub factor_type: String,         // Type of factor
    pub impact: f32,                 // -1.0 to 1.0 impact on quality
    pub description: String,         // Description of factor
}

/// Event when therapy session completes
#[derive(Event, Debug, Clone)]
pub struct TherapySessionCompleteEvent {
    pub session_entity: Entity,
    pub companion_entity: Entity,
    pub companion_id: Uuid,
    pub session_duration_minutes: f32, // How long session lasted
    pub emotional_progress: f32,      // -1.0 to 1.0 emotional change
    pub insights_count: usize,        // Number of insights gained
    pub breakthrough_occurred: bool,  // Did breakthrough happen?
    pub homework_assigned: Vec<String>, // Take-home therapeutic work
    pub next_session_recommended: bool, // Should there be another session?
    pub next_session_focus: Option<String>, // Focus for next session
}

/// Event for professional support integration
#[derive(Event, Debug, Clone)]
pub struct ProfessionalSupportEvent {
    pub companion_entity: Entity,
    pub companion_id: Uuid,
    pub support_type: String,         // "therapy", "psychiatry", "support_group", "medical"
    pub provider_id: String,          // Which professional provider
    pub support_action: ProfessionalSupportAction,
    pub cost: f32,                    // Cost of this support
    pub accessibility_barriers: Vec<String>, // Barriers to accessing support
    pub cultural_fit: f32,           // 0.0-1.0 how well this fits companion's culture
}

/// Actions professional support can take
#[derive(Debug, Clone)]
pub enum ProfessionalSupportAction {
    InitialAssessment,               // First evaluation
    ProvideTherapy(String),         // Specific therapy type
    PrescribeMedication(String),    // Medication recommendation
    ReferToSpecialist(String),      // Refer to another professional
    CrisisIntervention,             // Emergency intervention
    FollowUpAssessment,             // Check progress
    DischargeFromCare,              // End professional relationship
}

/// Event for dread level affecting psychology systems
#[derive(Event, Debug, Clone)]
pub struct DreadPsychologyEvent {
    pub current_dread_level: u8,      // 0-4 current dread level
    pub previous_dread_level: u8,     // Previous dread level
    pub affected_companions: Vec<Uuid>, // Companions affected by change
    pub psychological_effects: Vec<DreadPsychologicalEffect>, // Specific effects
    pub system_modifications: Vec<String>, // How systems are modified
}

/// Psychological effects of dread level changes
#[derive(Debug, Clone)]
pub struct DreadPsychologicalEffect {
    pub effect_type: String,          // Type of psychological effect
    pub intensity: f32,               // 0.0-1.0 intensity of effect
    pub affected_systems: Vec<String>, // Which psychological systems affected
    pub duration_estimate: f32,       // Estimated duration in seconds
    pub reversible: bool,             // Can this effect be reversed?
}

/// Event for companion psychological assessment
#[derive(Event, Debug, Clone)]
pub struct PsychologicalAssessmentEvent {
    pub companion_entity: Entity,
    pub companion_id: Uuid,
    pub assessment_type: String,      // "routine", "crisis", "pre_therapy", "progress_check"
    pub assessor: String,             // Who is doing assessment ("player", "professional", "peer")
    pub assessment_results: PsychologicalAssessmentResults,
    pub recommendations: Vec<String>, // Recommended actions
    pub urgency_level: f32,          // 0.0-1.0 urgency of addressing issues
}

/// Results of psychological assessment
#[derive(Debug, Clone)]
pub struct PsychologicalAssessmentResults {
    pub overall_mental_health: f32,   // 0.0-1.0 overall assessment
    pub trauma_severity: f32,         // 0.0-5.0 trauma level assessment
    pub resilience_factors: Vec<String>, // Identified strengths
    pub risk_factors: Vec<String>,    // Identified risks
    pub immediate_needs: Vec<String>, // Urgent needs
    pub treatment_readiness: f32,     // 0.0-1.0 readiness for treatment
    pub support_system_quality: f32, // 0.0-1.0 quality of current support
}

/// Event for companion psychology research and learning
#[derive(Event, Debug, Clone)]
pub struct PsychologyResearchEvent {
    pub researcher_entity: Entity,    // Who is doing research (player, NPC)
    pub research_focus: String,       // What aspect being researched
    pub research_method: String,      // How research is conducted
    pub knowledge_gained: Vec<String>, // New insights gained
    pub practical_applications: Vec<String>, // How knowledge can be applied
    pub research_quality: f32,        // 0.0-1.0 quality of research
    pub ethical_considerations: Vec<String>, // Ethical aspects considered
}

/// Event for companion behavioral changes due to psychology
#[derive(Event, Debug, Clone)]
pub struct CompanionBehaviorChangeEvent {
    pub companion_entity: Entity,
    pub companion_id: Uuid,
    pub behavior_change_type: String, // Type of behavioral change
    pub previous_behavior_pattern: String, // How they behaved before
    pub new_behavior_pattern: String, // How they behave now
    pub change_trigger: String,       // What caused the change
    pub permanence_estimate: f32,     // 0.0-1.0 likelihood of lasting change
    pub impact_on_gameplay: Vec<String>, // How this affects gameplay
    pub player_adaptation_needed: bool, // Does player need to adapt?
}

/// Event for trauma contamination (trauma affecting other companions)
#[derive(Event, Debug, Clone)]
pub struct TraumaContaminationEvent {
    pub source_companion: Uuid,      // Companion with original trauma
    pub affected_companions: Vec<Uuid>, // Companions being affected
    pub contamination_type: String,   // Type of trauma spread
    pub transmission_mechanism: String, // How trauma spread
    pub severity_reduction: f32,      // How much trauma reduces when spreading
    pub protective_factors: Vec<String>, // What might prevent spread
    pub intervention_opportunities: Vec<String>, // How player can help
}

/// Event for companion psychology integration with other systems
#[derive(Event, Debug, Clone)]
pub struct PsychologyIntegrationEvent {
    pub companion_entity: Entity,
    pub companion_id: Uuid,
    pub integrating_system: String,   // Which system is integrating
    pub integration_type: String,     // Type of integration
    pub psychological_modifiers: Vec<PsychologicalModifier>, // How psychology affects system
    pub feedback_effects: Vec<String>, // How system affects psychology back
}

/// Psychological modifiers applied to other systems
#[derive(Debug, Clone)]
pub struct PsychologicalModifier {
    pub modifier_type: String,        // Type of modifier
    pub target_attribute: String,     // What attribute is modified
    pub modifier_value: f32,          // Modifier value
    pub duration: Option<f32>,        // Duration in seconds (None for permanent)
    pub stacks: bool,                 // Can multiple modifiers stack?
}

/// Event for player psychological skill development
#[derive(Event, Debug, Clone)]
pub struct PlayerPsychologySkillEvent {
    pub player_entity: Entity,
    pub skill_area: String,           // Area of psychological skill
    pub skill_improvement: f32,       // Amount of improvement
    pub learning_source: String,      // How skill was learned
    pub practical_application: String, // How skill was applied
    pub teaching_opportunities: Vec<String>, // Opportunities to teach others
    pub skill_level_reached: f32,     // Current skill level 0.0-1.0
}

/// Event for companion trust and relationship changes
#[derive(Event, Debug, Clone)]
pub struct CompanionTrustEvent {
    pub companion_entity: Entity,
    pub companion_id: Uuid,
    pub trust_change: f32,            // Change in trust level (-1.0 to 1.0)
    pub loyalty_change: f32,          // Change in loyalty level (-1.0 to 1.0)
    pub trust_change_reason: String,  // Why trust changed
    pub relationship_milestone: Option<String>, // Relationship milestone reached
    pub behavioral_implications: Vec<String>, // How this affects behavior
    pub recovery_potential: f32,      // 0.0-1.0 potential to rebuild if negative
}

/// Event for therapeutic dialogue opportunities
#[derive(Event, Debug, Clone)]
pub struct TherapeuticDialogueEvent {
    pub companion_entity: Entity,
    pub companion_id: Uuid,
    pub dialogue_opportunity: String, // Type of therapeutic dialogue available
    pub emotional_context: String,    // Current emotional state/context
    pub available_approaches: Vec<TherapeuticApproach>, // Ways player can respond
    pub timing_window: f32,           // How long opportunity lasts (seconds)
    pub difficulty_level: f32,        // 0.0-1.0 difficulty of handling well
    pub potential_outcomes: Vec<DialogueOutcome>, // Possible results
}

/// Therapeutic approaches available in dialogue
#[derive(Debug, Clone)]
pub struct TherapeuticApproach {
    pub approach_name: String,        // Name of approach
    pub skill_requirement: f32,       // Required player skill level 0.0-1.0
    pub risk_level: f32,              // 0.0-1.0 risk of making things worse
    pub potential_benefit: f32,       // 0.0-1.0 potential positive impact
    pub approach_description: String, // What this approach involves
}

/// Possible outcomes of therapeutic dialogue
#[derive(Debug, Clone)]
pub struct DialogueOutcome {
    pub outcome_type: String,         // Type of outcome
    pub likelihood: f32,              // 0.0-1.0 likelihood of this outcome
    pub impact_description: String,   // Description of impact
    pub follow_up_opportunities: Vec<String>, // What this opens up
}

/// Event for companion psychological crisis
#[derive(Event, Debug, Clone)]
pub struct PsychologicalCrisisEvent {
    pub companion_entity: Entity,
    pub companion_id: Uuid,
    pub crisis_type: String,          // "panic_attack", "dissociation", "self_harm_risk", "suicide_ideation"
    pub crisis_severity: f32,         // 0.0-1.0 severity of crisis
    pub immediate_danger: bool,       // Is there immediate physical danger?
    pub triggers: Vec<String>,        // What triggered the crisis
    pub available_interventions: Vec<CrisisIntervention>, // Possible responses
    pub professional_help_needed: bool, // Should professional help be sought?
    pub support_people_present: Vec<Uuid>, // Who is available to help
}

/// Crisis intervention options
#[derive(Debug, Clone)]
pub struct CrisisIntervention {
    pub intervention_name: String,    // Name of intervention
    pub intervention_type: String,    // Type of intervention
    pub effectiveness_estimate: f32,  // 0.0-1.0 estimated effectiveness
    pub resource_requirements: Vec<String>, // What resources are needed
    pub time_to_effectiveness: f32,   // Seconds until intervention works
    pub side_effects: Vec<String>,    // Potential negative effects
}
