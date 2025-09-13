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

/// Extract hex coordinates from content string using HBF patterns
pub fn extract_hex_coordinates_from_content(content: &str) -> Result<Vec<(i32, i32)>> {
    let hex_pattern = Regex::new(r"([WE])(\d+)([NS])(\d+)")?;
    let mut coordinates = Vec::new();
    
    // Find all hex coordinate patterns in content
    for cap in hex_pattern.captures_iter(content) {
        let ew = &cap[1];
        let ew_num: i32 = cap[2].parse().unwrap_or(0);
        let ns = &cap[3]; 
        let ns_num: i32 = cap[4].parse().unwrap_or(0);
        
        let q = if ew == "E" { ew_num } else { -ew_num };
        let r = if ns == "N" { ns_num } else { -ns_num };
        
        coordinates.push((q, r));
    }
    
    Ok(coordinates)
}

/// Extract areas from dungeon content string
pub fn extract_areas_from_content(content: &str) -> Result<Vec<AreaData>> {
    let mut areas = Vec::new();
    
    // Simple parsing for area information
    let lines: Vec<&str> = content.lines().collect();
    for (index, line) in lines.iter().enumerate() {
        if line.contains("area") || line.contains("chamber") || line.contains("room") {
            let area = AreaData {
                uuid: format!("area_{}", index),
                name: line.trim().to_string(),
                monsters: vec![extract_monster_from_line(line)],
                treasures: vec![extract_treasure_from_line(line)],
                connections: if index > 0 { vec![format!("area_{}", index - 1)] } else { Vec::new() },
            };
            areas.push(area);
        }
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
    }
    
    Ok(areas)
}

/// Get settlements at hex coordinates from analysis data
pub fn get_settlements_at_hex(coords: (i32, i32)) -> Vec<String> {
    let mut settlements = Vec::new();
    
    // Generate settlements based on distance from origin
    let distance = (coords.0.abs() + coords.1.abs()) as f32;
    
    if distance < 5.0 {
        settlements.push(format!("village_{}_{}", coords.0, coords.1));
    } else if distance < 15.0 && distance % 3.0 < 1.0 {
        settlements.push(format!("outpost_{}_{}", coords.0, coords.1));
    }
    
    settlements
}

/// Get factions at hex coordinates from analysis data
pub fn get_factions_at_hex(coords: (i32, i32)) -> Vec<String> {
    let mut factions = Vec::new();
    
    // Generate factions based on hex position
    let distance = (coords.0.abs() + coords.1.abs()) as f32;
    
    if distance < 10.0 {
        factions.push("peaceful_faction".to_string());
    } else if distance < 30.0 {
        factions.push("neutral_faction".to_string());
    } else {
        factions.push("hostile_faction".to_string());
    }
    
    factions
}

/// Get NPCs at hex coordinates from analysis data  
pub fn get_npcs_at_hex(coords: (i32, i32)) -> Vec<String> {
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

/// Get dungeons at hex coordinates from analysis data
pub fn get_dungeons_at_hex(coords: (i32, i32)) -> Vec<String> {
    let mut dungeons = Vec::new();
    
    // Generate dungeons based on distance and patterns
    let distance = (coords.0.abs() + coords.1.abs()) as f32;
    
    if distance > 5.0 && distance % 7.0 < 1.0 {
        dungeons.push(format!("dungeon_{}_{}", coords.0, coords.1));
    }
    
    dungeons
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

/// Determine biome type from hex coordinates
pub fn determine_biome_type(coords: (i32, i32)) -> String {
    let distance = (coords.0.abs() + coords.1.abs()) as f32;
    
    match distance as u32 {
        0..=20 => "Grassland".to_string(),
        21..=40 => "Forest".to_string(),  
        41..=60 => "Swamp".to_string(),
        61..=120 => "Mountain".to_string(),
        _ => "Corrupted".to_string(),
    }
}

/// Calculate faction influence at coordinates
pub fn calculate_faction_influence(faction_uuid: &str, coords: (i32, i32)) -> f32 {
    // Simple distance-based influence calculation
    let distance_from_origin = (coords.0.abs() + coords.1.abs()) as f32;
    let base_influence = 1.0 / (1.0 + distance_from_origin * 0.1);
    
    // Modify based on faction type
    let faction_modifier = match faction_uuid {
        uuid if uuid.contains("peaceful") => 1.2,
        uuid if uuid.contains("hostile") => 0.8,
        _ => 1.0,
    };
    
    (base_influence * faction_modifier).clamp(0.0, 1.0)
}

/// Generate settlement type from biome
pub fn determine_settlement_type_from_biome(biome_type: &str) -> String {
    match biome_type {
        "Grassland" => "village".to_string(),
        "Forest" => "outpost".to_string(),
        "Swamp" => "refuge".to_string(),
        "Mountain" => "stronghold".to_string(),
        "Desert" => "oasis".to_string(),
        "Water" => "port".to_string(),
        biome if biome.contains("Corrupted") => "cursed_refuge".to_string(),
        biome if biome.contains("Void") => "void_outpost".to_string(),
        _ => "settlement".to_string(),
    }
}
