//! Region-specific entity models and processing.
//! 
//! This module contains specialized region processing logic including:
//! - RegionHexTile entity model matching Python regions.py
//! - RawRegionEntities cluster with specialized AI generation
//! - Region-specific inventory schemas, prompts, and templates
//! - Integration with the spatial container system

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use anyhow::Result;

use crate::clusters::{BaseEntitiesCluster, EntityCluster};
use crate::raw::{RawEntity, EntityCategory};
use crate::results::GenerationResults;
use crate::base::HexKey;
use crate::templates::TemplateManager;

/// Re-export the main entity from entities module
pub use crate::entities::RegionHexTile;

/// Specialized cluster for region entities with region-specific AI generation
#[derive(Debug, Clone)]
pub struct RawRegionEntities {
    base: BaseEntitiesCluster,
    template_manager: Option<TemplateManager>,
}

impl RawRegionEntities {
    pub fn new(cluster_name: String) -> Self {
        Self {
            base: BaseEntitiesCluster::new(EntityCategory::Regions, cluster_name),
            template_manager: TemplateManager::new().ok(),
        }
    }

    pub fn combined() -> Self {
        Self::new("combined".to_string())
    }

    /// Get specialized inventory schema for regions
    fn region_inventory_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "entities": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "properties": {
                            "name": {"type": "string", "description": "Entity class name like RegionHexTile"},
                            "description": {"type": "string", "description": "Description of the region entity"},
                            "fields": {
                                "type": "array",
                                "items": {
                                    "type": "object",
                                    "properties": {
                                        "name": {"type": "string", "description": "Field name"},
                                        "type": {"type": "string", "description": "Rust type like String, Option<HexKey>, Vec<String>"},
                                        "required": {"type": "boolean", "description": "Whether field is required"},
                                        "description": {"type": "string", "description": "Field description"},
                                        "is_uuid": {"type": "boolean", "description": "Whether field contains UUID references"},
                                        "is_connection": {"type": "boolean", "description": "Whether field represents entity connections"},
                                        "is_spatial": {"type": "boolean", "description": "Whether field contains hex coordinates"}
                                    },
                                    "required": ["name", "type", "required"],
                                    "additionalProperties": false
                                }
                            }
                        },
                        "required": ["name", "fields"],
                        "additionalProperties": false
                    }
                },
                "connections": {"type": "object", "additionalProperties": {"type": "string"}},
                "notes": {"type": "array", "items": {"type": "string"}}
            },
            "required": ["entities"],
            "additionalProperties": false
        })
    }

    /// Get specialized analysis prompt for regions
    fn region_analysis_prompt(&self) -> String {
        "Analyze the supplied HTML/JSON snippets related to *regions and hex tiles*.\n\
         Focus on hex-based world structure, spatial relationships, and regional features.\n\
         Look for:\n\
         - Hex coordinates in formats like 'W2S51', 'E3N12', etc.\n\
         - Map coordinates and spatial positioning data\n\
         - Settlement references and their locations within regions\n\
         - Dungeon entrances and their hex locations\n\
         - Faction presence and territorial control\n\
         - Biome types and terrain features\n\
         - Resource nodes and special features\n\
         - Regional boundaries and area definitions\n\
         - Connections between hexes and neighboring regions\n\
         Return a JSON object with an 'entities' array describing region data models.\n\
         Focus on spatial relationships - which entities exist at which hex coordinates.\n\
         Identify UUID references that connect regions to settlements, dungeons, and factions.\n\
         If uncertain about a field, omit rather than invent.".to_string()
    }

    /// Generate models using sophisticated template system
    fn generate_models_with_templates(
        &self,
        models_dir: &Path,
        logger: &mut dyn std::io::Write,
    ) -> Result<GenerationResults> {
        writeln!(logger, "Generating region models using template system...")?;

        let model_filename = format!("{}.rs", self.category().as_str());
        let model_path = models_dir.join(&model_filename);

        // Check if model already exists (idempotent)
        if model_path.exists() {
            writeln!(logger, "Region model already exists: {}", model_path.display())?;
            return Ok(GenerationResults::success(vec![model_path.to_string_lossy().to_string()])
                .add_note("Model already exists, skipped generation".to_string()));
        }

        // Generate inventory through AI analysis
        let inventory = self.base.analyze_entities(logger)?;

        // Use template manager if available
        let model_content = if let Some(ref template_manager) = self.template_manager {
            let mut metadata = HashMap::new();
            metadata.insert("category".to_string(), "regions".to_string());
            metadata.insert("module_type".to_string(), "regional_entities".to_string());
            
            template_manager.render_entity_template(&self.category(), &inventory, Some(&metadata))?
        } else {
            // Fallback to basic template
            self.base.render_model_template(&inventory)?
        };

        // Write the generated model to disk
        std::fs::create_dir_all(models_dir)?;
        std::fs::write(&model_path, model_content)?;

        // Extract connection information for container generation
        let connections = self.base.extract_connections_from_inventory(&inventory);

        writeln!(logger, "✓ Generated region model: {}", model_path.display())?;

        Ok(GenerationResults::success(vec![model_path.to_string_lossy().to_string()])
            .with_connections(connections)
            .add_note(format!("Generated from {} region entity specifications", inventory.entities.len())))
    }
}

