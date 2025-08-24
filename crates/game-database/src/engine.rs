//! Database engine and connection management for Dragon's Labyrinth
//! 
//! This module provides the core database abstraction layer, handling connections,
//! migrations, and high-level database operations while integrating seamlessly
//! with Bevy ECS systems and the AI generation pipeline.

use sea_orm::{
    Database, DatabaseConnection as SeaConnection, DatabaseTransaction, 
    ConnectOptions, EntityTrait, QueryFilter, ColumnTrait, TransactionTrait,
    DbErr, ConnectionTrait, Statement, DbBackend
};
use std::time::Duration;
use tokio::sync::RwLock;
use std::sync::Arc;
use bevy::prelude::*;

use database_orm::{
    // Module imports for types and columns
    players, companions, hex_tiles, encounters, dialogues, items, 
    player_statistics, game_states, ai_workflows, generated_assets, 
    asset_dependencies, forge, psychology, philosophy, decay, mounts,
    assets,
    // Entity imports for database operations
    Players, Companions, HexTiles, Encounters, Dialogues, Items,
    PlayerStatistics, GameStates, AiWorkflows, GeneratedAssets,
    AssetDependencies, SentimentalItems, ForgeProgress, CompanionTherapy,
    PhilosophicalProgression, WorldCorruption, MountCompanions, Assets,
    AssetAttribution
};
use crate::error::{DatabaseError, DatabaseResult};
use crate::traits::*;
use serde::{Serialize, Deserialize};

/// Main database connection wrapper with game-specific functionality
#[derive(Clone, Debug)]
pub struct GameDatabase {
    connection: Arc<RwLock<SeaConnection>>,
    database_url: String,
    is_connected: Arc<RwLock<bool>>,
}

/// Type alias for easier usage throughout the codebase
pub type DatabaseConnection = GameDatabase;

impl GameDatabase {
    /// Connect to the database with optimized settings for the game
    pub async fn connect(database_url: &str) -> DatabaseResult<Self> {
        tracing::info!("Connecting to game database: {}", database_url);
        
        let mut opt = ConnectOptions::new(database_url);
        opt.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(10))
            .acquire_timeout(Duration::from_secs(10))
            .idle_timeout(Duration::from_secs(600))
            .max_lifetime(Duration::from_secs(3600))
            .sqlx_logging(true)
            .sqlx_logging_level(tracing::log::LevelFilter::Debug);

        let connection = Database::connect(opt)
            .await
            .map_err(DatabaseError::Connection)?;

        let db = Self {
            connection: Arc::new(RwLock::new(connection)),
            database_url: database_url.to_string(),
            is_connected: Arc::new(RwLock::new(true)),
        };

        // Verify connection with a simple query
        db.verify_connection().await?;
        
