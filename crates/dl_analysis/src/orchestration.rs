//! Orchestration system for the 3-phase analysis pipeline.
//! 
//! Mirrors the Python orchestration.py with master coordination of:
//! Phase 1: Individual category models  
//! Phase 2: Dungeon container models
//! Phase 3: Region container models

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::io::Write;
use anyhow::Result;

use crate::base::{KNOWN_REGIONS, KNOWN_SETTLEMENTS, KNOWN_FACTIONS, KNOWN_DUNGEONS};
use crate::raw::{RawEntity, EntityStats};
use crate::clusters::{
    EntityCluster, RegionEntitiesCluster, SettlementEntitiesCluster, 
    FactionEntitiesCluster, DungeonEntitiesCluster
};
use crate::results::{GenerationResults, AnalysisSummary};

/// Master orchestration container for 3-phase analysis pipeline
#[derive(Debug)]
pub struct RawEntities {
    pub regions: HashMap<String, Box<dyn EntityCluster>>,
    pub settlements: HashMap<String, Box<dyn EntityCluster>>,
    pub factions: HashMap<String, Box<dyn EntityCluster>>,
    pub dungeons: HashMap<String, Box<dyn EntityCluster>>,
    pub uncategorized: Vec<RawEntity>,
    pub total_entities: usize,
    pub stats: EntityStats,
}

impl RawEntities {
    /// Create new orchestrator with initialized clusters
    pub fn new() -> Self {
        let mut orchestrator = Self {
            regions: HashMap::new(),
            settlements: HashMap::new(),
            factions: HashMap::new(),
            dungeons: HashMap::new(),
            uncategorized: Vec::new(),
            total_entities: 0,
            stats: EntityStats::new(),
        };

        // Initialize region clusters
        for region in KNOWN_REGIONS {
            orchestrator.regions.insert(
                region.to_string(),
                Box::new(RegionEntitiesCluster::new(region.to_string()))
            );
        }

        // Initialize settlement clusters
        for settlement in KNOWN_SETTLEMENTS {
            orchestrator.settlements.insert(
                settlement.to_string(),
                Box::new(SettlementEntitiesCluster::new(settlement.to_string()))
            );
        }

        // Initialize faction clusters
        for faction in KNOWN_FACTIONS {
            orchestrator.factions.insert(
                faction.to_string(),
                Box::new(FactionEntitiesCluster::new(faction.to_string()))
            );
        }

        // Initialize dungeon clusters  
        for dungeon in KNOWN_DUNGEONS {
            orchestrator.dungeons.insert(
                dungeon.to_string(),
                Box::new(DungeonEntitiesCluster::new(dungeon.to_string()))
            );
        }

        orchestrator
    }

    /// Add entity and route to appropriate cluster using factory method
    pub fn add_entity(&mut self, uuid: String, value: String) {
        let entity = RawEntity::create(uuid, value);
        self.stats.add_entity(&entity);

        // Try to add to appropriate cluster
        let added = match &entity.category {
            crate::raw::EntityCategory::Regions => {
                if let Some(entity_name) = &entity.entity_name {
                    if let Some(cluster) = self.regions.get_mut(entity_name) {
                        cluster.add_entity(entity.clone())
                    } else { false }
                } else { false }
            }
            crate::raw::EntityCategory::Settlements => {
                if let Some(entity_name) = &entity.entity_name {
                    if let Some(cluster) = self.settlements.get_mut(entity_name) {
                        cluster.add_entity(entity.clone())
                    } else { false }
                } else { false }
            }
            crate::raw::EntityCategory::Factions => {
                if let Some(entity_name) = &entity.entity_name {
                    if let Some(cluster) = self.factions.get_mut(entity_name) {
                        cluster.add_entity(entity.clone())
                    } else { false }
                } else { false }
            }
            crate::raw::EntityCategory::Dungeons => {
                if let Some(entity_name) = &entity.entity_name {
                    if let Some(cluster) = self.dungeons.get_mut(entity_name) {
                        cluster.add_entity(entity.clone())
                    } else { false }
                } else { false }
            }
            _ => false
        };

        if !added {
            self.uncategorized.push(entity);
        }

        self.total_entities += 1;
    }

