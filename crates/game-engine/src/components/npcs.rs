//! NPC models for non-player characters in settlements and encounters

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// NPCs - Non-player characters that can be found throughout the world
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "npcs")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    
    // Basic NPC data
    #[sea_orm(column_type = "Text")]
    pub name: String, // "Gareth the Innkeeper", "Old Woman Martha"
    
    #[sea_orm(column_type = "Text")]
    pub role: String, // "innkeeper", "merchant", "guard", "sage", "traveler", "hermit"
    
    #[sea_orm(column_type = "Text")]
    pub race: String, // "human", "elf", "dwarf", "halfling", "other"
    
    // Location data
    pub hex_tile_id: Option<Uuid>, // What hex tile they're on
    pub settlement_id: Option<Uuid>, // What settlement they belong to
    pub dungeon_id: Option<Uuid>, // What dungeon they're in (if any)
    
    // HBF import data
    #[sea_orm(column_type = "Text", nullable)]
    pub hbf_uuid: Option<String>, // Original HBF UUID if imported
    pub hbf_x: Option<i32>, // Original HBF coordinates
    pub hbf_y: Option<i32>,
    
    // Character description
    #[sea_orm(column_type = "Text")]
    pub description: String, // Physical description and mannerisms
    
    #[sea_orm(column_type = "Text", nullable)]
    pub personality: Option<String>, // Personality traits and quirks
    
    #[sea_orm(column_type = "Text", nullable)]
    pub background: Option<String>, // Personal history and background
    
    // Stats and abilities (if relevant for encounters)
    pub level: Option<i32>, // Character level (if combatant)
    pub hit_points: Option<i32>, // HP for combat encounters
    pub armor_class: Option<i32>, // AC for combat
    
    #[sea_orm(column_type = "Json", nullable)]
    pub ability_scores: Option<serde_json::Value>, // STR, DEX, CON, INT, WIS, CHA
    
    #[sea_orm(column_type = "Json", nullable)]
    pub equipment: Option<serde_json::Value>, // What they carry/wear
    
    // Social interactions
    pub disposition: i32, // -10 to +10 how they feel about the player
    pub reputation_awareness: i32, // 0-10 how much they know about player's reputation
    
    #[sea_orm(column_type = "Json", nullable)]
    pub dialogue_options: Option<serde_json::Value>, // Available conversation topics
    
    #[sea_orm(column_type = "Json", nullable)]
    pub rumors_known: Option<serde_json::Value>, // What rumors they might share
    
    #[sea_orm(column_type = "Json", nullable)]
    pub services_offered: Option<serde_json::Value>, // What services they provide
    
    // Economic data
    #[sea_orm(column_type = "Json", nullable)]
    pub trade_goods: Option<serde_json::Value>, // What they buy/sell
    
    #[sea_orm(column_type = "Json", nullable)]
    pub price_modifiers: Option<serde_json::Value>, // Their pricing adjustments
    
    pub wealth_level: i32, // 0-10 how wealthy they are
    
    // Relationships
    #[sea_orm(column_type = "Json", nullable)]
    pub relationships: Option<serde_json::Value>, // Relations with other NPCs/factions
    
    #[sea_orm(column_type = "Text", nullable)]
    pub faction: Option<String>, // What faction they belong to
    
    // Behavior and AI
    #[sea_orm(column_type = "Text")]
    pub behavior_type: String, // "friendly", "neutral", "hostile", "trader", "guard"
    
    #[sea_orm(column_type = "Json", nullable)]
    pub daily_schedule: Option<serde_json::Value>, // Where they go at different times
    
    #[sea_orm(column_type = "Json", nullable)]
    pub interaction_triggers: Option<serde_json::Value>, // What causes special interactions
    
    // Horror progression integration
    pub corruption_susceptibility: f32, // 0.0-1.0 how easily they're corrupted
    pub current_corruption_level: f32, // 0.0-1.0 current corruption
    pub dread_level_effects: i32, // How they change with world dread 0-4
    
    #[sea_orm(column_type = "Text", nullable)]
    pub corrupted_description: Option<String>, // How they appear when corrupted
    
    #[sea_orm(column_type = "Json", nullable)]
    pub corruption_triggers: Option<serde_json::Value>, // What causes their corruption
    
    // Companion interactions
    #[sea_orm(column_type = "Json", nullable)]
    pub companion_reactions: Option<serde_json::Value>, // How companions react to this NPC
    
    #[sea_orm(column_type = "Json", nullable)]
    pub companion_memories: Option<serde_json::Value>, // Companion memories involving this NPC
    
    // Quest and story integration
    #[sea_orm(column_type = "Json", nullable)]
    pub quest_connections: Option<serde_json::Value>, // Quests they're involved in
    
    #[sea_orm(column_type = "Json", nullable)]
    pub story_importance: Option<serde_json::Value>, // Their role in the main story
    
    // State tracking
    pub alive: bool, // Is this NPC still alive?
    pub encountered: bool, // Has player met this NPC?
    pub first_met_at: Option<DateTime<Utc>>,
    pub last_interaction_at: Option<DateTime<Utc>>,
    pub times_interacted: i32, // How many times player has talked to them
    
    // Movement and location
    pub mobile: bool, // Does this NPC move around?
    #[sea_orm(column_type = "Json", nullable)]
    pub movement_pattern: Option<serde_json::Value>, // How they move (if mobile)
    
    // Timestamps
    pub created_at: DateTime<Utc>,
    pub last_modified: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = "super::hex_tiles::Entity", from = "Column::HexTileId", to = "super::hex_tiles::Column::Id")]
    HexTile,
    #[sea_orm(belongs_to = "super::settlements::Entity", from = "Column::SettlementId", to = "super::settlements::Column::Id")]
    Settlement,
    #[sea_orm(belongs_to = "super::dungeons::Entity", from = "Column::DungeonId", to = "super::dungeons::Column::Id")]
    Dungeon,
}

impl Related<super::hex_tiles::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::HexTile.def()
    }
}

impl Related<super::settlements::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Settlement.def()
    }
}

impl Related<super::dungeons::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Dungeon.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
