use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AgentMetadata {
    name: String,
    version: String,
    description: String,
    domain: String,
    author: Option<String>,
    tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AgentInterface {
    inputs: Vec<serde_json::Value>,
    outputs: Vec<serde_json::Value>,
    required_context: Vec<String>,
    optional_context: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PromptTemplate {
    template: String,
    variables: Vec<String>,
    system_prompt: Option<String>,
    temperature: Option<f32>,
    max_tokens: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AgentSpec {
    metadata: AgentMetadata,
    capabilities: Vec<String>,
    interface: AgentInterface,
    prompts: HashMap<String, PromptTemplate>,
    config: HashMap<String, serde_json::Value>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string("agent.toml")?;
    println!("TOML content preview:\n{}\n", &content[..500.min(content.len())]);
    
    let spec: AgentSpec = toml::from_str(&content)?;
    println!("Successfully loaded: {} v{}", spec.metadata.name, spec.metadata.version);
    println!("Capabilities: {:?}", spec.capabilities);
    
    Ok(())
}
