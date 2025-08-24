//! Intelligent Database Router for Dragon's Labyrinth
//! 
//! This module provides automatic routing between the game content database (game.db)
//! and player data database (player.db) based on operation type. Systems can use
//! the router transparently without needing to know which database to use.

use async_trait::async_trait;
use sea_orm::{DatabaseConnection as SeaConnection, DatabaseTransaction, EntityTrait, QueryFilter, ColumnTrait};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use serde_json::Value as JsonValue;

use database_orm::{
    players, companions, hex_tiles, encounters, dialogues, items, 
    player_statistics, game_states, ai_workflows, generated_assets,
    Players, Companions, HexTiles, Encounters, Dialogues, Items,
    PlayerStatistics, GameStates, AiWorkflows, GeneratedAssets,
};

use crate::engine::{GameDatabase, DatabaseType, OperationType};
use crate::error::{DatabaseError, DatabaseResult};
use crate::traits::*;

/// Smart database router that automatically routes operations to the correct database
pub struct DatabaseRouter {
    game_database: GameDatabase,
}

impl DatabaseRouter {
    /// Create a new database router with dual database connections
    pub async fn new(game_db_url: &str, player_db_url: Option<&str>) -> DatabaseResult<Self> {
        let game_database = GameDatabase::connect_dual(game_db_url, player_db_url).await?;
        
        Ok(Self {
            game_database,
        })
    }

    /// Analyze operation to determine which database to use
    pub fn classify_operation(&self, entity_type: &str, operation: &str) -> OperationType {
        match entity_type {
            // World content entities -> game.db
            "hex_tiles" | "encounters" | "dialogues" | "items" | 
            "generated_assets" | "ai_workflows" | "asset_dependencies" |
            "biomes" | "dungeons" | "settlements" | "npcs" |
            "world_generation" | "content_templates" => OperationType::WorldContent,
            
            // Player state entities -> player.db
            "players" | "companions" | "player_statistics" | "game_states" |
            "forge_progress" | "companion_therapy" | "philosophical_progression" |
            "world_corruption" | "mount_companions" | "player_preferences" |
            "save_games" | "achievements" => OperationType::PlayerState,
            
            // Hybrid operations need coordination
            _ => {
                match operation {
                    "world_discovery" | "dread_progression" | "corruption_spread" => OperationType::Hybrid,
                    _ => OperationType::PlayerState, // Default to player database for safety
                }
            }
        }
    }

    /// Route a query to the appropriate database
    pub async fn route_query<T, F, Fut>(&self, entity_type: &str, operation: &str, query_fn: F) -> DatabaseResult<T>
    where
        F: FnOnce(Arc<RwLock<SeaConnection>>) -> Fut + Send,
        Fut: std::future::Future<Output = DatabaseResult<T>> + Send,
    {
        let operation_type = self.classify_operation(entity_type, operation);
        let connection = self.game_database.route_connection(operation_type).await;
        query_fn(connection).await
    }

    /// Execute a hybrid operation that coordinates between both databases
    pub async fn execute_hybrid_operation<T, F, Fut>(&self, operation_fn: F) -> DatabaseResult<T>
    where
        F: FnOnce(Arc<RwLock<SeaConnection>>, Arc<RwLock<SeaConnection>>) -> Fut + Send,
        Fut: std::future::Future<Output = DatabaseResult<T>> + Send,
    {
        let game_conn = self.game_database.get_connection(DatabaseType::GameContent).await;
        let player_conn = self.game_database.get_connection(DatabaseType::PlayerData).await;
        operation_fn(game_conn, player_conn).await
    }

    /// Begin a transaction on the appropriate database
    pub async fn begin_transaction(&self, entity_type: &str, operation: &str) -> DatabaseResult<DatabaseTransaction> {
        let operation_type = self.classify_operation(entity_type, operation);
        let db_type = match operation_type {
            OperationType::WorldContent => DatabaseType::GameContent,
            OperationType::PlayerState => DatabaseType::PlayerData,
            OperationType::Hybrid => DatabaseType::PlayerData, // Default for hybrid
        };
        self.game_database.begin_transaction(db_type).await
    }

