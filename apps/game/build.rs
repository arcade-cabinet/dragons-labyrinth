use anyhow::Result;
use dl_analysis::{
    containers::{RegionContainer, DungeonContainer},
    orchestration::RawEntities,
    results::GenerationResults,
};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;

/// Phase 3: Apps/Game Integration - Advanced hex-tile resource generation
/// This creates the final game-ready resources with all correlated data baked into specific hex tiles
fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../../crates/dl_processors");
    println!("cargo:rerun-if-changed=../../crates/dl_analysis");
    
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    
    // Phase 3.1: Trigger dl_processors build to get generated ECS resources
    println!("cargo:warning=Phase 3: Running dl_processors integration...");
    let processors_out_dir = dl_processors::generated_dir();
    
    // Phase 3.2: Load analysis results and container data for spatial correlations
    println!("cargo:warning=Loading container data for spatial correlations...");
    let orchestrator = RawEntities::new();
    let analysis_results = orchestrator.run_full_analysis()?;
    
    // Phase 3.3: Build spatial containers for O(1) hex-based queries
    let region_container = build_region_container(&analysis_results)?;
    let dungeon_container = build_dungeon_container(&analysis_results)?;
    
    // Phase 3.4: Generate hex-tile specific modules with all correlated data baked in
    println!("cargo:warning=Generating hex-tile specific resources...");
    generate_hex_specific_resources(&analysis_results, &region_container, &dungeon_container, &out_dir)?;
    
    // Phase 3.5: Generate dungeon area resources with container-based pathfinding
    generate_dungeon_area_resources(&analysis_results, &dungeon_container, &out_dir)?;
    
    // Phase 3.6: Generate faction and settlement correlation resources
    generate_correlation_resources(&analysis_results, &region_container, &out_dir)?;
    
    // Phase 3.7: Generate main game integration module
    generate_game_integration_module(&analysis_results, &out_dir)?;
    
    // Phase 3.8: Copy processed dl_processors resources
    copy_processors_resources(&processors_out_dir, &out_dir)?;
    
    println!("cargo:warning=Phase 3 complete - generated {} hex-tile specific modules", 
             analysis_results.summary.total_entities);
    
    Ok(())
}

/// Build region container with O(1) hex-based entity lookups
fn build_region_container(results: &GenerationResults) -> Result<RegionContainer> {
    println!("cargo:warning=Building region container with {} entities...", 
             results.summary.total_entities);
    
    let mut container = RegionContainer::new();
    
    // Process all region entities and build spatial index
    container.build_indexes(
        &results.entities.regions,
        &results.entities.settlements, 
        &results.entities.factions,
    )?;
    
    Ok(container)
}

/// Build dungeon container with area-based spatial indexing
fn build_dungeon_container(results: &GenerationResults) -> Result<DungeonContainer> {
    println!("cargo:warning=Building dungeon container with area-based indexing...");
    
    let mut container = DungeonContainer::new();
    
    // Process all dungeon entities and build area relationships
    container.build_indexes(&results.entities.dungeons)?;
    
    Ok(container)
}

/// Generate hex-tile specific resources with all correlated data baked in
fn generate_hex_specific_resources(
    results: &GenerationResults,
    region_container: &RegionContainer,
    dungeon_container: &DungeonContainer,
    out_dir: &PathBuf,
) -> Result<()> {
    let hex_resources_dir = out_dir.join("hex_resources");
    fs::create_dir_all(&hex_resources_dir)?;
    
    // Group entities by region first
    let mut region_hex_map: HashMap<String, Vec<(i32, i32)>> = HashMap::new();
    
    // Extract hex coordinates from all region entities
    for region_entity in &results.entities.regions {
        if let Some(region_uuid) = region_entity.uuid.as_ref() {
            let hex_coords = extract_hex_coordinates_from_entity(region_entity)?;
            region_hex_map.insert(region_uuid.clone(), hex_coords);
        }
    }
    
    // Generate resources for each region and its hex tiles
    for (region_uuid, hex_coordinates) in region_hex_map {
        generate_region_hex_resources(
            &region_uuid,
            &hex_coordinates,
            results,
            region_container,
            dungeon_container,
            &hex_resources_dir,
        )?;
    }
    
    Ok(())
}

