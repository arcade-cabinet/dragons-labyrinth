//! Error handling for Dragon's Labyrinth database operations
//! 
//! This module provides comprehensive error handling for all database operations,
//! with specific error types for different failure scenarios and integration
//! with both SeaORM and Bevy's error handling systems.

use thiserror::Error;
use sea_orm::DbErr;
use serde::{Serialize, Deserialize};
use std::fmt;

/// Main database error type for Dragon's Labyrinth
/// 
/// This enum covers all possible database operation failures, from connection
/// issues to game-specific logic errors. Each variant provides detailed
/// context to help with debugging and user feedback.
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum DatabaseError {
    /// Database connection errors
    #[error("Database connection failed: {0}")]
    Connection(String),

    /// Transaction-related errors
    #[error("Database transaction failed: {0}")]
    Transaction(String),

    /// Migration and schema errors
    #[error("Database migration failed: {0}")]
    Migration(String),

    /// Query execution errors
    #[error("Database query failed: {0}")]
    Query(String),

    /// Query execution errors (alternate naming for compatibility)
    #[error("Database query failed: {0}")]
    QueryFailed(String),

    /// Invalid query format or parameters
    #[error("Invalid query: {0}")]
    InvalidQuery(String),

    /// Data validation errors
    #[error("Data validation failed: {0}")]
    Validation(String),

    /// Constraint violation errors (foreign key, unique, etc.)
    #[error("Database constraint violation: {constraint_type} - {details}")]
    ConstraintViolation {
        constraint_type: String,
        details: String,
    },

    /// Entity not found errors
    #[error("Entity not found: {entity_type} with {identifier_type} = {identifier_value}")]
    EntityNotFound {
        entity_type: String,
        identifier_type: String,
        identifier_value: String,
    },

    /// Duplicate entity errors
    #[error("Duplicate entity: {entity_type} with {identifier_type} = {identifier_value} already exists")]
    DuplicateEntity {
        entity_type: String,
        identifier_type: String,
        identifier_value: String,
    },

    /// Serialization/deserialization errors for JSON fields
    #[error("JSON serialization error: {0}")]
    JsonSerialization(String),

    /// Save slot specific errors
    #[error("Save slot error: {0}")]
    SaveSlot(String),

    /// Horror progression validation errors
    #[error("Horror progression error: {0}")]
    HorrorProgression(String),

    /// Companion system errors
    #[error("Companion system error: {0}")]
    CompanionSystem(String),

    /// Hex world system errors
    #[error("Hex world error: {0}")]
    HexWorld(String),

    /// AI workflow errors
    #[error("AI workflow error: {workflow_type} - {details}")]
    AIWorkflow {
        workflow_type: String,
        details: String,
    },

    /// Asset management errors
    #[error("Asset management error: {asset_type} - {details}")]
    AssetManagement {
        asset_type: String,
        details: String,
    },

    /// ECS integration errors
    #[error("ECS integration error: {0}")]
    EcsIntegration(String),

    /// Permission/authorization errors
    #[error("Permission denied: {operation} requires {required_permission}")]
    PermissionDenied {
        operation: String,
        required_permission: String,
    },

    /// Rate limiting errors for AI operations
    #[error("Rate limit exceeded: {operation} - {retry_after_seconds} seconds until retry")]
    RateLimitExceeded {
        operation: String,
        retry_after_seconds: u64,
    },

    /// Configuration errors
    #[error("Configuration error: {0}")]
    Configuration(String),

    /// Internal/unexpected errors
    #[error("Internal database error: {0}")]
    Internal(String),

    /// UUID parsing errors
    #[error("Invalid UUID: {0}")]
    InvalidUuid(String),

    /// Data integrity errors
    #[error("Data integrity violation: {0}")]
    DataIntegrity(String),

    /// Concurrency/locking errors
    #[error("Concurrency error: {0}")]
    Concurrency(String),

    /// External service integration errors (OpenAI, etc.)
    #[error("External service error: {service} - {details}")]
    ExternalService {
        service: String,
        details: String,
    },
}

