//! Progressive HBF transformation pipeline
//!
//! Orchestrates the complete transformation of HBF data through multiple stages,
//! each building on the previous to extract increasingly rich game content.

use super::{
    empty_remover::{EmptyRemover, EmptyRemovalStats},
    refs_extractor::{RefsExtractor, RefsExtractionResult},
    json_parser::{JsonParser, JsonParsingResult},
    html_parser::{HtmlParser, HtmlParsingResult},
    dungeon_parser::{DungeonParser, DungeonParsingResult},
};
use crate::models::hbf::{HbfData, HbfEntity, HbfRef};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Main transformer that orchestrates the progressive transformation pipeline
pub struct HbfTransformer {
    hbf_path: PathBuf,
    output_dir: PathBuf,
    backup_dir: PathBuf,
    current_pass: usize,
    reports: Vec<TransformReport>,
}

impl HbfTransformer {
    /// Create a new transformer for the given HBF file
    pub fn new(hbf_path: impl AsRef<Path>) -> Result<Self> {
        let hbf_path = hbf_path.as_ref().to_path_buf();
        
        // Set up output directories
        let output_dir = hbf_path.parent()
            .unwrap_or_else(|| Path::new("."))
            .join("hbf_transformed");
        
        // Use XDG-compliant backup directory
        let backup_dir = dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("dragons-labyrinth")
            .join("hbf-backups");
        
        // Create directories if they don't exist
        fs::create_dir_all(&output_dir)?;
        fs::create_dir_all(&backup_dir)?;
        
        Ok(Self {
            hbf_path,
            output_dir,
            backup_dir,
            current_pass: 0,
            reports: Vec::new(),
        })
    }

    /// Run the complete transformation pipeline
    pub fn transform(&mut self) -> Result<Vec<TransformReport>> {
        println!("Starting progressive HBF transformation...");
        
        // Load the original HBF data
        let hbf_data = self.load_hbf_data()?;
        
        // Pass 1: Remove empty entities
        let pass1_data = self.pass1_remove_empty(hbf_data)?;
        
        // Pass 2: Extract and process Refs
        let pass2_data = self.pass2_extract_refs(pass1_data)?;
        
        // Pass 3: Parse JSON map entities
        let pass3_data = self.pass3_parse_json(pass2_data)?;
        
        // Pass 4: Parse HTML content entities
        let pass4_data = self.pass4_parse_html(pass3_data)?;
        
        // Pass 5: Extract dungeon-specific content
        let pass5_data = self.pass5_parse_dungeons(pass4_data)?;
        
        // Pass 6: Generate final SeaORM models
        let _final_data = self.pass6_generate_models(pass5_data)?;
        
        println!("\nTransformation complete!");
        println!("Results saved to: {}", self.output_dir.display());
        
        Ok(self.reports.clone())
    }

    /// Load HBF data from file
    fn load_hbf_data(&self) -> Result<HbfData> {
        println!("Loading HBF data from: {}", self.hbf_path.display());
        
        let content = fs::read_to_string(&self.hbf_path)
            .with_context(|| format!("Failed to read HBF file: {}", self.hbf_path.display()))?;
        
        let data: HbfData = serde_json::from_str(&content)
            .with_context(|| "Failed to parse HBF JSON")?;
        
        println!("Loaded {} entities and {} refs", 
                 data.Entities.len(), 
                 data.Refs.as_ref().map(|r| r.len()).unwrap_or(0));
        
        Ok(data)
    }

    /// Pass 1: Remove empty placeholder entities
    fn pass1_remove_empty(&mut self, mut data: HbfData) -> Result<TransformedData> {
        self.current_pass = 1;
        println!("\n=== Pass 1: Removing empty entities ===");
        
        let mut remover = EmptyRemover::new();
        
        // Convert entities to Value for processing
        let entities: Vec<Value> = data.Entities.iter()
            .map(|e| serde_json::to_value(e).unwrap())
            .collect();
        
        let filtered = remover.process(entities);
        let stats = remover.get_stats();
        
        println!("Removed {} empty entities", stats.removed_count);
        println!("Remaining entities: {}", filtered.len());
        
        // Save checkpoint
        self.save_checkpoint(&filtered, "pass1_no_empty")?;
        
        // Create report
        let report = TransformReport {
            pass: 1,
            name: "Remove Empty Entities".to_string(),
            entities_processed: data.Entities.len(),
            entities_output: filtered.len(),
            details: format!("Removed {} empty placeholder entities", stats.removed_count),
            stats: Some(serde_json::to_value(stats)?),
        };
        self.reports.push(report);
        
        Ok(TransformedData {
            entities: filtered,
            refs: data.Refs.unwrap_or_default(),
            metadata: HashMap::new(),
        })
    }

