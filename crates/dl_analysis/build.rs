//! Build script for dl_analysis crate
//! 
//! Sets up the environment for HBF processing and downloads Seeds data

use std::env;
use std::fs;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=game.hbf");
    println!("cargo:rerun-if-changed=build.rs");
    
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    
    // Create directories for analysis output
    let analysis_dir = out_dir.join("analysis");
    let models_dir = out_dir.join("models");
    let html_dir = out_dir.join("html");
    let json_dir = out_dir.join("json");
    let ron_dir = out_dir.join("ron");
    let seeds_cache_dir = out_dir.join("seeds_cache");
    
    // Create all output directories
    let dirs = [
        &analysis_dir, &models_dir, &html_dir, &json_dir, &ron_dir, &seeds_cache_dir
    ];
    
    for dir in dirs {
        if let Err(e) = fs::create_dir_all(dir) {
            println!("cargo:warning=Failed to create directory {:?}: {}", dir, e);
        }
    }
    
    // Check if HBF database exists and report status
    let hbf_path = std::path::Path::new("game.hbf");
    if hbf_path.exists() {
        println!("cargo:warning=HBF database found at {:?}", hbf_path);
    } else {
        println!("cargo:warning=HBF database not found at {:?} - analysis will be skipped at runtime", hbf_path);
    }
    
    // Initialize Seeds data directly using SeedsManager
    println!("cargo:warning=Initializing Seeds data...");
    match dl_seeds::SeedsManager::initialize(&seeds_cache_dir) {
        Ok(seeds_manager) => {
            println!("cargo:warning=Seeds data initialized successfully");
            println!("cargo:warning=  Books loaded: {}", seeds_manager.books.get_downloaded_books().len());
            println!("cargo:warning=  Dictionary entries: {}", seeds_manager.linguistics.old_norse_dictionary.len());
            println!("cargo:warning=  Character archetypes: {}", seeds_manager.dialogue.character_archetypes.len());
        }
        Err(e) => {
            println!("cargo:warning=Failed to initialize Seeds data: {}. Build will continue.", e);
        }
    }
    
    // Create environment variables for runtime use
    println!("cargo:rustc-env=DL_ANALYSIS_OUT_DIR={}", out_dir.display());
    println!("cargo:rustc-env=DL_HBF_PATH=game.hbf");
    println!("cargo:rustc-env=DL_SEEDS_CACHE_DIR={}", seeds_cache_dir.display());
    
    println!("cargo:warning=dl_analysis build script completed successfully");
    
    Ok(())
}
