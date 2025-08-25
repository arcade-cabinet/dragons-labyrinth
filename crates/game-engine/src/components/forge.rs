//! Forge system models for sophisticated dual-path morality system

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Sentimental items collected throughout game as forge reagents
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "sentimental_items")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    
    pub player_id: Uuid,
    pub item_id: Uuid,
    
    // Sentimental value system
    pub memory_description: String,       // Why this item is meaningful
    pub emotional_weight: f32,           // 0.0-1.0 how much this matters to player
    pub sentimental_category: String,    // "friendship", "love", "loss", "hope", "fear"
    pub acquired_story_context: String, // When/where player got this item
    
    // Forge integration
    pub forge_reagent_power: f32,       // Power when used as reagent
    pub light_path_compatibility: f32,  // 0.0-1.0 compatibility with High Elves forge
    pub dark_path_compatibility: f32,   // 0.0-1.0 compatibility with Cursed forge
    pub essence_vs_blood_ratio: f32,    // -1.0 to 1.0 (essence to blood spectrum)
    
    // Memory integration
    pub triggers_memory: bool,          // Does this trigger flashbacks?
    pub memory_fragments: Json,         // Associated memory fragments
    pub emotional_resonance: Json,      // How this affects player emotionally
    
    // Sacrifice mechanics
    pub can_be_sacrificed: bool,        // Can be offered to forge
    pub sacrifice_resistance: f32,      // How hard it is to give up
    pub sacrifice_consequences: Json,   // What happens if sacrificed
    pub sacrifice_alternatives: Json,   // Other ways to use this item
    
    pub acquired_at: DateTime<Utc>,
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

pub mod forge_progress {
    use super::*;

    /// Forge progression tracking for ultimate endgame trials
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
    #[sea_orm(table_name = "forge_progress")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: Uuid,
        
        pub player_id: Uuid,
        
        // Forge path choice
        pub chosen_forge_path: Option<String>, // "light" (High Elves) or "dark" (Cursed)
        pub path_commitment_level: f32,        // 0.0-1.0 how committed to path
        pub can_still_switch_paths: bool,     // Can player change their mind?
        
        // Trial progression (tests ALL game systems)
        pub trials_completed: Json,           // Which trials completed
        pub trial_scores: Json,               // Performance on each trial
        pub current_trial: Option<String>,    // Currently active trial
        pub trial_failures: Json,             // Failed attempts and reasons
        
        // Trial categories
        pub hex_navigation_trials: Json,      // Hex grid navigation challenges
        pub mounted_combat_trials: Json,      // Combat while mounted challenges
        pub first_person_trials: Json,        // First-person perspective challenges
        pub party_coordination_trials: Json,  // Multi-companion challenges
        
        // Sentimental reagent collection
        pub reagents_collected: Json,         // Sentimental items available
        pub reagents_used: Json,              // Items already consumed
        pub reagent_essence_power: f32,       // Total essence power available
        pub reagent_blood_power: f32,         // Total blood power available
        
        // Companion sacrifice system
        pub companions_offered: Json,         // Companions offered for sacrifice
        pub sacrifice_method: Option<String>, // "essence" (painless) or "blood" (painful)
        pub sacrifice_resistance_overcome: Json, // How player overcame resistance
        pub mythic_gear_earned: Json,         // Gear blessed/cursed by forge
        
        // Forge readiness assessment
        pub readiness_score: f32,             // 0.0-1.0 ready for final forge use
        pub missing_requirements: Json,       // What's still needed
        pub forge_master_approval: bool,     // Has forge master approved player?
        
        // Ultimate choices
        pub final_forge_decision: Option<String>, // Final choice made at forge
        pub gear_blessing_type: Option<String>,   // Type of blessing/curse received
        pub forge_completion_timestamp: Option<DateTime<Utc>>,
        
        pub created_at: DateTime<Utc>,
        pub updated_at: DateTime<Utc>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {
        #[sea_orm(
            belongs_to = "super::super::players::Entity",
            from = "Column::PlayerId",
            to = "super::super::players::Column::Id"
        )]
        Player,
    }

    impl Related<super::super::players::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::Player.def()
        }
    }

    impl ActiveModelBehavior for ActiveModel {}
}