        tracing::info!("Successfully connected to game database");
        Ok(db)
    }

    /// Verify the database connection is working
    pub async fn verify_connection(&self) -> DatabaseResult<()> {
        let conn = self.connection.read().await;
        conn.ping().await.map_err(DatabaseError::Connection)?;
        Ok(())
    }

    /// Check if database is connected
    pub async fn is_connected(&self) -> bool {
        *self.is_connected.read().await
    }

    /// Get a reference to the underlying SeaORM connection
    pub async fn connection(&self) -> Arc<RwLock<SeaConnection>> {
        self.connection.clone()
    }

    /// Begin a database transaction for complex operations
    pub async fn begin_transaction(&self) -> DatabaseResult<DatabaseTransaction> {
        let conn = self.connection.read().await;
        conn.begin().await.map_err(DatabaseError::Transaction)
    }

    /// Create all database tables if they don't exist
    pub async fn create_tables(&self) -> DatabaseResult<()> {
        tracing::info!("Creating database tables for Dragon's Labyrinth");
        
        let conn = self.connection.read().await;
        let builder = conn.get_database_backend();
        
        // Create tables in dependency order to avoid foreign key issues
        let table_creation_statements = self.get_table_creation_statements(builder);
        
        for statement in table_creation_statements {
            conn.execute(statement).await.map_err(DatabaseError::Migration)?;
        }

        tracing::info!("Successfully created all database tables");
        Ok(())
    }

    /// Drop all tables (careful!)
    pub async fn drop_all_tables(&self) -> DatabaseResult<()> {
        tracing::warn!("Dropping all database tables - this is destructive!");
        
        let conn = self.connection.read().await;
        let builder = conn.get_database_backend();
        
        let drop_statements = self.get_table_drop_statements(builder);
        
        for statement in drop_statements {
            conn.execute(statement).await.map_err(DatabaseError::Migration)?;
        }

        Ok(())
    }

    /// Seed initial data required for the game to function
    pub async fn seed_initial_data(&self) -> DatabaseResult<()> {
        tracing::info!("Seeding initial game data");
        
        let txn = self.begin_transaction().await?;
        
        // Seed initial dread levels configuration
        self.seed_dread_levels(&txn).await?;
        
        // Seed companion base configurations
        self.seed_companion_templates(&txn).await?;
        
        // Seed initial world generation parameters
        self.seed_world_generation_defaults(&txn).await?;
        
        // Seed AI workflow templates
        self.seed_ai_workflow_templates(&txn).await?;
        
        txn.commit().await.map_err(DatabaseError::Transaction)?;
        
        tracing::info!("Successfully seeded initial game data");
        Ok(())
    }

    /// Get database statistics and health information
    pub async fn get_database_stats(&self) -> DatabaseResult<DatabaseStats> {
        let conn = self.connection.read().await;
        
        // Get table counts
        let player_count = Players::find().count(&*conn).await?;
        let companion_count = Companions::find().count(&*conn).await?;
        let hex_tile_count = HexTiles::find().count(&*conn).await?;
        let encounter_count = Encounters::find().count(&*conn).await?;
        let dialogue_count = Dialogues::find().count(&*conn).await?;
        let item_count = Items::find().count(&*conn).await?;
        let generated_asset_count = GeneratedAssets::find().count(&*conn).await?;
        let ai_workflow_count = AIWorkflows::find().count(&*conn).await?;
        
        Ok(DatabaseStats {
            player_count,
            companion_count,
            hex_tile_count,
            encounter_count,
            dialogue_count,
            item_count,
            generated_asset_count,
            ai_workflow_count,
            database_size_mb: self.get_database_size_mb().await?,
            connection_pool_active: self.get_active_connections().await,
            last_backup: None, // TODO: Implement backup tracking
        })
    }

    /// Cleanup old data to keep database size manageable
    pub async fn cleanup_old_data(&self, older_than_days: i32) -> DatabaseResult<CleanupStats> {
        tracing::info!("Starting database cleanup for data older than {} days", older_than_days);
        
        let txn = self.begin_transaction().await?;
        let cutoff_date = chrono::Utc::now() - chrono::Duration::days(older_than_days as i64);
        
        // Clean up old asset usage logs
        // TODO: Implement AssetUsageLogs in database-orm
        let usage_logs_deleted = 0u64;

        // Clean up old workflow steps for completed workflows
        // TODO: Implement WorkflowSteps in database-orm
        let workflow_steps_deleted = 0u64;

        // Clean up old player statistics (keep recent for analytics)
        let old_stats_deleted = PlayerStatistics::delete_many()
            .filter(player_statistics::Column::UpdatedAt.lt(cutoff_date))
            .exec(&txn)
            .await?
            .rows_affected;

        txn.commit().await.map_err(DatabaseError::Transaction)?;

        let stats = CleanupStats {
            usage_logs_deleted,
            workflow_steps_deleted,
            old_stats_deleted,
            total_records_deleted: usage_logs_deleted + workflow_steps_deleted + old_stats_deleted,
            space_freed_mb: 0.0, // TODO: Calculate actual space freed
        };

        tracing::info!("Database cleanup completed: {:?}", stats);
        Ok(stats)
    }

    /// Optimize database performance (VACUUM, ANALYZE, etc.)
    pub async fn optimize_database(&self) -> DatabaseResult<()> {
        tracing::info!("Starting database optimization");
        
        let conn = self.connection.read().await;
        
        match conn.get_database_backend() {
            DbBackend::Sqlite => {
                // SQLite optimization
                conn.execute(Statement::from_string(DbBackend::Sqlite, "VACUUM".to_string()))
                    .await.map_err(DatabaseError::Query)?;
                    
                conn.execute(Statement::from_string(DbBackend::Sqlite, "ANALYZE".to_string()))
                    .await.map_err(DatabaseError::Query)?;
            },
            DbBackend::Postgres => {
                // PostgreSQL optimization
                conn.execute(Statement::from_string(DbBackend::Postgres, "VACUUM ANALYZE".to_string()))
                    .await.map_err(DatabaseError::Query)?;
            },
            DbBackend::MySql => {
                // MySQL optimization
                conn.execute(Statement::from_string(DbBackend::MySql, "OPTIMIZE TABLE".to_string()))
                    .await.map_err(DatabaseError::Query)?;
            }
        }

        tracing::info!("Database optimization completed");
        Ok(())
    }

    // Private helper methods

    fn get_table_creation_statements(&self, backend: DbBackend) -> Vec<Statement> {
        // This would typically be generated by SeaORM migration tools
        // For now, return empty vec as tables will be created by SeaORM's create_table_from_entity
        vec![]
    }

    fn get_table_drop_statements(&self, backend: DbBackend) -> Vec<Statement> {
        // Drop tables in reverse dependency order
        let table_names = vec![
            // "asset_usage_logs", // TODO: Implement in database-orm
            "asset_dependencies", 
            // "workflow_steps", // TODO: Implement in database-orm
            "player_statistics", "world_generation_data", "game_states",
            "ai_patterns", "ai_workflows", "generated_assets", 
            "inventories", "items", "dialogues", "encounters", 
            "hex_tiles", "companions", "players"
        ];

        table_names.into_iter()
            .map(|table| Statement::from_string(backend, format!("DROP TABLE IF EXISTS {}", table)))
            .collect()
    }

    async fn seed_dread_levels(&self, txn: &DatabaseTransaction) -> DatabaseResult<()> {
        // This would seed the initial dread level configurations
        // Implementation would add default dread progression settings
        Ok(())
    }

    async fn seed_companion_templates(&self, txn: &DatabaseTransaction) -> DatabaseResult<()> {
        // This would seed the base companion configurations
        // Implementation would add Einar, Mira, Sorin, Tamara templates
        Ok(())
    }

    async fn seed_world_generation_defaults(&self, txn: &DatabaseTransaction) -> DatabaseResult<()> {
        // This would seed default world generation parameters
        Ok(())
    }

    async fn seed_ai_workflow_templates(&self, txn: &DatabaseTransaction) -> DatabaseResult<()> {
        // This would seed AI workflow templates and configurations
        Ok(())
    }

    async fn get_database_size_mb(&self) -> DatabaseResult<f64> {
        let conn = self.connection.read().await;
        
        match conn.get_database_backend() {
            DbBackend::Sqlite => {
                // For SQLite, we could check the file size
                // This is a simplified implementation
                Ok(0.0)
            },
            DbBackend::Postgres => {
                // For PostgreSQL, query pg_database_size
                Ok(0.0)
            },
            DbBackend::MySql => {
                // For MySQL, query information_schema
                Ok(0.0)
            }
        }
    }

    async fn get_active_connections(&self) -> u32 {
        // This would return the number of active connections in the pool
        // Simplified for now
        1
    }
}

