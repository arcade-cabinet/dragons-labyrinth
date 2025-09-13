use anyhow::Result;
use std::env;
use std::path::PathBuf;

/// Minimal build script - dl_seeds is now standalone binaries
/// 
/// The game no longer depends on dl_seeds as a build dependency.
/// Instead, use the standalone binaries:
/// - `cargo run --bin hbf-analyzer` - Analyze HBF database files
/// - `cargo run --bin ron-generator` - Generate organized RON assets  
/// - `cargo run --bin replit-prompter` - Create Replit 3D model prompts
fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=build.rs");
    
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    
    // Create placeholder generated files so the game compiles
    println!("cargo:warning=Using standalone dl_seeds binaries (no build-time generation)");
    
    // Create minimal placeholder for generated_world.rs
    let placeholder_world = r#"//! Placeholder generated world file
//! 
//! To generate actual content, use the standalone dl_seeds binaries:
//! 1. cargo run --bin hbf-analyzer --database game.hbf --output analysis/
//! 2. cargo run --bin ron-generator --input analysis/ --output assets/
//! 3. cargo run --bin replit-prompter --input analysis/ --assets assets/ --output replit_prompts/

// Placeholder structures for compilation
pub mod placeholder {
    use std::collections::HashMap;
    
    pub fn get_placeholder_data() -> HashMap<String, String> {
        HashMap::new()
    }
}
"#;
    
    std::fs::write(out_dir.join("generated_world.rs"), placeholder_world)?;
    
    println!("cargo:warning=Placeholder world generated - use standalone binaries for real content");
    
    Ok(())
}
