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

/// Training data for entity categorization
#[derive(Debug, Clone, Deserialize)]
pub struct TrainingData {
    pub category: TrainingCategory,
    pub examples: Vec<TrainingExample>,
    pub patterns: TrainingPatterns,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TrainingCategory {
    pub name: String,
    pub subcategory: String,
    pub description: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TrainingExample {
    pub name: String,
    pub content_patterns: Vec<String>,
    pub markers: Vec<String>,
    pub corruption_band: u8,
    pub horror_theme: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TrainingPatterns {
    pub positive_indicators: Vec<String>,
    pub negative_indicators: Vec<String>,
}

/// Training data repository for enhanced categorization
#[derive(Debug, Clone, Default)]
pub struct TrainingRepository {
    pub characters: Vec<TrainingData>,
    pub creatures: Vec<TrainingData>,
    pub items: Vec<TrainingData>,
    pub spells: Vec<TrainingData>,
    pub locations: Vec<TrainingData>,
    pub mechanics: Vec<TrainingData>,
}

impl TrainingRepository {
    /// Load all training data from TOML files
    pub fn load_from_directory<P: AsRef<Path>>(training_dir: P) -> Result<Self> {
        let mut repo = Self::default();
        
        let base_path = training_dir.as_ref();
        
        // Load character training data
        if let Ok(npc_content) = std::fs::read_to_string(base_path.join("characters/npcs.toml")) {
            if let Ok(training) = toml::from_str::<TrainingData>(&npc_content) {
                repo.characters.push(training);
            }
        }
        
        // Load creature training data
        if let Ok(monster_content) = std::fs::read_to_string(base_path.join("creatures/monsters.toml")) {
            if let Ok(training) = toml::from_str::<TrainingData>(&monster_content) {
                repo.creatures.push(training);
            }
        }
        
        // Load item training data
        if let Ok(treasure_content) = std::fs::read_to_string(base_path.join("items/treasure.toml")) {
            if let Ok(training) = toml::from_str::<TrainingData>(&treasure_content) {
                repo.items.push(training);
            }
        }
        
        // Load spell training data
        if let Ok(magic_content) = std::fs::read_to_string(base_path.join("spells/magic_systems.toml")) {
            if let Ok(training) = toml::from_str::<TrainingData>(&magic_content) {
                repo.spells.push(training);
            }
        }
        
        // Load location training data (all subcategories)
        let location_files = ["dungeons.toml", "regions.toml", "settlements.toml", "factions.toml"];
        for location_file in location_files {
            if let Ok(location_content) = std::fs::read_to_string(base_path.join("locations").join(location_file)) {
                if let Ok(training) = toml::from_str::<TrainingData>(&location_content) {
                    repo.locations.push(training);
                }
            }
        }
        
        // Load mechanics training data
        if let Ok(dice_content) = std::fs::read_to_string(base_path.join("mechanics/dice_rules.toml")) {
            if let Ok(training) = toml::from_str::<TrainingData>(&dice_content) {
                repo.mechanics.push(training);
            }
        }
        
        Ok(repo)
    }
    
    /// Get total training examples loaded
    pub fn total_examples(&self) -> usize {
        self.characters.iter().map(|t| t.examples.len()).sum::<usize>() +
        self.creatures.iter().map(|t| t.examples.len()).sum::<usize>() +
        self.items.iter().map(|t| t.examples.len()).sum::<usize>() +
        self.spells.iter().map(|t| t.examples.len()).sum::<usize>() +
        self.locations.iter().map(|t| t.examples.len()).sum::<usize>() +
        self.mechanics.iter().map(|t| t.examples.len()).sum::<usize>()
    }
}

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
    /// Character entities by name (NPCs, named individuals)
    pub characters: HashMap<String, Vec<RawEntity>>,
    /// Creature entities by name (monsters, beasts)
    pub creatures: HashMap<String, Vec<RawEntity>>,
    /// Item entities by name (equipment, treasure)
    pub items: HashMap<String, Vec<RawEntity>>,
    /// Spell entities by name (magic, enchantments)
    pub spells: HashMap<String, Vec<RawEntity>>,
    /// Mechanics entities by name (dice, rules, systems)
    pub mechanics: HashMap<String, Vec<RawEntity>>,
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
            characters: HashMap::new(),
            creatures: HashMap::new(),
            items: HashMap::new(),
            spells: HashMap::new(),
            mechanics: HashMap::new(),
            uncategorized: Vec::new(),
            total_entities: 0,
        }
    }

