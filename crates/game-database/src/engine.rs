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
use std::path::PathBuf;
use bevy::prelude::*;
use dirs;

use crate::models::{
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

/// Database type for intelligent routing
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DatabaseType {
    /// Read-only game content (bundled with game)
    GameContent,
    /// Read-write player data (stored in user state directory)
    PlayerData,
}

/// Operation classification for intelligent routing
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationType {
    /// World/content queries that should use game.db
    WorldContent,
    /// Player state queries that should use player.db
    PlayerState,
    /// Operations that need coordination between both databases
    Hybrid,
}

/// Dual database connection manager with intelligent routing
#[derive(Clone, Debug)]
pub struct GameDatabase {
    game_db: Arc<RwLock<SeaConnection>>,      // Read-only world content
    player_db: Arc<RwLock<SeaConnection>>,    // Read-write player data
    game_db_url: String,
    player_db_url: String,
    is_connected: Arc<RwLock<bool>>,
}

/// Legacy single database structure for backward compatibility
#[derive(Clone, Debug)]
pub struct LegacyGameDatabase {
    connection: Arc<RwLock<SeaConnection>>,
    database_url: String,
    is_connected: Arc<RwLock<bool>>,
}

/// Type alias for easier usage throughout the codebase
pub type DatabaseConnection = GameDatabase;

impl GameDatabase {
    /// Connect to both game and player databases with intelligent routing
    pub async fn connect_dual(game_db_url: &str, player_db_url: Option<&str>) -> DatabaseResult<Self> {
        tracing::info!("Connecting to dual-database system");
        tracing::info!("Game DB: {}", game_db_url);
        
        // Determine player database location
        let player_db_path = match player_db_url {
            Some(url) => url.to_string(),
            None => {
                // Default to XDG state directory
                let state_dir = dirs::state_dir()
                    .unwrap_or_else(|| dirs::home_dir().unwrap().join(".local/state"))
                    .join("dragons-labyrinth");
                
                std::fs::create_dir_all(&state_dir)
                    .map_err(|e| DatabaseError::Connection(DbErr::Custom(format!("Failed to create state directory: {}", e))))?;
                
                format!("sqlite://{}/player.db", state_dir.display())
            }
        };
        
        tracing::info!("Player DB: {}", player_db_path);
        
        // Connect to game database (read-only content)
        let game_connection = Self::create_connection(game_db_url, DatabaseType::GameContent).await?;
        
        // Connect to player database (read-write save data)
        let player_connection = Self::create_connection(&player_db_path, DatabaseType::PlayerData).await?;
        
        let db = Self {
            game_db: Arc::new(RwLock::new(game_connection)),
            player_db: Arc::new(RwLock::new(player_connection)),
            game_db_url: game_db_url.to_string(),
            player_db_url: player_db_path,
            is_connected: Arc::new(RwLock::new(true)),
        };

        // Verify both connections
        db.verify_connection().await?;
        
        tracing::info!("Successfully connected to dual-database system");
        Ok(db)
    }

    /// Legacy single database connection for backward compatibility
    pub async fn connect(database_url: &str) -> DatabaseResult<LegacyGameDatabase> {
        tracing::info!("Connecting to legacy single database: {}", database_url);
        
        let connection = Self::create_connection(database_url, DatabaseType::GameContent).await?;

        let db = LegacyGameDatabase {
            connection: Arc::new(RwLock::new(connection)),
            database_url: database_url.to_string(),
            is_connected: Arc::new(RwLock::new(true)),
        };

        // Verify connection with a simple query
        db.verify_connection().await?;
        
        tracing::info!("Successfully connected to legacy database");
        Ok(db)
    }

    /// Create optimized database connection based on database type
    async fn create_connection(database_url: &str, db_type: DatabaseType) -> DatabaseResult<SeaConnection> {
        let mut opt = ConnectOptions::new(database_url);
        
        match db_type {
            DatabaseType::GameContent => {
                // Read-only optimizations for game content
                opt.max_connections(50)
                    .min_connections(2)
                    .connect_timeout(Duration::from_secs(10))
                    .acquire_timeout(Duration::from_secs(5))
                    .idle_timeout(Duration::from_secs(1200))
                    .max_lifetime(Duration::from_secs(7200))
                    .sqlx_logging(false); // Less logging for content queries
            },
            DatabaseType::PlayerData => {
                // Read-write optimizations for player data
                opt.max_connections(20)
                    .min_connections(1)
                    .connect_timeout(Duration::from_secs(5))
                    .acquire_timeout(Duration::from_secs(3))
                    .idle_timeout(Duration::from_secs(300))
                    .max_lifetime(Duration::from_secs(1800))
                    .sqlx_logging(true)
                    .sqlx_logging_level(tracing::log::LevelFilter::Debug);
            }
        }

        Database::connect(opt)
            .await
            .map_err(DatabaseError::Connection)
    }

    /// Intelligent query routing based on operation type
    pub async fn route_connection(&self, operation_type: OperationType) -> Arc<RwLock<SeaConnection>> {
        match operation_type {
            OperationType::WorldContent => self.game_db.clone(),
            OperationType::PlayerState => self.player_db.clone(),
            OperationType::Hybrid => {
                // For hybrid operations, default to player database
                // Systems can explicitly call both if needed
                self.player_db.clone()
            }
        }
    }

    /// Get specific database connection
    pub async fn get_connection(&self, db_type: DatabaseType) -> Arc<RwLock<SeaConnection>> {
        match db_type {
            DatabaseType::GameContent => self.game_db.clone(),
            DatabaseType::PlayerData => self.player_db.clone(),
        }
    }

    /// Verify both database connections are working
    pub async fn verify_connection(&self) -> DatabaseResult<()> {
        // Test game database
        let game_conn = self.game_db.read().await;
        game_conn.ping().await.map_err(DatabaseError::Connection)?;
        
        // Test player database
        let player_conn = self.player_db.read().await;
        player_conn.ping().await.map_err(DatabaseError::Connection)?;
        
        Ok(())
    }

    /// Check if both databases are connected
    pub async fn is_connected(&self) -> bool {
        *self.is_connected.read().await
    }

    /// Get connection for backward compatibility (defaults to player database)
    pub async fn connection(&self) -> Arc<RwLock<SeaConnection>> {
        self.player_db.clone()
    }

    /// Begin a transaction on the specified database
    pub async fn begin_transaction(&self, db_type: DatabaseType) -> DatabaseResult<DatabaseTransaction> {
        let conn = match db_type {
            DatabaseType::GameContent => self.game_db.read().await,
            DatabaseType::PlayerData => self.player_db.read().await,
        };
        conn.begin().await.map_err(DatabaseError::Transaction)
    }

    /// Begin coordinated transactions on both databases for hybrid operations
    pub async fn begin_dual_transaction(&self) -> DatabaseResult<(DatabaseTransaction, DatabaseTransaction)> {
        let game_txn = self.begin_transaction(DatabaseType::GameContent).await?;
        let player_txn = self.begin_transaction(DatabaseType::PlayerData).await?;
        Ok((game_txn, player_txn))
    }

    /// Create all database tables in both databases
    pub async fn create_tables(&self) -> DatabaseResult<()> {
        tracing::info!("Creating database tables for Dragon's Labyrinth dual-database system");
        
        // Create game content tables (read-only)
        self.create_game_content_tables().await?;
        
        // Create player data tables (read-write)
        self.create_player_data_tables().await?;

        tracing::info!("Successfully created all database tables in dual-database system");
        Ok(())
    }

    /// Create tables for game content database (read-only)
    async fn create_game_content_tables(&self) -> DatabaseResult<()> {
        let conn = self.game_db.read().await;
        let builder = conn.get_database_backend();
        
        tracing::info!("Creating game content tables (read-only)");
        
        // Game content tables: hex_tiles, encounters, dialogues, items, etc.
        let table_creation_statements = self.get_game_content_table_statements(builder);
        
        for statement in table_creation_statements {
            conn.execute(statement).await.map_err(DatabaseError::Migration)?;
        }

        Ok(())
    }

    /// Create tables for player data database (read-write)
    async fn create_player_data_tables(&self) -> DatabaseResult<()> {
        let conn = self.player_db.read().await;
        let builder = conn.get_database_backend();
        
        tracing::info!("Creating player data tables (read-write)");
        
        // Player data tables: players, companions, player_statistics, game_states, etc.
        let table_creation_statements = self.get_player_data_table_statements(builder);
        
        for statement in table_creation_statements {
            conn.execute(statement).await.map_err(DatabaseError::Migration)?;
        }

        Ok(())
    }

    /// Drop all tables from both databases (careful!)
    pub async fn drop_all_tables(&self) -> DatabaseResult<()> {
        tracing::warn!("Dropping all database tables from dual-database system - this is destructive!");
        
        // Drop game content tables
        let game_conn = self.game_db.read().await;
        let game_builder = game_conn.get_database_backend();
        let game_drop_statements = self.get_game_content_drop_statements(game_builder);
        
        for statement in game_drop_statements {
            game_conn.execute(statement).await.map_err(DatabaseError::Migration)?;
        }

        // Drop player data tables
        let player_conn = self.player_db.read().await;
        let player_builder = player_conn.get_database_backend();
        let player_drop_statements = self.get_player_data_drop_statements(player_builder);
        
        for statement in player_drop_statements {
            player_conn.execute(statement).await.map_err(DatabaseError::Migration)?;
        }

        Ok(())
    }

    /// Seed initial data in both databases
    pub async fn seed_initial_data(&self) -> DatabaseResult<()> {
        tracing::info!("Seeding initial data in dual-database system");
        
        // Seed game content data (templates, world generation defaults)
        self.seed_game_content_data().await?;
        
        // Seed player data defaults (if any needed)
        self.seed_player_data_defaults().await?;
        
        tracing::info!("Successfully seeded initial data in dual-database system");
        Ok(())
    }

    /// Seed game content data (read-only templates and configurations)
    async fn seed_game_content_data(&self) -> DatabaseResult<()> {
        let txn = self.begin_transaction(DatabaseType::GameContent).await?;
        
        // Seed world generation parameters in game database
        self.seed_world_generation_defaults(&txn).await?;
        
        // Seed AI workflow templates in game database
        self.seed_ai_workflow_templates(&txn).await?;
        
        txn.commit().await.map_err(DatabaseError::Transaction)?;
        Ok(())
    }

    /// Seed player data defaults (runtime configurations)
    async fn seed_player_data_defaults(&self) -> DatabaseResult<()> {
        let txn = self.begin_transaction(DatabaseType::PlayerData).await?;
        
        // Seed initial dread levels configuration in player database
        self.seed_dread_levels(&txn).await?;
        
        // Seed companion base configurations in player database
        self.seed_companion_templates(&txn).await?;
        
        txn.commit().await.map_err(DatabaseError::Transaction)?;
        Ok(())
    }

    /// Get comprehensive statistics from both databases
    pub async fn get_database_stats(&self) -> DatabaseResult<DualDatabaseStats> {
        // Get game content statistics
        let game_conn = self.game_db.read().await;
        let hex_tile_count = HexTiles::find().count(&*game_conn).await?;
        let encounter_count = Encounters::find().count(&*game_conn).await?;
        let dialogue_count = Dialogues::find().count(&*game_conn).await?;
        let item_count = Items::find().count(&*game_conn).await?;
        let generated_asset_count = GeneratedAssets::find().count(&*game_conn).await?;
        let ai_workflow_count = AIWorkflows::find().count(&*game_conn).await?;
        
        // Get player data statistics
        let player_conn = self.player_db.read().await;
        let player_count = Players::find().count(&*player_conn).await?;
        let companion_count = Companions::find().count(&*player_conn).await?;
        
        Ok(DualDatabaseStats {
            // Game content counts
            hex_tile_count,
            encounter_count,
            dialogue_count,
            item_count,
            generated_asset_count,
            ai_workflow_count,
            
            // Player data counts
            player_count,
            companion_count,
            
            // Database metrics
            game_db_size_mb: self.get_database_size_mb(DatabaseType::GameContent).await?,
            player_db_size_mb: self.get_database_size_mb(DatabaseType::PlayerData).await?,
            game_db_connections: self.get_active_connections(DatabaseType::GameContent).await,
            player_db_connections: self.get_active_connections(DatabaseType::PlayerData).await,
            last_backup: None, // TODO: Implement backup tracking
        })
    }

    /// Cleanup old data from player database (game content remains untouched)
    pub async fn cleanup_old_data(&self, older_than_days: i32) -> DatabaseResult<CleanupStats> {
        tracing::info!("Starting player database cleanup for data older than {} days", older_than_days);
        
        // Only clean player data - game content is read-only
        let txn = self.begin_transaction(DatabaseType::PlayerData).await?;
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

        tracing::info!("Player database cleanup completed: {:?}", stats);
        Ok(stats)
    }

    /// Optimize both databases for performance
    pub async fn optimize_database(&self) -> DatabaseResult<()> {
        tracing::info!("Starting dual-database optimization");
        
        // Optimize game content database
        self.optimize_single_database(DatabaseType::GameContent).await?;
        
        // Optimize player data database
        self.optimize_single_database(DatabaseType::PlayerData).await?;

        tracing::info!("Dual-database optimization completed");
        Ok(())
    }

    /// Optimize a single database
    async fn optimize_single_database(&self, db_type: DatabaseType) -> DatabaseResult<()> {
        let conn = match db_type {
            DatabaseType::GameContent => self.game_db.read().await,
            DatabaseType::PlayerData => self.player_db.read().await,
        };
        
        let db_name = match db_type {
            DatabaseType::GameContent => "game content",
            DatabaseType::PlayerData => "player data",
        };
        
        tracing::info!("Optimizing {} database", db_name);
        
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

        Ok(())
    }

    // Private helper methods for dual-database management

    fn get_game_content_table_statements(&self, backend: DbBackend) -> Vec<Statement> {
        // Game content tables (read-only, bundled with game)
        // hex_tiles, encounters, dialogues, items, generated_assets, ai_workflows, etc.
        vec![]
    }

    fn get_player_data_table_statements(&self, backend: DbBackend) -> Vec<Statement> {
        // Player data tables (read-write, stored in user state directory)
        // players, companions, player_statistics, game_states, forge_progress, etc.
        vec![]
    }

    fn get_game_content_drop_statements(&self, backend: DbBackend) -> Vec<Statement> {
        // Drop game content tables in reverse dependency order
        let table_names = vec![
            "asset_dependencies", 
            "ai_workflows", "generated_assets", 
            "items", "dialogues", "encounters", 
            "hex_tiles",
        ];

        table_names.into_iter()
            .map(|table| Statement::from_string(backend, format!("DROP TABLE IF EXISTS {}", table)))
            .collect()
    }

    fn get_player_data_drop_statements(&self, backend: DbBackend) -> Vec<Statement> {
        // Drop player data tables in reverse dependency order
        let table_names = vec![
            "player_statistics", "game_states",
            "forge_progress", "companion_therapy", "philosophical_progression",
            "world_corruption", "mount_companions",
            "companions", "players"
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

    async fn get_database_size_mb(&self, db_type: DatabaseType) -> DatabaseResult<f64> {
        let conn = match db_type {
            DatabaseType::GameContent => self.game_db.read().await,
            DatabaseType::PlayerData => self.player_db.read().await,
        };
        
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

    async fn get_active_connections(&self, db_type: DatabaseType) -> u32 {
        // This would return the number of active connections in the pool
        // Simplified for now
        1
    }
}

// Implement intelligent dual-database operations
#[async_trait::async_trait]
impl GameDatabaseOperations for GameDatabase {
    async fn get_player_by_save_slot(&self, save_slot_id: i32) -> DatabaseResult<Option<players::Model>> {
        // Player data -> player.db
        let conn = self.route_connection(OperationType::PlayerState).await;
        let player = Players::find()
            .filter(players::Column::SaveSlotId.eq(save_slot_id))
            .one(&*conn.read().await)
            .await?;
        Ok(player)
    }

    async fn get_active_companions(&self, player_id: uuid::Uuid) -> DatabaseResult<Vec<companions::Model>> {
        // Player's companions -> player.db
        let conn = self.route_connection(OperationType::PlayerState).await;
        let companions = Companions::find()
            .filter(companions::Column::PlayerId.eq(player_id))
            .filter(companions::Column::IsActive.eq(true))
            .all(&*conn.read().await)
            .await?;
        Ok(companions)
    }

    async fn get_hex_tiles_in_radius(&self, center_q: i32, center_r: i32, radius: i32) -> DatabaseResult<Vec<hex_tiles::Model>> {
        // World content -> game.db
        let conn = self.route_connection(OperationType::WorldContent).await;
        
        // Hex distance calculation: max(abs(q1-q2), abs(r1-r2), abs(s1-s2))
        // This is a simplified query - for production, you'd want a more efficient spatial query
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
    }

    async fn get_dread_appropriate_assets(&self, dread_level: i32, asset_type: &str) -> DatabaseResult<Vec<generated_assets::Model>> {
        // Asset templates -> game.db
        let conn = self.route_connection(OperationType::WorldContent).await;
        let assets = GeneratedAssets::find()
            .filter(generated_assets::Column::AssetType.eq(asset_type))
            .filter(generated_assets::Column::DreadLevelTarget.lte(dread_level))
            .filter(generated_assets::Column::IsApproved.eq(true))
            .all(&*conn.read().await)
            .await?;
        Ok(assets)
    }

    async fn update_horror_progression(&self, player_id: uuid::Uuid, new_dread_level: i32, dread_progression: f32) -> DatabaseResult<()> {
        // Player progression -> player.db
        let conn = self.route_connection(OperationType::PlayerState).await;
        
        Players::update_many()
            .filter(players::Column::Id.eq(player_id))
            .col_expr(players::Column::CurrentDreadLevel, sea_orm::sea_query::Expr::value(new_dread_level))
            .col_expr(players::Column::DreadProgression, sea_orm::sea_query::Expr::value(dread_progression))
            .col_expr(players::Column::UpdatedAt, sea_orm::sea_query::Expr::current_timestamp())
            .exec(&*conn.read().await)
            .await?;

        Ok(())
    }

    async fn record_ai_workflow_completion(&self, workflow_id: uuid::Uuid, output_data: serde_json::Value, generated_asset_ids: Vec<uuid::Uuid>) -> DatabaseResult<()> {
        // AI workflow completion -> game.db (content generation)
        let txn = self.begin_transaction(DatabaseType::GameContent).await?;
        
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

/// Comprehensive database statistics for dual-database system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DualDatabaseStats {
    // Game content counts (from game.db)
    pub hex_tile_count: u64,
    pub encounter_count: u64,
    pub dialogue_count: u64,
    pub item_count: u64,
    pub generated_asset_count: u64,
    pub ai_workflow_count: u64,
    
    // Player data counts (from player.db)
    pub player_count: u64,
    pub companion_count: u64,
    
    // Database metrics
    pub game_db_size_mb: f64,
    pub player_db_size_mb: f64,
    pub game_db_connections: u32,
    pub player_db_connections: u32,
    pub last_backup: Option<chrono::DateTime<chrono::Utc>>,
}

/// Legacy database statistics for backward compatibility
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

// Legacy GameDatabase implementation for backward compatibility
impl LegacyGameDatabase {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_legacy_database_connection() {
        let db = GameDatabase::connect("sqlite::memory:").await.unwrap();
        assert!(db.is_connected().await);
    }

    #[tokio::test]
    async fn test_dual_database_connection() {
        let db = GameDatabase::connect_dual("sqlite::memory:", Some("sqlite::memory:")).await.unwrap();
        assert!(db.is_connected().await);
    }

    #[tokio::test]
    async fn test_dual_database_routing() {
        let db = GameDatabase::connect_dual("sqlite::memory:", Some("sqlite::memory:")).await.unwrap();
        
        // Test routing to different databases
        let game_conn = db.route_connection(OperationType::WorldContent).await;
        let player_conn = db.route_connection(OperationType::PlayerState).await;
        
        // Should be different connections
        assert!(!Arc::ptr_eq(&game_conn, &player_conn));
    }

    #[tokio::test]
    async fn test_dual_database_tables() {
        let db = GameDatabase::connect_dual("sqlite::memory:", Some("sqlite::memory:")).await.unwrap();
        db.create_tables().await.unwrap();
        
        // Verify both databases are accessible
        let stats = db.get_database_stats().await.unwrap();
        assert_eq!(stats.player_count, 0); // Should be 0 in fresh database
    }
}
