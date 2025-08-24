//! ECS mapping module for synchronizing SQLite database with Bevy ECS
//! Maps AI-generated assets from database to runtime components

use bevy::prelude::*;
use crate::{
    AssetMetadata, AssetSource, GeneratedAssetsDatabase, 
    ValidationStatus, python_bridge
};
use dragons_core::components::{HexPosition, HexTile};
use hexx::Hex;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Component marking an asset being validated
#[derive(Component, Reflect)]
pub struct ValidatingAsset {
    pub asset_id: String,
    pub validation_started: f64,
}

/// Component marking an asset approved for runtime
#[derive(Component, Reflect)]
pub struct ValidationApproved;

/// Component for preview assets in inspector
#[derive(Component, Reflect)]
pub struct PreviewAsset {
    pub asset_id: String,
    pub dread_variant: u8,
}

/// Generated hex tile data from MapsAgent
#[derive(Component, Reflect, Serialize, Deserialize)]
pub struct GeneratedHexTile {
    pub id: String,
    pub terrain_type: String,
    pub dread_level: u8,
    pub corruption_intensity: f32,
    pub model_path: String,
    pub texture_paths: Vec<String>,
    pub performance_score: f32,
    pub agent_generated: String,
}

/// Generated companion state from DialogueAgent
#[derive(Component, Reflect, Serialize, Deserialize)]
pub struct GeneratedCompanionState {
    pub companion_name: String,
    pub trauma_level: f32,
    pub dread_stage: u8,
    pub model_path: String,
    pub animation_paths: Vec<String>,
    pub dialogue_tree_path: String,
    pub agent_generated: String,
}

/// Generated UI element from UIAgent
#[derive(Component, Reflect, Serialize, Deserialize)]
pub struct GeneratedUIElement {
    pub element_id: String,
    pub ui_type: String,
    pub dread_level: u8,
    pub corruption_state: String,
    pub texture_path: String,
    pub animation_data: Option<String>,
    pub agent_generated: String,
}

/// Generated audio asset from AudioAgent
#[derive(Component, Reflect, Serialize, Deserialize)]
pub struct GeneratedAudioAsset {
    pub audio_id: String,
    pub audio_type: String,
    pub dread_level: u8,
    pub spatial_config: SpatialAudioConfig,
    pub file_path: String,
    pub agent_generated: String,
}

/// Spatial audio configuration
#[derive(Reflect, Serialize, Deserialize)]
pub struct SpatialAudioConfig {
    pub proximity_radius: f32,
    pub intensity_curve: String,
    pub reverb_level: f32,
    pub distortion_level: f32,
}

/// System to sync database changes to ECS
pub fn sync_database_to_ecs(
    mut commands: Commands,
    mut database: ResMut<GeneratedAssetsDatabase>,
    asset_server: Res<AssetServer>,
    existing_assets: Query<(Entity, &AssetMetadata)>,
) {
    // Check for new notifications from Python agents
    let notifications = python_bridge::get_pending_notifications();
    
    for notification in notifications {
        info!("Processing asset notification: {}", notification.asset_id);
        
        // Create asset metadata
        let metadata = AssetMetadata {
            id: notification.asset_id.clone(),
            source: AssetSource::Generated,
            dread_level: notification.metadata
                .get("dread_level")
                .and_then(|v| v.parse().ok())
                .unwrap_or(0),
            category: notification.metadata
                .get("category")
                .cloned()
                .unwrap_or_else(|| "unknown".to_string()),
            generation_agent: notification.agent_name,
            file_path: PathBuf::from(notification.file_path),
            bevy_handle: None,
            validation_status: ValidationStatus::Generated,
            human_approved: false,
            performance_score: 0.0,
            generation_timestamp: notification.timestamp,
        };
        
        // Insert into database
        if let Ok(conn) = database.connection.lock() {
            let _ = conn.execute(
                "INSERT OR REPLACE INTO generated_assets 
                 (id, source, dread_level, category, generation_agent, file_path, 
                  validation_status, human_approved, performance_score, generation_timestamp)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
                params![
                    &metadata.id,
                    "Generated",
                    metadata.dread_level,
                    &metadata.category,
                    &metadata.generation_agent,
                    metadata.file_path.to_str().unwrap_or(""),
                    "Generated",
                    false,
                    0.0,
                    &metadata.generation_timestamp,
                ],
            );
        }
        
        // Add to index
        let key = (metadata.dread_level, metadata.category.clone());
        database.asset_index
            .entry(key)
            .or_insert_with(Vec::new)
            .push(metadata.clone());
        
        // Update validation status
        database.validation_status.insert(
            metadata.id.clone(),
            ValidationStatus::Generated,
        );
        
        // Spawn ECS entity for validation
        spawn_asset_entity(&mut commands, &asset_server, &metadata);
        
        // Update Python bridge with initial status
        python_bridge::update_validation_status(
            notification.asset_id,
            "loaded".to_string(),
        );
    }
    
    // Check for status updates in existing assets
    for (entity, asset_meta) in existing_assets.iter() {
        if let Some(status) = database.validation_status.get(&asset_meta.id) {
            // Update entity based on validation status
            match status {
                ValidationStatus::Approved => {
                    commands.entity(entity).insert(ValidationApproved);
                }
                ValidationStatus::Rejected => {
                    commands.entity(entity).despawn();
                }
                _ => {}
            }
        }
    }
}

