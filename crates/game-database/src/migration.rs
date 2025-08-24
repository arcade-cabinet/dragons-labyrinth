//! Database Migration System for Dragon's Labyrinth Dual-Database Architecture
//!
//! This module handles migration between single-database and dual-database systems,
//! data distribution, and schema evolution for both game content and player data.

use sea_orm::{
    Database, DatabaseConnection as SeaConnection, DatabaseTransaction, 
    ConnectOptions, EntityTrait, QueryFilter, ColumnTrait, Statement, DbBackend,
    TransactionTrait, Set, ActiveModelTrait
};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

use database_orm::{
    players, companions, hex_tiles, encounters, dialogues, items,
    player_statistics, game_states, ai_workflows, generated_assets,
    Players, Companions, HexTiles, Encounters, Dialogues, Items,
    PlayerStatistics, GameStates, AiWorkflows, GeneratedAssets,
};

use crate::engine::{GameDatabase, DatabaseType};
use crate::error::{DatabaseError, DatabaseResult};

/// Migration orchestrator for dual-database system
pub struct DatabaseMigrator {
    source_db_url: String,
    game_db_url: String,
    player_db_url: String,
}

/// Migration status and progress tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationStatus {
    pub migration_id: Uuid,
    pub migration_type: MigrationType,
    pub status: MigrationState,
    pub progress_percentage: f32,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub records_migrated: u64,
    pub errors_encountered: u32,
    pub estimated_time_remaining: Option<i64>,
}

/// Types of migrations supported
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MigrationType {
    /// Migrate from single database to dual-database system
    SingleToDual,
    /// Migrate from dual-database back to single (for compatibility)
    DualToSingle,
    /// Update schema versions within dual-database system
    SchemaUpgrade,
    /// Redistribute data between game.db and player.db
    DataRedistribution,
}

/// Migration execution state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MigrationState {
    Planned,
    InProgress,
    Completed,
    Failed,
    Paused,
    RolledBack,
}

/// Data classification for migration routing
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataCategory {
    /// Static world content that should go to game.db
    WorldContent,
    /// Player save data that should go to player.db
    PlayerData,
    /// Hybrid data that needs special handling
    Hybrid,
}

impl DatabaseMigrator {
    /// Create a new migrator for single-to-dual database conversion
    pub fn new(source_db_url: &str, game_db_url: &str, player_db_url: &str) -> Self {
        Self {
            source_db_url: source_db_url.to_string(),
            game_db_url: game_db_url.to_string(),
            player_db_url: player_db_url.to_string(),
        }
    }

    /// Execute migration from single database to dual-database system
    pub async fn migrate_single_to_dual(&self) -> DatabaseResult<MigrationStatus> {
        let migration_id = Uuid::new_v4();
        tracing::info!("Starting single-to-dual database migration: {}", migration_id);

        let mut status = MigrationStatus {
            migration_id,
            migration_type: MigrationType::SingleToDual,
            status: MigrationState::InProgress,
            progress_percentage: 0.0,
            started_at: Utc::now(),
            completed_at: None,
            records_migrated: 0,
            errors_encountered: 0,
            estimated_time_remaining: None,
        };

        // Step 1: Connect to source database
        tracing::info!("Connecting to source database");
        let source_conn = Database::connect(&self.source_db_url)
            .await
            .map_err(DatabaseError::Connection)?;

        // Step 2: Create and connect to target databases
        tracing::info!("Creating dual-database connections");
        let dual_db = GameDatabase::connect_dual(&self.game_db_url, Some(&self.player_db_url)).await?;
        dual_db.create_tables().await?;

        // Step 3: Analyze source data
        tracing::info!("Analyzing source data distribution");
        let data_analysis = self.analyze_source_data(&source_conn).await?;
        status.progress_percentage = 10.0;

        // Step 4: Migrate world content to game.db
        tracing::info!("Migrating world content to game.db");
        let world_records = self.migrate_world_content(&source_conn, &dual_db).await?;
        status.records_migrated += world_records;
        status.progress_percentage = 50.0;

        // Step 5: Migrate player data to player.db
        tracing::info!("Migrating player data to player.db");
        let player_records = self.migrate_player_data(&source_conn, &dual_db).await?;
        status.records_migrated += player_records;
        status.progress_percentage = 90.0;

        // Step 6: Verify migration integrity
        tracing::info!("Verifying migration integrity");
        let verification_result = self.verify_migration_integrity(&source_conn, &dual_db).await?;
        if !verification_result.is_valid {
            return Err(DatabaseError::Migration(format!(
                "Migration verification failed: {:?}", verification_result.errors
            )));
        }

        status.status = MigrationState::Completed;
        status.completed_at = Some(Utc::now());
        status.progress_percentage = 100.0;

        tracing::info!("Migration completed successfully: {} records migrated", status.records_migrated);
        Ok(status)
    }

