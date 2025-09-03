//! Unified type definitions for Dragon's Labyrinth
//!
//! This crate provides a single source of truth for all data structures
//! used across the Dragon's Labyrinth project, with built-in audit capabilities.

pub mod audit;
pub mod world;
pub mod analysis;
pub mod processing;
pub mod seeds;

// Re-export core audit functionality  
pub use audit::{AuditableType, AuditMetadata};

// Re-export all type modules
pub use world::*;
pub use analysis::*;
pub use processing::*;
pub use seeds::*;

/// Version information for type compatibility
pub const DL_TYPES_VERSION: &str = "0.1.0";

/// Common result type used throughout the crate
pub type DlTypesResult<T> = Result<T, anyhow::Error>;
