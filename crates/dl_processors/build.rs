use anyhow::Result;
use dl_analysis::{orchestration::RawEntities, results::GenerationResults};
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
    let orchestrator = RawEntities::new();
    let results = orchestrator.run_full_analysis()?;
    
    println!("cargo:warning=Analysis complete - processed {} entities", 
             results.summary.total_entities);
    
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
    results: &GenerationResults,
    json_dir: &PathBuf,
    models_dir: &PathBuf,
    out_dir: &PathBuf,
) -> Result<()> {
    // Load generated Rust models (these replace the old static structs)
    let regions_model_path = models_dir.join("regions.rs");
    let dungeons_model_path = models_dir.join("dungeons.rs"); 
    let settlements_model_path = models_dir.join("settlements.rs");
    let factions_model_path = models_dir.join("factions.rs");
    
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

/// Generate ECS resource code for a region using container-based spatial processing
fn generate_region_ecs_code(uuid: &str, name: &str, json_data: &serde_json::Value) -> String {
    let sanitized_name = sanitize_ident(name);
    
    // Extract spatial data for container processing
    let corruption_level = json_data.get("corruption_level")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0);
    
    format!(r#"//! Region: {}
//! UUID: {}
//! Generated ECS resources with container-based spatial processing

use bevy::prelude::*;
use crate::components::*;

/// Region resource component
#[derive(Resource, Debug, Clone)]
pub struct {}Region {{
    pub uuid: String,
    pub name: String, 
    pub corruption_level: f32,
    pub hex_tiles: Vec<Entity>,
    pub settlements: Vec<Entity>,
    pub dungeons: Vec<Entity>,
}}

impl Default for {}Region {{
    fn default() -> Self {{
        Self {{
            uuid: "{}".to_string(),
            name: "{}".to_string(),
            corruption_level: {:.2},
            hex_tiles: Vec::new(),
            settlements: Vec::new(),
            dungeons: Vec::new(),
        }}
    }}
}}

/// Spawn this region using container-based spatial indexing
pub fn spawn_region_with_containers(
    mut commands: Commands,
    mut region_resource: ResMut<{}Region>,
) {{
    // Create region entity with spatial container component
    let region_entity = commands.spawn((
        RegionId("{}".to_string()),
        RegionName("{}".to_string()),
        CorruptionLevel({:.2}),
        SpatialContainer::new(),
    )).id();
    
    // TODO: Load hex tiles using container system for O(1) lookups
    // TODO: Process settlements with spatial relationships
    // TODO: Process dungeons with container-based pathfinding
    
    println!("Spawned region: {} (UUID: {})", "{}", "{}");
}}

/// Get static metadata for this region
pub const REGION_METADATA: RegionMetadata = RegionMetadata {{
    uuid: "{}",
    name: "{}",
    base_corruption: {:.2},
}};
"#, 
        name, uuid,
        sanitized_name,
        sanitized_name,
        uuid, name, corruption_level,
        sanitized_name,
        uuid, name, corruption_level,
        name, uuid, uuid,
        uuid, name, corruption_level
    )
}