/// Result type alias for database operations
pub type DatabaseResult<T> = Result<T, DatabaseError>;

// Implement conversions from common error types

impl From<DbErr> for DatabaseError {
    fn from(err: DbErr) -> Self {
        match err {
            DbErr::ConnectionAcquire(e) => DatabaseError::Connection(e.to_string()),
            DbErr::TryIntoErr { from, into, source } => {
                DatabaseError::Query(format!("Type conversion error: {} -> {}: {}", from, into, source))
            },
            DbErr::Conn(e) => DatabaseError::Connection(e.to_string()),
            DbErr::Exec(e) => DatabaseError::Query(e.to_string()),
            DbErr::Query(e) => DatabaseError::Query(e.to_string()),
            DbErr::ConvertFromU64(e) => DatabaseError::Query(format!("Conversion error: {}", e)),
            DbErr::UnpackInsertId => DatabaseError::Query("Failed to unpack insert ID".to_string()),
            DbErr::UpdateGetPrimaryKey => DatabaseError::Query("Failed to get primary key for update".to_string()),
            DbErr::RecordNotFound(e) => DatabaseError::EntityNotFound {
                entity_type: "Unknown".to_string(),
                identifier_type: "Unknown".to_string(),
                identifier_value: e,
            },
            DbErr::AttrNotSet(attr) => DatabaseError::Validation(format!("Required attribute not set: {}", attr)),
            DbErr::Custom(e) => DatabaseError::Internal(e),
            DbErr::Type(e) => DatabaseError::Internal(e),
            DbErr::Json(e) => DatabaseError::JsonSerialization(e),
            DbErr::Migration(e) => DatabaseError::Migration(e),
            DbErr::RecordNotInserted => DatabaseError::Internal("Record not inserted".to_string()),
            DbErr::RecordNotUpdated => DatabaseError::Internal("Record not updated".to_string()),
        }
    }
}

impl From<serde_json::Error> for DatabaseError {
    fn from(err: serde_json::Error) -> Self {
        DatabaseError::JsonSerialization(err.to_string())
    }
}

impl From<uuid::Error> for DatabaseError {
    fn from(err: uuid::Error) -> Self {
        DatabaseError::InvalidUuid(err.to_string())
    }
}

impl From<std::io::Error> for DatabaseError {
    fn from(err: std::io::Error) -> Self {
        DatabaseError::Internal(format!("IO error: {}", err))
    }
}

impl From<tokio::time::error::Elapsed> for DatabaseError {
    fn from(err: tokio::time::error::Elapsed) -> Self {
        DatabaseError::Internal(format!("Timeout error: {}", err))
    }
}

// Helper methods for creating specific error types

impl DatabaseError {
    /// Create a save slot error
    pub fn save_slot_not_found(slot_id: i32) -> Self {
        DatabaseError::EntityNotFound {
            entity_type: "SaveSlot".to_string(),
            identifier_type: "slot_id".to_string(),
            identifier_value: slot_id.to_string(),
        }
    }

    /// Create a save slot already occupied error
    pub fn save_slot_occupied(slot_id: i32) -> Self {
        DatabaseError::DuplicateEntity {
            entity_type: "SaveSlot".to_string(),
            identifier_type: "slot_id".to_string(),
            identifier_value: slot_id.to_string(),
        }
    }

    /// Create a player not found error
    pub fn player_not_found(player_id: uuid::Uuid) -> Self {
        DatabaseError::EntityNotFound {
            entity_type: "Player".to_string(),
            identifier_type: "player_id".to_string(),
            identifier_value: player_id.to_string(),
        }
    }

    /// Create a companion not found error
    pub fn companion_not_found(companion_id: uuid::Uuid) -> Self {
        DatabaseError::EntityNotFound {
            entity_type: "Companion".to_string(),
            identifier_type: "companion_id".to_string(),
            identifier_value: companion_id.to_string(),
        }
    }