/// Generate resources for a specific region and all its hex tiles
fn generate_region_hex_resources(
    region_uuid: &str,
    hex_coordinates: &[(i32, i32)],
    results: &GenerationResults,
    region_container: &RegionContainer,
    dungeon_container: &DungeonContainer,
    hex_resources_dir: &PathBuf,
) -> Result<()> {
    let sanitized_region_uuid = sanitize_uuid(region_uuid);
    let region_dir = hex_resources_dir.join("regions").join(&sanitized_region_uuid);
    fs::create_dir_all(&region_dir)?;
    
    // Generate individual hex tile modules with all correlated data
    for &(q, r) in hex_coordinates {
        let hex_uuid = format!("hex_{}_{}", q, r); // Generate consistent hex UUID
        let sanitized_hex_uuid = sanitize_uuid(&hex_uuid);
        
        // Use container system to get all entities at this hex coordinate
        let hex_entities = region_container.get_entities_at_hex((q, r));
        let nearby_dungeons = dungeon_container.get_dungeons_near_hex((q, r), 3); // 3 hex radius
        
        // Correlate settlements, factions, NPCs to this specific hex
        let correlated_settlements = get_settlements_at_hex(results, region_container, (q, r));
        let correlated_factions = get_factions_at_hex(results, region_container, (q, r));
        let correlated_npcs = get_npcs_at_hex(results, region_container, (q, r));
        
        // Generate the hex-specific module with all baked-in data
        let hex_module_code = generate_hex_tile_module(
            region_uuid,
            &hex_uuid,
            (q, r),
            &hex_entities,
            &nearby_dungeons,
            &correlated_settlements,
            &correlated_factions,
            &correlated_npcs,
        );
        
        let hex_file_path = region_dir.join(format!("{}.rs", sanitized_hex_uuid));
        fs::write(hex_file_path, hex_module_code)?;
    }
    
    // Generate region module that imports all hex modules
    let region_mod_code = generate_region_module(&sanitized_region_uuid, region_uuid, hex_coordinates);
    fs::write(region_dir.join("mod.rs"), region_mod_code)?;
    
    Ok(())
}

/// Generate a complete hex tile module with all correlated data baked in
fn generate_hex_tile_module(
    region_uuid: &str,
    hex_uuid: &str,
    coords: (i32, i32),
    hex_entities: &[String],
    nearby_dungeons: &[String],
    settlements: &[String],
    factions: &[String],
    npcs: &[String],
) -> String {
    let (q, r) = coords;
    let sanitized_hex = sanitize_ident(&format!("hex_{}_{}", q, r));
    
    format!(r#"//! Hex tile ({}, {}) with all correlated data baked in
//! Region: {}
//! Hex UUID: {}
//! Generated with spatial container correlations

use bevy::prelude::*;
use crate::components::*;
use crate::spatial::*;

/// Complete hex tile data with all correlations
#[derive(Resource, Debug, Clone)]
pub struct {}HexTile {{
    pub coords: (i32, i32),
    pub region_uuid: String,
    pub hex_uuid: String,
    pub entities: Vec<String>,
    pub settlements: Vec<String>,
    pub factions: Vec<String>,
    pub npcs: Vec<String>,
    pub nearby_dungeons: Vec<String>,
}}

impl Default for {}HexTile {{
    fn default() -> Self {{
        Self {{
            coords: ({}, {}),
            region_uuid: "{}".to_string(),
            hex_uuid: "{}".to_string(),
            entities: vec![{}],
            settlements: vec![{}],
            factions: vec![{}],
            npcs: vec![{}],
            nearby_dungeons: vec![{}],
        }}
    }}
}}

/// Spawn this hex tile with all correlated entities
pub fn spawn_{}_hex_with_correlations(
    mut commands: Commands,
    mut spatial_container: ResMut<SpatialContainer>,
) -> Entity {{
    // Create main hex tile entity
    let hex_entity = commands.spawn((
        HexPosition {{ q: {}, r: {} }},
        HexId("{}".to_string()),
        RegionId("{}".to_string()),
        HexCorrelations {{
            settlements: vec![{}],
            factions: vec![{}], 
            npcs: vec![{}],
            nearby_dungeons: vec![{}],
        }},
    )).id();
    
    // Register in spatial container for O(1) lookups
    spatial_container.register_hex_entity(({}, {}), hex_entity);
    
    // Spawn correlated settlements at this hex
    {}
    
    // Spawn faction representatives at this hex
    {}
    
    // Spawn NPCs at this hex  
    {}
    
    // Create dungeon entrance markers for nearby dungeons
    {}
    
    println!("Spawned hex ({}, {}) with {} entities, {} settlements, {} factions, {} NPCs, {} nearby dungeons",
             {}, {}, {}, {}, {}, {}, {});
    
    hex_entity
}}

/// Get static hex metadata for queries
pub const HEX_{}_METADATA: HexMetadata = HexMetadata {{
    coords: ({}, {}),
    region_uuid: "{}",
    hex_uuid: "{}",
    entity_count: {},
    settlement_count: {},
    faction_count: {},
    npc_count: {},
    dungeon_count: {},
}};

/// Efficient spawn functions for each correlated entity type
{}

/// Container-based queries for this hex
pub fn query_{}_entities(
    spatial_container: &SpatialContainer,
) -> Vec<Entity> {{
    spatial_container.get_entities_at_hex(({}, {}))
}}
"#,
        q, r, region_uuid, hex_uuid,
        sanitized_hex,
        sanitized_hex,
        q, r, region_uuid, hex_uuid,
        format_string_vec(hex_entities),
        format_string_vec(settlements),
        format_string_vec(factions), 
        format_string_vec(npcs),
        format_string_vec(nearby_dungeons),
        sanitized_hex.to_lowercase(),
        q, r, hex_uuid, region_uuid,
        format_string_vec(settlements),
        format_string_vec(factions),
        format_string_vec(npcs),
        format_string_vec(nearby_dungeons),
        q, r,
        generate_settlement_spawn_code(settlements),
        generate_faction_spawn_code(factions),
        generate_npc_spawn_code(npcs),
        generate_dungeon_marker_code(nearby_dungeons),
        q, r, hex_entities.len(), settlements.len(), factions.len(), npcs.len(), nearby_dungeons.len(),
        sanitized_hex.to_uppercase(),
        q, r, region_uuid, hex_uuid,
        hex_entities.len(), settlements.len(), factions.len(), npcs.len(), nearby_dungeons.len(),
        generate_entity_spawn_functions(hex_entities, settlements, factions, npcs),
        sanitized_hex.to_lowercase(),
        q, r
    )
}

