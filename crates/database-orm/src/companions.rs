//! Companion models for sophisticated psychology system

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Companion characters with trauma progression and betrayal paths
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "companions")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    
    // Player relationship
    pub player_id: Uuid,
    
    // Character identity
    pub companion_type: String, // "einar", "mira", "sorin", "tamara"
    pub name: String,
    pub display_name: String,
    
    // Combat stats
    pub level: i32,
    pub health: f32,
    pub max_health: f32,
    pub attack: f32,
    pub defense: f32,
    
    // Trauma system - core mechanic for companions
    pub trauma_level: f32,        // 0.0-5.0 trauma accumulation (0-5 scale from vision)
    pub trauma_sources: Json,     // Array of trauma events
    pub breaking_point: f32,      // When companion breaks/leaves
    pub loyalty: f32,             // 0.0-1.0 loyalty to player
    pub trust: f32,               // 0.0-1.0 trust level
    
    // Story progression
    pub current_arc_stage: String,      // Current story beat
    pub personal_quest_progress: Json,   // Personal quest states
    pub relationship_flags: Json,       // Story flags and choices
    pub has_betrayed: bool,             // Has this companion betrayed player?
    pub betrayal_reason: Option<String>, // Why they betrayed
    pub can_return: bool,               // Can they come back after betrayal?
    
    // Availability and status
    pub is_active: bool,          // Currently in party
    pub is_available: bool,       // Can be recruited
    pub current_location: Option<String>, // Where they are if not active
    pub departure_reason: Option<String>, // Why they left
    
    // Mount system integration
    pub is_mount: bool,           // Can be used as mount
    pub mount_speed: f32,         // Movement speed when mounted
    pub mount_capacity: f32,      // Carrying capacity bonus
    
    // Visual progression (AI-generated states)
    pub visual_state_id: Option<String>, // References AI-generated visual
    pub trauma_visual_level: i32,        // 0-4 trauma visualization level
    
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::players::Entity",
        from = "Column::PlayerId",
        to = "super::players::Column::Id"
    )]
    Player,
    #[sea_orm(has_many = "super::psychology::Entity")]
    Therapy,
}

impl Related<super::players::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Player.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
