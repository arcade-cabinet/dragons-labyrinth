//! Generic agent executor for spec-driven AI agents

use anyhow::{Context, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::agent_spec::{AgentSpec, AgentConfigValue, PromptTemplate};
use crate::context::BuildContext;
use crate::openai_client::OpenAIClient;
use crate::generation::{GenerationRequest, GenerationResult};

/// Execution request for an agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentExecutionRequest {
    /// Agent name to execute
    pub agent_name: String,
    
    /// Input data for the agent
    pub inputs: HashMap<String, AgentConfigValue>,
    
    /// Context data
    pub context: HashMap<String, AgentConfigValue>,
    
    /// Execution configuration overrides
    pub config_overrides: HashMap<String, AgentConfigValue>,
    
    /// Specific prompt template to use (optional)
    pub prompt_template: Option<String>,
}

/// Execution result from an agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentExecutionResult {
    /// Agent that was executed
    pub agent_name: String,
    
    /// Success status
    pub success: bool,
    
    /// Output data
    pub outputs: HashMap<String, AgentConfigValue>,
    
    /// Error message if failed
    pub error: Option<String>,
    
    /// Execution metadata
    pub metadata: ExecutionMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionMetadata {
    /// Execution duration in milliseconds
    pub duration_ms: u64,
    
    /// OpenAI tokens used (if applicable)
    pub tokens_used: Option<u32>,
    
    /// Model used for generation
    pub model: Option<String>,
    
    /// Template used
    pub template_used: Option<String>,
    
    /// Warnings during execution
    pub warnings: Vec<String>,
}

/// Generic agent executor that can run any agent based on its specification
pub struct AgentExecutor {
    openai_client: OpenAIClient,
}

impl AgentExecutor {
    pub fn new(openai_client: OpenAIClient) -> Self {
        Self { openai_client }
    }

    /// Execute an agent based on its specification
    pub async fn execute_agent(
        &self,
        spec: &AgentSpec,
        request: AgentExecutionRequest,
        build_context: &BuildContext,
    ) -> Result<AgentExecutionResult> {
        let start_time = std::time::Instant::now();
        let mut warnings = Vec::new();

        tracing::info!("Executing agent: {}", spec.metadata.name);

        // Validate inputs
        self.validate_inputs(spec, &request.inputs)?;

        // Validate context
        self.validate_context(spec, &request.context)?;

        // Merge configuration
        let config = self.merge_config(spec, &request.config_overrides);

        // Select prompt template
        let template_name = request.prompt_template
            .as_deref()
            .unwrap_or("main");

        let template = spec.prompts.get(template_name)
            .with_context(|| format!("Prompt template '{}' not found for agent '{}'", template_name, spec.metadata.name))?;

        // Execute the agent
        let result = match self.execute_with_template(spec, template, &request, &config, build_context).await {
            Ok(outputs) => AgentExecutionResult {
                agent_name: spec.metadata.name.clone(),
                success: true,
                outputs,
                error: None,
                metadata: ExecutionMetadata {
                    duration_ms: start_time.elapsed().as_millis() as u64,
                    tokens_used: None, // TODO: Track token usage
                    model: config.get("model").and_then(|v| v.as_string()).map(|s| s.to_string()),
                    template_used: Some(template_name.to_string()),
                    warnings,
                },
            },
            Err(e) => AgentExecutionResult {
                agent_name: spec.metadata.name.clone(),
                success: false,
                outputs: HashMap::new(),
                error: Some(e.to_string()),
                metadata: ExecutionMetadata {
                    duration_ms: start_time.elapsed().as_millis() as u64,
                    tokens_used: None,
                    model: config.get("model").and_then(|v| v.as_string()).map(|s| s.to_string()),
                    template_used: Some(template_name.to_string()),
                    warnings,
                },
            }
        };

        tracing::info!(
            "Agent execution complete: {} - {} in {}ms", 
            spec.metadata.name,
            if result.success { "SUCCESS" } else { "FAILED" },
            result.metadata.duration_ms
        );

        Ok(result)
    }

