//! Export module for approved assets
//! Handles exporting validated assets to runtime manifest for game consumption

use bevy::prelude::*;
use crate::{
    AssetMetadata, GeneratedAssetsDatabase, ValidationStatus,
    PerformanceMetrics, ecs_mapping
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use tracing::{info, warn, error};

/// Runtime asset manifest for game consumption
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RuntimeAssetManifest {
    pub version: String,
    pub generation_timestamp: String,
    pub total_assets: usize,
    pub dread_levels: HashMap<u8, DreadLevelAssets>,
    pub asset_entries: Vec<RuntimeAssetEntry>,
    pub performance_summary: PerformanceSummary,
}

/// Assets organized by dread level
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DreadLevelAssets {
    pub level: u8,
    pub name: String,
    pub hex_tiles: Vec<String>,
    pub companions: Vec<String>,
    pub ui_elements: Vec<String>,
    pub audio_assets: Vec<String>,
    pub dialogue_trees: Vec<String>,
}

/// Individual asset entry in manifest
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RuntimeAssetEntry {
    pub id: String,
    pub category: String,
    pub dread_level: u8,
    pub file_path: String,
    pub source_type: String,
    pub generation_agent: String,
    pub performance_score: f32,
    pub mobile_compatible: bool,
    pub metadata: HashMap<String, String>,
}

/// Performance summary for runtime optimization
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PerformanceSummary {
    pub total_memory_mb: f32,
    pub total_vertices: u32,
    pub mobile_compatible_percentage: f32,
    pub average_performance_score: f32,
    pub high_impact_assets: Vec<String>,
}

/// System to export approved assets
pub fn export_approved_assets(
    database: Res<GeneratedAssetsDatabase>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    // Export when E key is pressed (manual trigger)
    if !keyboard.just_pressed(KeyCode::KeyE) {
        return;
    }
    
    info!("Starting asset export process...");
    
    // Collect all approved assets
    let approved_assets: Vec<AssetMetadata> = database.asset_index
        .values()
        .flatten()
        .filter(|asset| matches!(asset.validation_status, ValidationStatus::Approved))
        .cloned()
        .collect();
    
    if approved_assets.is_empty() {
        warn!("No approved assets to export");
        return;
    }
    
    info!("Exporting {} approved assets", approved_assets.len());
    
    // Create runtime manifest
    let manifest = create_runtime_manifest(&approved_assets, &database.performance_metrics);
    
    // Export manifest to JSON
    let manifest_path = PathBuf::from("assets/generated/manifest.json");
    if let Err(e) = export_manifest_to_file(&manifest, &manifest_path) {
        error!("Failed to export manifest: {}", e);
        return;
    }
    
    info!("Manifest exported to: {}", manifest_path.display());
    
    // Copy approved assets to runtime directories
    if let Err(e) = organize_assets_for_runtime(&approved_assets) {
        error!("Failed to organize assets: {}", e);
        return;
    }
    
    // Generate asset loading code for Rust runtime
    if let Err(e) = generate_rust_asset_loader(&manifest) {
        error!("Failed to generate Rust loader: {}", e);
        return;
    }
    
    info!("Asset export completed successfully!");
}

/// Create runtime manifest from approved assets
fn create_runtime_manifest(
    assets: &[AssetMetadata],
    performance_metrics: &HashMap<String, PerformanceMetrics>,
) -> RuntimeAssetManifest {
    let mut dread_levels = HashMap::new();
    let mut total_memory = 0.0f32;
    let mut total_vertices = 0u32;
    let mut mobile_compatible_count = 0;
    let mut performance_scores = Vec::new();
    let mut high_impact_assets = Vec::new();
    
    // Initialize dread level structures
    for level in 0..=4 {
        dread_levels.insert(level, DreadLevelAssets {
            level,
            name: match level {
                0 => "Peace".to_string(),
                1 => "Unease".to_string(),
                2 => "Dread".to_string(),
                3 => "Terror".to_string(),
                4 => "Horror".to_string(),
                _ => "Unknown".to_string(),
            },
            hex_tiles: Vec::new(),
            companions: Vec::new(),
            ui_elements: Vec::new(),
            audio_assets: Vec::new(),
            dialogue_trees: Vec::new(),
        });
    }
    
    // Process each asset
    let mut asset_entries = Vec::new();
    for asset in assets {
        // Create runtime entry
        let entry = RuntimeAssetEntry {
            id: asset.id.clone(),
            category: asset.category.clone(),
            dread_level: asset.dread_level,
            file_path: asset.file_path.to_str().unwrap_or("").to_string(),
            source_type: format!("{:?}", asset.source),
            generation_agent: asset.generation_agent.clone(),
            performance_score: asset.performance_score,
            mobile_compatible: performance_metrics
                .get(&asset.id)
                .map(|m| m.mobile_compatible)
                .unwrap_or(false),
            metadata: HashMap::new(),
        };
        
        // Categorize by dread level
        if let Some(dread_assets) = dread_levels.get_mut(&asset.dread_level) {
            match asset.category.as_str() {
                "hex_tiles" => dread_assets.hex_tiles.push(asset.id.clone()),
                "companions" => dread_assets.companions.push(asset.id.clone()),
                "ui" => dread_assets.ui_elements.push(asset.id.clone()),
                "audio" => dread_assets.audio_assets.push(asset.id.clone()),
                "dialogue" => dread_assets.dialogue_trees.push(asset.id.clone()),
                _ => {}
            }
        }
        
        // Update performance statistics
        if let Some(metrics) = performance_metrics.get(&asset.id) {
            total_memory += metrics.memory_usage_mb;
            total_vertices += metrics.vertex_count;
            if metrics.mobile_compatible {
                mobile_compatible_count += 1;
            }
            if metrics.fps_impact > 5.0 {
                high_impact_assets.push(asset.id.clone());
            }
        }
        
        performance_scores.push(asset.performance_score);
        asset_entries.push(entry);
    }
    
    // Calculate performance summary
    let performance_summary = PerformanceSummary {
        total_memory_mb: total_memory,
        total_vertices,
        mobile_compatible_percentage: (mobile_compatible_count as f32 / assets.len() as f32) * 100.0,
        average_performance_score: performance_scores.iter().sum::<f32>() / performance_scores.len() as f32,
        high_impact_assets,
    };
    
    RuntimeAssetManifest {
        version: "1.0.0".to_string(),
        generation_timestamp: chrono::Utc::now().to_rfc3339(),
        total_assets: assets.len(),
        dread_levels,
        asset_entries,
        performance_summary,
    }
}

/// Export manifest to JSON file
fn export_manifest_to_file(
    manifest: &RuntimeAssetManifest,
    path: &Path,
) -> Result<(), String> {
    // Ensure directory exists
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create manifest directory: {}", e))?;
    }
    
    // Serialize to JSON
    let json = serde_json::to_string_pretty(manifest)
        .map_err(|e| format!("Failed to serialize manifest: {}", e))?;
    
    // Write to file
    fs::write(path, json)
        .map_err(|e| format!("Failed to write manifest file: {}", e))?;
    
    Ok(())
}

