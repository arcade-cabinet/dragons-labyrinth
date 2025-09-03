//! Example program to generate CSV reports from HBF analysis
//! 
//! Run with: cargo run --example generate_reports

use dl_analysis::orchestration::RawEntities;
use dl_analysis::reporting;
use std::io::Write;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Dragon's Labyrinth Analysis Report Generator");
    println!("============================================\n");
    
    // Check for REPORTS_DIR environment variable
    let reports_dir = if let Ok(dir) = std::env::var("REPORTS_DIR") {
        println!("Using REPORTS_DIR from environment: {}", dir);
        PathBuf::from(dir)
    } else {
        let default_dir = PathBuf::from("build/reports");
        println!("No REPORTS_DIR set, using default: {}", default_dir.display());
        default_dir
    };
    
    // Create reports directory
    std::fs::create_dir_all(&reports_dir)?;
    
    // Load HBF database if it exists
    let hbf_path = std::path::Path::new("game.hbf");
    if !hbf_path.exists() {
        println!("Warning: game.hbf not found. Creating sample data for demonstration...\n");
        
        // Create sample orchestrator with some test data
        let mut orchestrator = RawEntities::new();
        
        // Add some sample entities for demonstration
        orchestrator.add_entity("region_001".to_string(), "Aurora Bushes - A mystical forest region".to_string());
        orchestrator.add_entity("settlement_001".to_string(), "City of Headsmen - A fortified city".to_string());
        orchestrator.add_entity("dungeon_001".to_string(), "Crypt of the Corrupted Order - An ancient dungeon".to_string());
        orchestrator.add_entity("faction_001".to_string(), "The Corrupted Order - A dark faction".to_string());
        
        // Generate reports from sample data
        println!("Generating CSV reports from sample data...");
        reporting::generate_all_reports(&orchestrator, &reports_dir)?;
        
    } else {
        println!("Loading HBF database from: {}", hbf_path.display());
        
        // In a real implementation, we would:
        // 1. Open the SQLite database
        // 2. Query all entities
        // 3. Process them through RawEntities
        // 4. Generate reports
        
        // For now, create empty orchestrator
        let orchestrator = RawEntities::new();
        
        println!("Generating CSV reports from HBF data...");
        reporting::generate_all_reports(&orchestrator, &reports_dir)?;
    }
    
    println!("\n✅ Reports generated successfully!");
    println!("📁 Reports saved to: {}", reports_dir.display());
    
    // List generated files
    println!("\nGenerated files:");
    for entry in std::fs::read_dir(&reports_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("csv") {
            let metadata = entry.metadata()?;
            println!("  - {} ({} bytes)", 
                     path.file_name().unwrap().to_string_lossy(),
                     metadata.len());
        }
    }
    
    println!("\nTip: Set REPORTS_DIR environment variable to customize output location");
    println!("Example: REPORTS_DIR=/tmp/dl_reports cargo run --example generate_reports");
    
    Ok(())
}
