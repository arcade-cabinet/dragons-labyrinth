//! Dragon's Labyrinth Audit System
//!
//! Standalone crate for generating audit reports from any pipeline stage using Polars lazy API.
//! Implements rotational archiving to prevent report overwrites.

pub mod archive;
pub mod dataframe;
pub mod reports;
pub mod system;

// Re-export main functionality
pub use system::AuditSystem;
pub use reports::{AuditReportMetadata, ReportConfig};
pub use archive::ArchiveManager;
pub use dataframe::DataFrameBuilder;

/// Version information
pub const DL_AUDIT_VERSION: &str = "0.1.0";

/// Common result type
pub type AuditResult<T> = Result<T, anyhow::Error>;
