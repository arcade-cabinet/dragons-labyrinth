//! Raw entity model with automatic clustering and file writing capability.
//! 
//! Mirrors the Python raw.py with intelligent entity categorization.

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::fs;
use crate::base::{
    KNOWN_REGIONS, KNOWN_SETTLEMENTS, KNOWN_FACTIONS, KNOWN_DUNGEONS,
    hex_utils, uuid_utils, HexKey, MapCoord
};
use anyhow::Result;

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
    #[serde(rename = "unknown")]
    Unknown,
}

impl EntityCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            EntityCategory::Regions => "regions",
            EntityCategory::Settlements => "settlements",
            EntityCategory::Factions => "factions",
            EntityCategory::Dungeons => "dungeons",
            EntityCategory::Json => "json",
            EntityCategory::Unknown => "unknown",
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

/// Individual HBF entity with automatic clustering and spatial extraction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawEntity {
    pub uuid: String,
    pub value: String,
    pub category: EntityCategory,
    pub entity_name: Option<String>,
    pub format: ContentFormat,
    pub hex_coordinate: Option<HexKey>,
    pub map_coordinate: Option<MapCoord>,
    pub referenced_uuids: Vec<String>,
}

impl RawEntity {
    /// Factory method to create entity with automatic categorization
    pub fn create(uuid: String, value: String) -> Self {
        let format = Self::detect_format(&value);
        let category = Self::categorize_entity(&value);
        let entity_name = Self::extract_entity_name(&value, &category);
        let hex_coordinate = hex_utils::extract_hex_coordinate(&value);
        let map_coordinate = Self::extract_map_coordinate(&value);
        let referenced_uuids = uuid_utils::extract_uuids(&value);

        Self {
            uuid,
            value,
            category,
            entity_name,
            format,
            hex_coordinate,
            map_coordinate,
            referenced_uuids,
        }
    }

    /// Detect content format (HTML vs JSON)
    fn detect_format(content: &str) -> ContentFormat {
        let trimmed = content.trim();
        if (trimmed.starts_with('{') && trimmed.ends_with('}')) || 
           (trimmed.starts_with('[') && trimmed.ends_with(']')) {
            if serde_json::from_str::<serde_json::Value>(content).is_ok() {
                return ContentFormat::Json;
            }
        }
        ContentFormat::Html
    }

    /// Categorize entity based on content matching
    fn categorize_entity(content: &str) -> EntityCategory {
        // Check for JSON format first
        if Self::detect_format(content) == ContentFormat::Json {
            return EntityCategory::Json;
        }

        // Check for known entity names in content
        for region in KNOWN_REGIONS {
            if content.contains(region) {
                return EntityCategory::Regions;
            }
        }

        for settlement in KNOWN_SETTLEMENTS {
            if content.contains(settlement) {
                return EntityCategory::Settlements;
            }
        }

        for faction in KNOWN_FACTIONS {
            if content.contains(faction) {
                return EntityCategory::Factions;
            }
        }

        for dungeon in KNOWN_DUNGEONS {
            if content.contains(dungeon) {
                return EntityCategory::Dungeons;
            }
        }

        EntityCategory::Unknown
    }

    /// Extract entity name based on category and content
    fn extract_entity_name(content: &str, category: &EntityCategory) -> Option<String> {
        match category {
            EntityCategory::Regions => {
                for region in KNOWN_REGIONS {
                    if content.contains(region) {
                        return Some(region.to_string());
                    }
                }
            }
            EntityCategory::Settlements => {
                for settlement in KNOWN_SETTLEMENTS {
                    if content.contains(settlement) {
                        return Some(settlement.to_string());
                    }
                }
            }
            EntityCategory::Factions => {
                for faction in KNOWN_FACTIONS {
                    if content.contains(faction) {
                        return Some(faction.to_string());
                    }
                }
            }
            EntityCategory::Dungeons => {
                for dungeon in KNOWN_DUNGEONS {
                    if content.contains(dungeon) {
                        return Some(dungeon.to_string());
                    }
                }
            }
            _ => {}
        }
        None
    }

    /// Extract map coordinates from content (placeholder implementation)
    fn extract_map_coordinate(content: &str) -> Option<MapCoord> {
        // This would need proper implementation based on HBF coordinate patterns
        // Looking for patterns like map-coords elements with x,y,hex_id
        // For now, return None as placeholder
        if content.contains("map-coords") {
            Some(MapCoord::new(None, None, None))
        } else {
            None
        }
    }

    /// Write entity to disk in appropriate directory structure
    pub fn write_to_disk(&self, analysis_dir: &Path) -> Result<PathBuf> {
        let category_dir = analysis_dir.join(self.category.as_str());
        
        let entity_dir = if let Some(ref name) = self.entity_name {
            category_dir.join(Self::slugify(name))
        } else {
            category_dir.join("unknown")
        };

        fs::create_dir_all(&entity_dir)?;

        let filename = format!("entity_{}.{}", self.uuid, self.format.file_extension());
        let file_path = entity_dir.join(filename);

        fs::write(&file_path, &self.value)?;

        Ok(file_path)
    }

