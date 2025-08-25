//! Asset dependency models for complex asset relationship tracking

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Asset dependency relationships for complex asset management and build optimization
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "asset_dependencies")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    
    // Dependency relationship
    pub dependent_asset_id: Uuid, // Asset that depends on another
    pub dependency_asset_id: Uuid, // Asset that is depended upon
    
    // Dependency metadata
    #[sea_orm(column_type = "Text")]
    pub dependency_type: String, // "runtime", "build_time", "optional", "conditional"
    
    #[sea_orm(column_type = "Text")]
    pub relationship_type: String, // "texture_for_model", "audio_for_dialogue", "ui_component_asset"
    
    pub is_critical: bool, // Does dependent asset fail without this?
    pub is_optional: bool, // Can dependent asset work without this?
    
    // Load order and priority
    pub load_order_priority: i32, // 1-10 priority for loading (10 = load first)
    pub initialization_order: i32, // Order for initialization
    
    // Conditional dependencies
    #[sea_orm(column_type = "Json", nullable)]
    pub conditions: Option<serde_json::Value>, // JSON object with conditions for dependency
    
    pub required_dread_level_min: Option<i32>, // Min dread level for this dependency
    pub required_dread_level_max: Option<i32>, // Max dread level for this dependency
    
    #[sea_orm(column_type = "Json", nullable)]
    pub platform_conditions: Option<serde_json::Value>, // Platform-specific requirements
    
    #[sea_orm(column_type = "Json", nullable)]
    pub build_conditions: Option<serde_json::Value>, // Build configuration requirements
    
    // Performance and optimization
    pub preload_together: bool, // Should these assets be loaded together?
    pub cache_together: bool, // Should these assets be cached together?
    pub can_lazy_load: bool, // Can dependency be loaded on-demand?
    
    pub memory_impact_score: f32, // 0.0-1.0 how much memory impact this relationship has
    pub loading_time_impact_ms: Option<f32>, // Additional loading time this dependency adds
    
    // Asset-specific dependency data
    #[sea_orm(column_type = "Json", nullable)]
    pub dependency_metadata: Option<serde_json::Value>, // Type-specific dependency data
    
    // For texture dependencies on models
    #[sea_orm(column_type = "Text", nullable)]
    pub material_slot_name: Option<String>, // Which material slot this texture fills
    
    pub uv_channel: Option<i32>, // Which UV channel is used
    
    // For audio dependencies on dialogues/encounters
    #[sea_orm(column_type = "Text", nullable)]
    pub audio_trigger: Option<String>, // When audio should play
    
    pub audio_volume_modifier: Option<f32>, // Volume adjustment for this context
    
    // For UI component dependencies
    #[sea_orm(column_type = "Text", nullable)]
    pub ui_element_id: Option<String>, // Which UI element uses this asset
    
    #[sea_orm(column_type = "Text", nullable)]
    pub ui_state: Option<String>, // Which UI state requires this asset
    
    // Version compatibility
    #[sea_orm(column_type = "Text", nullable)]
    pub min_dependency_version: Option<String>, // Minimum version of dependency required
    
    #[sea_orm(column_type = "Text", nullable)]
    pub max_dependency_version: Option<String>, // Maximum compatible version
    
    pub version_locked: bool, // Must use exact version match?
    
    // Fallback and error handling
    pub has_fallback: bool, // Is there a fallback asset available?
    pub fallback_asset_id: Option<Uuid>, // Fallback asset to use if dependency missing
    
    #[sea_orm(column_type = "Text", nullable)]
    pub fallback_behavior: Option<String>, // "use_placeholder", "disable_feature", "graceful_degrade"
    
    // Build system integration
    pub affects_build_order: bool, // Does this dependency affect build compilation order?
    pub build_time_critical: bool, // Is this required at build time?
    
    #[sea_orm(column_type = "Json", nullable)]
    pub build_flags: Option<serde_json::Value>, // JSON object with build system flags
    
    // Validation and integrity
    pub dependency_validated: bool, // Has this relationship been validated?
    pub last_validation_check: Option<DateTime<Utc>>, // When dependency was last verified
    
    #[sea_orm(column_type = "Text", nullable)]
    pub validation_status: Option<String>, // "valid", "missing", "version_mismatch", "corrupted"
    
    #[sea_orm(column_type = "Text", nullable)]
    pub validation_error: Option<String>, // Error message if validation failed
    
    // Runtime tracking
    pub times_loaded_together: i32, // How often these assets were loaded together
    pub loading_success_rate: f32, // 0.0-1.0 success rate for loading this dependency
    
    pub average_loading_time_ms: Option<f32>, // Average time to load this dependency
    pub last_loading_time_ms: Option<f32>, // Most recent loading time
    
    // Asset lifecycle tracking
    pub dependency_created_at: DateTime<Utc>, // When dependency relationship was created
    pub first_loaded_together: Option<DateTime<Utc>>, // First time assets were used together
    pub last_used_together: Option<DateTime<Utc>>, // Most recent co-usage
    
    // Change tracking
    pub dependency_strength: f32, // 0.0-1.0 how tightly coupled these assets are
    pub break_risk_score: f32, // 0.0-1.0 risk of breaking if dependency changes
    
    #[sea_orm(column_type = "Text", nullable)]
    pub change_impact_notes: Option<String>, // Notes about impact of changes
    
    // Timestamps
    pub created_at: DateTime<Utc>,
    pub last_modified: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::generated_assets::Entity",
        from = "Column::DependentAssetId",
        to = "super::generated_assets::Column::Id"
    )]
    DependentAsset,
    
    #[sea_orm(
        belongs_to = "super::generated_assets::Entity",
        from = "Column::DependencyAssetId", 
        to = "super::generated_assets::Column::Id"
    )]
    DependencyAsset,
    
    #[sea_orm(
        belongs_to = "super::generated_assets::Entity",
        from = "Column::FallbackAssetId",
        to = "super::generated_assets::Column::Id"
    )]
    FallbackAsset,
}

impl Related<super::generated_assets::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DependentAsset.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
