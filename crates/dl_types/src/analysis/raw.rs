//! Raw entity types for HBF analysis
//!
//! Contains RawEntity and related types that represent the RAW state
//! before processing into final ECS entities

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use crate::analysis::base::{KNOWN_REGIONS, KNOWN_SETTLEMENTS, KNOWN_FACTIONS, KNOWN_DUNGEONS};

/// Raw entity extracted from HBF database with clustering logic
/// Matches Python RawEntity exactly
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawEntity {
    pub uuid: String,
    pub raw_value: String,
    pub entity_type: String,  // "json" or "html"
    pub data: serde_json::Value,
    pub category: String,
    pub entity_name: String,
    pub file_path: Option<PathBuf>,
}

impl RawEntity {
    /// Factory method to create RawEntity with computed fields (matches Python create())
    pub fn create(uuid: String, raw_value: String) -> Self {
        // Parse JSON/HTML content
        let (entity_type, data) = Self::parse_content(&raw_value);
        
        // Determine category and entity name using KNOWN_* constants
        let (category, entity_name) = Self::determine_clustering(&raw_value);
        
        Self {
            uuid,
            raw_value,
            entity_type,
            data,
            category,
            entity_name,
            file_path: None,
        }
    }

    /// Parse raw value into entity type and structured data (matches Python _parse_content)
    fn parse_content(raw_value: &str) -> (String, serde_json::Value) {
        let trimmed = raw_value.trim();
        if trimmed.starts_with('{') {
            match serde_json::from_str(raw_value) {
                Ok(json_data) => ("json".to_string(), json_data),
                Err(_) => ("html".to_string(), serde_json::json!({"content": raw_value})),
            }
        } else {
            ("html".to_string(), serde_json::json!({"content": raw_value}))
        }
    }

    /// Determine which category and entity this belongs to (matches Python _determine_clustering)
    fn determine_clustering(raw_value: &str) -> (String, String) {
        let content_lower = raw_value.to_lowercase();
        
        // Check regions
        for region in KNOWN_REGIONS {
            if content_lower.contains(&region.to_lowercase()) {
                return ("regions".to_string(), region.to_string());
            }
        }
        
        // Check settlements
        for settlement in KNOWN_SETTLEMENTS {
            if content_lower.contains(&settlement.to_lowercase()) {
                return ("settlements".to_string(), settlement.to_string());
            }
        }
        
        // Check factions
        for faction in KNOWN_FACTIONS {
            if content_lower.contains(&faction.to_lowercase()) {
                return ("factions".to_string(), faction.to_string());
            }
        }
        
        // Check dungeons
        for dungeon in KNOWN_DUNGEONS {
            if content_lower.contains(&dungeon.to_lowercase()) {
                return ("dungeons".to_string(), dungeon.to_string());
            }
        }
        
        // Uncategorized
        ("uncategorized".to_string(), "unknown".to_string())
    }

    /// Get sanitized name for directory creation (matches Python get_sanitized_name)
    pub fn get_sanitized_name(&self) -> String {
        if self.entity_name == "unknown" {
            return "unknown".to_string();
        }
        self.entity_name
            .to_lowercase()
            .replace(' ', "_")
            .replace('\'', "")
            .replace('-', "_")
            .replace('.', "")
    }
}

/// Category of entity based on content analysis
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EntityCategory {
    #[serde(rename = "regions")]
    Regions,
    #[serde(rename = "settlements")]
    Settlements,
    #[serde(rename = "factions")]
    Factions,
    #[serde(rename = "dungeons")]
    Dungeons,
    #[serde(rename = "json")]
    Json,
    #[serde(rename = "uncategorized")]
    Uncategorized,
}

impl EntityCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            EntityCategory::Regions => "regions",
            EntityCategory::Settlements => "settlements", 
            EntityCategory::Factions => "factions",
            EntityCategory::Dungeons => "dungeons",
            EntityCategory::Json => "json",
            EntityCategory::Uncategorized => "uncategorized",
        }
    }
}

/// Format of entity content
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ContentFormat {
    Html,
    Json,
}

impl ContentFormat {
    pub fn file_extension(&self) -> &'static str {
        match self {
            ContentFormat::Html => "html",
            ContentFormat::Json => "json",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_creation_html() {
        let content = r#"<div>Content about Aurora Bushes region with data</div>"#;
        let entity = RawEntity::create("test_uuid".to_string(), content.to_string());
        
        assert_eq!(entity.category, "regions");
        assert_eq!(entity.entity_name, "Aurora Bushes");
        assert_eq!(entity.entity_type, "html");
    }

    #[test]
    fn test_entity_creation_json() {
        let content = r#"{"type": "settlement", "name": "test"}"#;
        let entity = RawEntity::create("test_uuid".to_string(), content.to_string());
        
        assert_eq!(entity.entity_type, "json");
    }

    #[test]
    fn test_categorization() {
        let region_content = "Content about Aurora Bushes";
        let (category, entity_name) = RawEntity::determine_clustering(region_content);
        assert_eq!(category, "regions");
        assert_eq!(entity_name, "Aurora Bushes");
        
        let settlement_content = "Content about Village of Harad";
        let (category, entity_name) = RawEntity::determine_clustering(settlement_content);
        assert_eq!(category, "settlements");
        assert_eq!(entity_name, "Village of Harad");
        
        let unknown_content = "Some random content";
        let (category, entity_name) = RawEntity::determine_clustering(unknown_content);
        assert_eq!(category, "uncategorized");
        assert_eq!(entity_name, "unknown");
    }

    #[test]
    fn test_sanitized_names() {
        let entity = RawEntity {
            uuid: "test".to_string(),
            raw_value: "".to_string(),
            entity_type: "html".to_string(),
            data: serde_json::Value::Null,
            category: "regions".to_string(),
            entity_name: "Aurora Bushes".to_string(),
            file_path: None,
        };
        
        assert_eq!(entity.get_sanitized_name(), "aurora_bushes");
    }
}