impl EntityCluster for RawRegionEntities {
    fn category(&self) -> EntityCategory {
        EntityCategory::Regions
    }

    fn cluster_name(&self) -> &str {
        &self.base.cluster_name
    }

    fn add_entity(&mut self, entity: RawEntity) -> bool {
        self.base.add_entity(entity)
    }

    fn can_generate_models(&self) -> bool {
        self.base.can_generate_models()
    }

    fn write_entities_to_disk(&mut self, analysis_dir: &Path) -> Result<()> {
        self.base.write_entities_to_disk(analysis_dir)
    }

    fn inventory_schema(&self) -> serde_json::Value {
        self.region_inventory_schema()
    }

    fn analysis_prompt(&self) -> String {
        self.region_analysis_prompt()
    }

    fn model_template(&self) -> String {
        // This is overridden by the template manager, but provide fallback
        r#"//! Generated models for regions
//! 
//! This file was generated by the analysis system. Do not edit manually.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::base::HexKey;

{% for entity in inventory.entities %}
/// {{ entity.description or entity.name }}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct {{ entity.name }} {
{% for field in entity.fields %}
    /// {{ field.description or '' }}
    pub {{ field.name }}: {{ field.type }},
{% endfor %}
}

impl {{ entity.name }} {
    pub fn new(entity_uuid: String) -> Self {
        Self {
            entity_uuid,
{% for field in entity.fields %}
{% if field.name != "entity_uuid" %}
            {{ field.name }}: Default::default(),
{% endif %}
{% endfor %}
        }
    }

    /// Extract all referenced UUIDs from this region hex tile
    pub fn get_referenced_uuids(&self) -> Vec<String> {
        let mut uuids = Vec::new();
        // Implementation would be generated based on UUID fields
        uuids
    }
}

{% endfor %}
"#.to_string()
    }

    fn generate_models(
        &self,
        models_dir: &Path,
        logger: &mut dyn std::io::Write,
    ) -> Result<GenerationResults> {
        self.generate_models_with_templates(models_dir, logger)
    }
}

/// Region-specific utility functions
pub mod utils {
    use super::*;

    /// Extract hex coordinates from region entities
    pub fn extract_hex_coordinates(regions: &[RegionHexTile]) -> Vec<HexKey> {
        regions.iter()
            .filter_map(|region| region.hex_key.clone())
            .collect()
    }

    /// Group regions by biome type
    pub fn group_by_biome(regions: Vec<RegionHexTile>) -> HashMap<String, Vec<RegionHexTile>> {
        let mut biome_groups: HashMap<String, Vec<RegionHexTile>> = HashMap::new();
        
        for region in regions {
            let biome = region.biome_type.clone().unwrap_or_else(|| "unknown".to_string());
            biome_groups.entry(biome).or_insert_with(Vec::new).push(region);
        }
        
        biome_groups
    }

