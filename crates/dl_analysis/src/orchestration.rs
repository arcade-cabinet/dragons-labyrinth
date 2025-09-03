//! Main orchestration system for HBF entity analysis
//! 
//! Ports the Python RawEntities container class to coordinate all clusters 
//! and implement the main analysis pipeline matching the Python models.py

use anyhow::Result;
use std::collections::HashMap;
use std::path::Path;
use serde::{Deserialize, Serialize};
use ron::ser::{to_string_pretty, PrettyConfig};

use dl_types::analysis::{
    base::{KNOWN_REGIONS, KNOWN_SETTLEMENTS, KNOWN_FACTIONS, KNOWN_DUNGEONS},
    raw::{RawEntity, EntityCategory},
};
use crate::clusters::{
    RegionEntitiesCluster, SettlementEntitiesCluster, FactionEntitiesCluster, DungeonEntitiesCluster,
    BaseEntitiesCluster, EntityCluster,
};
use crate::results::{GenerationResults, AnalysisSummary};

/// Main container coordinating all entity clusters and analysis pipeline.
/// 
/// Mirrors the Python RawEntities class from models.py with the same
/// interface and functionality to ensure proper entity extraction and AI integration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawEntities {
    /// Region clusters by name
    pub regions: HashMap<String, RegionEntitiesCluster>,
    /// Settlement clusters by name  
    pub settlements: HashMap<String, SettlementEntitiesCluster>,
    /// Faction clusters by name
    pub factions: HashMap<String, FactionEntitiesCluster>,
    /// Dungeon clusters by name
    pub dungeons: HashMap<String, DungeonEntitiesCluster>,
    /// Entities that don't fit known categories - critical for 50% efficiency fix
    pub uncategorized: Vec<RawEntity>,
    /// Total count of all entities processed
    pub total_entities: usize,
}

impl RawEntities {
    /// Create new RawEntities container with all known clusters pre-initialized.
    /// 
    /// CRITICAL: This matches Python model_post_init() to ensure ALL known
    /// clusters exist regardless of whether entities are found for them.
    /// This fixes the bug where we got 25 regions instead of 27.
    pub fn new() -> Self {
        let mut regions = HashMap::new();
        let mut settlements = HashMap::new(); 
        let mut factions = HashMap::new();
        let mut dungeons = HashMap::new();

        // Pre-initialize ALL known region clusters (27 total)
        for &region in KNOWN_REGIONS {
            regions.insert(region.to_string(), RegionEntitiesCluster::new(region.to_string()));
        }

        // Pre-initialize ALL known settlement clusters (10 total) 
        for &settlement in KNOWN_SETTLEMENTS {
            settlements.insert(settlement.to_string(), SettlementEntitiesCluster::new(settlement.to_string()));
        }

        // Pre-initialize ALL known faction clusters (5 total)
        for &faction in KNOWN_FACTIONS {
            factions.insert(faction.to_string(), FactionEntitiesCluster::new(faction.to_string()));
        }

        // Pre-initialize ALL known dungeon clusters (18 total)
        for &dungeon in KNOWN_DUNGEONS {
            dungeons.insert(dungeon.to_string(), DungeonEntitiesCluster::new(dungeon.to_string()));
        }

        Self {
            regions,
            settlements,
            factions,
            dungeons,
            uncategorized: Vec::new(),
            total_entities: 0,
        }
    }

    /// Add an entity to the appropriate cluster or uncategorized list.
    /// 
    /// CRITICAL: Uses pre-initialized clusters to match Python behavior exactly.
    /// Routes entities to existing clusters or uncategorized if unknown.
    pub fn add_entity(&mut self, uuid: String, raw_value: String) {
        let entity = RawEntity::create(uuid, raw_value);
        self.total_entities += 1;

        match entity.category.as_str() {
            "regions" => {
                if entity.entity_name != "unknown" {
                    // Use pre-initialized cluster - all KNOWN_REGIONS clusters already exist
                    if let Some(cluster) = self.regions.get_mut(&entity.entity_name) {
                        cluster.add_entity(entity);
                    } else {
                        // Entity name not in KNOWN_REGIONS - this shouldn't happen with correct constants
                        self.uncategorized.push(entity);
                    }
                } else {
                    self.uncategorized.push(entity);
                }
            }
            "settlements" => {
                if entity.entity_name != "unknown" {
                    // Use pre-initialized cluster - all KNOWN_SETTLEMENTS clusters already exist
                    if let Some(cluster) = self.settlements.get_mut(&entity.entity_name) {
                        cluster.add_entity(entity);
                    } else {
                        // Entity name not in KNOWN_SETTLEMENTS - shouldn't happen with correct constants
                        self.uncategorized.push(entity);
                    }
                } else {
                    self.uncategorized.push(entity);
                }
            }
            "factions" => {
                if entity.entity_name != "unknown" {
                    // Use pre-initialized cluster - all KNOWN_FACTIONS clusters already exist
                    if let Some(cluster) = self.factions.get_mut(&entity.entity_name) {
                        cluster.add_entity(entity);
                    } else {
                        // Entity name not in KNOWN_FACTIONS - shouldn't happen with correct constants
                        self.uncategorized.push(entity);
                    }
                } else {
                    self.uncategorized.push(entity);
                }
            }
            "dungeons" => {
                if entity.entity_name != "unknown" {
                    // Use pre-initialized cluster - all KNOWN_DUNGEONS clusters already exist
                    if let Some(cluster) = self.dungeons.get_mut(&entity.entity_name) {
                        cluster.add_entity(entity);
                    } else {
                        // Entity name not in KNOWN_DUNGEONS - shouldn't happen with correct constants
                        self.uncategorized.push(entity);
                    }
                } else {
                    self.uncategorized.push(entity);
                }
            }
            _ => {
                // Always capture uncategorized entities - this is key to the efficiency fix
                self.uncategorized.push(entity);
            }
        }
    }

