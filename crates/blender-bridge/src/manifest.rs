//! Manifest management for tracking conversion state
//! 
//! Maintains a JSON manifest of converted files to enable idempotent conversions

use std::collections::HashMap;
use std::path::Path;
use std::fs;
use serde::{Deserialize, Serialize};
use crate::error::BlenderBridgeError;

/// A manifest entry recording previous conversion results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestEntry {
    /// Content hash of the source file(s) when converted
    pub hash: String,
    /// Absolute path to the destination GLB file
    pub dst: String,
    /// Timestamp when conversion was performed
    pub converted_at: u64,
}

/// Manifest managing conversion tracking
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ConversionManifest {
    /// Map from source file path to conversion entry
    entries: HashMap<String, ManifestEntry>,
}

impl ConversionManifest {
    /// Load manifest from disk, creating empty manifest if file doesn't exist
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, BlenderBridgeError> {
        let path = path.as_ref();
        
        if !path.exists() {
            return Ok(Self::default());
        }
        
        let content = fs::read_to_string(path)?;
        let manifest: Self = serde_json::from_str(&content)
            .map_err(|e| BlenderBridgeError::ManifestError(format!("Failed to parse manifest: {}", e)))?;
        
        Ok(manifest)
    }
    
    /// Save manifest to disk
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), BlenderBridgeError> {
        let path = path.as_ref();
        
        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)?;
        
        Ok(())
    }
    
    /// Check if a source file needs conversion based on its current hash
    pub fn needs_conversion(&self, src_path: &str, current_hash: &str) -> bool {
        match self.entries.get(src_path) {
            Some(entry) => {
                // Need conversion if hash changed OR destination doesn't exist
                entry.hash != current_hash || !Path::new(&entry.dst).exists()
            }
            None => true, // No entry means never converted
        }
    }
    
    /// Record a successful conversion
    pub fn record_conversion(&mut self, src_path: String, hash: String, dst_path: String) {
        let entry = ManifestEntry {
            hash,
            dst: dst_path,
            converted_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        };
        
        self.entries.insert(src_path, entry);
    }
    
    /// Get destination path for a previously converted file
    pub fn get_destination(&self, src_path: &str) -> Option<&str> {
        self.entries.get(src_path).map(|entry| entry.dst.as_str())
    }
    
    /// Get all entries for inspection
    pub fn entries(&self) -> &HashMap<String, ManifestEntry> {
        &self.entries
    }
    
    /// Clean up entries for files that no longer exist
    pub fn cleanup_stale_entries(&mut self) {
        self.entries.retain(|src_path, entry| {
            let src_exists = Path::new(src_path).exists();
            let dst_exists = Path::new(&entry.dst).exists();
            
            if !src_exists {
                tracing::debug!("Removing stale manifest entry for deleted source: {}", src_path);
                false
            } else if !dst_exists {
                tracing::debug!("Removing manifest entry for missing destination: {}", entry.dst);
                false
            } else {
                true
            }
        });
    }
}
