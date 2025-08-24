//! Game Database - Comprehensive SeaORM models for Dragon's Labyrinth
//! 
//! This crate provides a complete database abstraction layer for the horror-first RPG,
//! integrating seamlessly with Bevy ECS and supporting the revolutionary AI asset generation
//! pipeline while maintaining clean separation between build-time and runtime systems.

// Models are now included directly in this crate
pub mod models;
pub mod engine;
pub mod router;
pub mod migration;
pub mod traits;
pub mod bevy_integration;
pub mod error;
pub mod tools;
pub mod save_system;
pub mod systems;

pub use engine::{
    DatabaseConnection, GameDatabase, LegacyGameDatabase,
    DatabaseType, OperationType, DualDatabaseStats, DatabaseStats, CleanupStats
};
pub use router::{DatabaseRouter, DatabaseRouterResource};
pub use error::{DatabaseError, DatabaseResult};
// Models are imported directly from database-orm crate
pub use traits::*;
pub use tools::{DatabaseTool, DatabaseQueryParams, execute_database_query};

// Re-export commonly used types
pub use sea_orm::{prelude::*, DatabaseTransaction, TransactionTrait};
pub use uuid::Uuid;
pub use chrono::{DateTime, Utc};
pub use serde::{Deserialize, Serialize};

/// Database version for migration tracking
pub const DATABASE_VERSION: &str = "1.0.0";

/// Initialize the dual-database system with all required tables
pub async fn initialize_dual_database(game_db_url: &str, player_db_url: Option<&str>) -> DatabaseResult<GameDatabase> {
    let db = GameDatabase::connect_dual(game_db_url, player_db_url).await?;
    db.create_tables().await?;
    db.seed_initial_data().await?;
    Ok(db)
}

/// Initialize the legacy single database with all required tables (backward compatibility)
pub async fn initialize_database(database_url: &str) -> DatabaseResult<LegacyGameDatabase> {
    let db = GameDatabase::connect(database_url).await?;
    db.create_tables().await?;
    db.seed_initial_data().await?;
    Ok(db)
}

/// Initialize the database router for intelligent query routing
pub async fn initialize_database_router(game_db_url: &str, player_db_url: Option<&str>) -> DatabaseResult<DatabaseRouter> {
    DatabaseRouter::new(game_db_url, player_db_url).await
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_legacy_database_initialization() {
        let db = initialize_database("sqlite::memory:").await.unwrap();
        assert!(db.is_connected().await);
    }

    #[tokio::test]
    async fn test_dual_database_initialization() {
        let db = initialize_dual_database("sqlite::memory:", Some("sqlite::memory:")).await.unwrap();
        assert!(db.is_connected().await);
    }

    #[tokio::test]
    async fn test_database_router_initialization() {
        let router = initialize_database_router("sqlite::memory:", Some("sqlite::memory:")).await.unwrap();
        
        // Test that router can classify operations correctly
        assert_eq!(router.classify_operation("hex_tiles", "query"), OperationType::WorldContent);
        assert_eq!(router.classify_operation("players", "update"), OperationType::PlayerState);
    }
}
