//! Player model for sophisticated horror progression tracking

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Player character entity with horror progression tracking
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "players")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    
    // Basic character info
    pub name: String,
    pub level: i32,
    pub experience: i64,
    pub health: f32,
    pub max_health: f32,
    pub sanity: f32,
    pub max_sanity: f32,
    
    // Horror progression - core mechanic
    pub current_dread_level: i32, // 0-4 (Peace → Horror)
    pub dread_progression: f32,   // 0.0-1.0 within current level
    pub horror_exposure: f32,     // Cumulative horror experienced
    pub corruption_level: f32,    // Player corruption from dread
    
    // Position in hex world
    pub hex_position_q: i32,      // Hex coordinates (axial)
    pub hex_position_r: i32,
    pub hex_position_s: i32,      // Derived but stored for queries
    pub world_position_x: f32,    // World space position
    pub world_position_y: f32,
    pub world_position_z: f32,
    
    // Save slot management
    pub save_slot_id: i32,
    pub save_name: String,
    pub play_time_seconds: i64,
    pub last_played: DateTime<Utc>,
    pub completion_percentage: f32,
    
    // Progression tracking
    pub is_reverse_playthrough: bool,   // Second playthrough (harder)
    pub reverse_completion_count: i32,  // How many reverse runs completed
    pub endings_unlocked: Json,         // Array of ending IDs achieved
    
    // Timestamps
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::companions::Entity")]
    Companions,
    #[sea_orm(has_many = "super::forge::forge_progress::Entity")]
    ForgeProgress,
    #[sea_orm(has_many = "super::philosophy::Entity")]
    PhilosophicalProgression,
    #[sea_orm(has_many = "super::decay::Entity")]
    WorldCorruption,
}

impl Related<super::companions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Companions.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
