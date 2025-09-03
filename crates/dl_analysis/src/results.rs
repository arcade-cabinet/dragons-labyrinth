//! Result types for analysis and generation operations.
//! 
//! Mirrors the Python results.py with modern Rust types.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use dl_types::analysis::base::EdgeType;

/// Connection information for container integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConnections {
    /// UUID field names found in the model
    pub uuid_fields: Vec<String>,
    /// Fields that connect to other entities
    pub connection_fields: Vec<String>,
    /// Absolute import path for the model
    pub import_path: String,
    /// Class names exported by the model
    pub exported_classes: Vec<String>,
}

impl ModelConnections {
    pub fn new(import_path: String) -> Self {
        Self {
            uuid_fields: Vec::new(),
            connection_fields: Vec::new(),
            import_path,
            exported_classes: Vec::new(),
        }
    }

    pub fn add_uuid_field(mut self, field_name: String) -> Self {
        self.uuid_fields.push(field_name);
        self
    }

    pub fn add_connection_field(mut self, field_name: String) -> Self {
        self.connection_fields.push(field_name);
        self
    }

    pub fn add_exported_class(mut self, class_name: String) -> Self {
        self.exported_classes.push(class_name);
        self
    }
}

/// Entity collections organized by type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityCollections {
    pub regions: Vec<dl_types::analysis::entities::RegionHexTile>,
    pub settlements: Vec<dl_types::analysis::entities::SettlementEstablishment>,
    pub factions: Vec<dl_types::analysis::entities::FactionEntity>,
    pub dungeons: Vec<dl_types::analysis::entities::RegionHexTile>, // Dungeons are also hex tiles for now
}

impl EntityCollections {
    pub fn new() -> Self {
        Self {
            regions: Vec::new(),
            settlements: Vec::new(),
            factions: Vec::new(),
            dungeons: Vec::new(),
        }
    }
}

/// Results from AI model generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationResults {
    /// Generated file paths
    pub models_generated: Vec<String>,
    /// Generation details and notes
    pub analysis_notes: Vec<String>,
    /// UUID connection information if successful
    pub connections: Option<ModelConnections>,
    /// Success status
    pub success: bool,
    /// Organized entity collections
    pub entities: EntityCollections,
    /// Analysis summary information
    pub summary: AnalysisSummary,
}

impl GenerationResults {
    pub fn success(models_generated: Vec<String>) -> Self {
        Self {
            models_generated,
            analysis_notes: Vec::new(),
            connections: None,
            success: true,
            entities: EntityCollections::new(),
            summary: AnalysisSummary::new(),
        }
    }

    pub fn failure(error_message: String) -> Self {
        Self {
            models_generated: Vec::new(),
            analysis_notes: vec![error_message],
            connections: None,
            success: false,
            entities: EntityCollections::new(),
            summary: AnalysisSummary::new(),
        }
    }

    pub fn with_connections(mut self, connections: ModelConnections) -> Self {
        self.connections = Some(connections);
        self
    }

    pub fn add_note(mut self, note: String) -> Self {
        self.analysis_notes.push(note);
        self
    }

    pub fn with_entities(mut self, entities: EntityCollections) -> Self {
        self.entities = entities;
        self
    }

    pub fn with_summary(mut self, summary: AnalysisSummary) -> Self {
        self.summary = summary;
        self
    }
}

/// Summary of complete analysis run
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisSummary {
    /// Total entities processed
    pub total_entities: usize,
    /// Entities by category
    pub entity_counts: HashMap<String, HashMap<String, usize>>,
    /// Generation results by phase
    pub generation_results: HashMap<String, GenerationResults>,
    /// Uncategorized entity count
    pub uncategorized_count: usize,
    /// Success status
    pub success: bool,
    /// Summary notes
    pub notes: Vec<String>,
}

impl AnalysisSummary {
    pub fn new() -> Self {
        Self {
            total_entities: 0,
            entity_counts: HashMap::new(),
            generation_results: HashMap::new(),
            uncategorized_count: 0,
            success: true,
            notes: Vec::new(),
        }
    }

    pub fn set_entity_counts(mut self, counts: HashMap<String, HashMap<String, usize>>) -> Self {
        // Calculate total before moving counts
        self.total_entities = counts.values()
            .flat_map(|category| category.values())
            .sum();
        self.entity_counts = counts;
        self
    }

    pub fn set_uncategorized_count(mut self, count: usize) -> Self {
        self.uncategorized_count = count;
        self.total_entities += count;
        self
    }

    pub fn add_generation_result(mut self, phase: String, result: GenerationResults) -> Self {
        if !result.success {
            self.success = false;
        }
        self.generation_results.insert(phase, result);
        self
    }

    pub fn add_note(mut self, note: String) -> Self {
        self.notes.push(note);
        self
    }

