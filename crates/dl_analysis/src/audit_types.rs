//! Audit data types for dl_analysis pipeline reporting
//! 
//! Implements AuditableType trait for all analysis metrics to enable
//! comprehensive reporting via dl_audit system.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use dl_types::AuditableType;

/// Audit data for entity extraction performance from HBF database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityExtractionAudit {
    pub total_entities: usize,
    pub regions_found: usize,
    pub settlements_found: usize,
    pub factions_found: usize,
    pub dungeons_found: usize,
    pub uncategorized_count: usize,
    pub extraction_time_ms: u64,
    pub database_path: String,
}

impl AuditableType for EntityExtractionAudit {
    fn audit_headers() -> Vec<String> {
        vec![
            "total_entities".to_string(),
            "regions_found".to_string(),
            "settlements_found".to_string(), 
            "factions_found".to_string(),
            "dungeons_found".to_string(),
            "uncategorized_count".to_string(),
            "extraction_time_ms".to_string(),
            "database_path".to_string(),
            "extraction_rate_entities_per_ms".to_string(),
        ]
    }
    
    fn audit_row(&self) -> Vec<String> {
        let rate = if self.extraction_time_ms > 0 {
            (self.total_entities as f64 / self.extraction_time_ms as f64)
        } else {
            0.0
        };
        
        vec![
            self.total_entities.to_string(),
            self.regions_found.to_string(),
            self.settlements_found.to_string(),
            self.factions_found.to_string(),
            self.dungeons_found.to_string(),
            self.uncategorized_count.to_string(),
            self.extraction_time_ms.to_string(),
            self.database_path.clone(),
            format!("{:.2}", rate),
        ]
    }
    
    fn audit_category() -> String {
        "analysis".to_string()
    }
    
    fn audit_subcategory() -> String {
        "extraction".to_string()
    }
    
    fn extract_numeric_fields(&self) -> HashMap<String, f64> {
        let mut fields = HashMap::new();
        fields.insert("total_entities".to_string(), self.total_entities as f64);
        fields.insert("regions_found".to_string(), self.regions_found as f64);
        fields.insert("settlements_found".to_string(), self.settlements_found as f64);
        fields.insert("factions_found".to_string(), self.factions_found as f64);
        fields.insert("dungeons_found".to_string(), self.dungeons_found as f64);
        fields.insert("uncategorized_count".to_string(), self.uncategorized_count as f64);
        fields.insert("extraction_time_ms".to_string(), self.extraction_time_ms as f64);
        fields
    }
}

/// Audit data for hex tile metadata completeness validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HexTileMetadataAudit {
    pub total_hex_tiles: usize,
    pub tiles_with_biome_data: usize,
    pub tiles_with_poi_associations: usize,
    pub tiles_with_coordinate_mapping: usize,
    pub tiles_with_complete_metadata: usize,
    pub biome_data_completeness_percentage: f64,
    pub poi_association_completeness_percentage: f64,
    pub coordinate_mapping_completeness_percentage: f64,
    pub overall_metadata_completeness_percentage: f64,
    pub missing_biome_count: usize,
    pub missing_poi_count: usize,
    pub missing_coordinate_count: usize,
}

impl AuditableType for HexTileMetadataAudit {
    fn audit_headers() -> Vec<String> {
        vec![
            "total_hex_tiles".to_string(),
            "tiles_with_biome_data".to_string(),
            "tiles_with_poi_associations".to_string(),
            "tiles_with_coordinate_mapping".to_string(),
            "tiles_with_complete_metadata".to_string(),
            "biome_data_completeness_percentage".to_string(),
            "poi_association_completeness_percentage".to_string(),
            "coordinate_mapping_completeness_percentage".to_string(),
            "overall_metadata_completeness_percentage".to_string(),
            "missing_biome_count".to_string(),
            "missing_poi_count".to_string(),
            "missing_coordinate_count".to_string(),
            "metadata_quality_score".to_string(),
        ]
    }
    