/// Generate ECS code for hex tiles with spatial container integration
fn generate_hex_tile_ecs_code(hex_data: &serde_json::Value, index: usize) -> String {
    let q = hex_data.get("q").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
    let r = hex_data.get("r").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
    let biome = hex_data.get("biome").and_then(|v| v.as_str()).unwrap_or("Unknown");
    let hex_uuid = hex_data.get("uuid").and_then(|v| v.as_str()).unwrap_or("unknown");
    
    let biome_variant = match biome.to_lowercase().as_str() {
        "wet meadow" => "WetMeadow",
        "ashen forest" => "AshenForest", 
        "flooded village" => "FloodedVillage",
        "black swamp" => "BlackSwamp",
        "fungal cathedral" => "FungalCathedral",
        "shadowed fen" => "ShadowedFen",
        "rust plains" => "RustPlains",
        "hollow hills" => "HollowHills",
        "corroded battleground" => "CorrodedBattleground",
        "famine fields" => "FamineFields",
        "bone forest" => "BoneForest",
        "desolate expanse" => "DesolateExpanse",
        "dragon scar" => "DragonScar",
        "abyssal chasm" => "AbyssalChasm",
        "final dread terrain" => "FinalDreadTerrain",
        _ => "WetMeadow", // fallback
    };
    
    format!(r#"//! Hex tile {} at position ({}, {})
//! Biome: {}

use bevy::prelude::*;
use crate::components::*;

/// Spawn this hex tile with spatial container integration
pub fn spawn_hex_tile_with_container(
    mut commands: Commands,
    container: &mut SpatialContainer,
) -> Entity {{
    let entity = commands.spawn((
        HexPosition {{ q: {}, r: {} }},
        HexBiome::{},
        HexId("{}".to_string()),
        BiomeFeatures::default(),
    )).id();
    
    // Register in spatial container for O(1) lookups
    container.register_hex_entity(({}, {}), entity);
    
    entity
}}

/// Static hex data for container queries
pub const HEX_STATIC_DATA: HexStaticData = HexStaticData {{
    uuid: "{}",
    q: {},
    r: {},
    biome: "{}",
}};
"#,
        index, q, r, biome,
        q, r, biome_variant, hex_uuid,
        q, r,
        hex_uuid, q, r, biome
    )
}

/// Process dungeon entities (placeholder for now)
fn process_dungeon_entities(json_dir: &PathBuf, out_dir: &PathBuf) -> Result<()> {
    // TODO: Implement dungeon processing with DungeonContainer spatial indexing
    let dungeons_dir = out_dir.join("dungeons");
    
    let placeholder_code = r#"//! Dungeons module - using generated models and container system
//! TODO: Implement DungeonContainer integration

use bevy::prelude::*;

/// Placeholder dungeon spawning system
pub fn spawn_all_dungeons(commands: &mut Commands) {
    // TODO: Use generated dungeon models
    // TODO: Implement DungeonContainer for area relationships
    // TODO: Add pathfinding with container spatial indexing
}
"#;
    
    fs::write(dungeons_dir.join("mod.rs"), placeholder_code)?;
    Ok(())
}

/// Process settlement entities (placeholder for now)
fn process_settlement_entities(json_dir: &PathBuf, out_dir: &PathBuf) -> Result<()> {
    // TODO: Implement settlement processing with generated models
    let settlements_dir = out_dir.join("settlements");
    
    let placeholder_code = r#"//! Settlements module - using generated models

use bevy::prelude::*;

/// Placeholder settlement spawning system
pub fn spawn_all_settlements(commands: &mut Commands) {
    // TODO: Use generated settlement models
    // TODO: Implement container-based settlement relationships
}
"#;
    
    fs::write(settlements_dir.join("mod.rs"), placeholder_code)?;
    Ok(())
}

/// Process faction entities (placeholder for now)
fn process_faction_entities(json_dir: &PathBuf, out_dir: &PathBuf) -> Result<()> {
    // TODO: Implement faction processing
    let factions_dir = out_dir.join("factions");
    
    let placeholder_code = r#"//! Factions module - using generated models

use bevy::prelude::*;

/// Placeholder faction spawning system  
pub fn spawn_all_factions(commands: &mut Commands) {
    // TODO: Use generated faction models
    // TODO: Implement faction relationship system
}
"#;
    
    fs::write(factions_dir.join("mod.rs"), placeholder_code)?;
    Ok(())
}

/// Generate spatial container resources using dl_analysis container system
fn generate_container_resources(results: &GenerationResults, out_dir: &PathBuf) -> Result<()> {
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
    regions_processed: {},
    dungeons_processed: {},
}};
"#,
        results.summary.total_entities,
        results.summary.total_entities,
        results.summary.regions_processed,
        results.summary.dungeons_processed,
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
    
    // Generate regions mod
    let regions_mod_code = r#"//! All regions with container-based spatial processing

use bevy::prelude::*;

/// Spawn all regions using generated models
pub fn spawn_all_regions(commands: &mut Commands) {
    // TODO: Iterate through all generated region modules
    // TODO: Use container system for spatial relationships
    println!("Spawning regions with container integration...");
}
"#;
    
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
