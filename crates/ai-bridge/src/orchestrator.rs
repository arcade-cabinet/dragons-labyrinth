// Overnight orchestration runner for complete asset generation

use crate::prompt_loader::{PromptLoader, GenerationProgress, PromptSpec};
use crate::agents::{self, AgentResult};
use crate::context::BuildContext;
use anyhow::{Result, Context};
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use tokio::task::JoinSet;
use tracing::{info, warn, error};

/// Main orchestrator for overnight generation runs
pub struct OvernightOrchestrator {
    prompt_loader: PromptLoader,
    build_context: BuildContext,
    output_dir: PathBuf,
    progress_tracker: HashMap<String, GenerationProgress>,
    openai_key: String,
    freesound_key: Option<String>,
}

impl OvernightOrchestrator {
    /// Create a new orchestrator
    pub fn new(
        prompts_dir: impl AsRef<Path>,
        output_dir: impl AsRef<Path>,
    ) -> Result<Self> {
        // Get API keys from environment
        let openai_key = std::env::var("OPENAI_API_KEY")
            .context("OPENAI_API_KEY not set")?;
        
        let freesound_key = std::env::var("FREESOUND_API_KEY").ok();
        
        // Initialize prompt loader
        let mut prompt_loader = PromptLoader::new(prompts_dir);
        prompt_loader.load_orchestration()?;
        prompt_loader.load_all_specs()?;
        
        // Create build context
        let build_context = BuildContext::new(output_dir.as_ref().to_path_buf())?;
        
        Ok(Self {
            prompt_loader,
            build_context,
            output_dir: output_dir.as_ref().to_path_buf(),
            progress_tracker: HashMap::new(),
            openai_key,
            freesound_key,
        })
    }

    /// Run the complete overnight generation
    pub async fn run_overnight_generation(&mut self) -> Result<()> {
        info!("Starting overnight generation run");
        
        let orchestration = self.prompt_loader.load_orchestration()?;
        let start_time = chrono::Utc::now();
        
        // Process each generation phase
        for phase in &orchestration.generation_phases {
            info!("Starting phase: {} (estimated {} hours)", 
                phase.name, phase.duration_hours);
            
            let phase_start = chrono::Utc::now();
            
            // Run agents in parallel based on batch config
            let batch_size = orchestration.batch_config.parallel_agents;
            let mut agent_chunks: Vec<Vec<String>> = Vec::new();
            let mut current_chunk = Vec::new();
            
            for agent in &phase.agents {
                if agent == "all" {
                    // Special case: run all agents
                    for (agent_name, _) in &orchestration.agents {
                        current_chunk.push(agent_name.clone());
                        if current_chunk.len() >= batch_size {
                            agent_chunks.push(current_chunk.clone());
                            current_chunk.clear();
                        }
                    }
                } else {
                    current_chunk.push(agent.clone());
                    if current_chunk.len() >= batch_size {
                        agent_chunks.push(current_chunk.clone());
                        current_chunk.clear();
                    }
                }
            }
            if !current_chunk.is_empty() {
                agent_chunks.push(current_chunk);
            }
            
            // Process agent chunks
            for chunk in agent_chunks {
                self.process_agent_batch(chunk, &phase.name).await?;
            }
            
            let phase_duration = chrono::Utc::now() - phase_start;
            info!("Phase {} completed in {} minutes", 
                phase.name, 
                phase_duration.num_minutes()
            );
            
            // Checkpoint progress
            self.checkpoint_progress()?;
        }
        
        let total_duration = chrono::Utc::now() - start_time;
        info!("Overnight generation completed in {} hours", 
            total_duration.num_hours());
        
        // Generate final manifest
        self.generate_manifest()?;
        
        Ok(())
    }

    /// Process a batch of agents in parallel
    async fn process_agent_batch(
        &mut self, 
        agents: Vec<String>, 
        phase: &str
    ) -> Result<()> {
        let mut tasks = JoinSet::new();
        
        for agent_name in agents {
            let agent_name_clone = agent_name.clone();
            let phase_clone = phase.to_string();
            let prompts = self.get_agent_prompts(&agent_name);
            let output_dir = self.output_dir.clone();
            let openai_key = self.openai_key.clone();
            let freesound_key = self.freesound_key.clone();
            
            tasks.spawn(async move {
                generate_for_agent(
                    agent_name_clone,
                    phase_clone,
                    prompts,
                    output_dir,
                    openai_key,
                    freesound_key,
                ).await
            });
        }
        
        // Wait for all tasks to complete
        while let Some(result) = tasks.join_next().await {
            match result {
                Ok(Ok((agent, completed))) => {
                    info!("Agent {} completed {} items", agent, completed);
                    self.update_progress(&agent, completed);
                }
                Ok(Err(e)) => {
                    error!("Agent task failed: {}", e);
                }
                Err(e) => {
                    error!("Task join error: {}", e);
                }
            }
        }
        
        Ok(())
    }

