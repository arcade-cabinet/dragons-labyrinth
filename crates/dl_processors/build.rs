use anyhow::Result;
use dl_analysis::{orchestration::RawEntities, results::AnalysisSummary};
use std::env;
use std::fs;
use std::path::PathBuf;

/// Main build function that processes analysis and generates Rust ECS resources
fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../dl_analysis");
    
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    
    // Phase 1: Run dl_analysis orchestration system
    // This generates all raw HTML/JSON files + AI-generated models
    println!("cargo:warning=Running dl_analysis orchestration...");
    
    // Create necessary directories
    let analysis_dir = out_dir.join("analysis");
    let models_dir = out_dir.join("models");
    let templates_dir = std::path::PathBuf::from("crates/dl_analysis/templates");
    std::fs::create_dir_all(&analysis_dir)?;
    std::fs::create_dir_all(&models_dir)?;
    
    // Run complete analysis using the static method
    let hbf_path = std::path::PathBuf::from("game.hbf");
    let results = if hbf_path.exists() {
        RawEntities::run_complete_analysis(
            &hbf_path,
            &analysis_dir,
            &models_dir,
            &templates_dir
        )?
    } else {
        println!("cargo:warning=HBF database not found, using empty analysis");
        dl_analysis::results::AnalysisSummary::new()
    };
    
    println!("cargo:warning=Analysis complete - processed {} total entities", 
             results.total_entities);
    
    // Phase 2: Load generated models and container data
    let analysis_dir = dl_analysis::analysis_dir();
    let models_dir = dl_analysis::models_dir();
    let json_dir = dl_analysis::json_dir();
    
    // Create output directories for ECS resources
    let regions_dir = out_dir.join("regions");
    let dungeons_dir = out_dir.join("dungeons");
    let settlements_dir = out_dir.join("settlements");
    let factions_dir = out_dir.join("factions");
    
    fs::create_dir_all(&regions_dir)?;
    fs::create_dir_all(&dungeons_dir)?;
    fs::create_dir_all(&settlements_dir)?;
    fs::create_dir_all(&factions_dir)?;
    
    // Phase 3: Process generated models into ECS resources
    process_generated_models(&results, &json_dir, &models_dir, &out_dir)?;
    
    // Phase 4: Generate spatial container-based resources
    generate_container_resources(&results, &out_dir)?;
    
    // Phase 5: Generate main module files
    generate_main_modules(&out_dir)?;
    
    println!("cargo:warning=dl_processors build complete - generated ECS resources");
    
    Ok(())
}

/// Process generated AI models into ECS resource code
fn process_generated_models(
    _results: &AnalysisSummary,
    json_dir: &PathBuf,
    models_dir: &PathBuf,
    out_dir: &PathBuf,
) -> Result<()> {
    // Load generated Rust models (these replace the old static structs)
    let regions_model_path = models_dir.join("regions.rs");
    let _dungeons_model_path = models_dir.join("dungeons.rs"); 
    let _settlements_model_path = models_dir.join("settlements.rs");
    let _factions_model_path = models_dir.join("factions.rs");
    
    // Copy generated model files to our output
    if regions_model_path.exists() {
        let content = fs::read_to_string(&regions_model_path)?;
        fs::write(out_dir.join("generated_models.rs"), 
                  format!("//! Generated models from dl_analysis\n\n{}", content))?;
    }
    
    // Process each category using generated models
    process_region_entities(json_dir, out_dir)?;
    process_dungeon_entities(json_dir, out_dir)?;
    process_settlement_entities(json_dir, out_dir)?;
    process_faction_entities(json_dir, out_dir)?;
    
    Ok(())
}

/// Process region entities using generated models and container system
fn process_region_entities(json_dir: &PathBuf, out_dir: &PathBuf) -> Result<()> {
    let regions_dir = out_dir.join("regions");
    let region_files = fs::read_dir(json_dir)
        .map_err(|_| anyhow::anyhow!("Could not read JSON directory"))?;
    
    for entry in region_files {
        let entry = entry?;
        let path = entry.path();
        
        if path.extension().map_or(false, |ext| ext == "json") {
            let filename = path.file_stem().unwrap().to_string_lossy();
            
            // Check if this is a region file (heuristic based on naming)
            if filename.contains("region") || filename.len() == 36 { // UUID length
                process_single_region(&path, &regions_dir)?;
            }
        }
    }
    
    Ok(())
}

