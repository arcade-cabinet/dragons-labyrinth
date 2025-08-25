//! Encounter models for sophisticated narrative interactions

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Encounters with NPCs, environments, and narrative situations
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "encounters")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    
    pub hex_tile_id: Option<Uuid>, // Where this encounter occurs
    pub player_id: Option<Uuid>, // Which player this encounter is for (instance data)
    
    // Encounter metadata
    #[sea_orm(column_type = "Text")]
    pub encounter_type: String, // "npc_dialogue", "environmental", "combat", "discovery", "horror_event"
    
    #[sea_orm(column_type = "Text")]
    pub encounter_name: String,
    
    #[sea_orm(column_type = "Text")]
    pub description: String,
    
    // Narrative requirements
    pub required_dread_level_min: Option<i32>, // Min dread level to trigger
    pub required_dread_level_max: Option<i32>, // Max dread level to trigger
    
    #[sea_orm(column_type = "Json", nullable)]
    pub required_items: Option<serde_json::Value>, // JSON array of required item IDs
    
    #[sea_orm(column_type = "Json", nullable)]
    pub required_companion_states: Option<serde_json::Value>, // JSON object of companion requirements
    
    #[sea_orm(column_type = "Json", nullable)]
    pub required_story_flags: Option<serde_json::Value>, // JSON array of required story progression flags
    
    // Encounter state and availability
    pub is_available: bool, // Can this encounter be triggered?
    pub is_repeatable: bool, // Can encounter happen multiple times?
    pub times_encountered: i32, // How many times player has had this encounter
    pub first_encountered_at: Option<DateTime<Utc>>, // When first encountered
    pub last_encountered_at: Option<DateTime<Utc>>, // Most recent encounter
    
    // Horror progression integration
    pub dread_level_impact: f32, // -1.0 to 1.0 change to dread level
    pub corruption_impact: f32, // -1.0 to 1.0 change to corruption
    pub companion_trauma_impact: f32, // Impact on companion psychological states
    
    // Dialogue and narrative content
    #[sea_orm(column_type = "Text", nullable)]
    pub dialogue_tree_id: Option<String>, // Reference to dialogue system
    
    #[sea_orm(column_type = "Json", nullable)]
    pub dialogue_options: Option<serde_json::Value>, // JSON array of dialogue choices
    
    #[sea_orm(column_type = "Json", nullable)]
    pub narrative_outcomes: Option<serde_json::Value>, // JSON object of possible outcomes
    
    // Rewards and consequences
    #[sea_orm(column_type = "Json", nullable)]
    pub item_rewards: Option<serde_json::Value>, // JSON array of items that can be gained
    
    #[sea_orm(column_type = "Json", nullable)]
    pub story_flag_changes: Option<serde_json::Value>, // JSON object of story flags that change
    
    #[sea_orm(column_type = "Json", nullable)]
    pub companion_relationship_changes: Option<serde_json::Value>, // Impact on companion bonds
    
    // Environmental storytelling
    #[sea_orm(column_type = "Text", nullable)]
    pub atmospheric_description: Option<String>, // Rich environmental description
    
    #[sea_orm(column_type = "Text", nullable)]
    pub horror_description: Option<String>, // Description when horror-influenced
    
    // Asset references for rich presentation
    #[sea_orm(column_type = "Text", nullable)]
    pub background_image_id: Option<String>, // Visual background for encounter
    
    #[sea_orm(column_type = "Text", nullable)]
    pub character_portrait_id: Option<String>, // NPC portrait if applicable
    
    #[sea_orm(column_type = "Text", nullable)]
    pub ambient_audio_id: Option<String>, // Background audio
    
    #[sea_orm(column_type = "Text", nullable)]
    pub music_track_id: Option<String>, // Music for this encounter
    
    // Forge system integration
    pub light_essence_gain: f32, // Light essence gained from this encounter  
    pub dark_essence_gain: f32, // Dark essence gained from this encounter
    pub sentimental_item_chance: f32, // 0.0-1.0 chance to create sentimental item
    
    // Timestamps
    pub created_at: DateTime<Utc>,
    pub last_modified: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::hex_tiles::Entity",
        from = "Column::HexTileId", 
        to = "super::hex_tiles::Column::Id"
    )]
    HexTile,
    
    #[sea_orm(
        belongs_to = "super::players::Entity",
        from = "Column::PlayerId",
        to = "super::players::Column::Id" 
    )]
    Player,
    
    #[sea_orm(has_many = "super::dialogues::Entity")]
    Dialogues,
}

impl Related<super::hex_tiles::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::HexTile.def()
    }
}

impl Related<super::players::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Player.def()
    }
}

impl Related<super::dialogues::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Dialogues.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
