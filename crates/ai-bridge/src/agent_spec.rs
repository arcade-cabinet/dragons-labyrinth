//! Agent specification system for spec-driven AI agents

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Agent specification loaded from TOML files in domain crates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentSpec {
    /// Agent metadata
    pub metadata: AgentMetadata,
    
    /// Capabilities this agent provides
    pub capabilities: Vec<String>,
    
    /// Input/output specifications
    pub interface: AgentInterface,
    
    /// Prompt templates and configurations
    pub prompts: HashMap<String, PromptTemplate>,
    
    /// Configuration parameters
    pub config: HashMap<String, AgentConfigValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMetadata {
    pub name: String,
    pub version: String,
    pub description: String,
    pub domain: String,
    pub author: Option<String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentInterface {
    /// Input data schema
    pub inputs: Vec<InputSpec>,
    
    /// Output data schema
    pub outputs: Vec<OutputSpec>,
    
    /// Required context data
    pub required_context: Vec<String>,
    
    /// Optional context data
    pub optional_context: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputSpec {
    pub name: String,
    pub data_type: String,
    pub description: String,
    pub required: bool,
    pub default_value: Option<AgentConfigValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputSpec {
    pub name: String,
    pub data_type: String,
    pub description: String,
    pub format: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptTemplate {
    pub template: String,
    pub variables: Vec<String>,
    pub system_prompt: Option<String>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AgentConfigValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Array(Vec<AgentConfigValue>),
    Object(HashMap<String, AgentConfigValue>),
}

impl AgentConfigValue {
    pub fn as_string(&self) -> Option<&str> {
        match self {
            AgentConfigValue::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_integer(&self) -> Option<i64> {
        match self {
            AgentConfigValue::Integer(i) => Some(*i),
            _ => None,
        }
    }

    pub fn as_float(&self) -> Option<f64> {
        match self {
            AgentConfigValue::Float(f) => Some(*f),
            _ => None,
        }
    }

    pub fn as_boolean(&self) -> Option<bool> {
        match self {
            AgentConfigValue::Boolean(b) => Some(*b),
            _ => None,
        }
    }
}

/// Agent specification loader
pub struct AgentSpecLoader {
    loaded_specs: HashMap<String, AgentSpec>,
    spec_directories: Vec<PathBuf>,
}

impl AgentSpecLoader {
    pub fn new() -> Self {
        Self {
            loaded_specs: HashMap::new(),
            spec_directories: Vec::new(),
        }
    }

    /// Add a directory to search for agent specs
    pub fn add_spec_directory<P: AsRef<Path>>(&mut self, path: P) {
        self.spec_directories.push(path.as_ref().to_path_buf());
    }

    /// Load all agent specs from registered directories
    pub fn load_all_specs(&mut self) -> Result<()> {
        for spec_dir in &self.spec_directories.clone() {
            self.load_specs_from_directory(spec_dir)?;
        }
        Ok(())
    }

    /// Load agent specs from a specific directory
    pub fn load_specs_from_directory<P: AsRef<Path>>(&mut self, dir: P) -> Result<()> {
        let dir = dir.as_ref();
        
        if !dir.exists() {
            tracing::debug!("Agent spec directory does not exist: {}", dir.display());
            return Ok(());
        }

        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() && path.extension().map_or(false, |ext| ext == "toml") {
                self.load_spec_from_file(&path)?;
            }
        }

        Ok(())
    }

    /// Load a single agent spec from a file
    pub fn load_spec_from_file<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let path = path.as_ref();
        let content = std::fs::read_to_string(path)?;
        let spec: AgentSpec = toml::from_str(&content)?;
        
        tracing::info!("Loaded agent spec: {} v{}", spec.metadata.name, spec.metadata.version);
        self.loaded_specs.insert(spec.metadata.name.clone(), spec);
        
        Ok(())
    }

    /// Get a loaded agent spec by name
    pub fn get_spec(&self, name: &str) -> Option<&AgentSpec> {
        self.loaded_specs.get(name)
    }

    /// Get all loaded agent specs
    pub fn get_all_specs(&self) -> &HashMap<String, AgentSpec> {
        &self.loaded_specs
    }

    /// Get agents by capability
    pub fn get_agents_by_capability(&self, capability: &str) -> Vec<&AgentSpec> {
        self.loaded_specs
            .values()
            .filter(|spec| spec.capabilities.contains(&capability.to_string()))
            .collect()
    }

    /// Get agents by domain
    pub fn get_agents_by_domain(&self, domain: &str) -> Vec<&AgentSpec> {
        self.loaded_specs
            .values()
            .filter(|spec| spec.metadata.domain == domain)
            .collect()
    }

    /// Validate that all required specs are loaded
    pub fn validate_required_specs(&self, required: &[&str]) -> Result<()> {
        for spec_name in required {
            if !self.loaded_specs.contains_key(*spec_name) {
                return Err(anyhow::anyhow!("Required agent spec not found: {}", spec_name));
            }
        }
        Ok(())
    }
}

impl Default for AgentSpecLoader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn create_test_spec() -> AgentSpec {
        AgentSpec {
            metadata: AgentMetadata {
                name: "test-agent".to_string(),
                version: "1.0.0".to_string(),
                description: "Test agent".to_string(),
                domain: "testing".to_string(),
                author: Some("Test Author".to_string()),
                tags: vec!["test".to_string()],
            },
            capabilities: vec!["generate".to_string(), "analyze".to_string()],
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
                optional_context: vec!["player_history".to_string()],
            },
            prompts: HashMap::from([
                (
                    "main".to_string(),
                    PromptTemplate {
                        template: "Generate content for: {prompt}".to_string(),
                        variables: vec!["prompt".to_string()],
                        system_prompt: Some("You are a helpful assistant.".to_string()),
                        temperature: Some(0.7),
                        max_tokens: Some(1000),
                    },
                ),
            ]),
            config: HashMap::from([
                ("model".to_string(), AgentConfigValue::String("gpt-4".to_string())),
                ("max_retries".to_string(), AgentConfigValue::Integer(3)),
            ]),
        }
    }

    #[test]
    fn test_agent_spec_serialization() {
        let spec = create_test_spec();
        let toml_str = toml::to_string(&spec).unwrap();
        let deserialized: AgentSpec = toml::from_str(&toml_str).unwrap();
        
        assert_eq!(spec.metadata.name, deserialized.metadata.name);
        assert_eq!(spec.capabilities, deserialized.capabilities);
    }

    #[test]
    fn test_agent_spec_loader() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let spec_path = temp_dir.path().join("test-agent.toml");
        
        let spec = create_test_spec();
        let toml_content = toml::to_string(&spec)?;
        std::fs::write(&spec_path, toml_content)?;
        
        let mut loader = AgentSpecLoader::new();
        loader.add_spec_directory(temp_dir.path());
        loader.load_all_specs()?;
        
        assert!(loader.get_spec("test-agent").is_some());
        assert_eq!(loader.get_agents_by_domain("testing").len(), 1);
        assert_eq!(loader.get_agents_by_capability("generate").len(), 1);
        
        Ok(())
    }
}
