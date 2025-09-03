//! Dungeon-specific entity models and processing.
//!
//! This module contains specialized dungeon processing logic including:
//! - DungeonArea entity model matching Python dungeons.py
//! - RawDungeonEntities cluster with specialized AI generation
//! - Dungeon-specific inventory schemas, prompts, and templates

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use anyhow::Result;

use dl_analysis::clusters::{BaseEntitiesCluster, EntityCluster};
use dl_types::analysis::raw::{RawEntity, EntityCategory};
use dl_analysis::results::GenerationResults;

/// Dungeon area entity matching Python DungeonArea model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DungeonArea {
    pub entity_uuid: String,
    pub dungeon_name: Option<String>,
    pub area_number: Option<i32>,
    pub area_description: Option<String>,
    pub entrance_hex: Option<String>,
    pub connected_areas: Vec<i32>,
    pub monsters: Vec<serde_json::Value>,
    pub treasure: Option<serde_json::Value>,
    pub traps: Vec<serde_json::Value>,
    pub special_features: Vec<String>,
    pub difficulty_level: Option<i32>,
    pub map_coordinates: Option<HashMap<String, f32>>,
}

impl DungeonArea {
    pub fn new(entity_uuid: String) -> Self {
        Self {
            entity_uuid,
            dungeon_name: None,
            area_number: None,
            area_description: None,
            entrance_hex: None,
            connected_areas: Vec::new(),
            monsters: Vec::new(),
            treasure: None,
            traps: Vec::new(),
            special_features: Vec::new(),
            difficulty_level: None,
            map_coordinates: None,
        }
    }

    /// Extract UUIDs from monsters, treasures, and traps
    pub fn extract_referenced_uuids(&self) -> Vec<String> {
        let mut uuids = Vec::new();
        
        // Extract UUIDs from monsters
        for monster in &self.monsters {
            if let Some(uuid) = monster.get("uuid").and_then(|v| v.as_str()) {
                uuids.push(uuid.to_string());
            }
        }

        // Extract UUIDs from treasure
        if let Some(treasure) = &self.treasure {
            if let Some(uuid) = treasure.get("uuid").and_then(|v| v.as_str()) {
                uuids.push(uuid.to_string());
            }
        }

        // Extract UUIDs from traps
        for trap in &self.traps {
            if let Some(uuid) = trap.get("uuid").and_then(|v| v.as_str()) {
                uuids.push(uuid.to_string());
            }
        }

        uuids
    }
}

/// Specialized cluster for dungeon entities with dungeon-specific AI generation
#[derive(Debug, Clone)]
pub struct RawDungeonEntities {
    base: BaseEntitiesCluster,
}

impl RawDungeonEntities {
    pub fn new(cluster_name: String) -> Self {
        Self {
            base: BaseEntitiesCluster::new(EntityCategory::Dungeons, cluster_name),
        }
    }

    pub fn combined() -> Self {
        Self::new("combined".to_string())
    }
}