/// Generate region module that imports all hex modules
fn generate_region_module(sanitized_region_uuid: &str, region_uuid: &str, hex_coordinates: &[(i32, i32)]) -> String {
    let hex_mod_declarations: String = hex_coordinates.iter()
        .map(|(q, r)| format!("pub mod hex_{}_{};\npub use hex_{}_{}::*;", q, r, q, r))
        .collect::<Vec<_>>()
        .join("\n");
    
    let hex_spawn_calls: String = hex_coordinates.iter()
        .map(|(q, r)| format!("    spawn_hex_{}_{}({}, {}, &mut commands, &mut spatial_container);", q, r, q, r))
        .collect::<Vec<_>>()
        .join("\n");
    
    format!(r#"//! Region {} with all hex tiles and correlations
//! UUID: {}

use bevy::prelude::*;
use crate::components::*;
use crate::spatial::*;

// Import all hex tile modules
{}

/// Spawn entire region with all hex tiles and correlations
pub fn spawn_{}_region_with_all_correlations(
    mut commands: Commands,
    mut spatial_container: ResMut<SpatialContainer>,
) {{
    println!("Spawning region {} with {} hex tiles...", "{}", {});
    
{}
    
    println!("Region {} spawned successfully", "{}");
}}

/// Query all entities in this region using container system
pub fn query_{}_region_entities(
    spatial_container: &SpatialContainer,
) -> Vec<Entity> {{
    let mut entities = Vec::new();
    {}
    entities
}}
"#,
        sanitized_region_uuid, region_uuid,
        hex_mod_declarations,
        sanitized_region_uuid.to_lowercase(),
        region_uuid, hex_coordinates.len(),
        hex_spawn_calls,
        region_uuid,
        sanitized_region_uuid.to_lowercase(),
        hex_coordinates.iter()
            .map(|(q, r)| format!("    entities.extend(spatial_container.get_entities_at_hex(({}, {})));", q, r))
            .collect::<Vec<_>>()
            .join("\n")
    )
}

/// Generate dungeon area resources with container-based pathfinding
/// Creates dungeons/{dungeon_uuid}/{area_uuid}.rs for each dungeon map
fn generate_dungeon_area_resources(
    results: &dl_analysis::results::AnalysisSummary,
    dungeon_container: &DungeonContainer,
    out_dir: &PathBuf,
) -> Result<()> {
    let dungeons_dir = out_dir.join("dungeon_resources");
    fs::create_dir_all(&dungeons_dir)?;
    
    // Extract dungeon entities from analysis results
    let dungeon_entities = extract_dungeon_entities_from_results(results)?;
    
    // Generate area-specific resources for each dungeon
    for (dungeon_uuid, dungeon_areas) in dungeon_entities {
        generate_dungeon_with_areas(
            &dungeon_uuid,
            &dungeon_areas,
            dungeon_container,
            &dungeons_dir,
        )?;
    }
    
    // Generate dungeons main module
    let dungeons_mod_code = r#"//! Dungeon areas with container-based pathfinding

use bevy::prelude::*;
use crate::components::*;
use crate::spatial::*;

/// Spawn all dungeon areas with pathfinding
pub fn spawn_all_dungeon_areas_with_pathfinding(
    mut commands: Commands,
    mut spatial_container: ResMut<SpatialContainer>,
) {
    // TODO: Import and spawn all generated dungeon modules
    println!("Spawning dungeon areas with container-based pathfinding...");
}
"#;
    
    fs::write(dungeons_dir.join("mod.rs"), dungeons_mod_code)?;
    
    Ok(())
}

