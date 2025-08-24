//! Error types for build tools

use thiserror::Error;

/// Build tools error types
#[derive(Error, Debug)]
pub enum BuildToolError {
    #[error("OpenAI API error: {0}")]
    OpenAIError(String),
    
    #[error("Token limit exceeded: {current}/{max}")]
    TokenLimitExceeded { current: usize, max: usize },
    
    #[error("Asset generation failed: {0}")]
    GenerationFailed(String),
    
    #[error("Tool execution failed: {0}")]
    ToolExecutionFailed(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("Memory system error: {0}")]
    MemoryError(String),
    
    #[error("Agent specification error: {0}")]
    AgentSpecError(String),
    
    #[error("Agent execution error: {0}")]
    AgentExecutionError(String),
}

pub type Result<T> = std::result::Result<T, BuildToolError>;
