//! Production audit capabilities for dl_analysis crate
//! 
//! This module provides comprehensive audit reporting that integrates directly
//! into the analysis pipeline, leveraging dl_audit for real-time data validation.

use anyhow::Result;
use std::path::Path;
use std::collections::HashMap;
use std::time::Instant;

use dl_audit::AuditSystem;
use crate::audit_types::{
    EntityExtractionAudit, CategorizationAccuracyAudit, AnalysisPerformanceAudit,
    HexTileMetadataAudit, DungeonAreaRichDataAudit
};
use crate::orchestration::RawEntities;
use dl_types::analysis::base::{KNOWN_REGIONS, KNOWN_SETTLEMENTS, KNOWN_FACTIONS, KNOWN_DUNGEONS};

/// Production audit manager for dl_analysis pipeline
pub struct AnalysisAuditor {
    audit_system: Option<AuditSystem>,
}

impl AnalysisAuditor {
    /// Create new auditor - only enabled if AUDIT_REPORTS_DIR is set
    pub fn new() -> Self {
        let audit_system = std::env::var("AUDIT_REPORTS_DIR")
            .ok()
            .map(|dir| AuditSystem::new(dir));
            
        Self { audit_system }
    }

    /// Check if auditing is enabled
    pub fn is_enabled(&self) -> bool {
        self.audit_system.is_some()
    }

    /// Generate comprehensive entity extraction audit report
    pub fn audit_entity_extraction(
        &self,
        entities: &RawEntities,
        extraction_time_ms: u64,
        database_path: &str,
    ) -> Result<()> {
        if let Some(ref audit_system) = self.audit_system {
            let extraction_audit = EntityExtractionAudit {
                total_entities: entities.total_entities,
                regions_found: entities.regions.values().map(|c| c.base.entities.len()).sum(),
                settlements_found: entities.settlements.values().map(|c| c.base.entities.len()).sum(),
                factions_found: entities.factions.values().map(|c| c.base.entities.len()).sum(),
                dungeons_found: entities.dungeons.values().map(|c| c.base.entities.len()).sum(),
                uncategorized_count: entities.uncategorized.len(),
                extraction_time_ms,
                database_path: database_path.to_string(),
            };
            
            audit_system.generate_report(&[extraction_audit], "entity_extraction")?;
        }
        Ok(())
    }

    /// Generate categorization accuracy audit report
    pub fn audit_categorization_accuracy(&self, entities: &RawEntities) -> Result<()> {
        if let Some(ref audit_system) = self.audit_system {
            let categorization_audit = CategorizationAccuracyAudit {
                total_regions_expected: KNOWN_REGIONS.len(),
                total_regions_found: entities.regions.len(),
                regions_match: entities.regions.len() == KNOWN_REGIONS.len(),
                total_settlements_expected: KNOWN_SETTLEMENTS.len(),
                total_settlements_found: entities.settlements.len(),
                settlements_match: entities.settlements.len() == KNOWN_SETTLEMENTS.len(),
                total_factions_expected: KNOWN_FACTIONS.len(),
                total_factions_found: entities.factions.len(),
                factions_match: entities.factions.len() == KNOWN_FACTIONS.len(),
                total_dungeons_expected: KNOWN_DUNGEONS.len(),
                total_dungeons_found: entities.dungeons.len(),
                dungeons_match: entities.dungeons.len() == KNOWN_DUNGEONS.len(),
                categorization_accuracy: Self::calculate_categorization_accuracy(entities),
                uncategorized_entities: entities.uncategorized.len(),
                total_entities_processed: entities.total_entities,
            };
            
            audit_system.generate_report(&[categorization_audit], "categorization_accuracy")?;
        }
        Ok(())
    }

