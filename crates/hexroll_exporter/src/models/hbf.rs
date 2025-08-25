//! HBF (HexRoll Binary Format) data structures
//!
//! These structures represent the parsed HBF file format used by HexRoll
//! to store campaign data including entities, references, and relationships.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Root HBF data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HbfData {
    /// All entities in the HBF file
    pub Entities: Vec<HbfEntity>,
    /// References linking entities together
    pub Refs: Option<Vec<HbfRef>>,
    /// Additional metadata
    #[serde(flatten)]
    pub metadata: HashMap<String, Value>,
}

/// A single entity in the HBF data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HbfEntity {
    /// Unique identifier for this entity
    pub uuid: String,
    /// When this entity was created
    pub created_at: Option<i64>,
    /// When this entity was last updated
    pub updated_at: Option<i64>,
    /// Entity content - can be any JSON structure
    #[serde(flatten)]
    pub content: HashMap<String, Value>,
}

/// A reference linking entities together
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HbfRef {
    /// Unique identifier for this reference
    pub uuid: String,
    /// The entity this reference points to
    pub entity_uuid: Option<String>,
    /// Alternative field names for entity reference
    #[serde(alias = "entityUuid", alias = "target_uuid")]
    pub target_uuid: Option<String>,
    /// Reference type or category
    #[serde(rename = "type")]
    pub ref_type: Option<String>,
    /// Additional reference data
    #[serde(flatten)]
    pub data: HashMap<String, Value>,
}

impl HbfEntity {
    /// Check if this entity is empty (only has metadata fields)
    pub fn is_empty(&self) -> bool {
        // An empty entity has only uuid, created_at, updated_at
        let essential_fields = ["uuid", "created_at", "updated_at"];
        
        // Count non-essential fields
        let content_fields: Vec<_> = self.content.keys()
            .filter(|k| !essential_fields.contains(&k.as_str()))
            .collect();
        
        content_fields.is_empty()
    }
    
    /// Get the entity's name if it has one
    pub fn name(&self) -> Option<&str> {
        self.content.get("name")
            .or_else(|| self.content.get("title"))
            .or_else(|| self.content.get("settlement_name"))
            .or_else(|| self.content.get("dungeon_name"))
            .and_then(|v| v.as_str())
    }
    
    /// Get the entity's content as a string if it has one
    pub fn content_text(&self) -> Option<&str> {
        self.content.get("content")
            .or_else(|| self.content.get("body"))
            .or_else(|| self.content.get("description"))
            .and_then(|v| v.as_str())
    }
    
    /// Check if this entity contains HTML content
    pub fn has_html_content(&self) -> bool {
        if let Some(content) = self.content_text() {
            return content.contains("<") && content.contains(">");
        }
        
        // Check specific HTML fields
        self.content.get("html").is_some() || 
        self.content.get("body").and_then(|v| v.as_str())
            .map(|s| s.contains("<") && s.contains(">"))
            .unwrap_or(false)
    }
    
    /// Check if this entity contains JSON data
    pub fn has_json_content(&self) -> bool {
        // Check for explicit JSON fields
        if self.content.contains_key("json_content") ||
           self.content.contains_key("hexes") ||
           self.content.contains_key("grid") ||
           self.content.contains_key("map") {
            return true;
        }
        
        // Check if content field contains JSON
        if let Some(content) = self.content_text() {
            let trimmed = content.trim();
            return trimmed.starts_with('{') || trimmed.starts_with('[');
        }
        
        false
    }
    
    /// Check if this entity represents a dungeon
    pub fn is_dungeon(&self) -> bool {
        // Check for dungeon-specific fields
        if self.content.contains_key("rooms") ||
           self.content.contains_key("dungeon_level") ||
           self.content.contains_key("dungeon_name") {
            return true;
        }
        
        // Check content for dungeon keywords
        if let Some(content) = self.content_text() {
            let content_lower = content.to_lowercase();
            return content_lower.contains("dungeon") ||
                   content_lower.contains("chamber") ||
                   content_lower.contains("corridor");
        }
        
        false
    }
}

impl HbfRef {
    /// Get the target entity UUID, checking multiple possible field names
    pub fn target_entity_uuid(&self) -> Option<&str> {
        self.entity_uuid.as_deref()
            .or_else(|| self.target_uuid.as_deref())
            .or_else(|| self.data.get("entity_uuid").and_then(|v| v.as_str()))
            .or_else(|| self.data.get("entityUuid").and_then(|v| v.as_str()))
            .or_else(|| self.data.get("target_uuid").and_then(|v| v.as_str()))
    }
    