    /// Execute agent using a specific prompt template
    async fn execute_with_template(
        &self,
        spec: &AgentSpec,
        template: &PromptTemplate,
        request: &AgentExecutionRequest,
        config: &HashMap<String, AgentConfigValue>,
        build_context: &BuildContext,
    ) -> Result<HashMap<String, AgentConfigValue>> {
        // Build prompt from template
        let prompt = self.build_prompt_from_template(template, &request.inputs, &request.context)?;

        // Get OpenAI parameters from template and config
        let temperature = template.temperature
            .or_else(|| config.get("temperature").and_then(|v| v.as_float()).map(|f| f as f32))
            .unwrap_or(0.7);

        let max_tokens = template.max_tokens
            .or_else(|| config.get("max_tokens").and_then(|v| v.as_integer()).map(|i| i as u32))
            .unwrap_or(1000);

        // Call OpenAI API
        let system_prompt = template.system_prompt.as_deref();
        let response = self.openai_client.generate(&prompt, system_prompt).await
            .with_context(|| format!("Failed to generate content for agent '{}'", spec.metadata.name))?;

        // Process response based on expected outputs
        let mut outputs = HashMap::new();
        
        // For now, simple string output - could be extended based on output spec
        outputs.insert("result".to_string(), AgentConfigValue::String(response));

        // Add any additional processing based on agent spec outputs
        for output_spec in &spec.interface.outputs {
            if !outputs.contains_key(&output_spec.name) && output_spec.name != "result" {
                // Provide default values or process response further
                match output_spec.data_type.as_str() {
                    "string" => {
                        outputs.insert(output_spec.name.clone(), AgentConfigValue::String("".to_string()));
                    }
                    "boolean" => {
                        outputs.insert(output_spec.name.clone(), AgentConfigValue::Boolean(false));
                    }
                    "integer" => {
                        outputs.insert(output_spec.name.clone(), AgentConfigValue::Integer(0));
                    }
                    "float" => {
                        outputs.insert(output_spec.name.clone(), AgentConfigValue::Float(0.0));
                    }
                    _ => {}
                }
            }
        }

        Ok(outputs)
    }

    /// Build prompt from template by substituting variables
    fn build_prompt_from_template(
        &self,
        template: &PromptTemplate,
        inputs: &HashMap<String, AgentConfigValue>,
        context: &HashMap<String, AgentConfigValue>,
    ) -> Result<String> {
        let mut prompt = template.template.clone();

        // Substitute input variables
        for (key, value) in inputs {
            let placeholder = format!("{{{}}}", key);
            let value_str = match value {
                AgentConfigValue::String(s) => s.clone(),
                AgentConfigValue::Integer(i) => i.to_string(),
                AgentConfigValue::Float(f) => f.to_string(),
                AgentConfigValue::Boolean(b) => b.to_string(),
                _ => serde_json::to_string(value)?,
            };
            prompt = prompt.replace(&placeholder, &value_str);
        }

        // Substitute context variables
        for (key, value) in context {
            let placeholder = format!("{{{}}}", key);
            let value_str = match value {
                AgentConfigValue::String(s) => s.clone(),
                AgentConfigValue::Integer(i) => i.to_string(),
                AgentConfigValue::Float(f) => f.to_string(),
                AgentConfigValue::Boolean(b) => b.to_string(),
                _ => serde_json::to_string(value)?,
            };
            prompt = prompt.replace(&placeholder, &value_str);
        }

        Ok(prompt)
    }

    /// Validate that required inputs are provided
    fn validate_inputs(
        &self,
        spec: &AgentSpec,
        inputs: &HashMap<String, AgentConfigValue>,
    ) -> Result<()> {
        for input_spec in &spec.interface.inputs {
            if input_spec.required && !inputs.contains_key(&input_spec.name) {
                return Err(anyhow::anyhow!(
                    "Required input '{}' not provided for agent '{}'",
                    input_spec.name,
                    spec.metadata.name
                ));
            }
        }
        Ok(())
    }

    /// Validate that required context is provided
    fn validate_context(
        &self,
        spec: &AgentSpec,
        context: &HashMap<String, AgentConfigValue>,
    ) -> Result<()> {
        for required_context in &spec.interface.required_context {
            if !context.contains_key(required_context) {
                return Err(anyhow::anyhow!(
                    "Required context '{}' not provided for agent '{}'",
                    required_context,
                    spec.metadata.name
                ));
            }
        }
        Ok(())
    }

    /// Merge agent config with execution overrides
    fn merge_config(
        &self,
        spec: &AgentSpec,
        overrides: &HashMap<String, AgentConfigValue>,
    ) -> HashMap<String, AgentConfigValue> {
        let mut config = spec.config.clone();
        for (key, value) in overrides {
            config.insert(key.clone(), value.clone());
        }
        config
    }
}

/// Simplified interface for backwards compatibility with existing Agent trait
#[async_trait]
pub trait SpecDrivenAgent: Send + Sync {
    /// Execute the agent with a spec-driven approach
    async fn execute_spec(
        &mut self,
        context: &BuildContext,
        request: GenerationRequest,
    ) -> Result<GenerationResult>;

    /// Get the agent's spec name
    fn spec_name(&self) -> &str;
}

/// Agent wrapper that bridges between old Agent trait and new spec system
pub struct SpecBasedAgentWrapper {
    pub spec_name: String,
    pub executor: AgentExecutor,
}

impl SpecBasedAgentWrapper {
    pub fn new(spec_name: String, executor: AgentExecutor) -> Self {
        Self { spec_name, executor }
    }
}

