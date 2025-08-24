//! Spec-driven agent system for ai-bridge

use anyhow::Result;
use async_trait::async_trait;
use std::collections::HashMap;
use std::path::Path;

use crate::agent_spec::{AgentSpec, AgentSpecLoader, AgentConfigValue};
use crate::agent_executor::{AgentExecutor, AgentExecutionRequest, AgentExecutionResult};
use crate::context::BuildContext;
use crate::generation::{GenerationRequest, GenerationResult};
use crate::openai_client::OpenAIClient;

/// Spec-driven agent orchestrator that replaces hardcoded agents
pub struct SpecDrivenOrchestrator {
    spec_loader: AgentSpecLoader,
    executor: AgentExecutor,
}

impl SpecDrivenOrchestrator {
    /// Create a new spec-driven orchestrator
    pub fn new(openai_client: OpenAIClient) -> Self {
        Self {
            spec_loader: AgentSpecLoader::new(),
            executor: AgentExecutor::new(openai_client),
        }
    }

    /// Add a directory to search for agent specifications
    pub fn add_spec_directory<P: AsRef<Path>>(&mut self, path: P) {
        self.spec_loader.add_spec_directory(path);
    }

    /// Load all agent specifications from registered directories
    pub async fn load_specs(&mut self) -> Result<()> {
        self.spec_loader.load_all_specs()?;
        
        tracing::info!(
            "Loaded {} agent specifications",
            self.spec_loader.get_all_specs().len()
        );

        // Log loaded specs
        for (name, spec) in self.spec_loader.get_all_specs() {
            tracing::info!(
                "  - {} v{} ({}): {:?}",
                name,
                spec.metadata.version,
                spec.metadata.domain,
                spec.capabilities
            );
        }

        Ok(())
    }

    /// Execute an agent by name with the given request
    pub async fn execute_agent(
        &self,
        agent_name: &str,
        request: GenerationRequest,
        context: &BuildContext,
    ) -> Result<GenerationResult> {
        // Get agent spec
        let spec = self.spec_loader.get_spec(agent_name)
            .ok_or_else(|| anyhow::anyhow!("Agent spec not found: {}", agent_name))?;

        // Convert GenerationRequest to AgentExecutionRequest
        let exec_request = self.convert_generation_request(agent_name, request)?;

        // Execute the agent
        let result = self.executor.execute_agent(spec, exec_request, context).await?;

        // Convert result back to GenerationResult
        self.convert_execution_result(result)
    }

    /// Execute all agents of a specific capability for a dread level
    pub async fn execute_by_capability(
        &self,
        capability: &str,
        dread_level: u8,
        output_dir: &Path,
        context: &BuildContext,
    ) -> Result<Vec<GenerationResult>> {
        let agents = self.spec_loader.get_agents_by_capability(capability);
        
        if agents.is_empty() {
            tracing::warn!("No agents found with capability: {}", capability);
            return Ok(Vec::new());
        }

        let mut results = Vec::new();
        
        for spec in agents {
            tracing::info!("Executing agent {} for capability {}", spec.metadata.name, capability);
            
            let request = GenerationRequest::new(
                capability.to_string(),
                dread_level,
                format!("Generate {} content for dread level {}", capability, dread_level)
            );

            match self.execute_agent(&spec.metadata.name, request, context).await {
                Ok(result) => {
                    tracing::info!("Agent {} completed successfully", spec.metadata.name);
                    results.push(result);
                }
                Err(e) => {
                    tracing::error!("Agent {} failed: {}", spec.metadata.name, e);
                    // Continue with other agents
                }
            }
        }

        Ok(results)
    }

    /// Generate content for all dread levels using available agents
    pub async fn generate_all_dread_levels(
        &self,
        output_dir: &Path,
        context: &BuildContext,
    ) -> Result<SpecDrivenGenerationReport> {
        let mut report = SpecDrivenGenerationReport {
            dread_levels: Vec::new(),
        };

        for dread_level in 0..=4 {
            tracing::info!("Generating content for dread level {}", dread_level);
            
            let level_report = self.generate_dread_level(dread_level, output_dir, context).await?;
            report.dread_levels.push(level_report);
        }

        Ok(report)
    }

    /// Generate content for a specific dread level
    async fn generate_dread_level(
        &self,
        dread_level: u8,
        output_dir: &Path,
        context: &BuildContext,
    ) -> Result<DreadLevelReport> {
        let mut report = DreadLevelReport {
            dread_level,
            successful_agents: Vec::new(),
            failed_agents: Vec::new(),
        };

        // Define capabilities that should be generated for each dread level
        let capabilities = [
            "ui_generation",
            "decay_modeling",
            "mount_system",
            "level_design",
            "map_generation", 
            "dialogue_generation",
            "audio_generation",
            "hbf_analysis", // For HBF import functionality
        ];

        for capability in &capabilities {
            match self.execute_by_capability(capability, dread_level, output_dir, context).await {
                Ok(results) => {
                    for result in results {
                        let agent_name = result.metadata.get("agent_name")
                            .and_then(|v| v.as_str())
                            .unwrap_or("unknown");
                        report.successful_agents.push(format!("{} ({})", capability, agent_name));
                    }
                }
                Err(e) => {
                    tracing::error!("Failed to execute capability {}: {}", capability, e);
                    report.failed_agents.push(format!("{}: {}", capability, e));
                }
            }
        }

        Ok(report)
    }