    /// Generate analysis performance audit report
    pub fn audit_analysis_performance(
        &self,
        total_analysis_time_ms: u64,
        phase1_ai_generation_time_ms: u64,
        phase3_container_time_ms: u64,
        models_generated: usize,
        entities_per_second: u64,
        hbf_database_size_mb: u64,
        analysis_output_path: String,
        models_output_path: String,
    ) -> Result<()> {
        if let Some(ref audit_system) = self.audit_system {
            let performance_audit = AnalysisPerformanceAudit {
                total_analysis_time_ms,
                phase1_ai_generation_time_ms,
                phase3_container_time_ms,
                models_generated,
                entities_per_second,
                hbf_database_size_mb,
                analysis_output_path,
                models_output_path,
            };
            
            audit_system.generate_report(&[performance_audit], "analysis_performance")?;
        }
        Ok(())
    }

    /// Generate comprehensive hex tile metadata audit report
    pub fn audit_hex_tile_metadata(&self, entities: &RawEntities) -> Result<()> {
        if let Some(ref audit_system) = self.audit_system {
            let (total_tiles, complete_metadata, biome_data, poi_associations, coord_mapping) = 
                Self::analyze_hex_tile_metadata_completeness(entities);

            if total_tiles > 0 {
                let biome_pct = (biome_data as f64 / total_tiles as f64) * 100.0;
                let poi_pct = (poi_associations as f64 / total_tiles as f64) * 100.0;
                let coord_pct = (coord_mapping as f64 / total_tiles as f64) * 100.0;
                let overall_pct = (complete_metadata as f64 / total_tiles as f64) * 100.0;

                let hex_tile_audit = HexTileMetadataAudit {
                    total_hex_tiles: total_tiles,
                    tiles_with_biome_data: biome_data,
                    tiles_with_poi_associations: poi_associations,
                    tiles_with_coordinate_mapping: coord_mapping,
                    tiles_with_complete_metadata: complete_metadata,
                    biome_data_completeness_percentage: biome_pct,
                    poi_association_completeness_percentage: poi_pct,
                    coordinate_mapping_completeness_percentage: coord_pct,
                    overall_metadata_completeness_percentage: overall_pct,
                    missing_biome_count: total_tiles - biome_data,
                    missing_poi_count: total_tiles - poi_associations,
                    missing_coordinate_count: total_tiles - coord_mapping,
                };
                
                audit_system.generate_report(&[hex_tile_audit], "hex_tile_metadata")?;
            }
        }
        Ok(())
    }

    /// Generate comprehensive dungeon area rich data audit report  
    pub fn audit_dungeon_rich_data(&self, entities: &RawEntities) -> Result<()> {
        if let Some(ref audit_system) = self.audit_system {
            let (total_areas, complete_rich_data, cr_levels, loot_tables, narrative, descriptions) = 
                Self::analyze_dungeon_area_rich_data_completeness(entities);

            if total_areas > 0 {
                let cr_pct = (cr_levels as f64 / total_areas as f64) * 100.0;
                let loot_pct = (loot_tables as f64 / total_areas as f64) * 100.0;
                let narrative_pct = (narrative as f64 / total_areas as f64) * 100.0;
                let desc_pct = (descriptions as f64 / total_areas as f64) * 100.0;
                let overall_pct = (complete_rich_data as f64 / total_areas as f64) * 100.0;

                let dungeon_audit = DungeonAreaRichDataAudit {
                    total_dungeon_areas: total_areas,
                    areas_with_cr_levels: cr_levels,
                    areas_with_loot_tables: loot_tables,
                    areas_with_narrative_content: narrative,
                    areas_with_area_descriptions: descriptions,
                    areas_with_complete_rich_data: complete_rich_data,
                    cr_levels_completeness_percentage: cr_pct,
                    loot_tables_completeness_percentage: loot_pct,
                    narrative_content_completeness_percentage: narrative_pct,
                    area_descriptions_completeness_percentage: desc_pct,
                    overall_rich_data_completeness_percentage: overall_pct,
                    missing_cr_levels_count: total_areas - cr_levels,
                    missing_loot_tables_count: total_areas - loot_tables,
                    missing_narrative_count: total_areas - narrative,
                    missing_descriptions_count: total_areas - descriptions,
                };
                
                audit_system.generate_report(&[dungeon_audit], "dungeon_rich_data")?;
            }
        }
        Ok(())
    }

