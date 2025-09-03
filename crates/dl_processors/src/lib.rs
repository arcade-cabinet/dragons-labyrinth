//! Processor crate for Dragon's Labyrinth
//! 
//! This crate takes the analyzed data from dl_analysis and generates
//! Rust code for the game to use, using external templates.

use anyhow::Result;
use std::path::PathBuf;

// Module declarations
pub mod build_api;
pub mod components;
pub mod generators;
pub mod utilities;
pub mod ai_dialogue;

// Re-export public API
pub use components::*;
pub use utilities::AreaData;
pub use ai_dialogue::*;

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
    
    // Call dl_analysis with OUR OUT_DIR - it will analyze both HBF entities AND seeds data 
    let mut orchestrator = dl_analysis::orchestration::RawEntities::new();
    let mut logger = std::io::stdout();
    let analysis_dir = std::path::Path::new("analysis");
    
    // dl_analysis processes everything and outputs organized data to our OUT_DIR
    let analysis_summary = orchestrator.run_complete_analysis(&analysis_dir, out_dir, &mut logger)?;
    let analysis_results = dl_analysis::results::GenerationResults::success(vec!["sample.rs".to_string()])
        .with_summary(analysis_summary)
        .with_entities(utilities::create_sample_entities());
    
    // Load pre-analyzed, pre-categorized seeds data from dl_analysis output  
    let analyzed_seeds = load_analyzed_seeds_data(out_dir)?;
    
    // Generate hex tile modules using templates
    generators::generate_hex_tiles_from_data(&env, &analysis_results, out_dir)?;
    
    // Generate dungeon area modules using templates
    generators::generate_dungeon_areas_from_data(&env, &analysis_results, out_dir)?;
    
    // Generate dialogue modules with REQUIRED analyzed seeds integration
    generators::generate_dialogue_modules_from_data(&env, &analysis_results, &analyzed_seeds, out_dir)?;
    
    // Generate main world integration module
    let world_template = env.get_template("world_integration.rs.jinja2")?;
    let world_context = minijinja::context! {
        regions => analysis_results.entities.regions,
        dungeons => analysis_results.entities.dungeons,
        total_entities => analysis_results.summary.total_entities,
        has_dialogue => true,
    };
    let world_module = world_template.render(&world_context)?;
    std::fs::write(out_dir.join("generated_world.rs"), world_module)?;
    
    println!("Generated {} regions and {} dungeons with Seeds-powered dialogue system", 
            analysis_results.entities.regions.len(),
            analysis_results.entities.dungeons.len());
    
    Ok(())
}

/// Load pre-analyzed and categorized seeds data from dl_analysis output
fn load_analyzed_seeds_data(out_dir: &std::path::Path) -> Result<AnalyzedSeedsData> {
    // dl_analysis outputs categorized seeds data in structured directories:
    // dialogue/{act}/, quests/{pattern}/, linguistics/{region_type}/
    let seeds_output_dir = out_dir.join("analyzed_seeds");
    
    if !seeds_output_dir.exists() {
        return Err(anyhow::anyhow!("Analyzed seeds data not found at {:?}. dl_analysis must output categorized seeds first.", seeds_output_dir));
    }
    
    // For now, create a placeholder structure
    // This will be replaced with actual loading from Ron/JSON files output by dl_analysis
    Ok(AnalyzedSeedsData {
        dialogue_by_act: std::collections::HashMap::new(),
        quests_by_pattern: std::collections::HashMap::new(),
        linguistics_by_region: std::collections::HashMap::new(),
    })
}

/// Pre-analyzed and categorized seeds data from dl_analysis
#[derive(Debug, Clone)]
pub struct AnalyzedSeedsData {
    pub dialogue_by_act: std::collections::HashMap<u8, Vec<String>>,
    pub quests_by_pattern: std::collections::HashMap<String, Vec<String>>,
    pub linguistics_by_region: std::collections::HashMap<String, Vec<String>>,
}

/// Include the generated world module
/// This is used by apps/game to include all the generated code
#[macro_export]
macro_rules! include_generated_world {
    () => {
        include!(concat!(env!("OUT_DIR"), "/mod.rs"));
    };
}