/// Organize assets into runtime directory structure
fn organize_assets_for_runtime(assets: &[AssetMetadata]) -> Result<(), String> {
    let runtime_dir = PathBuf::from("assets/runtime");
    
    // Create runtime directories
    for subdir in &["models", "textures", "audio", "dialogue", "ui"] {
        let dir_path = runtime_dir.join(subdir);
        fs::create_dir_all(&dir_path)
            .map_err(|e| format!("Failed to create runtime directory: {}", e))?;
    }
    
    // Copy assets to appropriate directories
    for asset in assets {
        let target_dir = match asset.category.as_str() {
            "hex_tiles" | "companions" => "models",
            "ui" => "ui",
            "audio" => "audio",
            "dialogue" => "dialogue",
            _ => "textures",
        };
        
        let target_path = runtime_dir.join(target_dir).join(
            asset.file_path.file_name().unwrap_or_default()
        );
        
        // Copy file if source exists
        if asset.file_path.exists() {
            fs::copy(&asset.file_path, &target_path)
                .map_err(|e| format!("Failed to copy asset {}: {}", asset.id, e))?;
            
            info!("Copied {} to runtime/{}", asset.id, target_dir);
        }
    }
    
    Ok(())
}

/// Generate Rust code for loading assets at runtime
fn generate_rust_asset_loader(manifest: &RuntimeAssetManifest) -> Result<(), String> {
    let mut code = String::new();
    
    // Header
    code.push_str("// Auto-generated asset loader for Dragon's Labyrinth\n");
    code.push_str("// Generated by assets-inspector\n\n");
    code.push_str("use bevy::prelude::*;\n");
    code.push_str("use std::collections::HashMap;\n\n");
    
    // Asset loader function
    code.push_str("pub fn load_runtime_assets(\n");
    code.push_str("    mut commands: Commands,\n");
    code.push_str("    asset_server: Res<AssetServer>,\n");
    code.push_str(") {\n");
    code.push_str("    let mut asset_handles = HashMap::new();\n\n");
    
    // Generate loading code for each asset
    for entry in &manifest.asset_entries {
        let load_code = match entry.category.as_str() {
            "hex_tiles" | "companions" => {
                format!("    asset_handles.insert(\n        \"{}\".to_string(),\n        asset_server.load::<Scene>(\"runtime/models/{}\"),\n    );\n",
                    entry.id,
                    Path::new(&entry.file_path).file_name().unwrap_or_default().to_str().unwrap_or("")
                )
            }
            "ui" => {
                format!("    asset_handles.insert(\n        \"{}\".to_string(),\n        asset_server.load::<Image>(\"runtime/ui/{}\"),\n    );\n",
                    entry.id,
                    Path::new(&entry.file_path).file_name().unwrap_or_default().to_str().unwrap_or("")
                )
            }
            "audio" => {
                format!("    asset_handles.insert(\n        \"{}\".to_string(),\n        asset_server.load::<AudioSource>(\"runtime/audio/{}\"),\n    );\n",
                    entry.id,
                    Path::new(&entry.file_path).file_name().unwrap_or_default().to_str().unwrap_or("")
                )
            }
            _ => String::new(),
        };
        code.push_str(&load_code);
    }
    
    code.push_str("\n    // Store asset handles as resource\n");
    code.push_str("    commands.insert_resource(RuntimeAssets { handles: asset_handles });\n");
    code.push_str("}\n\n");
    
    // RuntimeAssets resource struct
    code.push_str("#[derive(Resource)]\n");
    code.push_str("pub struct RuntimeAssets {\n");
    code.push_str("    pub handles: HashMap<String, Handle<dyn Asset>>,\n");
    code.push_str("}\n");
    
    // Write to file
    let loader_path = PathBuf::from("crates/game/src/runtime_assets.rs");
    fs::write(&loader_path, code)
        .map_err(|e| format!("Failed to write asset loader: {}", e))?;
    
    info!("Generated Rust asset loader at: {}", loader_path.display());
    
    Ok(())
}