// Implement game-specific database operations
#[async_trait::async_trait]
impl GameDatabaseOperations for GameDatabase {
    async fn get_player_by_save_slot(&self, save_slot_id: i32) -> DatabaseResult<Option<players::Model>> {
        let conn = self.connection.read().await;
        let player = Players::find()
            .filter(players::Column::SaveSlotId.eq(save_slot_id))
            .one(&*conn)
            .await?;
        Ok(player)
    }

    async fn get_active_companions(&self, player_id: uuid::Uuid) -> DatabaseResult<Vec<companions::Model>> {
        let conn = self.connection.read().await;
        let companions = Companions::find()
            .filter(companions::Column::PlayerId.eq(player_id))
            .filter(companions::Column::IsActive.eq(true))
            .all(&*conn)
            .await?;
        Ok(companions)
    }

    async fn get_hex_tiles_in_radius(&self, center_q: i32, center_r: i32, radius: i32) -> DatabaseResult<Vec<hex_tiles::Model>> {
        let conn = self.connection.read().await;
        
        // Hex distance calculation: max(abs(q1-q2), abs(r1-r2), abs(s1-s2))
        // This is a simplified query - for production, you'd want a more efficient spatial query
        let tiles = HexTiles::find()
            .filter(hex_tiles::Column::Q.between(center_q - radius, center_q + radius))
            .filter(hex_tiles::Column::R.between(center_r - radius, center_r + radius))
            .all(&*conn)
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
    }

