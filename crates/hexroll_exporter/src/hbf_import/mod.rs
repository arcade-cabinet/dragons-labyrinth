//! HBF Import Module - Convert HBF files to Dragon's Labyrinth database format
//!
//! This module handles the complete import pipeline from HBF SQLite files to 
//! properly structured SeaORM entities that integrate with Dragon's Labyrinth's
//! sophisticated horror progression and companion systems.

use anyhow::Result;

pub mod database;
pub mod parsers;
pub mod converters;
pub mod stats;
pub mod analyzer;
pub mod inspector;

pub use stats::ImportStats;
pub use analyzer::{HbfAnalyzer, AnalysisReport};
pub use inspector::SqliteInspector;

// Analysis-focused module - no import functionality yet
// Main goal: understand HBF structure to generate proper ORM models
