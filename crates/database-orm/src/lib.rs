//! Database ORM Models for Dragon's Labyrinth
//! 
//! This crate contains all SeaORM model definitions for the sophisticated systems.
//! Used by build scripts to create database schema and by runtime for queries.
//! 
//! Architecture:
//! - database-orm: Model definitions (this crate)
//! - game-database: Build script creates SQLite + runtime bevy_sqlx integration
//! - game-engine: Uses bevy_sqlx for ECS integration

use sea_orm::entity::prelude::*;
use sea_orm::DbBackend;

// Import all model modules
pub mod players;
pub mod companions;
pub mod forge;
pub mod psychology;
pub mod philosophy;
pub mod decay;
pub mod mounts;
pub mod assets;
pub mod items;
pub mod hex_tiles;
pub mod encounters;
pub mod dialogues;
pub mod player_statistics;
pub mod game_states;
pub mod ai_workflows;
pub mod generated_assets;
pub mod asset_dependencies;

// HBF integration modules
pub mod settlements;
pub mod dungeons;
pub mod npcs;

// Re-export entities for easy access
pub use players::Entity as Players;
pub use companions::Entity as Companions;
pub use forge::Entity as SentimentalItems;
pub use forge::forge_progress::Entity as ForgeProgress;
pub use psychology::Entity as CompanionTherapy;
pub use philosophy::Entity as PhilosophicalProgression;
pub use decay::Entity as WorldCorruption;
pub use mounts::Entity as MountCompanions;
pub use assets::Entity as Assets;
pub use assets::attribution::Entity as AssetAttribution;
pub use items::Entity as Items;
pub use hex_tiles::Entity as HexTiles;
pub use encounters::Entity as Encounters;
pub use dialogues::Entity as Dialogues;
pub use player_statistics::Entity as PlayerStatistics;
pub use game_states::Entity as GameStates;
pub use ai_workflows::Entity as AiWorkflows;
pub use generated_assets::Entity as GeneratedAssets;
pub use asset_dependencies::Entity as AssetDependencies;

// HBF integration entities
pub use settlements::Entity as Settlements;
pub use settlements::weather::Entity as SettlementWeather;
pub use dungeons::Entity as Dungeons;
pub use dungeons::rooms::Entity as DungeonRooms;
pub use dungeons::doorways::Entity as DungeonDoorways;
pub use npcs::Entity as Npcs;

// Re-export sea-orm for build scripts
pub use sea_orm;

/// Database version for migration tracking
pub const DATABASE_VERSION: &str = "1.0.0";

/// Create all database tables for sophisticated systems
pub async fn create_all_tables(db: &DatabaseConnection) -> Result<(), DbErr> {
    use sea_orm::Schema;
    
    // Create all sophisticated system tables
    let schema = Schema::new(DbBackend::Sqlite);
    
    // Core tables
    db.execute(db.get_database_backend().build(&schema.create_table_from_entity(Players))).await?;
    db.execute(db.get_database_backend().build(&schema.create_table_from_entity(Companions))).await?;
    
    // Asset management tables
    db.execute(db.get_database_backend().build(&schema.create_table_from_entity(Assets))).await?;
    db.execute(db.get_database_backend().build(&schema.create_table_from_entity(AssetAttribution))).await?;
    
    // Sophisticated system tables
    db.execute(db.get_database_backend().build(&schema.create_table_from_entity(SentimentalItems))).await?;
    db.execute(db.get_database_backend().build(&schema.create_table_from_entity(ForgeProgress))).await?;
    db.execute(db.get_database_backend().build(&schema.create_table_from_entity(CompanionTherapy))).await?;
    db.execute(db.get_database_backend().build(&schema.create_table_from_entity(PhilosophicalProgression))).await?;
    db.execute(db.get_database_backend().build(&schema.create_table_from_entity(WorldCorruption))).await?;
    db.execute(db.get_database_backend().build(&schema.create_table_from_entity(MountCompanions))).await?;
    
    // Game content tables
    db.execute(db.get_database_backend().build(&schema.create_table_from_entity(Items))).await?;
    db.execute(db.get_database_backend().build(&schema.create_table_from_entity(HexTiles))).await?;
    db.execute(db.get_database_backend().build(&schema.create_table_from_entity(Encounters))).await?;
    db.execute(db.get_database_backend().build(&schema.create_table_from_entity(Dialogues))).await?;
    db.execute(db.get_database_backend().build(&schema.create_table_from_entity(PlayerStatistics))).await?;
    db.execute(db.get_database_backend().build(&schema.create_table_from_entity(GameStates))).await?;
    
    // AI workflow and asset generation tables
    db.execute(db.get_database_backend().build(&schema.create_table_from_entity(AiWorkflows))).await?;
    db.execute(db.get_database_backend().build(&schema.create_table_from_entity(GeneratedAssets))).await?;
    db.execute(db.get_database_backend().build(&schema.create_table_from_entity(AssetDependencies))).await?;
    
    // HBF integration tables
    db.execute(db.get_database_backend().build(&schema.create_table_from_entity(Settlements))).await?;
    db.execute(db.get_database_backend().build(&schema.create_table_from_entity(SettlementWeather))).await?;
    db.execute(db.get_database_backend().build(&schema.create_table_from_entity(Dungeons))).await?;
    db.execute(db.get_database_backend().build(&schema.create_table_from_entity(DungeonRooms))).await?;
    db.execute(db.get_database_backend().build(&schema.create_table_from_entity(DungeonDoorways))).await?;
    db.execute(db.get_database_backend().build(&schema.create_table_from_entity(Npcs))).await?;
    
    tracing::info!("All sophisticated system database tables created successfully");
    Ok(())
}

/// Get all entity models for database operations
pub fn get_all_entities() -> Vec<&'static str> {
    vec![
        // Core tables
        "players",
        "companions",
        // Asset management
        "assets",
        "asset_attribution",
        // Sophisticated systems
        "sentimental_items",
        "forge_progress",
        "companion_therapy",
        "philosophical_progression",
        "world_corruption",
        "mount_companions",
        // Game content
        "items",
        "hex_tiles",
        "encounters",
        "dialogues",
        "player_statistics",
        "game_states",
        // AI workflows and generation
        "ai_workflows",
        "generated_assets",
        "asset_dependencies",
        // HBF integration
        "settlements",
        "settlement_weather",
        "dungeons",
        "dungeon_rooms",
        "dungeon_doorways",
        "npcs",
    ]
}