    /// Write all entities to disk in their cluster directories
    pub fn write_all_entities(&mut self, analysis_dir: &Path, logger: &mut dyn Write) -> Result<()> {
        writeln!(logger, "Writing clustered entities to disk...")?;

        // Write region entities
        for (cluster_name, cluster) in &mut self.regions {
            if cluster.can_generate_models() {
                writeln!(logger, "  Writing {}: entities", cluster_name)?;
                cluster.write_entities_to_disk(analysis_dir)?;
            }
        }

        // Write settlement entities
        for (cluster_name, cluster) in &mut self.settlements {
            if cluster.can_generate_models() {
                writeln!(logger, "  Writing {}: entities", cluster_name)?;
                cluster.write_entities_to_disk(analysis_dir)?;
            }
        }

        // Write faction entities
        for (cluster_name, cluster) in &mut self.factions {
            if cluster.can_generate_models() {
                writeln!(logger, "  Writing {}: entities", cluster_name)?;
                cluster.write_entities_to_disk(analysis_dir)?;
            }
        }

        // Write dungeon entities
        for (cluster_name, cluster) in &mut self.dungeons {
            if cluster.can_generate_models() {
                writeln!(logger, "  Writing {}: entities", cluster_name)?;
                cluster.write_entities_to_disk(analysis_dir)?;
            }
        }

        Ok(())
    }

    /// Generate AI models for all clusters - Phase 1
    pub fn generate_all_individual_models(
        &self,
        models_dir: &Path,
        logger: &mut dyn Write,
    ) -> Result<HashMap<String, GenerationResults>> {
        writeln!(logger, "PHASE 1: Generating individual category models...")?;
        let mut results = HashMap::new();

        // Process regions by combining all individual clusters
        writeln!(logger, "Processing regions...")?;
        let mut combined_regions = RegionEntitiesCluster::combined();
        let mut region_entity_count = 0;
        for (_name, cluster) in &self.regions {
            // Extract entities from individual clusters and add to combined
            // This is where we'd need to access the entities in each cluster
            // For now, just count clusters that can generate models
            if cluster.can_generate_models() {
                region_entity_count += 1;
            }
        }
        
        if region_entity_count > 0 {
            writeln!(logger, "  Found {} region clusters with entities", region_entity_count)?;
            let result = combined_regions.generate_models(models_dir, logger)?;
            results.insert("regions".to_string(), result);
            if results["regions"].success {
                writeln!(logger, "✓ Generated models for regions")?;
            } else {
                writeln!(logger, "✗ Failed to generate models for regions")?;
            }
        } else {
            writeln!(logger, "No region clusters with samples found")?;
        }

        // Process settlements by combining all individual clusters
        writeln!(logger, "Processing settlements...")?;
        let mut combined_settlements = SettlementEntitiesCluster::combined();
        let mut settlement_entity_count = 0;
        for (_name, cluster) in &self.settlements {
            if cluster.can_generate_models() {
                settlement_entity_count += 1;
            }
        }
        
        if settlement_entity_count > 0 {
            writeln!(logger, "  Found {} settlement clusters with entities", settlement_entity_count)?;
            let result = combined_settlements.generate_models(models_dir, logger)?;
            results.insert("settlements".to_string(), result);
            if results["settlements"].success {
                writeln!(logger, "✓ Generated models for settlements")?;
            } else {
                writeln!(logger, "✗ Failed to generate models for settlements")?;
            }
        } else {
            writeln!(logger, "No settlement clusters with samples found")?;
        }

        // Process factions by combining all individual clusters
        writeln!(logger, "Processing factions...")?;
        let mut combined_factions = FactionEntitiesCluster::combined();
        let mut faction_entity_count = 0;
        for (_name, cluster) in &self.factions {
            if cluster.can_generate_models() {
                faction_entity_count += 1;
            }
        }
        
        if faction_entity_count > 0 {
            writeln!(logger, "  Found {} faction clusters with entities", faction_entity_count)?;
            let result = combined_factions.generate_models(models_dir, logger)?;
            results.insert("factions".to_string(), result);
            if results["factions"].success {
                writeln!(logger, "✓ Generated models for factions")?;
            } else {
                writeln!(logger, "✗ Failed to generate models for factions")?;
            }
        } else {
            writeln!(logger, "No faction clusters with samples found")?;
        }

        // Process dungeons by combining all individual clusters
        writeln!(logger, "Processing dungeons...")?;
        let mut combined_dungeons = DungeonEntitiesCluster::combined();
        let mut dungeon_entity_count = 0;
        for (_name, cluster) in &self.dungeons {
            if cluster.can_generate_models() {
                dungeon_entity_count += 1;
            }
        }
        
        if dungeon_entity_count > 0 {
            writeln!(logger, "  Found {} dungeon clusters with entities", dungeon_entity_count)?;
            let result = combined_dungeons.generate_models(models_dir, logger)?;
            results.insert("dungeons".to_string(), result);
            if results["dungeons"].success {
                writeln!(logger, "✓ Generated models for dungeons")?;
            } else {
                writeln!(logger, "✗ Failed to generate models for dungeons")?;
            }
        } else {
            writeln!(logger, "No dungeon clusters with samples found")?;
        }

        Ok(results)
    }

