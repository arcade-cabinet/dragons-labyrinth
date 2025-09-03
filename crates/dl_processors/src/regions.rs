//! Region-specific entity models and processing.
//! 
//! This module contains specialized region processing logic including:
//! - Region entity model matching Python regions.py
//! - RawRegionEntities cluster with specialized AI generation
//! - Region-specific inventory schemas, prompts, and templates

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use anyhow::Result;

use dl_analysis::clusters::{BaseEntitiesCluster, EntityCluster};
use dl_types::analysis::raw::{RawEntity, EntityCategory};
use dl_analysis::results::GenerationResults;

/// Specialized cluster for region entities with region-specific AI generation
#[derive(Debug, Clone)]
pub struct RawRegionEntities {
    base: BaseEntitiesCluster,
}

impl RawRegionEntities {
    pub fn new(cluster_name: String) -> Self {
        Self {
            base: BaseEntitiesCluster::new(EntityCategory::Regions, cluster_name),
        }
    }

    pub fn combined() -> Self {
        Self::new("combined".to_string())
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
        serde_json::json!({
            "type": "object",
            "properties": {
                "entities": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "properties": {
                            "name": {"type": "string"},
                            "description": {"type": "string"},
                            "fields": {
                                "type": "array",
                                "items": {
                                    "type": "object",
                                    "properties": {
                                        "name": {"type": "string"},
                                        "type": {"type": "string"},
                                        "required": {"type": "boolean"},
                                        "description": {"type": "string"},
                                        "is_uuid": {"type": "boolean"},
                                        "is_connection": {"type": "boolean"}
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

    fn analysis_prompt(&self) -> String {
        "Analyze the supplied HTML/JSON snippets related to *regions*.\n\
         Return a JSON object with an 'entities' array describing data models.\n\
         Focus on names, descriptions, field names/types, and which fields are UUIDs or connections.\n\
         Look for hex coordinates, map positions, settlements, dungeons, and faction references.\n\
         If uncertain, omit rather than invent.".to_string()
    }

    fn model_template(&self) -> String {
        "regions_analysis.j2".to_string()
    }

    fn generate_models(
        &self,
        models_dir: &Path,
        logger: &mut dyn std::io::Write,
    ) -> Result<GenerationResults> {
        let inventory = self.base.analyze_entities(logger)?;
        self.base.generate_code_from_inventory(&inventory, models_dir, logger)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dl_types::analysis::raw::RawEntity;

    #[test]
    fn test_region_cluster() {
        let cluster = RawRegionEntities::new("Aurora Bushes".to_string());
        assert_eq!(cluster.category(), EntityCategory::Regions);
        assert_eq!(cluster.cluster_name(), "Aurora Bushes");
    }
}
