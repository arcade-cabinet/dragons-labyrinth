//! Binary for running narrative content generation

use anyhow::Result;
use content_generation::ContentGenerator;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    
    println!("Dragon's Labyrinth - Narrative Content Generator");
    println!("================================================\n");
    
    // Check for API key
    if std::env::var("OPENAI_API_KEY").is_err() {
        eprintln!("ERROR: OPENAI_API_KEY environment variable not set!");
        eprintln!("Please set your OpenAI API key to generate content.");
        std::process::exit(1);
    }
    
    // Set up output directory
    let out_dir = Path::new("target/generated_content");
    std::fs::create_dir_all(out_dir)?;
    
    println!("Output directory: {}", out_dir.display());
    println!("Starting generation...\n");
    
    // Create and run generator
    let mut generator = ContentGenerator::new(out_dir)?;
    let report = generator.generate_all_content().await?;
    
    // Print final report
    println!("\n{}", report.summary());
    
    if !report.errors.is_empty() {
        eprintln!("\n⚠️  Errors encountered:");
        for error in &report.errors {
            eprintln!("  - {}", error);
        }
        std::process::exit(1);
    }
    
    println!("✅ Content generation complete!");
    println!("   Files saved to: {}", out_dir.display());
    
    Ok(())
}
