//! Error types for build tools

use thiserror::Error;
#[cfg(feature = "with-database")]
use game_database::DatabaseError;

/// Build tools error types
#[derive(Error, Debug)]
pub enum BuildToolError {
    #[error("OpenAI API error: {0}")]
    OpenAIError(String),
    
    #[cfg(feature = "with-database")]
    #[error("Database error: {0}")]
    DatabaseError(#[from] DatabaseError),
    
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
}

pub type Result<T> = std::result::Result<T, BuildToolError>;