    /// Generate container models - Phase 2 & 3
    pub fn generate_container_models(
        &self,
        models_dir: &Path,
        phase1_results: &HashMap<String, GenerationResults>,
        logger: &mut dyn Write,
    ) -> Result<HashMap<String, GenerationResults>> {
        let mut results = HashMap::new();

        // Phase 2: Dungeon containers
        writeln!(logger, "PHASE 2: Generating dungeon container models...")?;
        if let Some(dungeons_result) = phase1_results.get("dungeons") {
            if dungeons_result.success {
                if let Some(dungeons_connections) = &dungeons_result.connections {
                    let dungeon_container_result = self.generate_dungeon_container_model(
                        models_dir,
                        dungeons_connections,
                        logger,
                    )?;
                    results.insert("dungeon_container".to_string(), dungeon_container_result);
                    writeln!(logger, "✓ Generated dungeon_container model")?;
                }
            }
        }

        // Phase 3: Region containers
        writeln!(logger, "PHASE 3: Generating region container models...")?;
        let mut individual_models = HashMap::new();
        for category in ["regions", "settlements", "factions", "dungeons"] {
            if let Some(result) = phase1_results.get(category) {
                if result.success {
                    if let Some(connections) = &result.connections {
                        individual_models.insert(category.to_string(), connections);
                    }
                }
            }
        }

        if !individual_models.is_empty() {
            let region_container_result = self.generate_region_container_model(
                models_dir,
                &individual_models,
                logger,
            )?;
            results.insert("region_container".to_string(), region_container_result);
            writeln!(logger, "✓ Generated region_container model")?;
        }

        Ok(results)
    }

    /// Generate dungeon container model (placeholder implementation)
    fn generate_dungeon_container_model(
        &self,
        models_dir: &Path,
        dungeons_connections: &crate::results::ModelConnections,
        logger: &mut dyn Write,
    ) -> Result<GenerationResults> {
        writeln!(logger, "  Generating dungeon container from connections...")?;

        let model_content = format!(
            r#"//! Generated dungeon container models
//! 
//! This file was generated by the analysis system. Do not edit manually.

use serde::{{Deserialize, Serialize}};
use std::collections::HashMap;

/// Container for complete dungeon complexes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DungeonContainer {{
    pub dungeon_uuid: String,
    pub areas: Vec<DungeonAreaRef>,
    pub area_connections: HashMap<String, Vec<String>>,
    pub entrance_hex: Option<String>,
}}

impl DungeonContainer {{
    pub fn new(dungeon_uuid: String) -> Self {{
        Self {{
            dungeon_uuid,
            areas: Vec::new(),
            area_connections: HashMap::new(),
            entrance_hex: None,
        }}
    }}
}}

/// Reference to a dungeon area
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DungeonAreaRef {{
    pub area_uuid: String,
    pub level: u32,
}}

/*
Generation Notes:
- Generated from dungeon connections: {}
- Import path: {}
*/
"#,
            dungeons_connections.exported_classes.join(", "),
            dungeons_connections.import_path
        );

        let model_path = models_dir.join("dungeon_container.rs");
        std::fs::write(&model_path, model_content)?;

        Ok(GenerationResults::success(vec![model_path.to_string_lossy().to_string()])
            .add_note("Generated from dungeon connections".to_string()))
    }

    /// Generate region container model (placeholder implementation)
    fn generate_region_container_model(
        &self,
        models_dir: &Path,
        individual_models: &HashMap<String, &crate::results::ModelConnections>,
        logger: &mut dyn Write,
    ) -> Result<GenerationResults> {
        writeln!(logger, "  Generating region container from all individual models...")?;

        let model_content = format!(
            r#"//! Generated region container models
//! 
//! This file was generated by the analysis system. Do not edit manually.

use serde::{{Deserialize, Serialize}};
use std::collections::HashMap;

/// Container for complete regional gameplay
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionContainer {{
    pub region_uuid: String,
    pub hex_tiles: HashMap<String, HexTileRef>,
    pub settlements: Vec<SettlementRef>,
    pub factions: Vec<FactionRef>,
    pub dungeons: Vec<DungeonRef>,
    pub spatial_index: HashMap<String, Vec<String>>,
}}

impl RegionContainer {{
    pub fn new(region_uuid: String) -> Self {{
        Self {{
            region_uuid,
            hex_tiles: HashMap::new(),
            settlements: Vec::new(),
            factions: Vec::new(),
            dungeons: Vec::new(),
            spatial_index: HashMap::new(),
        }}
    }}
}}

/// Reference to a hex tile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HexTileRef {{
    pub hex_uuid: String,
    pub hex_coordinate: String,
}}

