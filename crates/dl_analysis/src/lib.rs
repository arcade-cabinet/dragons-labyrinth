//! Analysis crate for Dragon's Labyrinth
//! 
//! This crate provides sophisticated HBF analysis matching the Python system:
//! - 70,801+ entity processing with intelligent clustering
//! - 3-phase AI pipeline (individual → dungeon containers → region containers)
//! - Spatial coordinate extraction (hex patterns like "W2S51")
//! - UUID relationship mapping with edge typing
//! - Template-based AI model generation
//! - Structured outputs with field specifications

pub mod base;
pub mod clusters;
pub mod containers;
pub mod dungeons;
pub mod entities;
pub mod factions;
pub mod orchestration;
pub mod raw;
pub mod regions;
pub mod results;
pub mod settlements;
pub mod seeds;
pub mod templates;
pub mod reporting;

// Re-export key types for external usage
pub use base::{HexKey, MapCoord, EdgeType, FieldSpec, EntitySpec, Inventory};
pub use orchestration::RawEntities;
pub use results::{GenerationResults, ModelConnections, AnalysisSummary};

use std::path::PathBuf;

/// Get the path to the generated analysis data
pub fn analysis_dir() -> PathBuf {
    PathBuf::from(env!("OUT_DIR"))
}

/// Get the path to the HTML fragments directory
pub fn html_dir() -> PathBuf {
    analysis_dir().join("html")
}

/// Get the path to the JSON analysis directory
pub fn json_dir() -> PathBuf {
    analysis_dir().join("json")
}

/// Get the path to the RON files directory
pub fn ron_dir() -> PathBuf {
    analysis_dir().join("ron")
}

/// Get the path to generated models directory
pub fn models_dir() -> PathBuf {
    analysis_dir().join("models")
}
