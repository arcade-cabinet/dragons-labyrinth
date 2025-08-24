//! Generated assets models for AI-created content tracking

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// AI-generated assets with comprehensive metadata and lineage tracking
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "generated_assets")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    
    pub ai_workflow_id: Option<Uuid>, // Which AI workflow generated this
    
    // Asset identification
    #[sea_orm(column_type = "Text")]
    pub asset_name: String, // Human-readable name
    
    #[sea_orm(column_type = "Text")]
    pub asset_type: String, // "ui_component", "audio_file", "dialogue_line", "texture", "model"
    
    #[sea_orm(column_type = "Text")]
    pub file_format: String, // "json", "wav", "mp3", "png", "glb", etc.
    
    #[sea_orm(column_type = "Text")]
    pub file_path: String, // Relative path to generated file
    
    #[sea_orm(column_type = "Text")]
    pub content_hash: String, // SHA-256 hash of file content
    
    // Generation metadata
    pub target_dread_level: i32, // 0-4 dread level this asset targets
    
    #[sea_orm(column_type = "Text")]
    pub generation_agent: String, // Which AI agent created this ("UIAgent", "AudioAgent", etc.)
    
    #[sea_orm(column_type = "Json")]
    pub generation_prompts: serde_json::Value, // JSON array of prompts used to generate
    
    #[sea_orm(column_type = "Json")]
    pub generation_parameters: serde_json::Value, // JSON object with generation settings
    
    #[sea_orm(column_type = "Text", nullable)]
    pub api_model_used: Option<String>, // "gpt-4", "claude-3", "stable-diffusion", etc.
    
    // File properties
    pub file_size_bytes: i64, // Size of generated file
    pub creation_timestamp: DateTime<Utc>, // When file was generated
    pub last_modified: DateTime<Utc>, // Last modification time
    
    // Content analysis
    pub quality_score: Option<f32>, // 0.0-1.0 automated quality assessment
    pub horror_intensity: Option<f32>, // 0.0-1.0 how horror-focused the content is
    pub emotional_impact: Option<f32>, // 0.0-1.0 emotional weight of content
    
    #[sea_orm(column_type = "Text", nullable)]
    pub content_description: Option<String>, // AI-generated description of content
    
    #[sea_orm(column_type = "Json", nullable)]
    pub content_tags: Option<serde_json::Value>, // JSON array of content tags
    
    // Asset-specific metadata (varies by type)
    #[sea_orm(column_type = "Json", nullable)]
    pub type_specific_metadata: Option<serde_json::Value>, // Type-specific properties
    
    // For audio assets
    pub audio_duration_seconds: Option<f32>, // Length of audio (if applicable)
    pub audio_sample_rate: Option<i32>, // Sample rate for audio files
    pub audio_bitrate: Option<i32>, // Bitrate for audio files
    
    // For image/texture assets
    pub image_width: Option<i32>, // Width in pixels (if applicable)
    pub image_height: Option<i32>, // Height in pixels (if applicable)
    pub image_color_depth: Option<i32>, // Color depth (if applicable)
    
    // For 3D model assets
    pub model_vertex_count: Option<i32>, // Number of vertices (if applicable)
    pub model_face_count: Option<i32>, // Number of faces (if applicable)
    pub model_material_count: Option<i32>, // Number of materials (if applicable)
    
    // Usage and integration
    pub is_integrated: bool, // Has been integrated into game build
    pub integration_status: String, // "pending", "integrated", "failed"
    pub usage_count: i32, // How many times referenced in game
    
    #[sea_orm(column_type = "Json", nullable)]
    pub referenced_by: Option<serde_json::Value>, // JSON array of systems using this asset
    
    // Versioning and iteration
    pub version_number: i32, // Version of this asset (1, 2, 3...)
    pub is_current_version: bool, // Is this the active version?
    pub parent_asset_id: Option<Uuid>, // Previous version (if this is an iteration)
    
    // Quality assurance
    pub human_reviewed: bool, // Has human reviewer checked this?
    pub human_approved: Option<bool>, // Reviewer approval status
    
    #[sea_orm(column_type = "Text", nullable)]
    pub review_notes: Option<String>, // Human reviewer comments
    
    pub automated_tests_passed: bool, // Did automated validation pass?
    
    #[sea_orm(column_type = "Json", nullable)]
    pub validation_results: Option<serde_json::Value>, // JSON object with test results
    
    // Attribution and licensing
    #[sea_orm(column_type = "Text")]
    pub license_type: String, // "AI_Generated", "CC0", "Proprietary"
    
    #[sea_orm(column_type = "Text", nullable)]
    pub attribution_required: Option<String>, // Attribution text if needed
    
    #[sea_orm(column_type = "Text", nullable)]
    pub source_inspiration: Option<String>, // What this was based on (if any)
    
    // Performance optimization
    pub compression_applied: bool, // Was file compressed/optimized?
    pub original_file_size_bytes: Option<i64>, // Size before optimization
    pub compression_ratio: Option<f32>, // How much compression was achieved
    
    // Build system integration
    #[sea_orm(column_type = "Text", nullable)]
    pub build_target: Option<String>, // "debug", "release", "wasm"
    
    pub included_in_build: bool, // Is included in current build
    pub build_priority: i32, // Load order priority (1-10)
    
    // Asset dependencies
    #[sea_orm(column_type = "Json", nullable)]
    pub depends_on_assets: Option<serde_json::Value>, // JSON array of asset IDs this depends on
    
    #[sea_orm(column_type = "Json", nullable)]
    pub required_by_assets: Option<serde_json::Value>, // JSON array of assets that need this
    
    // Caching and optimization
    #[sea_orm(column_type = "Text", nullable)]
    pub cache_key: Option<String>, // Key for runtime caching
    
    pub preload_recommended: bool, // Should be preloaded for performance
    pub memory_usage_mb: Option<f32>, // Runtime memory usage estimate
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::ai_workflows::Entity",
        from = "Column::AiWorkflowId",
        to = "super::ai_workflows::Column::Id"
    )]
    AiWorkflow,
    
    #[sea_orm(
        belongs_to = "Entity",
        from = "Column::ParentAssetId", 
        to = "Column::Id"
    )]
    ParentAsset,
    
    #[sea_orm(has_many = "super::asset_dependencies::Entity")]
    Dependencies,
}

impl Related<super::ai_workflows::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AiWorkflow.def()
    }
}

impl Related<super::asset_dependencies::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Dependencies.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