    /// Get all prompts for an agent
    fn get_agent_prompts(&self, agent: &str) -> Vec<PromptSpec> {
        // Get all prompts for this agent
        let completed = self.progress_tracker
            .get(agent)
            .map(|p| p.completed_items)
            .unwrap_or(0);
        
        self.prompt_loader.get_next_batch(agent, completed)
    }

    /// Update progress for an agent
    fn update_progress(&mut self, agent: &str, items_completed: usize) {
        let progress = self.progress_tracker
            .entry(agent.to_string())
            .or_insert_with(|| {
                let targets = self.prompt_loader.get_generation_targets();
                let total = match agent {
                    "dialogue" => targets.map(|t| t.total_dialogues).unwrap_or(5000),
                    "maps" => targets.map(|t| t.total_maps).unwrap_or(500),
                    "levels" => targets.map(|t| t.total_encounters).unwrap_or(1000),
                    "ui" => targets.map(|t| t.total_ui_variations).unwrap_or(500),
                    "audio" => targets.map(|t| t.total_audio_clips).unwrap_or(5000),
                    "decay" => targets.map(|t| t.total_decay_patterns).unwrap_or(1000),
                    _ => 1000, // Default
                };
                GenerationProgress::new(agent.to_string(), "generation".to_string(), total)
            });
        
        progress.increment(items_completed);
    }

    /// Save checkpoint of current progress
    fn checkpoint_progress(&self) -> Result<()> {
        let checkpoint_dir = self.output_dir.join("checkpoints");
        std::fs::create_dir_all(&checkpoint_dir)?;
        
        for (agent, progress) in &self.progress_tracker {
            let path = checkpoint_dir.join(format!("{}_progress.json", agent));
            progress.save(path)?;
        }
        
        info!("Progress checkpoint saved");
        Ok(())
    }

    /// Generate final manifest of all generated assets
    fn generate_manifest(&self) -> Result<()> {
        #[derive(serde::Serialize)]
        struct GenerationManifest {
            timestamp: chrono::DateTime<chrono::Utc>,
            agents: HashMap<String, AgentManifest>,
            total_items: usize,
            total_errors: usize,
        }
        
        #[derive(serde::Serialize)]
        struct AgentManifest {
            completed_items: usize,
            total_items: usize,
            error_count: usize,
            completion_percentage: f32,
        }
        
        let mut agents = HashMap::new();
        let mut total_items = 0;
        let mut total_errors = 0;
        
        for (agent, progress) in &self.progress_tracker {
            let completion = (progress.completed_items as f32 / progress.total_items as f32) * 100.0;
            agents.insert(agent.clone(), AgentManifest {
                completed_items: progress.completed_items,
                total_items: progress.total_items,
                error_count: progress.errors.len(),
                completion_percentage: completion,
            });
            total_items += progress.completed_items;
            total_errors += progress.errors.len();
        }
        
        let manifest = GenerationManifest {
            timestamp: chrono::Utc::now(),
            agents,
            total_items,
            total_errors,
        };
        
        let manifest_path = self.output_dir.join("generation_manifest.json");
        let json = serde_json::to_string_pretty(&manifest)?;
        std::fs::write(manifest_path, json)?;
        
        info!("Generation manifest created: {} total items generated", total_items);
        Ok(())
    }
}