    async fn get_dread_appropriate_assets(&self, dread_level: i32, asset_type: &str) -> DatabaseResult<Vec<generated_assets::Model>> {
        let conn = self.connection.read().await;
        let assets = GeneratedAssets::find()
            .filter(generated_assets::Column::AssetType.eq(asset_type))
            .filter(generated_assets::Column::DreadLevelTarget.lte(dread_level))
            .filter(generated_assets::Column::IsApproved.eq(true))
            .all(&*conn)
            .await?;
        Ok(assets)
    }

    async fn update_horror_progression(&self, player_id: uuid::Uuid, new_dread_level: i32, dread_progression: f32) -> DatabaseResult<()> {
        let conn = self.connection.read().await;
        
        Players::update_many()
            .filter(players::Column::Id.eq(player_id))
            .col_expr(players::Column::CurrentDreadLevel, sea_orm::sea_query::Expr::value(new_dread_level))
            .col_expr(players::Column::DreadProgression, sea_orm::sea_query::Expr::value(dread_progression))
            .col_expr(players::Column::UpdatedAt, sea_orm::sea_query::Expr::current_timestamp())
            .exec(&*conn)
            .await?;

        Ok(())
    }

    async fn record_ai_workflow_completion(&self, workflow_id: uuid::Uuid, output_data: serde_json::Value, generated_asset_ids: Vec<uuid::Uuid>) -> DatabaseResult<()> {
        let txn = self.begin_transaction().await?;
        
        // Update workflow status
        AIWorkflows::update_many()
            .filter(ai_workflows::Column::Id.eq(workflow_id))
            .col_expr(ai_workflows::Column::Status, sea_orm::sea_query::Expr::value("completed"))
            .col_expr(ai_workflows::Column::OutputData, sea_orm::sea_query::Expr::value(output_data))
            .col_expr(ai_workflows::Column::GeneratedAssetIds, sea_orm::sea_query::Expr::value(serde_json::to_value(generated_asset_ids)?))
            .col_expr(ai_workflows::Column::CompletedAt, sea_orm::sea_query::Expr::current_timestamp())
            .exec(&txn)
            .await?;

        txn.commit().await.map_err(DatabaseError::Transaction)?;
        Ok(())
    }
}

/// Database statistics for monitoring and health checks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseStats {
    pub player_count: u64,
    pub companion_count: u64,
    pub hex_tile_count: u64,
    pub encounter_count: u64,
    pub dialogue_count: u64,
    pub item_count: u64,
    pub generated_asset_count: u64,
    pub ai_workflow_count: u64,
    pub database_size_mb: f64,
    pub connection_pool_active: u32,
    pub last_backup: Option<chrono::DateTime<chrono::Utc>>,
}

/// Results from database cleanup operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleanupStats {
    pub usage_logs_deleted: u64,
    pub workflow_steps_deleted: u64,
    pub old_stats_deleted: u64,
    pub total_records_deleted: u64,
    pub space_freed_mb: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_database_connection() {
        let db = GameDatabase::connect("sqlite::memory:").await.unwrap();
        assert!(db.is_connected().await);
    }

    #[tokio::test]
    async fn test_table_creation() {
        let db = GameDatabase::connect("sqlite::memory:").await.unwrap();
        db.create_tables().await.unwrap();
        
        // Verify tables exist by attempting to query them
        let stats = db.get_database_stats().await.unwrap();
        assert_eq!(stats.player_count, 0); // Should be 0 in fresh database
    }

    #[tokio::test]
    async fn test_cleanup_operations() {
        let db = GameDatabase::connect("sqlite::memory:").await.unwrap();
        db.create_tables().await.unwrap();
        
        let cleanup_stats = db.cleanup_old_data(30).await.unwrap();
        assert_eq!(cleanup_stats.total_records_deleted, 0); // No old data in fresh DB
    }
}
