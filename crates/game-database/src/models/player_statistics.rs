//! Player statistics models for comprehensive gameplay tracking

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Comprehensive player statistics and achievement tracking
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "player_statistics")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    
    pub player_id: Uuid, // Which player these statistics belong to
    
    // Core gameplay statistics
    pub total_play_time_seconds: i64, // Total time played
    pub tiles_discovered: i32, // Number of hex tiles discovered
    pub tiles_fully_explored: i32, // Number of tiles completely explored
    pub encounters_completed: i32, // Number of encounters finished
    pub dialogue_lines_spoken: i32, // Total dialogue interactions
    
    // Horror progression tracking
    pub times_reached_dread_level_1: i32, // Unease
    pub times_reached_dread_level_2: i32, // Fear  
    pub times_reached_dread_level_3: i32, // Terror
    pub times_reached_dread_level_4: i32, // Horror
    pub time_spent_at_each_dread_level: serde_json::Value, // JSON object with seconds per level
    
    pub max_dread_level_reached: i32, // Highest dread level achieved
    pub total_corruption_accumulated: f32, // Cumulative corruption over time
    pub corruption_events_triggered: i32, // Number of corruption events
    
    // Companion interaction statistics
    pub companion_bonds_formed: i32, // Number of companions befriended
    pub companion_betrayals_experienced: i32, // Number of companion betrayals
    pub companion_deaths_witnessed: i32, // Number of companions who died
    pub traumatic_events_with_companions: i32, // Shared traumatic experiences
    
    #[sea_orm(column_type = "Json")]
    pub companion_relationship_history: serde_json::Value, // JSON object tracking relationship changes
    
    // Forge system statistics
    pub sentimental_items_forged: i32, // Items created in forge
    pub light_essence_gathered: f32, // Total light essence collected
    pub dark_essence_gathered: f32, // Total dark essence collected
    pub forge_sessions_completed: i32, // Number of forging sessions
    pub high_elf_path_progress: f32, // 0.0-1.0 progress on light path
    pub cursed_path_progress: f32, // 0.0-1.0 progress on dark path
    
    // Item and inventory statistics  
    pub items_collected: i32, // Total items found
    pub sentimental_items_lost: i32, // Sentimental items lost to corruption/betrayal
    pub items_given_to_companions: i32, // Items shared with companions
    pub items_received_from_companions: i32, // Gifts from companions
    
    // Story progression tracking
    pub story_flags_set: i32, // Number of story progression markers reached
    pub major_story_beats_completed: i32, // Key narrative moments
    pub optional_content_discovered: i32, // Side content found
    pub secrets_uncovered: i32, // Hidden lore or content discovered
    
    // Psychological and philosophical tracking
    pub philosophical_choices_made: i32, // Number of moral/philosophical decisions
    pub times_chose_hope_over_despair: i32, // Positive choices made
    pub times_chose_despair_over_hope: i32, // Negative choices made
    pub moral_flexibility_score: f32, // How consistent player's moral choices are
    
    // Achievement and milestone tracking
    #[sea_orm(column_type = "Json")]
    pub achievements_unlocked: serde_json::Value, // JSON array of achievement IDs
    
    #[sea_orm(column_type = "Json")]
    pub milestones_reached: serde_json::Value, // JSON object with milestone timestamps
    
    pub perfect_companion_bonds_maintained: i32, // Companions with max trust
    pub companions_successfully_redeemed: i32, // Companions saved from corruption
    
    // Death and failure statistics
    pub times_died: i32, // Player death count
    pub companions_lost_to_corruption: i32, // Companions who became corrupted
    pub critical_failures_in_dialogue: i32, // Failed important conversations
    pub opportunities_missed: i32, // Key opportunities not taken
    
    // Session tracking
    pub total_sessions: i32, // Number of play sessions
    pub average_session_length_seconds: f32, // Average session duration
    pub longest_session_seconds: i64, // Longest single session
    pub shortest_session_seconds: i64, // Shortest single session
    
    pub last_session_start: Option<DateTime<Utc>>, // When current/last session started
    pub last_session_end: Option<DateTime<Utc>>, // When last session ended
    
    // Performance and engagement metrics
    pub dialogue_choices_per_minute: f32, // Player decision rate
    pub exploration_efficiency: f32, // How efficiently player explores
    pub companion_interaction_frequency: f32, // How often player talks to companions
    pub story_engagement_score: f32, // 0.0-1.0 how engaged with main story
    
    // Timestamps
    pub first_recorded: DateTime<Utc>, // When statistics started being tracked
    pub last_updated: DateTime<Utc>, // Most recent update
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