/// Spawn an entity for asset validation
fn spawn_asset_entity(
    commands: &mut Commands,
    asset_server: &AssetServer,
    metadata: &AssetMetadata,
) {
    let mut entity = commands.spawn((
        metadata.clone(),
        ValidatingAsset {
            asset_id: metadata.id.clone(),
            validation_started: 0.0, // Will be set by validation system
        },
        Name::new(format!("Asset: {}", metadata.id)),
    ));
    
    // Add category-specific components
    match metadata.category.as_str() {
        "hex_tiles" => {
            entity.insert(GeneratedHexTile {
                id: metadata.id.clone(),
                terrain_type: "generated".to_string(),
                dread_level: metadata.dread_level,
                corruption_intensity: metadata.dread_level as f32 / 4.0,
                model_path: metadata.file_path.to_str().unwrap_or("").to_string(),
                texture_paths: vec![],
                performance_score: 0.0,
                agent_generated: metadata.generation_agent.clone(),
            });
            
            // Load 3D model if it's a GLTF file
            if metadata.file_path.extension().and_then(|s| s.to_str()) == Some("glb") {
                let scene_handle: Handle<Scene> = asset_server.load(metadata.file_path.clone());
                entity.insert(SceneRoot(scene_handle.clone()));
            }
        }
        "companions" => {
            entity.insert(GeneratedCompanionState {
                companion_name: metadata.id.clone(),
                trauma_level: 0.0,
                dread_stage: metadata.dread_level,
                model_path: metadata.file_path.to_str().unwrap_or("").to_string(),
                animation_paths: vec![],
                dialogue_tree_path: String::new(),
                agent_generated: metadata.generation_agent.clone(),
            });
        }
        "ui" => {
            entity.insert(GeneratedUIElement {
                element_id: metadata.id.clone(),
                ui_type: "generated".to_string(),
                dread_level: metadata.dread_level,
                corruption_state: format!("dread_{}", metadata.dread_level),
                texture_path: metadata.file_path.to_str().unwrap_or("").to_string(),
                animation_data: None,
                agent_generated: metadata.generation_agent.clone(),
            });
        }
        "audio" => {
            entity.insert(GeneratedAudioAsset {
                audio_id: metadata.id.clone(),
                audio_type: "generated".to_string(),
                dread_level: metadata.dread_level,
                spatial_config: SpatialAudioConfig {
                    proximity_radius: 10.0,
                    intensity_curve: "linear".to_string(),
                    reverb_level: metadata.dread_level as f32 * 0.2,
                    distortion_level: metadata.dread_level as f32 * 0.15,
                },
                file_path: metadata.file_path.to_str().unwrap_or("").to_string(),
                agent_generated: metadata.generation_agent.clone(),
            });
        }
        _ => {}
    }
}

/// Load new assets that have been validated
pub fn load_new_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    database: Res<GeneratedAssetsDatabase>,
    mut query: Query<(Entity, &mut AssetMetadata), With<ValidatingAsset>>,
) {
    for (entity, mut metadata) in query.iter_mut() {
        // Skip if already has a handle
        if metadata.bevy_handle.is_some() {
            continue;
        }
        
        // Load based on file type
        let path = &metadata.file_path;
        if path.exists() {
            match path.extension().and_then(|s| s.to_str()) {
                Some("glb") | Some("gltf") => {
                    let handle: Handle<Scene> = asset_server.load(path.clone());
                    metadata.bevy_handle = Some(handle.clone());
                    commands.entity(entity).insert(SceneRoot(handle));
                    info!("Loaded 3D model: {}", metadata.id);
                }
                Some("png") | Some("jpg") | Some("jpeg") => {
                    let handle: Handle<Image> = asset_server.load(path.clone());
                    commands.entity(entity).insert(handle);
                    info!("Loaded texture: {}", metadata.id);
                }
                Some("ogg") | Some("wav") | Some("mp3") => {
                    let handle: Handle<AudioSource> = asset_server.load(path.clone());
                    commands.entity(entity).insert(handle);
                    info!("Loaded audio: {}", metadata.id);
                }
                Some("yarn") => {
                    // YarnSpinner dialogue files handled separately
                    info!("Dialogue file detected: {}", metadata.id);
                }
                _ => {
                    warn!("Unknown file type for asset: {}", metadata.id);
                }
            }
            
            // Update validation status
            metadata.validation_status = ValidationStatus::Loaded;
        } else {
            error!("Asset file not found: {}", path.display());
            metadata.validation_status = ValidationStatus::Error(
                format!("File not found: {}", path.display())
            );
        }
    }
}