/// Generate complete dungeon with all its areas: dungeons/{dungeon_uuid}/{area_uuid}.rs
fn generate_dungeon_with_areas(
    dungeon_uuid: &str,
    dungeon_areas: &[(String, DungeonAreaData)],
    dungeon_container: &DungeonContainer,
    dungeons_dir: &PathBuf,
) -> Result<()> {
    let sanitized_dungeon_uuid = dl_analysis::templates::sanitize_uuid(dungeon_uuid);
    let dungeon_dir = dungeons_dir.join("dungeons").join(&sanitized_dungeon_uuid);
    fs::create_dir_all(&dungeon_dir)?;
    
    // Generate individual area modules for this dungeon
    for (area_uuid, area_data) in dungeon_areas {
        let sanitized_area_uuid = dl_analysis::templates::sanitize_uuid(area_uuid);
        
        // Use container system to get area connections and pathfinding data
        let connected_areas = dungeon_container.get_connected_areas(area_uuid);
        let pathfinding_data = dungeon_container.get_pathfinding_data(area_uuid);
        let area_monsters = get_monsters_in_area(area_uuid, dungeon_container);
        let area_treasures = get_treasures_in_area(area_uuid, dungeon_container);
        
        // Generate the area-specific module with all pathfinding data baked in
        let area_module_code = generate_dungeon_area_module(
            dungeon_uuid,
            area_uuid,
            area_data,
            &connected_areas,
            &pathfinding_data,
            &area_monsters,
            &area_treasures,
        );
        
        let area_file_path = dungeon_dir.join(format!("{}.rs", sanitized_area_uuid));
        fs::write(area_file_path, area_module_code)?;
    }
    
    // Generate dungeon module that imports all area modules
    let dungeon_mod_code = generate_dungeon_module(&sanitized_dungeon_uuid, dungeon_uuid, dungeon_areas);
    fs::write(dungeon_dir.join("mod.rs"), dungeon_mod_code)?;
    
    Ok(())
}

/// Generate individual dungeon area module with pathfinding and connections baked in
fn generate_dungeon_area_module(
    dungeon_uuid: &str,
    area_uuid: &str,
    area_data: &DungeonAreaData,
    connected_areas: &[String],
    pathfinding_data: &PathfindingData,
    area_monsters: &[String],
    area_treasures: &[String],
) -> String {
    let sanitized_area = dl_analysis::templates::sanitize_ident(&format!("area_{}", area_uuid));
    
    format!(r#"//! Dungeon area {} in dungeon {}
//! Area UUID: {}
//! Generated with pathfinding and connections baked in

use bevy::prelude::*;
use crate::components::*;
use crate::spatial::*;

/// Complete dungeon area data with pathfinding
#[derive(Resource, Debug, Clone)]
pub struct {}DungeonArea {{
    pub dungeon_uuid: String,
    pub area_uuid: String,
    pub area_name: String,
    pub connected_areas: Vec<String>,
    pub monsters: Vec<String>,
    pub treasures: Vec<String>,
    pub pathfinding_nodes: Vec<(i32, i32)>,
}}

impl Default for {}DungeonArea {{
    fn default() -> Self {{
        Self {{
            dungeon_uuid: "{}".to_string(),
            area_uuid: "{}".to_string(),
            area_name: "{}".to_string(),
            connected_areas: vec![{}],
            monsters: vec![{}],
            treasures: vec![{}],
            pathfinding_nodes: vec![{}],
        }}
    }}
}}

/// Spawn this dungeon area with pathfinding and connections
pub fn spawn_{}_area_with_pathfinding(
    mut commands: Commands,
    mut spatial_container: ResMut<SpatialContainer>,
) -> Entity {{
    // Create main dungeon area entity
    let area_entity = commands.spawn((
        DungeonId("{}".to_string()),
        DungeonAreaId("{}".to_string()),
        DungeonAreaName("{}".to_string()),
        DungeonConnections {{
            connected_areas: vec![{}],
        }},
        PathfindingNodes(vec![{}]),
    )).id();
    
    // Register in spatial container for area-based lookups
    spatial_container.register_dungeon_area_entity("{}".to_string(), area_entity);
    
    // Spawn monsters in this area
    {}
    
    // Spawn treasures in this area
    {}
    
    // Create pathfinding connections to other areas
    {}
    
    println!("Spawned dungeon area {{}} with {} monsters, {} treasures, {} connections",
             "{}", {}, {}, {});
    
    area_entity
}}

/// Get static area metadata for pathfinding queries
pub const AREA_{}_METADATA: DungeonAreaMetadata = DungeonAreaMetadata {{
    dungeon_uuid: "{}",
    area_uuid: "{}",
    monster_count: {},
    treasure_count: {},
    connection_count: {},
}};
"#,
        area_data.name, dungeon_uuid, area_uuid,
        sanitized_area,
        sanitized_area,
        dungeon_uuid, area_uuid, area_data.name,
        format_string_vec(connected_areas),
        format_string_vec(area_monsters),
        format_string_vec(area_treasures),
        format_pathfinding_nodes(pathfinding_data),
        sanitized_area.to_lowercase(),
        dungeon_uuid, area_uuid, area_data.name,
        format_string_vec(connected_areas),
        format_pathfinding_nodes(pathfinding_data),
        area_uuid,
        generate_monster_spawn_code(area_monsters),
        generate_treasure_spawn_code(area_treasures),
        generate_area_connection_code(connected_areas),
        area_uuid, area_monsters.len(), area_treasures.len(), connected_areas.len(),
        sanitized_area.to_uppercase(),
        dungeon_uuid, area_uuid,
        area_monsters.len(), area_treasures.len(), connected_areas.len()
    )
}