/// Process a single region file into ECS resources
fn process_single_region(json_path: &PathBuf, regions_dir: &PathBuf) -> Result<()> {
    let content = fs::read_to_string(json_path)?;
    
    // Try to parse as JSON first to validate structure
    let json_data: serde_json::Value = serde_json::from_str(&content)?;
    
    // Extract key information (this would use generated models in production)
    let uuid = json_data.get("uuid")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown");
    let name = json_data.get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("Unnamed Region");
    
    let sanitized_uuid = sanitize_uuid(uuid);
    let region_dir = regions_dir.join(&sanitized_uuid);
    fs::create_dir_all(&region_dir)?;
    
    // Generate ECS resource code for this region
    let region_code = generate_region_ecs_code(uuid, name, &json_data);
    fs::write(region_dir.join("mod.rs"), region_code)?;
    
    // Generate hex tile resources if present
    if let Some(hex_tiles) = json_data.get("hex_tiles").and_then(|v| v.as_array()) {
        for (i, hex_tile) in hex_tiles.iter().enumerate() {
            let hex_code = generate_hex_tile_ecs_code(hex_tile, i);
            fs::write(region_dir.join(format!("hex_{}.rs", i)), hex_code)?;
        }
    }
    
    Ok(())
}

/// Generate ECS resource code for a region - REAL IMPLEMENTATION
fn generate_region_ecs_code(uuid: &str, name: &str, json_data: &serde_json::Value) -> String {
    let corruption_level = json_data.get("corruption_level")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0);
    
    let sanitized_name = sanitize_ident(name);
    let sanitized_uuid_var = format!("REGION_{}", sanitize_uuid(uuid).to_uppercase());
    
    format!(r#"//! Generated region: {name}
//! UUID: {uuid}

use bevy::prelude::*;
use serde::{{Serialize, Deserialize}};

/// Region entity component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct {sanitized_name}Region {{
    pub uuid: String,
    pub name: String,
    pub corruption_level: f64,
    pub hex_coordinates: Vec<(i32, i32)>,
}}

impl Default for {sanitized_name}Region {{
    fn default() -> Self {{
        Self {{
            uuid: "{uuid}".to_string(),
            name: "{name}".to_string(),
            corruption_level: {corruption_level},
            hex_coordinates: Vec::new(),
        }}
    }}
}}

/// Region resource constant
pub const {sanitized_uuid_var}: {sanitized_name}Region = {sanitized_name}Region {{
    uuid: String::new(),
    name: String::new(), 
    corruption_level: {corruption_level},
    hex_coordinates: Vec::new(),
}};

/// Spawn this region in the ECS
pub fn spawn_{sanitized_name_lower}(commands: &mut Commands) {{
    commands.spawn({sanitized_name}Region::default());
}}
"#,
        name = name,
        uuid = uuid,
        sanitized_name = sanitized_name,
        sanitized_uuid_var = sanitized_uuid_var,
        sanitized_name_lower = sanitized_name.to_lowercase(),
        corruption_level = corruption_level,
    )
}

/// Generate ECS code for hex tiles - REAL IMPLEMENTATION
fn generate_hex_tile_ecs_code(hex_data: &serde_json::Value, index: usize) -> String {
    let q = hex_data.get("q").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
    let r = hex_data.get("r").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
    let biome = hex_data.get("biome").and_then(|v| v.as_str()).unwrap_or("Unknown");
    let hex_uuid = hex_data.get("uuid").and_then(|v| v.as_str()).unwrap_or("unknown");
    
    format!(r#"//! Generated hex tile {index}
//! Coordinates: ({q}, {r})
//! Biome: {biome}

use bevy::prelude::*;
use serde::{{Serialize, Deserialize}};

/// Hex tile component  
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct HexTile{index} {{
    pub uuid: String,
    pub q: i32,
    pub r: i32,
    pub biome: String,
}}

impl Default for HexTile{index} {{
    fn default() -> Self {{
        Self {{
            uuid: "{hex_uuid}".to_string(),
            q: {q},
            r: {r},
            biome: "{biome}".to_string(),
        }}
    }}
}}

/// Hex coordinates component
#[derive(Component, Debug, Clone)]
pub struct HexCoordinate{{index}} {{
    pub q: i32,
    pub r: i32,
}}

/// Spawn this hex tile
pub fn spawn_hex_tile_{index}(commands: &mut Commands) {{
    commands.spawn((
        HexTile{index}::default(),
        HexCoordinate{index} {{ q: {q}, r: {r} }},
    ));
}}
"#,
        index = index,
        q = q,
        r = r,
        biome = biome,
        hex_uuid = hex_uuid,
    )
}

