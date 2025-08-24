//! Database traits for Dragon's Labyrinth
//! 
//! This module defines traits that provide clean abstractions for game-specific
//! database operations, enabling easy testing, mocking, and integration with
//! Bevy ECS systems and the AI generation pipeline.

use async_trait::async_trait;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use chrono::{DateTime, Utc};

use database_orm::{players, companions, hex_tiles, encounters, dialogues, items, 
                   player_statistics, game_states, ai_workflows, generated_assets, 
                   asset_dependencies, forge, psychology, philosophy, decay, mounts,
                   assets};
use crate::error::DatabaseResult;

/// Core game-specific database operations
/// 
/// This trait abstracts the most common database operations needed by the game engine,
/// providing a clean interface that can be implemented by different database backends
/// or mock implementations for testing.
#[async_trait]
pub trait GameDatabaseOperations {
    // ============================================================================
    // PLAYER AND SAVE SYSTEM
    // ============================================================================

    /// Get a player by their save slot ID
    async fn get_player_by_save_slot(&self, save_slot_id: i32) -> DatabaseResult<Option<players::Model>>;

    /// Create a new player in the specified save slot
    async fn create_player(&self, save_slot_id: i32, name: String) -> DatabaseResult<players::Model>;

    /// Update player progression and stats
    async fn update_player_progression(&self, player_id: Uuid, level: i32, experience: i64) -> DatabaseResult<()>;

    /// Get all save slots with their basic info
    async fn get_save_slots(&self) -> DatabaseResult<Vec<SaveSlotInfo>>;

    /// Delete a save slot and all associated data
    async fn delete_save_slot(&self, save_slot_id: i32) -> DatabaseResult<()>;

    // ============================================================================
    // HORROR PROGRESSION SYSTEM
    // ============================================================================

    /// Update horror progression for a player
    async fn update_horror_progression(&self, player_id: Uuid, new_dread_level: i32, dread_progression: f32) -> DatabaseResult<()>;

    /// Get current horror state for a player
    async fn get_horror_state(&self, player_id: Uuid) -> DatabaseResult<HorrorState>;

    /// Record a horror event that affects dread progression
    async fn record_horror_event(&self, player_id: Uuid, event_type: String, impact: f32) -> DatabaseResult<()>;

    /// Get horror progression history for analytics
    async fn get_horror_progression_history(&self, player_id: Uuid) -> DatabaseResult<Vec<HorrorEvent>>;

    // ============================================================================
    // COMPANION SYSTEM
    // ============================================================================

    /// Get all active companions for a player
    async fn get_active_companions(&self, player_id: Uuid) -> DatabaseResult<Vec<companions::Model>>;

    /// Update companion trauma and relationship stats
    async fn update_companion_trauma(&self, companion_id: Uuid, trauma_level: f32, loyalty: f32, trust: f32) -> DatabaseResult<()>;

    /// Record companion betrayal or departure
    async fn record_companion_betrayal(&self, companion_id: Uuid, reason: String) -> DatabaseResult<()>;

    /// Get available companions for recruitment
    async fn get_available_companions(&self, player_id: Uuid) -> DatabaseResult<Vec<companions::Model>>;

    /// Update companion availability and location
    async fn update_companion_availability(&self, companion_id: Uuid, is_available: bool, location: Option<String>) -> DatabaseResult<()>;

    // ============================================================================
    // HEX WORLD SYSTEM
    // ============================================================================

    /// Get hex tiles within a radius of a center point
    async fn get_hex_tiles_in_radius(&self, center_q: i32, center_r: i32, radius: i32) -> DatabaseResult<Vec<hex_tiles::Model>>;

    /// Update hex tile corruption and properties
    async fn update_hex_tile_corruption(&self, tile_id: Uuid, corruption_level: f32, dread_level: i32) -> DatabaseResult<()>;

    /// Get tiles that need corruption spreading
    async fn get_tiles_for_corruption_spread(&self, max_tiles: i32) -> DatabaseResult<Vec<hex_tiles::Model>>;

