//! Mount system models with witness mechanics

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Living mount companions that witness moral journey
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "mount_companions")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    
    pub player_id: Uuid,
    pub companion_id: Option<Uuid>,      // If mount is also a companion
    
    // Mount identity
    pub mount_type: String,              // "horse", "wolf", "corrupted_steed", etc.
    pub name: String,
    pub display_name: String,
    pub bond_level: f32,                 // 0.0-1.0 bond with player
    
    // Mount capabilities
    pub movement_speed: f32,             // Speed bonus when mounted
    pub carrying_capacity: f32,          // Inventory bonus
    pub terrain_bonuses: Json,           // Terrain-specific advantages
    pub special_abilities: Json,         // Mount-specific abilities
    
    // Witness system - mount sees player's moral choices
    pub moral_witness_score: f32,        // -1.0 to 1.0 (horrified to inspired)
    pub witnessed_events: Json,          // Events mount has witnessed
    pub moral_judgment_level: f32,       // How much mount judges player
    pub emotional_response_to_player: String, // Current feeling toward player
    
    // Trauma and corruption response
    pub trauma_from_witnessing: f32,     // 0.0-1.0 trauma from seeing player's actions
    pub corruption_resistance: f32,      // Resistance to environmental corruption
    pub corruption_level: f32,           // How corrupted mount has become
    pub can_flee_in_horror: bool,        // Will mount abandon player?
    
    // Environmental protection
    pub provides_protection_from: Json,  // Environmental hazards mount protects from
    pub protection_effectiveness: f32,   // 0.0-1.0 how well mount protects
    pub required_for_biomes: Json,       // Biomes that require this mount
    
    // Mount availability and status
    pub is_available: bool,              // Can be summoned/used
    pub current_location: Option<String>, // Where mount is if not with player
    pub fled_in_horror: bool,            // Has mount fled due to player's actions?
    pub corruption_breaking_point: f32,  // Corruption level where mount flees
    
    // Recovery and relationship repair
    pub can_relationship_be_repaired: bool, // Can bond be restored?
    pub repair_requirements: Json,       // What's needed to repair relationship
    pub forgiveness_potential: f32,      // 0.0-1.0 likelihood of forgiveness
    
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
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

impl ActiveModelBehavior for ActiveModel {}