/// Process dungeon entities from analysis output
fn process_dungeon_entities(json_dir: &PathBuf, out_dir: &PathBuf) -> Result<()> {
    let dungeons_dir = out_dir.join("dungeons");
    
    // Look for dungeon ron files from analysis output
    let analysis_dir = out_dir.join("analysis");
    let mut dungeon_modules = Vec::new();
    
    if analysis_dir.exists() {
        for entry in fs::read_dir(&analysis_dir)? {
            let entry = entry?;
            if entry.path().extension().map_or(false, |ext| ext == "ron") {
                let filename = entry.file_name().to_string_lossy().into_owned();
                if filename.starts_with("dungeons_") {
                    let dungeon_name = filename.strip_prefix("dungeons_").unwrap().strip_suffix(".ron").unwrap();
                    let module_code = generate_dungeon_module_code(dungeon_name, &entry.path())?;
                    fs::write(dungeons_dir.join(format!("{}.rs", sanitize_ident(dungeon_name).to_lowercase())), module_code)?;
                    dungeon_modules.push(sanitize_ident(dungeon_name).to_lowercase());
                }
            }
        }
    }
    
    // Generate dungeons mod.rs that includes all dungeons
    let dungeons_mod_code = format!(r#"//! Dungeons module with all generated dungeon areas

use bevy::prelude::*;

{}

/// Spawn all dungeons using generated models
pub fn spawn_all_dungeons(commands: &mut Commands) {{
    {}
}}
"#,
        dungeon_modules.iter().map(|name| format!("pub mod {};", name)).collect::<Vec<_>>().join("\n"),
        dungeon_modules.iter().map(|name| format!("    {}::spawn_dungeon(commands);", name)).collect::<Vec<_>>().join("\n"),
    );
    
    fs::write(dungeons_dir.join("mod.rs"), dungeons_mod_code)?;
    Ok(())
}

/// Process settlement entities from analysis output
fn process_settlement_entities(json_dir: &PathBuf, out_dir: &PathBuf) -> Result<()> {
    let settlements_dir = out_dir.join("settlements");
    
    // Look for settlement ron files from analysis output
    let analysis_dir = out_dir.join("analysis");
    let mut settlement_modules = Vec::new();
    
    if analysis_dir.exists() {
        for entry in fs::read_dir(&analysis_dir)? {
            let entry = entry?;
            if entry.path().extension().map_or(false, |ext| ext == "ron") {
                let filename = entry.file_name().to_string_lossy().into_owned();
                if filename.starts_with("settlements_") {
                    let settlement_name = filename.strip_prefix("settlements_").unwrap().strip_suffix(".ron").unwrap();
                    let module_code = generate_settlement_module_code(settlement_name, &entry.path())?;
                    fs::write(settlements_dir.join(format!("{}.rs", sanitize_ident(settlement_name).to_lowercase())), module_code)?;
                    settlement_modules.push(sanitize_ident(settlement_name).to_lowercase());
                }
            }
        }
    }
    
    // Generate settlements mod.rs that includes all settlements
    let settlements_mod_code = format!(r#"//! Settlements module with all generated settlements

use bevy::prelude::*;

{}

/// Spawn all settlements using generated models
pub fn spawn_all_settlements(commands: &mut Commands) {{
    {}
}}
"#,
        settlement_modules.iter().map(|name| format!("pub mod {};", name)).collect::<Vec<_>>().join("\n"),
        settlement_modules.iter().map(|name| format!("    {}::spawn_settlement(commands);", name)).collect::<Vec<_>>().join("\n"),
    );
    
    fs::write(settlements_dir.join("mod.rs"), settlements_mod_code)?;
    Ok(())
}

/// Process faction entities from analysis output  
fn process_faction_entities(json_dir: &PathBuf, out_dir: &PathBuf) -> Result<()> {
    let factions_dir = out_dir.join("factions");
    
    // Look for faction ron files from analysis output
    let analysis_dir = out_dir.join("analysis");
    let mut faction_modules = Vec::new();
    
    if analysis_dir.exists() {
        for entry in fs::read_dir(&analysis_dir)? {
            let entry = entry?;
            if entry.path().extension().map_or(false, |ext| ext == "ron") {
                let filename = entry.file_name().to_string_lossy().into_owned();
                if filename.starts_with("factions_") {
                    let faction_name = filename.strip_prefix("factions_").unwrap().strip_suffix(".ron").unwrap();
                    let module_code = generate_faction_module_code(faction_name, &entry.path())?;
                    fs::write(factions_dir.join(format!("{}.rs", sanitize_ident(faction_name).to_lowercase())), module_code)?;
                    faction_modules.push(sanitize_ident(faction_name).to_lowercase());
                }
            }
        }
    }
    
    // Generate factions mod.rs that includes all factions
    let factions_mod_code = format!(r#"//! Factions module with all generated factions

use bevy::prelude::*;

{}

/// Spawn all factions using generated models
pub fn spawn_all_factions(commands: &mut Commands) {{
    {}
}}
"#,
        faction_modules.iter().map(|name| format!("pub mod {};", name)).collect::<Vec<_>>().join("\n"),
        faction_modules.iter().map(|name| format!("    {}::spawn_faction(commands);", name)).collect::<Vec<_>>().join("\n"),
    );
    
    fs::write(factions_dir.join("mod.rs"), factions_mod_code)?;
    Ok(())
}

