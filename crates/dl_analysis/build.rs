//! Build script for dl_analysis crate
//! 
//! Simplified build script that sets up the environment for the analysis system.
//! The actual HBF processing and AI generation happens at runtime, not build time.

use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=game.hbf");
    println!("cargo:rerun-if-changed=build.rs");
    
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    
    // Create directories for analysis output
    let analysis_dir = out_dir.join("analysis");
    let models_dir = out_dir.join("models");
    let html_dir = out_dir.join("html");
    let json_dir = out_dir.join("json");
    let ron_dir = out_dir.join("ron");
    
    // Create all output directories
    for dir in [&analysis_dir, &models_dir, &html_dir, &json_dir, &ron_dir] {
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
    
    // Create environment variables for runtime use
    println!("cargo:rustc-env=DL_ANALYSIS_OUT_DIR={}", out_dir.display());
    println!("cargo:rustc-env=DL_HBF_PATH=game.hbf");
    
    println!("cargo:warning=dl_analysis build script completed successfully");
}