    fn audit_row(&self) -> Vec<String> {
        // Calculate metadata quality score (weighted average)
        let quality_score = (self.biome_data_completeness_percentage * 0.4) +
                           (self.poi_association_completeness_percentage * 0.3) +
                           (self.coordinate_mapping_completeness_percentage * 0.3);
        
        vec![
            self.total_hex_tiles.to_string(),
            self.tiles_with_biome_data.to_string(),
            self.tiles_with_poi_associations.to_string(),
            self.tiles_with_coordinate_mapping.to_string(),
            self.tiles_with_complete_metadata.to_string(),
            format!("{:.2}", self.biome_data_completeness_percentage),
            format!("{:.2}", self.poi_association_completeness_percentage),
            format!("{:.2}", self.coordinate_mapping_completeness_percentage),
            format!("{:.2}", self.overall_metadata_completeness_percentage),
            self.missing_biome_count.to_string(),
            self.missing_poi_count.to_string(),
            self.missing_coordinate_count.to_string(),
            format!("{:.2}", quality_score),
        ]
    }
    
    fn audit_category() -> String {
        "analysis".to_string()
    }
    
    fn audit_subcategory() -> String {
        "hex_tile_metadata".to_string()
    }
    
    fn extract_numeric_fields(&self) -> HashMap<String, f64> {
        let mut fields = HashMap::new();
        fields.insert("total_hex_tiles".to_string(), self.total_hex_tiles as f64);
        fields.insert("tiles_with_biome_data".to_string(), self.tiles_with_biome_data as f64);
        fields.insert("tiles_with_poi_associations".to_string(), self.tiles_with_poi_associations as f64);
        fields.insert("tiles_with_coordinate_mapping".to_string(), self.tiles_with_coordinate_mapping as f64);
        fields.insert("tiles_with_complete_metadata".to_string(), self.tiles_with_complete_metadata as f64);
        fields.insert("biome_data_completeness_percentage".to_string(), self.biome_data_completeness_percentage);
        fields.insert("poi_association_completeness_percentage".to_string(), self.poi_association_completeness_percentage);
        fields.insert("coordinate_mapping_completeness_percentage".to_string(), self.coordinate_mapping_completeness_percentage);
        fields.insert("overall_metadata_completeness_percentage".to_string(), self.overall_metadata_completeness_percentage);
        fields.insert("missing_biome_count".to_string(), self.missing_biome_count as f64);
        fields.insert("missing_poi_count".to_string(), self.missing_poi_count as f64);
        fields.insert("missing_coordinate_count".to_string(), self.missing_coordinate_count as f64);
        fields
    }
}

/// Audit data for dungeon area rich data validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DungeonAreaRichDataAudit {
    pub total_dungeon_areas: usize,
    pub areas_with_cr_levels: usize,
    pub areas_with_loot_tables: usize,
    pub areas_with_narrative_content: usize,
    pub areas_with_area_descriptions: usize,
    pub areas_with_complete_rich_data: usize,
    pub cr_levels_completeness_percentage: f64,
    pub loot_tables_completeness_percentage: f64,
    pub narrative_content_completeness_percentage: f64,
    pub area_descriptions_completeness_percentage: f64,
    pub overall_rich_data_completeness_percentage: f64,
    pub missing_cr_levels_count: usize,
    pub missing_loot_tables_count: usize,
    pub missing_narrative_count: usize,
    pub missing_descriptions_count: usize,
}

impl AuditableType for DungeonAreaRichDataAudit {
    fn audit_headers() -> Vec<String> {
        vec![
            "total_dungeon_areas".to_string(),
            "areas_with_cr_levels".to_string(),
            "areas_with_loot_tables".to_string(),
            "areas_with_narrative_content".to_string(),
            "areas_with_area_descriptions".to_string(),
            "areas_with_complete_rich_data".to_string(),
            "cr_levels_completeness_percentage".to_string(),
            "loot_tables_completeness_percentage".to_string(),
            "narrative_content_completeness_percentage".to_string(),
            "area_descriptions_completeness_percentage".to_string(),
            "overall_rich_data_completeness_percentage".to_string(),
            "missing_cr_levels_count".to_string(),
            "missing_loot_tables_count".to_string(),
            "missing_narrative_count".to_string(),
            "missing_descriptions_count".to_string(),
            "rich_data_quality_score".to_string(),
        ]
    }
    