    /// Run comprehensive audit of all analysis pipeline stages
    pub fn run_comprehensive_audit<P: AsRef<Path>>(
        &self,
        hbf_database_path: P,
    ) -> Result<()> {
        if !self.is_enabled() {
            return Ok(()); // Silently skip if auditing disabled
        }

        let analysis_start = Instant::now();
        let mut entities = RawEntities::new();

        // Extract entities and time it
        let extraction_start = Instant::now();
        entities.load_from_hbf_database(&hbf_database_path, self.audit_system.as_ref())?;
        let extraction_time = extraction_start.elapsed();

        // Generate all audit reports
        self.audit_entity_extraction(
            &entities,
            extraction_time.as_millis() as u64,
            hbf_database_path.as_ref().to_string_lossy().as_ref(),
        )?;

        self.audit_categorization_accuracy(&entities)?;
        self.audit_hex_tile_metadata(&entities)?;
        self.audit_dungeon_rich_data(&entities)?;

        // Performance audit
        let total_time = analysis_start.elapsed();
        let entities_per_second = (entities.total_entities as f64 / total_time.as_secs_f64()) as u64;
        let database_size = Self::get_file_size_mb(&hbf_database_path)?;

        self.audit_analysis_performance(
            total_time.as_millis() as u64,
            0, // Phase 1 time (not applicable for this audit)
            0, // Phase 3 time (not applicable for this audit)
            0, // Models generated (not applicable for this audit)
            entities_per_second,
            database_size,
            "comprehensive_audit".to_string(),
            "comprehensive_audit".to_string(),
        )?;

        Ok(())
    }

    /// Calculate categorization accuracy percentage
    fn calculate_categorization_accuracy(entities: &RawEntities) -> f64 {
        let total_expected = KNOWN_REGIONS.len() + KNOWN_SETTLEMENTS.len() + KNOWN_FACTIONS.len() + KNOWN_DUNGEONS.len();
        
        if total_expected == 0 {
            return 0.0;
        }
        
        let regions_correct = if entities.regions.len() == KNOWN_REGIONS.len() { KNOWN_REGIONS.len() } else { 0 };
        let settlements_correct = if entities.settlements.len() == KNOWN_SETTLEMENTS.len() { KNOWN_SETTLEMENTS.len() } else { 0 };
        let factions_correct = if entities.factions.len() == KNOWN_FACTIONS.len() { KNOWN_FACTIONS.len() } else { 0 };
        let dungeons_correct = if entities.dungeons.len() == KNOWN_DUNGEONS.len() { KNOWN_DUNGEONS.len() } else { 0 };
        
        let correct_count = regions_correct + settlements_correct + factions_correct + dungeons_correct;
        (correct_count as f64 / total_expected as f64) * 100.0
    }

    /// Analyze hex tile metadata completeness
    fn analyze_hex_tile_metadata_completeness(entities: &RawEntities) -> (usize, usize, usize, usize, usize) {
        let mut total_hex_tiles = 0;
        let mut tiles_with_biome_data = 0;
        let mut tiles_with_poi_associations = 0;
        let mut tiles_with_coordinate_mapping = 0;
        let mut tiles_with_complete_metadata = 0;

        for (_region_name, cluster) in &entities.regions {
            for entity in &cluster.base.entities {
                if Self::is_hex_tile_entity(entity) {
                    total_hex_tiles += 1;
                    
                    let has_biome = Self::entity_has_biome_data(entity);
                    let has_poi = Self::entity_has_poi_associations(entity);
                    let has_coordinates = Self::entity_has_coordinate_mapping(entity);
                    
                    if has_biome { tiles_with_biome_data += 1; }
                    if has_poi { tiles_with_poi_associations += 1; }
                    if has_coordinates { tiles_with_coordinate_mapping += 1; }
                    
                    if has_biome && has_poi && has_coordinates {
                        tiles_with_complete_metadata += 1;
                    }
                }
            }
        }

        (total_hex_tiles, tiles_with_complete_metadata, tiles_with_biome_data, tiles_with_poi_associations, tiles_with_coordinate_mapping)
    }

