//! Audit trait definition for Dragon's Labyrinth types
//!
//! Provides the trait interface for types to define their audit capabilities.
//! Actual audit report generation is handled by the dl_audit crate.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Core trait that all auditable types must implement
/// Similar to pandas DataFrame functionality where types know how to audit themselves
pub trait AuditableType {
    /// Get CSV headers for this type
    fn audit_headers() -> Vec<String>
    where 
        Self: Sized;
    
    /// Convert instance to audit row (CSV values)
    fn audit_row(&self) -> Vec<String>;
    
    /// Get audit category for organizing reports (e.g., "analytics", "world", "seeds")
    fn audit_category() -> String
    where 
        Self: Sized;
    
    /// Get audit subcategory for deeper organization (e.g., "entities", "dialogue", "hex_tiles")
    fn audit_subcategory() -> String
    where 
        Self: Sized;
    
    /// Optional: Custom field calculations (default: empty)
    fn custom_fields(&self) -> HashMap<String, String> {
        HashMap::new()
    }
    
    /// Optional: Extract numeric values for metrics calculation
    fn extract_numeric_fields(&self) -> HashMap<String, f64> {
        HashMap::new()
    }
}

/// Basic audit metadata structure (actual report generation in dl_audit)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditMetadata {
    pub category: String,
    pub subcategory: String,
    pub type_name: String,
}

impl AuditMetadata {
    pub fn new<T: AuditableType>() -> Self {
        Self {
            category: T::audit_category(),
            subcategory: T::audit_subcategory(),
            type_name: std::any::type_name::<T>().to_string(),
        }
    }
}
