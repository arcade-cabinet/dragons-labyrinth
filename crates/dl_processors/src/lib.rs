//! Processor crate for Dragon's Labyrinth
//! 
//! This crate takes the analyzed data from dl_analysis and generates
//! Rust code for the game to use, using external templates.

use anyhow::Result;
use std::path::PathBuf;

// Module declarations
pub mod components;
pub mod generators;
pub mod utilities;

// Re-export public API
pub use components::*;
pub use utilities::AreaData;

/// Get the path to the generated code
pub fn generated_dir() -> PathBuf {
    PathBuf::from(env!("OUT_DIR"))
}

/// Production API called by apps/game build.rs to generate world resources
pub fn generate_world_resources(out_dir: &std::path::Path) -> Result<()> {
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
    env.add_template("dialogue_module.rs.jinja2", include_str!("../templates/dialogue_module.rs.jinja2"))?;
    env.add_template("npc_dialogue.rs.jinja2", include_str!("../templates/npc_dialogue.rs.jinja2"))?;
    
    // Call dl_analysis to get the processed HBF data
    let mut orchestrator = dl_analysis::orchestration::RawEntities::new();
    let mut logger = std::io::stdout();
    let analysis_dir = std::path::Path::new("analysis");
    let models_dir = std::path::Path::new("target/models");
    
    // Create a simple GenerationResults with sample data for now
    let analysis_summary = orchestrator.run_complete_analysis(&analysis_dir, &models_dir, &mut logger)?;
    let analysis_results = dl_analysis::results::GenerationResults::success(vec!["sample.rs".to_string()])
        .with_summary(analysis_summary)
        .with_entities(utilities::create_sample_entities());
    
    // Load Seeds data for dialogue generation
    let seeds_dir = std::path::Path::new(env!("DL_SEEDS_DIR"));
    let seeds_manager = match dl_analysis::seeds::SeedsDataManager::load_from_cache(seeds_dir) {
        Ok(manager) => Some(manager),
        Err(e) => {
            println!("cargo:warning=Could not load Seeds data, using fallback: {}", e);
            None
        }
    };
    
    // Generate hex tile modules using templates
    generators::generate_hex_tiles_from_data(&env, &analysis_results, out_dir)?;
    
    // Generate dungeon area modules using templates
    generators::generate_dungeon_areas_from_data(&env, &analysis_results, out_dir)?;
    
    // Generate dialogue modules with Seeds integration
    generators::generate_dialogue_modules_from_data(&env, &analysis_results, &seeds_manager, out_dir)?;
    
    // Generate main world integration module
    let world_template = env.get_template("world_integration.rs.jinja2")?;
    let world_context = minijinja::context! {
        regions => analysis_results.entities.regions,
        dungeons => analysis_results.entities.dungeons,
        total_entities => analysis_results.summary.total_entities,
        has_dialogue => seeds_manager.is_some(),
    };
    let world_module = world_template.render(&world_context)?;
    std::fs::write(out_dir.join("generated_world.rs"), world_module)?;
    
    println!("Generated {} regions and {} dungeons with {} dialogue systems", 
            analysis_results.entities.regions.len(),
            analysis_results.entities.dungeons.len(),
            if seeds_manager.is_some() { "Seeds-powered" } else { "fallback" });
    
    Ok(())
}

/// Include the generated world module
/// This is used by apps/game to include all the generated code
#[macro_export]
macro_rules! include_generated_world {
    () => {
        include!(concat!(env!("OUT_DIR"), "/mod.rs"));
    };
}
