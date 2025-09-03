//! CSV reporting functionality for Dragon's Labyrinth analysis
//! 
//! Generates comprehensive CSV reports for all identified D&D resources
//! with support for REPORTS_DIR environment variable override.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use csv::Writer;
use anyhow::Result;

use dl_types::analysis::raw::RawEntity;
use dl_types::analysis::base::{KNOWN_REGIONS, KNOWN_SETTLEMENTS, KNOWN_FACTIONS, KNOWN_DUNGEONS};
use crate::orchestration::RawEntities;
use crate::clusters::EntityCluster;

/// Get the reports directory from environment or use default
pub fn get_reports_dir() -> Result<PathBuf> {
    if let Ok(reports_dir) = env::var("REPORTS_DIR") {
        Ok(PathBuf::from(reports_dir))
    } else if let Ok(out_dir) = env::var("OUT_DIR") {
        Ok(PathBuf::from(out_dir).join("reports"))
    } else {
        Ok(PathBuf::from("build/reports"))
    }
}

/// Generate all CSV reports from the analysis results
pub fn generate_all_reports(orchestrator: &RawEntities, reports_dir: &Path) -> Result<()> {
    // Create reports directory if it doesn't exist
    fs::create_dir_all(reports_dir)?;
    
    // Generate individual reports
    generate_regions_overview(orchestrator, reports_dir)?;
    generate_settlements_overview(orchestrator, reports_dir)?;
    generate_factions_overview(orchestrator, reports_dir)?;
    generate_dungeons_detailed(orchestrator, reports_dir)?;
    generate_analysis_summary(orchestrator, reports_dir)?;
    
    Ok(())
}

/// Generate regions overview CSV
fn generate_regions_overview(orchestrator: &RawEntities, reports_dir: &Path) -> Result<()> {
    let file_path = reports_dir.join("regions_overview.csv");
    let mut wtr = Writer::from_path(file_path)?;
    
    // Write headers
    wtr.write_record(&[
        "Region Name",
        "Has Entities",
        "Entity Count",
        "Can Generate Models",
        "Status"
    ])?;
    
    // Write data for each known region
    for region_name in KNOWN_REGIONS {
        let region_key = region_name.to_string();
        let has_cluster = orchestrator.regions.contains_key(&region_key);
        let can_generate = if has_cluster {
            orchestrator.regions.get(&region_key)
                .map(|c| c.can_generate_models())
                .unwrap_or(false)
        } else {
            false
        };
        
        let entity_count = if has_cluster && can_generate { "1+" } else { "0" };
        let status = if can_generate { "Ready" } else { "No Data" };
        
        let has_cluster_str = format!("{}", has_cluster);
        let can_generate_str = format!("{}", can_generate);
        
        wtr.write_record(&[
            region_name,
            has_cluster_str.as_str(),
            entity_count,
            can_generate_str.as_str(),
            status,
        ])?;
    }
    
    wtr.flush()?;
    Ok(())
}

/// Generate settlements overview CSV
fn generate_settlements_overview(orchestrator: &RawEntities, reports_dir: &Path) -> Result<()> {
    let file_path = reports_dir.join("settlements_overview.csv");
    let mut wtr = Writer::from_path(file_path)?;
    
    // Write headers
    wtr.write_record(&[
        "Settlement Name",
        "Has Entities",
        "Entity Count",
        "Can Generate Models",
        "Status"
    ])?;
    
    // Write data for each known settlement
    for settlement_name in KNOWN_SETTLEMENTS {
        let settlement_key = settlement_name.to_string();
        let has_cluster = orchestrator.settlements.contains_key(&settlement_key);
        let can_generate = if has_cluster {
            orchestrator.settlements.get(&settlement_key)
                .map(|c| c.can_generate_models())
                .unwrap_or(false)
        } else {
            false
        };
        
        let entity_count = if has_cluster && can_generate { "1+" } else { "0" };
        let status = if can_generate { "Ready" } else { "No Data" };
        
        let has_cluster_str = format!("{}", has_cluster);
        let can_generate_str = format!("{}", can_generate);
        
        wtr.write_record(&[
            settlement_name,
            has_cluster_str.as_str(),
            entity_count,
            can_generate_str.as_str(),
            status,
        ])?;
    }
    
    wtr.flush()?;
    Ok(())
}