    /// Analyze source database to understand data distribution
    async fn analyze_source_data(&self, source_conn: &SeaConnection) -> DatabaseResult<DataAnalysis> {
        let hex_tile_count = HexTiles::find().count(source_conn).await?;
        let encounter_count = Encounters::find().count(source_conn).await?;
        let dialogue_count = Dialogues::find().count(source_conn).await?;
        let item_count = Items::find().count(source_conn).await?;
        let generated_asset_count = GeneratedAssets::find().count(source_conn).await?;
        let ai_workflow_count = AiWorkflows::find().count(source_conn).await?;
        
        let player_count = Players::find().count(source_conn).await?;
        let companion_count = Companions::find().count(source_conn).await?;
        let player_stats_count = PlayerStatistics::find().count(source_conn).await?;
        let game_state_count = GameStates::find().count(source_conn).await?;

        let world_content_size = hex_tile_count + encounter_count + dialogue_count + 
                                item_count + generated_asset_count + ai_workflow_count;
        let player_data_size = player_count + companion_count + player_stats_count + game_state_count;

        Ok(DataAnalysis {
            total_records: world_content_size + player_data_size,
            world_content_records: world_content_size,
            player_data_records: player_data_size,
            estimated_game_db_size_mb: (world_content_size as f64 * 2.5) / 1024.0, // Rough estimate
            estimated_player_db_size_mb: (player_data_size as f64 * 1.0) / 1024.0,
            migration_complexity: if world_content_size > 50000 { "High" } else { "Medium" }.to_string(),
        })
    }

    /// Migrate world content data to game.db
    async fn migrate_world_content(&self, source_conn: &SeaConnection, dual_db: &GameDatabase) -> DatabaseResult<u64> {
        let game_conn = dual_db.get_connection(DatabaseType::GameContent).await;
        let game_conn = game_conn.read().await;
        let mut records_migrated = 0u64;

        // Migrate hex tiles (world geography)
        tracing::info!("Migrating hex tiles to game.db");
        let hex_tiles = HexTiles::find().all(source_conn).await?;
        for tile in hex_tiles {
            let mut tile_active: hex_tiles::ActiveModel = tile.into();
            tile_active.insert(&*game_conn).await?;
            records_migrated += 1;
        }

        // Migrate encounters (world events)
        tracing::info!("Migrating encounters to game.db");
        let encounters = Encounters::find().all(source_conn).await?;
        for encounter in encounters {
            let mut encounter_active: encounters::ActiveModel = encounter.into();
            encounter_active.insert(&*game_conn).await?;
            records_migrated += 1;
        }

        // Migrate dialogues (world conversation templates)
        tracing::info!("Migrating dialogues to game.db");
        let dialogues = Dialogues::find().all(source_conn).await?;
        for dialogue in dialogues {
            let mut dialogue_active: dialogues::ActiveModel = dialogue.into();
            dialogue_active.insert(&*game_conn).await?;
            records_migrated += 1;
        }

        // Migrate items (world item templates)
        tracing::info!("Migrating items to game.db");
        let items = Items::find().all(source_conn).await?;
        for item in items {
            let mut item_active: items::ActiveModel = item.into();
            item_active.insert(&*game_conn).await?;
            records_migrated += 1;
        }

        // Migrate generated assets (world content assets)
        tracing::info!("Migrating generated assets to game.db");
        let assets = GeneratedAssets::find().all(source_conn).await?;
        for asset in assets {
            let mut asset_active: generated_assets::ActiveModel = asset.into();
            asset_active.insert(&*game_conn).await?;
            records_migrated += 1;
        }

        // Migrate AI workflows (content generation workflows)
        tracing::info!("Migrating AI workflows to game.db");
        let workflows = AiWorkflows::find().all(source_conn).await?;
        for workflow in workflows {
            let mut workflow_active: ai_workflows::ActiveModel = workflow.into();
            workflow_active.insert(&*game_conn).await?;
            records_migrated += 1;
        }

        tracing::info!("Migrated {} world content records to game.db", records_migrated);
        Ok(records_migrated)
    }

