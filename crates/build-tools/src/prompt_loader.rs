// Prompt specification loader for TOML-based agent prompts

use std::path::{Path, PathBuf};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use anyhow::{Result, Context};

/// A single prompt specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptSpec {
    pub id: String,
    pub category: String,
    pub dread_levels: Vec<u8>,
    pub template: String,
}

/// Generation rules for a prompt category
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationRules {
    #[serde(flatten)]
    pub rules: HashMap<String, toml::Value>,
}

/// Complete specification file for an agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentPromptSpec {
    #[serde(flatten)]
    pub prompts: HashMap<String, Vec<PromptSpec>>,
    pub generation_rules: Option<GenerationRules>,
}

/// Orchestration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestrationConfig {
    pub orchestration: OrchestrationMeta,
    pub agents: HashMap<String, AgentConfig>,
    pub generation_phases: Vec<GenerationPhase>,
    pub batch_config: BatchConfig,
    pub api_config: ApiConfig,
    pub generation_targets: GenerationTargets,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestrationMeta {
    pub name: String,
    pub version: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    pub spec: String,
    pub agent: String,
    pub priority: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationPhase {
    pub name: String,
    pub duration_hours: f32,
    pub agents: Vec<String>,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchConfig {
    pub parallel_agents: usize,
    pub items_per_batch: usize,
    pub max_retries: u32,
    pub checkpoint_interval: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    pub openai_model: String,
    pub openai_temperature: f32,
    pub dalle_model: String,
    pub dalle_quality: String,
    pub freesound_batch_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationTargets {
    pub total_dialogues: usize,
    pub total_maps: usize,
    pub total_encounters: usize,
    pub total_items: usize,
    pub total_audio_clips: usize,
    pub total_ui_variations: usize,
    pub total_decay_patterns: usize,
}

/// Main prompt loader
pub struct PromptLoader {
    prompts_dir: PathBuf,
    orchestration: Option<OrchestrationConfig>,
    agent_specs: HashMap<String, AgentPromptSpec>,
}

impl PromptLoader {
    /// Create a new prompt loader
    pub fn new(prompts_dir: impl AsRef<Path>) -> Self {
        Self {
            prompts_dir: prompts_dir.as_ref().to_path_buf(),
            orchestration: None,
            agent_specs: HashMap::new(),
        }
    }

    /// Load orchestration configuration
    pub fn load_orchestration(&mut self) -> Result<&OrchestrationConfig> {
        let path = self.prompts_dir.join("orchestration.toml");
        let content = std::fs::read_to_string(&path)
            .context("Failed to read orchestration.toml")?;
        
        let config: OrchestrationConfig = toml::from_str(&content)
            .context("Failed to parse orchestration.toml")?;
        
        self.orchestration = Some(config);
        Ok(self.orchestration.as_ref().unwrap())
    }

    /// Load all agent specifications
    pub fn load_all_specs(&mut self) -> Result<()> {
        let orchestration = self.orchestration.as_ref()
            .context("Orchestration must be loaded first")?;
        
        for (agent_name, agent_config) in &orchestration.agents {
            let spec_path = self.prompts_dir.join(&agent_config.spec);
            if spec_path.exists() {
                let content = std::fs::read_to_string(&spec_path)
                    .with_context(|| format!("Failed to read {}", agent_config.spec))?;
                
                let spec: AgentPromptSpec = toml::from_str(&content)
                    .with_context(|| format!("Failed to parse {}", agent_config.spec))?;
                
                self.agent_specs.insert(agent_name.clone(), spec);
                tracing::info!("Loaded {} prompts for {}", 
                    spec.prompts.values().map(|v| v.len()).sum::<usize>(),
                    agent_name
                );
            } else {
                tracing::warn!("Spec file not found: {}", spec_path.display());
            }
        }
        
        Ok(())
    }

    /// Get prompts for a specific agent and category
    pub fn get_prompts(&self, agent: &str, category: &str) -> Option<&Vec<PromptSpec>> {
        self.agent_specs.get(agent)
            .and_then(|spec| {
                // Look for the category in the flattened structure
                spec.prompts.iter()
                    .find(|(key, prompts)| {
                        key.ends_with("_prompts") && 
                        prompts.iter().any(|p| p.category == category)
                    })
                    .map(|(_, prompts)| prompts)
            })
    }

    /// Get prompts filtered by dread level
    pub fn get_prompts_for_dread(&self, agent: &str, dread_level: u8) -> Vec<&PromptSpec> {
        self.agent_specs.get(agent)
            .map(|spec| {
                spec.prompts.values()
                    .flat_map(|prompts| prompts.iter())
                    .filter(|p| p.dread_levels.contains(&dread_level))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Render a prompt template with variables
    pub fn render_prompt(&self, template: &str, vars: &HashMap<&str, &str>) -> String {
        let mut result = template.to_string();
        for (key, value) in vars {
            let placeholder = format!("{{{}}}", key);
            result = result.replace(&placeholder, value);
        }
        result
    }

    /// Get generation targets
    pub fn get_generation_targets(&self) -> Option<&GenerationTargets> {
        self.orchestration.as_ref().map(|o| &o.generation_targets)
    }

    /// Get the next batch of work for an agent
    pub fn get_next_batch(&self, agent: &str, completed: usize) -> Vec<PromptSpec> {
        let batch_size = self.orchestration.as_ref()
            .map(|o| o.batch_config.items_per_batch)
            .unwrap_or(100);
        
        self.agent_specs.get(agent)
            .map(|spec| {
                spec.prompts.values()
                    .flat_map(|prompts| prompts.iter())
                    .skip(completed)
                    .take(batch_size)
                    .cloned()
                    .collect()
            })
            .unwrap_or_default()
    }
}

/// Progress tracker for overnight generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationProgress {
    pub agent: String,
    pub phase: String,
    pub completed_items: usize,
    pub total_items: usize,
    pub errors: Vec<String>,
    pub checkpoints: Vec<GenerationCheckpoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationCheckpoint {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub items_completed: usize,
    pub phase: String,
}

impl GenerationProgress {
    pub fn new(agent: String, phase: String, total_items: usize) -> Self {
        Self {
            agent,
            phase,
            completed_items: 0,
            total_items,
            errors: Vec::new(),
            checkpoints: Vec::new(),
        }
    }

    pub fn increment(&mut self, count: usize) {
        self.completed_items += count;
    }

    pub fn add_error(&mut self, error: String) {
        self.errors.push(error);
    }

    pub fn checkpoint(&mut self) {
        self.checkpoints.push(GenerationCheckpoint {
            timestamp: chrono::Utc::now(),
            items_completed: self.completed_items,
            phase: self.phase.clone(),
        });
    }

    pub fn save(&self, path: impl AsRef<Path>) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str(&json)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prompt_rendering() {
        let loader = PromptLoader::new(".");
        let template = "Generate dialogue for {character} at dread level {dread_level}";
        let mut vars = HashMap::new();
        vars.insert("character", "Therapist");
        vars.insert("dread_level", "3");
        
        let result = loader.render_prompt(template, &vars);
        assert_eq!(result, "Generate dialogue for Therapist at dread level 3");
    }
}