    /// Convert entity name to filesystem-safe slug
    fn slugify(name: &str) -> String {
        name.to_lowercase()
            .replace(' ', "_")
            .replace("'", "")
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '_')
            .collect()
    }

    /// Get file extension for this entity
    pub fn file_extension(&self) -> &'static str {
        self.format.file_extension()
    }

    /// Check if entity belongs to a specific category
    pub fn belongs_to_category(&self, category: EntityCategory) -> bool {
        self.category == category
    }

    /// Check if entity matches a specific entity name
    pub fn matches_entity_name(&self, name: &str) -> bool {
        self.entity_name.as_ref().map_or(false, |n| n == name)
    }

    /// Get all spatial information for this entity
    pub fn spatial_info(&self) -> SpatialInfo {
        SpatialInfo {
            hex_coordinate: self.hex_coordinate.clone(),
            map_coordinate: self.map_coordinate.clone(),
            referenced_uuids: self.referenced_uuids.clone(),
        }
    }
}

/// Spatial information extracted from entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpatialInfo {
    pub hex_coordinate: Option<HexKey>,
    pub map_coordinate: Option<MapCoord>,
    pub referenced_uuids: Vec<String>,
}

impl SpatialInfo {
    pub fn has_spatial_data(&self) -> bool {
        self.hex_coordinate.is_some() || self.map_coordinate.is_some()
    }

    pub fn has_references(&self) -> bool {
        !self.referenced_uuids.is_empty()
    }
}

/// Statistics about entity extraction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityStats {
    pub total_entities: usize,
    pub by_category: std::collections::HashMap<String, usize>,
    pub by_format: std::collections::HashMap<String, usize>,
    pub with_spatial_data: usize,
    pub with_references: usize,
}

impl EntityStats {
    pub fn new() -> Self {
        Self {
            total_entities: 0,
            by_category: std::collections::HashMap::new(),
            by_format: std::collections::HashMap::new(),
            with_spatial_data: 0,
            with_references: 0,
        }
    }

    pub fn add_entity(&mut self, entity: &RawEntity) {
        self.total_entities += 1;
        
        *self.by_category.entry(entity.category.as_str().to_string()).or_insert(0) += 1;
        *self.by_format.entry(entity.format.file_extension().to_string()).or_insert(0) += 1;
        
        if entity.spatial_info().has_spatial_data() {
            self.with_spatial_data += 1;
        }
        
        if entity.spatial_info().has_references() {
            self.with_references += 1;
        }
    }

    pub fn summary(&self) -> String {
        let mut lines = vec![
            format!("Total entities: {}", self.total_entities),
            "".to_string(),
            "By category:".to_string(),
        ];

        for (category, count) in &self.by_category {
            lines.push(format!("  {}: {}", category, count));
        }

        lines.push("".to_string());
        lines.push("By format:".to_string());
        for (format, count) in &self.by_format {
            lines.push(format!("  {}: {}", format, count));
        }

        lines.push("".to_string());
        lines.push(format!("With spatial data: {}", self.with_spatial_data));
        lines.push(format!("With references: {}", self.with_references));

        lines.join("\n")
    }
}

impl Default for EntityStats {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_creation_html() {
        let content = r#"<div>Content about Aurora Bushes region with Hex W2S51</div>"#;
        let entity = RawEntity::create("test_uuid".to_string(), content.to_string());
        
        assert_eq!(entity.category, EntityCategory::Regions);
        assert_eq!(entity.entity_name, Some("Aurora Bushes".to_string()));
        assert_eq!(entity.format, ContentFormat::Html);
        assert_eq!(entity.hex_coordinate, Some("W2S51".to_string()));
    }

    #[test]
    fn test_entity_creation_json() {
        let content = r#"{"type": "settlement", "name": "test"}"#;
        let entity = RawEntity::create("test_uuid".to_string(), content.to_string());
        
        assert_eq!(entity.category, EntityCategory::Json);
        assert_eq!(entity.format, ContentFormat::Json);
    }

    #[test]
    fn test_slugify() {
        assert_eq!(RawEntity::slugify("Aurora Bushes"), "aurora_bushes");
        assert_eq!(RawEntity::slugify("Village of Harad"), "village_of_harad");
        assert_eq!(RawEntity::slugify("The Defiled Wolves"), "the_defiled_wolves");
    }

    #[test]
    fn test_entity_categorization() {
        let region_content = "Content about Aurora Bushes";
        assert_eq!(RawEntity::categorize_entity(region_content), EntityCategory::Regions);
        
        let settlement_content = "Content about Village of Harad";
        assert_eq!(RawEntity::categorize_entity(settlement_content), EntityCategory::Settlements);
        
        let json_content = r#"{"test": true}"#;
        assert_eq!(RawEntity::categorize_entity(json_content), EntityCategory::Json);
        
        let unknown_content = "Some random content";
        assert_eq!(RawEntity::categorize_entity(unknown_content), EntityCategory::Unknown);
    }

    #[test]
    fn test_spatial_info() {
        let content = r#"<div>Content about Aurora Bushes region with Hex W2S51</div>"#;
        let entity = RawEntity::create("test_uuid".to_string(), content.to_string());
        let spatial = entity.spatial_info();
        
        assert!(spatial.has_spatial_data());
        assert_eq!(spatial.hex_coordinate, Some("W2S51".to_string()));
    }

    #[test]
    fn test_entity_stats() {
        let mut stats = EntityStats::new();
        let entity = RawEntity::create("test".to_string(), "Aurora Bushes Hex W2S51".to_string());
        stats.add_entity(&entity);
        
        assert_eq!(stats.total_entities, 1);
        assert_eq!(stats.with_spatial_data, 1);
        assert_eq!(stats.by_category.get("regions"), Some(&1));
    }
}