    /// Find regions that contain a specific entity type
    pub fn find_regions_with_entities<'a>(
        regions: &'a [RegionHexTile],
        entity_type: &str
    ) -> Vec<&'a RegionHexTile> {
        match entity_type {
            "settlements" => regions.iter()
                .filter(|region| !region.settlement_uuids.is_empty())
                .collect(),
            "dungeons" => regions.iter()
                .filter(|region| !region.dungeon_uuids.is_empty())
                .collect(),
            "factions" => regions.iter()
                .filter(|region| !region.faction_uuids.is_empty())
                .collect(),
            _ => Vec::new(),
        }
    }

    /// Calculate region density (entities per hex)
    pub fn calculate_region_density(regions: &[RegionHexTile]) -> f32 {
        if regions.is_empty() {
            return 0.0;
        }

        let total_entities: usize = regions.iter()
            .map(|region| region.entity_count())
            .sum();
            
        total_entities as f32 / regions.len() as f32
    }

    /// Find neighboring hexes (simplified - would need proper hex math)
    pub fn find_neighboring_hexes(hex_key: &HexKey) -> Vec<HexKey> {
        // This is a placeholder - would need proper hex coordinate parsing
        // and neighbor calculation using axial coordinates
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::raw::RawEntity;

    #[test]
    fn test_region_cluster() {
        let mut cluster = RawRegionEntities::new("Aurora Bushes".to_string());
        assert_eq!(cluster.category(), EntityCategory::Regions);
        assert_eq!(cluster.cluster_name(), "Aurora Bushes");
    }

    #[test]
    fn test_region_inventory_schema() {
        let cluster = RawRegionEntities::new("test".to_string());
        let schema = cluster.inventory_schema();
        assert!(schema.get("type").is_some());
        assert_eq!(schema["type"], "object");
    }

    #[test]
    fn test_region_analysis_prompt() {
        let cluster = RawRegionEntities::new("test".to_string());
        let prompt = cluster.analysis_prompt();
        assert!(prompt.contains("regions"));
        assert!(prompt.contains("hex"));
        assert!(prompt.contains("spatial"));
    }

    #[test]
    fn test_hex_coordinate_extraction() {
        let mut region1 = RegionHexTile::new("region1".to_string());
        region1.hex_key = Some("W2S51".to_string());
        
        let mut region2 = RegionHexTile::new("region2".to_string());
        region2.hex_key = Some("E3N12".to_string());
        
        let regions = vec![region1, region2];
        let hex_coords = utils::extract_hex_coordinates(&regions);
        
        assert_eq!(hex_coords.len(), 2);
        assert!(hex_coords.contains(&"W2S51".to_string()));
        assert!(hex_coords.contains(&"E3N12".to_string()));
    }

    #[test]
    fn test_biome_grouping() {
        let mut region1 = RegionHexTile::new("region1".to_string());
        region1.biome_type = Some("forest".to_string());
        
        let mut region2 = RegionHexTile::new("region2".to_string());
        region2.biome_type = Some("desert".to_string());
        
        let mut region3 = RegionHexTile::new("region3".to_string());
        region3.biome_type = Some("forest".to_string());
        
        let regions = vec![region1, region2, region3];
        let biome_groups = utils::group_by_biome(regions);
        
        assert_eq!(biome_groups.len(), 2);
        assert_eq!(biome_groups["forest"].len(), 2);
        assert_eq!(biome_groups["desert"].len(), 1);
    }

    #[test]
    fn test_region_density_calculation() {
        let mut region1 = RegionHexTile::new("region1".to_string());
        region1.settlement_uuids.push("settlement1".to_string());
        region1.dungeon_uuids.push("dungeon1".to_string());
        
        let mut region2 = RegionHexTile::new("region2".to_string());
        region2.faction_uuids.push("faction1".to_string());
        
        let regions = vec![region1, region2];
        let density = utils::calculate_region_density(&regions);
        
        assert_eq!(density, 1.5); // (2 + 1) entities / 2 regions = 1.5
    }
}
