//! HBF Import Module - Convert HBF files to Dragon's Labyrinth database format
//!
//! This module handles the complete import pipeline from HBF SQLite files to 
//! properly structured SeaORM entities that integrate with Dragon's Labyrinth's
//! sophisticated horror progression and companion systems.

use anyhow::Result;
use database_orm::*;
use sea_orm::DatabaseConnection;

pub mod database;
pub mod parsers;
pub mod converters;
pub mod stats;

pub use stats::ImportStats;

/// Main entry point for HBF import process
pub async fn import_hbf_file(hbf_path: &str, db: &DatabaseConnection) -> Result<ImportStats> {
    let mut stats = ImportStats::default();
    
    // Load raw HBF data
    let hbf_snapshot = database::load_hbf_snapshot(hbf_path).await?;
    
    tracing::info!("Loaded {} hex tiles, {} entities, {} refs from HBF", 
                   hbf_snapshot.map_data.tiles.len(), 
                   hbf_snapshot.entities.len(), 
                   hbf_snapshot.refs.len());
    
    // Parse HTML entities into structured data using horror-aware parsers
    let parsed_entities = parsers::parse_all_entities(&hbf_snapshot.entities, &hbf_snapshot.refs).await?;
    
    // Convert parsed data to Dragon's Labyrinth entities with horror integration
    let dragon_entities = converters::convert_to_dragon_entities(&hbf_snapshot.map_data, &parsed_entities).await?;
    
    // Import into database with proper relationships
    stats = database::import_entities(db, dragon_entities).await?;
    
    tracing::info!("HBF import completed: {:?}", stats);
    Ok(stats)
}