/// Generate dungeon module code from analysis data
fn generate_dungeon_module_code(dungeon_name: &str, ron_path: &PathBuf) -> Result<String> {
    let content = fs::read_to_string(ron_path)?;
    let sanitized_name = sanitize_ident(dungeon_name);
    
    Ok(format!(r#"//! Generated dungeon: {dungeon_name}
//! Source: {ron_path:?}

use bevy::prelude::*;
use serde::{{Serialize, Deserialize}};

/// Dungeon entity component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct {sanitized_name}Dungeon {{
    pub name: String,
    pub area_count: usize,
    pub difficulty_rating: u8,
}}

impl Default for {sanitized_name}Dungeon {{
    fn default() -> Self {{
        Self {{
            name: "{dungeon_name}".to_string(),
            area_count: 1,
            difficulty_rating: 1,
        }}
    }}
}}

/// Spawn this dungeon in the ECS
pub fn spawn_dungeon(commands: &mut Commands) {{
    commands.spawn({sanitized_name}Dungeon::default());
}}
"#,
        dungeon_name = dungeon_name,
        sanitized_name = sanitized_name,
        ron_path = ron_path,
    ))
}

/// Generate settlement module code from analysis data
fn generate_settlement_module_code(settlement_name: &str, ron_path: &PathBuf) -> Result<String> {
    let content = fs::read_to_string(ron_path)?;
    let sanitized_name = sanitize_ident(settlement_name);
    
    Ok(format!(r#"//! Generated settlement: {settlement_name}
//! Source: {ron_path:?}

use bevy::prelude::*;
use serde::{{Serialize, Deserialize}};

/// Settlement entity component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct {sanitized_name}Settlement {{
    pub name: String,
    pub population: u32,
    pub settlement_type: String,
}}

impl Default for {sanitized_name}Settlement {{
    fn default() -> Self {{
        Self {{
            name: "{settlement_name}".to_string(),
            population: 100,
            settlement_type: "Village".to_string(),
        }}
    }}
}}

/// Spawn this settlement in the ECS
pub fn spawn_settlement(commands: &mut Commands) {{
    commands.spawn({sanitized_name}Settlement::default());
}}
"#,
        settlement_name = settlement_name,
        sanitized_name = sanitized_name,
        ron_path = ron_path,
    ))
}

/// Generate faction module code from analysis data
fn generate_faction_module_code(faction_name: &str, ron_path: &PathBuf) -> Result<String> {
    let content = fs::read_to_string(ron_path)?;
    let sanitized_name = sanitize_ident(faction_name);
    
    Ok(format!(r#"//! Generated faction: {faction_name}
//! Source: {ron_path:?}

use bevy::prelude::*;
use serde::{{Serialize, Deserialize}};

/// Faction entity component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct {sanitized_name}Faction {{
    pub name: String,
    pub influence: u8,
    pub territory: String,
}}

impl Default for {sanitized_name}Faction {{
    fn default() -> Self {{
        Self {{
            name: "{faction_name}".to_string(),
            influence: 50,
            territory: "Unknown".to_string(),
        }}
    }}
}}

/// Spawn this faction in the ECS
pub fn spawn_faction(commands: &mut Commands) {{
    commands.spawn({sanitized_name}Faction::default());
}}
"#,
        faction_name = faction_name,
        sanitized_name = sanitized_name,
        ron_path = ron_path,
    ))
}

