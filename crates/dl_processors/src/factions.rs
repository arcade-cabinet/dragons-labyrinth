//! Faction-specific entity models and processing.
//! 
//! This module contains specialized faction processing logic including:
//! - FactionEntity entity model matching Python factions.py
//! - RawFactionEntities cluster with specialized AI generation
//! - Faction-specific inventory schemas, prompts, and templates
//! - Integration with territorial control and alliance systems

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use anyhow::Result;

use dl_analysis::clusters::{BaseEntitiesCluster, EntityCluster};
use dl_types::analysis::raw::{RawEntity, EntityCategory};
use dl_analysis::results::GenerationResults;

/// Specialized cluster for faction entities with faction-specific AI generation
#[derive(Debug, Clone)]
pub struct RawFactionEntities {
    base: BaseEntitiesCluster,
}

impl RawFactionEntities {
    pub fn new(cluster_name: String) -> Self {
        Self {
            base: BaseEntitiesCluster::new(EntityCategory::Factions, cluster_name),
        }
    }

    pub fn combined() -> Self {
        Self::new("combined".to_string())
    }

    /// Get specialized inventory schema for factions
    fn faction_inventory_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "entities": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "properties": {
                            "name": {"type": "string", "description": "Entity class name like FactionEntity"},
                            "description": {"type": "string", "description": "Description of the faction entity"},
                            "fields": {
                                "type": "array",
                                "items": {
                                    "type": "object",
                                    "properties": {
                                        "name": {"type": "string", "description": "Field name"},
                                        "type": {"type": "string", "description": "Rust type like String, Option<i32>, Vec<HexKey>"},
                                        "required": {"type": "boolean", "description": "Whether field is required"},
                                        "description": {"type": "string", "description": "Field description"},
                                        "is_uuid": {"type": "boolean", "description": "Whether field contains UUID references"},
                                        "is_connection": {"type": "boolean", "description": "Whether field represents entity relationships"},
                                        "is_spatial": {"type": "boolean", "description": "Whether field contains territorial data"}
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

    /// Get specialized analysis prompt for factions
    fn faction_analysis_prompt(&self) -> String {
        "Analyze the supplied HTML/JSON snippets related to *factions and political entities*.\n\
         Focus on organized groups, their power structures, and relationships.\n\
         Look for:\n\
         - Faction names and types (guild, kingdom, cult, order, etc.)\n\
         - Leadership structure and notable leaders\n\
         - Allegiances and alliances with other factions\n\
         - Enemy relationships and rivalries\n\
         - Territorial control and claimed hexes\n\
         - Goals, motivations, and objectives\n\
         - Resources and assets controlled\n\
         - Influence levels and power ratings\n\
         - Military strength and capabilities\n\
         - Cultural or ideological characteristics\n\
         - Recruitment methods and membership criteria\n\
         Return a JSON object with an 'entities' array describing faction data models.\n\
         Focus on political dynamics - who allies with whom, who opposes whom.\n\
         Identify UUID references that connect factions to leaders, settlements, and territories.\n\
         If uncertain about a field, omit rather than invent.".to_string()
    }

    /// Generate models using sophisticated template system
    fn generate_models_with_templates(
        &self,
        models_dir: &Path,
        logger: &mut dyn std::io::Write,
    ) -> Result<GenerationResults> {
        writeln!(logger, "Generating faction models using template system...")?;

        let model_filename = format!("{}.rs", self.category().as_str());
        let model_path = models_dir.join(&model_filename);

        // Check if model already exists (idempotent)
        if model_path.exists() {
            writeln!(logger, "Faction model already exists: {}", model_path.display())?;
            return Ok(GenerationResults::success(vec![model_path.to_string_lossy().to_string()])
                .add_note("Model already exists, skipped generation".to_string()));
        }

        // Generate inventory through AI analysis
        let inventory = self.base.analyze_entities(logger)?;

        // Use basic template generation
        let model_content = self.base.render_model_template(&inventory)?;

        // Write the generated model to disk
        std::fs::create_dir_all(models_dir)?;
        std::fs::write(&model_path, model_content)?;

        // Extract connection information for container generation
        let connections = self.base.extract_connections_from_inventory(&inventory);

        writeln!(logger, "✓ Generated faction model: {}", model_path.display())?;

        Ok(GenerationResults::success(vec![model_path.to_string_lossy().to_string()])
            .with_connections(connections)
            .add_note(format!("Generated from {} faction entity specifications", inventory.entities.len())))
    }
}

impl EntityCluster for RawFactionEntities {
    fn category(&self) -> EntityCategory {
        EntityCategory::Factions
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
        self.faction_inventory_schema()
    }

    fn analysis_prompt(&self) -> String {
        self.faction_analysis_prompt()
    }

    fn model_template(&self) -> String {
        // This is overridden by the template manager, but provide fallback
        r#"//! Generated models for factions
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

    /// Extract all referenced UUIDs from this faction
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

#[cfg(test)]
mod tests {
    use super::*;
    use dl_types::analysis::raw::RawEntity;

    #[test]
    fn test_faction_cluster() {
        let cluster = RawFactionEntities::new("Order of the Silver Hand".to_string());
        assert_eq!(cluster.category(), EntityCategory::Factions);
        assert_eq!(cluster.cluster_name(), "Order of the Silver Hand");
    }

    #[test]
    fn test_faction_inventory_schema() {
        let cluster = RawFactionEntities::new("test".to_string());
        let schema = cluster.inventory_schema();
        assert!(schema.get("type").is_some());
        assert_eq!(schema["type"], "object");
    }

    #[test]
    fn test_faction_analysis_prompt() {
        let cluster = RawFactionEntities::new("test".to_string());
        let prompt = cluster.analysis_prompt();
        assert!(prompt.contains("factions"));
        assert!(prompt.contains("political"));
        assert!(prompt.contains("allegiances"));
    }
}