    /// Begin coordinated transactions for hybrid operations
    pub async fn begin_dual_transaction(&self) -> DatabaseResult<(DatabaseTransaction, DatabaseTransaction)> {
        self.game_database.begin_dual_transaction().await
    }

    /// Get access to the underlying game database for direct operations
    pub fn database(&self) -> &GameDatabase {
        &self.game_database
    }
}

/// Smart query operations with automatic routing
impl DatabaseRouter {
    /// Get hex tiles with automatic routing to game.db
    pub async fn get_hex_tiles_in_radius(&self, center_q: i32, center_r: i32, radius: i32) -> DatabaseResult<Vec<hex_tiles::Model>> {
        self.route_query("hex_tiles", "query", |conn| async move {
            let tiles = HexTiles::find()
                .filter(hex_tiles::Column::Q.between(center_q - radius, center_q + radius))
                .filter(hex_tiles::Column::R.between(center_r - radius, center_r + radius))
                .all(&*conn.read().await)
                .await?;

            // Filter by actual hex distance
            let filtered_tiles: Vec<_> = tiles.into_iter()
                .filter(|tile| {
                    let distance = ((tile.q - center_q).abs())
                        .max((tile.r - center_r).abs())
                        .max((tile.s - (center_q + center_r)).abs());
                    distance <= radius
                })
                .collect();

            Ok(filtered_tiles)
        }).await
    }

    /// Get player data with automatic routing to player.db
    pub async fn get_player_by_save_slot(&self, save_slot_id: i32) -> DatabaseResult<Option<players::Model>> {
        self.route_query("players", "query", |conn| async move {
            let player = Players::find()
                .filter(players::Column::SaveSlotId.eq(save_slot_id))
                .one(&*conn.read().await)
                .await?;
            Ok(player)
        }).await
    }

    /// Get companions with automatic routing to player.db
    pub async fn get_active_companions(&self, player_id: Uuid) -> DatabaseResult<Vec<companions::Model>> {
        self.route_query("companions", "query", |conn| async move {
            let companions = Companions::find()
                .filter(companions::Column::PlayerId.eq(player_id))
                .filter(companions::Column::IsActive.eq(true))
                .all(&*conn.read().await)
                .await?;
            Ok(companions)
        }).await
    }

    /// Get dread-appropriate assets with automatic routing to game.db
    pub async fn get_dread_appropriate_assets(&self, dread_level: i32, asset_type: &str) -> DatabaseResult<Vec<generated_assets::Model>> {
        let asset_type_owned = asset_type.to_string();
        self.route_query("generated_assets", "query", |conn| async move {
            let assets = GeneratedAssets::find()
                .filter(generated_assets::Column::AssetType.eq(asset_type_owned))
                .filter(generated_assets::Column::DreadLevelTarget.lte(dread_level))
                .filter(generated_assets::Column::IsApproved.eq(true))
                .all(&*conn.read().await)
                .await?;
            Ok(assets)
        }).await
    }

    /// Update player horror progression with automatic routing to player.db
    pub async fn update_horror_progression(&self, player_id: Uuid, new_dread_level: i32, dread_progression: f32) -> DatabaseResult<()> {
        self.route_query("players", "update", |conn| async move {
            Players::update_many()
                .filter(players::Column::Id.eq(player_id))
                .col_expr(players::Column::CurrentDreadLevel, sea_orm::sea_query::Expr::value(new_dread_level))
                .col_expr(players::Column::DreadProgression, sea_orm::sea_query::Expr::value(dread_progression))
                .col_expr(players::Column::UpdatedAt, sea_orm::sea_query::Expr::current_timestamp())
                .exec(&*conn.read().await)
                .await?;
            Ok(())
        }).await
    }

