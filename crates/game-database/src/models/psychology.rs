//! Psychology models for sophisticated companion therapy system

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Therapy quest system with psychological authenticity
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "companion_therapy")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    
    pub companion_id: Uuid,
    pub player_id: Uuid,
    
    // Therapy quest system
    pub therapy_quest_id: String,         // Current therapy quest
    pub therapy_stage: String,            // "assessment", "processing", "healing", "integration"
    pub therapy_progress: f32,            // 0.0-1.0 progress through current quest
    pub total_therapy_quests_completed: i32,
    
    // Psychological assessment
    pub trauma_triggers: Json,            // Specific things that trigger trauma
    pub coping_mechanisms: Json,          // How companion deals with trauma
    pub emotional_vulnerabilities: Json, // Emotional weak points
    pub psychological_strengths: Json,    // Mental resilience factors
    
    // Healing mission tracking
    pub active_healing_missions: Json,    // Currently active therapy missions
    pub completed_healing_missions: Json, // Finished therapy work
    pub healing_dialogue_trees: Json,     // Available therapeutic dialogues
    pub breakthrough_moments: Json,       // Major healing breakthroughs
    
    // Therapy dialogue system
    pub therapeutic_conversation_history: Json, // Past therapy conversations
    pub current_therapeutic_focus: String,      // Current therapy topic
    pub therapy_dialogue_options: Json,         // Available dialogue choices
    pub therapeutic_relationship_quality: f32,  // 0.0-1.0 therapy bond strength
    
    // Inter-companion support system
    pub support_relationships: Json,      // Which companions can help this one
    pub provides_support_to: Json,        // Which companions this one helps
    pub group_therapy_sessions: Json,     // Multi-companion therapy
    pub peer_support_effectiveness: f32,  // How well companions help each other
    
    // Recovery tracking
    pub recovery_milestones: Json,        // Major recovery achievements
    pub setback_incidents: Json,          // Times when trauma got worse
    pub recovery_resilience_score: f32,   // 0.0-1.0 resistance to setbacks
    pub long_term_healing_trajectory: String, // "improving", "stable", "declining"
    
    // Professional help integration
    pub has_professional_support: bool,   // Are professionals involved?
    pub professional_support_type: String, // Type of professional help
    pub professional_support_quality: f32, // Quality of professional help
    
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::companions::Entity",
        from = "Column::CompanionId",
        to = "super::companions::Column::Id"
    )]
    Companion,
    #[sea_orm(
        belongs_to = "super::players::Entity",
        from = "Column::PlayerId",
        to = "super::players::Column::Id"
    )]
    Player,
}

impl Related<super::companions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Companion.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
