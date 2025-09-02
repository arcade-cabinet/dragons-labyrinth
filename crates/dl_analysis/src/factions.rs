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

use crate::clusters::{BaseEntitiesCluster, EntityCluster};
use crate::raw::{RawEntity, EntityCategory};
use crate::results::GenerationResults;
use crate::base::HexKey;
use crate::templates::TemplateManager;

/// Re-export the main entity from entities module
pub use crate::entities::{FactionEntity, FactionPower};

/// Specialized cluster for faction entities with faction-specific AI generation
#[derive(Debug, Clone)]
pub struct RawFactionEntities {
    base: BaseEntitiesCluster,
    template_manager: Option<TemplateManager>,
}

impl RawFactionEntities {
    pub fn new(cluster_name: String) -> Self {
        Self {
            base: BaseEntitiesCluster::new(EntityCategory::Factions, cluster_name),
            template_manager: TemplateManager::new().ok(),
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

        // Use template manager if available
        let model_content = if let Some(ref template_manager) = self.template_manager {
            let mut metadata = HashMap::new();
            metadata.insert("category".to_string(), "factions".to_string());
            metadata.insert("module_type".to_string(), "faction_entities".to_string());
            
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

/// Faction-specific utility functions
pub mod utils {
    use super::*;

    /// Group factions by power level
    pub fn group_by_power(factions: Vec<FactionEntity>) -> HashMap<FactionPower, Vec<FactionEntity>> {
        let mut power_groups: HashMap<FactionPower, Vec<FactionEntity>> = HashMap::new();
        
        for faction in factions {
            let power = faction.get_power_level();
            power_groups.entry(power).or_insert_with(Vec::new).push(faction);
        }
        
        power_groups
    }

    /// Find factions that control a specific hex
    pub fn find_factions_controlling_hex<'a>(
        factions: &'a [FactionEntity],
        hex_key: &HexKey
    ) -> Vec<&'a FactionEntity> {
        factions.iter()
            .filter(|faction| faction.controls_hex(hex_key))
            .collect()
    }

    /// Find all allies of a faction
    pub fn find_faction_allies<'a>(
        factions: &'a [FactionEntity],
        faction_uuid: &str
    ) -> Vec<&'a FactionEntity> {
        factions.iter()
            .filter(|faction| faction.is_ally(faction_uuid))
            .collect()
    }

    /// Find all enemies of a faction
    pub fn find_faction_enemies<'a>(
        factions: &'a [FactionEntity],
        faction_uuid: &str
    ) -> Vec<&'a FactionEntity> {
        factions.iter()
            .filter(|faction| faction.is_enemy(faction_uuid))
            .collect()
    }

    /// Calculate faction influence map (hex -> [faction_uuids])
    pub fn build_influence_map(factions: &[FactionEntity]) -> HashMap<HexKey, Vec<String>> {
        let mut influence_map: HashMap<HexKey, Vec<String>> = HashMap::new();
        
        for faction in factions {
            for hex_key in &faction.territories {
                influence_map
                    .entry(hex_key.clone())
                    .or_insert_with(Vec::new)
                    .push(faction.entity_uuid.clone());
            }
        }
        
        influence_map
    }

    /// Find contested territories (hexes with multiple faction claims)
    pub fn find_contested_territories(factions: &[FactionEntity]) -> HashMap<HexKey, Vec<String>> {
        let influence_map = build_influence_map(factions);
        
        influence_map.into_iter()
            .filter(|(_, faction_uuids)| faction_uuids.len() > 1)
            .collect()
    }

    /// Calculate average influence level
    pub fn calculate_average_influence(factions: &[FactionEntity]) -> Option<f32> {
        let influence_levels: Vec<i32> = factions.iter()
            .filter_map(|faction| faction.influence_level)
            .collect();
            
        if influence_levels.is_empty() {
            None
        } else {
            Some(influence_levels.iter().sum::<i32>() as f32 / influence_levels.len() as f32)
        }
    }

    /// Build alliance network (faction relationships)
    pub fn build_alliance_network(factions: &[FactionEntity]) -> HashMap<String, Vec<String>> {
        let mut alliance_network: HashMap<String, Vec<String>> = HashMap::new();
        
        for faction in factions {
            alliance_network.insert(
                faction.entity_uuid.clone(),
                faction.allegiances.clone()
            );
        }
        
        alliance_network
    }

    /// Find faction by type
    pub fn find_factions_by_type<'a>(
        factions: &'a [FactionEntity],
        faction_type: &str
    ) -> Vec<&'a FactionEntity> {
        factions.iter()
            .filter(|faction| {
                faction.faction_type
                    .as_ref()
                    .map_or(false, |ftype| ftype == faction_type)
            })
            .collect()
    }

    /// Calculate territorial control (total hexes controlled by each faction)
    pub fn calculate_territorial_control(factions: &[FactionEntity]) -> HashMap<String, usize> {
        let mut control_map: HashMap<String, usize> = HashMap::new();
        
        for faction in factions {
            control_map.insert(
                faction.entity_uuid.clone(),
                faction.territories.len()
            );
        }
        
        control_map
    }

    /// Find the most powerful faction
    pub fn find_most_powerful_faction(factions: &[FactionEntity]) -> Option<&FactionEntity> {
        factions.iter()
            .filter(|faction| faction.influence_level.is_some())
            .max_by_key(|faction| faction.influence_level.unwrap_or(0))
    }

    /// Count factions by type
    pub fn count_by_type(factions: &[FactionEntity]) -> HashMap<String, usize> {
        let mut type_counts: HashMap<String, usize> = HashMap::new();
        
        for faction in factions {
            let faction_type = faction.faction_type
                .clone()
                .unwrap_or_else(|| "unknown".to_string());
            *type_counts.entry(faction_type).or_insert(0) += 1;
        }
        
        type_counts
    }

    /// Find neutral factions (no allies or enemies)
    pub fn find_neutral_factions(factions: &[FactionEntity]) -> Vec<&FactionEntity> {
        factions.iter()
            .filter(|faction| faction.allegiances.is_empty() && faction.enemies.is_empty())
            .collect()
    }

    /// Calculate faction relationship strength
    pub fn calculate_relationship_strength(
        faction1: &FactionEntity,
        faction2: &FactionEntity
    ) -> RelationshipStrength {
        let uuid1 = &faction1.entity_uuid;
        let uuid2 = &faction2.entity_uuid;
        
        if faction1.is_ally(uuid2) && faction2.is_ally(uuid1) {
            RelationshipStrength::StrongAlliance
        } else if faction1.is_ally(uuid2) || faction2.is_ally(uuid1) {
            RelationshipStrength::WeakAlliance
        } else if faction1.is_enemy(uuid2) && faction2.is_enemy(uuid1) {
            RelationshipStrength::StrongEnmity
        } else if faction1.is_enemy(uuid2) || faction2.is_enemy(uuid1) {
            RelationshipStrength::WeakEnmity
        } else {
            RelationshipStrength::Neutral
        }
    }
}