    /// Migrate player data to player.db
    async fn migrate_player_data(&self, source_conn: &SeaConnection, dual_db: &GameDatabase) -> DatabaseResult<u64> {
        let player_conn = dual_db.get_connection(DatabaseType::PlayerData).await;
        let player_conn = player_conn.read().await;
        let mut records_migrated = 0u64;

        // Migrate players (save data)
        tracing::info!("Migrating players to player.db");
        let players = Players::find().all(source_conn).await?;
        for player in players {
            let mut player_active: players::ActiveModel = player.into();
            player_active.insert(&*player_conn).await?;
            records_migrated += 1;
        }

        // Migrate companions (player's companion relationships)
        tracing::info!("Migrating companions to player.db");
        let companions = Companions::find().all(source_conn).await?;
        for companion in companions {
            let mut companion_active: companions::ActiveModel = companion.into();
            companion_active.insert(&*player_conn).await?;
            records_migrated += 1;
        }

        // Migrate player statistics (save data analytics)
        tracing::info!("Migrating player statistics to player.db");
        let player_stats = PlayerStatistics::find().all(source_conn).await?;
        for stat in player_stats {
            let mut stat_active: player_statistics::ActiveModel = stat.into();
            stat_active.insert(&*player_conn).await?;
            records_migrated += 1;
        }

        // Migrate game states (player's world state)
        tracing::info!("Migrating game states to player.db");
        let game_states = GameStates::find().all(source_conn).await?;
        for state in game_states {
            let mut state_active: game_states::ActiveModel = state.into();
            state_active.insert(&*player_conn).await?;
            records_migrated += 1;
        }

        tracing::info!("Migrated {} player data records to player.db", records_migrated);
        Ok(records_migrated)
    }

    /// Verify migration integrity between source and target databases
    async fn verify_migration_integrity(&self, source_conn: &SeaConnection, dual_db: &GameDatabase) -> DatabaseResult<MigrationVerification> {
        let mut verification = MigrationVerification {
            is_valid: true,
            errors: Vec::new(),
            source_record_counts: HashMap::new(),
            target_record_counts: HashMap::new(),
        };

        // Verify world content in game.db
        let game_conn = dual_db.get_connection(DatabaseType::GameContent).await;
        let game_conn = game_conn.read().await;
        
        let source_hex_count = HexTiles::find().count(source_conn).await?;
        let target_hex_count = HexTiles::find().count(&*game_conn).await?;
        verification.source_record_counts.insert("hex_tiles".to_string(), source_hex_count);
        verification.target_record_counts.insert("hex_tiles".to_string(), target_hex_count);
        
        if source_hex_count != target_hex_count {
            verification.is_valid = false;
            verification.errors.push(format!(
                "Hex tiles count mismatch: source={}, target={}", 
                source_hex_count, target_hex_count
            ));
        }

        // Verify player data in player.db
        let player_conn = dual_db.get_connection(DatabaseType::PlayerData).await;
        let player_conn = player_conn.read().await;
        
        let source_player_count = Players::find().count(source_conn).await?;
        let target_player_count = Players::find().count(&*player_conn).await?;
        verification.source_record_counts.insert("players".to_string(), source_player_count);
        verification.target_record_counts.insert("players".to_string(), target_player_count);
        
        if source_player_count != target_player_count {
            verification.is_valid = false;
            verification.errors.push(format!(
                "Players count mismatch: source={}, target={}", 
                source_player_count, target_player_count
            ));
        }

        // Verify companions migration
        let source_companion_count = Companions::find().count(source_conn).await?;
        let target_companion_count = Companions::find().count(&*player_conn).await?;
        verification.source_record_counts.insert("companions".to_string(), source_companion_count);
        verification.target_record_counts.insert("companions".to_string(), target_companion_count);
        
        if source_companion_count != target_companion_count {
            verification.is_valid = false;
            verification.errors.push(format!(
                "Companions count mismatch: source={}, target={}", 
                source_companion_count, target_companion_count
            ));
        }

        Ok(verification)
    }

