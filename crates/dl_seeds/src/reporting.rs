//! CSV reporting functionality for Dragon's Labyrinth analysis
//! 
//! Generates comprehensive CSV reports for all identified D&D resources
//! with support for REPORTS_DIR environment variable override.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use csv::Writer;
use anyhow::Result;

use crate::containers::RawEntity;

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
pub fn generate_all_reports(
    regions: &std::collections::HashMap<String, Vec<RawEntity>>,
    settlements: &std::collections::HashMap<String, Vec<RawEntity>>,
    factions: &std::collections::HashMap<String, Vec<RawEntity>>,
    dungeons: &std::collections::HashMap<String, Vec<RawEntity>>,
    uncategorized: &[RawEntity],
    reports_dir: &Path
) -> Result<()> {
    // Create reports directory if it doesn't exist
    fs::create_dir_all(reports_dir)?;
    
    // Generate individual reports
    generate_regions_overview(regions, reports_dir)?;
    generate_settlements_overview(settlements, reports_dir)?;
    generate_factions_overview(factions, reports_dir)?;
    generate_dungeons_detailed(dungeons, reports_dir)?;
    generate_analysis_summary(regions, settlements, factions, dungeons, uncategorized, reports_dir)?;
    
    // Generate uncategorized report if there are any
    if !uncategorized.is_empty() {
        generate_uncategorized_report(uncategorized, reports_dir)?;
    }
    
    Ok(())
}

/// Generate regions overview CSV
fn generate_regions_overview(
    regions: &std::collections::HashMap<String, Vec<RawEntity>>, 
    reports_dir: &Path
) -> Result<()> {
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
    
    // Write data for each region
    for (region_name, entities) in regions {
        let has_entities = !entities.is_empty();
        let entity_count = entities.len();
        let can_generate = entity_count > 0;
        let status = if can_generate { "Ready" } else { "No Data" };
        
        wtr.write_record(&[
            region_name,
            &has_entities.to_string(),
            &entity_count.to_string(),
            &can_generate.to_string(),
            status,
        ])?;
    }
    
    wtr.flush()?;
    Ok(())
}

/// Generate settlements overview CSV
fn generate_settlements_overview(
    settlements: &std::collections::HashMap<String, Vec<RawEntity>>,
    reports_dir: &Path
) -> Result<()> {
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
    
    // Write data for each settlement
    for (settlement_name, entities) in settlements {
        let has_entities = !entities.is_empty();
        let entity_count = entities.len();
        let can_generate = entity_count > 0;
        let status = if can_generate { "Ready" } else { "No Data" };
        
        wtr.write_record(&[
            settlement_name,
            &has_entities.to_string(),
            &entity_count.to_string(),
            &can_generate.to_string(),
            status,
        ])?;
    }
    
    wtr.flush()?;
    Ok(())
}

/// Generate factions overview CSV
fn generate_factions_overview(
    factions: &std::collections::HashMap<String, Vec<RawEntity>>,
    reports_dir: &Path
) -> Result<()> {
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
    
    // Write data for each faction
    for (faction_name, entities) in factions {
        let has_entities = !entities.is_empty();
        let entity_count = entities.len();
        let can_generate = entity_count > 0;
        let status = if can_generate { "Ready" } else { "No Data" };
        
        wtr.write_record(&[
            faction_name,
            &has_entities.to_string(),
            &entity_count.to_string(),
            &can_generate.to_string(),
            status,
        ])?;
    }
    
    wtr.flush()?;
    Ok(())
}

/// Generate detailed dungeons CSV
fn generate_dungeons_detailed(
    dungeons: &std::collections::HashMap<String, Vec<RawEntity>>,
    reports_dir: &Path
) -> Result<()> {
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
    
    // Write data for each dungeon
    for (dungeon_name, entities) in dungeons {
        let has_entities = !entities.is_empty();
        let entity_count = entities.len();
        let can_generate = entity_count > 0;
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
        
        wtr.write_record(&[
            dungeon_name,
            &has_entities.to_string(),
            &entity_count.to_string(),
            &can_generate.to_string(),
            dungeon_type,
            status,
        ])?;
    }
    
    wtr.flush()?;
    Ok(())
}

