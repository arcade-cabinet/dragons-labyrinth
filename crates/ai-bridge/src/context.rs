//! Build context and configuration for AI-powered asset generation

use std::path::{Path, PathBuf};
use openai_dive::v1::api::Client;
use openai_dive::v1::models::FlagshipModel;
use openai_dive::v1::resources::chat::ChatCompletionTool;
use tiktoken_rs::{p50k_base, CoreBPE};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::BuildToolError;
use crate::memory::AgentMemory;

/// Configuration for build tools
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildConfig {
    /// OpenAI model to use
    pub model: String,
    
    /// Maximum tokens per request
    pub max_tokens: usize,
    
    /// Temperature for generation
    pub temperature: f32,
    
    /// Enable caching of results
    pub enable_cache: bool,
    
    /// Dread level for horror progression (0-4)
    pub dread_level: u8,
    
    /// Asset categories to generate
    pub asset_categories: Vec<String>,
}

impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            model: FlagshipModel::Gpt4O.to_string(),
            max_tokens: 4000,
            temperature: 0.7,
            enable_cache: true,
            dread_level: 0,
            asset_categories: vec![
                "hex_tiles".to_string(),
                "companions".to_string(),
                "ui_elements".to_string(),
                "dialogue".to_string(),
                "audio".to_string(),
            ],
        }
    }
}

/// Main build context for AI-powered asset generation
pub struct BuildContext {
    /// OpenAI client for AI operations
    client: Client,
    
    /// Token counter for managing API costs
    tokenizer: CoreBPE,
    
    /// Output directory for generated assets
    output_dir: PathBuf,
    
    /// Cache directory for intermediate results
    cache_dir: PathBuf,
    
    /// Configuration for build process
    config: BuildConfig,
    
    /// Memory system for maintaining context
    memory: AgentMemory,
}

impl BuildContext {
    /// Create a new build context
    pub fn new(output_dir: impl AsRef<Path>) -> Result<Self> {
        let client = Client::new_from_env();
        let tokenizer = p50k_base()?;
        
        Ok(Self {
            client,
            tokenizer,
            output_dir: output_dir.as_ref().to_path_buf(),
            cache_dir: output_dir.as_ref().join(".cache"),
            config: BuildConfig::default(),
            memory: AgentMemory::new(),
        })
    }
    
    /// Create with custom configuration
    pub fn with_config(output_dir: impl AsRef<Path>, config: BuildConfig) -> Result<Self> {
        let mut context = Self::new(output_dir)?;
        context.config = config;
        Ok(context)
    }
    
    /// Get the configuration
    pub fn config(&self) -> &BuildConfig {
        &self.config
    }
    
    /// Get mutable configuration
    pub fn config_mut(&mut self) -> &mut BuildConfig {
        &mut self.config
    }
    
    /// Get the client
    pub fn client(&self) -> &Client {
        &self.client
    }
    
    /// Get the output directory
    pub fn output_dir(&self) -> &Path {
        &self.output_dir
    }
    
    /// Get the cache directory
    pub fn cache_dir(&self) -> &Path {
        &self.cache_dir
    }
    
    /// Get the memory system
    pub fn memory(&self) -> &AgentMemory {
        &self.memory
    }
    
    /// Get mutable memory system
    pub fn memory_mut(&mut self) -> &mut AgentMemory {
        &mut self.memory
    }
    
    /// Count tokens in a message
    pub fn count_tokens(&self, text: &str) -> usize {
        self.tokenizer.encode_with_special_tokens(text).len()
    }
    
    /// Check if text exceeds token limit
    pub fn check_token_limit(&self, text: &str) -> Result<()> {
        let tokens = self.count_tokens(text);
        if tokens > self.config.max_tokens {
            return Err(BuildToolError::TokenLimitExceeded {
                current: tokens,
                max: self.config.max_tokens,
            }.into());
        }
        Ok(())
    }
    
}
