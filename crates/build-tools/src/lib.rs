//! AI-powered build tools for Dragon's Labyrinth asset generation
//! 
//! This crate provides structured AI agents using openai_dive for asset generation
//! during the build process. It integrates with game-database to provide runtime
//! tools and context for AI decision-making.

pub mod agents;
pub mod context;
pub mod error;
pub mod generation;
pub mod memory;
pub mod mcp_client;
pub mod orchestrator;
pub mod prompt_loader;
pub mod tools;

// Re-export main types
pub use context::{BuildConfig, BuildContext};
pub use error::BuildToolError;
pub use generation::{GenerationRequest, GenerationResult};

// Re-export Agent trait
pub use agents::Agent;

// Re-export UUID for convenience
pub use uuid::Uuid;

// For build.rs integration
pub mod build_integration {
    use super::*;
    use anyhow::Result;
    
    /// Run asset generation for build.rs
    pub async fn generate_assets() -> Result<()> {
        let output_dir = std::env::var("OUT_DIR")?;
        let mut context = context::BuildContext::new(output_dir)?;
        
        // Connect to database if available
        #[cfg(feature = "with-database")]
        if let Ok(db_url) = std::env::var("DATABASE_URL") {
            context.connect_database(&db_url).await?;
        }
        
        // Run generation for each category
        for category in context.config().asset_categories.iter() {
            println!("Generating assets for category: {}", category);
            // Agent-specific generation would happen here
        }
        
        Ok(())
    }
}
