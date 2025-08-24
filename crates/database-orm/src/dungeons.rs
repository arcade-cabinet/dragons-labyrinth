//! Dungeon models for complex multi-room dungeons and labyrinth structures

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

pub mod rooms;
pub mod doorways;

/// Dungeons - complex multi-room structures like crypts, ruins, and labyrinths
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "dungeons")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    
    // Basic dungeon data
    #[sea_orm(column_type = "Text")]
    pub name: String, // "Crypt of the Corrupted Order", "Ancient Temple Ruins"
    
    #[sea_orm(column_type = "Text")]
    pub dungeon_type: String, // "crypt", "ruins", "temple", "labyrinth", "cave_system"
    
    // Location and HBF import data
    pub hex_tile_id: Option<Uuid>, // Reference to hex tile
    #[sea_orm(column_type = "Text", nullable)]
    pub hbf_uuid: Option<String>, // Original HBF UUID if imported
    pub hbf_x: Option<i32>, // Original HBF coordinates
    pub hbf_y: Option<i32>,
    
    // Dungeon details
    #[sea_orm(column_type = "Text")]
    pub description: String, // Rich description of the dungeon
    
    pub total_rooms: i32, // Total number of rooms/areas
    pub levels: i32, // How many levels deep (1 for single level)
    pub estimated_size: String, // "small", "medium", "large", "massive"
    
    // Difficulty and danger
    pub danger_level: i32, // 0-10 how dangerous this dungeon is
    pub recommended_level: i32, // Recommended player level
    pub corruption_intensity: f32, // 0.0-1.0 how corrupted this dungeon is
    
    // Dungeon features
    #[sea_orm(column_type = "Json", nullable)]
    pub themes: Option<serde_json::Value>, // JSON array of dungeon themes
    
    #[sea_orm(column_type = "Json", nullable)]
    pub special_features: Option<serde_json::Value>, // Teleportals, secret passages, etc.
    
    #[sea_orm(column_type = "Json", nullable)]
    pub environmental_hazards: Option<serde_json::Value>, // Traps, environmental dangers
    
    // Loot and rewards
    #[sea_orm(column_type = "Json", nullable)]
    pub treasure_hints: Option<serde_json::Value>, // General treasure information
    
    #[sea_orm(column_type = "Json", nullable)]
    pub boss_encounters: Option<serde_json::Value>, // Major encounters in this dungeon
    
    // Narrative integration
    #[sea_orm(column_type = "Text", nullable)]
    pub lore: Option<String>, // Historical background
    
    #[sea_orm(column_type = "Json", nullable)]
    pub story_connections: Option<serde_json::Value>, // How this connects to main story
    
    // Horror progression
    pub dread_level_effects: i32, // How this dungeon changes with dread 0-4
    #[sea_orm(column_type = "Text", nullable)]
    pub corrupted_description: Option<String>, // Description when world is corrupted
    
    // Exploration state
    pub discovered: bool, // Has player found this dungeon entrance?
    pub partially_explored: bool, // Has player entered but not completed?
    pub fully_cleared: bool, // Has player cleared all rooms?
    pub first_entered_at: Option<DateTime<Utc>>,
    pub last_visited_at: Option<DateTime<Utc>>,
    pub cleared_at: Option<DateTime<Utc>>,
    
    // Timestamps
    pub created_at: DateTime<Utc>,
    pub last_modified: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = "super::hex_tiles::Entity", from = "Column::HexTileId", to = "super::hex_tiles::Column::Id")]
    HexTile,
    #[sea_orm(has_many = "rooms::Entity")]
    Rooms,
}

impl Related<super::hex_tiles::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::HexTile.def()
    }
}

impl Related<rooms::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Rooms.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