/// Generate factions overview CSV
fn generate_factions_overview(orchestrator: &RawEntities, reports_dir: &Path) -> Result<()> {
    let file_path = reports_dir.join("factions_overview.csv");
    let mut wtr = Writer::from_path(file_path)?;
    
    // Write headers
    wtr.write_record(&[
        "Faction Name",
        "Has Entities",
        "Entity Count",
        "Can Generate Models",
        "Status"
    ])?;
    
    // Write data for each known faction
    for faction_name in KNOWN_FACTIONS {
        let faction_key = faction_name.to_string();
        let has_cluster = orchestrator.factions.contains_key(&faction_key);
        let can_generate = if has_cluster {
            orchestrator.factions.get(&faction_key)
                .map(|c| c.can_generate_models())
                .unwrap_or(false)
        } else {
            false
        };
        
        let entity_count = if has_cluster && can_generate { "1+" } else { "0" };
        let status = if can_generate { "Ready" } else { "No Data" };
        
        let has_cluster_str = format!("{}", has_cluster);
        let can_generate_str = format!("{}", can_generate);
        
        wtr.write_record(&[
            faction_name,
            has_cluster_str.as_str(),
            entity_count,
            can_generate_str.as_str(),
            status,
        ])?;
    }
    
    wtr.flush()?;
    Ok(())
}

/// Generate detailed dungeons CSV
fn generate_dungeons_detailed(orchestrator: &RawEntities, reports_dir: &Path) -> Result<()> {
    let file_path = reports_dir.join("dungeons_detailed.csv");
    let mut wtr = Writer::from_path(file_path)?;
    
    // Write headers
    wtr.write_record(&[
        "Dungeon Name",
        "Has Entities",
        "Entity Count",
        "Can Generate Models",
        "Type",
        "Status"
    ])?;
    
    // Write data for each known dungeon
    for dungeon_name in KNOWN_DUNGEONS {
        let dungeon_key = dungeon_name.to_string();
        let has_cluster = orchestrator.dungeons.contains_key(&dungeon_key);
        let can_generate = if has_cluster {
            orchestrator.dungeons.get(&dungeon_key)
                .map(|c| c.can_generate_models())
                .unwrap_or(false)
        } else {
            false
        };
        
        let entity_count = if has_cluster && can_generate { "1+" } else { "0" };
        let status = if can_generate { "Ready" } else { "No Data" };
        
        // Determine dungeon type from name
        let dungeon_type = if dungeon_name.contains("Crypt") {
            "Crypt"
        } else if dungeon_name.contains("Tomb") {
            "Tomb"
        } else if dungeon_name.contains("Caverns") || dungeon_name.contains("Cavern") {
            "Cavern"
        } else if dungeon_name.contains("Temple") {
            "Temple"
        } else if dungeon_name.contains("Shrine") {
            "Shrine"
        } else if dungeon_name.contains("Lair") {
            "Lair"
        } else if dungeon_name.contains("Hideout") {
            "Hideout"
        } else if dungeon_name.contains("Bowel") {
            "Bowel"
        } else {
            "Unknown"
        };
        
        let has_cluster_str = format!("{}", has_cluster);
        let can_generate_str = format!("{}", can_generate);
        
        wtr.write_record(&[
            dungeon_name,
            has_cluster_str.as_str(),
            entity_count,
            can_generate_str.as_str(),
            dungeon_type,
            status,
        ])?;
    }
    
    wtr.flush()?;
    Ok(())
}