    fn audit_row(&self) -> Vec<String> {
        // Calculate rich data quality score (weighted average)
        let quality_score = (self.cr_levels_completeness_percentage * 0.3) +
                           (self.loot_tables_completeness_percentage * 0.25) +
                           (self.narrative_content_completeness_percentage * 0.25) +
                           (self.area_descriptions_completeness_percentage * 0.2);
        
        vec![
            self.total_dungeon_areas.to_string(),
            self.areas_with_cr_levels.to_string(),
            self.areas_with_loot_tables.to_string(),
            self.areas_with_narrative_content.to_string(),
            self.areas_with_area_descriptions.to_string(),
            self.areas_with_complete_rich_data.to_string(),
            format!("{:.2}", self.cr_levels_completeness_percentage),
            format!("{:.2}", self.loot_tables_completeness_percentage),
            format!("{:.2}", self.narrative_content_completeness_percentage),
            format!("{:.2}", self.area_descriptions_completeness_percentage),
            format!("{:.2}", self.overall_rich_data_completeness_percentage),
            self.missing_cr_levels_count.to_string(),
            self.missing_loot_tables_count.to_string(),
            self.missing_narrative_count.to_string(),
            self.missing_descriptions_count.to_string(),
            format!("{:.2}", quality_score),
        ]
    }
    
    fn audit_category() -> String {
        "analysis".to_string()
    }
    
    fn audit_subcategory() -> String {
        "dungeon_rich_data".to_string()
    }
    
    fn extract_numeric_fields(&self) -> HashMap<String, f64> {
        let mut fields = HashMap::new();
        fields.insert("total_dungeon_areas".to_string(), self.total_dungeon_areas as f64);
        fields.insert("areas_with_cr_levels".to_string(), self.areas_with_cr_levels as f64);
        fields.insert("areas_with_loot_tables".to_string(), self.areas_with_loot_tables as f64);
        fields.insert("areas_with_narrative_content".to_string(), self.areas_with_narrative_content as f64);
        fields.insert("areas_with_area_descriptions".to_string(), self.areas_with_area_descriptions as f64);
        fields.insert("areas_with_complete_rich_data".to_string(), self.areas_with_complete_rich_data as f64);
        fields.insert("cr_levels_completeness_percentage".to_string(), self.cr_levels_completeness_percentage);
        fields.insert("loot_tables_completeness_percentage".to_string(), self.loot_tables_completeness_percentage);
        fields.insert("narrative_content_completeness_percentage".to_string(), self.narrative_content_completeness_percentage);
        fields.insert("area_descriptions_completeness_percentage".to_string(), self.area_descriptions_completeness_percentage);
        fields.insert("overall_rich_data_completeness_percentage".to_string(), self.overall_rich_data_completeness_percentage);
        fields.insert("missing_cr_levels_count".to_string(), self.missing_cr_levels_count as f64);
        fields.insert("missing_loot_tables_count".to_string(), self.missing_loot_tables_count as f64);
        fields.insert("missing_narrative_count".to_string(), self.missing_narrative_count as f64);
        fields.insert("missing_descriptions_count".to_string(), self.missing_descriptions_count as f64);
        fields
    }
}

/// Audit data for categorization accuracy against known constants
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategorizationAccuracyAudit {
    pub total_regions_expected: usize,
    pub total_regions_found: usize,
    pub regions_match: bool,
    pub total_settlements_expected: usize,
    pub total_settlements_found: usize,
    pub settlements_match: bool,
    pub total_factions_expected: usize,
    pub total_factions_found: usize,
    pub factions_match: bool,
    pub total_dungeons_expected: usize,
    pub total_dungeons_found: usize,
    pub dungeons_match: bool,
    pub categorization_accuracy: f64,
    pub uncategorized_entities: usize,
    pub total_entities_processed: usize,
}

impl AuditableType for CategorizationAccuracyAudit {
    fn audit_headers() -> Vec<String> {
        vec![
            "total_regions_expected".to_string(),
            "total_regions_found".to_string(),
            "regions_match".to_string(),
            "total_settlements_expected".to_string(),
            "total_settlements_found".to_string(),
            "settlements_match".to_string(),
            "total_factions_expected".to_string(),
            "total_factions_found".to_string(),
            "factions_match".to_string(),
            "total_dungeons_expected".to_string(),
            "total_dungeons_found".to_string(),
            "dungeons_match".to_string(),
            "categorization_accuracy".to_string(),
            "uncategorized_entities".to_string(),
            "total_entities_processed".to_string(),
            "overall_match_status".to_string(),
        ]
    }
    
    fn audit_row(&self) -> Vec<String> {
        let overall_match = self.regions_match && self.settlements_match && self.factions_match && self.dungeons_match;
        
        vec![
            self.total_regions_expected.to_string(),
            self.total_regions_found.to_string(),
            self.regions_match.to_string(),
            self.total_settlements_expected.to_string(),
            self.total_settlements_found.to_string(),
            self.settlements_match.to_string(),
            self.total_factions_expected.to_string(),
            self.total_factions_found.to_string(),
            self.factions_match.to_string(),
            self.total_dungeons_expected.to_string(),
            self.total_dungeons_found.to_string(),
            self.dungeons_match.to_string(),
            format!("{:.2}", self.categorization_accuracy),
            self.uncategorized_entities.to_string(),
            self.total_entities_processed.to_string(),
            overall_match.to_string(),
        ]
    }
    