    /// Create a hex tile not found error
    pub fn hex_tile_not_found(q: i32, r: i32) -> Self {
        DatabaseError::EntityNotFound {
            entity_type: "HexTile".to_string(),
            identifier_type: "coordinates".to_string(),
            identifier_value: format!("({}, {})", q, r),
        }
    }

    /// Create an invalid dread level error
    pub fn invalid_dread_level(level: i32) -> Self {
        DatabaseError::HorrorProgression(format!("Invalid dread level: {}. Must be 0-4.", level))
    }

    /// Create an invalid trauma level error
    pub fn invalid_trauma_level(level: f32) -> Self {
        DatabaseError::CompanionSystem(format!("Invalid trauma level: {}. Must be 0.0-1.0.", level))
    }

    /// Create an asset not found error
    pub fn asset_not_found(asset_id: uuid::Uuid) -> Self {
        DatabaseError::EntityNotFound {
            entity_type: "GeneratedAsset".to_string(),
            identifier_type: "asset_id".to_string(),
            identifier_value: asset_id.to_string(),
        }
    }

    /// Create an AI workflow error
    pub fn ai_workflow_failed(workflow_type: &str, details: &str) -> Self {
        DatabaseError::AIWorkflow {
            workflow_type: workflow_type.to_string(),
            details: details.to_string(),
        }
    }

    /// Create an asset approval required error
    pub fn asset_approval_required(asset_id: uuid::Uuid) -> Self {
        DatabaseError::AssetManagement {
            asset_type: "GeneratedAsset".to_string(),
            details: format!("Asset {} requires human approval before use", asset_id),
        }
    }

    /// Create a companion betrayal error (when trying to use a betrayed companion)
    pub fn companion_betrayed(companion_name: &str) -> Self {
        DatabaseError::CompanionSystem(format!("Companion {} has betrayed the player and is no longer available", companion_name))
    }

    /// Create a hex coordinate validation error
    pub fn invalid_hex_coordinates(q: i32, r: i32, s: i32) -> Self {
        DatabaseError::HexWorld(format!("Invalid hex coordinates: ({}, {}, {}). Sum must equal zero.", q, r, s))
    }

    /// Create a data integrity error for related entities
    pub fn related_entity_integrity(entity_type: &str, related_entity: &str, details: &str) -> Self {
        DatabaseError::DataIntegrity(format!(
            "Data integrity violation between {} and {}: {}", 
            entity_type, related_entity, details
        ))
    }

    /// Create a configuration error for missing required settings
    pub fn missing_configuration(config_key: &str) -> Self {
        DatabaseError::Configuration(format!("Missing required configuration: {}", config_key))
    }

    /// Create an external service error (for AI API calls, etc.)
    pub fn external_service_error(service: &str, details: &str) -> Self {
        DatabaseError::ExternalService {
            service: service.to_string(),
            details: details.to_string(),
        }
    }

    /// Create a rate limit error for AI operations
    pub fn ai_rate_limit_exceeded(operation: &str, retry_after_seconds: u64) -> Self {
        DatabaseError::RateLimitExceeded {
            operation: operation.to_string(),
            retry_after_seconds,
        }
    }
}

/// Error severity levels for logging and user feedback
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ErrorSeverity {
    /// Low severity - user can continue, functionality may be degraded
    Low,
    /// Medium severity - user can continue but should take action
    Medium,
    /// High severity - user should stop current action
    High,
    /// Critical severity - application may be unstable
    Critical,
}