    /// Mark hex tile as discovered by player
    async fn discover_hex_tile(&self, player_id: Uuid, tile_id: Uuid) -> DatabaseResult<()>;

    /// Get all discovered tiles for a player
    async fn get_discovered_tiles(&self, player_id: Uuid) -> DatabaseResult<Vec<hex_tiles::Model>>;

    // ============================================================================
    // ENCOUNTER AND DIALOGUE SYSTEM
    // ============================================================================

    /// Get active encounters for a player at their current location
    async fn get_active_encounters(&self, player_id: Uuid, hex_tile_id: Option<Uuid>) -> DatabaseResult<Vec<encounters::Model>>;

    /// Complete an encounter with a specific choice
    async fn complete_encounter(&self, encounter_id: Uuid, player_id: Uuid, choice: String) -> DatabaseResult<()>;

    /// Get available dialogue options for a character
    async fn get_dialogue_options(&self, character_id: Uuid, player_id: Uuid, dread_level: i32) -> DatabaseResult<Vec<dialogues::Model>>;

    /// Update dialogue state after player choice
    async fn update_dialogue_state(&self, dialogue_id: Uuid, choice_made: String, variables: JsonValue) -> DatabaseResult<()>;

    // ============================================================================
    // INVENTORY AND ITEM SYSTEM
    // ============================================================================

    /// Get player inventory with all items
    async fn get_player_inventory(&self, player_id: Uuid) -> DatabaseResult<Vec<InventoryItem>>;

    /// Add item to player inventory
    async fn add_item_to_inventory(&self, player_id: Uuid, item_id: Uuid, quantity: i32) -> DatabaseResult<()>;

    /// Remove item from player inventory
    async fn remove_item_from_inventory(&self, player_id: Uuid, item_id: Uuid, quantity: i32) -> DatabaseResult<()>;

    /// Update item condition and durability
    async fn update_item_condition(&self, inventory_id: Uuid, new_condition: f32) -> DatabaseResult<()>;

    /// Get items by corruption level for dread-appropriate equipment
    async fn get_items_by_corruption_level(&self, min_corruption: f32, max_corruption: f32) -> DatabaseResult<Vec<items::Model>>;

    // ============================================================================
    // AI ASSET SYSTEM
    // ============================================================================

    /// Get assets appropriate for the current dread level
    async fn get_dread_appropriate_assets(&self, dread_level: i32, asset_type: &str) -> DatabaseResult<Vec<generated_assets::Model>>;

    /// Record asset usage for analytics and optimization
    async fn record_asset_usage(&self, asset_id: Uuid, player_id: Option<Uuid>, context: String, performance_metrics: JsonValue) -> DatabaseResult<()>;

    /// Get asset dependencies for proper loading order
    async fn get_asset_dependencies(&self, asset_id: Uuid) -> DatabaseResult<Vec<asset_dependencies::Model>>;

    /// Update asset approval status after human review
    async fn update_asset_approval(&self, asset_id: Uuid, is_approved: bool, notes: Option<String>, approved_by: String) -> DatabaseResult<()>;

    // ============================================================================
    // AI WORKFLOW SYSTEM
    // ============================================================================

    /// Create a new AI workflow execution record
    async fn create_ai_workflow(&self, workflow_type: String, agent_type: String, input_parameters: JsonValue, target_dread_level: i32) -> DatabaseResult<Uuid>;

    /// Update workflow execution status
    async fn update_workflow_status(&self, workflow_id: Uuid, status: String, current_step: String, step_data: JsonValue) -> DatabaseResult<()>;

    /// Record workflow completion with outputs
    async fn record_ai_workflow_completion(&self, workflow_id: Uuid, output_data: JsonValue, generated_asset_ids: Vec<Uuid>) -> DatabaseResult<()>;

    /// Get workflows that need human review
    async fn get_workflows_pending_review(&self) -> DatabaseResult<Vec<ai_workflows::Model>>;

    /// Record human feedback on workflow results
    async fn record_workflow_human_feedback(&self, workflow_id: Uuid, feedback: JsonValue, approval_status: String, reviewed_by: String) -> DatabaseResult<()>;

