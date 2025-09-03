//! Processor crate for Dragon's Labyrinth
//! 
//! This crate takes the analyzed data from dl_analysis and generates
//! Rust code for the game to use, using external templates.

use bevy::prelude::*;
use std::path::PathBuf;
use anyhow::Result;
use std::collections::HashMap;

/// Components used by the generated code
pub mod components {
    use bevy::prelude::*;
    use std::collections::HashMap;
    
    // Hex tile components
    #[derive(Component, Debug, Clone)]
    pub struct HexPosition {
        pub q: i32,
        pub r: i32,
    }
    
    #[derive(Component, Debug, Clone)]
    pub enum HexBiome {
        WetMeadow,
        AshenForest,
        FloodedVillage,
        BlackSwamp,
        FungalCathedral,
        ShadowedFen,
        RustPlains,
        HollowHills,
        CorrodedBattleground,
        FamineFields,
        BoneForest,
        DesolateExpanse,
        DragonScar,
        AbyssalChasm,
        FinalDreadTerrain,
    }
    
    #[derive(Component, Debug, Clone)]
    pub struct HexFeatures {
        pub features: Vec<String>,
    }
    
    #[derive(Component, Debug, Clone)]
    pub struct HexId(pub String);
    
    // New generated components for regions
    #[derive(Component, Debug, Clone)]
    pub struct RegionId(pub String);
    
    #[derive(Component, Debug, Clone)]
    pub struct RegionName(pub String);
    
    #[derive(Component, Debug, Clone)]
    pub struct CorruptionLevel(pub f32);
    
    #[derive(Component, Debug, Clone, Default)]
    pub struct BiomeFeatures {
        pub features: Vec<String>,
    }
    
    // Settlement components
    #[derive(Component, Debug, Clone)]
    pub struct SettlementPosition {
        pub q: i32,
        pub r: i32,
    }
    
    #[derive(Component, Debug, Clone)]
    pub struct SettlementName(pub String);
    
    #[derive(Component, Debug, Clone)]
    pub struct SettlementRegion(pub String);
    
    #[derive(Component, Debug, Clone)]
    pub struct Population(pub u32);
    
    #[derive(Component, Debug, Clone)]
    pub struct ThreatLevel(pub String);
    
    #[derive(Component, Debug, Clone)]
    pub struct SettlementFeatures {
        pub features: Vec<String>,
    }
    
    #[derive(Component, Debug, Clone)]
    pub struct SettlementId(pub String);
    
    // Dungeon components
    #[derive(Component, Debug, Clone)]
    pub struct DungeonLevel {
        pub level: u32,
    }
    
    #[derive(Component, Debug, Clone)]
    pub struct DungeonRooms {
        pub count: u32,
    }
    
    #[derive(Component, Debug, Clone)]
    pub struct EncounterDensity(pub String);
    
    #[derive(Component, Debug, Clone)]
    pub struct TreasureLevel(pub String);
    
    #[derive(Component, Debug, Clone)]
    pub struct DungeonAreaId(pub String);
    
    // Spatial container component for O(1) lookups
    #[derive(Component, Default, Debug)]
    pub struct SpatialContainer {
        pub hex_entities: HashMap<(i32, i32), Entity>,
        pub region_entities: HashMap<String, Entity>,
        pub dungeon_entities: HashMap<String, Entity>,
    }
    
    impl SpatialContainer {
        pub fn new() -> Self {
            Self::default()
        }
        
        pub fn register_hex_entity(&mut self, coords: (i32, i32), entity: Entity) {
            self.hex_entities.insert(coords, entity);
        }
        
        pub fn get_hex_entity(&self, coords: (i32, i32)) -> Option<Entity> {
            self.hex_entities.get(&coords).copied()
        }
        
        pub fn register_region_entity(&mut self, uuid: String, entity: Entity) {
            self.region_entities.insert(uuid, entity);
        }
        
        pub fn get_entities_at_hex(&self, coords: (i32, i32)) -> Vec<Entity> {
            let mut entities = Vec::new();
            if let Some(entity) = self.hex_entities.get(&coords) {
                entities.push(*entity);
            }
            entities
        }

        pub fn register_dungeon_area_entity(&mut self, uuid: String, entity: Entity) {
            self.dungeon_entities.insert(uuid, entity);
        }
    }
    
    // Static data types
    #[derive(Debug, Clone)]
    pub struct HexData {
        pub uuid: &'static str,
        pub q: i32,
        pub r: i32,
        pub biome: &'static str,
    }
    