impl DatabaseError {
    /// Get the severity level of this error
    pub fn severity(&self) -> ErrorSeverity {
        match self {
            DatabaseError::EntityNotFound { .. } => ErrorSeverity::Low,
            DatabaseError::Validation(_) => ErrorSeverity::Low,
            DatabaseError::JsonSerialization(_) => ErrorSeverity::Medium,
            DatabaseError::SaveSlot(_) => ErrorSeverity::Medium,
            DatabaseError::HorrorProgression(_) => ErrorSeverity::Medium,
            DatabaseError::CompanionSystem(_) => ErrorSeverity::Medium,
            DatabaseError::HexWorld(_) => ErrorSeverity::Medium,
            DatabaseError::AssetManagement { .. } => ErrorSeverity::Medium,
            DatabaseError::InvalidUuid(_) => ErrorSeverity::Medium,
            DatabaseError::PermissionDenied { .. } => ErrorSeverity::High,
            DatabaseError::RateLimitExceeded { .. } => ErrorSeverity::High,
            DatabaseError::DataIntegrity(_) => ErrorSeverity::High,
            DatabaseError::Connection(_) => ErrorSeverity::Critical,
            DatabaseError::Transaction(_) => ErrorSeverity::Critical,
            DatabaseError::Migration(_) => ErrorSeverity::Critical,
            DatabaseError::Query(_) => ErrorSeverity::Critical,
            DatabaseError::ConstraintViolation { .. } => ErrorSeverity::Critical,
            DatabaseError::DuplicateEntity { .. } => ErrorSeverity::Medium,
            DatabaseError::AIWorkflow { .. } => ErrorSeverity::Medium,
            DatabaseError::EcsIntegration(_) => ErrorSeverity::High,
            DatabaseError::Configuration(_) => ErrorSeverity::Critical,
            DatabaseError::Internal(_) => ErrorSeverity::Critical,
            DatabaseError::Concurrency(_) => ErrorSeverity::High,
            DatabaseError::ExternalService { .. } => ErrorSeverity::Medium,
            DatabaseError::QueryFailed(_) => ErrorSeverity::High,
            DatabaseError::InvalidQuery(_) => ErrorSeverity::Medium,
        }
    }

    /// Get a user-friendly error message
    pub fn user_message(&self) -> String {
        match self {
            DatabaseError::EntityNotFound { entity_type, .. } => {
                match entity_type.as_str() {
                    "Player" => "Save file not found. Please check your save slot.".to_string(),
                    "Companion" => "Companion not found. They may have left your party.".to_string(),
                    "HexTile" => "Location not found on the map.".to_string(),
                    "GeneratedAsset" => "Required game asset missing. The game may need to regenerate content.".to_string(),
                    _ => "Requested item not found.".to_string(),
                }
            },
            DatabaseError::SaveSlot(_) => "Save slot issue. Please try a different save slot.".to_string(),
            DatabaseError::CompanionSystem(msg) if msg.contains("betrayed") => {
                "This companion is no longer available due to story events.".to_string()
            },
            DatabaseError::HorrorProgression(_) => "Horror progression error. Your save may be corrupted.".to_string(),
            DatabaseError::RateLimitExceeded { retry_after_seconds, .. } => {
                format!("Too many requests. Please wait {} seconds before trying again.", retry_after_seconds)
            },
            DatabaseError::Connection(_) => "Cannot connect to game database. Please restart the game.".to_string(),
            DatabaseError::AIWorkflow { .. } => "AI content generation failed. Some features may be unavailable.".to_string(),
            DatabaseError::AssetManagement { .. } => "Asset loading failed. Some graphics or audio may be missing.".to_string(),
            _ => "A database error occurred. Please try again.".to_string(),
        }
    }

