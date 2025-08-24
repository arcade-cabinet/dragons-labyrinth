//! Agent system for AI-bridge
//! 
//! This module provides a spec-driven agent architecture that loads agent
//! specifications from TOML files instead of hardcoding agents in Rust.

pub mod spec_driven;

// Re-export the main types from the spec-driven system
pub use spec_driven::{
    SpecDrivenOrchestrator,
    SpecDrivenGenerationReport,
    DreadLevelReport,
    AgentInfo,
};

use anyhow::Result;
use async_trait::async_trait;
use crate::context::BuildContext;
use crate::generation::{GenerationRequest, GenerationResult};

/// Simplified Agent trait for backwards compatibility
/// 
/// This trait provides a bridge between the old agent system and the new 
/// spec-driven system. New agents should be implemented as spec files
/// rather than implementing this trait directly.
#[async_trait]
pub trait Agent: Send + Sync {
    /// Generate content based on the request
    async fn generate(&mut self, context: &BuildContext, request: GenerationRequest) -> Result<GenerationResult>;
    
    /// Get the agent's name
    fn name(&self) -> &str;
    
    /// Get the agent's capabilities
    fn capabilities(&self) -> Vec<String> {
        vec![]
    }
}

/// Backwards compatibility wrapper for the old orchestrator API
/// 
/// This provides the same interface as the old AgentOrchestrator but uses
/// the new spec-driven system underneath.
pub struct AgentOrchestrator {
    orchestrator: SpecDrivenOrchestrator,
}

impl AgentOrchestrator {
    /// Create a new agent orchestrator with spec-driven backend
    pub fn new(openai_client: crate::openai_client::OpenAIClient) -> Self {
        Self {
            orchestrator: SpecDrivenOrchestrator::new(openai_client),
        }
    }

    /// Add a directory to search for agent specifications
    pub fn add_spec_directory<P: AsRef<std::path::Path>>(&mut self, path: P) {
        self.orchestrator.add_spec_directory(path);
    }

    /// Load all agent specifications
    pub async fn load_specs(&mut self) -> Result<()> {
        self.orchestrator.load_specs().await
    }

    /// Generate all content for all dread levels
    pub async fn generate_all_dread_levels(
        &self,
        output_dir: &std::path::Path,
        context: &BuildContext,
    ) -> Result<SpecDrivenGenerationReport> {
        self.orchestrator.generate_all_dread_levels(output_dir, context).await
    }

    /// List all available agents
    pub fn list_agents(&self) -> Vec<AgentInfo> {
        self.orchestrator.list_agents()
    }

    /// Execute a specific agent by name
    pub async fn execute_agent(
        &self,
        agent_name: &str,
        request: GenerationRequest,
        context: &BuildContext,
    ) -> Result<GenerationResult> {
        self.orchestrator.execute_agent(agent_name, request, context).await
    }
}

/// Create output directories for agent generation
pub fn create_output_directories(out_dir: &std::path::Path) -> Result<()> {
    let dirs_to_create = [
        "ui",
        "decay",
        "mounts", 
        "levels",
        "maps",
        "dialogue",
        "audio",
        "hbf_analysis", // For HBF import functionality
    ];
    
    for dir in &dirs_to_create {
        let dir_path = out_dir.join(dir);
        std::fs::create_dir_all(&dir_path)?;
        tracing::info!("Created directory: {}", dir_path.display());
    }
    
    Ok(())
}

/// Placeholder generation report for backwards compatibility
#[derive(Debug, Clone)]
pub struct GenerationReport {
    pub dread_level: u8,
    pub successful_agents: Vec<String>,
    pub failed_agents: Vec<String>,
    pub warnings: Vec<String>,
}

impl GenerationReport {
    pub fn is_successful(&self) -> bool {
        self.failed_agents.is_empty()
    }

    pub fn has_critical_failures(&self) -> bool {
        // Consider UI, Decay, Mount, and Levels as critical
        let critical_agents = ["ui_generation", "decay_modeling", "mount_system", "level_design"];
        
        for failed in &self.failed_agents {
            for critical in &critical_agents {
                if failed.starts_with(critical) {
                    return true;
                }
            }
        }
        false
    }

    pub fn summary(&self) -> String {
        format!(
            "Dread Level {} Generation: {} successful, {} failed{}",
            self.dread_level,
            self.successful_agents.len(),
            self.failed_agents.len(),
            if self.has_critical_failures() { " (CRITICAL FAILURES)" } else { "" }
        )
    }
}

/// Convert spec-driven report to old-style reports for backwards compatibility
pub fn convert_spec_report_to_legacy(spec_report: SpecDrivenGenerationReport) -> Vec<GenerationReport> {
    spec_report.dread_levels.into_iter().map(|level_report| {
        GenerationReport {
            dread_level: level_report.dread_level,
            successful_agents: level_report.successful_agents,
            failed_agents: level_report.failed_agents,
            warnings: Vec::new(), // No warnings in new system yet
        }
    }).collect()
}

/// Generate content for all dread levels (backwards compatibility function)
pub async fn generate_all_dread_levels(
    out_dir: &std::path::Path,
    openai_client: crate::openai_client::OpenAIClient,
) -> Result<Vec<GenerationReport>> {
    tracing::info!("Starting legacy generate_all_dread_levels with spec-driven backend");
    
    let mut orchestrator = AgentOrchestrator::new(openai_client);
    
    // Add default spec directories
    orchestrator.add_spec_directory("crates/game-dialogue");
    orchestrator.add_spec_directory("crates/hexroll_exporter");
    orchestrator.add_spec_directory("crates/game-assets");
    orchestrator.add_spec_directory("crates/ai-bridge/specs");
    
    // Load specs
    orchestrator.load_specs().await?;
    
    // Create build context
    let context = BuildContext::new(out_dir)?;
    
    // Generate content
    let spec_report = orchestrator.generate_all_dread_levels(out_dir, &context).await?;
    
    // Convert to legacy format
    let legacy_reports = convert_spec_report_to_legacy(spec_report);
    
    tracing::info!(
        "Legacy generation complete: {} dread levels processed",
        legacy_reports.len()
    );
    
    Ok(legacy_reports)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_spec_report_to_legacy() {
        let spec_report = SpecDrivenGenerationReport {
            dread_levels: vec![
                DreadLevelReport {
                    dread_level: 0,
                    successful_agents: vec!["ui_generation".to_string()],
                    failed_agents: vec!["audio_generation".to_string()],
                },
                DreadLevelReport {
                    dread_level: 1,
                    successful_agents: vec!["ui_generation".to_string(), "decay_modeling".to_string()],
                    failed_agents: vec![],
                },
            ],
        };
        
        let legacy_reports = convert_spec_report_to_legacy(spec_report);
        
        assert_eq!(legacy_reports.len(), 2);
        assert_eq!(legacy_reports[0].dread_level, 0);
        assert_eq!(legacy_reports[0].successful_agents.len(), 1);
        assert_eq!(legacy_reports[0].failed_agents.len(), 1);
        assert_eq!(legacy_reports[1].dread_level, 1);
        assert_eq!(legacy_reports[1].successful_agents.len(), 2);
        assert_eq!(legacy_reports[1].failed_agents.len(), 0);
    }
}