/// Generate analysis summary CSV
fn generate_analysis_summary(
    regions: &std::collections::HashMap<String, Vec<RawEntity>>,
    settlements: &std::collections::HashMap<String, Vec<RawEntity>>,
    factions: &std::collections::HashMap<String, Vec<RawEntity>>,
    dungeons: &std::collections::HashMap<String, Vec<RawEntity>>,
    uncategorized: &[RawEntity],
    reports_dir: &Path
) -> Result<()> {
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
    let regions_total = regions.len();
    let regions_with_data = regions.values().filter(|entities| !entities.is_empty()).count();
    let regions_coverage = if regions_total > 0 {
        (regions_with_data as f32 / regions_total as f32 * 100.0) as u32
    } else { 0 };
    
    wtr.write_record(&[
        "Regions",
        &regions_total.to_string(),
        &regions_with_data.to_string(),
        &regions_with_data.to_string(),
        &format!("{}%", regions_coverage),
    ])?;
    
    // Calculate settlement statistics
    let settlements_total = settlements.len();
    let settlements_with_data = settlements.values().filter(|entities| !entities.is_empty()).count();
    let settlements_coverage = if settlements_total > 0 {
        (settlements_with_data as f32 / settlements_total as f32 * 100.0) as u32
    } else { 0 };
    
    wtr.write_record(&[
        "Settlements",
        &settlements_total.to_string(),
        &settlements_with_data.to_string(),
        &settlements_with_data.to_string(),
        &format!("{}%", settlements_coverage),
    ])?;
    
    // Calculate faction statistics
    let factions_total = factions.len();
    let factions_with_data = factions.values().filter(|entities| !entities.is_empty()).count();
    let factions_coverage = if factions_total > 0 {
        (factions_with_data as f32 / factions_total as f32 * 100.0) as u32
    } else { 0 };
    
    wtr.write_record(&[
        "Factions",
        &factions_total.to_string(),
        &factions_with_data.to_string(),
        &factions_with_data.to_string(),
        &format!("{}%", factions_coverage),
    ])?;
    
    // Calculate dungeon statistics
    let dungeons_total = dungeons.len();
    let dungeons_with_data = dungeons.values().filter(|entities| !entities.is_empty()).count();
    let dungeons_coverage = if dungeons_total > 0 {
        (dungeons_with_data as f32 / dungeons_total as f32 * 100.0) as u32
    } else { 0 };
    
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
    let total_coverage = if total_known > 0 {
        (total_with_data as f32 / total_known as f32 * 100.0) as u32
    } else { 0 };
    
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
    
    let total_entities: usize = regions.values().map(|v| v.len()).sum::<usize>() + 
                              settlements.values().map(|v| v.len()).sum::<usize>() + 
                              factions.values().map(|v| v.len()).sum::<usize>() + 
                              dungeons.values().map(|v| v.len()).sum::<usize>();
    
    wtr.write_record(&[
        "Total Entities",
        &total_entities.to_string(),
        "",
        "",
        ""
    ])?;
    
    wtr.write_record(&[
        "Uncategorized",
        &uncategorized.len().to_string(),
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
        unsafe { env::remove_var("REPORTS_DIR"); }
        let dir = get_reports_dir().unwrap();
        assert!(dir.to_string_lossy().contains("reports"));
    }
    
    #[test]
    fn test_get_reports_dir_from_env() {
        // When REPORTS_DIR is set, should use it
        unsafe { env::set_var("REPORTS_DIR", "/tmp/test_reports"); }
        let dir = get_reports_dir().unwrap();
        assert_eq!(dir, PathBuf::from("/tmp/test_reports"));
        unsafe { env::remove_var("REPORTS_DIR"); }
    }
}