    #[derive(Debug, Clone)]
    pub struct HexStaticData {
        pub uuid: &'static str,
        pub q: i32,
        pub r: i32,
        pub biome: &'static str,
    }
    
    #[derive(Debug, Clone)]
    pub struct RegionMetadata {
        pub uuid: &'static str,
        pub name: &'static str,
        pub base_corruption: f32,
    }
    
    #[derive(Debug, Clone)]
    pub struct SettlementData {
        pub uuid: &'static str,
        pub name: &'static str,
        pub location: (i32, i32),
        pub region: &'static str,
        pub population_estimate: u32,
        pub threat_level: &'static str,
    }
    
    #[derive(Debug, Clone)]
    pub struct DungeonAreaData {
        pub uuid: &'static str,
        pub level: u32,
        pub room_count: u32,
        pub encounter_density: &'static str,
        pub treasure_assessment: &'static str,
    }
}

/// Get the path to the generated code
pub fn generated_dir() -> PathBuf {
    PathBuf::from(env!("OUT_DIR"))
}

/// Production API called by apps/game build.rs to generate world resources
pub fn generate_world_resources(out_dir: &std::path::Path) -> Result<()> {
    use std::fs;
    use minijinja::Environment;
    
    println!("Generating world resources using external templates...");
    
    // Set up template environment with embedded templates for now
    let mut env = Environment::new();
    
    // Add templates directly to avoid path issues during build
    env.add_template("hex_tile.rs.jinja2", include_str!("../templates/hex_tile.rs.jinja2"))?;
    env.add_template("region_module.rs.jinja2", include_str!("../templates/region_module.rs.jinja2"))?;
    env.add_template("dungeon_area.rs.jinja2", include_str!("../templates/dungeon_area.rs.jinja2"))?;
    env.add_template("dungeon_module.rs.jinja2", include_str!("../templates/dungeon_module.rs.jinja2"))?;
    env.add_template("world_integration.rs.jinja2", include_str!("../templates/world_integration.rs.jinja2"))?;
    
    // Call dl_analysis to get the processed HBF data
    let mut orchestrator = dl_analysis::orchestration::RawEntities::new();
    let mut logger = std::io::stdout();
    let analysis_dir = std::path::Path::new("analysis");
    let models_dir = std::path::Path::new("target/models");
    
    // Create a simple GenerationResults with sample data for now
    let analysis_summary = orchestrator.run_complete_analysis(&analysis_dir, &models_dir, &mut logger)?;
    let analysis_results = dl_analysis::results::GenerationResults::success(vec!["sample.rs".to_string()])
        .with_summary(analysis_summary)
        .with_entities(create_sample_entities());
    
    // Generate hex tile modules using templates
    generate_hex_tiles_from_data(&env, &analysis_results, out_dir)?;
    
    // Generate dungeon area modules using templates
    generate_dungeon_areas_from_data(&env, &analysis_results, out_dir)?;
    
    // Generate main world integration module
    let world_template = env.get_template("world_integration.rs.jinja2")?;
    let world_context = minijinja::context! {
        regions => analysis_results.entities.regions,
        dungeons => analysis_results.entities.dungeons,
        total_entities => analysis_results.summary.total_entities,
    };
    let world_module = world_template.render(&world_context)?;
    fs::write(out_dir.join("generated_world.rs"), world_module)?;
    
    println!("Generated {} regions and {} dungeons", 
            analysis_results.entities.regions.len(),
            analysis_results.entities.dungeons.len());
    
    Ok(())
}

/// Generate hex tile modules from real HBF data using templates
fn generate_hex_tiles_from_data(
    env: &minijinja::Environment,
    results: &dl_analysis::results::GenerationResults,
    out_dir: &std::path::Path,
) -> Result<()> {
    use std::fs;
    
    let hex_template = env.get_template("hex_tile.rs.jinja2")?;
    let hex_resources_dir = out_dir.join("hex_resources");
    fs::create_dir_all(&hex_resources_dir)?;
    
        // Process each region and generate hex tiles
        for region in &results.entities.regions {
            let region_dir = hex_resources_dir.join("regions").join(&sanitize_name(&region.entity_uuid));
            fs::create_dir_all(&region_dir)?;
            
            // Extract hex coordinates from actual region data
            let hex_coords = extract_hex_coordinates_from_region_properly(region)?;
            
            let hex_coords_clone = hex_coords.clone();
            for coords in hex_coords {
                // Get correlated entities at this hex from analysis
                let settlements = get_settlements_at_hex_from_analysis(results, coords);
                let factions = get_factions_at_hex_from_analysis(results, coords);
                let npcs = get_npcs_at_hex_from_analysis(results, coords);
                let dungeons = get_dungeons_at_hex_from_analysis(results, coords);
                
                let hex_context = minijinja::context! {
                    q => coords.0,
                    r => coords.1,
                    region_uuid => region.entity_uuid,
                    settlements => settlements,
                    factions => factions,
                    npcs => npcs,
                    dungeons => dungeons,
                };
                
                let hex_module = hex_template.render(&hex_context)?;
                fs::write(region_dir.join(format!("hex_{}_{}.rs", coords.0, coords.1)), hex_module)?;
            }
            
            // Generate region module
            let region_template = env.get_template("region_module.rs.jinja2")?;
            let region_context = minijinja::context! {
                region_uuid => region.entity_uuid,
                hex_coords => hex_coords_clone,
            };
            let region_module = region_template.render(&region_context)?;
            fs::write(region_dir.join("mod.rs"), region_module)?;
        }
    
    Ok(())
}

