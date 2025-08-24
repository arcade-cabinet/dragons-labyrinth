//! Hex tile models for sophisticated world state management

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Hex tiles representing the game world with sophisticated corruption and discovery
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "hex_tiles")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    
    // Hex coordinate system
    pub q: i32, // Hex q coordinate
    pub r: i32, // Hex r coordinate 
    pub s: i32, // Hex s coordinate (q + r + s = 0)
    
    // Tile data
    #[sea_orm(column_type = "Text")]
    pub biome_type: String, // "forest", "mountain", "swamp", "ruins", "village", "cursed_ground"
    
    #[sea_orm(column_type = "Text")]
    pub tile_variant: String, // Specific variant within biome
    
    // Horror corruption system
    pub corruption_level: f32, // 0.0-1.0 environmental corruption
    pub dread_intensity: i32, // 0-4 how much this tile contributes to dread
    pub horror_events_count: i32, // How many horror events occurred here
    
    // Discovery and exploration
    pub discovered: bool, // Has player discovered this tile?
    pub fully_explored: bool, // Has player fully explored this tile?
    pub first_discovered_at: Option<DateTime<Utc>>, // When first discovered
    pub last_visited_at: Option<DateTime<Utc>>, // Last time player was here
    
    // Tile features and resources
    #[sea_orm(column_type = "Json", nullable)]
    pub features: Option<serde_json::Value>, // JSON array of features on this tile
    
    #[sea_orm(column_type = "Json", nullable)]
    pub resources: Option<serde_json::Value>, // JSON object of available resources
    
    #[sea_orm(column_type = "Json", nullable)]
    pub encounters: Option<serde_json::Value>, // JSON array of available encounters
    
    // Environmental storytelling
    #[sea_orm(column_type = "Text", nullable)]
    pub atmospheric_description: Option<String>, // Rich atmospheric text
    
    #[sea_orm(column_type = "Text", nullable)]
    pub horror_description: Option<String>, // Description when corrupted
    
    // NPC and companion presence
    #[sea_orm(column_type = "Json", nullable)]
    pub npcs_present: Option<serde_json::Value>, // JSON array of NPC IDs present
    
    #[sea_orm(column_type = "Json", nullable)]
    pub companion_memories: Option<serde_json::Value>, // Companion memory fragments from this tile
    
    // Forge system integration 
    pub light_essence_strength: f32, // How much light essence emanates from here
    pub dark_essence_strength: f32, // How much dark essence emanates from here
    pub essence_stability: f32, // How stable the essence is (0.0-1.0)
    
    // Asset references
    #[sea_orm(column_type = "Text", nullable)]
    pub tile_asset_id: Option<String>, // Reference to tile texture/model
    
    #[sea_orm(column_type = "Text", nullable)]
    pub ambient_audio_id: Option<String>, // Reference to ambient audio
    
    // Weather and time-of-day effects
    #[sea_orm(column_type = "Json", nullable)]
    pub weather_modifiers: Option<serde_json::Value>, // How weather affects this tile
    
    #[sea_orm(column_type = "Json", nullable)]
    pub time_of_day_effects: Option<serde_json::Value>, // Day/night differences
    
    // Timestamps
    pub created_at: DateTime<Utc>,
    pub last_modified: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::encounters::Entity")]
    Encounters,
}

impl Related<super::encounters::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Encounters.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
