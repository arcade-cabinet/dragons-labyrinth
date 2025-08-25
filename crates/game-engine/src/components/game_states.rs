//! Game state models for save/load and world persistence

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Game state snapshots for save/load functionality and world persistence
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "game_states")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    
    pub player_id: Uuid, // Which player this save belongs to
    
    // Save slot information
    pub save_slot_number: i32, // Which save slot (1-10)
    pub is_autosave: bool, // Is this an automatic save?
    pub is_checkpoint: bool, // Is this a story checkpoint save?
    
    #[sea_orm(column_type = "Text")]
    pub save_name: String, // Player-given name for save
    
    #[sea_orm(column_type = "Text", nullable)]
    pub save_description: Option<String>, // Optional description
    
    // World state at time of save
    pub current_dread_level: i32, // 0-4 dread level at save time
    pub current_corruption_level: f32, // 0.0-1.0 corruption at save time
    pub current_hex_tile_q: i32, // Player position q coordinate
    pub current_hex_tile_r: i32, // Player position r coordinate
    pub current_hex_tile_s: i32, // Player position s coordinate
    
    // Player character state
    #[sea_orm(column_type = "Json")]
    pub player_character_data: serde_json::Value, // Complete player character snapshot
    
    #[sea_orm(column_type = "Json")]
    pub inventory_snapshot: serde_json::Value, // Player inventory state
    
    #[sea_orm(column_type = "Json")]
    pub companion_states_snapshot: serde_json::Value, // All companion states
    
    // World exploration progress
    #[sea_orm(column_type = "Json")]
    pub discovered_tiles: serde_json::Value, // JSON array of discovered hex tile coordinates
    
    #[sea_orm(column_type = "Json")]
    pub completed_encounters: serde_json::Value, // JSON array of completed encounter IDs
    
    #[sea_orm(column_type = "Json")]
    pub active_encounters: serde_json::Value, // JSON array of currently available encounters
    
    // Story progression state
    #[sea_orm(column_type = "Json")]
    pub story_flags: serde_json::Value, // JSON object of all story progression flags
    
    #[sea_orm(column_type = "Json")]
    pub dialogue_history: serde_json::Value, // JSON array of completed dialogue trees
    
    pub current_story_chapter: i32, // Which chapter of story player is in
    
    #[sea_orm(column_type = "Text", nullable)]
    pub current_story_beat: Option<String>, // Current narrative beat/checkpoint
    
    // Companion relationship states
    #[sea_orm(column_type = "Json")]
    pub companion_relationship_levels: serde_json::Value, // Trust/bond levels with each companion
    
    #[sea_orm(column_type = "Json")]
    pub companion_trauma_states: serde_json::Value, // Psychological state of each companion
    
    #[sea_orm(column_type = "Json")]
    pub companion_memory_fragments: serde_json::Value, // Traumatic memories for companions
    
    // Forge system state
    pub light_essence_amount: f32, // Current light essence
    pub dark_essence_amount: f32, // Current dark essence
    pub forge_unlock_progress: f32, // 0.0-1.0 progress toward unlocking forge
    pub high_elf_relationship_level: f32, // Relationship with High Elves
    pub cursed_entity_relationship_level: f32, // Relationship with Cursed entities
    
    #[sea_orm(column_type = "Json")]
    pub forged_items_history: serde_json::Value, // JSON array of items forged so far
    
    // Environmental state
    #[sea_orm(column_type = "Json")]
    pub world_corruption_map: serde_json::Value, // JSON object mapping tile coords to corruption levels
    
    #[sea_orm(column_type = "Json")]
    pub active_environmental_effects: serde_json::Value, // Current weather, time of day, etc.
    
    pub global_horror_intensity: f32, // 0.0-1.0 overall world horror level
    
    // Statistics snapshot
    #[sea_orm(column_type = "Json")]
    pub player_statistics_snapshot: serde_json::Value, // Complete statistics at save time
    
    // Technical metadata
    pub game_version: String, // Version of game when save was created
    pub save_format_version: i32, // Version of save format used
    
    #[sea_orm(column_type = "Text", nullable)]
    pub mod_list: Option<String>, // List of active mods (if any)
    
    // Validation and integrity
    #[sea_orm(column_type = "Text")]
    pub save_checksum: String, // Checksum to verify save integrity
    
    pub total_playtime_seconds: i64, // Total playtime at save creation
    
    // Timestamps
    pub saved_at: DateTime<Utc>, // When save was created
    pub game_time_when_saved: DateTime<Utc>, // In-game time when saved (for day/night cycles)
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::players::Entity",
        from = "Column::PlayerId",
        to = "super::players::Column::Id"
    )]
    Player,
}

impl Related<super::players::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Player.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