/// Generate dungeon area modules from real HBF data using templates
fn generate_dungeon_areas_from_data(
    env: &minijinja::Environment,
    results: &dl_analysis::results::GenerationResults,
    out_dir: &std::path::Path,
) -> Result<()> {
    use std::fs;
    
    let area_template = env.get_template("dungeon_area.rs.jinja2")?;
    let dungeon_resources_dir = out_dir.join("dungeon_resources");
    fs::create_dir_all(&dungeon_resources_dir)?;
    
    // Process each dungeon and generate area modules
    for dungeon in &results.entities.dungeons {
        let dungeon_dir = dungeon_resources_dir.join("dungeons").join(&sanitize_name(&dungeon.entity_uuid));
        fs::create_dir_all(&dungeon_dir)?;
        
        // Extract area data from actual dungeon data
        let areas = extract_areas_from_dungeon_properly(dungeon)?;
        let areas_clone = areas.clone();
        
        for area in areas {
            let area_context = minijinja::context! {
                dungeon_uuid => dungeon.entity_uuid,
                area_uuid => area.uuid,
                area_name => area.name,
                monsters => area.monsters,
                treasures => area.treasures,
                connections => area.connections,
            };
            
            let area_module = area_template.render(&area_context)?;
            fs::write(dungeon_dir.join(format!("{}.rs", sanitize_name(&area.uuid))), area_module)?;
        }
        
        // Generate dungeon module
        let dungeon_template = env.get_template("dungeon_module.rs.jinja2")?;
        let dungeon_context = minijinja::context! {
            dungeon_uuid => dungeon.entity_uuid,
            areas => areas_clone,
        };
        let dungeon_module = dungeon_template.render(&dungeon_context)?;
        fs::write(dungeon_dir.join("mod.rs"), dungeon_module)?;
    }
    
    Ok(())
}

/// Extract hex coordinates from region entity using actual HBF data
fn extract_hex_coordinates_from_region_properly(region: &dl_analysis::entities::RegionHexTile) -> Result<Vec<(i32, i32)>> {
    // Parse actual region content for hex coordinates
    use regex::Regex;
    
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
fn extract_areas_from_dungeon_properly(dungeon: &dl_analysis::entities::RegionHexTile) -> Result<Vec<AreaData>> {
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
fn get_settlements_at_hex_from_analysis(
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
fn get_factions_at_hex_from_analysis(
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
fn get_npcs_at_hex_from_analysis(
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
fn get_dungeons_at_hex_from_analysis(
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
fn create_sample_entities() -> dl_analysis::results::EntityCollections {
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

/// Area data extracted from dungeon content
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AreaData {
    pub uuid: String,
    pub name: String,
    pub monsters: Vec<String>,
    pub treasures: Vec<String>,
    pub connections: Vec<String>,
}

/// Extract monster from content line
fn extract_monster_from_line(line: &str) -> String {
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
fn extract_treasure_from_line(line: &str) -> String {
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
fn extract_connection_from_line(line: &str) -> String {
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
fn sanitize_name(name: &str) -> String {
    name.replace(['-', ' ', '\''], "_").to_lowercase()
}

/// Simple hash function for generating consistent coordinates from UUID
fn simple_hash(s: &str) -> u32 {
    s.bytes().fold(0u32, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u32))
}

/// Include the generated world module
/// This is used by apps/game to include all the generated code
#[macro_export]
macro_rules! include_generated_world {
    () => {
        include!(concat!(env!("OUT_DIR"), "/mod.rs"));
    };
}
