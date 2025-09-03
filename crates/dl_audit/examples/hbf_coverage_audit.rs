//! Example: HBF Coverage Audit
//!
//! Demonstrates how dl_audit can analyze HBF data utilization and identify
//! the missing hex tiles issue mentioned by the user.

use dl_audit::{AuditSystem, ReportConfig};
use dl_types::{RegionHexTile, AuditableType};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Dragon's Labyrinth HBF Coverage Audit Example");
    println!("================================================");
    
    // Create audit system with reports directory
    let audit_config = ReportConfig::new("./audit_reports")
        .with_archiving(true)
        .with_statistics(true);
    
    let audit_system = AuditSystem::with_config(audit_config);
    
    // Simulate HBF hex tile data (some complete, some missing critical fields)
    let hex_tiles = vec![
        // Complete hex tile
        RegionHexTile {
            entity_uuid: "hex_001".to_string(),
            hex_key: Some("q1r2".to_string()),
            region_uuid: Some("region_meadows".to_string()),
            biome_type: Some("grassland".to_string()),
            settlement_uuids: vec!["settlement_village_001".to_string()],
            dungeon_uuids: vec!["dungeon_cave_001".to_string()],
            faction_uuids: vec!["faction_farmers".to_string()],
            terrain_features: vec!["river".to_string(), "hill".to_string()],
            special_features: vec!["shrine".to_string()],
            resource_nodes: vec!["iron_deposit".to_string()],
            map: Some(HashMap::new()),
        },
        
        // Incomplete hex tile (missing critical data) - THE ISSUE!
        RegionHexTile {
            entity_uuid: "hex_002".to_string(),
            hex_key: None, // MISSING - causes game world mapping problems
            region_uuid: None, // MISSING - prevents region assignment
            biome_type: None, // MISSING - no game mechanics
            settlement_uuids: vec![], // No entities discovered
            dungeon_uuids: vec![],
            faction_uuids: vec![],
            terrain_features: vec![],
            special_features: vec![],
            resource_nodes: vec![],
            map: None,
        },
        
        // Partially complete hex tile
        RegionHexTile {
            entity_uuid: "hex_003".to_string(),
            hex_key: Some("q3r4".to_string()),
            region_uuid: Some("region_forest".to_string()),
            biome_type: Some("forest".to_string()),
            settlement_uuids: vec![], // Has location data but no entities
            dungeon_uuids: vec![],
            faction_uuids: vec![],
            terrain_features: vec!["dense_canopy".to_string()],
            special_features: vec![],
            resource_nodes: vec![],
            map: Some(HashMap::new()),
        },
    ];
    
    println!("📊 Analyzing {} hex tiles from HBF...", hex_tiles.len());
    
    // Generate comprehensive audit report
    let metadata = audit_system.generate_report(&hex_tiles, "hbf_coverage_analysis")?;
    
    println!("\n✅ Audit Report Generated Successfully!");
    println!("═══════════════════════════════════════");
    println!("📁 Report path: {}", metadata.file_path);
    println!("📈 Total hex tiles analyzed: {}", metadata.row_count);
    println!("📊 Data columns captured: {}", metadata.column_count);
    println!("💾 Report size: {}", metadata.file_size_human());
    println!("⏱️  Generation time: {}", metadata.generation_time_human());
    
    if let Some(archive) = &metadata.archive_created {
        println!("🗃️  Previous reports archived: {}", archive);
    }
    
    println!("\n🔍 Expected Findings:");
    println!("• hex_001: High completeness score (~0.9) - GOOD");
    println!("• hex_002: Low completeness score (~0.0) - NEEDS ATTENTION");
    println!("• hex_003: Medium completeness score (~0.8) - ACCEPTABLE");
    
    println!("\n📋 Report Location Structure:");
    println!("audits/analytics/hbf_coverage/hbf_coverage_analysis.csv");
    println!("📊 This report will show exactly which hex tiles are causing");
    println!("   the 'only identifying half the necessary hex tiles' issue!");
    
    println!("\n💡 Next Steps:");
    println!("1. Integrate this audit into dl_analysis build_api.rs");
    println!("2. Add audit calls to dl_processors for downstream tracking");
    println!("3. Use audit reports to optimize HBF data extraction");
    println!("4. Monitor pipeline efficiency improvements over time");
    
    Ok(())
}