    // ============================================================================
    // WORLD STATE AND GAME PROGRESSION
    // ============================================================================

    /// Get current global game state
    async fn get_game_state(&self, player_id: Uuid) -> DatabaseResult<Option<game_states::Model>>;

    /// Update global game state (world events, dragon proximity, etc.)
    async fn update_game_state(&self, player_id: Uuid, state_updates: JsonValue) -> DatabaseResult<()>;

    /// Record major world events
    async fn record_world_event(&self, player_id: Uuid, event_type: String, event_data: JsonValue) -> DatabaseResult<()>;

    /// Update dragon proximity and activity level
    async fn update_dragon_proximity(&self, player_id: Uuid, proximity: f32, activity_level: String) -> DatabaseResult<()>;

    // ============================================================================
    // STATISTICS AND ANALYTICS
    // ============================================================================

    /// Get comprehensive player statistics
    async fn get_player_statistics(&self, player_id: Uuid) -> DatabaseResult<Option<player_statistics::Model>>;

    /// Update player statistics after significant events
    async fn update_player_statistics(&self, player_id: Uuid, stat_updates: JsonValue) -> DatabaseResult<()>;

    /// Get aggregated statistics across all players (for game balancing)
    async fn get_aggregated_statistics(&self) -> DatabaseResult<AggregatedStats>;
}

/// Trait for ECS-specific database operations
/// 
/// This trait provides operations specifically designed for integration with
/// Bevy's ECS system, allowing entities to sync with database state.
#[async_trait]
pub trait EcsDatabaseOperations {
    /// Sync an ECS entity with its database representation
    async fn sync_entity_with_database(&self, entity_id: Uuid, entity_type: String, component_data: JsonValue) -> DatabaseResult<()>;

    /// Load database state for an ECS entity
    async fn load_entity_from_database(&self, entity_id: Uuid, entity_type: String) -> DatabaseResult<Option<JsonValue>>;

    /// Mark entities that need database synchronization
    async fn mark_entities_for_sync(&self, entity_ids: Vec<Uuid>) -> DatabaseResult<()>;

    /// Get entities that need to be synchronized (returns their database IDs)
    async fn get_entities_needing_sync(&self) -> DatabaseResult<Vec<Uuid>>;

    /// Batch update multiple entities for performance
    async fn batch_update_entities(&self, updates: Vec<EntityUpdate>) -> DatabaseResult<()>;
}

/// Trait for AI generation integration
/// 
/// This trait provides operations for integrating with the AI generation pipeline,
/// allowing the database to coordinate with AI agents and track generation workflows.
#[async_trait]
pub trait AIGenerationOperations {
    /// Check if content already exists to avoid redundant generation
    async fn check_content_exists(&self, content_hash: String, content_type: String) -> DatabaseResult<bool>;

    /// Store generation request for idempotency
    async fn store_generation_request(&self, request_hash: String, request_data: JsonValue, workflow_id: Uuid) -> DatabaseResult<()>;

    /// Get cached generation results
    async fn get_cached_generation_result(&self, request_hash: String) -> DatabaseResult<Option<JsonValue>>;

    /// Record human-in-the-loop review decisions
    async fn record_human_review(&self, workflow_id: Uuid, review_data: JsonValue, decision: String, reviewer: String) -> DatabaseResult<()>;

    /// Get content that needs human review
    async fn get_content_pending_review(&self, agent_type: Option<String>) -> DatabaseResult<Vec<AIContentReview>>;

    /// Update generation costs and token usage
    async fn update_generation_costs(&self, workflow_id: Uuid, api_calls: i32, tokens_consumed: i32, cost_dollars: f32) -> DatabaseResult<()>;
}

// ============================================================================
// SUPPORTING DATA TYPES
// ============================================================================

/// Basic save slot information for the save system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveSlotInfo {
    pub slot_id: i32,
    pub player_name: String,
    pub save_name: String,
    pub last_played: DateTime<Utc>,
    pub play_time_seconds: i64,
    pub completion_percentage: f32,
    pub current_dread_level: i32,
    pub is_reverse_playthrough: bool,
}

