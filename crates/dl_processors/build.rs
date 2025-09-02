use anyhow::Result;
use quote::quote;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use dl_analysis;

/// Main build function that processes analysis and generates Rust code
fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=build.rs");
    
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    
    // Get the analysis data from dl_analysis
    let ron_dir = dl_analysis::ron_dir();
    let index = dl_analysis::load_index();
    
    // Create output directories for generated code
    let regions_dir = out_dir.join("regions");
    let dungeons_dir = out_dir.join("dungeons");
    let settlements_dir = out_dir.join("settlements");
    
    fs::create_dir_all(&regions_dir)?;
    fs::create_dir_all(&dungeons_dir)?;
    fs::create_dir_all(&settlements_dir)?;
    
    // Process each entity type
    let mut regions = Vec::new();
    let mut dungeons = Vec::new();
    let mut settlements = Vec::new();
    
    for (uuid, path) in index {
        let content = fs::read_to_string(&path)?;
        
        // Try to parse as each type
        if let Ok(region) = ron::from_str::<RegionAnalysis>(&content) {
            generate_region_code(&region, &regions_dir)?;
            regions.push(region);
        } else if let Ok(dungeon) = ron::from_str::<DungeonAnalysis>(&content) {
            generate_dungeon_code(&dungeon, &dungeons_dir)?;
            dungeons.push(dungeon);
        } else if let Ok(settlement) = ron::from_str::<SettlementAnalysis>(&content) {
            generate_settlement_code(&settlement, &settlements_dir)?;
            settlements.push(settlement);
        }
    }
    
    // Generate mod files
    generate_regions_mod(&regions, &regions_dir)?;
    generate_dungeons_mod(&dungeons, &dungeons_dir)?;
    generate_settlements_mod(&settlements, &settlements_dir)?;
    
    // Generate main module file
    generate_main_mod(&out_dir)?;
    
    Ok(())
}

// Re-define the structures from dl_analysis (in real code, we'd share these)
#[derive(Debug, serde::Deserialize)]
struct RegionAnalysis {
    uuid: String,
    name: String,
    hex_tiles: Vec<HexTile>,
    biome_distribution: HashMap<String, u32>,
    corruption_level: f32,
    settlements: Vec<String>,
    dungeons: Vec<String>,
    rivers: Vec<(i32, i32)>,
    trails: Vec<(i32, i32)>,
}

#[derive(Debug, serde::Deserialize)]
struct HexTile {
    uuid: String,
    q: i32,
    r: i32,
    biome: String,
    features: Vec<String>,
}

#[derive(Debug, serde::Deserialize)]
struct SettlementAnalysis {
    uuid: String,
    name: String,
    location: (i32, i32),
    region: String,
    population_estimate: u32,
    threat_level: String,
    notable_features: Vec<String>,
}

#[derive(Debug, serde::Deserialize)]
struct DungeonAnalysis {
    uuid: String,
    name: String,
    areas: Vec<DungeonArea>,
    total_depth: u32,
    horror_themes: Vec<String>,
    boss_entities: Vec<String>,
}

#[derive(Debug, serde::Deserialize)]
struct DungeonArea {
    uuid: String,
    level: u32,
    room_count: u32,
    encounter_density: String,
    treasure_assessment: String,
}

/// Generate code for a region
fn generate_region_code(region: &RegionAnalysis, regions_dir: &Path) -> Result<()> {
    let region_uuid = sanitize_uuid(&region.uuid);
    let region_dir = regions_dir.join(&region_uuid);
    fs::create_dir_all(&region_dir)?;
    
    // Generate mod.rs for the region
    let mut hex_modules = Vec::new();
    
    for hex_tile in &region.hex_tiles {
        let hex_uuid = sanitize_uuid(&hex_tile.uuid);
        hex_modules.push(hex_uuid.clone());
        
        // Generate individual hex tile file
        let hex_code = generate_hex_tile_code(hex_tile, &region.name);
        let hex_path = region_dir.join(format!("{}.rs", hex_uuid));
        fs::write(hex_path, hex_code)?;
    }
    
    // Generate region mod.rs
    let mod_code = generate_region_mod_code(&region_uuid, &region.name, &hex_modules);
    let mod_path = region_dir.join("mod.rs");
    fs::write(mod_path, mod_code)?;
    
    Ok(())
}

/// Generate code for a hex tile
fn generate_hex_tile_code(hex: &HexTile, region_name: &str) -> String {
    let hex_uuid = sanitize_uuid(&hex.uuid);
    let biome_ident = sanitize_ident(&hex.biome);
    let features: Vec<String> = hex.features.iter()
        .map(|f| format!("\"{}\"", f))
        .collect();
    let features_array = features.join(", ");
    
    format!(r#"//! Hex tile {} in region {}
//! Position: ({}, {})

use bevy::prelude::*;
use crate::components::*;

/// Spawn this hex tile into the world
pub fn spawn_hex_tile(commands: &mut Commands) -> Entity {{
    commands.spawn((
        HexPosition {{ q: {}, r: {} }},
        HexBiome::{},
        HexFeatures {{ features: vec![{}] }},
        HexId("{}")
    )).id()
}}

/// Get the static data for this hex tile
pub const HEX_DATA: HexData = HexData {{
    uuid: "{}",
    q: {},
    r: {},
    biome: "{}",
}};
"#, 
        hex_uuid, region_name,
        hex.q, hex.r,
        hex.q, hex.r,
        biome_ident,
        features_array,
        hex.uuid,
        hex.uuid,
        hex.q,
        hex.r,
        hex.biome,
    )
}