    /// Pass 2: Extract and process Refs
    fn pass2_extract_refs(&mut self, data: TransformedData) -> Result<TransformedData> {
        self.current_pass = 2;
        println!("\n=== Pass 2: Extracting references ===");
        
        let mut extractor = RefsExtractor::new();
        
        // Convert refs to Value for processing
        let refs: Vec<Value> = data.refs.iter()
            .map(|r| serde_json::to_value(r).unwrap())
            .collect();
        
        let result = extractor.process(refs);
        
        println!("{}", result.summary());
        
        // Build relationship map
        let relationships = extractor.build_relationship_map();
        println!("Found {} entity relationships", relationships.len());
        
        // Save extracted refs
        self.save_json(&result, "pass2_refs_extracted.json")?;
        
        // Create report
        let report = TransformReport {
            pass: 2,
            name: "Extract References".to_string(),
            entities_processed: data.refs.len(),
            entities_output: result.total_refs,
            details: result.summary(),
            stats: Some(serde_json::to_value(&result)?),
        };
        self.reports.push(report);
        
        // Add refs data to metadata
        let mut metadata = data.metadata;
        metadata.insert("refs".to_string(), serde_json::to_value(result)?);
        metadata.insert("relationships".to_string(), serde_json::to_value(relationships)?);
        
        Ok(TransformedData {
            entities: data.entities,
            refs: data.refs,
            metadata,
        })
    }

    /// Pass 3: Parse JSON map entities
    fn pass3_parse_json(&mut self, data: TransformedData) -> Result<TransformedData> {
        self.current_pass = 3;
        println!("\n=== Pass 3: Parsing JSON map data ===");
        
        let mut parser = JsonParser::new();
        
        // Filter entities that likely contain JSON
        let json_entities: Vec<Value> = data.entities.iter()
            .filter(|e| self.is_json_entity(e))
            .cloned()
            .collect();
        
        println!("Found {} potential JSON entities", json_entities.len());
        
        let result = parser.process(json_entities);
        
        println!("{}", result.summary());
        
        // Save parsed maps
        self.save_json(&result, "pass3_maps_parsed.json")?;
        
        // Create report
        let report = TransformReport {
            pass: 3,
            name: "Parse JSON Maps".to_string(),
            entities_processed: json_entities.len(),
            entities_output: result.maps.len(),
            details: result.summary(),
            stats: Some(serde_json::to_value(&result)?),
        };
        self.reports.push(report);
        
        // Add map data to metadata
        let mut metadata = data.metadata;
        metadata.insert("maps".to_string(), serde_json::to_value(result)?);
        
        Ok(TransformedData {
            entities: data.entities,
            refs: data.refs,
            metadata,
        })
    }

    /// Pass 4: Parse HTML content entities
    fn pass4_parse_html(&mut self, data: TransformedData) -> Result<TransformedData> {
        self.current_pass = 4;
        println!("\n=== Pass 4: Parsing HTML content ===");
        
        let mut parser = HtmlParser::new();
        
        // Filter entities that likely contain HTML
        let html_entities: Vec<Value> = data.entities.iter()
            .filter(|e| self.is_html_entity(e))
            .cloned()
            .collect();
        
        println!("Found {} potential HTML entities", html_entities.len());
        
        let result = parser.process(html_entities);
        
        println!("{}", result.summary());
        
        // Save parsed content
        self.save_json(&result, "pass4_html_parsed.json")?;
        
        // Create report
        let report = TransformReport {
            pass: 4,
            name: "Parse HTML Content".to_string(),
            entities_processed: html_entities.len(),
            entities_output: result.settlements.len() + result.dungeons.len() + result.other_content.len(),
            details: result.summary(),
            stats: Some(serde_json::to_value(&result)?),
        };
        self.reports.push(report);
        
        // Add HTML content to metadata
        let mut metadata = data.metadata;
        metadata.insert("html_content".to_string(), serde_json::to_value(result)?);
        
        Ok(TransformedData {
            entities: data.entities,
            refs: data.refs,
            metadata,
        })
    }