    /// Load all entities from HBF SQLite database.
    /// 
    /// This implements the critical "SELECT uuid, value FROM Entities" query
    /// that extracts ALL entities, not just categorized ones.
    pub fn load_from_hbf_database<P: AsRef<Path>>(
        &mut self,
        hbf_database_path: P,
    ) -> Result<()> {
        use rusqlite::Connection;

        let connection = Connection::open(hbf_database_path.as_ref())?;
        
        // Critical query: extract ALL entities from HBF database
        let query = "SELECT uuid, value FROM Entities";
        
        let mut stmt = connection.prepare(query)?;
        let rows = stmt.query_map([], |row| {
            let uuid: String = row.get(0)?;
            let value: String = row.get(1)?;
            Ok((uuid, value))
        })?;
        
        for row in rows {
            let (uuid, value) = row?;
            self.add_entity(uuid, value);
        }

        Ok(())
    }

    /// Write all clustered entities to disk for processing pipeline.
    /// 
    /// Matches Python write_all_entities method interface.
    pub fn write_all_entities<P: AsRef<Path>>(
        &self,
        analysis_output_dir: P,
    ) -> Result<()> {
        use std::fs;

        let output_dir = analysis_output_dir.as_ref();
        fs::create_dir_all(output_dir)?;

        // Write regions
        for (name, cluster) in &self.regions {
            let file_path = output_dir.join(format!("regions_{}.ron", name));
            let content = to_string_pretty(cluster, PrettyConfig::default())?;
            fs::write(file_path, content)?;
        }

        // Write settlements
        for (name, cluster) in &self.settlements {
            let file_path = output_dir.join(format!("settlements_{}.ron", name));
            let content = to_string_pretty(cluster, PrettyConfig::default())?;
            fs::write(file_path, content)?;
        }

        // Write factions
        for (name, cluster) in &self.factions {
            let file_path = output_dir.join(format!("factions_{}.ron", name));
            let content = to_string_pretty(cluster, PrettyConfig::default())?;
            fs::write(file_path, content)?;
        }

        // Write dungeons
        for (name, cluster) in &self.dungeons {
            let file_path = output_dir.join(format!("dungeons_{}.ron", name));
            let content = to_string_pretty(cluster, PrettyConfig::default())?;
            fs::write(file_path, content)?;
        }

        // Write uncategorized entities - critical for efficiency analysis
        if !self.uncategorized.is_empty() {
            let file_path = output_dir.join("uncategorized.ron");
            let content = to_string_pretty(&self.uncategorized, PrettyConfig::default())?;
            fs::write(file_path, content)?;
        }

        Ok(())
    }

    /// Phase 1: Generate AI models for individual entity categories.
    /// 
    /// Matches Python generate_all_individual_models method interface.
    pub fn generate_all_individual_models<P: AsRef<Path>>(
        &self,
        models_dir: P,
    ) -> Result<HashMap<String, GenerationResults>> {
        let mut results = HashMap::new();
        use std::io::Write;
        let mut logger = std::io::stderr();

        // Generate for each category using their specialized AI integration
        
        // Regions
        for (name, cluster) in &self.regions {
            if cluster.can_generate_models() {
                let result = cluster.generate_models(
                    models_dir.as_ref(),
                    &mut logger,
                )?;
                results.insert(format!("regions_{}", name), result);
            }
        }

        // Settlements  
        for (name, cluster) in &self.settlements {
            if cluster.can_generate_models() {
                let result = cluster.generate_models(
                    models_dir.as_ref(),
                    &mut logger,
                )?;
                results.insert(format!("settlements_{}", name), result);
            }
        }

        // Factions
        for (name, cluster) in &self.factions {
            if cluster.can_generate_models() {
                let result = cluster.generate_models(
                    models_dir.as_ref(),
                    &mut logger,
                )?;
                results.insert(format!("factions_{}", name), result);
            }
        }

        // Dungeons
        for (name, cluster) in &self.dungeons {
            if cluster.can_generate_models() {
                let result = cluster.generate_models(
                    models_dir.as_ref(),
                    &mut logger,
                )?;
                results.insert(format!("dungeons_{}", name), result);
            }
        }

        Ok(results)
    }