/// Generate dungeon module that imports all area modules
fn generate_dungeon_module(
    sanitized_dungeon_uuid: &str,
    dungeon_uuid: &str,
    dungeon_areas: &[(String, DungeonAreaData)],
) -> String {
    let area_mod_declarations: String = dungeon_areas.iter()
        .map(|(area_uuid, _)| {
            let sanitized_area_uuid = dl_analysis::templates::sanitize_uuid(area_uuid);
            format!("pub mod {};\npub use {}::*;", sanitized_area_uuid, sanitized_area_uuid)
        })
        .collect::<Vec<_>>()
        .join("\n");
    
    let area_spawn_calls: String = dungeon_areas.iter()
        .map(|(area_uuid, _)| {
            let sanitized_area_uuid = dl_analysis::templates::sanitize_uuid(area_uuid);
            format!("    spawn_{}_area_with_pathfinding(&mut commands, &mut spatial_container);", sanitized_area_uuid)
        })
        .collect::<Vec<_>>()
        .join("\n");
    
    format!(r#"//! Dungeon {} with all areas and pathfinding
//! UUID: {}

use bevy::prelude::*;
use crate::components::*;
use crate::spatial::*;

// Import all area modules
{}

/// Spawn entire dungeon with all areas and pathfinding
pub fn spawn_{}_dungeon_with_all_areas(
    mut commands: Commands,
    mut spatial_container: ResMut<SpatialContainer>,
) {{
    println!("Spawning dungeon {} with {} areas...", "{}", {});
    
{}
    
    println!("Dungeon {} spawned successfully", "{}");
}}

/// Query all entities in this dungeon using container system
pub fn query_{}_dungeon_entities(
    spatial_container: &SpatialContainer,
) -> Vec<Entity> {{
    // TODO: Query all area entities in this dungeon
    Vec::new()
}}
"#,
        sanitized_dungeon_uuid, dungeon_uuid,
        area_mod_declarations,
        sanitized_dungeon_uuid.to_lowercase(),
        dungeon_uuid, dungeon_areas.len(),
        area_spawn_calls,
        dungeon_uuid,
        sanitized_dungeon_uuid.to_lowercase()
    )
}

