//! Psychology components for sophisticated companion therapy system

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Therapy quest system with psychological authenticity
#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub struct CompanionTherapy {
    pub id: Uuid,
    pub companion_entity: Entity,
    pub player_entity: Entity,
    
    // Therapy quest system
    pub therapy_quest_id: String,         // Current therapy quest
    pub therapy_stage: TherapyStage,
    pub therapy_progress: f32,            // 0.0-1.0 progress through current quest
    pub total_therapy_quests_completed: i32,
    
    // Psychological assessment
    pub trauma_triggers: Vec<TraumaTrigger>,
    pub coping_mechanisms: Vec<CopingMechanism>,
    pub emotional_vulnerabilities: Vec<EmotionalVulnerability>,
    pub psychological_strengths: Vec<PsychologicalStrength>,
    
    // Healing mission tracking
    pub active_healing_missions: Vec<HealingMission>,
    pub completed_healing_missions: Vec<HealingMission>,
    pub healing_dialogue_trees: Vec<TherapeuticDialogue>,
    pub breakthrough_moments: Vec<BreakthroughMoment>,
    
    // Therapy dialogue system
    pub therapeutic_conversation_history: Vec<TherapeuticConversation>,
    pub current_therapeutic_focus: String,
    pub therapy_dialogue_options: Vec<TherapyDialogueOption>,
    pub therapeutic_relationship_quality: f32,  // 0.0-1.0 therapy bond strength
    
    // Inter-companion support system
    pub support_relationships: Vec<Entity>,     // Companion entities that can help
    pub provides_support_to: Vec<Entity>,       // Companion entities this one helps
    pub group_therapy_sessions: Vec<GroupTherapySession>,
    pub peer_support_effectiveness: f32,        // How well companions help each other
    
    // Recovery tracking
    pub recovery_milestones: Vec<RecoveryMilestone>,
    pub setback_incidents: Vec<SetbackIncident>,
    pub recovery_resilience_score: f32,   // 0.0-1.0 resistance to setbacks
    pub long_term_healing_trajectory: HealingTrajectory,
    
    // Professional help integration
    pub has_professional_support: bool,   // Are professionals involved?
    pub professional_support_type: ProfessionalSupportType,
    pub professional_support_quality: f32, // Quality of professional help
    
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub enum TherapyStage {
    Assessment,
    Processing,
    Healing,
    Integration,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub struct TraumaTrigger {
    pub trigger_type: String,
    pub description: String,
    pub severity_level: f32,  // 0.0-1.0 how severe the trigger is
    pub frequency: f32,       // 0.0-1.0 how often this occurs
    pub coping_strategies: Vec<String>,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub struct CopingMechanism {
    pub mechanism_type: String,
    pub description: String,
    pub effectiveness: f32,   // 0.0-1.0 how well this works
    pub healthiness: f32,     // 0.0-1.0 how healthy this coping is
    pub requires_support: bool,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub struct EmotionalVulnerability {
    pub vulnerability_type: String,
    pub description: String,
    pub impact_level: f32,    // 0.0-1.0 how much this affects companion
    pub related_trauma: Option<String>,
    pub protection_strategies: Vec<String>,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub struct PsychologicalStrength {
    pub strength_type: String,
    pub description: String,
    pub resilience_contribution: f32,  // 0.0-1.0 how much this helps
    pub can_help_others: bool,
    pub development_potential: f32,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub struct HealingMission {
    pub mission_id: String,
    pub mission_name: String,
    pub description: String,
    pub mission_type: HealingMissionType,
    pub required_support: Vec<String>,
    pub completion_criteria: Vec<String>,
    pub rewards: Vec<String>,
    pub is_completed: bool,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub enum HealingMissionType {
    MemoryProcessing,
    TraumaCoping,
    RelationshipBuilding,
    SelfCareRoutine,
    PeerSupport,
    ProfessionalTherapy,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub struct TherapeuticDialogue {
    pub dialogue_id: String,
    pub dialogue_name: String,
    pub therapeutic_approach: TherapeuticApproach,
    pub required_relationship_quality: f32,
    pub conversation_branches: Vec<String>,
    pub healing_potential: f32,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub enum TherapeuticApproach {
    CognitiveBehavioral,
    TraumaInformed,
    Mindfulness,
    NarrativeTherapy,
    GroupSupport,
    PeerSupport,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub struct BreakthroughMoment {
    pub moment_description: String,
    pub emotional_impact: f32,
    pub healing_acceleration: f32,
    pub affects_other_companions: bool,
    pub timestamp: DateTime<Utc>,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub struct TherapeuticConversation {
    pub conversation_id: String,
    pub participant_entities: Vec<Entity>,  // Player + involved companions
    pub therapeutic_focus: String,
    pub conversation_quality: f32,
    pub healing_progress_made: f32,
    pub timestamp: DateTime<Utc>,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub struct TherapyDialogueOption {
    pub option_text: String,
    pub therapeutic_value: f32,
    pub requires_training: bool,
    pub emotional_risk: f32,
    pub potential_outcomes: Vec<String>,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub struct GroupTherapySession {
    pub session_id: String,
    pub participating_companions: Vec<Entity>,
    pub facilitator_entity: Option<Entity>,  // Player or professional
    pub session_focus: String,
    pub group_dynamics_quality: f32,
    pub individual_benefits: HashMap<Entity, f32>,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub struct RecoveryMilestone {
    pub milestone_name: String,
    pub description: String,
    pub significance_level: f32,
    pub celebration_method: String,
    pub reinforcement_value: f32,
    pub achieved_at: DateTime<Utc>,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub struct SetbackIncident {
    pub incident_description: String,
    pub trigger_source: String,
    pub severity_level: f32,
    pub recovery_time_days: u32,
    pub lessons_learned: Vec<String>,
    pub occurred_at: DateTime<Utc>,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub enum HealingTrajectory {
    Improving,
    Stable,
    Declining,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub enum ProfessionalSupportType {
    Therapist,
    Counselor,
    SupportGroup,
    MedicalProfessional,
    PeerSupport,
    None,
}

/// Event fired when therapy progress is made
#[derive(Event, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Event)]
pub struct TherapyProgressEvent {
    pub companion_entity: Entity,
    pub progress_type: TherapyProgressType,
    pub progress_amount: f32,
    pub related_mission: Option<String>,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub enum TherapyProgressType {
    BreakthroughAchieved,
    MilestoneReached,
    SetbackOccurred,
    SupportProvided,
    SupportReceived,
    ProfessionalHelpEngaged,
}

/// Component for tracking memory palace healing mechanics
#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub struct MemoryPalace {
    pub palace_id: String,
    pub owner_entity: Entity,
    pub rooms: Vec<MemoryRoom>,
    pub healing_progress: f32,     // 0.0-1.0 overall healing in palace
    pub accessibility: f32,        // 0.0-1.0 how easily memories can be accessed
    pub organization_level: f32,   // 0.0-1.0 how well organized the palace is
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub struct MemoryRoom {
    pub room_id: String,
    pub room_type: MemoryRoomType,
    pub emotional_charge: f32,     // -1.0 to 1.0 (negative to positive)
    pub healing_status: HealingStatus,
    pub associated_memories: Vec<String>,
    pub requires_support: bool,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub enum MemoryRoomType {
    TraumaRoom,
    SafeRoom,
    RelationshipRoom,
    AchievementRoom,
    FearRoom,
    HopeRoom,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub enum HealingStatus {
    Untouched,
    InProgress,
    Stabilized,
    Healed,
    Integrated,
}
