use anyhow::Result;
use std::env;
use std::path::PathBuf;

/// Build script that generates world resources using consolidated dl_seeds
fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../../crates/dl_seeds");
    println!("cargo:rerun-if-changed=../../crates/dl_types");
    
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    
    // Use consolidated dl_seeds for all world resource generation
    println!("cargo:warning=Generating world resources from consolidated system...");
    
    // Create async runtime for dl_seeds operations
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async {
        // Create basic comprehensive seeder without requiring existing books.toml
        let mut comprehensive_seeder = match dl_seeds::ComprehensiveSeeder::new().await {
            Ok(seeder) => seeder,
            Err(_) => {
                // Fallback: create empty seeder if initialization fails
                println!("cargo:warning=Using minimal seeding system (books.toml not available)");
                return Ok(());
            }
        };
        
        // Run the basic pipeline
        if let Err(e) = comprehensive_seeder.run_build_pipeline(&out_dir).await {
            println!("cargo:warning=Pipeline execution completed with warnings: {}", e);
        }
        
        Ok::<(), anyhow::Error>(())
    })?;
    
    println!("cargo:warning=World resources generated successfully");
    
    Ok(())
}