#[async_trait]
impl SpecDrivenAgent for SpecBasedAgentWrapper {
    async fn execute_spec(
        &mut self,
        context: &BuildContext,
        request: GenerationRequest,
    ) -> Result<GenerationResult> {
        // Convert GenerationRequest to AgentExecutionRequest
        let mut inputs = HashMap::new();
        inputs.insert("asset_type".to_string(), AgentConfigValue::String(request.asset_type.clone()));
        inputs.insert("description".to_string(), AgentConfigValue::String(request.description.clone()));
        
        let mut context_data = HashMap::new();
        context_data.insert("dread_level".to_string(), AgentConfigValue::Integer(request.dread_level as i64));

        let exec_request = AgentExecutionRequest {
            agent_name: self.spec_name.clone(),
            inputs,
            context: context_data,
            config_overrides: HashMap::new(),
            prompt_template: None,
        };

        // This would require the spec to be available - for now return placeholder
        Ok(GenerationResult::success(format!("{}-generated", self.spec_name))
            .with_source(crate::generation::AssetSource::Generated)
            .with_metadata("agent_name", serde_json::Value::String(self.spec_name.clone()))
            .with_metadata("content", serde_json::Value::String("Generated content via spec-driven agent".to_string())))
    }

    fn spec_name(&self) -> &str {
        &self.spec_name
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent_spec::{AgentMetadata, AgentInterface, InputSpec, OutputSpec};

    fn create_test_spec() -> AgentSpec {
        AgentSpec {
            metadata: AgentMetadata {
                name: "test-agent".to_string(),
                version: "1.0.0".to_string(),
                description: "Test agent".to_string(),
                domain: "testing".to_string(),
                author: None,
                tags: vec![],
            },
            capabilities: vec!["generate".to_string()],
            interface: AgentInterface {
                inputs: vec![
                    InputSpec {
                        name: "prompt".to_string(),
                        data_type: "string".to_string(),
                        description: "Input prompt".to_string(),
                        required: true,
                        default_value: None,
                    },
                ],
                outputs: vec![
                    OutputSpec {
                        name: "result".to_string(),
                        data_type: "string".to_string(),
                        description: "Generated result".to_string(),
                        format: Some("text".to_string()),
                    },
                ],
                required_context: vec!["dread_level".to_string()],
                optional_context: vec![],
            },
            prompts: HashMap::from([
                (
                    "main".to_string(),
                    PromptTemplate {
                        template: "Generate content for dread level {dread_level}: {prompt}".to_string(),
                        variables: vec!["dread_level".to_string(), "prompt".to_string()],
                        system_prompt: None,
                        temperature: Some(0.7),
                        max_tokens: Some(1000),
                    },
                ),
            ]),
            config: HashMap::from([
                ("model".to_string(), AgentConfigValue::String("gpt-4".to_string())),
            ]),
        }
    }

    #[test]
    fn test_build_prompt_from_template() {
        let executor = AgentExecutor::new(OpenAIClient::new().unwrap());
        
        let template = PromptTemplate {
            template: "Hello {name}, your level is {level}!".to_string(),
            variables: vec!["name".to_string(), "level".to_string()],
            system_prompt: None,
            temperature: None,
            max_tokens: None,
        };

        let mut inputs = HashMap::new();
        inputs.insert("name".to_string(), AgentConfigValue::String("Alice".to_string()));

        let mut context = HashMap::new();
        context.insert("level".to_string(), AgentConfigValue::Integer(5));

        let result = executor.build_prompt_from_template(&template, &inputs, &context).unwrap();
        assert_eq!(result, "Hello Alice, your level is 5!");
    }

    #[test]
    fn test_validate_inputs() {
        let spec = create_test_spec();
        let executor = AgentExecutor::new(OpenAIClient::new().unwrap());

        // Valid inputs
        let mut inputs = HashMap::new();
        inputs.insert("prompt".to_string(), AgentConfigValue::String("test".to_string()));
        assert!(executor.validate_inputs(&spec, &inputs).is_ok());

        // Missing required input
        let empty_inputs = HashMap::new();
        assert!(executor.validate_inputs(&spec, &empty_inputs).is_err());
    }

    #[test]
    fn test_merge_config() {
        let spec = create_test_spec();
        let executor = AgentExecutor::new(OpenAIClient::new().unwrap());

        let mut overrides = HashMap::new();
        overrides.insert("temperature".to_string(), AgentConfigValue::Float(0.9));
        overrides.insert("model".to_string(), AgentConfigValue::String("gpt-3.5-turbo".to_string()));

        let merged = executor.merge_config(&spec, &overrides);
        
        assert_eq!(merged.get("temperature").unwrap().as_float().unwrap(), 0.9);
        assert_eq!(merged.get("model").unwrap().as_string().unwrap(), "gpt-3.5-turbo");
    }
}
