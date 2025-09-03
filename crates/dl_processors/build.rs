//! dl_processors build script
//!
//! CORRECT ARCHITECTURE:
//! Build-time: Call dl_analysis to generate organized output (RON files by category)
//! Runtime: Process that organized output using AI-generated models into ECS resources

use anyhow::Result;
use std::env;
use std::path::PathBuf;

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../dl_analysis");
    
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    
    // CORRECT: dl_processors BUILD TIME calls dl_analysis to generate organized output
    println!("cargo:warning=Calling dl_analysis to generate organized output...");
    
    // Get dl_analysis output directories
    let analysis_output_dir = dl_analysis::analysis_dir(); 
    let models_dir = dl_analysis::models_dir();
    let templates_dir = PathBuf::from("crates/dl_analysis/templates");
    
    // Call dl_analysis orchestration to generate organized subdirectories
    let hbf_path = PathBuf::from("crates/dl_analysis/game.hbf");
    if hbf_path.exists() {
        // Run dl_analysis to generate organized output
        let summary = dl_analysis::orchestration::RawEntities::run_complete_analysis(
            &hbf_path,
            &analysis_output_dir, 
            &models_dir,
            &templates_dir
        )?;
        
        println!("cargo:warning=dl_analysis generated organized output: {} total entities", 
                 summary.total_entities);
        println!("cargo:warning=  Organized into regions/, dungeons/, settlements/, factions/ subdirectories");
        println!("cargo:warning=  AI-generated models available in {}", models_dir.display());
    } else {
        println!("cargo:warning=HBF database not found at: {}", hbf_path.display());
        println!("cargo:warning=dl_analysis will provide empty organized output");
        
        // Generate empty organized output for development
        std::fs::create_dir_all(&analysis_output_dir)?;
        std::fs::create_dir_all(&models_dir)?;
    }
    
    // Pass output directory info to runtime
    println!("cargo:rustc-env=DL_ANALYSIS_OUTPUT_DIR={}", analysis_output_dir.display());
    println!("cargo:rustc-env=DL_MODELS_DIR={}", models_dir.display());
    
    println!("cargo:warning=dl_processors build complete - organized output ready for runtime");
    
    Ok(())
}
