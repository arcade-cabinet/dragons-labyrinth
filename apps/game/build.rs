use anyhow::Result;
use std::env;
use std::path::PathBuf;

/// Build script that generates world resources directly
fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../../crates/dl_processors");
    println!("cargo:rerun-if-changed=../../crates/dl_analysis");
    println!("cargo:rerun-if-changed=../../crates/dl_seeds");
    
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    
    // Call dl_processors directly to generate world resources
    // This internally calls dl_analysis orchestration which processes HBF and seeds data
    println!("cargo:warning=Generating world resources...");
    dl_processors::generate_world_resources(&out_dir)?;
    
    println!("cargo:warning=World resources generated successfully");
    
    Ok(())
}