/// Map hex tile assets to game world
pub fn map_hex_tiles_to_world(
    mut commands: Commands,
    hex_tiles: Query<(&GeneratedHexTile, &Handle<Scene>), With<ValidationApproved>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (tile_data, scene_handle) in hex_tiles.iter() {
        // Parse hex position from tile ID if encoded
        let hex = if let Some(pos_str) = tile_data.id.split('_').nth(1) {
            // Try to parse "q,r" format
            let parts: Vec<&str> = pos_str.split(',').collect();
            if parts.len() == 2 {
                if let (Ok(q), Ok(r)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                    Hex::new(q, r)
                } else {
                    Hex::ZERO
                }
            } else {
                Hex::ZERO
            }
        } else {
            Hex::ZERO
        };
        
        // Create hex tile entity for game world
        commands.spawn((
            HexPosition(hex),
            HexTile {
                hex,
                tile_type: dragons_core::components::TileType::Grass, // Map from string
                dread_level: tile_data.dread_level,
                corruption: tile_data.corruption_intensity,
                elevation: 0.0,
                passable: true,
            },
            SceneRoot(scene_handle.clone()),
            Transform::from_xyz(hex.x() as f32, 0.0, hex.y() as f32),
            Name::new(format!("HexTile_{}", tile_data.id)),
        ));
    }
}

/// Map companion assets to game entities
pub fn map_companion_assets(
    mut commands: Commands,
    companions: Query<(&GeneratedCompanionState, &Handle<Scene>), With<ValidationApproved>>,
) {
    for (companion_data, scene_handle) in companions.iter() {
        // Create companion entity for game
        commands.spawn((
            dragons_core::components::Companion {
                name: companion_data.companion_name.clone(),
                companion_type: dragons_core::components::CompanionType::Einar, // Map from name
                sanity: 100.0 - (companion_data.trauma_level * 100.0),
                loyalty: 100.0 - (companion_data.dread_stage as f32 * 20.0),
                trauma_level: companion_data.trauma_level,
            },
            SceneRoot(scene_handle.clone()),
            Transform::from_xyz(0.0, 0.0, 0.0),
            Name::new(format!("Companion_{}", companion_data.companion_name)),
        ));
    }
}

/// Query helper to get assets by category and dread level
pub fn get_assets_by_criteria(
    database: &GeneratedAssetsDatabase,
    category: &str,
    dread_level: u8,
    only_approved: bool,
) -> Vec<AssetMetadata> {
    let key = (dread_level, category.to_string());
    
    if let Some(assets) = database.asset_index.get(&key) {
        if only_approved {
            assets.iter()
                .filter(|a| matches!(a.validation_status, ValidationStatus::Approved))
                .cloned()
                .collect()
        } else {
            assets.clone()
        }
    } else {
        Vec::new()
    }
}

/// Update asset metadata in database
pub fn update_asset_in_database(
    database: &GeneratedAssetsDatabase,
    asset_id: &str,
    status: ValidationStatus,
    performance_score: Option<f32>,
) {
    if let Ok(conn) = database.connection.lock() {
        let status_str = match status {
            ValidationStatus::Generated => "Generated",
            ValidationStatus::Loaded => "Loaded",
            ValidationStatus::Validated => "Validated",
            ValidationStatus::Approved => "Approved",
            ValidationStatus::Rejected => "Rejected",
            ValidationStatus::Error(ref e) => &format!("Error:{}", e),
        };
        
        if let Some(score) = performance_score {
            let _ = conn.execute(
                "UPDATE generated_assets 
                 SET validation_status = ?1, performance_score = ?2 
                 WHERE id = ?3",
                params![status_str, score, asset_id],
            );
        } else {
            let _ = conn.execute(
                "UPDATE generated_assets 
                 SET validation_status = ?1 
                 WHERE id = ?2",
                params![status_str, asset_id],
            );
        }
    }
}