impl EntityCluster for RawDungeonEntities {
    fn category(&self) -> EntityCategory {
        EntityCategory::Dungeons
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
                            "name": {"type": "string", "description": "Entity class name like DungeonArea"},
                            "description": {"type": "string", "description": "Description of the entity"},
                            "fields": {
                                "type": "array",
                                "items": {
                                    "type": "object",
                                    "properties": {
                                        "name": {"type": "string", "description": "Field name"},
                                        "type": {"type": "string", "description": "Rust type like String, Option<i32>, Vec<serde_json::Value>"},
                                        "required": {"type": "boolean", "description": "Whether field is required"},
                                        "description": {"type": "string", "description": "Field description"},
                                        "is_uuid": {"type": "boolean", "description": "Whether field contains UUID references"},
                                        "is_connection": {"type": "boolean", "description": "Whether field represents entity connections"},
                                        "is_spatial": {"type": "boolean", "description": "Whether field contains spatial/hex coordinates"}
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
        "Analyze the supplied HTML/JSON snippets related to *dungeons and dungeon areas*.\n\
         Focus on dungeon structure, areas, rooms, connections between areas, monsters, treasures, traps.\n\
         Look for:\n\
         - Dungeon names and area descriptions\n\
         - Area numbers and connections (which areas connect to which)\n\
         - Monster encounters with stats and UUIDs\n\
         - Treasure items and their properties\n\
         - Trap mechanisms and triggers\n\
         - Special features or environmental hazards\n\
         - Difficulty levels or challenge ratings\n\
         - Entrance locations and hex coordinates\n\
         - Map coordinates and spatial relationships\n\
         Return a JSON object with an 'entities' array describing data models.\n\
         Focus on entity relationships - which monsters are in which areas, what treasures are where.\n\
         If uncertain about a field, omit rather than invent.".to_string()
    }

    fn model_template(&self) -> String {
        r#"//! Generated models for dungeons
//! 
//! This file was generated by the analysis system. Do not edit manually.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

{% for entity in inventory.entities %}
/// {{ entity.description or entity.name }}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct {{ entity.name }} {
{% for field in entity.fields %}
    /// {{ field.description or '' }}
    pub {{ field.name }}: {% if field.required %}{{ field.type }}{% else %}Option<{{ field.type }}>{% endif %},
{% endfor %}
}

impl {{ entity.name }} {
    pub fn new(entity_uuid: String) -> Self {
        Self {
            entity_uuid,
{% for field in entity.fields %}
{% if field.name != "entity_uuid" %}
            {{ field.name }}: {% if field.required %}Default::default(){% else %}None{% endif %},
{% endif %}
{% endfor %}
        }
    }

    /// Extract UUID connections from this entity
    pub fn extract_uuid_connections(&self) -> HashMap<String, Vec<String>> {
        let mut connections = HashMap::new();
{% for field in entity.fields %}
{% if field.is_uuid or field.is_connection %}
        {% if field.type.startswith("Vec<") %}
        connections.insert("{{ field.name }}".to_string(), self.{{ field.name }}.clone());
        {% elif field.type.startswith("Option<String>") %}
        if let Some(ref uuid) = self.{{ field.name }} {
            connections.insert("{{ field.name }}".to_string(), vec![uuid.clone()]);
        }
        {% elif field.type == "String" %}
        connections.insert("{{ field.name }}".to_string(), vec![self.{{ field.name }}.clone()]);
        {% endif %}
{% endif %}
{% endfor %}
        connections
    }

    /// Extract spatial coordinates from this entity
    pub fn extract_spatial_info(&self) -> HashMap<String, String> {
        let mut spatial = HashMap::new();
{% for field in entity.fields %}
{% if field.is_spatial %}
        {% if field.type.startswith("Option<String>") %}
        if let Some(ref coord) = self.{{ field.name }} {
            spatial.insert("{{ field.name }}".to_string(), coord.clone());
        }
        {% elif field.type == "String" %}
        spatial.insert("{{ field.name }}".to_string(), self.{{ field.name }}.clone());
        {% endif %}
{% endif %}
{% endfor %}
        spatial
    }
}

{% endfor %}

{% if inventory.notes %}
/*
Generation Notes:
{% for note in inventory.notes %}
- {{ note }}
{% endfor %}
*/
{% endif %}
"#.to_string()
    }

    fn generate_models(
        &self,
        models_dir: &Path,
        logger: &mut dyn std::io::Write,
    ) -> Result<GenerationResults> {
        // Use the base implementation for now, but this could be specialized
        // for dungeon-specific AI generation if needed
        let inventory = self.base.analyze_entities(logger)?;
        self.base.generate_code_from_inventory(&inventory, models_dir, logger)
    }
}

/// Dungeon-specific utility functions
pub mod utils {
    use super::*;

    /// Extract all area connections from a dungeon
    pub fn extract_area_connections(areas: &[DungeonArea]) -> HashMap<i32, Vec<i32>> {
        let mut connections = HashMap::new();
        
        for area in areas {
            if let Some(area_num) = area.area_number {
                connections.insert(area_num, area.connected_areas.clone());
            }
        }
        
        connections
    }

    /// Find areas accessible from a given hex coordinate
    pub fn find_areas_at_hex<'a>(areas: &'a [DungeonArea], hex_key: &str) -> Vec<&'a DungeonArea> {
        areas.iter()
            .filter(|area| {
                area.entrance_hex.as_ref().map_or(false, |hex| hex == hex_key)
            })
            .collect()
    }

    /// Calculate total dungeon difficulty
    pub fn calculate_dungeon_difficulty(areas: &[DungeonArea]) -> Option<f32> {
        let difficulties: Vec<i32> = areas.iter()
            .filter_map(|area| area.difficulty_level)
            .collect();
            
        if difficulties.is_empty() {
            None
        } else {
            Some(difficulties.iter().sum::<i32>() as f32 / difficulties.len() as f32)
        }
    }

    /// Extract all monster UUIDs from a dungeon
    pub fn extract_monster_uuids(areas: &[DungeonArea]) -> Vec<String> {
        let mut monster_uuids = Vec::new();
        
        for area in areas {
            for monster in &area.monsters {
                if let Some(uuid) = monster.get("uuid").and_then(|v| v.as_str()) {
                    monster_uuids.push(uuid.to_string());
                }
            }
        }
        
        monster_uuids
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dl_types::analysis::raw::RawEntity;

    #[test]
    fn test_dungeon_area_creation() {
        let area = DungeonArea::new("test-uuid".to_string());
        assert_eq!(area.entity_uuid, "test-uuid");
        assert!(area.connected_areas.is_empty());
        assert!(area.monsters.is_empty());
    }

    #[test]
    fn test_dungeon_cluster() {
        let mut cluster = RawDungeonEntities::new("Test Dungeon".to_string());
        let entity = RawEntity::create("test-uuid".to_string(), "Test dungeon content".to_string());
        
        // Test entity addition (would need proper categorization)
        assert_eq!(cluster.category(), EntityCategory::Dungeons);
        assert_eq!(cluster.cluster_name(), "Test Dungeon");
    }

    #[test]
    fn test_area_connections() {
        let mut area1 = DungeonArea::new("area1".to_string());
        area1.area_number = Some(1);
        area1.connected_areas = vec![2, 3];

        let mut area2 = DungeonArea::new("area2".to_string());
        area2.area_number = Some(2);
        area2.connected_areas = vec![1];

        let areas = vec![area1, area2];
        let connections = utils::extract_area_connections(&areas);
        
        assert_eq!(connections[&1], vec![2, 3]);
        assert_eq!(connections[&2], vec![1]);
    }

    #[test]
    fn test_uuid_extraction() {
        let mut area = DungeonArea::new("test-area".to_string());
        area.monsters.push(serde_json::json!({
            "uuid": "monster-123",
            "name": "Goblin"
        }));
        area.treasure = Some(serde_json::json!({
            "uuid": "treasure-456",
            "name": "Gold Chest"
        }));

        let uuids = area.extract_referenced_uuids();
        assert!(uuids.contains(&"monster-123".to_string()));
        assert!(uuids.contains(&"treasure-456".to_string()));
    }
}
