//! Data models for HBF processing
//! 
//! These models represent the structure of HBF (HexRoll Binary Format) data
//! used throughout the transformation pipeline.

pub mod hbf;

pub use hbf::{HbfData, HbfEntity, HbfRef};
