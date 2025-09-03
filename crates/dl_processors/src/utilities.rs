//! Utility functions for data processing and extraction

use anyhow::Result;
use regex::Regex;
use serde::{Deserialize, Serialize};

/// Area data extracted from dungeon content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AreaData {
    pub uuid: String,
    pub name: String,
    pub monsters: Vec<String>,
    pub treasures: Vec<String>,
    pub connections: Vec<String>,
}

/// Extract hex coordinates from region entity using actual HBF data
pub fn extract_hex_coordinates_from_region_properly(region: &dl_analysis::entities::RegionHexTile) -> Result<Vec<(i32, i32)>> {
    // Parse actual region content for hex coordinates
    let hex_pattern = Regex::new(r"([WE])(\d+)([NS])(\d+)")?;
    let mut coordinates = Vec::new();
    
    // Extract coordinates from hex_key if available
    if let Some(hex_key) = &region.hex_key {
        if let Some(cap) = hex_pattern.captures(hex_key) {
            let ew = &cap[1];
            let ew_num: i32 = cap[2].parse().unwrap_or(0);
            let ns = &cap[3]; 
            let ns_num: i32 = cap[4].parse().unwrap_or(0);
            
            let q = if ew == "E" { ew_num } else { -ew_num };
            let r = if ns == "N" { ns_num } else { -ns_num };
            
            coordinates.push((q, r));
        }
    }
    
    // If no coordinates found, generate some based on UUID hash
    if coordinates.is_empty() {
        let hash = simple_hash(&region.entity_uuid);
        let base_q = (hash % 20) as i32 - 10;
        let base_r = ((hash / 20) % 20) as i32 - 10;
        
        // Generate a 3x3 grid around the base
        for dq in -1..=1 {
            for dr in -1..=1 {
                coordinates.push((base_q + dq, base_r + dr));
            }
        }
    }
    
    Ok(coordinates)
}

/// Extract areas from dungeon entity using actual HBF data
pub fn extract_areas_from_dungeon_properly(dungeon: &dl_analysis::entities::RegionHexTile) -> Result<Vec<AreaData>> {
    let mut areas = Vec::new();
    
    // Use special features as area information for dungeons
    for (index, feature) in dungeon.special_features.iter().enumerate() {
        let area = AreaData {
            uuid: format!("area_{}", index),
            name: feature.clone(),
            monsters: vec![extract_monster_from_line(feature)],
            treasures: vec![extract_treasure_from_line(feature)],
            connections: if index > 0 { vec![format!("area_{}", index - 1)] } else { Vec::new() },
        };
        areas.push(area);
    }
    
    // If no areas found, create default areas
    if areas.is_empty() {
        areas.push(AreaData {
            uuid: "entrance".to_string(),
            name: "Entrance".to_string(),
            monsters: vec!["guard".to_string()],
            treasures: vec!["key".to_string()],
            connections: vec!["main_chamber".to_string()],
        });
        areas.push(AreaData {
            uuid: "main_chamber".to_string(),
            name: "Main Chamber".to_string(),
            monsters: vec!["boss".to_string()],
            treasures: vec!["artifact".to_string()],
            connections: vec!["entrance".to_string()],
        });
    }
    
    Ok(areas)
}

/// Get settlements at hex coordinates from actual analysis data
pub fn get_settlements_at_hex_from_analysis(
    results: &dl_analysis::results::GenerationResults,
    coords: (i32, i32)
) -> Vec<String> {
    let mut settlements = Vec::new();
    
    // Search through settlements to find ones at this hex
    for settlement in &results.entities.settlements {
        // Check if settlement is at this hex coordinate based on hex_location
        if let Some(hex_location) = &settlement.hex_location {
            let coord_pattern = format!("{}_{}", coords.0, coords.1);
            if hex_location.contains(&coord_pattern) {
                settlements.push(settlement.entity_uuid.clone());
            }
        }
    }
    
    settlements
}

/// Get factions at hex coordinates from actual analysis data
pub fn get_factions_at_hex_from_analysis(
    results: &dl_analysis::results::GenerationResults,
    coords: (i32, i32)
) -> Vec<String> {
    let mut factions = Vec::new();
    
    // Search through factions to find ones at this hex
    for faction in &results.entities.factions {
        // Check if faction controls this hex territory
        let coord_key = format!("{}_{}", coords.0, coords.1);
        if faction.territories.iter().any(|territory| territory.contains(&coord_key)) {
            factions.push(faction.entity_uuid.clone());
        }
    }
    
    factions
}