    /// Analyze dungeon area rich data completeness
    fn analyze_dungeon_area_rich_data_completeness(entities: &RawEntities) -> (usize, usize, usize, usize, usize, usize) {
        let mut total_dungeon_areas = 0;
        let mut areas_with_cr_levels = 0;
        let mut areas_with_loot_tables = 0;
        let mut areas_with_narrative_content = 0;
        let mut areas_with_area_descriptions = 0;
        let mut areas_with_complete_rich_data = 0;

        for (_dungeon_name, cluster) in &entities.dungeons {
            for entity in &cluster.base.entities {
                if Self::is_dungeon_area_entity(entity) {
                    total_dungeon_areas += 1;
                    
                    let has_cr_levels = Self::entity_has_cr_levels(entity);
                    let has_loot_tables = Self::entity_has_loot_tables(entity);
                    let has_narrative = Self::entity_has_narrative_content(entity);
                    let has_descriptions = Self::entity_has_area_descriptions(entity);
                    
                    if has_cr_levels { areas_with_cr_levels += 1; }
                    if has_loot_tables { areas_with_loot_tables += 1; }
                    if has_narrative { areas_with_narrative_content += 1; }
                    if has_descriptions { areas_with_area_descriptions += 1; }
                    
                    if has_cr_levels && has_loot_tables && has_narrative && has_descriptions {
                        areas_with_complete_rich_data += 1;
                    }
                }
            }
        }

        (total_dungeon_areas, areas_with_complete_rich_data, areas_with_cr_levels, areas_with_loot_tables, areas_with_narrative_content, areas_with_area_descriptions)
    }

    /// Generate comprehensive audit reports for both hex tiles and dungeons
    pub fn audit_comprehensive_data_association<P: AsRef<Path>>(
        &self,
        hbf_database_path: P,
    ) -> Result<(usize, usize, usize, usize)> {
        if !self.is_enabled() {
            return Ok((0, 0, 0, 0)); // Return zeros if auditing disabled
        }

        let mut entities = RawEntities::new();
        entities.load_from_hbf_database(&hbf_database_path, self.audit_system.as_ref())?;

        // Audit hex tile metadata
        self.audit_hex_tile_metadata(&entities)?;
        
        // Audit dungeon rich data
        self.audit_dungeon_rich_data(&entities)?;

        // Return summary counts
        let (hex_tiles_total, hex_complete, _, _, _) = Self::analyze_hex_tile_metadata_completeness(&entities);
        let (dungeon_areas_total, dungeon_complete, _, _, _, _) = Self::analyze_dungeon_area_rich_data_completeness(&entities);

        Ok((hex_tiles_total, hex_complete, dungeon_areas_total, dungeon_complete))
    }

    // Helper methods for content analysis

    fn is_hex_tile_entity(entity: &dl_types::analysis::raw::RawEntity) -> bool {
        // Check if entity represents a hex tile (spatial/geographical data)
        let content = entity.raw_value.to_lowercase();
        content.contains("hex") ||
        content.contains("tile") ||
        content.contains("coordinate") ||
        content.contains("biome") ||
        content.contains("terrain") ||
        content.contains("location") ||
        (content.contains("area") && content.contains("map"))
    }

    fn entity_has_biome_data(entity: &dl_types::analysis::raw::RawEntity) -> bool {
        let content = entity.raw_value.to_lowercase();
        content.contains("biome") ||
        content.contains("terrain") ||
        content.contains("climate") ||
        content.contains("environment") ||
        content.contains("ecosystem") ||
        content.contains("flora") ||
        content.contains("fauna")
    }

