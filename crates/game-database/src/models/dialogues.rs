//! Dialogue models for sophisticated conversation system

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Dialogue lines and conversation trees with horror progression integration
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "dialogues")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    
    pub encounter_id: Option<Uuid>, // Which encounter this dialogue belongs to
    pub parent_dialogue_id: Option<Uuid>, // Previous dialogue in conversation tree
    pub speaker_companion_id: Option<Uuid>, // Which companion is speaking (if any)
    
    // Dialogue content
    #[sea_orm(column_type = "Text")]
    pub speaker_name: String, // Who is speaking ("Player", "Einar", "Stranger", etc.)
    
    #[sea_orm(column_type = "Text")]
    pub dialogue_text: String, // The actual spoken text
    
    #[sea_orm(column_type = "Text")]
    pub speaker_emotion: String, // "calm", "fearful", "angry", "sad", "hopeful", "corrupted"
    
    // Dialogue tree structure
    pub sequence_order: i32, // Order in conversation sequence
    pub is_player_choice: bool, // Is this a player dialogue option?
    pub is_available: bool, // Can this dialogue option be selected?
    
    // Conditional requirements  
    pub required_dread_level_min: Option<i32>, // Min dread to see this dialogue
    pub required_dread_level_max: Option<i32>, // Max dread to see this dialogue
    
    #[sea_orm(column_type = "Json", nullable)]
    pub required_items: Option<serde_json::Value>, // JSON array of required items
    
    #[sea_orm(column_type = "Json", nullable)]
    pub required_story_flags: Option<serde_json::Value>, // JSON array of required flags
    
    #[sea_orm(column_type = "Json", nullable)]
    pub required_companion_states: Option<serde_json::Value>, // Companion relationship requirements
    
    // Narrative consequences
    pub dread_level_change: f32, // -1.0 to 1.0 change to player's dread
    pub corruption_change: f32, // -1.0 to 1.0 change to corruption level
    
    #[sea_orm(column_type = "Json", nullable)]
    pub story_flags_set: Option<serde_json::Value>, // JSON array of flags to set
    
    #[sea_orm(column_type = "Json", nullable)]
    pub story_flags_unset: Option<serde_json::Value>, // JSON array of flags to unset
    
    #[sea_orm(column_type = "Json", nullable)]
    pub companion_relationship_changes: Option<serde_json::Value>, // Changes to companion bonds
    
    // Item interactions
    #[sea_orm(column_type = "Json", nullable)]
    pub items_given: Option<serde_json::Value>, // JSON array of items given to player
    
    #[sea_orm(column_type = "Json", nullable)]
    pub items_taken: Option<serde_json::Value>, // JSON array of items taken from player
    
    // Horror progression effects
    pub psychological_impact: f32, // Impact on companion psychological state
    pub trauma_trigger: bool, // Does this dialogue trigger trauma response?
    pub memory_fragment_created: bool, // Does this create a traumatic memory fragment?
    
    #[sea_orm(column_type = "Text", nullable)]
    pub trauma_description: Option<String>, // Description of trauma if triggered
    
    // Forge system integration
    pub sentimental_value_created: f32, // 0.0-1.0 sentimental attachment created
    pub light_essence_impact: f32, // Impact on light essence (-1.0 to 1.0)
    pub dark_essence_impact: f32, // Impact on dark essence (-1.0 to 1.0)
    
    // Audio/Visual presentation
    #[sea_orm(column_type = "Text", nullable)]
    pub voice_line_asset_id: Option<String>, // Reference to generated voice line
    
    #[sea_orm(column_type = "Text", nullable)]
    pub speaker_portrait_id: Option<String>, // Speaker portrait asset
    
    #[sea_orm(column_type = "Text", nullable)]
    pub background_music_id: Option<String>, // Music change for this line
    
    #[sea_orm(column_type = "Text", nullable)]
    pub sound_effect_id: Option<String>, // Sound effect to play
    
    // Animation and presentation cues
    #[sea_orm(column_type = "Text", nullable)]
    pub speaker_animation: Option<String>, // Animation cue for speaker
    
    #[sea_orm(column_type = "Text", nullable)]
    pub camera_direction: Option<String>, // Camera movement instruction
    
    #[sea_orm(column_type = "Text", nullable)]
    pub environmental_effect: Option<String>, // Environmental change during dialogue
    
    // Timestamps
    pub created_at: DateTime<Utc>,
    pub last_modified: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::encounters::Entity",
        from = "Column::EncounterId",
        to = "super::encounters::Column::Id"
    )]
    Encounter,
    
    #[sea_orm(
        belongs_to = "Entity", 
        from = "Column::ParentDialogueId",
        to = "Column::Id"
    )]
    ParentDialogue,
    
    #[sea_orm(
        belongs_to = "super::companions::Entity",
        from = "Column::SpeakerCompanionId",
        to = "super::companions::Column::Id"
    )]
    SpeakerCompanion,
}

impl Related<super::encounters::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Encounter.def()
    }
}

impl Related<super::companions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SpeakerCompanion.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