/// Generate correlation resources for efficient entity relationships
fn generate_correlation_resources(
    results: &GenerationResults,
    region_container: &RegionContainer,
    out_dir: &PathBuf,
) -> Result<()> {
    let correlations_dir = out_dir.join("correlations");
    fs::create_dir_all(&correlations_dir)?;
    
    let correlations_code = format!(r#"//! Entity correlation resources for efficient lookups
//! Generated from {} total entities

use bevy::prelude::*;
use std::collections::HashMap;
use crate::spatial::*;

/// Entity correlation system for fast relationship queries
#[derive(Resource, Default)]
pub struct EntityCorrelations {{
    /// Map hex coordinates to all entity types at that location
    pub hex_to_entities: HashMap<(i32, i32), HexEntitySet>,
    /// Map settlement UUIDs to their controlling factions
    pub settlement_to_factions: HashMap<String, Vec<String>>,
    /// Map faction UUIDs to their territories (hex coordinates)
    pub faction_territories: HashMap<String, Vec<(i32, i32)>>,
    /// Map NPC UUIDs to their current hex location
    pub npc_locations: HashMap<String, (i32, i32)>,
}}

#[derive(Debug, Clone, Default)]
pub struct HexEntitySet {{
    pub settlements: Vec<String>,
    pub factions: Vec<String>,
    pub npcs: Vec<String>,
    pub dungeons: Vec<String>,
    pub special_features: Vec<String>,
}}

impl EntityCorrelations {{
    /// Initialize correlations from analysis results
    pub fn from_analysis_results() -> Self {{
        // TODO: Build from container system data
        Self::default()
    }}
    
    /// Get all entities at a specific hex coordinate
    pub fn get_entities_at_hex(&self, coords: (i32, i32)) -> &HexEntitySet {{
        self.hex_to_entities.get(&coords).unwrap_or(&HexEntitySet::default())
    }}
    
    /// Get faction controlling a settlement
    pub fn get_settlement_faction(&self, settlement_uuid: &str) -> Vec<&String> {{
        self.settlement_to_factions.get(settlement_uuid)
            .map(|factions| factions.iter().collect())
            .unwrap_or_default()
    }}
    
    /// Get all hex coordinates controlled by a faction
    pub fn get_faction_territory(&self, faction_uuid: &str) -> &[i32, i32]] {{
        self.faction_territories.get(faction_uuid)
            .map(|coords| coords.as_slice())
            .unwrap_or(&[])
    }}
}}

/// Plugin to register correlation resources
pub struct CorrelationsPlugin;

impl Plugin for CorrelationsPlugin {{
    fn build(&self, app: &mut App) {{
        app.init_resource::<EntityCorrelations>()
           .add_systems(Startup, initialize_correlations);
    }}
}}

fn initialize_correlations(mut correlations: ResMut<EntityCorrelations>) {{
    *correlations = EntityCorrelations::from_analysis_results();
    println!("Entity correlations initialized");
}}
"#,
        results.summary.total_entities
    );
    
    fs::write(correlations_dir.join("mod.rs"), correlations_code)?;
    Ok(())
}

/// Generate main game integration module
fn generate_game_integration_module(results: &GenerationResults, out_dir: &PathBuf) -> String {
    let integration_code = format!(r#"//! Main game integration module for all generated resources
//! Processes {} entities into game-ready ECS resources

pub mod hex_resources;
pub mod dungeon_areas; 
pub mod correlations;

use bevy::prelude::*;
pub use correlations::*;

/// Main plugin that integrates all generated world resources
pub struct GeneratedGameWorldPlugin;

impl Plugin for GeneratedGameWorldPlugin {{
    fn build(&self, app: &mut App) {{
        app
            .add_plugins(CorrelationsPlugin)
            .init_resource::<SpatialContainer>()
            .add_systems(Startup, (
                initialize_spatial_containers,
                spawn_all_generated_world_resources,
            ).chain())
            .add_systems(Update, update_spatial_queries);
    }}
}}

/// Initialize all spatial containers for O(1) queries
fn initialize_spatial_containers(mut commands: Commands) {{
    println!("Initializing spatial containers for {} entities...", {});
    // Spatial containers are initialized by individual spawn systems
}}

/// Spawn all generated world resources with correlations
fn spawn_all_generated_world_resources(
    mut commands: Commands,
    mut spatial_container: ResMut<SpatialContainer>,
) {{
    // Spawn all regions with their hex tiles and correlations
    hex_resources::spawn_all_regions_with_correlations(&mut commands, &mut spatial_container);
    
    // Spawn all dungeon areas with pathfinding
    dungeon_areas::spawn_all_dungeon_areas_with_pathfinding(&mut commands, &mut spatial_container);
    
    println!("All generated world resources spawned successfully");
}}

/// Update spatial queries each frame for efficient entity lookups
fn update_spatial_queries(
    spatial_container: Res<SpatialContainer>,
    correlations: Res<EntityCorrelations>,
) {{
    // This system handles real-time spatial queries
    // Called every frame but optimized for O(1) container lookups
}}

/// Query API for game systems to use generated data
pub struct WorldQuery;

impl WorldQuery {{
    /// Get all entities at a hex coordinate (O(1) lookup)
    pub fn entities_at_hex(
        spatial_container: &SpatialContainer,
        coords: (i32, i32)
    ) -> Vec<Entity> {{
        spatial_container.get_entities_at_hex(coords)
    }}
    
    /// Get settlement data for a hex (with faction correlations)
    pub fn settlements_at_hex(
        correlations: &EntityCorrelations,
        coords: (i32, i32)
    ) -> &[String] {{
        &correlations.get_entities_at_hex(coords).settlements
    }}
    
    /// Get faction presence at a hex
    pub fn factions_at_hex(
        correlations: &EntityCorrelations,
        coords: (i32, i32)
    ) -> &[String] {{
        &correlations.get_entities_at_hex(coords).factions
    }}
}}
"#,
        results.summary.total_entities,
        results.summary.total_entities
    );
    
    fs::write(out_dir.join("generated_world.rs"), integration_code)?;
    Ok(())
}

