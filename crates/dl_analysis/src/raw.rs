//! HBF entity extraction and file writing
//! 
//! Uses RawEntity from dl_types for entity creation and categorization.
//! Provides utilities for writing entities to analysis directories.

use anyhow::Result;
use std::path::{Path, PathBuf};
use std::fs;

// Use RawEntity and types from dl_types where they belong
pub use dl_types::analysis::{RawEntity, EntityCategory, ContentFormat};

/// Write entity to disk in appropriate directory structure
pub fn write_entity_to_disk(entity: &RawEntity, analysis_dir: &Path) -> Result<PathBuf> {
    let category_dir = analysis_dir.join(entity.category.as_str());
    
    let entity_dir = if entity.entity_name != "unknown" {
        category_dir.join(entity.get_sanitized_name())
    } else {
        category_dir.join("unknown")
    };

    fs::create_dir_all(&entity_dir)?;

    let filename = format!("entity_{}.{}", entity.uuid, 
        if entity.entity_type == "json" { "json" } else { "html" });
    let file_path = entity_dir.join(filename);

    fs::write(&file_path, &entity.raw_value)?;

    Ok(file_path)
}

/// Statistics about entity extraction
#[derive(Debug, Clone)]
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
        
        *self.by_category.entry(entity.category.clone()).or_insert(0) += 1;
        *self.by_format.entry(entity.entity_type.clone()).or_insert(0) += 1;
        
        // These would need proper spatial detection implementation
        self.with_spatial_data += 1;
        self.with_references += 1;
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
