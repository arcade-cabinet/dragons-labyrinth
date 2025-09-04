//! Analysis crate for Dragon's Labyrinth
//! 
//! This crate provides sophisticated HBF analysis matching the Python system:
//! - SQLite entity extraction from HBF database
//! - Entity categorization and clustering  
//! - AI-powered analysis using OpenAI structured outputs
//! - Spatial coordinate extraction (hex patterns like "W2S51")
//! - UUID relationship mapping
//! - Analysis reporting and statistics

pub mod audit_types;
pub mod audit;
pub mod clusters;
pub mod orchestration;
pub mod raw;
pub mod reporting;
pub mod results;
pub mod seeds;

// Re-export key types for external usage  
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
