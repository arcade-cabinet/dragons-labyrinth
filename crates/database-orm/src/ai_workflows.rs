//! AI workflow models for build-time content generation orchestration

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// AI workflow tracking for build-time asset and content generation
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "ai_workflows")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    
    // Workflow identification
    #[sea_orm(column_type = "Text")]
    pub workflow_type: String, // "ui_generation", "audio_generation", "dialogue_generation", "asset_creation"
    
    #[sea_orm(column_type = "Text")]
    pub workflow_name: String, // Human-readable name
    
    #[sea_orm(column_type = "Text")]
    pub agent_name: String, // Which AI agent is responsible ("UIAgent", "AudioAgent", etc.)
    
    // Workflow request and parameters
    #[sea_orm(column_type = "Json")]
    pub input_parameters: serde_json::Value, // JSON object with generation parameters
    
    #[sea_orm(column_type = "Json")]
    pub generation_prompts: serde_json::Value, // JSON array of prompts used
    
    pub target_dread_level: i32, // 0-4 dread level this generation targets
    
    #[sea_orm(column_type = "Text", nullable)]
    pub content_category: Option<String>, // "ui", "audio", "dialogue", "environmental", "narrative"
    
    // Execution state
    #[sea_orm(column_type = "Text")]
    pub status: String, // "pending", "in_progress", "completed", "failed", "requires_review"
    
    pub progress_percentage: f32, // 0.0-1.0 completion progress
    
    pub priority_level: i32, // 1-10 priority (10 = highest)
    
    // Execution timestamps
    pub created_at: DateTime<Utc>, // When workflow was requested
    pub started_at: Option<DateTime<Utc>>, // When execution began
    pub completed_at: Option<DateTime<Utc>>, // When execution finished
    pub estimated_completion: Option<DateTime<Utc>>, // ETA for completion
    
    // Resource usage tracking
    pub api_calls_made: i32, // Number of AI API calls
    pub tokens_consumed: i64, // Total tokens used
    pub processing_time_seconds: f32, // CPU/GPU time used
    
    #[sea_orm(column_type = "Text", nullable)]
    pub api_provider: Option<String>, // "OpenAI", "Anthropic", "Local", etc.
    
    #[sea_orm(column_type = "Text", nullable)]
    pub model_used: Option<String>, // Which AI model was used
    
    // Output and results
    #[sea_orm(column_type = "Json", nullable)]
    pub generated_assets: Option<serde_json::Value>, // JSON array of created asset paths/IDs
    
    #[sea_orm(column_type = "Json", nullable)]
    pub output_metadata: Option<serde_json::Value>, // Metadata about generated content
    
    pub output_file_count: i32, // Number of files generated
    pub output_total_size_bytes: i64, // Total size of generated content
    
    // Quality and validation
    pub quality_score: Option<f32>, // 0.0-1.0 automated quality assessment
    pub requires_human_review: bool, // Needs manual validation?
    pub human_approved: Option<bool>, // Has human reviewer approved?
    
    #[sea_orm(column_type = "Text", nullable)]
    pub review_notes: Option<String>, // Notes from human reviewer
    
    // Error handling and debugging
    #[sea_orm(column_type = "Json", nullable)]
    pub error_log: Option<serde_json::Value>, // JSON array of errors encountered
    
    #[sea_orm(column_type = "Json", nullable)]
    pub debug_information: Option<serde_json::Value>, // Debug info for troubleshooting
    
    #[sea_orm(column_type = "Text", nullable)]
    pub failure_reason: Option<String>, // Why workflow failed (if it did)
    
    pub retry_count: i32, // Number of times execution was retried
    pub max_retries: i32, // Maximum allowed retries
    
    // Dependencies and workflow chaining
    #[sea_orm(column_type = "Json", nullable)]
    pub depends_on_workflows: Option<serde_json::Value>, // JSON array of prerequisite workflow IDs
    
    #[sea_orm(column_type = "Json", nullable)]
    pub triggers_workflows: Option<serde_json::Value>, // JSON array of workflows to trigger on completion
    
    pub is_blocking: bool, // Does failure of this workflow block others?
    
    // Caching and optimization
    #[sea_orm(column_type = "Text", nullable)]
    pub cache_key: Option<String>, // Key for caching results
    
    pub cache_hit: bool, // Was result served from cache?
    
    #[sea_orm(column_type = "Text", nullable)]
    pub content_hash: Option<String>, // Hash of generated content for deduplication
    
    // Build system integration
    #[sea_orm(column_type = "Text", nullable)]
    pub build_target: Option<String>, // Which build target this supports ("debug", "release", "wasm")
    
    #[sea_orm(column_type = "Text", nullable)]
    pub output_directory: Option<String>, // Where generated assets are stored
    
    pub integrated_with_build: bool, // Has output been integrated into build system?
    
    // Monitoring and analytics
    pub cpu_usage_peak: Option<f32>, // Peak CPU usage during generation
    pub memory_usage_peak_mb: Option<f32>, // Peak memory usage
    pub network_requests_made: i32, // External API calls
    
    #[sea_orm(column_type = "Json", nullable)]
    pub performance_metrics: Option<serde_json::Value>, // Detailed performance data
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::generated_assets::Entity")]
    GeneratedAssets,
}

impl Related<super::generated_assets::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::GeneratedAssets.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