    /// List all available agents and their capabilities
    pub fn list_agents(&self) -> Vec<AgentInfo> {
        self.spec_loader.get_all_specs()
            .values()
            .map(|spec| AgentInfo {
                name: spec.metadata.name.clone(),
                version: spec.metadata.version.clone(),
                description: spec.metadata.description.clone(),
                domain: spec.metadata.domain.clone(),
                capabilities: spec.capabilities.clone(),
            })
            .collect()
    }

    /// Convert GenerationRequest to AgentExecutionRequest
    fn convert_generation_request(
        &self,
        agent_name: &str,
        request: GenerationRequest,
    ) -> Result<AgentExecutionRequest> {
        let mut inputs = HashMap::new();
        inputs.insert("asset_type".to_string(), AgentConfigValue::String(request.asset_type));
        inputs.insert("description".to_string(), AgentConfigValue::String(request.description));

        let mut context = HashMap::new();
        context.insert("dread_level".to_string(), AgentConfigValue::Integer(request.dread_level as i64));

        // Add requirements as context
        for (key, value) in request.requirements {
            let context_value = match value {
                serde_json::Value::String(s) => AgentConfigValue::String(s),
                serde_json::Value::Number(n) => {
                    if let Some(i) = n.as_i64() {
                        AgentConfigValue::Integer(i)
                    } else if let Some(f) = n.as_f64() {
                        AgentConfigValue::Float(f)
                    } else {
                        AgentConfigValue::String(n.to_string())
                    }
                }
                serde_json::Value::Bool(b) => AgentConfigValue::Boolean(b),
                _ => AgentConfigValue::String(value.to_string()),
            };
            context.insert(key, context_value);
        }

        Ok(AgentExecutionRequest {
            agent_name: agent_name.to_string(),
            inputs,
            context,
            config_overrides: HashMap::new(),
            prompt_template: None,
        })
    }

    /// Convert AgentExecutionResult to GenerationResult
    fn convert_execution_result(&self, result: AgentExecutionResult) -> Result<GenerationResult> {
        if !result.success {
            let error_message = result.error.unwrap_or_else(|| "Unknown error".to_string());
            return Ok(GenerationResult::failure(result.agent_name, error_message));
        }

        let content = result.outputs.get("result")
            .and_then(|v| v.as_string())
            .unwrap_or("No content generated")
            .to_string();

        let agent_name = result.agent_name.clone();
        let duration_ms = result.metadata.duration_ms;

        let mut gen_result = GenerationResult::success(&agent_name)
            .with_source(crate::generation::AssetSource::Generated)
            .with_metadata("agent_name", serde_json::Value::String(agent_name))
            .with_metadata("duration_ms", serde_json::Value::Number(serde_json::Number::from(duration_ms)))
            .with_metadata("content", serde_json::Value::String(content));
        
        if let Some(model) = result.metadata.model {
            gen_result = gen_result.with_metadata("model", serde_json::Value::String(model));
        }
        
        if let Some(template) = result.metadata.template_used {
            gen_result = gen_result.with_metadata("template_used", serde_json::Value::String(template));
        }

        Ok(gen_result)
    }
}

#[derive(Debug, Clone)]
pub struct SpecDrivenGenerationReport {
    pub dread_levels: Vec<DreadLevelReport>,
}

#[derive(Debug, Clone)]
pub struct DreadLevelReport {
    pub dread_level: u8,
    pub successful_agents: Vec<String>,
    pub failed_agents: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct AgentInfo {
    pub name: String,
    pub version: String,
    pub description: String,
    pub domain: String,
    pub capabilities: Vec<String>,
}

impl SpecDrivenGenerationReport {
    pub fn total_successful(&self) -> usize {
        self.dread_levels.iter()
            .map(|level| level.successful_agents.len())
            .sum()
    }

    pub fn total_failed(&self) -> usize {
        self.dread_levels.iter()
            .map(|level| level.failed_agents.len())
            .sum()
    }

    pub fn summary(&self) -> String {
        format!(
            "Generated content for {} dread levels: {} successful, {} failed",
            self.dread_levels.len(),
            self.total_successful(),
            self.total_failed()
        )
    }
}

impl DreadLevelReport {
    pub fn is_successful(&self) -> bool {
        self.failed_agents.is_empty()
    }

    pub fn summary(&self) -> String {
        format!(
            "Dread Level {}: {} successful, {} failed",
            self.dread_level,
            self.successful_agents.len(),
            self.failed_agents.len()
        )
    }
}