    /// Get the reference type
    pub fn reference_type(&self) -> Option<&str> {
        self.ref_type.as_deref()
            .or_else(|| self.data.get("type").and_then(|v| v.as_str()))
    }
    
    /// Check if this is a location reference
    pub fn is_location_ref(&self) -> bool {
        if let Some(ref_type) = self.reference_type() {
            matches!(ref_type.to_lowercase().as_str(), 
                    "location" | "settlement" | "dungeon" | "poi")
        } else {
            // Check for location-specific fields
            self.data.contains_key("settlement_name") ||
            self.data.contains_key("dungeon_level")
        }
    }
    
    /// Check if this is a hex/map reference
    pub fn is_hex_ref(&self) -> bool {
        if let Some(ref_type) = self.reference_type() {
            matches!(ref_type.to_lowercase().as_str(), "hex" | "tile" | "map")
        } else {
            // Check for coordinate fields
            (self.data.contains_key("x") && self.data.contains_key("y")) ||
            self.data.contains_key("hex_x") || self.data.contains_key("hex_y")
        }
    }
    
    /// Check if this is a faction reference
    pub fn is_faction_ref(&self) -> bool {
        if let Some(ref_type) = self.reference_type() {
            matches!(ref_type.to_lowercase().as_str(), 
                    "faction" | "organization" | "group")
        } else {
            self.data.contains_key("faction_name") ||
            self.data.contains_key("alignment")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_empty_entity_detection() {
        let mut content = HashMap::new();
        
        let empty_entity = HbfEntity {
            uuid: "test-uuid".to_string(),
            created_at: Some(1234567890),
            updated_at: Some(1234567891),
            content: content.clone(),
        };
        assert!(empty_entity.is_empty());
        
        content.insert("name".to_string(), json!("Test Entity"));
        let non_empty_entity = HbfEntity {
            uuid: "test-uuid".to_string(),
            created_at: Some(1234567890),
            updated_at: Some(1234567891),
            content,
        };
        assert!(!non_empty_entity.is_empty());
    }

    #[test]
    fn test_html_content_detection() {
        let mut content = HashMap::new();
        content.insert("content".to_string(), json!("<h1>Test HTML</h1><p>Content</p>"));
        
        let entity = HbfEntity {
            uuid: "test-uuid".to_string(),
            created_at: None,
            updated_at: None,
            content,
        };
        
        assert!(entity.has_html_content());
    }

    #[test]
    fn test_json_content_detection() {
        let mut content = HashMap::new();
        content.insert("content".to_string(), json!(r#"{"hexes": [{"x": 0, "y": 0}]}"#));
        
        let entity = HbfEntity {
            uuid: "test-uuid".to_string(),
            created_at: None,
            updated_at: None,
            content,
        };
        
        assert!(entity.has_json_content());
    }

    #[test]
    fn test_reference_target_uuid() {
        let mut data = HashMap::new();
        data.insert("type".to_string(), json!("location"));
        
        let ref1 = HbfRef {
            uuid: "ref-1".to_string(),
            entity_uuid: Some("entity-1".to_string()),
            target_uuid: None,
            ref_type: None,
            data: data.clone(),
        };
        assert_eq!(ref1.target_entity_uuid(), Some("entity-1"));
        
        let ref2 = HbfRef {
            uuid: "ref-2".to_string(),
            entity_uuid: None,
            target_uuid: Some("entity-2".to_string()),
            ref_type: None,
            data,
        };
        assert_eq!(ref2.target_entity_uuid(), Some("entity-2"));
    }

    #[test]
    fn test_reference_type_detection() {
        let mut data = HashMap::new();
        data.insert("x".to_string(), json!(10));
        data.insert("y".to_string(), json!(20));
        
        let hex_ref = HbfRef {
            uuid: "hex-ref".to_string(),
            entity_uuid: Some("entity-1".to_string()),
            target_uuid: None,
            ref_type: None,
            data,
        };
        assert!(hex_ref.is_hex_ref());
        
        let mut data2 = HashMap::new();
        data2.insert("settlement_name".to_string(), json!("Test Village"));
        
        let location_ref = HbfRef {
            uuid: "loc-ref".to_string(),
            entity_uuid: Some("entity-2".to_string()),
            target_uuid: None,
            ref_type: None,
            data: data2,
        };
        assert!(location_ref.is_location_ref());
    }
}
