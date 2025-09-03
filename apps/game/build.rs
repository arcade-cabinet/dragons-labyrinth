use anyhow::Result;
use std::env;
use std::path::PathBuf;

/// Build script that calls the complete build chain via proper APIs
fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../../crates/dl_processors");
    println!("cargo:rerun-if-changed=../../crates/dl_analysis");
    println!("cargo:rerun-if-changed=../../crates/dl_seeds");
    
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    
    // Define paths for the complete build chain
    let hbf_path = std::path::Path::new("game.hbf");
    let seeds_cache_dir = out_dir.join("seeds_cache");
    
    // Call the complete build chain via proper API
    // This flows: dl_seeds → dl_analysis → dl_processors → apps/game
    println!("cargo:warning=Starting complete build chain...");
    let processed_game_data = dl_processors::build_api::provide_game_resources_for_integration(
        &hbf_path,
        &seeds_cache_dir,
        &out_dir,
    )?;
    
    // Report build statistics
    println!("cargo:warning=Build chain completed successfully:");
    println!("cargo:warning=  HBF entities processed: {}", processed_game_data.generation_stats.hbf_entities_processed);
    println!("cargo:warning=  Books analyzed: {}", processed_game_data.generation_stats.seeds_books_analyzed);
    println!("cargo:warning=  Hex tiles generated: {}", processed_game_data.generation_stats.hex_tiles_generated);
    println!("cargo:warning=  Dialogue modules created: {}", processed_game_data.generation_stats.dialogue_modules_created);
    println!("cargo:warning=  Quests generated: {}", processed_game_data.generation_stats.quests_generated);
    
    Ok(())
}
