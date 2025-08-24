//! Settlement models for taverns, inns, shops, and other civilized locations

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Settlements - taverns, inns, shops, temples, and other civilized locations
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "settlements")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    
    // Basic settlement data
    #[sea_orm(column_type = "Text")]
    pub name: String, // "The Flaming Torch Tavern", "Moonlight Inn"
    
    #[sea_orm(column_type = "Text")]
    pub settlement_type: String, // "tavern", "inn", "shop", "temple", "village", "outpost"
    
    // Location and HBF import data
    pub hex_tile_id: Option<Uuid>, // Reference to hex tile
    #[sea_orm(column_type = "Text", nullable)]
    pub hbf_uuid: Option<String>, // Original HBF UUID if imported
    pub hbf_x: Option<i32>, // Original HBF coordinates
    pub hbf_y: Option<i32>,
    
    // Settlement details
    #[sea_orm(column_type = "Text")]
    pub description: String, // Rich description of the settlement
    
    pub population: Option<i32>, // Estimated population
    pub prosperity_level: i32, // 0-10 how prosperous the settlement is
    pub corruption_influence: f32, // 0.0-1.0 how corrupted this place is
    
    // Services and features
    #[sea_orm(column_type = "Json", nullable)]
    pub services: Option<serde_json::Value>, // JSON array of available services
    
    #[sea_orm(column_type = "Json", nullable)]
    pub notable_features: Option<serde_json::Value>, // Special features of this settlement
    
    #[sea_orm(column_type = "Json", nullable)]
    pub rumors: Option<serde_json::Value>, // Rumors available here
    
    // Economic data
    #[sea_orm(column_type = "Json", nullable)]
    pub trade_goods: Option<serde_json::Value>, // What can be bought/sold here
    
    #[sea_orm(column_type = "Json", nullable)]
    pub price_modifiers: Option<serde_json::Value>, // Price adjustments for different goods
    
    // Safety and reputation
    pub safety_rating: i32, // 0-10 how safe this place is
    pub reputation: i32, // -10 to +10 how well regarded this place is
    
    // Weather and environmental
    #[sea_orm(column_type = "Json", nullable)]
    pub weather_data: Option<serde_json::Value>, // Regional weather tables if applicable
    
    // Relationships and politics
    #[sea_orm(column_type = "Text", nullable)]
    pub faction: Option<String>, // What faction controls this settlement
    
    #[sea_orm(column_type = "Json", nullable)]
    pub relationships: Option<serde_json::Value>, // Relations with other settlements/factions
    
    // Horror progression integration
    pub dread_level_effects: i32, // How this place changes with dread 0-4
    #[sea_orm(column_type = "Text", nullable)]
    pub corrupted_description: Option<String>, // Description when world is corrupted
    
    // Companion interactions
    #[sea_orm(column_type = "Json", nullable)]
    pub companion_reactions: Option<serde_json::Value>, // How companions react to this place
    
    // Timestamps and state
    pub discovered: bool, // Has player found this settlement?
    pub first_visited_at: Option<DateTime<Utc>>,
    pub last_visited_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub last_modified: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = "super::hex_tiles::Entity", from = "Column::HexTileId", to = "super::hex_tiles::Column::Id")]
    HexTile,
    #[sea_orm(has_many = "super::npcs::Entity")]
    Npcs,
}

impl Related<super::hex_tiles::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::HexTile.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

pub mod weather;