    /// Create new RawEntities with training data enhancement
    pub fn new_with_training<P: AsRef<Path>>(training_dir: P) -> Result<(Self, TrainingRepository)> {
        let training_repo = TrainingRepository::load_from_directory(training_dir)?;
        let entities = Self::new();
        Ok((entities, training_repo))
    }

    /// Enhanced categorization using training data patterns
    fn categorize_entity_with_training(&self, entity: &RawEntity, training: &TrainingRepository) -> Option<EntityCategory> {
        let content = entity.raw_value.to_lowercase();
        let name = entity.entity_name.to_lowercase();
        
        // Check training patterns for each category
        
        // Characters (NPCs)
        for char_training in &training.characters {
            for example in &char_training.examples {
                if name.contains(&example.name.to_lowercase()) ||
                   self.matches_training_patterns(&content, &name, &example.markers) {
                    return Some(EntityCategory::Characters);
                }
            }
            if self.matches_positive_indicators(&content, &name, &char_training.patterns.positive_indicators) {
                return Some(EntityCategory::Characters);
            }
        }
        
        // Creatures  
        for creature_training in &training.creatures {
            for example in &creature_training.examples {
                if name.contains(&example.name.to_lowercase()) ||
                   self.matches_training_patterns(&content, &name, &example.markers) {
                    return Some(EntityCategory::Creatures);
                }
            }
            if self.matches_positive_indicators(&content, &name, &creature_training.patterns.positive_indicators) {
                return Some(EntityCategory::Creatures);
            }
        }
        
        // Items
        for item_training in &training.items {
            for example in &item_training.examples {
                if name.contains(&example.name.to_lowercase()) ||
                   self.matches_training_patterns(&content, &name, &example.markers) {
                    return Some(EntityCategory::Items);
                }
            }
            if self.matches_positive_indicators(&content, &name, &item_training.patterns.positive_indicators) {
                return Some(EntityCategory::Items);
            }
        }
        
        // Spells
        for spell_training in &training.spells {
            for example in &spell_training.examples {
                if name.contains(&example.name.to_lowercase()) ||
                   self.matches_training_patterns(&content, &name, &example.markers) {
                    return Some(EntityCategory::Spells);
                }
            }
            if self.matches_positive_indicators(&content, &name, &spell_training.patterns.positive_indicators) {
                return Some(EntityCategory::Spells);
            }
        }
        
        // Mechanics
        for mech_training in &training.mechanics {
            for example in &mech_training.examples {
                if name.contains(&example.name.to_lowercase()) ||
                   self.matches_training_patterns(&content, &name, &example.markers) {
                    return Some(EntityCategory::Mechanics);
                }
            }
            if self.matches_positive_indicators(&content, &name, &mech_training.patterns.positive_indicators) {
                return Some(EntityCategory::Mechanics);
            }
        }
        
        // Fall back to original categorization
        self.categorize_entity(entity)
    }
    
    /// Check if content matches training patterns
    fn matches_training_patterns(&self, content: &str, name: &str, markers: &[String]) -> bool {
        for marker in markers {
            let marker_lower = marker.to_lowercase();
            if content.contains(&marker_lower) || name.contains(&marker_lower) {
                return true;
            }
        }
        false
    }
    
    /// Check if content matches positive indicators
    fn matches_positive_indicators(&self, content: &str, name: &str, indicators: &[String]) -> bool {
        for indicator in indicators {
            let indicator_lower = indicator.to_lowercase();
            if content.contains(&indicator_lower) || name.contains(&indicator_lower) {
                return true;
            }
        }
        false
    }