    /// Hybrid operation: Update hex tile corruption (coordinates between game and player data)
    pub async fn update_hex_tile_corruption(&self, tile_id: Uuid, corruption_level: f32, player_id: Uuid) -> DatabaseResult<()> {
        self.execute_hybrid_operation(|_game_conn, player_conn| async move {
            // Update tile corruption in game database (if it's tracked there)
            // This could be read-only data, so we might only track player-specific corruption
            
            // Update player's corruption exposure in player database
            Players::update_many()
                .filter(players::Column::Id.eq(player_id))
                .col_expr(players::Column::CorruptionLevel, sea_orm::sea_query::Expr::value(corruption_level))
                .col_expr(players::Column::UpdatedAt, sea_orm::sea_query::Expr::current_timestamp())
                .exec(&*player_conn.read().await)
                .await?;

            Ok(())
        }).await
    }

    /// Hybrid operation: Companion trauma progression (affects both companion data and world state)
    pub async fn update_companion_trauma_with_world_effects(&self, companion_id: Uuid, trauma_level: f32, affects_dread: bool) -> DatabaseResult<()> {
        self.execute_hybrid_operation(|_game_conn, player_conn| async move {
            // Update companion trauma in player database
            Companions::update_many()
                .filter(companions::Column::Id.eq(companion_id))
                .col_expr(companions::Column::TraumaLevel, sea_orm::sea_query::Expr::value(trauma_level))
                .col_expr(companions::Column::UpdatedAt, sea_orm::sea_query::Expr::current_timestamp())
                .exec(&*player_conn.read().await)
                .await?;

            // If trauma affects dread, we could update player dread here
            if affects_dread {
                // Additional dread progression logic would go here
            }

            Ok(())
        }).await
    }
}

/// Bevy resource wrapper for the database router
#[derive(Resource, Clone)]
pub struct DatabaseRouterResource {
    pub router: DatabaseRouter,
}

impl DatabaseRouterResource {
    /// Create a new database router resource for Bevy
    pub async fn new(game_db_url: &str, player_db_url: Option<&str>) -> DatabaseResult<Self> {
        let router = DatabaseRouter::new(game_db_url, player_db_url).await?;
        Ok(Self { router })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_router_classification() {
        let router = DatabaseRouter::new("sqlite::memory:", Some("sqlite::memory:")).await.unwrap();
        
        // Test world content classification
        assert_eq!(router.classify_operation("hex_tiles", "query"), OperationType::WorldContent);
        assert_eq!(router.classify_operation("encounters", "query"), OperationType::WorldContent);
        assert_eq!(router.classify_operation("items", "query"), OperationType::WorldContent);
        
        // Test player state classification
        assert_eq!(router.classify_operation("players", "query"), OperationType::PlayerState);
        assert_eq!(router.classify_operation("companions", "update"), OperationType::PlayerState);
        assert_eq!(router.classify_operation("game_states", "query"), OperationType::PlayerState);
        
        // Test hybrid classification
        assert_eq!(router.classify_operation("unknown", "world_discovery"), OperationType::Hybrid);
        assert_eq!(router.classify_operation("unknown", "dread_progression"), OperationType::Hybrid);
    }

    #[tokio::test]
    async fn test_router_query_routing() {
        let router = DatabaseRouter::new("sqlite::memory:", Some("sqlite::memory:")).await.unwrap();
        
        // Test player query routing
        let result = router.get_player_by_save_slot(1).await;
        assert!(result.is_ok());
        
        // Test hex tiles query routing
        let result = router.get_hex_tiles_in_radius(0, 0, 5).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_router_hybrid_operations() {
        let router = DatabaseRouter::new("sqlite::memory:", Some("sqlite::memory:")).await.unwrap();
        
        let test_player_id = Uuid::new_v4();
        let test_tile_id = Uuid::new_v4();
        
        // Test hybrid operation (should not fail even with missing data)
        let result = router.update_hex_tile_corruption(test_tile_id, 0.5, test_player_id).await;
        assert!(result.is_ok());
    }
}
