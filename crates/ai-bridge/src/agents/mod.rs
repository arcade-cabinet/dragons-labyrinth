pub mod audio;
pub mod decay;
pub mod dialogue;
pub mod levels;
pub mod maps;
pub mod mounts;
pub mod ui;
pub mod quests;  // Quest chain generation for narrative complexity

pub use audio::AudioAgent;
pub use decay::DecayAgent;
pub use dialogue::DialogueAgent;
pub use levels::LevelsAgent;
pub use maps::MapsAgent;
pub use mounts::MountAgent;
pub use ui::UIAgent;

// Export the Agent trait
pub use self::Agent;

use anyhow::Result;
use async_trait::async_trait;
use std::path::Path;
use crate::context::BuildContext;
use crate::generation::{GenerationRequest, GenerationResult};
use crate::mcp_client::MCPClient;

/// Common trait for all AI agents
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

/// Orchestrates all AI agents to generate content for a specific dread level
pub struct AgentOrchestrator {
    dread_level: u8,
    out_dir: std::path::PathBuf,
    mcp_client: MCPClient,
}

impl AgentOrchestrator {
    pub fn new(dread_level: u8, out_dir: &Path, mcp_client: MCPClient) -> Self {
        Self {
            dread_level,
            out_dir: out_dir.to_path_buf(),
            mcp_client,
        }
    }

    /// Generate all content for this dread level
    pub async fn generate_all(&self) -> Result<GenerationReport> {
        println!("AgentOrchestrator: Starting generation for dread level {}", self.dread_level);
        
        let mut report = GenerationReport {
            dread_level: self.dread_level,
            successful_agents: Vec::new(),
            failed_agents: Vec::new(),
            warnings: Vec::new(),
        };

        // Generate UI content
        match self.generate_ui_content().await {
            Ok(_) => {
                report.successful_agents.push("UIAgent".to_string());
                println!("AgentOrchestrator: UIAgent completed successfully");
            }
            Err(e) => {
                report.failed_agents.push(format!("UIAgent: {}", e));
                eprintln!("AgentOrchestrator: UIAgent failed: {}", e);
            }
        }

        // Generate decay content
        match self.generate_decay_content().await {
            Ok(_) => {
                report.successful_agents.push("DecayAgent".to_string());
                println!("AgentOrchestrator: DecayAgent completed successfully");
            }
            Err(e) => {
                report.failed_agents.push(format!("DecayAgent: {}", e));
                eprintln!("AgentOrchestrator: DecayAgent failed: {}", e);
            }
        }

        // Generate mount content
        match self.generate_mount_content().await {
            Ok(_) => {
                report.successful_agents.push("MountAgent".to_string());
                println!("AgentOrchestrator: MountAgent completed successfully");
            }
            Err(e) => {
                report.failed_agents.push(format!("MountAgent: {}", e));
                eprintln!("AgentOrchestrator: MountAgent failed: {}", e);
            }
        }

        // Generate levels content
        match self.generate_levels_content().await {
            Ok(_) => {
                report.successful_agents.push("LevelsAgent".to_string());
                println!("AgentOrchestrator: LevelsAgent completed successfully");
            }
            Err(e) => {
                report.failed_agents.push(format!("LevelsAgent: {}", e));
                eprintln!("AgentOrchestrator: LevelsAgent failed: {}", e);
            }
        }

        // Generate maps content (using existing agent)
        match self.generate_maps_content().await {
            Ok(_) => {
                report.successful_agents.push("MapsAgent".to_string());
                println!("AgentOrchestrator: MapsAgent completed successfully");
            }
            Err(e) => {
                report.failed_agents.push(format!("MapsAgent: {}", e));
                eprintln!("AgentOrchestrator: MapsAgent failed: {}", e);
            }
        }

        // Generate dialogue content (using existing agent)
        match self.generate_dialogue_content().await {
            Ok(_) => {
                report.successful_agents.push("DialogueAgent".to_string());
                println!("AgentOrchestrator: DialogueAgent completed successfully");
            }
            Err(e) => {
                report.failed_agents.push(format!("DialogueAgent: {}", e));
                eprintln!("AgentOrchestrator: DialogueAgent failed: {}", e);
            }
        }

        // Generate audio content (using existing agent)
        match self.generate_audio_content().await {
            Ok(_) => {
                report.successful_agents.push("AudioAgent".to_string());
                println!("AgentOrchestrator: AudioAgent completed successfully");
            }
            Err(e) => {
                report.failed_agents.push(format!("AudioAgent: {}", e));
                eprintln!("AgentOrchestrator: AudioAgent failed: {}", e);
            }
        }

        println!("AgentOrchestrator: Generation complete for dread level {}. Success: {}, Failed: {}", 
                 self.dread_level, report.successful_agents.len(), report.failed_agents.len());

        Ok(report)
    }