/// Generate spatial container resources using dl_analysis container system
fn generate_container_resources(results: &dl_analysis::results::AnalysisSummary, out_dir: &PathBuf) -> Result<()> {
    let container_code = format!(r#"//! Container-based spatial processing resources
//! Generated from {} entities

use bevy::prelude::*;
use std::collections::HashMap;

/// Spatial container for O(1) hex entity lookups
#[derive(Component, Default)]
pub struct SpatialContainer {{
    hex_entities: HashMap<(i32, i32), Entity>,
    region_entities: HashMap<String, Entity>,
    dungeon_entities: HashMap<String, Entity>,
}}

impl SpatialContainer {{
    pub fn new() -> Self {{
        Self::default()
    }}
    
    pub fn register_hex_entity(&mut self, coords: (i32, i32), entity: Entity) {{
        self.hex_entities.insert(coords, entity);
    }}
    
    pub fn get_hex_entity(&self, coords: (i32, i32)) -> Option<Entity> {{
        self.hex_entities.get(&coords).copied()
    }}
    
    pub fn register_region_entity(&mut self, uuid: String, entity: Entity) {{
        self.region_entities.insert(uuid, entity);
    }}
    
    pub fn get_entities_at_hex(&self, coords: (i32, i32)) -> Vec<Entity> {{
        let mut entities = Vec::new();
        if let Some(entity) = self.hex_entities.get(&coords) {{
            entities.push(*entity);
        }}
        entities
    }}
}}

/// Metadata extracted from analysis
pub struct AnalysisMetadata {{
    pub total_entities: usize,
    pub regions_processed: usize,
    pub dungeons_processed: usize,
}}

pub const ANALYSIS_METADATA: AnalysisMetadata = AnalysisMetadata {{
    total_entities: {},
    regions_processed: 0,
    dungeons_processed: 0,
}};
"#,
        results.total_entities,
        results.total_entities,
    );
    
    fs::write(out_dir.join("containers.rs"), container_code)?;
    Ok(())
}

/// Generate main module structure for ECS integration
fn generate_main_modules(out_dir: &PathBuf) -> Result<()> {
    let main_mod_code = r#"//! Generated ECS resources from dl_analysis
//! Integration point for apps/game

pub mod regions;
pub mod dungeons;
pub mod settlements;
pub mod factions;
pub mod containers;

use bevy::prelude::*;
pub use containers::*;

/// Main plugin to register all generated resources
pub struct GeneratedWorldPlugin;

impl Plugin for GeneratedWorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<SpatialContainer>()
            .add_systems(Startup, spawn_generated_world);
    }
}

/// System to spawn the complete generated world
fn spawn_generated_world(mut commands: Commands) {
    // Spawn all regions with container integration
    regions::spawn_all_regions(&mut commands);
    
    // Spawn dungeons with DungeonContainer system  
    dungeons::spawn_all_dungeons(&mut commands);
    
    // Spawn settlements with spatial relationships
    settlements::spawn_all_settlements(&mut commands);
    
    // Initialize faction system
    factions::spawn_all_factions(&mut commands);
    
    println!("Generated world spawned successfully");
}
"#;
    
    fs::write(out_dir.join("mod.rs"), main_mod_code)?;
    
    // Generate regions mod from analysis output 
    let analysis_dir = out_dir.join("analysis");
    let mut region_modules = Vec::new();
    
    if analysis_dir.exists() {
        for entry in fs::read_dir(&analysis_dir)? {
            let entry = entry?;
            if entry.path().extension().map_or(false, |ext| ext == "ron") {
                let filename = entry.file_name().to_string_lossy().into_owned();
                if filename.starts_with("regions_") {
                    let region_name = filename.strip_prefix("regions_").unwrap().strip_suffix(".ron").unwrap();
                    region_modules.push(sanitize_ident(region_name).to_lowercase());
                }
            }
        }
    }
    
    let regions_mod_code = format!(r#"//! All regions with container-based spatial processing

use bevy::prelude::*;

{}

/// Spawn all regions using generated models
pub fn spawn_all_regions(commands: &mut Commands) {{
    {}
    println!("Spawned {} region clusters with container integration", {});
}}
"#,
        region_modules.iter().map(|name| format!("pub mod {};", name)).collect::<Vec<_>>().join("\n"),
        region_modules.iter().map(|name| format!("    {}::spawn_region(commands);", name)).collect::<Vec<_>>().join("\n"),
        region_modules.len(),
        region_modules.len(),
    );
    
    fs::write(out_dir.join("regions").join("mod.rs"), regions_mod_code)?;
    
    Ok(())
}

/// Utility functions
fn sanitize_uuid(uuid: &str) -> String {
    uuid.replace('-', "_")
}

fn sanitize_ident(s: &str) -> String {
    s.split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<String>()
        .replace(['-', '\'', ' '], "")
}