/// Relationship strength between factions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RelationshipStrength {
    StrongAlliance,
    WeakAlliance,
    Neutral,
    WeakEnmity,
    StrongEnmity,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::raw::RawEntity;

    #[test]
    fn test_faction_cluster() {
        let mut cluster = RawFactionEntities::new("Order of the Silver Hand".to_string());
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

    #[test]
    fn test_power_grouping() {
        let mut minor_faction = FactionEntity::new("minor1".to_string());
        minor_faction.influence_level = Some(2);
        
        let mut major_faction = FactionEntity::new("major1".to_string());
        major_faction.influence_level = Some(8);
        
        let factions = vec![minor_faction, major_faction];
        let power_groups = utils::group_by_power(factions);
        
        assert_eq!(power_groups[&FactionPower::Minor].len(), 1);
        assert_eq!(power_groups[&FactionPower::Major].len(), 1);
    }

    #[test]
    fn test_territorial_control() {
        let mut faction1 = FactionEntity::new("faction1".to_string());
        faction1.territories.push("W2S51".to_string());
        faction1.territories.push("W2S52".to_string());
        
        let mut faction2 = FactionEntity::new("faction2".to_string());
        faction2.territories.push("E3N12".to_string());
        
        let factions = vec![faction1, faction2];
        let influence_map = utils::build_influence_map(&factions);
        
        assert!(influence_map.contains_key("W2S51"));
        assert!(influence_map.contains_key("E3N12"));
        assert_eq!(influence_map["W2S51"], vec!["faction1"]);
    }

    #[test]
    fn test_alliance_detection() {
        let mut faction1 = FactionEntity::new("faction1".to_string());
        faction1.allegiances.push("faction2".to_string());
        
        let mut faction2 = FactionEntity::new("faction2".to_string());
        faction2.allegiances.push("faction1".to_string());
        
        let factions = vec![faction1, faction2];
        let allies = utils::find_faction_allies(&factions, "faction1");
        
        assert_eq!(allies.len(), 1);
        assert_eq!(allies[0].entity_uuid, "faction2");
    }

    #[test]
    fn test_contested_territories() {
        let mut faction1 = FactionEntity::new("faction1".to_string());
        faction1.territories.push("contested_hex".to_string());
        
        let mut faction2 = FactionEntity::new("faction2".to_string());
        faction2.territories.push("contested_hex".to_string());
        
        let factions = vec![faction1, faction2];
        let contested = utils::find_contested_territories(&factions);
        
        assert!(contested.contains_key("contested_hex"));
        assert_eq!(contested["contested_hex"].len(), 2);
    }

    #[test]
    fn test_relationship_strength() {
        let mut faction1 = FactionEntity::new("faction1".to_string());
        faction1.allegiances.push("faction2".to_string());
        
        let mut faction2 = FactionEntity::new("faction2".to_string());
        faction2.allegiances.push("faction1".to_string());
        
        let strength = utils::calculate_relationship_strength(&faction1, &faction2);
        assert_eq!(strength, RelationshipStrength::StrongAlliance);
        
        let mut faction3 = FactionEntity::new("faction3".to_string());
        faction3.enemies.push("faction1".to_string());
        
        let strength = utils::calculate_relationship_strength(&faction1, &faction3);
        assert_eq!(strength, RelationshipStrength::WeakEnmity);
    }

    #[test]
    fn test_neutral_factions() {
        let mut allied_faction = FactionEntity::new("allied".to_string());
        allied_faction.allegiances.push("someone".to_string());
        
        let neutral_faction = FactionEntity::new("neutral".to_string());
        
        let factions = vec![allied_faction, neutral_faction];
        let neutral_factions = utils::find_neutral_factions(&factions);
        
        assert_eq!(neutral_factions.len(), 1);
        assert_eq!(neutral_factions[0].entity_uuid, "neutral");
    }

    #[test]
    fn test_most_powerful_faction() {
        let mut weak_faction = FactionEntity::new("weak".to_string());
        weak_faction.influence_level = Some(3);
        
        let mut strong_faction = FactionEntity::new("strong".to_string());
        strong_faction.influence_level = Some(9);
        
        let factions = vec![weak_faction, strong_faction];
        let most_powerful = utils::find_most_powerful_faction(&factions);
        
        assert!(most_powerful.is_some());
        assert_eq!(most_powerful.unwrap().entity_uuid, "strong");
    }
}
