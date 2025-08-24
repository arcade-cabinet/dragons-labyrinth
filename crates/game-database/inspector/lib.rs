//! Dragon's Labyrinth Assets Inspector
//! 
//! Revolutionary three-layer validation bridge between Python AI generation
//! and Rust game runtime. Provides visual validation, ECS mapping, and
//! human-in-the-loop quality assurance for all AI-generated content.

use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;
use pyo3::prelude::*;
use rusqlite::{Connection, Result as SqliteResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

pub mod inspector_ui;
pub mod ecs_mapping;
pub mod validation;
pub mod export;

use python_bridge::*;
use inspector_ui::*;
use ecs_mapping::*;
use validation::*;
use export::*;

/// Core resource for managing AI-generated assets database
#[derive(Resource, Reflect, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct GeneratedAssetsDatabase {
    /// Direct connection to Python AI generation SQLite database
    #[reflect(ignore)]
    pub connection: Arc<Mutex<Connection>>,
    
    /// Asset metadata indexed by dread level and category
    pub asset_index: HashMap<(u8, String), Vec<AssetMetadata>>,
    
    /// Validation status for each generated asset
    pub validation_status: HashMap<String, ValidationStatus>,
    
    /// Assets awaiting human approval
    pub approval_queue: Vec<String>,
    
    /// Performance metrics for validated assets
    pub performance_metrics: HashMap<String, PerformanceMetrics>,
}

/// Metadata for each AI-generated asset
#[derive(Clone, Debug, Serialize, Deserialize, Reflect, InspectorOptions)]
pub struct AssetMetadata {
    pub id: String,
    pub source: AssetSource,
    pub dread_level: u8,
    pub category: String,
    pub generation_agent: String,
    pub file_path: PathBuf,
    #[reflect(ignore)]
    pub bevy_handle: Option<Handle<Scene>>,
    pub validation_status: ValidationStatus,
    pub human_approved: bool,
    pub performance_score: f32,
    pub generation_timestamp: String,
}

/// Source of the asset
#[derive(Clone, Debug, Serialize, Deserialize, Reflect, InspectorOptions)]
pub enum AssetSource {
    Core,      // Sacred assets, never modified
    Library,   // CC0 library assets
    Generated, // AI-generated content
    Hybrid,    // Library asset with AI enhancements
}

/// Validation status for assets
#[derive(Clone, Debug, Serialize, Deserialize, Reflect, InspectorOptions)]
pub enum ValidationStatus {
    Generated,      // Python AI agent completed generation
    Loaded,         // Successfully loaded into Bevy inspector
    Validated,      // Passed all validation checks
    Approved,       // Human approved for runtime use
    Rejected,       // Human rejected, needs regeneration
    Error(String),  // Validation failed with error details
}

/// Performance metrics for asset validation
#[derive(Clone, Debug, Serialize, Deserialize, Reflect, InspectorOptions)]
pub struct PerformanceMetrics {
    pub vertex_count: u32,
    pub texture_size: (u32, u32),
    pub memory_usage_mb: f32,
    pub load_time_ms: f32,
    pub fps_impact: f32,
    pub mobile_compatible: bool,
}

/// Inspector UI state
#[derive(Resource, Default, Reflect, InspectorOptions)]
#[reflect(Resource)]
pub struct InspectorUiState {
    pub selected_asset: Option<String>,
    pub current_dread_preview: u8,
    pub show_performance_overlay: bool,
    pub approval_mode: bool,
    pub filter_category: Option<String>,
    pub filter_agent: Option<String>,
}

/// Main plugin for the assets inspector
pub struct AssetsInspectorPlugin {
    pub database_path: PathBuf,
    pub standalone_mode: bool,
}

impl Default for AssetsInspectorPlugin {
    fn default() -> Self {
        Self {
            database_path: PathBuf::from("assets/generated/dragon_assets.db"),
            standalone_mode: false,
        }
    }
}

impl Plugin for AssetsInspectorPlugin {
    fn build(&self, app: &mut App) {
        // Initialize database connection
        let connection = Connection::open(&self.database_path)
            .expect("Failed to open assets database");
        
        let database = GeneratedAssetsDatabase {
            connection: Arc::new(Mutex::new(connection)),
            asset_index: HashMap::new(),
            validation_status: HashMap::new(),
            approval_queue: Vec::new(),
            performance_metrics: HashMap::new(),
        };
        
        app
            // Resources
            .insert_resource(database)
            .init_resource::<InspectorUiState>()
            .register_type::<GeneratedAssetsDatabase>()
            .register_type::<AssetMetadata>()
            .register_type::<InspectorUiState>()
            
            // Inspector UI plugin
            .add_plugins(bevy_inspector_egui::DefaultInspectorConfigPlugin)
            
            // Systems
            .add_systems(Startup, (
                initialize_database,
                load_existing_assets,
                setup_inspector_camera,
            ))
            .add_systems(Update, (
                // Database sync
                sync_database_to_ecs,
                
                // Asset loading and validation
                load_new_assets,
                validate_loaded_assets,
                calculate_performance_metrics,
                
                // Human review
                human_review_system,
                preview_dread_variants,
                
                // UI
                inspector_ui_system,
                
                // Export
                export_approved_assets,
            ).chain())
            .add_systems(FixedUpdate, (
                // Performance monitoring
                monitor_fps_impact,
                check_memory_usage,
            ));
            
        // Initialize Python module if not in standalone mode
        if !self.standalone_mode {
            initialize_python_module();
        }
    }
}