/// Export assets for specific dread level
pub fn export_dread_level_assets(
    database: &GeneratedAssetsDatabase,
    dread_level: u8,
) -> Vec<RuntimeAssetEntry> {
    let mut entries = Vec::new();
    
    // Get all assets for this dread level
    for category in &["hex_tiles", "companions", "ui", "audio", "dialogue"] {
        let assets = ecs_mapping::get_assets_by_criteria(
            database,
            category,
            dread_level,
            true, // Only approved
        );
        
        for asset in assets {
            entries.push(RuntimeAssetEntry {
                id: asset.id.clone(),
                category: asset.category.clone(),
                dread_level: asset.dread_level,
                file_path: asset.file_path.to_str().unwrap_or("").to_string(),
                source_type: format!("{:?}", asset.source),
                generation_agent: asset.generation_agent.clone(),
                performance_score: asset.performance_score,
                mobile_compatible: database.performance_metrics
                    .get(&asset.id)
                    .map(|m| m.mobile_compatible)
                    .unwrap_or(false),
                metadata: HashMap::new(),
            });
        }
    }
    
    entries
}

/// Create asset bundle for efficient loading
pub fn create_asset_bundle(
    assets: &[RuntimeAssetEntry],
    bundle_name: &str,
) -> Result<PathBuf, String> {
    let bundle_dir = PathBuf::from("assets/bundles");
    fs::create_dir_all(&bundle_dir)
        .map_err(|e| format!("Failed to create bundle directory: {}", e))?;
    
    let bundle_path = bundle_dir.join(format!("{}.bundle.json", bundle_name));
    
    let bundle_data = serde_json::to_string_pretty(assets)
        .map_err(|e| format!("Failed to serialize bundle: {}", e))?;
    
    fs::write(&bundle_path, bundle_data)
        .map_err(|e| format!("Failed to write bundle: {}", e))?;
    
    info!("Created asset bundle: {}", bundle_path.display());
    
    Ok(bundle_path)
}