    async fn generate_ui_content(&self) -> Result<()> {
        let agent = UIAgent::new(self.dread_level, &self.out_dir, self.mcp_client.clone());
        agent.generate().await?;
        Ok(())
    }

    async fn generate_decay_content(&self) -> Result<()> {
        let agent = DecayAgent::new(self.dread_level, &self.out_dir, self.mcp_client.clone());
        agent.generate().await?;
        Ok(())
    }

    async fn generate_mount_content(&self) -> Result<()> {
        let agent = MountAgent::new(self.dread_level, &self.out_dir, self.mcp_client.clone());
        agent.generate().await?;
        Ok(())
    }

    async fn generate_levels_content(&self) -> Result<()> {
        let agent = LevelsAgent::new(self.dread_level, &self.out_dir, self.mcp_client.clone());
        agent.generate().await?;
        Ok(())
    }

    async fn generate_maps_content(&self) -> Result<()> {
        let agent = MapsAgent::new(self.dread_level, &self.out_dir, self.mcp_client.clone());
        agent.generate().await?;
        Ok(())
    }

    async fn generate_dialogue_content(&self) -> Result<()> {
        let agent = DialogueAgent::new(self.dread_level, &self.out_dir, self.mcp_client.clone());
        agent.generate().await?;
        Ok(())
    }

    async fn generate_audio_content(&self) -> Result<()> {
        let agent = AudioAgent::new(self.dread_level, &self.out_dir, self.mcp_client.clone());
        agent.generate().await?;
        Ok(())
    }
}

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
        let critical_agents = ["UIAgent", "DecayAgent", "MountAgent", "LevelsAgent"];
        
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

/// Generate content for all dread levels (0-4)
pub async fn generate_all_dread_levels(out_dir: &Path, mcp_client: MCPClient) -> Result<Vec<GenerationReport>> {
    println!("AgentOrchestrator: Starting full generation pipeline for all dread levels");
    
    let mut reports = Vec::new();
    
    for dread_level in 0..=4 {
        println!("AgentOrchestrator: Processing dread level {}", dread_level);
        
        let orchestrator = AgentOrchestrator::new(dread_level, out_dir, mcp_client.clone());
        let report = orchestrator.generate_all().await?;
        
        println!("AgentOrchestrator: {}", report.summary());
        
        // Log any failures
        for failure in &report.failed_agents {
            eprintln!("AgentOrchestrator: FAILURE - {}", failure);
        }
        
        reports.push(report);
    }
    
    // Summary statistics
    let total_successful: usize = reports.iter().map(|r| r.successful_agents.len()).sum();
    let total_failed: usize = reports.iter().map(|r| r.failed_agents.len()).sum();
    let critical_failures = reports.iter().filter(|r| r.has_critical_failures()).count();
    
    println!("AgentOrchestrator: PIPELINE COMPLETE");
    println!("AgentOrchestrator: Total successful: {}, Total failed: {}, Critical failures: {}", 
             total_successful, total_failed, critical_failures);
    
    if critical_failures > 0 {
        eprintln!("AgentOrchestrator: WARNING - {} dread levels have critical failures", critical_failures);
    }
    
    Ok(reports)
}

/// Utility function to create all necessary output directories
pub fn create_output_directories(out_dir: &Path) -> Result<()> {
    let dirs_to_create = [
        "ui",
        "decay", 
        "mounts",
        "levels",
        "maps",
        "dialogue", 
        "audio"
    ];
    
    for dir in &dirs_to_create {
        let dir_path = out_dir.join(dir);
        std::fs::create_dir_all(&dir_path)?;
        println!("AgentOrchestrator: Created directory: {}", dir_path.display());
    }
    
    Ok(())
}