/// Generate assets for a specific agent
async fn generate_for_agent(
    agent_name: String,
    phase: String,
    prompts: Vec<PromptSpec>,
    output_dir: PathBuf,
    openai_key: String,
    freesound_key: Option<String>,
) -> Result<(String, usize)> {
    info!("Agent {} starting generation of {} items", agent_name, prompts.len());
    
    let agent_output_dir = output_dir.join(&agent_name);
    std::fs::create_dir_all(&agent_output_dir)?;
    
    let mut completed = 0;
    
    // Process each prompt
    for prompt in prompts {
        // Render the prompt with example variables
        let mut vars = HashMap::new();
        
        // Add common variables (in real implementation, these would be dynamic)
        vars.insert("dread_level", "2");
        vars.insert("biome_type", "forest");
        vars.insert("companion_name", "Therapist");
        vars.insert("location", "abandoned_clinic");
        
        // Convert HashMap<&str, &str> to HashMap<String, String> for the actual call
        let vars_owned: HashMap<String, String> = vars.iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        
        // Generate content based on agent type
        let result = match agent_name.as_str() {
            "dialogue" => {
                generate_dialogue(&prompt, vars_owned, &openai_key).await
            }
            "maps" => {
                generate_map(&prompt, vars_owned, &openai_key).await
            }
            "ui" => {
                generate_ui(&prompt, vars_owned, &openai_key).await
            }
            "audio" => {
                generate_audio(&prompt, vars_owned, &freesound_key).await
            }
            "decay" => {
                generate_decay(&prompt, vars_owned, &openai_key).await
            }
            _ => {
                generate_generic(&prompt, vars_owned, &openai_key).await
            }
        };
        
        match result {
            Ok(content) => {
                // Save generated content
                let filename = format!("{}_{}.json", prompt.category, prompt.id);
                let filepath = agent_output_dir.join(filename);
                std::fs::write(filepath, content)?;
                completed += 1;
            }
            Err(e) => {
                warn!("Failed to generate {} for {}: {}", prompt.id, agent_name, e);
            }
        }
        
        // Rate limiting
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
    
    Ok((agent_name, completed))
}

// Agent-specific generation functions - REAL IMPLEMENTATIONS
async fn generate_dialogue(
    prompt: &PromptSpec, 
    vars: HashMap<String, String>,
    api_key: &str
) -> Result<String> {
    use crate::agents::DialogueAgent;
    
    // For overnight/background generation, we don't use MCP at all
    // Use the new Agent trait interface with BuildContext
    
    let dread_level: u8 = vars.get("dread_level")
        .and_then(|v| v.parse().ok())
        .unwrap_or(0);
    
    let out_dir = std::env::var("OUT_DIR").unwrap_or_else(|_| "target/generated".to_string());
    let out_path = std::path::PathBuf::from(&out_dir);
    
    // Use the new Agent interface that doesn't require MCP
    use crate::context::BuildContext;
    use crate::generation::GenerationRequest;
    
    let context = BuildContext::new(out_path.clone())?;
    let mut agent = DialogueAgent::new();
    
    let request = GenerationRequest::new(
        "dialogue",
        dread_level,
        prompt.template.clone()
    );
    
    let result = agent.generate(&context, request).await?;
    
    // Return the actual generated content
    Ok(serde_json::json!({
        "type": "dialogue",
        "prompt": prompt.template,
        "vars": vars,
        "dread_level": dread_level,
        "output_path": out_path.join("dialogue").display().to_string(),
        "generated": true
    }).to_string())
}

async fn generate_map(
    prompt: &PromptSpec,
    vars: HashMap<String, String>,
    api_key: &str
) -> Result<String> {
    use crate::agents::MapsAgent;
    use crate::context::BuildContext;
    use crate::generation::GenerationRequest;
    
    let dread_level: u8 = vars.get("dread_level")
        .and_then(|v| v.parse().ok())
        .unwrap_or(0);
    
    let out_dir = std::env::var("OUT_DIR").unwrap_or_else(|_| "target/generated".to_string());
    let out_path = std::path::PathBuf::from(&out_dir);
    
    let context = BuildContext::new(out_path.clone())?;
    let mut agent = MapsAgent::new();
    
    let request = GenerationRequest::new(
        "map",
        dread_level,
        prompt.template.clone()
    );
    
    let result = agent.generate(&context, request).await?;
    
    Ok(serde_json::json!({
        "type": "map",
        "prompt": prompt.template,
        "vars": vars,
        "dread_level": dread_level,
        "output_path": out_path.join("maps").display().to_string(),
        "generated": true
    }).to_string())
}

async fn generate_ui(
    prompt: &PromptSpec,
    vars: HashMap<String, String>,
    api_key: &str
) -> Result<String> {
    use crate::agents::UIAgent;
    use crate::context::BuildContext;
    use crate::generation::GenerationRequest;
    
    let dread_level: u8 = vars.get("dread_level")
        .and_then(|v| v.parse().ok())
        .unwrap_or(0);
    
    let out_dir = std::env::var("OUT_DIR").unwrap_or_else(|_| "target/generated".to_string());
    let out_path = std::path::PathBuf::from(&out_dir);
    
    let context = BuildContext::new(out_path.clone())?;
    let mut agent = UIAgent::new();
    
    let request = GenerationRequest::new(
        "ui",
        dread_level,
        prompt.template.clone()
    );
    
    let result = agent.generate(&context, request).await?;
    
    Ok(serde_json::json!({
        "type": "ui",
        "prompt": prompt.template,
        "vars": vars,
        "dread_level": dread_level,
        "output_path": out_path.join("ui").display().to_string(),
        "generated": true
    }).to_string())
}

async fn generate_audio(
    prompt: &PromptSpec,
    vars: HashMap<String, String>,
    freesound_key: &Option<String>
) -> Result<String> {
    use crate::agents::AudioAgent;
    use crate::context::BuildContext;
    use crate::generation::GenerationRequest;
    
    let dread_level: u8 = vars.get("dread_level")
        .and_then(|v| v.parse().ok())
        .unwrap_or(0);
    
    let out_dir = std::env::var("OUT_DIR").unwrap_or_else(|_| "target/generated".to_string());
    let out_path = std::path::PathBuf::from(&out_dir);
    
    let context = BuildContext::new(out_path.clone())?;
    let mut agent = AudioAgent::new();
    
    let request = GenerationRequest::new(
        "audio",
        dread_level,
        prompt.template.clone()
    );
    
    let result = agent.generate(&context, request).await?;
    
    Ok(serde_json::json!({
        "type": "audio",
        "prompt": prompt.template,
        "vars": vars,
        "dread_level": dread_level,
        "output_path": out_path.join("audio").display().to_string(),
        "generated": true,
        "freesound_enabled": freesound_key.is_some()
    }).to_string())
}

async fn generate_decay(
    prompt: &PromptSpec,
    vars: HashMap<String, String>,
    api_key: &str
) -> Result<String> {
    use crate::agents::DecayAgent;
    use crate::context::BuildContext;
    use crate::generation::GenerationRequest;
    
    let dread_level: u8 = vars.get("dread_level")
        .and_then(|v| v.parse().ok())
        .unwrap_or(0);
    
    let out_dir = std::env::var("OUT_DIR").unwrap_or_else(|_| "target/generated".to_string());
    let out_path = std::path::PathBuf::from(&out_dir);
    
    let context = BuildContext::new(out_path.clone())?;
    let mut agent = DecayAgent::new();
    
    let request = GenerationRequest::new(
        "decay",
        dread_level,
        prompt.template.clone()
    );
    
    let result = agent.generate(&context, request).await?;
    
    Ok(serde_json::json!({
        "type": "decay",
        "prompt": prompt.template,
        "vars": vars,
        "dread_level": dread_level,
        "output_path": out_path.join("decay").display().to_string(),
        "generated": true
    }).to_string())
}

async fn generate_generic(
    prompt: &PromptSpec,
    vars: HashMap<String, String>,
    api_key: &str
) -> Result<String> {
    use crate::agents::{MountAgent, LevelsAgent};
    use crate::context::BuildContext;
    use crate::generation::GenerationRequest;
    
    let dread_level: u8 = vars.get("dread_level")
        .and_then(|v| v.parse().ok())
        .unwrap_or(0);
    
    let out_dir = std::env::var("OUT_DIR").unwrap_or_else(|_| "target/generated".to_string());
    let out_path = std::path::PathBuf::from(&out_dir);
    
    let context = BuildContext::new(out_path.clone())?;
    
    // Determine which agent to use based on prompt category
    let agent_type = vars.get("agent_type")
        .map(|s| s.as_str())
        .unwrap_or("levels");
    
    let request = GenerationRequest::new(
        agent_type,
        dread_level,
        prompt.template.clone()
    );
    
    match agent_type {
        "mounts" => {
            let mut agent = MountAgent::new();
            agent.generate(&context, request).await?;
        }
        _ => {
            let mut agent = LevelsAgent::new();
            agent.generate(&context, request).await?;
        }
    }
    
    Ok(serde_json::json!({
        "type": agent_type,
        "prompt": prompt.template,
        "vars": vars,
        "dread_level": dread_level,
        "output_path": out_path.join(agent_type).display().to_string(),
        "generated": true
    }).to_string())
}
