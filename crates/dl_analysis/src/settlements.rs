//! Settlement-specific entity models and processing.
//! 
//! This module contains specialized settlement processing logic including:
//! - SettlementEstablishment entity model matching Python settlements.py
//! - RawSettlementEntities cluster with specialized AI generation
//! - Settlement-specific inventory schemas, prompts, and templates
//! - Integration with faction control and population management

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
pub use crate::entities::{SettlementEstablishment, SettlementSize};

/// Specialized cluster for settlement entities with settlement-specific AI generation
#[derive(Debug, Clone)]
pub struct RawSettlementEntities {
    base: BaseEntitiesCluster,
    template_manager: Option<TemplateManager>,
}

impl RawSettlementEntities {
    pub fn new(cluster_name: String) -> Self {
        Self {
            base: BaseEntitiesCluster::new(EntityCategory::Settlements, cluster_name),
            template_manager: TemplateManager::new().ok(),
        }
    }

    pub fn combined() -> Self {
        Self::new("combined".to_string())
    }

    /// Get specialized inventory schema for settlements
    fn settlement_inventory_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "entities": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "properties": {
                            "name": {"type": "string", "description": "Entity class name like SettlementEstablishment"},
                            "description": {"type": "string", "description": "Description of the settlement entity"},
                            "fields": {
                                "type": "array",
                                "items": {
                                    "type": "object",
                                    "properties": {
                                        "name": {"type": "string", "description": "Field name"},
                                        "type": {"type": "string", "description": "Rust type like String, Option<i32>, Vec<String>"},
                                        "required": {"type": "boolean", "description": "Whether field is required"},
                                        "description": {"type": "string", "description": "Field description"},
                                        "is_uuid": {"type": "boolean", "description": "Whether field contains UUID references"},
                                        "is_connection": {"type": "boolean", "description": "Whether field represents entity connections"},
                                        "is_spatial": {"type": "boolean", "description": "Whether field contains location data"}
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

    /// Get specialized analysis prompt for settlements
    fn settlement_analysis_prompt(&self) -> String {
        "Analyze the supplied HTML/JSON snippets related to *settlements and establishments*.\n\
         Focus on populated places, their characteristics, and relationships.\n\
         Look for:\n\
         - Settlement names and types (village, town, city, hamlet, etc.)\n\
         - Population numbers and size indicators\n\
         - Services and facilities available (shops, inns, temples, etc.)\n\
         - Notable NPCs and their roles (mayor, merchant, guard captain, etc.)\n\
         - Defense levels and fortifications\n\
         - Trade goods and economic activities\n\
         - Controlling faction or ruling authority\n\
         - Location information and hex coordinates\n\
         - Relationships with other settlements\n\
         - Cultural or racial demographics\n\
         Return a JSON object with an 'entities' array describing settlement data models.\n\
         Focus on social and economic aspects - what makes each settlement unique.\n\
         Identify UUID references that connect settlements to factions, NPCs, and regions.\n\
         If uncertain about a field, omit rather than invent.".to_string()
    }

    /// Generate models using sophisticated template system
    fn generate_models_with_templates(
        &self,
        models_dir: &Path,
        logger: &mut dyn std::io::Write,
    ) -> Result<GenerationResults> {
        writeln!(logger, "Generating settlement models using template system...")?;

        let model_filename = format!("{}.rs", self.category().as_str());
        let model_path = models_dir.join(&model_filename);

        // Check if model already exists (idempotent)
        if model_path.exists() {
            writeln!(logger, "Settlement model already exists: {}", model_path.display())?;
            return Ok(GenerationResults::success(vec![model_path.to_string_lossy().to_string()])
                .add_note("Model already exists, skipped generation".to_string()));
        }

        // Generate inventory through AI analysis
        let inventory = self.base.analyze_entities(logger)?;

        // Use template manager if available
        let model_content = if let Some(ref template_manager) = self.template_manager {
            let mut metadata = HashMap::new();
            metadata.insert("category".to_string(), "settlements".to_string());
            metadata.insert("module_type".to_string(), "settlement_entities".to_string());
            
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

        writeln!(logger, "✓ Generated settlement model: {}", model_path.display())?;

        Ok(GenerationResults::success(vec![model_path.to_string_lossy().to_string()])
            .with_connections(connections)
            .add_note(format!("Generated from {} settlement entity specifications", inventory.entities.len())))
    }
}

impl EntityCluster for RawSettlementEntities {
    fn category(&self) -> EntityCategory {
        EntityCategory::Settlements
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
        self.settlement_inventory_schema()
    }

    fn analysis_prompt(&self) -> String {
        self.settlement_analysis_prompt()
    }

    fn model_template(&self) -> String {
        // This is overridden by the template manager, but provide fallback
        r#"//! Generated models for settlements
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

    /// Extract all referenced UUIDs from this settlement
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

/// Settlement-specific utility functions
pub mod utils {
    use super::*;

    /// Group settlements by size category
    pub fn group_by_size(settlements: Vec<SettlementEstablishment>) -> HashMap<SettlementSize, Vec<SettlementEstablishment>> {
        let mut size_groups: HashMap<SettlementSize, Vec<SettlementEstablishment>> = HashMap::new();
        
        for settlement in settlements {
            let size = settlement.get_size_category();
            size_groups.entry(size).or_insert_with(Vec::new).push(settlement);
        }
        
        size_groups
    }

    /// Find settlements controlled by a specific faction
    pub fn find_settlements_by_faction<'a>(
        settlements: &'a [SettlementEstablishment],
        faction_uuid: &str
    ) -> Vec<&'a SettlementEstablishment> {
        settlements.iter()
            .filter(|settlement| {
                settlement.controlling_faction
                    .as_ref()
                    .map_or(false, |faction| faction == faction_uuid)
            })
            .collect()
    }

    /// Find fortified settlements (defense level > 3)
    pub fn find_fortified_settlements(settlements: &[SettlementEstablishment]) -> Vec<&SettlementEstablishment> {
        settlements.iter()
            .filter(|settlement| settlement.is_fortified())
            .collect()
    }

    /// Calculate total population across settlements
    pub fn calculate_total_population(settlements: &[SettlementEstablishment]) -> i32 {
        settlements.iter()
            .filter_map(|settlement| settlement.population)
            .sum()
    }

    /// Find settlements with specific services
    pub fn find_settlements_with_service<'a>(
        settlements: &'a [SettlementEstablishment],
        service: &str
    ) -> Vec<&'a SettlementEstablishment> {
        settlements.iter()
            .filter(|settlement| settlement.services.contains(&service.to_string()))
            .collect()
    }

    /// Get settlement trade network (settlements with shared trade goods)
    pub fn find_trade_connections(settlements: &[SettlementEstablishment]) -> HashMap<String, Vec<String>> {
        let mut trade_network: HashMap<String, Vec<String>> = HashMap::new();
        
        for settlement in settlements {
            for trade_good in &settlement.trade_goods {
                trade_network
                    .entry(trade_good.clone())
                    .or_insert_with(Vec::new)
                    .push(settlement.entity_uuid.clone());
            }
        }
        
        // Only keep trade goods with multiple settlements
        trade_network.retain(|_, settlements| settlements.len() > 1);
        trade_network
    }

    /// Find settlements at a specific hex location
    pub fn find_settlements_at_hex<'a>(
        settlements: &'a [SettlementEstablishment],
        hex_key: &HexKey
    ) -> Vec<&'a SettlementEstablishment> {
        settlements.iter()
            .filter(|settlement| {
                settlement.hex_location
                    .as_ref()
                    .map_or(false, |hex| hex == hex_key)
            })
            .collect()
    }

    /// Calculate average defense level
    pub fn calculate_average_defense(settlements: &[SettlementEstablishment]) -> Option<f32> {
        let defense_levels: Vec<i32> = settlements.iter()
            .filter_map(|settlement| settlement.defense_level)
            .collect();
            
        if defense_levels.is_empty() {
            None
        } else {
            Some(defense_levels.iter().sum::<i32>() as f32 / defense_levels.len() as f32)
        }
    }

    /// Find the largest settlement by population
    pub fn find_largest_settlement(settlements: &[SettlementEstablishment]) -> Option<&SettlementEstablishment> {
        settlements.iter()
            .filter(|settlement| settlement.population.is_some())
            .max_by_key(|settlement| settlement.population.unwrap_or(0))
    }

    /// Count settlements by type
    pub fn count_by_type(settlements: &[SettlementEstablishment]) -> HashMap<String, usize> {
        let mut type_counts: HashMap<String, usize> = HashMap::new();
        
        for settlement in settlements {
            let settlement_type = settlement.settlement_type
                .clone()
                .unwrap_or_else(|| "unknown".to_string());
            *type_counts.entry(settlement_type).or_insert(0) += 1;
        }
        
        type_counts
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::raw::RawEntity;

    #[test]
    fn test_settlement_cluster() {
        let mut cluster = RawSettlementEntities::new("Village of Ashamar".to_string());
        assert_eq!(cluster.category(), EntityCategory::Settlements);
        assert_eq!(cluster.cluster_name(), "Village of Ashamar");
    }

    #[test]
    fn test_settlement_inventory_schema() {
        let cluster = RawSettlementEntities::new("test".to_string());
        let schema = cluster.inventory_schema();
        assert!(schema.get("type").is_some());
        assert_eq!(schema["type"], "object");
    }

    #[test]
    fn test_settlement_analysis_prompt() {
        let cluster = RawSettlementEntities::new("test".to_string());
        let prompt = cluster.analysis_prompt();
        assert!(prompt.contains("settlements"));
        assert!(prompt.contains("population"));
        assert!(prompt.contains("services"));
    }

    #[test]
    fn test_size_grouping() {
        let mut hamlet = SettlementEstablishment::new("hamlet1".to_string());
        hamlet.population = Some(50);
        
        let mut village = SettlementEstablishment::new("village1".to_string());
        village.population = Some(500);
        
        let mut city = SettlementEstablishment::new("city1".to_string());
        city.population = Some(10000);
        
        let settlements = vec![hamlet, village, city];
        let size_groups = utils::group_by_size(settlements);
        
        assert_eq!(size_groups[&SettlementSize::Hamlet].len(), 1);
        assert_eq!(size_groups[&SettlementSize::Village].len(), 1);
        assert_eq!(size_groups[&SettlementSize::City].len(), 1);
    }

    #[test]
    fn test_faction_filtering() {
        let mut settlement1 = SettlementEstablishment::new("settlement1".to_string());
        settlement1.controlling_faction = Some("faction1".to_string());
        
        let mut settlement2 = SettlementEstablishment::new("settlement2".to_string());
        settlement2.controlling_faction = Some("faction2".to_string());
        
        let settlements = vec![settlement1, settlement2];
        let faction1_settlements = utils::find_settlements_by_faction(&settlements, "faction1");
        
        assert_eq!(faction1_settlements.len(), 1);
        assert_eq!(faction1_settlements[0].entity_uuid, "settlement1");
    }

    #[test]
    fn test_fortification_detection() {
        let mut fortified = SettlementEstablishment::new("fort".to_string());
        fortified.defense_level = Some(5);
        
        let mut unfortified = SettlementEstablishment::new("village".to_string());
        unfortified.defense_level = Some(2);
        
        let settlements = vec![fortified, unfortified];
        let forts = utils::find_fortified_settlements(&settlements);
        
        assert_eq!(forts.len(), 1);
        assert_eq!(forts[0].entity_uuid, "fort");
    }

    #[test]
    fn test_population_calculation() {
        let mut settlement1 = SettlementEstablishment::new("settlement1".to_string());
        settlement1.population = Some(1000);
        
        let mut settlement2 = SettlementEstablishment::new("settlement2".to_string());
        settlement2.population = Some(2000);
        
        let settlements = vec![settlement1, settlement2];
        let total_pop = utils::calculate_total_population(&settlements);
        
        assert_eq!(total_pop, 3000);
    }

    #[test]
    fn test_service_filtering() {
        let mut settlement1 = SettlementEstablishment::new("settlement1".to_string());
        settlement1.services.push("blacksmith".to_string());
        settlement1.services.push("inn".to_string());
        
        let mut settlement2 = SettlementEstablishment::new("settlement2".to_string());
        settlement2.services.push("temple".to_string());
        
        let settlements = vec![settlement1, settlement2];
        let blacksmith_settlements = utils::find_settlements_with_service(&settlements, "blacksmith");
        
        assert_eq!(blacksmith_settlements.len(), 1);
        assert_eq!(blacksmith_settlements[0].entity_uuid, "settlement1");
    }

    #[test]
    fn test_trade_network() {
        let mut settlement1 = SettlementEstablishment::new("settlement1".to_string());
        settlement1.trade_goods.push("grain".to_string());
        settlement1.trade_goods.push("wool".to_string());
        
        let mut settlement2 = SettlementEstablishment::new("settlement2".to_string());
        settlement2.trade_goods.push("grain".to_string());
        settlement2.trade_goods.push("iron".to_string());
        
        let settlements = vec![settlement1, settlement2];
        let trade_network = utils::find_trade_connections(&settlements);
        
        assert!(trade_network.contains_key("grain"));
        assert_eq!(trade_network["grain"].len(), 2);
        assert!(!trade_network.contains_key("wool")); // Only one settlement has wool
    }
}