    /// Pass 5: Parse dungeon-specific content
    fn pass5_parse_dungeons(&mut self, data: TransformedData) -> Result<TransformedData> {
        self.current_pass = 5;
        println!("\n=== Pass 5: Extracting dungeon data ===");
        
        let mut parser = DungeonParser::new();
        
        // Filter entities that likely contain dungeon data
        let dungeon_entities: Vec<Value> = data.entities.iter()
            .filter(|e| self.is_dungeon_entity(e))
            .cloned()
            .collect();
        
        println!("Found {} potential dungeon entities", dungeon_entities.len());
        
        let result = parser.process(dungeon_entities);
        
        println!("{}", result.summary());
        
        // Save parsed dungeons
        self.save_json(&result, "pass5_dungeons_parsed.json")?;
        
        // Create report
        let report = TransformReport {
            pass: 5,
            name: "Parse Dungeons".to_string(),
            entities_processed: dungeon_entities.len(),
            entities_output: result.dungeons.len(),
            details: result.summary(),
            stats: Some(serde_json::to_value(&result)?),
        };
        self.reports.push(report);
        
        // Add dungeon data to metadata
        let mut metadata = data.metadata;
        metadata.insert("dungeons".to_string(), serde_json::to_value(result)?);
        
        Ok(TransformedData {
            entities: data.entities,
            refs: data.refs,
            metadata,
        })
    }

    /// Pass 6: Generate SeaORM models
    fn pass6_generate_models(&mut self, data: TransformedData) -> Result<TransformedData> {
        self.current_pass = 6;
        println!("\n=== Pass 6: Generating SeaORM models ===");
        
        // Extract all parsed data from metadata
        let refs_data = data.metadata.get("refs");
        let maps_data = data.metadata.get("maps");
        let html_data = data.metadata.get("html_content");
        let dungeon_data = data.metadata.get("dungeons");
        
        // Generate model definitions
        let models = self.generate_seaorm_models(&data)?;
        
        // Save model definitions
        self.save_models(&models)?;
        
        // Create final summary report
        let report = TransformReport {
            pass: 6,
            name: "Generate SeaORM Models".to_string(),
            entities_processed: data.entities.len(),
            entities_output: models.len(),
            details: format!("Generated {} SeaORM model definitions", models.len()),
            stats: Some(serde_json::json!({
                "models_generated": models.len(),
                "refs_processed": refs_data.is_some(),
                "maps_processed": maps_data.is_some(),
                "html_processed": html_data.is_some(),
                "dungeons_processed": dungeon_data.is_some(),
            })),
        };
        self.reports.push(report);
        
        // Save final transformed data
        self.save_json(&data, "pass6_final_transformed.json")?;
        
        Ok(data)
    }

    /// Check if entity likely contains JSON data
    fn is_json_entity(&self, entity: &Value) -> bool {
        if let Some(obj) = entity.as_object() {
            // Check for JSON indicators
            if obj.contains_key("json_content") || 
               obj.contains_key("hexes") || 
               obj.contains_key("grid") ||
               obj.contains_key("map") {
                return true;
            }
            
            // Check content field for JSON
            if let Some(content) = obj.get("content").and_then(|v| v.as_str()) {
                if content.trim().starts_with('{') || content.trim().starts_with('[') {
                    return true;
                }
            }
        }
        false
    }

    /// Check if entity likely contains HTML data
    fn is_html_entity(&self, entity: &Value) -> bool {
        if let Some(obj) = entity.as_object() {
            // Check for HTML indicators
            if obj.contains_key("html") || obj.contains_key("body") {
                return true;
            }
            
            // Check content field for HTML
            if let Some(content) = obj.get("content").and_then(|v| v.as_str()) {
                if content.contains("<") && content.contains(">") {
                    return true;
                }
            }
        }
        false
    }

    /// Check if entity likely contains dungeon data
    fn is_dungeon_entity(&self, entity: &Value) -> bool {
        if let Some(obj) = entity.as_object() {
            // Check for dungeon indicators
            if obj.contains_key("rooms") || 
               obj.contains_key("dungeon_level") ||
               obj.contains_key("dungeon_name") {
                return true;
            }
            
            // Check content for dungeon keywords
            if let Some(content) = obj.get("content").and_then(|v| v.as_str()) {
                let content_lower = content.to_lowercase();
                if content_lower.contains("dungeon") || 
                   content_lower.contains("chamber") ||
                   content_lower.contains("corridor") {
                    return true;
                }
            }
        }
        false
    }

