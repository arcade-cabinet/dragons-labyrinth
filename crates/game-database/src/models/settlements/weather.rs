//! Weather models for settlements

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Weather tables for settlements with regional weather data
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "settlement_weather")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    
    pub settlement_id: Uuid, // Reference to parent settlement
    
    #[sea_orm(column_type = "Text")]
    pub season: String, // "dry", "wet", "spring", "summer", "fall", "winter"
    
    #[sea_orm(column_type = "Text")]
    pub dice_roll: String, // "2", "3-4", "5-9", "10-11", "12"
    
    #[sea_orm(column_type = "Text")]
    pub weather_condition: String, // "Clear", "Rainy", "Stormy", "Foggy"
    
    #[sea_orm(column_type = "Json", nullable)]
    pub mechanical_effects: Option<serde_json::Value>, // Game mechanical effects
    
    // Special weather effects
    #[sea_orm(column_type = "Text", nullable)]
    pub flood_chance: Option<String>, // "1-in-6 chance once a week"
    
    #[sea_orm(column_type = "Json", nullable)]
    pub horror_weather_variants: Option<serde_json::Value>, // Weather changes with corruption
    
    pub created_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = "super::Entity", from = "Column::SettlementId", to = "super::Column::Id")]
    Settlement,
}

impl Related<super::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Settlement.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
