//! Environmental decay models for world corruption system

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Detailed world corruption tracking - world literally responds to player
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "world_corruption")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    
    pub player_id: Uuid,
    pub hex_tile_id: Option<Uuid>,       // Specific tile affected (or global)
    
    // Corruption progression
    pub corruption_type: String,         // "visual", "economic", "social", "physical", "psychological"
    pub corruption_intensity: f32,       // 0.0-1.0 how severe
    pub corruption_spread_rate: f32,     // How fast it spreads to nearby areas
    pub corruption_source: String,       // What caused this corruption
    
    // Visual corruption effects
    pub color_desaturation: f32,         // 0.0-1.0 how much color is drained
    pub shadow_lengthening: f32,         // How much shadows have grown
    pub texture_degradation: f32,        // Visual decay of textures
    pub lighting_dimming: f32,           // How much light has been lost
    
    // Social corruption effects
    pub npc_fear_level: f32,             // 0.0-1.0 how afraid NPCs are of player
    pub social_isolation_level: f32,     // How isolated player has become
    pub doors_locked_count: i32,         // How many doors NPCs lock when player approaches
    pub npc_flee_distance: f32,          // Distance NPCs maintain from player
    
    // Economic corruption effects
    pub gold_value_degradation: f32,     // How worthless gold has become
    pub survival_item_inflation: f32,    // How valuable survival items have become
    pub trade_network_collapse: f32,     // 0.0-1.0 economic system breakdown
    pub merchant_availability: f32,      // How many merchants still trade
    
    // Physical world corruption
    pub environmental_decay: f32,        // Physical decay of world
    pub weather_corruption: Json,        // Corrupted weather patterns
    pub natural_cycles_disruption: f32,  // How much natural order is broken
    pub ecological_collapse: f32,        // Ecosystem breakdown level
    
    // Psychological corruption effects
    pub reality_distortion_level: f32,   // How much reality is warped
    pub false_audio_frequency: f32,      // Rate of hallucinatory sounds
    pub false_visual_frequency: f32,     // Rate of hallucinatory visuals
    pub sanity_drain_rate: f32,          // Passive sanity loss in this area
    
    // Corruption resistance and recovery
    pub natural_resistance: f32,         // Area's resistance to corruption
    pub recovery_potential: f32,         // Can this area heal?
    pub purification_requirements: Json, // What would purify this area
    pub corruption_permanence: f32,      // 0.0-1.0 how permanent corruption is
    
    // Corruption spread mechanics
    pub spreads_to_adjacent: bool,       // Does corruption spread to nearby tiles?
    pub spread_conditions: Json,         // Conditions for corruption spread
    pub corruption_vectors: Json,        // How corruption moves through world
    pub containment_status: String,      // "spreading", "contained", "accelerating"
    
    pub corruption_started: DateTime<Utc>,
    pub last_corruption_update: DateTime<Utc>,
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
}

impl Related<super::players::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Player.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