    /// Generate SeaORM model definitions
    fn generate_seaorm_models(&self, data: &TransformedData) -> Result<Vec<ModelDefinition>> {
        let mut models = Vec::new();
        
        // Generate Realm model
        models.push(ModelDefinition {
            name: "Realm".to_string(),
            table_name: "realms".to_string(),
            fields: vec![
                ("id", "i32", true),
                ("name", "String", false),
                ("description", "Option<String>", false),
                ("created_at", "DateTime", false),
            ],
        });
        
        // Generate Hex model
        models.push(ModelDefinition {
            name: "Hex".to_string(),
            table_name: "hexes".to_string(),
            fields: vec![
                ("id", "i32", true),
                ("realm_id", "i32", false),
                ("x", "i32", false),
                ("y", "i32", false),
                ("terrain_type", "String", false),
                ("features", "Json", false),
            ],
        });
        
        // Generate Settlement model
        models.push(ModelDefinition {
            name: "Settlement".to_string(),
            table_name: "settlements".to_string(),
            fields: vec![
                ("id", "i32", true),
                ("hex_id", "Option<i32>", false),
                ("name", "String", false),
                ("population", "Option<i32>", false),
                ("description", "Text", false),
            ],
        });
        
        // Generate Dungeon model
        models.push(ModelDefinition {
            name: "Dungeon".to_string(),
            table_name: "dungeons".to_string(),
            fields: vec![
                ("id", "i32", true),
                ("hex_id", "Option<i32>", false),
                ("name", "String", false),
                ("theme", "String", false),
                ("danger_level", "i32", false),
                ("treasure_value", "i32", false),
            ],
        });
        
        // Generate NPC model
        models.push(ModelDefinition {
            name: "Npc".to_string(),
            table_name: "npcs".to_string(),
            fields: vec![
                ("id", "i32", true),
                ("settlement_id", "Option<i32>", false),
                ("name", "String", false),
                ("occupation", "Option<String>", false),
                ("description", "Option<Text>", false),
            ],
        });
        
        Ok(models)
    }

    /// Save models to files
    fn save_models(&self, models: &[ModelDefinition]) -> Result<()> {
        let models_dir = self.output_dir.join("models");
        fs::create_dir_all(&models_dir)?;
        
        for model in models {
            let filename = format!("{}.rs", model.name.to_lowercase());
            let path = models_dir.join(filename);
            
            let code = model.generate_rust_code();
            fs::write(path, code)?;
        }
        
        // Generate mod.rs
        let mod_content = models.iter()
            .map(|m| format!("pub mod {};", m.name.to_lowercase()))
            .collect::<Vec<_>>()
            .join("\n");
        
        fs::write(models_dir.join("mod.rs"), mod_content)?;
        
        println!("Generated {} model files in {}", models.len(), models_dir.display());
        
        Ok(())
    }

    /// Save checkpoint data
    fn save_checkpoint(&self, data: &[Value], name: &str) -> Result<()> {
        let checkpoint_path = self.backup_dir.join(format!("{}.json", name));
        let json = serde_json::to_string_pretty(&data)?;
        fs::write(&checkpoint_path, json)?;
        println!("Checkpoint saved: {}", checkpoint_path.display());
        Ok(())
    }

    /// Save JSON data
    fn save_json<T: Serialize>(&self, data: &T, filename: &str) -> Result<()> {
        let path = self.output_dir.join(filename);
        let json = serde_json::to_string_pretty(data)?;
        fs::write(&path, json)?;
        println!("Saved: {}", path.display());
        Ok(())
    }
}

/// Intermediate transformed data structure
#[derive(Debug, Clone)]
struct TransformedData {
    entities: Vec<Value>,
    refs: Vec<HbfRef>,
    metadata: HashMap<String, Value>,
}

/// Model definition for code generation
struct ModelDefinition {
    name: String,
    table_name: String,
    fields: Vec<(&'static str, &'static str, bool)>, // (name, type, is_primary)
}

impl ModelDefinition {
    fn generate_rust_code(&self) -> String {
        let fields: Vec<String> = self.fields.iter()
            .map(|(name, rust_type, is_primary)| {
                if *is_primary {
                    format!("    #[sea_orm(primary_key)]\n    pub {}: {},", name, rust_type)
                } else {
                    format!("    pub {}: {},", name, rust_type)
                }
            })
            .collect();
        
        format!(
            r#"//! SeaORM model for {}

use sea_orm::entity::prelude::*;
use serde::{{Deserialize, Serialize}};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "{}")]
pub struct Model {{
{}
}}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {{}}

impl ActiveModelBehavior for ActiveModel {{}}
"#,
            self.name,
            self.table_name,
            fields.join("\n")
        )
    }
}

/// Report for each transformation pass
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformReport {
    pub pass: usize,
    pub name: String,
    pub entities_processed: usize,
    pub entities_output: usize,
    pub details: String,
    pub stats: Option<Value>,
}

impl TransformReport {
    /// Generate a summary of the report
    pub fn summary(&self) -> String {
        format!(
            "Pass {}: {} - Processed {} entities, output {}. {}",
            self.pass, self.name, self.entities_processed, self.entities_output, self.details
        )
    }
}
