//! Item models for sophisticated inventory and equipment system

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Sentimental items and equipment with horror progression integration
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "items")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    
    pub player_id: Option<Uuid>, // Who owns this item (if owned)
    pub companion_id: Option<Uuid>, // Which companion gave this (if applicable)
    
    // Basic item data
    #[sea_orm(column_type = "Text")]
    pub name: String,
    
    #[sea_orm(column_type = "Text")]
    pub description: String,
    
    #[sea_orm(column_type = "Text")]
    pub item_type: String, // "sentimental", "weapon", "armor", "consumable", "key"
    
    #[sea_orm(column_type = "Text")]
    pub rarity: String, // "common", "uncommon", "rare", "legendary", "cursed"
    
    // Sentimental value system (core to horror progression)
    pub sentimental_value: f32, // 0.0-1.0 how emotionally significant
    
    #[sea_orm(column_type = "Text", nullable)]
    pub memory_description: Option<String>, // Why this item matters
    
    #[sea_orm(column_type = "Text", nullable)]
    pub emotional_category: Option<String>, // "love", "loss", "friendship", "fear", "hope"
    
    #[sea_orm(column_type = "Text", nullable)]
    pub acquired_context: Option<String>, // Story of how player got this
    
    // Horror corruption system
    pub corruption_level: f32, // 0.0-1.0 how corrupted by horror
    pub dread_resonance: i32, // Which dread level (0-4) this item resonates with
    
    // Forge system integration
    pub forge_reagent_power: f32, // Power when used as reagent
    pub light_path_compatibility: f32, // 0.0-1.0 for High Elves forge
    pub dark_path_compatibility: f32, // 0.0-1.0 for Cursed forge
    pub essence_vs_blood_ratio: f32, // -1.0 to 1.0 (essence to blood)
    
    // Game mechanics
    pub stack_size: i32, // How many can be stacked
    pub durability: Option<f32>, // Current durability (if applicable)
    pub max_durability: Option<f32>, // Maximum durability
    
    // Asset references
    #[sea_orm(column_type = "Text", nullable)]
    pub icon_asset_id: Option<String>, // Reference to asset in game-assets
    
    #[sea_orm(column_type = "Text", nullable)]
    pub model_asset_id: Option<String>, // 3D model reference
    
    // Timestamps
    pub created_at: DateTime<Utc>,
    pub last_modified: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::players::Entity",
        from = "Column::PlayerId",
        to = "super::players::Column::Id"
    )]
    Player,
    
    #[sea_orm(
        belongs_to = "super::companions::Entity", 
        from = "Column::CompanionId",
        to = "super::companions::Column::Id"
    )]
    Companion,
}

impl Related<super::players::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Player.def()
    }
}

impl Related<super::companions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Companion.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