    /// Get a human-readable summary string
    pub fn summary_text(&self) -> String {
        let mut lines = vec![
            "=== ANALYSIS SUMMARY ===".to_string(),
            "".to_string(),
        ];

        // Entity counts by category
        for (category, entities) in &self.entity_counts {
            lines.push(format!("{}:", category.to_uppercase()));
            for (entity_name, count) in entities {
                lines.push(format!("  {}: {} entities", entity_name, count));
            }
            let total: usize = entities.values().sum();
            lines.push(format!("  TOTAL {}: {} entities", category, total));
            
            // Check if we have generation results for this category
            if let Some(result) = self.generation_results.get(category) {
                if result.success {
                    lines.push(format!("  ✓ AI models generated: {} files", result.models_generated.len()));
                } else {
                    lines.push("  ✗ AI model generation failed".to_string());
                }
            }
            lines.push("".to_string());
        }

        if self.uncategorized_count > 0 {
            lines.push(format!("UNCATEGORIZED: {} entities", self.uncategorized_count));
            lines.push("".to_string());
        }

        lines.push(format!("Total entities processed: {}", self.total_entities));
        
        if self.success {
            lines.push("Analysis completed successfully!".to_string());
        } else {
            lines.push("Analysis completed with errors.".to_string());
        }

        // Add notes
        if !self.notes.is_empty() {
            lines.push("".to_string());
            lines.push("Notes:".to_string());
            for note in &self.notes {
                lines.push(format!("- {}", note));
            }
        }

        lines.join("\n")
    }
}

impl Default for AnalysisSummary {
    fn default() -> Self {
        Self::new()
    }
}

/// Entity edge information for relationship tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityEdge {
    pub source_uuid: String,
    pub target_uuid: String,
    pub edge_type: EdgeType,
    pub field_name: String,
}

impl EntityEdge {
    pub fn new(source_uuid: String, target_uuid: String, edge_type: EdgeType, field_name: String) -> Self {
        Self {
            source_uuid,
            target_uuid,
            edge_type,
            field_name,
        }
    }
}

/// Collection of entity edges for relationship mapping
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityGraph {
    pub edges: Vec<EntityEdge>,
}

impl EntityGraph {
    pub fn new() -> Self {
        Self {
            edges: Vec::new(),
        }
    }

    pub fn add_edge(mut self, edge: EntityEdge) -> Self {
        self.edges.push(edge);
        self
    }

    /// Get all edges of a specific type
    pub fn edges_of_type(&self, edge_type: EdgeType) -> Vec<&EntityEdge> {
        self.edges.iter().filter(|e| e.edge_type == edge_type).collect()
    }

    /// Get all edges from a source entity
    pub fn edges_from(&self, source_uuid: &str) -> Vec<&EntityEdge> {
        self.edges.iter().filter(|e| e.source_uuid == source_uuid).collect()
    }

    /// Get all edges to a target entity
    pub fn edges_to(&self, target_uuid: &str) -> Vec<&EntityEdge> {
        self.edges.iter().filter(|e| e.target_uuid == target_uuid).collect()
    }
}

impl Default for EntityGraph {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generation_results_success() {
        let result = GenerationResults::success(vec!["test.rs".to_string()])
            .add_note("Generated successfully".to_string());
        
        assert!(result.success);
        assert_eq!(result.models_generated.len(), 1);
        assert_eq!(result.analysis_notes.len(), 1);
    }

    #[test]
    fn test_generation_results_failure() {
        let result = GenerationResults::failure("Test error".to_string());
        
        assert!(!result.success);
        assert_eq!(result.models_generated.len(), 0);
        assert_eq!(result.analysis_notes[0], "Test error");
    }

    #[test]
    fn test_analysis_summary() {
        let mut counts = HashMap::new();
        let mut regions = HashMap::new();
        regions.insert("Aurora Bushes".to_string(), 45);
        counts.insert("regions".to_string(), regions);

        let summary = AnalysisSummary::new()
            .set_entity_counts(counts)
            .set_uncategorized_count(5)
            .add_note("Test run".to_string());
        
        assert_eq!(summary.total_entities, 50);
        assert_eq!(summary.uncategorized_count, 5);
        assert!(!summary.notes.is_empty());
    }

    #[test]
    fn test_entity_graph() {
        let graph = EntityGraph::new()
            .add_edge(EntityEdge::new(
                "uuid1".to_string(),
                "uuid2".to_string(),
                EdgeType::SettlementInHex,
                "settlement_uuid".to_string(),
            ));
        
        assert_eq!(graph.edges.len(), 1);
        assert_eq!(graph.edges_of_type(EdgeType::SettlementInHex).len(), 1);
        assert_eq!(graph.edges_from("uuid1").len(), 1);
        assert_eq!(graph.edges_to("uuid2").len(), 1);
    }
}