/// Generate mod.rs for a region
fn generate_region_mod_code(region_uuid: &str, region_name: &str, hex_modules: &[String]) -> String {
    let mod_declarations: Vec<String> = hex_modules.iter()
        .map(|m| format!("pub mod {};", m))
        .collect();
    let mod_list = mod_declarations.join("\n");
    
    let spawn_calls: Vec<String> = hex_modules.iter()
        .map(|m| format!("    {}::spawn_hex_tile(commands);", m))
        .collect();
    let spawn_list = spawn_calls.join("\n");
    
    format!(r#"//! Region: {}
//! UUID: {}

use bevy::prelude::*;

{}

/// Spawn all hex tiles for this region
pub fn spawn_region(commands: &mut Commands) {{
{}
}}
"#, 
        region_name, region_uuid,
        mod_list,
        spawn_list
    )
}

/// Generate code for a dungeon
fn generate_dungeon_code(dungeon: &DungeonAnalysis, dungeons_dir: &Path) -> Result<()> {
    let dungeon_uuid = sanitize_uuid(&dungeon.uuid);
    let dungeon_dir = dungeons_dir.join(&dungeon_uuid);
    fs::create_dir_all(&dungeon_dir)?;
    
    // Generate mod.rs for the dungeon
    let mut area_modules = Vec::new();
    
    for area in &dungeon.areas {
        let area_uuid = sanitize_uuid(&area.uuid);
        area_modules.push(area_uuid.clone());
        
        // Generate individual area file
        let area_code = generate_dungeon_area_code(area, &dungeon.name);
        let area_path = dungeon_dir.join(format!("{}.rs", area_uuid));
        fs::write(area_path, area_code)?;
    }
    
    // Generate dungeon mod.rs
    let mod_code = generate_dungeon_mod_code(&dungeon_uuid, &dungeon.name, &area_modules);
    let mod_path = dungeon_dir.join("mod.rs");
    fs::write(mod_path, mod_code)?;
    
    Ok(())
}

/// Generate code for a dungeon area
fn generate_dungeon_area_code(area: &DungeonArea, dungeon_name: &str) -> String {
    let area_uuid = sanitize_uuid(&area.uuid);
    
    format!(r#"//! Dungeon area {} in {}
//! Level: {}

use bevy::prelude::*;
use crate::components::*;

/// Spawn this dungeon area into the world
pub fn spawn_dungeon_area(commands: &mut Commands) -> Entity {{
    commands.spawn((
        DungeonLevel {{ level: {} }},
        DungeonRooms {{ count: {} }},
        EncounterDensity("{}".to_string()),
        TreasureLevel("{}".to_string()),
        DungeonAreaId("{}")
    )).id()
}}

/// Get the static data for this dungeon area
pub const AREA_DATA: DungeonAreaData = DungeonAreaData {{
    uuid: "{}",
    level: {},
    room_count: {},
    encounter_density: "{}",
    treasure_assessment: "{}",
}};
"#, 
        area_uuid, dungeon_name,
        area.level,
        area.level,
        area.room_count,
        area.encounter_density,
        area.treasure_assessment,
        area.uuid,
        area.uuid,
        area.level,
        area.room_count,
        area.encounter_density,
        area.treasure_assessment,
    )
}

/// Generate mod.rs for a dungeon
fn generate_dungeon_mod_code(dungeon_uuid: &str, dungeon_name: &str, area_modules: &[String]) -> String {
    let mod_declarations: Vec<String> = area_modules.iter()
        .map(|m| format!("pub mod {};", m))
        .collect();
    let mod_list = mod_declarations.join("\n");
    
    let spawn_calls: Vec<String> = area_modules.iter()
        .map(|m| format!("    {}::spawn_dungeon_area(commands);", m))
        .collect();
    let spawn_list = spawn_calls.join("\n");
    
    format!(r#"//! Dungeon: {}
//! UUID: {}

use bevy::prelude::*;

{}

/// Spawn all areas for this dungeon
pub fn spawn_dungeon(commands: &mut Commands) {{
{}
}}
"#, 
        dungeon_name, dungeon_uuid,
        mod_list,
        spawn_list
    )
}

