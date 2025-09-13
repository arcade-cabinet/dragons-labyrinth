//! Main orchestration system for HBF entity analysis
//! 
//! Simplified orchestration system for coordinating entity processing
//! and analysis pipeline for Dragon's Labyrinth.

use anyhow::Result;
use std::collections::HashMap;
use std::path::Path;
use serde::{Deserialize, Serialize};
use std::time::Instant;

use crate::containers::RawEntity;

/// Main container coordinating all entity clusters and analysis pipeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawEntities {
    /// Region entities by name
    pub regions: HashMap<String, Vec<RawEntity>>,
    /// Settlement entities by name  
    pub settlements: HashMap<String, Vec<RawEntity>>,
    /// Faction entities by name
    pub factions: HashMap<String, Vec<RawEntity>>,
    /// Dungeon entities by name
    pub dungeons: HashMap<String, Vec<RawEntity>>,
    /// Entities that don't fit known categories
    pub uncategorized: Vec<RawEntity>,
    /// Total count of all entities processed
    pub total_entities: usize,
}

impl RawEntities {
    /// Create new RawEntities container
    pub fn new() -> Self {
        Self {
            regions: HashMap::new(),
            settlements: HashMap::new(), 
            factions: HashMap::new(),
            dungeons: HashMap::new(),
            uncategorized: Vec::new(),
            total_entities: 0,
        }
    }

    /// Add an entity to the appropriate category or uncategorized list
    pub fn add_entity(&mut self, uuid: String, raw_value: String) {
        let entity = RawEntity::new(uuid, "unknown".to_string(), "unknown".to_string(), raw_value);
        self.total_entities += 1;

        // Simple categorization logic
        match self.categorize_entity(&entity) {
            Some(EntityCategory::Regions) => {
                let key = self.extract_entity_name(&entity, "regions");
                self.regions.entry(key).or_insert_with(Vec::new).push(entity);
            }
            Some(EntityCategory::Settlements) => {
                let key = self.extract_entity_name(&entity, "settlements");
                self.settlements.entry(key).or_insert_with(Vec::new).push(entity);
            }
            Some(EntityCategory::Factions) => {
                let key = self.extract_entity_name(&entity, "factions");
                self.factions.entry(key).or_insert_with(Vec::new).push(entity);
            }
            Some(EntityCategory::Dungeons) => {
                let key = self.extract_entity_name(&entity, "dungeons");
                self.dungeons.entry(key).or_insert_with(Vec::new).push(entity);
            }
            None => {
                self.uncategorized.push(entity);
            }
        }
    }

    /// Load entities from HBF SQLite database
    pub fn load_from_hbf_database<P: AsRef<Path>>(&mut self, hbf_database_path: P) -> Result<()> {
        let connection = rusqlite::Connection::open(hbf_database_path.as_ref())?;
        
        // Extract ALL entities from HBF database
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

    /// Write all clustered entities to disk for processing pipeline
    pub fn write_all_entities<P: AsRef<Path>>(&self, analysis_output_dir: P) -> Result<()> {
        use std::fs;

        let output_dir = analysis_output_dir.as_ref();
        fs::create_dir_all(output_dir)?;

        // Write each category as JSON
        if !self.regions.is_empty() {
            let regions_json = serde_json::to_string_pretty(&self.regions)?;
            fs::write(output_dir.join("regions.json"), regions_json)?;
        }

        if !self.settlements.is_empty() {
            let settlements_json = serde_json::to_string_pretty(&self.settlements)?;
            fs::write(output_dir.join("settlements.json"), settlements_json)?;
        }

        if !self.factions.is_empty() {
            let factions_json = serde_json::to_string_pretty(&self.factions)?;
            fs::write(output_dir.join("factions.json"), factions_json)?;
        }

        if !self.dungeons.is_empty() {
            let dungeons_json = serde_json::to_string_pretty(&self.dungeons)?;
            fs::write(output_dir.join("dungeons.json"), dungeons_json)?;
        }

        // Write uncategorized entities
        if !self.uncategorized.is_empty() {
            let uncategorized_json = serde_json::to_string_pretty(&self.uncategorized)?;
            fs::write(output_dir.join("uncategorized.json"), uncategorized_json)?;
        }

        Ok(())
    }

    /// Simple categorization based on content analysis
    fn categorize_entity(&self, entity: &RawEntity) -> Option<EntityCategory> {
        let content = entity.raw_value.to_lowercase();
        
        if content.contains("region") || content.contains("biome") || content.contains("terrain") {
            Some(EntityCategory::Regions)
        } else if content.contains("settlement") || content.contains("village") || content.contains("town") {
            Some(EntityCategory::Settlements)
        } else if content.contains("faction") || content.contains("guild") || content.contains("organization") {
            Some(EntityCategory::Factions)
        } else if content.contains("dungeon") || content.contains("lair") || content.contains("cave") {
            Some(EntityCategory::Dungeons)
        } else {
            None
        }
    }

    /// Extract entity name for categorization key
    fn extract_entity_name(&self, entity: &RawEntity, category: &str) -> String {
        // Simple name extraction from UUID or content
        if entity.entity_name != "unknown" && !entity.entity_name.is_empty() {
            entity.entity_name.clone()
        } else {
            // Extract from UUID or use generic name
            format!("{}_{}", category, entity.uuid.chars().take(8).collect::<String>())
        }
    }

    /// Get analysis summary
    pub fn get_analysis_summary(&self) -> AnalysisSummary {
        AnalysisSummary {
            total_entities: self.total_entities,
            regions_count: self.regions.len(),
            settlements_count: self.settlements.len(),
            factions_count: self.factions.len(),
            dungeons_count: self.dungeons.len(),
            uncategorized_count: self.uncategorized.len(),
        }
    }

    /// Main analysis entry point
    pub fn run_complete_analysis<P: AsRef<Path>>(
        hbf_database_path: P,
        analysis_output_dir: P,
    ) -> Result<AnalysisSummary> {
        let mut entities = RawEntities::new();

        // Load all entities from HBF database
        entities.load_from_hbf_database(&hbf_database_path)?;

        // Write clustered entities to disk
        entities.write_all_entities(&analysis_output_dir)?;

        // Return analysis summary
        Ok(entities.get_analysis_summary())
    }
}

impl Default for RawEntities {
    fn default() -> Self {
        Self::new()
    }
}

/// Entity categories for classification
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum EntityCategory {
    Regions,
    Settlements,
    Factions,
    Dungeons,
}

impl EntityCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            EntityCategory::Regions => "regions",
            EntityCategory::Settlements => "settlements", 
            EntityCategory::Factions => "factions",
            EntityCategory::Dungeons => "dungeons",
        }
    }
}

/// Analysis summary with key metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisSummary {
    pub total_entities: usize,
    pub regions_count: usize,
    pub settlements_count: usize,
    pub factions_count: usize,
    pub dungeons_count: usize,
    pub uncategorized_count: usize,
}

impl AnalysisSummary {
    pub fn new() -> Self {
        Self {
            total_entities: 0,
            regions_count: 0,
            settlements_count: 0,
            factions_count: 0,
            dungeons_count: 0,
            uncategorized_count: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_raw_entities_creation() {
        let entities = RawEntities::new();
        assert_eq!(entities.total_entities, 0);
        assert!(entities.uncategorized.is_empty());
    }

    #[test]
    fn test_add_entity() {
        let mut entities = RawEntities::new();
        entities.add_entity("test-uuid".to_string(), "test entity content".to_string());
        
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
