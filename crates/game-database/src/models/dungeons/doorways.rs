//! Dungeon doorway models

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Doorways connecting dungeon rooms
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "dungeon_doorways")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    
    pub room_id: Uuid, // Reference to room containing this doorway
    
    #[sea_orm(column_type = "Text")]
    pub direction: String, // "north", "south", "east", "west", "up", "down"
    
    #[sea_orm(column_type = "Text")]
    pub material: String, // "wooden", "iron", "bronze", "marble", "stone"
    
    #[sea_orm(column_type = "Text")]
    pub shape: String, // "rectangular", "round", "arched"
    
    #[sea_orm(column_type = "Text")]
    pub condition: String, // "normal", "stuck", "broken", "barricaded", "locked"
    
    // Connection data
    pub leads_to_room_id: Option<Uuid>, // What room this connects to (if known)
    pub leads_to_area_number: Option<i32>, // Area number it connects to
    
    // Door properties
    pub locked: bool,
    pub trapped: bool,
    pub secret: bool,
    pub magical: bool,
    
    #[sea_orm(column_type = "Text", nullable)]
    pub unlock_method: Option<String>, // How to unlock if locked
    
    #[sea_orm(column_type = "Text", nullable)]
    pub trap_description: Option<String>, // Description of trap if present
    
    // Discovery state
    pub discovered: bool,
    pub opened: bool,
    pub trap_triggered: bool,
    
    pub created_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = "super::rooms::Entity", from = "Column::RoomId", to = "super::rooms::Column::Id")]
    Room,
}

impl Related<super::rooms::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Room.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