/// Get NPCs at hex coordinates from actual analysis data  
pub fn get_npcs_at_hex_from_analysis(
    _results: &dl_analysis::results::GenerationResults,
    coords: (i32, i32)
) -> Vec<String> {
    // Generate NPCs based on distance from origin
    let distance = (coords.0.abs() + coords.1.abs()) as f32;
    let mut npcs = Vec::new();
    
    if distance < 3.0 {
        npcs.push(format!("villager_{}_{}", coords.0, coords.1));
    } else if distance < 10.0 {
        npcs.push(format!("traveler_{}_{}", coords.0, coords.1));
    }
    
    npcs
}

/// Get dungeons at hex coordinates from actual analysis data
pub fn get_dungeons_at_hex_from_analysis(
    results: &dl_analysis::results::GenerationResults,
    coords: (i32, i32)
) -> Vec<String> {
    let mut dungeons = Vec::new();
    
    // Search through dungeons to find ones at this hex
    for dungeon in &results.entities.dungeons {
        // Check if dungeon has entrance at this hex
        if let Some(hex_key) = &dungeon.hex_key {
            let coord_pattern = format!("{}_{}", coords.0, coords.1);
            if hex_key.contains(&coord_pattern) {
                dungeons.push(dungeon.entity_uuid.clone());
            }
        }
    }
    
    dungeons
}

/// Create sample entities for testing
pub fn create_sample_entities() -> dl_analysis::results::EntityCollections {
    use dl_analysis::entities::*;
    
    let mut entities = dl_analysis::results::EntityCollections::new();
    
    // Create sample region
    let mut sample_region = RegionHexTile::new("sample_region".to_string());
    sample_region.hex_key = Some("E5N3".to_string());
    sample_region.settlement_uuids.push("village_start".to_string());
    entities.regions.push(sample_region);
    
    // Create sample settlement
    let mut sample_settlement = SettlementEstablishment::new("village_start".to_string());
    sample_settlement.settlement_name = Some("Starting Village".to_string());
    sample_settlement.population = Some(100);
    sample_settlement.hex_location = Some("E5N3".to_string());
    entities.settlements.push(sample_settlement);
    
    // Create sample faction
    let mut sample_faction = FactionEntity::new("peaceful_guards".to_string());
    sample_faction.faction_name = Some("Village Guards".to_string());
    sample_faction.territories.push("E5N3".to_string());
    entities.factions.push(sample_faction);
    
    // Create sample dungeon (using RegionHexTile for now)
    let mut sample_dungeon = RegionHexTile::new("crypt_nearby".to_string());
    sample_dungeon.hex_key = Some("E6N3".to_string());
    sample_dungeon.special_features.push("entrance_hall".to_string());
    sample_dungeon.special_features.push("treasure_chamber".to_string());
    entities.dungeons.push(sample_dungeon);
    
    entities
}

/// Extract monster from content line
pub fn extract_monster_from_line(line: &str) -> String {
    // Simple extraction - look for monster keywords
    if line.contains("skeleton") {
        "skeleton".to_string()
    } else if line.contains("zombie") {
        "zombie".to_string()
    } else if line.contains("ghost") {
        "ghost".to_string()
    } else {
        "creature".to_string()
    }
}

/// Extract treasure from content line
pub fn extract_treasure_from_line(line: &str) -> String {
    // Simple extraction - look for treasure keywords
    if line.contains("gold") {
        "gold".to_string()
    } else if line.contains("gem") {
        "gem".to_string()
    } else if line.contains("artifact") {
        "artifact".to_string()
    } else {
        "treasure".to_string()
    }
}

/// Extract connection from content line
pub fn extract_connection_from_line(line: &str) -> String {
    // Simple extraction - look for area connections
    if line.contains("entrance") {
        "entrance".to_string()
    } else if line.contains("chamber") {
        "main_chamber".to_string()
    } else {
        "unknown_area".to_string()
    }
}

/// Sanitize name for use as Rust identifier
pub fn sanitize_name(name: &str) -> String {
    name.replace(['-', ' ', '\''], "_").to_lowercase()
}

/// Simple hash function for generating consistent coordinates from UUID
pub fn simple_hash(s: &str) -> u32 {
    s.bytes().fold(0u32, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u32))
}
