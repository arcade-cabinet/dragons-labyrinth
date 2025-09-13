//! Organized JSON data pools for runtime AI analysis
//! 
//! This module manages categorized data pools that are generated at build time
//! and consumed by the runtime analysis engine for dynamic seed generation.

use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;
use std::path::Path;
use std::fs;

use crate::books::{WorldSeed, QuestSeed, DialogueSeed};

/// Organized data pools by category for runtime consumption
#[derive(Debug, Clone)]
pub struct CategorizedDataPools {
    pub regions: Vec<Value>,
    pub settlements: Vec<Value>, 
    pub dungeons: Vec<Value>,
    pub factions: Vec<Value>,
    pub books: Vec<Value>,
    pub metadata: HashMap<String, PoolMetadata>,
}

impl CategorizedDataPools {
    /// Create new empty categorized pools
    pub fn new() -> Self {
        Self {
            regions: Vec::new(),
            settlements: Vec::new(),
            dungeons: Vec::new(),
            factions: Vec::new(),
            books: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Create categorized pools from seed data
    pub fn from_seeds(
        world_seeds: &[WorldSeed],
        quest_seeds: &[QuestSeed], 
        dialogue_seeds: &[DialogueSeed],
    ) -> Self {
        let mut pools = Self::new();
        
        // Convert world seeds to regions pool
        for seed in world_seeds {
            let region_data = serde_json::json!({
                "id": seed.source_text.chars().take(16).collect::<String>(),
                "biome_type": seed.biome_hint.as_ref().unwrap_or(&"Unknown".to_string()),
                "corruption_level": seed.corruption_band,
                "theme_keywords": seed.theme_keywords,
                "environmental_description": seed.source_text,
                "world_seed_id": format!("world-{}", seed.corruption_band),
            });
            pools.regions.push(region_data);
        }
        
        // Convert quest seeds to settlements and factions pools
        for seed in quest_seeds {
            let settlement_data = serde_json::json!({
                "id": seed.quest_type.chars().take(16).collect::<String>(),
                "quest_type": seed.quest_type,
                "corruption_band": seed.corruption_band,
                "settlement_theme": seed.theme_elements.get(0).unwrap_or(&"village".to_string()),
                "description": seed.source_text,
                "quest_seed_id": format!("quest-{}", seed.corruption_band),
            });
            pools.settlements.push(settlement_data);
        }
        
        // Convert dialogue seeds to books and factions pools
        for seed in dialogue_seeds {
            let book_data = serde_json::json!({
                "id": seed.character_archetype.chars().take(16).collect::<String>(),
                "character_type": seed.character_archetype,
                "corruption_band": seed.corruption_band,
                "dialogue_theme": seed.emotional_tone,
                "source_content": seed.source_text,
                "dialogue_seed_id": format!("dialogue-{}", seed.corruption_band),
            });
            pools.books.push(book_data);
        }
        
        // Set metadata for each category
        pools.set_metadata("regions", world_seeds.len(), "World environmental data");
        pools.set_metadata("settlements", quest_seeds.len(), "Settlement and quest data");
        pools.set_metadata("dungeons", 0, "Dungeon data (to be implemented)");
        pools.set_metadata("factions", 0, "Faction data (to be implemented)"); 
        pools.set_metadata("books", dialogue_seeds.len(), "Literature and dialogue data");
        
        pools
    }

    /// Load categorized pools from directory
    pub fn load_from_dir(dir: &Path) -> Result<Self> {
        let mut pools = Self::new();
        
        // Load each category if files exist
        if let Ok(content) = fs::read_to_string(dir.join("regions.json")) {
            pools.regions = serde_json::from_str(&content)?;
        }
        
        if let Ok(content) = fs::read_to_string(dir.join("settlements.json")) {
            pools.settlements = serde_json::from_str(&content)?;
        }
        
        if let Ok(content) = fs::read_to_string(dir.join("dungeons.json")) {
            pools.dungeons = serde_json::from_str(&content)?;
        }
        
        if let Ok(content) = fs::read_to_string(dir.join("factions.json")) {
            pools.factions = serde_json::from_str(&content)?;
        }
        
        if let Ok(content) = fs::read_to_string(dir.join("books.json")) {
            pools.books = serde_json::from_str(&content)?;
        }
        
        // Load metadata
        if let Ok(content) = fs::read_to_string(dir.join("metadata.json")) {
            pools.metadata = serde_json::from_str(&content)?;
        }
        
        Ok(pools)
    }

    /// Save categorized pools to directory
    pub fn save_to_dir(&self, dir: &Path) -> Result<()> {
        fs::create_dir_all(dir)?;
        
        // Save each category
        fs::write(
            dir.join("regions.json"),
            serde_json::to_string_pretty(&self.regions)?,
        )?;
        
        fs::write(
            dir.join("settlements.json"),
            serde_json::to_string_pretty(&self.settlements)?,
        )?;
        
        fs::write(
            dir.join("dungeons.json"),
            serde_json::to_string_pretty(&self.dungeons)?,
        )?;
        
        fs::write(
            dir.join("factions.json"),
            serde_json::to_string_pretty(&self.factions)?,
        )?;
        
        fs::write(
            dir.join("books.json"),
            serde_json::to_string_pretty(&self.books)?,
        )?;
        
        // Save metadata
        fs::write(
            dir.join("metadata.json"),
            serde_json::to_string_pretty(&self.metadata)?,
        )?;
        
        Ok(())
    }

    /// Get data for a specific category
    pub fn get_category_data(&self, category: &str) -> Result<&Vec<Value>> {
        match category {
            "regions" => Ok(&self.regions),
            "settlements" => Ok(&self.settlements),
            "dungeons" => Ok(&self.dungeons),
            "factions" => Ok(&self.factions),
            "books" => Ok(&self.books),
            _ => Err(anyhow::anyhow!("Unknown category: {}", category)),
        }
    }

    /// Get mutable reference to category data
    pub fn get_category_data_mut(&mut self, category: &str) -> Result<&mut Vec<Value>> {
        match category {
            "regions" => Ok(&mut self.regions),
            "settlements" => Ok(&mut self.settlements),
            "dungeons" => Ok(&mut self.dungeons),
            "factions" => Ok(&mut self.factions),
            "books" => Ok(&mut self.books),
            _ => Err(anyhow::anyhow!("Unknown category: {}", category)),
        }
    }

    /// Add data to a specific category
    pub fn add_to_category(&mut self, category: &str, data: Value) -> Result<()> {
        let category_data = self.get_category_data_mut(category)?;
        category_data.push(data);
        Ok(())
    }

    /// Set metadata for a category
    pub fn set_metadata(&mut self, category: &str, count: usize, description: &str) {
        let metadata = PoolMetadata {
            count,
            description: description.to_string(),
            last_updated: chrono::Utc::now(),
        };
        self.metadata.insert(category.to_string(), metadata);
    }

    /// Get metadata for a category
    pub fn get_metadata(&self, category: &str) -> Option<&PoolMetadata> {
        self.metadata.get(category)
    }

    /// Get all category names
    pub fn get_categories(&self) -> Vec<String> {
        vec![
            "regions".to_string(),
            "settlements".to_string(),
            "dungeons".to_string(),
            "factions".to_string(),
            "books".to_string(),
        ]
    }

    /// Get total count of all data across categories
    pub fn total_count(&self) -> usize {
        self.regions.len() 
        + self.settlements.len()
        + self.dungeons.len()
        + self.factions.len()
        + self.books.len()
    }

    /// Check if pools are empty
    pub fn is_empty(&self) -> bool {
        self.total_count() == 0
    }

    /// Merge another pool into this one
    pub fn merge(&mut self, other: CategorizedDataPools) {
        self.regions.extend(other.regions);
        self.settlements.extend(other.settlements);
        self.dungeons.extend(other.dungeons);
        self.factions.extend(other.factions);
        self.books.extend(other.books);
        
        // Merge metadata
        for (category, metadata) in other.metadata {
            self.metadata.insert(category, metadata);
        }
    }

    /// Filter pools by corruption band
    pub fn filter_by_corruption_band(&self, band: u8) -> Self {
        let mut filtered = Self::new();
        
        // Filter regions
        for region in &self.regions {
            if let Some(corruption_level) = region.get("corruption_level").and_then(|v| v.as_u64()) {
                if corruption_level == band as u64 {
                    filtered.regions.push(region.clone());
                }
            }
        }
        
        // Filter settlements
        for settlement in &self.settlements {
            if let Some(corruption_band) = settlement.get("corruption_band").and_then(|v| v.as_u64()) {
                if corruption_band == band as u64 {
                    filtered.settlements.push(settlement.clone());
                }
            }
        }
        
        // Filter books
        for book in &self.books {
            if let Some(corruption_band) = book.get("corruption_band").and_then(|v| v.as_u64()) {
                if corruption_band == band as u64 {
                    filtered.books.push(book.clone());
                }
            }
        }
        
        // Copy other categories as-is for now (dungeons, factions)
        filtered.dungeons = self.dungeons.clone();
        filtered.factions = self.factions.clone();
        
        filtered
    }
}

impl Default for CategorizedDataPools {
    fn default() -> Self {
        Self::new()
    }
}

/// Metadata about a data pool category
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PoolMetadata {
    pub count: usize,
    pub description: String,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_categorized_pools_creation() {
        let pools = CategorizedDataPools::new();
        assert!(pools.is_empty());
        assert_eq!(pools.total_count(), 0);
        assert_eq!(pools.get_categories().len(), 5);
    }

    #[test]
    fn test_add_to_category() {
        let mut pools = CategorizedDataPools::new();
        let test_data = serde_json::json!({"id": "test", "name": "Test Region"});
        
        pools.add_to_category("regions", test_data.clone()).unwrap();
        assert_eq!(pools.regions.len(), 1);
        assert_eq!(pools.total_count(), 1);
        
        let retrieved = pools.get_category_data("regions").unwrap();
        assert_eq!(retrieved[0], test_data);
    }

    #[test]
    fn test_metadata_operations() {
        let mut pools = CategorizedDataPools::new();
        pools.set_metadata("regions", 5, "Test regions");
        
        let metadata = pools.get_metadata("regions").unwrap();
        assert_eq!(metadata.count, 5);
        assert_eq!(metadata.description, "Test regions");
    }

    #[test]
    fn test_save_and_load() {
        let temp_dir = TempDir::new().unwrap();
        let mut pools = CategorizedDataPools::new();
        
        // Add test data
        pools.add_to_category("regions", serde_json::json!({"id": "test-region"})).unwrap();
        pools.set_metadata("regions", 1, "Test data");
        
        // Save
        pools.save_to_dir(temp_dir.path()).unwrap();
        
        // Load
        let loaded_pools = CategorizedDataPools::load_from_dir(temp_dir.path()).unwrap();
        assert_eq!(loaded_pools.regions.len(), 1);
        assert_eq!(loaded_pools.get_metadata("regions").unwrap().count, 1);
    }

    #[test]
    fn test_filter_by_corruption_band() {
        let mut pools = CategorizedDataPools::new();
        
        // Add data with different corruption bands
        pools.add_to_category("regions", serde_json::json!({
            "id": "region1",
            "corruption_level": 1
        })).unwrap();
        
        pools.add_to_category("regions", serde_json::json!({
            "id": "region2", 
            "corruption_level": 2
        })).unwrap();
        
        let filtered = pools.filter_by_corruption_band(1);
        assert_eq!(filtered.regions.len(), 1);
        assert_eq!(filtered.regions[0]["id"], "region1");
    }
}
