//! Philosophy models for 4-path philosophical framework

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Philosophical progression through Strength/Harmony/Light/Dark paths
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "philosophical_progression")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    
    pub player_id: Uuid,
    
    // Four philosophical paths
    pub strength_path_progress: f32,      // 0.0-1.0 progression on Strength path
    pub harmony_path_progress: f32,       // 0.0-1.0 progression on Harmony path  
    pub light_path_progress: f32,         // 0.0-1.0 progression on Light path
    pub dark_path_progress: f32,          // 0.0-1.0 progression on Dark path
    
    // Current philosophical identity
    pub dominant_philosophy: String,      // Current strongest philosophy
    pub secondary_philosophy: String,     // Second strongest influence
    pub philosophical_conflict_level: f32, // 0.0-1.0 internal conflict
    pub identity_stability: f32,          // 0.0-1.0 how stable identity is
    
    // Trait accumulation system
    pub accumulated_traits: Json,         // Traits gained through choices
    pub trait_expression_patterns: Json, // How traits manifest in behavior
    pub trait_synergy_bonuses: Json,     // Bonuses from trait combinations
    pub trait_conflict_penalties: Json,  // Penalties from conflicting traits
    
    // 12 Transition scenarios across 3 acts
    pub act_transitions_completed: Json, // Which transitions completed
    pub current_transition_scenario: Option<String>, // Active transition
    pub transition_test_results: Json,   // Performance on transition tests
    
    // Act 1: Journey TO Labyrinth (6 transitions establishing identity)
    pub act1_identity_transitions: Json, // 6 identity-establishing scenarios
    pub act1_core_values_established: Json, // Values identified in Act 1
    pub act1_philosophical_foundation: String, // Foundation laid in Act 1
    
    // Act 2: Fighting the Dragon (4 transitions testing philosophy)
    pub act2_philosophy_tests: Json,     // 4 philosophy-testing scenarios
    pub act2_conviction_strength: f32,   // How strong convictions are
    pub act2_philosophical_challenges: Json, // Challenges to philosophy
    
    // Act 3: Sealing the Void (2 transitions dealing with consequences)
    pub act3_consequence_transitions: Json, // 2 consequence scenarios
    pub act3_philosophical_synthesis: String, // Final philosophical understanding
    pub act3_wisdom_gained: Json,        // Wisdom from philosophical journey
    
    // Identity emergence tracking
    pub identity_emergence_stage: String, // "forming", "testing", "crystallizing", "transcending"
    pub philosophical_authenticity: f32,  // 0.0-1.0 how authentic to philosophy
    pub moral_consistency: f32,          // 0.0-1.0 consistency in choices
    
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