/// Generate analysis summary CSV
fn generate_analysis_summary(orchestrator: &RawEntities, reports_dir: &Path) -> Result<()> {
    let file_path = reports_dir.join("analysis_summary.csv");
    let mut wtr = Writer::from_path(file_path)?;
    
    // Write headers
    wtr.write_record(&[
        "Category",
        "Total Known",
        "With Data",
        "Can Generate",
        "Coverage %"
    ])?;
    
    // Calculate region statistics
    let regions_total = KNOWN_REGIONS.len();
    let regions_with_data = orchestrator.regions.values()
        .filter(|c| c.can_generate_models())
        .count();
    let regions_coverage = (regions_with_data as f32 / regions_total as f32 * 100.0) as u32;
    
    wtr.write_record(&[
        "Regions",
        &regions_total.to_string(),
        &regions_with_data.to_string(),
        &regions_with_data.to_string(),
        &format!("{}%", regions_coverage),
    ])?;
    
    // Calculate settlement statistics
    let settlements_total = KNOWN_SETTLEMENTS.len();
    let settlements_with_data = orchestrator.settlements.values()
        .filter(|c| c.can_generate_models())
        .count();
    let settlements_coverage = (settlements_with_data as f32 / settlements_total as f32 * 100.0) as u32;
    
    wtr.write_record(&[
        "Settlements",
        &settlements_total.to_string(),
        &settlements_with_data.to_string(),
        &settlements_with_data.to_string(),
        &format!("{}%", settlements_coverage),
    ])?;
    
    // Calculate faction statistics
    let factions_total = KNOWN_FACTIONS.len();
    let factions_with_data = orchestrator.factions.values()
        .filter(|c| c.can_generate_models())
        .count();
    let factions_coverage = (factions_with_data as f32 / factions_total as f32 * 100.0) as u32;
    
    wtr.write_record(&[
        "Factions",
        &factions_total.to_string(),
        &factions_with_data.to_string(),
        &factions_with_data.to_string(),
        &format!("{}%", factions_coverage),
    ])?;
    
    // Calculate dungeon statistics
    let dungeons_total = KNOWN_DUNGEONS.len();
    let dungeons_with_data = orchestrator.dungeons.values()
        .filter(|c| c.can_generate_models())
        .count();
    let dungeons_coverage = (dungeons_with_data as f32 / dungeons_total as f32 * 100.0) as u32;
    
    wtr.write_record(&[
        "Dungeons",
        &dungeons_total.to_string(),
        &dungeons_with_data.to_string(),
        &dungeons_with_data.to_string(),
        &format!("{}%", dungeons_coverage),
    ])?;
    
    // Add totals row
    let total_known = regions_total + settlements_total + factions_total + dungeons_total;
    let total_with_data = regions_with_data + settlements_with_data + factions_with_data + dungeons_with_data;
    let total_coverage = (total_with_data as f32 / total_known as f32 * 100.0) as u32;
    
    wtr.write_record(&[
        "TOTAL",
        &total_known.to_string(),
        &total_with_data.to_string(),
        &total_with_data.to_string(),
        &format!("{}%", total_coverage),
    ])?;
    
    // Add metadata rows
    wtr.write_record(&["", "", "", "", ""])?;
    wtr.write_record(&[
        "Metadata",
        "Value",
        "",
        "",
        ""
    ])?;
    
    wtr.write_record(&[
        "Total Entities",
        &orchestrator.total_entities.to_string(),
        "",
        "",
        ""
    ])?;
    
    wtr.write_record(&[
        "Uncategorized",
        &orchestrator.uncategorized.len().to_string(),
        "",
        "",
        ""
    ])?;
    
    wtr.flush()?;
    Ok(())
}

/// Generate a report of uncategorized entities
pub fn generate_uncategorized_report(
    uncategorized: &[RawEntity],
    reports_dir: &Path
) -> Result<()> {
    if uncategorized.is_empty() {
        return Ok(());
    }
    
    let file_path = reports_dir.join("uncategorized_entities.csv");
    let mut wtr = Writer::from_path(file_path)?;
    
    // Write headers
    wtr.write_record(&[
        "UUID",
        "Category",
        "Entity Name",
        "Content Preview",
        "Hex Coordinates"
    ])?;
    
    // Write each uncategorized entity
    for entity in uncategorized {
        let content_preview = if entity.raw_value.len() > 100 {
            format!("{}...", &entity.raw_value[..100])
        } else {
            entity.raw_value.clone()
        };
        
        wtr.write_record(&[
            &entity.uuid,
            &entity.category,
            &entity.entity_name,
            &content_preview,
            "Unknown", // Placeholder for hex coordinates - need to extract from content
        ])?;
    }
    
    wtr.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_reports_dir_default() {
        // When no env vars are set, should use default
        env::remove_var("REPORTS_DIR");
        let dir = get_reports_dir().unwrap();
        assert!(dir.to_string_lossy().contains("reports"));
    }
    
    #[test]
    fn test_get_reports_dir_from_env() {
        // When REPORTS_DIR is set, should use it
        env::set_var("REPORTS_DIR", "/tmp/test_reports");
        let dir = get_reports_dir().unwrap();
        assert_eq!(dir, PathBuf::from("/tmp/test_reports"));
        env::remove_var("REPORTS_DIR");
    }
}