    fn audit_category() -> String {
        "analysis".to_string()
    }
    
    fn audit_subcategory() -> String {
        "categorization".to_string()
    }
    
    fn extract_numeric_fields(&self) -> HashMap<String, f64> {
        let mut fields = HashMap::new();
        fields.insert("total_regions_expected".to_string(), self.total_regions_expected as f64);
        fields.insert("total_regions_found".to_string(), self.total_regions_found as f64);
        fields.insert("total_settlements_expected".to_string(), self.total_settlements_expected as f64);
        fields.insert("total_settlements_found".to_string(), self.total_settlements_found as f64);
        fields.insert("total_factions_expected".to_string(), self.total_factions_expected as f64);
        fields.insert("total_factions_found".to_string(), self.total_factions_found as f64);
        fields.insert("total_dungeons_expected".to_string(), self.total_dungeons_expected as f64);
        fields.insert("total_dungeons_found".to_string(), self.total_dungeons_found as f64);
        fields.insert("categorization_accuracy".to_string(), self.categorization_accuracy);
        fields.insert("uncategorized_entities".to_string(), self.uncategorized_entities as f64);
        fields.insert("total_entities_processed".to_string(), self.total_entities_processed as f64);
        fields
    }
}

/// Audit data for overall analysis pipeline performance  
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisPerformanceAudit {
    pub total_analysis_time_ms: u64,
    pub phase1_ai_generation_time_ms: u64,
    pub phase3_container_time_ms: u64,
    pub models_generated: usize,
    pub entities_per_second: u64,
    pub hbf_database_size_mb: u64,
    pub analysis_output_path: String,
    pub models_output_path: String,
}

impl AuditableType for AnalysisPerformanceAudit {
    fn audit_headers() -> Vec<String> {
        vec![
            "total_analysis_time_ms".to_string(),
            "phase1_ai_generation_time_ms".to_string(),
            "phase3_container_time_ms".to_string(),
            "models_generated".to_string(),
            "entities_per_second".to_string(),
            "hbf_database_size_mb".to_string(),
            "analysis_output_path".to_string(),
            "models_output_path".to_string(),
            "ai_generation_percentage".to_string(),
            "container_generation_percentage".to_string(),
        ]
    }
    
    fn audit_row(&self) -> Vec<String> {
        let ai_percentage = if self.total_analysis_time_ms > 0 {
            (self.phase1_ai_generation_time_ms as f64 / self.total_analysis_time_ms as f64) * 100.0
        } else {
            0.0
        };
        
        let container_percentage = if self.total_analysis_time_ms > 0 {
            (self.phase3_container_time_ms as f64 / self.total_analysis_time_ms as f64) * 100.0
        } else {
            0.0
        };
        
        vec![
            self.total_analysis_time_ms.to_string(),
            self.phase1_ai_generation_time_ms.to_string(),
            self.phase3_container_time_ms.to_string(),
            self.models_generated.to_string(),
            self.entities_per_second.to_string(),
            self.hbf_database_size_mb.to_string(),
            self.analysis_output_path.clone(),
            self.models_output_path.clone(),
            format!("{:.1}", ai_percentage),
            format!("{:.1}", container_percentage),
        ]
    }
    
    fn audit_category() -> String {
        "analysis".to_string()
    }
    
    fn audit_subcategory() -> String {
        "performance".to_string()
    }
    
    fn extract_numeric_fields(&self) -> HashMap<String, f64> {
        let mut fields = HashMap::new();
        fields.insert("total_analysis_time_ms".to_string(), self.total_analysis_time_ms as f64);
        fields.insert("phase1_ai_generation_time_ms".to_string(), self.phase1_ai_generation_time_ms as f64);
        fields.insert("phase3_container_time_ms".to_string(), self.phase3_container_time_ms as f64);
        fields.insert("models_generated".to_string(), self.models_generated as f64);
        fields.insert("entities_per_second".to_string(), self.entities_per_second as f64);
        fields.insert("hbf_database_size_mb".to_string(), self.hbf_database_size_mb as f64);
        fields
    }
}