/// Initialize the database schema
fn initialize_database(
    mut database: ResMut<GeneratedAssetsDatabase>,
) {
    let conn = database.connection.lock().unwrap();
    
    // Create tables for asset tracking
    conn.execute(
        "CREATE TABLE IF NOT EXISTS generated_assets (
            id TEXT PRIMARY KEY,
            source TEXT NOT NULL,
            dread_level INTEGER NOT NULL,
            category TEXT NOT NULL,
            generation_agent TEXT NOT NULL,
            file_path TEXT NOT NULL,
            validation_status TEXT NOT NULL,
            human_approved BOOLEAN DEFAULT FALSE,
            performance_score REAL DEFAULT 0.0,
            generation_timestamp TEXT NOT NULL,
            metadata JSON
        )",
        [],
    ).expect("Failed to create assets table");
    
    // Create validation history table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS validation_history (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            asset_id TEXT NOT NULL,
            validation_timestamp TEXT NOT NULL,
            validation_result TEXT NOT NULL,
            error_details TEXT,
            performance_metrics JSON,
            FOREIGN KEY(asset_id) REFERENCES generated_assets(id)
        )",
        [],
    ).expect("Failed to create validation history table");
    
    info!("Assets database initialized");
}

/// Load existing assets from database
fn load_existing_assets(
    mut database: ResMut<GeneratedAssetsDatabase>,
) {
    let conn = database.connection.lock().unwrap();
    
    let mut stmt = conn.prepare(
        "SELECT id, source, dread_level, category, generation_agent, 
                file_path, validation_status, human_approved, 
                performance_score, generation_timestamp
         FROM generated_assets
         ORDER BY generation_timestamp DESC"
    ).expect("Failed to prepare query");
    
    let asset_iter = stmt.query_map([], |row| {
        Ok(AssetMetadata {
            id: row.get(0)?,
            source: match row.get::<_, String>(1)?.as_str() {
                "Core" => AssetSource::Core,
                "Library" => AssetSource::Library,
                "Generated" => AssetSource::Generated,
                "Hybrid" => AssetSource::Hybrid,
                _ => AssetSource::Generated,
            },
            dread_level: row.get(2)?,
            category: row.get(3)?,
            generation_agent: row.get(4)?,
            file_path: PathBuf::from(row.get::<_, String>(5)?),
            bevy_handle: None,
            validation_status: match row.get::<_, String>(6)?.as_str() {
                "Generated" => ValidationStatus::Generated,
                "Loaded" => ValidationStatus::Loaded,
                "Validated" => ValidationStatus::Validated,
                "Approved" => ValidationStatus::Approved,
                "Rejected" => ValidationStatus::Rejected,
                status if status.starts_with("Error:") => {
                    ValidationStatus::Error(status[6..].to_string())
                }
                _ => ValidationStatus::Generated,
            },
            human_approved: row.get(7)?,
            performance_score: row.get(8)?,
            generation_timestamp: row.get(9)?,
        })
    }).expect("Failed to query assets");
    
    for asset in asset_iter {
        if let Ok(asset) = asset {
            // Index by dread level and category
            let key = (asset.dread_level, asset.category.clone());
            database.asset_index
                .entry(key)
                .or_insert_with(Vec::new)
                .push(asset.clone());
            
            // Track validation status
            database.validation_status.insert(
                asset.id.clone(),
                asset.validation_status.clone(),
            );
            
            // Add to approval queue if needed
            if matches!(asset.validation_status, ValidationStatus::Validated) 
                && !asset.human_approved {
                database.approval_queue.push(asset.id.clone());
            }
        }
    }
    
    info!(
        "Loaded {} existing assets from database", 
        database.validation_status.len()
    );
}

/// Setup camera for asset inspection
fn setup_inspector_camera(
    mut commands: Commands,
) {
    // Spawn camera for 3D asset preview
    commands.spawn((
        Camera3d::default(),
        Camera {
            order: 0,
            clear_color: ClearColorConfig::Custom(Color::srgb(0.1, 0.1, 0.15)),
            ..default()
        },
        Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        Name::new("Inspector Camera"),
    ));
    
    // Add lighting for asset preview
    commands.spawn((
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                illuminance: 10000.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_rotation(Quat::from_euler(
                EulerRot::XYZ,
                -std::f32::consts::PI / 4.0,
                std::f32::consts::PI / 4.0,
                0.0,
            )),
            ..default()
        },
        Name::new("Inspector Light"),
    ));
    
    // Ambient light
    commands.insert_resource(AmbientLight {
        color: Color::srgb(0.8, 0.8, 0.8),
        brightness: 500.0,
    });
}

/// Initialize Python module for external access
fn initialize_python_module() {
    Python::with_gil(|py| {
        if let Err(e) = dragon_assets_inspector_module(py) {
            error!("Failed to initialize Python module: {}", e);
        } else {
            info!("Python module 'dragon_assets_inspector' initialized");
        }
    });
}

/// Python module definition
#[pymodule]
fn dragon_assets_inspector_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(notify_asset_generated, m)?)?;
    m.add_function(wrap_pyfunction!(get_validation_status, m)?)?;
    m.add_function(wrap_pyfunction!(mark_asset_approved, m)?)?;
    m.add_function(wrap_pyfunction!(get_performance_metrics, m)?)?;
    m.add_function(wrap_pyfunction!(request_regeneration, m)?)?;
    Ok(())
}

// Re-export Python functions
pub use python_bridge::{
    notify_asset_generated,
    get_validation_status,
    mark_asset_approved,
    get_performance_metrics,
    request_regeneration,
};