/// Copy processed dl_processors resources
fn copy_processors_resources(processors_out_dir: &PathBuf, game_out_dir: &PathBuf) -> Result<()> {
    if processors_out_dir.exists() {
        let processors_target = game_out_dir.join("dl_processors_output");
        copy_dir_all(processors_out_dir, &processors_target)?;
    }
    Ok(())
}

// === Dungeon Area Data Structures ===

#[derive(Debug, Clone)]
struct DungeonAreaData {
    name: String,
    description: String,
    area_type: String,
}

#[derive(Debug, Clone)]
struct PathfindingData {
    nodes: Vec<(i32, i32)>,
    connections: Vec<((i32, i32), (i32, i32))>,
}

// === Utility Functions ===

fn extract_hex_coordinates_from_entity(entity: &dl_analysis::entities::Entity) -> Result<Vec<(i32, i32)>> {
    // Extract hex coordinates from entity metadata/content
    // This would parse the actual entity data for hex coordinates
    // For now, return placeholder coordinates
    Ok(vec![(0, 0), (1, 0), (0, 1)]) // Placeholder
}

/// Extract dungeon entities and organize them by dungeon with their areas
fn extract_dungeon_entities_from_results(results: &dl_analysis::results::AnalysisSummary) -> Result<HashMap<String, Vec<(String, DungeonAreaData)>>> {
    let mut dungeon_map: HashMap<String, Vec<(String, DungeonAreaData)>> = HashMap::new();
    
    // For now, create placeholder dungeon structure
    // In real implementation, this would parse the analysis results
    let placeholder_dungeon_uuid = "crypt_of_the_corrupted_order".to_string();
    let placeholder_areas = vec![
        ("entrance_hall".to_string(), DungeonAreaData {
            name: "Entrance Hall".to_string(),
            description: "The foreboding entrance to the corrupted crypt".to_string(),
            area_type: "entrance".to_string(),
        }),
        ("main_chamber".to_string(), DungeonAreaData {
            name: "Main Chamber".to_string(),
            description: "The central chamber with ancient altars".to_string(),
            area_type: "chamber".to_string(),
        }),
        ("treasure_vault".to_string(), DungeonAreaData {
            name: "Treasure Vault".to_string(),
            description: "Hidden vault containing cursed artifacts".to_string(),
            area_type: "vault".to_string(),
        }),
    ];
    
    dungeon_map.insert(placeholder_dungeon_uuid, placeholder_areas);
    
    Ok(dungeon_map)
}

/// Get monsters in a specific dungeon area
fn get_monsters_in_area(area_uuid: &str, dungeon_container: &DungeonContainer) -> Vec<String> {
    // Use container system to get monsters for this area
    // For now, placeholder data
    match area_uuid {
        "entrance_hall" => vec!["skeleton_guard_1".to_string(), "skeleton_guard_2".to_string()],
        "main_chamber" => vec!["corrupted_priest".to_string(), "shadow_wraith".to_string()],
        "treasure_vault" => vec!["vault_guardian".to_string()],
        _ => Vec::new(),
    }
}

/// Get treasures in a specific dungeon area
fn get_treasures_in_area(area_uuid: &str, dungeon_container: &DungeonContainer) -> Vec<String> {
    // Use container system to get treasures for this area
    // For now, placeholder data
    match area_uuid {
        "entrance_hall" => vec!["rusty_key".to_string()],
        "main_chamber" => vec!["ancient_tome".to_string(), "silver_chalice".to_string()],
        "treasure_vault" => vec!["cursed_crown".to_string(), "dark_artifact".to_string()],
        _ => Vec::new(),
    }
}

/// Format pathfinding nodes for template
fn format_pathfinding_nodes(pathfinding_data: &PathfindingData) -> String {
    pathfinding_data.nodes.iter()
        .map(|(x, y)| format!("({}, {})", x, y))
        .collect::<Vec<_>>()
        .join(", ")
}