    fn entity_has_poi_associations(entity: &dl_types::analysis::raw::RawEntity) -> bool {
        let content = entity.raw_value.to_lowercase();
        content.contains("poi") ||
        content.contains("point of interest") ||
        content.contains("landmark") ||
        content.contains("feature") ||
        content.contains("structure") ||
        content.contains("building") ||
        content.contains("monument")
    }

    fn entity_has_coordinate_mapping(entity: &dl_types::analysis::raw::RawEntity) -> bool {
        let content = entity.raw_value.to_lowercase();
        content.contains("coordinate") ||
        content.contains("position") ||
        content.contains("location") ||
        content.contains("x:") ||
        content.contains("y:") ||
        content.contains("lat") ||
        content.contains("lon") ||
        content.contains("spatial")
    }

    fn is_dungeon_area_entity(entity: &dl_types::analysis::raw::RawEntity) -> bool {
        let content = entity.raw_value.to_lowercase();
        content.contains("room") ||
        content.contains("chamber") ||
        content.contains("hall") ||
        content.contains("corridor") ||
        content.contains("area") ||
        content.contains("zone") ||
        content.contains("level") ||
        content.contains("floor")
    }

    fn entity_has_cr_levels(entity: &dl_types::analysis::raw::RawEntity) -> bool {
        let content = entity.raw_value.to_lowercase();
        content.contains("cr") ||
        content.contains("challenge rating") ||
        content.contains("difficulty") ||
        content.contains("level") ||
        content.contains("encounter rating") ||
        content.contains("threat level")
    }

    fn entity_has_loot_tables(entity: &dl_types::analysis::raw::RawEntity) -> bool {
        let content = entity.raw_value.to_lowercase();
        content.contains("loot") ||
        content.contains("treasure") ||
        content.contains("reward") ||
        content.contains("item") ||
        content.contains("equipment") ||
        content.contains("artifact") ||
        content.contains("chest")
    }

    fn entity_has_narrative_content(entity: &dl_types::analysis::raw::RawEntity) -> bool {
        let content = entity.raw_value.to_lowercase();
        content.contains("story") ||
        content.contains("narrative") ||
        content.contains("lore") ||
        content.contains("history") ||
        content.contains("dialogue") ||
        content.contains("quest") ||
        content.contains("event") ||
        content.contains("plot")
    }

    fn entity_has_area_descriptions(entity: &dl_types::analysis::raw::RawEntity) -> bool {
        let content = entity.raw_value.to_lowercase();
        content.contains("description") ||
        content.contains("appearance") ||
        content.contains("environment") ||
        content.contains("atmosphere") ||
        content.contains("details") ||
        content.contains("scenery") ||
        content.contains("ambiance") ||
        (content.contains("room") && content.contains("looks"))
    }

    fn get_file_size_mb<P: AsRef<Path>>(path: P) -> Result<u64> {
        let metadata = std::fs::metadata(path)?;
        Ok(metadata.len() / (1024 * 1024))
    }
}

/// Production function to run comprehensive audit during analysis pipeline
pub fn run_analysis_audit<P: AsRef<Path>>(hbf_database_path: P) -> Result<()> {
    let auditor = AnalysisAuditor::new();
    
    if auditor.is_enabled() {
        println!("📊 Running comprehensive analysis audit...");
        let (hex_tiles, hex_complete, dungeon_areas, dungeon_complete) = 
            auditor.audit_comprehensive_data_association(hbf_database_path)?;
        
        println!("✅ Audit complete:");
        println!("   🗺️  Hex tiles: {} total, {} with complete metadata", hex_tiles, hex_complete);
        println!("   🏰 Dungeon areas: {} total, {} with complete rich data", dungeon_areas, dungeon_complete);
        println!("   📁 Reports generated in $AUDIT_REPORTS_DIR");
    }
    
    Ok(())
}