    /// Add an entity to the appropriate category or uncategorized list
    pub fn add_entity(&mut self, uuid: String, raw_value: String) {
        // Extract meaningful entity name and category from content
        let (category, entity_name) = self.extract_category_and_name(&raw_value);
        let entity = RawEntity::new(uuid, category.clone(), entity_name, raw_value);
        self.total_entities += 1;

        // Categorize based on extracted information
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
            Some(EntityCategory::Characters) => {
                let key = self.extract_entity_name(&entity, "characters");
                self.characters.entry(key).or_insert_with(Vec::new).push(entity);
            }
            Some(EntityCategory::Creatures) => {
                let key = self.extract_entity_name(&entity, "creatures");
                self.creatures.entry(key).or_insert_with(Vec::new).push(entity);
            }
            Some(EntityCategory::Items) => {
                let key = self.extract_entity_name(&entity, "items");
                self.items.entry(key).or_insert_with(Vec::new).push(entity);
            }
            Some(EntityCategory::Spells) => {
                let key = self.extract_entity_name(&entity, "spells");
                self.spells.entry(key).or_insert_with(Vec::new).push(entity);
            }
            Some(EntityCategory::Mechanics) => {
                let key = self.extract_entity_name(&entity, "mechanics");
                self.mechanics.entry(key).or_insert_with(Vec::new).push(entity);
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

    /// Load entities from HBF SQLite database with enhanced training-based categorization
    pub fn load_from_hbf_database_with_training<P: AsRef<Path>>(
        &mut self, 
        hbf_database_path: P, 
        training: &TrainingRepository
    ) -> Result<()> {
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
            self.add_entity_with_training(uuid, value, training);
        }

        Ok(())
    }

    /// Add entity using enhanced training-based categorization
    pub fn add_entity_with_training(&mut self, uuid: String, raw_value: String, training: &TrainingRepository) {
        // Extract meaningful entity name and category from content
        let (category, entity_name) = self.extract_category_and_name(&raw_value);
        let entity = RawEntity::new(uuid, category.clone(), entity_name, raw_value);
        self.total_entities += 1;

        // Use enhanced categorization with training data
        match self.categorize_entity_with_training(&entity, training) {
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
            Some(EntityCategory::Characters) => {
                let key = self.extract_entity_name(&entity, "characters");
                self.characters.entry(key).or_insert_with(Vec::new).push(entity);
            }
            Some(EntityCategory::Creatures) => {
                let key = self.extract_entity_name(&entity, "creatures");
                self.creatures.entry(key).or_insert_with(Vec::new).push(entity);
            }
            Some(EntityCategory::Items) => {
                let key = self.extract_entity_name(&entity, "items");
                self.items.entry(key).or_insert_with(Vec::new).push(entity);
            }
            Some(EntityCategory::Spells) => {
                let key = self.extract_entity_name(&entity, "spells");
                self.spells.entry(key).or_insert_with(Vec::new).push(entity);
            }
            Some(EntityCategory::Mechanics) => {
                let key = self.extract_entity_name(&entity, "mechanics");
                self.mechanics.entry(key).or_insert_with(Vec::new).push(entity);
            }
            None => {
                self.uncategorized.push(entity);
            }
        }
    }

    /// Write all clustered entities to disk for processing pipeline
    pub fn write_all_entities<P: AsRef<Path>>(&self, analysis_output_dir: P) -> Result<()> {
        use std::fs;

        let output_dir = analysis_output_dir.as_ref();
        fs::create_dir_all(output_dir)?;

        // Write each category as JSON (including new categories)
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

        // Write new categories
        if !self.characters.is_empty() {
            let characters_json = serde_json::to_string_pretty(&self.characters)?;
            fs::write(output_dir.join("characters.json"), characters_json)?;
        }

        if !self.creatures.is_empty() {
            let creatures_json = serde_json::to_string_pretty(&self.creatures)?;
            fs::write(output_dir.join("creatures.json"), creatures_json)?;
        }

        if !self.items.is_empty() {
            let items_json = serde_json::to_string_pretty(&self.items)?;
            fs::write(output_dir.join("items.json"), items_json)?;
        }

        if !self.spells.is_empty() {
            let spells_json = serde_json::to_string_pretty(&self.spells)?;
            fs::write(output_dir.join("spells.json"), spells_json)?;
        }

        if !self.mechanics.is_empty() {
            let mechanics_json = serde_json::to_string_pretty(&self.mechanics)?;
            fs::write(output_dir.join("mechanics.json"), mechanics_json)?;
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

    /// Extract category and meaningful name from raw HTML content
    fn extract_category_and_name(&self, raw_value: &str) -> (String, String) {
        let content_lower = raw_value.to_lowercase();
        
        // Extract entity name from HTML title tags or content
        let entity_name = self.extract_entity_name_from_html(raw_value);
        
        // Determine category based on content patterns  
        let category = if content_lower.contains("village") || content_lower.contains("town") || 
                         content_lower.contains("city") || content_lower.contains("settlement") ||
                         content_lower.contains("hamlet") || content_lower.contains("outpost") {
            "settlements"
        } else if content_lower.contains("guild") || content_lower.contains("organization") ||
                  content_lower.contains("cult") || content_lower.contains("order") ||
                  content_lower.contains("covenant") || content_lower.contains("brotherhood") ||
                  content_lower.contains("faction") || content_lower.contains("company") {
            "factions"
        } else if content_lower.contains("cave") || content_lower.contains("lair") || 
                  content_lower.contains("crypt") || content_lower.contains("tomb") ||
                  content_lower.contains("temple") || content_lower.contains("shrine") ||
                  content_lower.contains("hideout") || content_lower.contains("cavern") ||
                  content_lower.contains("dungeon") || content_lower.contains("ruins") {
            "dungeons"
        } else if content_lower.contains("forest") || content_lower.contains("mountain") || 
                  content_lower.contains("biome") || content_lower.contains("region") ||
                  content_lower.contains("wilderness") || content_lower.contains("plains") ||
                  content_lower.contains("hills") || content_lower.contains("valley") ||
                  content_lower.contains("desert") || content_lower.contains("swamp") {
            "regions"
        } else if content_lower.contains("npc") || content_lower.contains("character") ||
                  content_lower.contains("person") || content_lower.contains("individual") ||
                  content_lower.contains("merchant") || content_lower.contains("guard") {
            "characters"
        } else if content_lower.contains("item") || content_lower.contains("weapon") ||
                  content_lower.contains("armor") || content_lower.contains("equipment") ||
                  content_lower.contains("tool") || content_lower.contains("artifact") {
            "items"
        } else if content_lower.contains("spell") || content_lower.contains("magic") ||
                  content_lower.contains("enchantment") || content_lower.contains("ritual") ||
                  content_lower.contains("incantation") || content_lower.contains("cantrip") {
            "spells"
        } else if content_lower.contains("creature") || content_lower.contains("monster") ||
                  content_lower.contains("beast") || content_lower.contains("dragon") ||
                  content_lower.contains("goblin") || content_lower.contains("orc") {
            "creatures"
        } else {
            "unknown"
        };
        
        (category.to_string(), entity_name)
    }
    
    /// Extract meaningful entity name from HTML content
    fn extract_entity_name_from_html(&self, raw_value: &str) -> String {
        // Try to extract from common HTML patterns
        if let Some(title_start) = raw_value.find("<title>") {
            if let Some(title_end) = raw_value[title_start..].find("</title>") {
                let title = &raw_value[title_start + 7..title_start + title_end];
                if !title.trim().is_empty() {
                    return title.trim().to_string();
                }
            }
        }
        
        // Try to extract from h1, h2, h3 tags
        for tag in ["<h1>", "<h2>", "<h3>"] {
            if let Some(start) = raw_value.find(tag) {
                let end_tag = tag.replace('<', "</");
                if let Some(end) = raw_value[start..].find(&end_tag) {
                    let header = &raw_value[start + tag.len()..start + end];
                    if !header.trim().is_empty() {
                        return header.trim().to_string();
                    }
                }
            }
        }
        
        // Try to extract from <b> or <strong> tags (often used for names)
        for tag in ["<b>", "<strong>"] {
            if let Some(start) = raw_value.find(tag) {
                let end_tag = tag.replace('<', "</");
                if let Some(end) = raw_value[start..].find(&end_tag) {
                    let bold_text = &raw_value[start + tag.len()..start + end];
                    if !bold_text.trim().is_empty() && bold_text.len() < 100 {
                        return bold_text.trim().to_string();
                    }
                }
            }
        }
        
        // Extract first meaningful text from content (fallback)
        let cleaned = raw_value
            .replace("<p>", " ")
            .replace("</p>", " ")
            .replace("<div>", " ")
            .replace("</div>", " ")
            .replace("<br>", " ")
            .replace("&nbsp;", " ");
            
        // Get first meaningful word sequence
        let words: Vec<&str> = cleaned.split_whitespace().take(3).collect();
        if !words.is_empty() {
            words.join(" ")
        } else {
            "unknown".to_string()
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
            characters_count: self.characters.len(),
            creatures_count: self.creatures.len(),
            items_count: self.items.len(),
            spells_count: self.spells.len(),
            mechanics_count: self.mechanics.len(),
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
    Characters,
    Creatures,
    Items,
    Spells,
    Mechanics,
}

impl EntityCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            EntityCategory::Regions => "regions",
            EntityCategory::Settlements => "settlements", 
            EntityCategory::Factions => "factions",
            EntityCategory::Dungeons => "dungeons",
            EntityCategory::Characters => "characters",
            EntityCategory::Creatures => "creatures", 
            EntityCategory::Items => "items",
            EntityCategory::Spells => "spells",
            EntityCategory::Mechanics => "mechanics",
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
    pub characters_count: usize,
    pub creatures_count: usize,
    pub items_count: usize,
    pub spells_count: usize,
    pub mechanics_count: usize,
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
            characters_count: 0,
            creatures_count: 0,
            items_count: 0,
            spells_count: 0,
            mechanics_count: 0,
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