/// Current horror state for a player
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HorrorState {
    pub current_dread_level: i32,
    pub dread_progression: f32,
    pub horror_exposure: f32,
    pub corruption_level: f32,
    pub sanity: f32,
    pub max_sanity: f32,
    pub last_horror_event: Option<DateTime<Utc>>,
}

/// Horror event record for progression tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HorrorEvent {
    pub event_type: String,
    pub impact: f32,
    pub dread_level_before: i32,
    pub dread_level_after: i32,
    pub occurred_at: DateTime<Utc>,
    pub context: JsonValue,
}

/// Inventory item with full item details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryItem {
    pub inventory_id: Uuid,
    pub item: items::Model,
    pub quantity: i32,
    pub condition: f32,
    pub acquired_at: DateTime<Utc>,
    pub inventory_slot: i32,
    pub is_favorited: bool,
}

/// Entity update for batch operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityUpdate {
    pub entity_id: Uuid,
    pub entity_type: String,
    pub component_data: JsonValue,
    pub needs_sync: bool,
}

/// AI content review item for human-in-the-loop
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIContentReview {
    pub workflow_id: Uuid,
    pub agent_type: String,
    pub content_type: String,
    pub content_preview: JsonValue,
    pub generation_cost: f32,
    pub quality_score: f32,
    pub requires_approval: bool,
    pub created_at: DateTime<Utc>,
}

/// Aggregated statistics for game balancing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregatedStats {
    pub total_players: u64,
    pub average_completion_time: f64,
    pub dread_level_distribution: JsonValue,
    pub companion_betrayal_rates: JsonValue,
    pub most_common_endings: JsonValue,
    pub asset_usage_patterns: JsonValue,
    pub average_horror_progression_time: JsonValue,
    pub player_retention_by_dread_level: JsonValue,
}

// ============================================================================
// MOCK IMPLEMENTATIONS FOR TESTING
// ============================================================================

/// Mock database implementation for testing
/// 
/// This provides a simple in-memory implementation of the database traits
/// that can be used for unit testing without requiring a real database.
#[cfg(test)]
pub mod mock {
    use super::*;
    use std::collections::HashMap;
    use std::sync::Mutex;

    pub struct MockGameDatabase {
        players: Mutex<HashMap<Uuid, players::Model>>,
        companions: Mutex<HashMap<Uuid, companions::Model>>,
        hex_tiles: Mutex<HashMap<Uuid, hex_tiles::Model>>,
        // Add other collections as needed for testing
    }

    impl MockGameDatabase {
        pub fn new() -> Self {
            Self {
                players: Mutex::new(HashMap::new()),
                companions: Mutex::new(HashMap::new()),
                hex_tiles: Mutex::new(HashMap::new()),
            }
        }
    }

    #[async_trait]
    impl GameDatabaseOperations for MockGameDatabase {
        async fn get_player_by_save_slot(&self, save_slot_id: i32) -> DatabaseResult<Option<players::Model>> {
            let players = self.players.lock().unwrap();
            let player = players.values()
                .find(|p| p.save_slot_id == save_slot_id)
                .cloned();
            Ok(player)
        }

        async fn create_player(&self, save_slot_id: i32, name: String) -> DatabaseResult<players::Model> {
            let player = players::Model {
                id: Uuid::new_v4(),
                name: name.clone(),
                save_slot_id,
                save_name: name,
                level: 1,
                experience: 0,
                health: 100.0,
                max_health: 100.0,
                sanity: 100.0,
                max_sanity: 100.0,
                current_dread_level: 0,
                dread_progression: 0.0,
                horror_exposure: 0.0,
                corruption_level: 0.0,
                hex_position_q: 0,
                hex_position_r: 0,
                hex_position_s: 0,
                world_position_x: 0.0,
                world_position_y: 0.0,
                world_position_z: 0.0,
                play_time_seconds: 0,
                last_played: Utc::now(),
                completion_percentage: 0.0,
                is_reverse_playthrough: false,
                reverse_completion_count: 0,
                endings_unlocked: JsonValue::Array(vec![]),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            };

            let mut players = self.players.lock().unwrap();
            players.insert(player.id, player.clone());
            Ok(player)
        }