    /// Classify data by type for migration routing
    pub fn classify_table(&self, table_name: &str) -> DataCategory {
        match table_name {
            // World content tables -> game.db
            "hex_tiles" | "encounters" | "dialogues" | "items" | 
            "generated_assets" | "ai_workflows" | "asset_dependencies" |
            "biomes" | "dungeons" | "settlements" | "npcs" |
            "world_generation_data" => DataCategory::WorldContent,
            
            // Player data tables -> player.db
            "players" | "companions" | "player_statistics" | "game_states" |
            "forge_progress" | "companion_therapy" | "philosophical_progression" |
            "world_corruption" | "mount_companions" | "player_preferences" |
            "save_games" | "achievements" => DataCategory::PlayerData,
            
            // Hybrid tables need special handling
            _ => DataCategory::Hybrid,
        }
    }

    /// Create game distribution package
    pub async fn create_game_distribution(&self, output_dir: &Path) -> DatabaseResult<DistributionPackage> {
        tracing::info!("Creating game distribution package");
        
        let game_db_path = output_dir.join("game.db");
        let distribution_manifest_path = output_dir.join("distribution.json");
        
        // Copy game.db to distribution directory
        std::fs::create_dir_all(output_dir)
            .map_err(|e| DatabaseError::Migration(format!("Failed to create output directory: {}", e)))?;
        
        std::fs::copy(&self.game_db_url.replace("sqlite://", ""), &game_db_path)
            .map_err(|e| DatabaseError::Migration(format!("Failed to copy game.db: {}", e)))?;

        // Create distribution manifest
        let manifest = DistributionManifest {
            game_db_path: "game.db".to_string(),
            database_version: crate::DATABASE_VERSION.to_string(),
            content_hash: self.calculate_content_hash(&game_db_path).await?,
            created_at: Utc::now(),
            compatible_player_db_versions: vec![crate::DATABASE_VERSION.to_string()],
            minimum_engine_version: "1.0.0".to_string(),
        };

        let manifest_json = serde_json::to_string_pretty(&manifest)
            .map_err(|e| DatabaseError::Migration(format!("Failed to serialize manifest: {}", e)))?;
        
        std::fs::write(&distribution_manifest_path, manifest_json)
            .map_err(|e| DatabaseError::Migration(format!("Failed to write manifest: {}", e)))?;

        let package = DistributionPackage {
            game_db_path,
            manifest_path: distribution_manifest_path,
            manifest,
            total_size_bytes: self.calculate_package_size(output_dir).await?,
        };

        tracing::info!("Game distribution package created: {:.2} MB", package.total_size_bytes as f64 / 1024.0 / 1024.0);
        Ok(package)
    }

    /// Setup player database in user state directory
    pub async fn setup_player_database(player_db_url: Option<&str>) -> DatabaseResult<PlayerDatabaseSetup> {
        let player_db_path = match player_db_url {
            Some(url) => url.to_string(),
            None => {
                // Use XDG state directory
                let state_dir = dirs::state_dir()
                    .unwrap_or_else(|| dirs::home_dir().unwrap().join(".local/state"))
                    .join("dragons-labyrinth");
                
                std::fs::create_dir_all(&state_dir)
                    .map_err(|e| DatabaseError::Migration(format!("Failed to create state directory: {}", e)))?;
                
                format!("sqlite://{}/player.db", state_dir.display())
            }
        };

        tracing::info!("Setting up player database: {}", player_db_path);

        // Create player database with optimized settings
        let db = GameDatabase::connect_dual("sqlite::memory:", Some(&player_db_path)).await?;
        db.create_player_data_tables().await?;
        db.seed_player_data_defaults().await?;

        let setup = PlayerDatabaseSetup {
            database_path: player_db_path,
            state_directory: dirs::state_dir()
                .unwrap_or_else(|| dirs::home_dir().unwrap().join(".local/state"))
                .join("dragons-labyrinth"),
            backup_enabled: true,
            auto_cleanup_enabled: true,
            max_save_slots: 10,
        };

        tracing::info!("Player database setup completed");
        Ok(setup)
    }