/// Generate monster spawn code for dungeon area
fn generate_monster_spawn_code(monsters: &[String]) -> String {
    if monsters.is_empty() {
        "// No monsters in this area".to_string()
    } else {
        monsters.iter()
            .map(|m| format!("    spawn_monster_in_area(\"{}\", area_entity, &mut commands);", m))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

/// Generate treasure spawn code for dungeon area
fn generate_treasure_spawn_code(treasures: &[String]) -> String {
    if treasures.is_empty() {
        "// No treasures in this area".to_string()
    } else {
        treasures.iter()
            .map(|t| format!("    spawn_treasure_in_area(\"{}\", area_entity, &mut commands);", t))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

/// Generate area connection code for pathfinding
fn generate_area_connection_code(connected_areas: &[String]) -> String {
    if connected_areas.is_empty() {
        "// No connections from this area".to_string()
    } else {
        connected_areas.iter()
            .map(|a| format!("    create_area_connection(\"{}\", area_entity, &mut commands);", a))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

fn get_settlements_at_hex(
    results: &GenerationResults,
    region_container: &RegionContainer, 
    coords: (i32, i32)
) -> Vec<String> {
    // Use container system to correlate settlements to hex coordinates
    // This would query the region container for settlements at specific hex
    Vec::new() // Placeholder
}

fn get_factions_at_hex(
    results: &GenerationResults,
    region_container: &RegionContainer,
    coords: (i32, i32)
) -> Vec<String> {
    Vec::new() // Placeholder
}

fn get_npcs_at_hex(
    results: &GenerationResults,
    region_container: &RegionContainer,
    coords: (i32, i32)
) -> Vec<String> {
    Vec::new() // Placeholder
}

fn generate_dungeon_area_module(
    dungeon_entity: &dl_analysis::entities::Entity,
    dungeon_container: &DungeonContainer,
    dungeons_dir: &PathBuf,
) -> Result<()> {
    // Generate sophisticated dungeon area module with pathfinding
    Ok(()) // Placeholder
}

fn format_string_vec(vec: &[String]) -> String {
    vec.iter()
        .map(|s| format!("\"{}\"", s))
        .collect::<Vec<_>>()
        .join(", ")
}

fn generate_settlement_spawn_code(settlements: &[String]) -> String {
    if settlements.is_empty() {
        "// No settlements at this hex".to_string()
    } else {
        settlements.iter()
            .map(|s| format!("    spawn_settlement_at_hex(\"{}\", hex_entity, &mut commands);", s))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

fn generate_faction_spawn_code(factions: &[String]) -> String {
    if factions.is_empty() {
        "// No faction presence at this hex".to_string()
    } else {
        factions.iter()
            .map(|f| format!("    spawn_faction_presence(\"{}\", hex_entity, &mut commands);", f))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

fn generate_npc_spawn_code(npcs: &[String]) -> String {
    if npcs.is_empty() {
        "// No NPCs at this hex".to_string()
    } else {
        npcs.iter()
            .map(|n| format!("    spawn_npc_at_hex(\"{}\", hex_entity, &mut commands);", n))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

fn generate_dungeon_marker_code(dungeons: &[String]) -> String {
    if dungeons.is_empty() {
        "// No nearby dungeons".to_string()
    } else {
        dungeons.iter()
            .map(|d| format!("    create_dungeon_entrance_marker(\"{}\", hex_entity, &mut commands);", d))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

fn generate_entity_spawn_functions(
    hex_entities: &[String],
    settlements: &[String], 
    factions: &[String],
    npcs: &[String]
) -> String {
    format!(r#"
/// Spawn settlement at this hex
fn spawn_settlement_at_hex(settlement_uuid: &str, hex_entity: Entity, commands: &mut Commands) {{
    // TODO: Implement settlement spawning with generated models
}}

/// Spawn faction presence at this hex  
fn spawn_faction_presence(faction_uuid: &str, hex_entity: Entity, commands: &mut Commands) {{
    // TODO: Implement faction presence with territorial data
}}

/// Spawn NPC at this hex
fn spawn_npc_at_hex(npc_uuid: &str, hex_entity: Entity, commands: &mut Commands) {{
    // TODO: Implement NPC spawning with relationship data
}}

/// Create dungeon entrance marker
fn create_dungeon_entrance_marker(dungeon_uuid: &str, hex_entity: Entity, commands: &mut Commands) {{
    // TODO: Implement dungeon entrance markers with pathfinding
}}
"#)
}

fn sanitize_uuid(uuid: &str) -> String {
    uuid.replace(['-', ' '], "_").to_lowercase()
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

fn copy_dir_all(src: &PathBuf, dst: &PathBuf) -> Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        
        if ty.is_dir() {
            copy_dir_all(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}