        // Implement other required methods with simple mock behavior
        async fn update_player_progression(&self, _player_id: Uuid, _level: i32, _experience: i64) -> DatabaseResult<()> {
            Ok(())
        }

        async fn get_save_slots(&self) -> DatabaseResult<Vec<SaveSlotInfo>> {
            Ok(vec![])
        }

        async fn delete_save_slot(&self, _save_slot_id: i32) -> DatabaseResult<()> {
            Ok(())
        }

        async fn update_horror_progression(&self, _player_id: Uuid, _new_dread_level: i32, _dread_progression: f32) -> DatabaseResult<()> {
            Ok(())
        }

        async fn get_horror_state(&self, _player_id: Uuid) -> DatabaseResult<HorrorState> {
            Ok(HorrorState {
                current_dread_level: 0,
                dread_progression: 0.0,
                horror_exposure: 0.0,
                corruption_level: 0.0,
                sanity: 100.0,
                max_sanity: 100.0,
                last_horror_event: None,
            })
        }

        // Add more mock implementations as needed...
        // For brevity, implementing the minimum required methods
        
        async fn record_horror_event(&self, _player_id: Uuid, _event_type: String, _impact: f32) -> DatabaseResult<()> { Ok(()) }
        async fn get_horror_progression_history(&self, _player_id: Uuid) -> DatabaseResult<Vec<HorrorEvent>> { Ok(vec![]) }
        async fn get_active_companions(&self, _player_id: Uuid) -> DatabaseResult<Vec<companions::Model>> { Ok(vec![]) }
        async fn update_companion_trauma(&self, _companion_id: Uuid, _trauma_level: f32, _loyalty: f32, _trust: f32) -> DatabaseResult<()> { Ok(()) }
        async fn record_companion_betrayal(&self, _companion_id: Uuid, _reason: String) -> DatabaseResult<()> { Ok(()) }
        async fn get_available_companions(&self, _player_id: Uuid) -> DatabaseResult<Vec<companions::Model>> { Ok(vec![]) }
        async fn update_companion_availability(&self, _companion_id: Uuid, _is_available: bool, _location: Option<String>) -> DatabaseResult<()> { Ok(()) }
        async fn get_hex_tiles_in_radius(&self, _center_q: i32, _center_r: i32, _radius: i32) -> DatabaseResult<Vec<hex_tiles::Model>> { Ok(vec![]) }
        async fn update_hex_tile_corruption(&self, _tile_id: Uuid, _corruption_level: f32, _dread_level: i32) -> DatabaseResult<()> { Ok(()) }
        async fn get_tiles_for_corruption_spread(&self, _max_tiles: i32) -> DatabaseResult<Vec<hex_tiles::Model>> { Ok(vec![]) }
        async fn discover_hex_tile(&self, _player_id: Uuid, _tile_id: Uuid) -> DatabaseResult<()> { Ok(()) }
        async fn get_discovered_tiles(&self, _player_id: Uuid) -> DatabaseResult<Vec<hex_tiles::Model>> { Ok(vec![]) }
        async fn get_active_encounters(&self, _player_id: Uuid, _hex_tile_id: Option<Uuid>) -> DatabaseResult<Vec<encounters::Model>> { Ok(vec![]) }
        async fn complete_encounter(&self, _encounter_id: Uuid, _player_id: Uuid, _choice: String) -> DatabaseResult<()> { Ok(()) }
        async fn get_dialogue_options(&self, _character_id: Uuid, _player_id: Uuid, _dread_level: i32) -> DatabaseResult<Vec<dialogues::Model>> { Ok(vec![]) }
        async fn update_dialogue_state(&self, _dialogue_id: Uuid, _choice_made: String, _variables: JsonValue) -> DatabaseResult<()> { Ok(()) }
        async fn get_player_inventory(&self, _player_id: Uuid) -> DatabaseResult<Vec<InventoryItem>> { Ok(vec![]) }
        async fn add_item_to_inventory(&self, _player_id: Uuid, _item_id: Uuid, _quantity: i32) -> DatabaseResult<()> { Ok(()) }
        async fn remove_item_from_inventory(&self, _player_id: Uuid, _item_id: Uuid, _quantity: i32) -> DatabaseResult<()> { Ok(()) }
        async fn update_item_condition(&self, _inventory_id: Uuid, _new_condition: f32) -> DatabaseResult<()> { Ok(()) }
        async fn get_items_by_corruption_level(&self, _min_corruption: f32, _max_corruption: f32) -> DatabaseResult<Vec<items::Model>> { Ok(vec![]) }
        async fn get_dread_appropriate_assets(&self, _dread_level: i32, _asset_type: &str) -> DatabaseResult<Vec<generated_assets::Model>> { Ok(vec![]) }
        async fn record_asset_usage(&self, _asset_id: Uuid, _player_id: Option<Uuid>, _context: String, _performance_metrics: JsonValue) -> DatabaseResult<()> { Ok(()) }
        async fn get_asset_dependencies(&self, _asset_id: Uuid) -> DatabaseResult<Vec<asset_dependencies::Model>> { Ok(vec![]) }
        async fn update_asset_approval(&self, _asset_id: Uuid, _is_approved: bool, _notes: Option<String>, _approved_by: String) -> DatabaseResult<()> { Ok(()) }
        async fn create_ai_workflow(&self, _workflow_type: String, _agent_type: String, _input_parameters: JsonValue, _target_dread_level: i32) -> DatabaseResult<Uuid> { Ok(Uuid::new_v4()) }
        async fn update_workflow_status(&self, _workflow_id: Uuid, _status: String, _current_step: String, _step_data: JsonValue) -> DatabaseResult<()> { Ok(()) }
        async fn record_ai_workflow_completion(&self, _workflow_id: Uuid, _output_data: JsonValue, _generated_asset_ids: Vec<Uuid>) -> DatabaseResult<()> { Ok(()) }
        async fn get_workflows_pending_review(&self) -> DatabaseResult<Vec<ai_workflows::Model>> { Ok(vec![]) }
        async fn record_workflow_human_feedback(&self, _workflow_id: Uuid, _feedback: JsonValue, _approval_status: String, _reviewed_by: String) -> DatabaseResult<()> { Ok(()) }
        async fn get_game_state(&self, _player_id: Uuid) -> DatabaseResult<Option<game_states::Model>> { Ok(None) }
        async fn update_game_state(&self, _player_id: Uuid, _state_updates: JsonValue) -> DatabaseResult<()> { Ok(()) }
        async fn record_world_event(&self, _player_id: Uuid, _event_type: String, _event_data: JsonValue) -> DatabaseResult<()> { Ok(()) }
        async fn update_dragon_proximity(&self, _player_id: Uuid, _proximity: f32, _activity_level: String) -> DatabaseResult<()> { Ok(()) }
        async fn get_player_statistics(&self, _player_id: Uuid) -> DatabaseResult<Option<player_statistics::Model>> { Ok(None) }
        async fn update_player_statistics(&self, _player_id: Uuid, _stat_updates: JsonValue) -> DatabaseResult<()> { Ok(()) }
        async fn get_aggregated_statistics(&self) -> DatabaseResult<AggregatedStats> { 
            Ok(AggregatedStats {
                total_players: 0,
                average_completion_time: 0.0,
                dread_level_distribution: JsonValue::Object(serde_json::Map::new()),
                companion_betrayal_rates: JsonValue::Object(serde_json::Map::new()),
                most_common_endings: JsonValue::Array(vec![]),
                asset_usage_patterns: JsonValue::Object(serde_json::Map::new()),
                average_horror_progression_time: JsonValue::Object(serde_json::Map::new()),
                player_retention_by_dread_level: JsonValue::Object(serde_json::Map::new()),
            })
        }
    }
}