    /// Calculate content hash for distribution verification
    async fn calculate_content_hash(&self, file_path: &Path) -> DatabaseResult<String> {
        use std::io::Read;
        let mut file = std::fs::File::open(file_path)
            .map_err(|e| DatabaseError::Migration(format!("Failed to open file for hashing: {}", e)))?;
        
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)
            .map_err(|e| DatabaseError::Migration(format!("Failed to read file for hashing: {}", e)))?;
        
        // Simple hash for now - in production would use SHA-256
        let hash = format!("{:x}", md5::compute(&contents));
        Ok(hash)
    }

    /// Calculate total package size
    async fn calculate_package_size(&self, package_dir: &Path) -> DatabaseResult<u64> {
        let mut total_size = 0u64;
        
        if let Ok(entries) = std::fs::read_dir(package_dir) {
            for entry in entries.flatten() {
                if let Ok(metadata) = entry.metadata() {
                    total_size += metadata.len();
                }
            }
        }
        
        Ok(total_size)
    }
}

// Supporting data structures

#[derive(Debug, Clone)]
pub struct DataAnalysis {
    pub total_records: u64,
    pub world_content_records: u64,
    pub player_data_records: u64,
    pub estimated_game_db_size_mb: f64,
    pub estimated_player_db_size_mb: f64,
    pub migration_complexity: String,
}

#[derive(Debug, Clone)]
pub struct MigrationVerification {
    pub is_valid: bool,
    pub errors: Vec<String>,
    pub source_record_counts: HashMap<String, u64>,
    pub target_record_counts: HashMap<String, u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributionManifest {
    pub game_db_path: String,
    pub database_version: String,
    pub content_hash: String,
    pub created_at: DateTime<Utc>,
    pub compatible_player_db_versions: Vec<String>,
    pub minimum_engine_version: String,
}

#[derive(Debug, Clone)]
pub struct DistributionPackage {
    pub game_db_path: PathBuf,
    pub manifest_path: PathBuf,
    pub manifest: DistributionManifest,
    pub total_size_bytes: u64,
}

#[derive(Debug, Clone)]
pub struct PlayerDatabaseSetup {
    pub database_path: String,
    pub state_directory: PathBuf,
    pub backup_enabled: bool,
    pub auto_cleanup_enabled: bool,
    pub max_save_slots: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_classification() {
        let migrator = DatabaseMigrator::new("sqlite::memory:", "sqlite::memory:", "sqlite::memory:");
        
        assert_eq!(migrator.classify_table("hex_tiles"), DataCategory::WorldContent);
        assert_eq!(migrator.classify_table("encounters"), DataCategory::WorldContent);
        assert_eq!(migrator.classify_table("items"), DataCategory::WorldContent);
        
        assert_eq!(migrator.classify_table("players"), DataCategory::PlayerData);
        assert_eq!(migrator.classify_table("companions"), DataCategory::PlayerData);
        assert_eq!(migrator.classify_table("game_states"), DataCategory::PlayerData);
        
        assert_eq!(migrator.classify_table("unknown_table"), DataCategory::Hybrid);
    }

    #[tokio::test]
    async fn test_migration_setup() {
        let migrator = DatabaseMigrator::new("sqlite::memory:", "sqlite::memory:", "sqlite::memory:");
        
        // Test data analysis on empty database
        let source_conn = Database::connect("sqlite::memory:").await.unwrap();
        let analysis = migrator.analyze_source_data(&source_conn).await.unwrap();
        
        assert_eq!(analysis.total_records, 0);
        assert_eq!(analysis.world_content_records, 0);
        assert_eq!(analysis.player_data_records, 0);
    }
}
