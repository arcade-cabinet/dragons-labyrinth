// Overnight asset generator binary
// This is what the background agent runs to generate thousands of assets

use anyhow::Result;
use build_tools::orchestrator::OvernightOrchestrator;
use std::env;
use tracing::{info, error};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    info!("=================================================");
    info!("Dragon's Labyrinth Overnight Asset Generator");
    info!("=================================================");
    
    // Verify API keys
    if env::var("OPENAI_API_KEY").is_err() {
        error!("OPENAI_API_KEY not set!");
        error!("Please export OPENAI_API_KEY=your_key");
        std::process::exit(1);
    }
    
    if env::var("FREESOUND_API_KEY").is_err() {
        info!("FREESOUND_API_KEY not set - audio generation will be limited");
    }
    
    // Set up paths
    let prompts_dir = "crates/game-assets/prompts";
    let output_dir = env::var("OUT_DIR")
        .unwrap_or_else(|_| "target/generated".to_string());
    
    info!("Prompt specifications: {}", prompts_dir);
    info!("Output directory: {}", output_dir);
    info!("");
    info!("Starting overnight generation run...");
    info!("This will generate THOUSANDS of assets.");
    info!("Estimated time: 8-12 hours");
    info!("");
    
    // Create orchestrator and run
    let mut orchestrator = OvernightOrchestrator::new(prompts_dir, &output_dir)?;
    
    match orchestrator.run_overnight_generation().await {
        Ok(_) => {
            info!("");
            info!("=================================================");
            info!("OVERNIGHT GENERATION COMPLETE!");
            info!("=================================================");
            info!("");
            info!("Generated assets are in: {}", output_dir);
            info!("");
            info!("Next steps:");
            info!("1. Review generation manifest");
            info!("2. Copy assets to assets/generated/");
            info!("3. Build game-engine");
            Ok(())
        }
        Err(e) => {
            error!("");
            error!("=================================================");
            error!("GENERATION FAILED!");
            error!("=================================================");
            error!("Error: {}", e);
            Err(e)
        }
    }
}