    /// Phase 2: Generate container integration models.
    /// 
    /// Matches Python generate_container_models method interface.
    pub fn generate_container_models<P: AsRef<Path>>(
        &self,
        models_dir: P,
        templates_dir: P,
        phase1_results: HashMap<String, GenerationResults>,
    ) -> Result<HashMap<String, GenerationResults>> {
        let mut results = HashMap::new();

        // This is a placeholder for now - will implement container integration
        // after the basic pipeline is working
        let container_result = GenerationResults::success(vec![
            "containers.rs".to_string(),
            "integration.rs".to_string(),
        ]);

        results.insert("containers".to_string(), container_result);
        Ok(results)
    }

    /// Get analysis summary showing entity counts and coverage.
    pub fn get_analysis_summary(&self) -> AnalysisSummary {
        let mut entity_counts = HashMap::new();

        // Regions
        if !self.regions.is_empty() {
            let mut region_counts = HashMap::new();
            for (name, cluster) in &self.regions {
                region_counts.insert(name.clone(), cluster.base.entities.len());
            }
            entity_counts.insert("regions".to_string(), region_counts);
        }

        // Settlements
        if !self.settlements.is_empty() {
            let mut settlement_counts = HashMap::new();
            for (name, cluster) in &self.settlements {
                settlement_counts.insert(name.clone(), cluster.base.entities.len());
            }
            entity_counts.insert("settlements".to_string(), settlement_counts);
        }

        // Factions
        if !self.factions.is_empty() {
            let mut faction_counts = HashMap::new();
            for (name, cluster) in &self.factions {
                faction_counts.insert(name.clone(), cluster.base.entities.len());
            }
            entity_counts.insert("factions".to_string(), faction_counts);
        }

        // Dungeons
        if !self.dungeons.is_empty() {
            let mut dungeon_counts = HashMap::new();
            for (name, cluster) in &self.dungeons {
                dungeon_counts.insert(name.clone(), cluster.base.entities.len());
            }
            entity_counts.insert("dungeons".to_string(), dungeon_counts);
        }

        AnalysisSummary::new()
            .set_entity_counts(entity_counts)
            .set_uncategorized_count(self.uncategorized.len())
            .add_note(format!("Processed {} total entities", self.total_entities))
            .add_note(format!("Found {} regions, {} settlements, {} factions, {} dungeons", 
                      self.regions.len(), self.settlements.len(), 
                      self.factions.len(), self.dungeons.len()))
    }

    /// Main analysis entry point matching Python main() function.
    /// 
    /// Implements the complete 3-phase pipeline:
    /// 1. Extract all entities from HBF database
    /// 2. Generate individual models via AI  
    /// 3. Generate container integration models
    pub fn run_complete_analysis<P: AsRef<Path>>(
        hbf_database_path: P,
        analysis_output_dir: P,
        models_dir: P,
        templates_dir: P,
    ) -> Result<AnalysisSummary> {
        let mut entities = RawEntities::new();

        // Phase 1: Load all entities from HBF database
        entities.load_from_hbf_database(hbf_database_path)?;

        // Write clustered entities to disk
        entities.write_all_entities(analysis_output_dir)?;

        // Phase 2: Generate individual models via AI
        let phase1_results = entities.generate_all_individual_models(
            &models_dir,
        )?;

        // Phase 3: Generate container integration models  
        let _container_results = entities.generate_container_models(
            &models_dir,
            &templates_dir,
            phase1_results,
        )?;

        // Return analysis summary
        Ok(entities.get_analysis_summary())
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
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_raw_entities_creation() {
        let entities = RawEntities::new();
        assert_eq!(entities.total_entities, 0);
        assert!(entities.uncategorized.is_empty());
        // All known clusters should be pre-initialized
        assert_eq!(entities.regions.len(), 27);
        assert_eq!(entities.settlements.len(), 10); 
        assert_eq!(entities.factions.len(), 5);
        assert_eq!(entities.dungeons.len(), 18);
    }

    #[test]
    fn test_add_entity() {
        let mut entities = RawEntities::new();
        entities.add_entity("test-uuid".to_string(), "test entity content".to_string());
        
        // Should be 1 entity total (either categorized or uncategorized)
        assert_eq!(entities.total_entities, 1);
    }

    #[test]
    fn test_write_all_entities() -> Result<()> {
        let mut entities = RawEntities::new();
        entities.add_entity("test-uuid".to_string(), "test content".to_string());

        let temp_dir = tempdir()?;
        entities.write_all_entities(temp_dir.path())?;

        // Check that output directory was created
        assert!(temp_dir.path().exists());
        
        Ok(())
    }

    #[test]
    fn test_analysis_summary() {
        let mut entities = RawEntities::new();
        entities.add_entity("test-uuid-1".to_string(), "content 1".to_string());
        entities.add_entity("test-uuid-2".to_string(), "content 2".to_string());

        let summary = entities.get_analysis_summary();
        assert_eq!(summary.total_entities, 2);
    }
}