    /// Check if this error should trigger a retry
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            DatabaseError::Connection(_) |
            DatabaseError::Query(_) |
            DatabaseError::Transaction(_) |
            DatabaseError::Concurrency(_) |
            DatabaseError::ExternalService { .. }
        )
    }

    /// Check if this error indicates a temporary issue
    pub fn is_temporary(&self) -> bool {
        matches!(
            self,
            DatabaseError::Connection(_) |
            DatabaseError::RateLimitExceeded { .. } |
            DatabaseError::Concurrency(_) |
            DatabaseError::ExternalService { .. }
        )
    }

    /// Get suggested recovery actions for this error
    pub fn recovery_suggestions(&self) -> Vec<String> {
        match self {
            DatabaseError::Connection(_) => vec![
                "Check your internet connection".to_string(),
                "Restart the game".to_string(),
                "Verify game files".to_string(),
            ],
            DatabaseError::SaveSlot(_) => vec![
                "Try a different save slot".to_string(),
                "Check available disk space".to_string(),
                "Backup your saves".to_string(),
            ],
            DatabaseError::RateLimitExceeded { retry_after_seconds, .. } => vec![
                format!("Wait {} seconds before retrying", retry_after_seconds),
                "Reduce frequency of actions".to_string(),
            ],
            DatabaseError::AIWorkflow { .. } => vec![
                "Retry the operation".to_string(),
                "Check internet connection for AI services".to_string(),
                "Continue playing - content will be generated later".to_string(),
            ],
            DatabaseError::AssetManagement { .. } => vec![
                "Restart the game to reload assets".to_string(),
                "Check available disk space".to_string(),
                "Verify game files".to_string(),
            ],
            _ => vec![
                "Retry the operation".to_string(),
                "Restart the game if the problem persists".to_string(),
            ],
        }
    }
}

/// Extended error context for debugging and logging
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorContext {
    pub error: DatabaseError,
    pub operation: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub player_id: Option<uuid::Uuid>,
    pub session_id: Option<String>,
    pub additional_context: serde_json::Value,
}

impl ErrorContext {
    pub fn new(error: DatabaseError, operation: String) -> Self {
        Self {
            error,
            operation,
            timestamp: chrono::Utc::now(),
            player_id: None,
            session_id: None,
            additional_context: serde_json::Value::Object(serde_json::Map::new()),
        }
    }

    pub fn with_player(mut self, player_id: uuid::Uuid) -> Self {
        self.player_id = Some(player_id);
        self
    }

    pub fn with_session(mut self, session_id: String) -> Self {
        self.session_id = Some(session_id);
        self
    }

    pub fn with_context(mut self, key: &str, value: serde_json::Value) -> Self {
        if let serde_json::Value::Object(ref mut map) = self.additional_context {
            map.insert(key.to_string(), value);
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_severity_classification() {
        assert_eq!(DatabaseError::player_not_found(uuid::Uuid::new_v4()).severity(), ErrorSeverity::Low);
        assert_eq!(DatabaseError::invalid_dread_level(5).severity(), ErrorSeverity::Medium);
        assert_eq!(DatabaseError::Connection("test".to_string()).severity(), ErrorSeverity::Critical);
    }

    #[test]
    fn test_user_friendly_messages() {
        let error = DatabaseError::player_not_found(uuid::Uuid::new_v4());
        assert_eq!(error.user_message(), "Save file not found. Please check your save slot.");

        let error = DatabaseError::companion_betrayed("Einar");
        assert_eq!(error.user_message(), "This companion is no longer available due to story events.");
    }

    #[test]
    fn test_retryable_errors() {
        assert!(DatabaseError::Connection("test".to_string()).is_retryable());
        assert!(!DatabaseError::invalid_dread_level(5).is_retryable());
    }

    #[test]
    fn test_error_context_builder() {
        let error = DatabaseError::player_not_found(uuid::Uuid::new_v4());
        let player_id = uuid::Uuid::new_v4();
        
        let context = ErrorContext::new(error, "get_player".to_string())
            .with_player(player_id)
            .with_session("session_123".to_string())
            .with_context("dread_level", serde_json::Value::Number(serde_json::Number::from(2)));
        
        assert_eq!(context.player_id, Some(player_id));
        assert_eq!(context.session_id, Some("session_123".to_string()));
        assert_eq!(context.operation, "get_player");
    }

    #[test]
    fn test_recovery_suggestions() {
        let error = DatabaseError::Connection("test".to_string());
        let suggestions = error.recovery_suggestions();
        assert!(!suggestions.is_empty());
        assert!(suggestions.iter().any(|s| s.contains("internet connection")));
    }
}
