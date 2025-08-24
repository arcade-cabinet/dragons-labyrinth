//! Game Database - Comprehensive SeaORM models for Dragon's Labyrinth
//! 
//! This crate provides a complete database abstraction layer for the horror-first RPG,
//! integrating seamlessly with Bevy ECS and supporting the revolutionary AI asset generation
//! pipeline while maintaining clean separation between build-time and runtime systems.

// Models are imported from database-orm crate
pub mod engine;
pub mod traits;
pub mod bevy_integration;
pub mod error;
pub mod tools;
pub mod save_system;

pub use engine::{DatabaseConnection, GameDatabase};
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

/// Initialize the game database with all required tables
pub async fn initialize_database(database_url: &str) -> DatabaseResult<GameDatabase> {
    let db = GameDatabase::connect(database_url).await?;
    db.create_tables().await?;
    db.seed_initial_data().await?;
    Ok(db)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_database_initialization() {
        let db = initialize_database("sqlite::memory:").await.unwrap();
        assert!(db.is_connected().await);
    }
}
