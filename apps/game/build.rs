use anyhow::Result;
use std::env;
use std::path::PathBuf;

/// Build script that calls dl_processors to generate world resources
fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../../crates/dl_processors");
    println!("cargo:rerun-if-changed=../../crates/dl_analysis");
    
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    
    // Call dl_processors to generate hex tiles and dungeon areas
    println!("cargo:warning=Calling dl_processors to generate world resources...");
    dl_processors::generate_world_resources(&out_dir)?;
    
    println!("cargo:warning=Game build complete - world resources generated");
    Ok(())
}