/// Generate code for a settlement
fn generate_settlement_code(settlement: &SettlementAnalysis, settlements_dir: &Path) -> Result<()> {
    let settlement_uuid = sanitize_uuid(&settlement.uuid);
    
    let features: Vec<String> = settlement.notable_features.iter()
        .map(|f| format!("\"{}\"", f))
        .collect();
    let features_array = features.join(", ");
    
    let code = format!(r#"//! Settlement: {}
//! UUID: {}

use bevy::prelude::*;
use crate::components::*;

/// Spawn this settlement into the world
pub fn spawn_settlement(commands: &mut Commands) -> Entity {{
    commands.spawn((
        SettlementPosition {{ q: {}, r: {} }},
        SettlementName("{}".to_string()),
        SettlementRegion("{}".to_string()),
        Population({}),
        ThreatLevel("{}".to_string()),
        SettlementFeatures {{ features: vec![{}] }},
        SettlementId("{}")
    )).id()
}}

/// Get the static data for this settlement
pub const SETTLEMENT_DATA: SettlementData = SettlementData {{
    uuid: "{}",
    name: "{}",
    location: ({}, {}),
    region: "{}",
    population_estimate: {},
    threat_level: "{}",
}};
"#, 
        settlement.name, settlement_uuid,
        settlement.location.0, settlement.location.1,
        settlement.name,
        settlement.region,
        settlement.population_estimate,
        settlement.threat_level,
        features_array,
        settlement.uuid,
        settlement.uuid,
        settlement.name,
        settlement.location.0, settlement.location.1,
        settlement.region,
        settlement.population_estimate,
        settlement.threat_level,
    );
    
    let path = settlements_dir.join(format!("{}.rs", settlement_uuid));
    fs::write(path, code)?;
    
    Ok(())
}

/// Generate regions/mod.rs
fn generate_regions_mod(regions: &[RegionAnalysis], regions_dir: &Path) -> Result<()> {
    let mod_declarations: Vec<String> = regions.iter()
        .map(|r| format!("pub mod {};", sanitize_uuid(&r.uuid)))
        .collect();
    let mod_list = mod_declarations.join("\n");
    
    let spawn_calls: Vec<String> = regions.iter()
        .map(|r| format!("    {}::spawn_region(commands);", sanitize_uuid(&r.uuid)))
        .collect();
    let spawn_list = spawn_calls.join("\n");
    
    let code = format!(r#"//! All regions in the world

use bevy::prelude::*;

{}

/// Spawn all regions
pub fn spawn_all_regions(commands: &mut Commands) {{
{}
}}
"#, mod_list, spawn_list);
    
    let path = regions_dir.join("mod.rs");
    fs::write(path, code)?;
    
    Ok(())
}

/// Generate dungeons/mod.rs
fn generate_dungeons_mod(dungeons: &[DungeonAnalysis], dungeons_dir: &Path) -> Result<()> {
    let mod_declarations: Vec<String> = dungeons.iter()
        .map(|d| format!("pub mod {};", sanitize_uuid(&d.uuid)))
        .collect();
    let mod_list = mod_declarations.join("\n");
    
    let spawn_calls: Vec<String> = dungeons.iter()
        .map(|d| format!("    {}::spawn_dungeon(commands);", sanitize_uuid(&d.uuid)))
        .collect();
    let spawn_list = spawn_calls.join("\n");
    
    let code = format!(r#"//! All dungeons in the world

use bevy::prelude::*;

{}

/// Spawn all dungeons
pub fn spawn_all_dungeons(commands: &mut Commands) {{
{}
}}
"#, mod_list, spawn_list);
    
    let path = dungeons_dir.join("mod.rs");
    fs::write(path, code)?;
    
    Ok(())
}

/// Generate settlements/mod.rs
fn generate_settlements_mod(settlements: &[SettlementAnalysis], settlements_dir: &Path) -> Result<()> {
    let mod_declarations: Vec<String> = settlements.iter()
        .map(|s| format!("pub mod {};", sanitize_uuid(&s.uuid)))
        .collect();
    let mod_list = mod_declarations.join("\n");
    
    let spawn_calls: Vec<String> = settlements.iter()
        .map(|s| format!("    {}::spawn_settlement(commands);", sanitize_uuid(&s.uuid)))
        .collect();
    let spawn_list = spawn_calls.join("\n");
    
    let code = format!(r#"//! All settlements in the world

use bevy::prelude::*;

{}

/// Spawn all settlements
pub fn spawn_all_settlements(commands: &mut Commands) {{
{}
}}
"#, mod_list, spawn_list);
    
    let path = settlements_dir.join("mod.rs");
    fs::write(path, code)?;
    
    Ok(())
}

/// Generate main mod.rs
fn generate_main_mod(out_dir: &Path) -> Result<()> {
    let code = r#"//! Generated world data

pub mod regions;
pub mod dungeons;
pub mod settlements;

use bevy::prelude::*;

/// Spawn the entire world
pub fn spawn_world(commands: &mut Commands) {
    regions::spawn_all_regions(commands);
    dungeons::spawn_all_dungeons(commands);
    settlements::spawn_all_settlements(commands);
}
"#;
    
    let path = out_dir.join("mod.rs");
    fs::write(path, code)?;
    
    Ok(())
}

/// Sanitize a UUID to be a valid Rust identifier
fn sanitize_uuid(uuid: &str) -> String {
    uuid.replace('-', "_")
}

/// Sanitize a string to be a valid Rust identifier
fn sanitize_ident(s: &str) -> String {
    // Convert to PascalCase
    s.split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<String>()
        .replace('-', "")
        .replace('\'', "")
}
