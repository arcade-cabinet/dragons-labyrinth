//! Dungeon room models

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Individual rooms within dungeons
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "dungeon_rooms")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    
    pub dungeon_id: Uuid, // Reference to parent dungeon
    
    // Room identification
    pub area_number: i32, // Room number within dungeon (1, 2, 3, etc.)
    #[sea_orm(column_type = "Text")]
    pub title: String, // "Chamber", "Corridor", "Crypt"
    
    #[sea_orm(column_type = "Text")]
    pub room_type: String, // "chamber", "corridor", "hall", "vault", "crypt"
    
    // HBF import data
    #[sea_orm(column_type = "Text", nullable)]
    pub hbf_uuid: Option<String>, // Original HBF UUID if imported
    
    // Room description
    #[sea_orm(column_type = "Text")]
    pub description: String, // Rich atmospheric description
    
    // Room features
    #[sea_orm(column_type = "Json", nullable)]
    pub doorways: Option<serde_json::Value>, // JSON array of doorway data
    
    #[sea_orm(column_type = "Json", nullable)]
    pub features: Option<serde_json::Value>, // Room features, furniture, etc.
    
    #[sea_orm(column_type = "Json", nullable)]
    pub encounters: Option<serde_json::Value>, // Creatures, traps, etc.
    
    #[sea_orm(column_type = "Json", nullable)]
    pub treasure: Option<serde_json::Value>, // Loot in this room
    
    // Environmental effects
    #[sea_orm(column_type = "Text", nullable)]
    pub lighting: Option<String>, // "dark", "dim", "bright", "magical"
    
    #[sea_orm(column_type = "Text", nullable)]
    pub atmosphere: Option<String>, // "cold", "damp", "eerie", "oppressive"
    
    #[sea_orm(column_type = "Json", nullable)]
    pub environmental_effects: Option<serde_json::Value>, // Special room effects
    
    // Exploration state
    pub discovered: bool, // Has player found this room?
    pub searched: bool, // Has player searched this room?
    pub cleared: bool, // Has player cleared encounters in this room?
    pub first_entered_at: Option<DateTime<Utc>>,
    pub last_visited_at: Option<DateTime<Utc>>,
    
    // Timestamps
    pub created_at: DateTime<Utc>,
    pub last_modified: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = "super::Entity", from = "Column::DungeonId", to = "super::Column::Id")]
    Dungeon,
}

impl Related<super::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Dungeon.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