/// Reference to a settlement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettlementRef {{
    pub settlement_uuid: String,
    pub name: String,
}}

/// Reference to a faction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactionRef {{
    pub faction_uuid: String,
    pub name: String,
}}

/// Reference to a dungeon
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DungeonRef {{
    pub dungeon_uuid: String,
    pub name: String,
}}

/*
Generation Notes:
- Generated from all individual models: {}
*/
"#,
            individual_models.keys().cloned().collect::<Vec<_>>().join(", ")
        );

        let model_path = models_dir.join("region_container.rs");
        std::fs::write(&model_path, model_content)?;

        Ok(GenerationResults::success(vec![model_path.to_string_lossy().to_string()])
            .add_note("Generated from all individual models".to_string()))
    }

    /// Get summary of entity counts by category
    pub fn get_summary(&self) -> HashMap<String, HashMap<String, usize>> {
        let mut summary = HashMap::new();

        // Regions summary - count entities that can generate models
        let mut regions_summary = HashMap::new();
        for (name, cluster) in &self.regions {
            let count = if cluster.can_generate_models() { 1 } else { 0 };
            regions_summary.insert(name.clone(), count);
        }
        summary.insert("regions".to_string(), regions_summary);

        // Settlements summary - count entities that can generate models
        let mut settlements_summary = HashMap::new();
        for (name, cluster) in &self.settlements {
            let count = if cluster.can_generate_models() { 1 } else { 0 };
            settlements_summary.insert(name.clone(), count);
        }
        summary.insert("settlements".to_string(), settlements_summary);

        // Factions summary - count entities that can generate models
        let mut factions_summary = HashMap::new();
        for (name, cluster) in &self.factions {
            let count = if cluster.can_generate_models() { 1 } else { 0 };
            factions_summary.insert(name.clone(), count);
        }
        summary.insert("factions".to_string(), factions_summary);

        // Dungeons summary - count entities that can generate models
        let mut dungeons_summary = HashMap::new();
        for (name, cluster) in &self.dungeons {
            let count = if cluster.can_generate_models() { 1 } else { 0 };
            dungeons_summary.insert(name.clone(), count);
        }
        summary.insert("dungeons".to_string(), dungeons_summary);

        summary
    }

    /// Run complete analysis pipeline
    pub fn run_complete_analysis(
        &mut self,
        analysis_dir: &Path,
        models_dir: &Path,
        logger: &mut dyn Write,
    ) -> Result<AnalysisSummary> {
        writeln!(logger, "=== STARTING COMPLETE ANALYSIS PIPELINE ===")?;
        writeln!(logger, "Total entities to process: {}", self.total_entities)?;
        writeln!(logger)?;

        // Write entities to disk
        self.write_all_entities(analysis_dir, logger)?;

        // Phase 1: Individual models
        let phase1_results = self.generate_all_individual_models(models_dir, logger)?;

        // Phase 2 & 3: Container models
        let container_results = self.generate_container_models(models_dir, &phase1_results, logger)?;

        // Combine all results
        let mut all_results = phase1_results;
        all_results.extend(container_results);

        // Create summary
        let entity_counts = self.get_summary();
        let summary = AnalysisSummary::new()
            .set_entity_counts(entity_counts)
            .set_uncategorized_count(self.uncategorized.len());

        let summary = all_results.into_iter().fold(summary, |acc, (phase, result)| {
            acc.add_generation_result(phase, result)
        });

        let summary = summary.add_note(format!("Processed {} total entities", self.total_entities));

        writeln!(logger)?;
        writeln!(logger, "{}", summary.summary_text())?;

        Ok(summary)
    }
}

impl Default for RawEntities {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_orchestrator_creation() {
        let orchestrator = RawEntities::new();
        
        assert!(!orchestrator.regions.is_empty());
        assert!(!orchestrator.settlements.is_empty());
        assert!(!orchestrator.factions.is_empty());
        assert!(!orchestrator.dungeons.is_empty());
        assert_eq!(orchestrator.total_entities, 0);
    }

    #[test]
    fn test_entity_addition() {
        let mut orchestrator = RawEntities::new();
        orchestrator.add_entity("test_uuid".to_string(), "Aurora Bushes content".to_string());
        
        assert_eq!(orchestrator.total_entities, 1);
        assert_eq!(orchestrator.stats.total_entities, 1);
    }

    #[test]
    fn test_summary_generation() {
        let orchestrator = RawEntities::new();
        let summary = orchestrator.get_summary();
        
        assert!(summary.contains_key("regions"));
        assert!(summary.contains_key("settlements"));
        assert!(summary.contains_key("factions"));
        assert!(summary.contains_key("dungeons"));
    }
}
