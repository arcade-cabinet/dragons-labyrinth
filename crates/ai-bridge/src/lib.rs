//! AI-powered build tools for Dragon's Labyrinth asset generation
//! 
//! This crate provides a spec-driven AI agent architecture using openai_dive 
//! for asset generation during the build process. It integrates with game-database 
//! to provide runtime tools and context for AI decision-making.
//!
//! ## Architecture
//! 
//! The new architecture is based on agent specifications (TOML files) that define:
//! - Agent capabilities and interfaces
//! - Prompt templates and configurations  
//! - Input/output schemas
//! - Domain-specific logic
//!
//! This replaces the previous hardcoded agent system with a flexible, 
//! spec-driven approach that allows agents to be defined in domain crates
//! rather than in ai-bridge itself.

pub mod agent_executor;
pub mod agent_spec;
pub mod agents;
pub mod context;
pub mod error;
pub mod generation;
pub mod memory;
pub mod openai_client;
// Legacy modules removed - now pure spec-driven infrastructure

// Re-export main types
pub use context::{BuildConfig, BuildContext};
pub use error::BuildToolError;
pub use generation::{GenerationRequest, GenerationResult};

// Re-export spec-driven system types
pub use agent_spec::{AgentSpec, AgentSpecLoader, AgentConfigValue};
pub use agent_executor::{AgentExecutor, AgentExecutionRequest, AgentExecutionResult};
pub use agents::{
    Agent,
    SpecDrivenOrchestrator, 
    AgentOrchestrator,
    SpecDrivenGenerationReport,
    DreadLevelReport,
    AgentInfo,
};

// Re-export UUID for convenience
pub use uuid::Uuid;

// For build.rs integration with spec-driven system
pub mod build_integration {
    use super::*;
    use anyhow::Result;
    
    /// Run asset generation for build.rs using spec-driven agents
    pub async fn generate_assets() -> Result<()> {
        let output_dir = std::env::var("OUT_DIR")?;
        let output_path = std::path::Path::new(&output_dir);
        let context = context::BuildContext::new(&output_dir)?;
        
        // Create spec-driven orchestrator
        let openai_client = openai_client::OpenAIClient::new()?;
        let mut orchestrator = SpecDrivenOrchestrator::new(openai_client);
        
        // Add spec directories
        orchestrator.add_spec_directory("crates/game-dialogue");
        orchestrator.add_spec_directory("crates/hexroll_exporter");
        orchestrator.add_spec_directory("crates/game-assets");
        orchestrator.add_spec_directory("crates/ai-bridge/specs");
        
        // Load specs
        orchestrator.load_specs().await?;
        
        // Generate content for all dread levels
        let _report = orchestrator.generate_all_dread_levels(output_path, &context).await?;
        
        tracing::info!("Asset generation complete using spec-driven agents");
        
        Ok(())
    }
}
