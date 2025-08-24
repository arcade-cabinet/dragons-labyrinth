//! Validation module for checking AI-generated assets
//! Performs technical validation, performance checks, and quality assurance

use bevy::prelude::*;
use crate::{
    AssetMetadata, GeneratedAssetsDatabase, ValidationStatus, 
    PerformanceMetrics, python_bridge, ecs_mapping
};
use gltf::Gltf;
use image::GenericImageView;
use std::fs;
use std::time::Instant;
use tracing::{info, warn, error};

/// Validation criteria for different asset types
#[derive(Debug, Clone)]
pub struct ValidationCriteria {
    pub max_vertices: u32,
    pub max_texture_size: (u32, u32),
    pub max_memory_mb: f32,
    pub max_load_time_ms: f32,
    pub mobile_vertex_limit: u32,
    pub mobile_texture_limit: (u32, u32),
}

impl Default for ValidationCriteria {
    fn default() -> Self {
        Self {
            max_vertices: 100_000,
            max_texture_size: (4096, 4096),
            max_memory_mb: 50.0,
            max_load_time_ms: 1000.0,
            mobile_vertex_limit: 10_000,
            mobile_texture_limit: (2048, 2048),
        }
    }
}

/// System to validate loaded assets
pub fn validate_loaded_assets(
    mut database: ResMut<GeneratedAssetsDatabase>,
    mut query: Query<(&mut AssetMetadata, Entity), With<ecs_mapping::ValidatingAsset>>,
    time: Res<Time>,
) {
    let criteria = ValidationCriteria::default();
    
    for (mut metadata, entity) in query.iter_mut() {
        // Skip if already validated
        if !matches!(metadata.validation_status, ValidationStatus::Loaded) {
            continue;
        }
        
        info!("Validating asset: {}", metadata.id);
        
        // Perform validation based on file type
        let validation_result = match metadata.file_path.extension().and_then(|s| s.to_str()) {
            Some("glb") | Some("gltf") => validate_3d_model(&metadata, &criteria),
            Some("png") | Some("jpg") | Some("jpeg") => validate_texture(&metadata, &criteria),
            Some("ogg") | Some("wav") | Some("mp3") => validate_audio(&metadata, &criteria),
            Some("yarn") => validate_dialogue(&metadata),
            _ => Err(format!("Unknown file type for validation")),
        };
        
        // Update validation status
        match validation_result {
            Ok(metrics) => {
                metadata.validation_status = ValidationStatus::Validated;
                metadata.performance_score = calculate_performance_score(&metrics, &criteria);
                
                // Store metrics
                database.performance_metrics.insert(
                    metadata.id.clone(),
                    metrics.clone(),
                );
                
                // Update Python bridge with metrics
                python_bridge::update_performance_metrics(
                    metadata.id.clone(),
                    python_bridge::PerformanceData {
                        vertex_count: metrics.vertex_count,
                        texture_size: metrics.texture_size,
                        memory_mb: metrics.memory_usage_mb,
                        fps_impact: metrics.fps_impact,
                        mobile_compatible: metrics.mobile_compatible,
                    },
                );
                
                // Add to approval queue if validation passed
                if metadata.performance_score > 0.5 {
                    database.approval_queue.push(metadata.id.clone());
                    info!("Asset {} passed validation with score {:.2}", 
                        metadata.id, metadata.performance_score);
                } else {
                    warn!("Asset {} failed validation with score {:.2}", 
                        metadata.id, metadata.performance_score);
                }
                
                // Update database
                ecs_mapping::update_asset_in_database(
                    &database,
                    &metadata.id,
                    ValidationStatus::Validated,
                    Some(metadata.performance_score),
                );
            }
            Err(error_msg) => {
                metadata.validation_status = ValidationStatus::Error(error_msg.clone());
                error!("Validation failed for {}: {}", metadata.id, error_msg);
                
                // Update database
                ecs_mapping::update_asset_in_database(
                    &database,
                    &metadata.id,
                    ValidationStatus::Error(error_msg),
                    None,
                );
            }
        }
        
        // Update Python bridge with validation status
        let status_str = match &metadata.validation_status {
            ValidationStatus::Validated => "validated",
            ValidationStatus::Error(e) => "error",
            _ => "unknown",
        };
        python_bridge::update_validation_status(metadata.id.clone(), status_str.to_string());
    }
}

/// Validate 3D model assets
fn validate_3d_model(
    metadata: &AssetMetadata,
    criteria: &ValidationCriteria,
) -> Result<PerformanceMetrics, String> {
    let start = Instant::now();
    
    // Load GLTF file
    let file_content = fs::read(&metadata.file_path)
        .map_err(|e| format!("Failed to read model file: {}", e))?;
    
    let gltf = Gltf::from_slice(&file_content)
        .map_err(|e| format!("Failed to parse GLTF: {}", e))?;
    
    // Count vertices and triangles
    let mut total_vertices = 0u32;
    let mut total_triangles = 0u32;
    
    for mesh in gltf.meshes() {
        for primitive in mesh.primitives() {
            if let Some(accessor) = primitive.indices() {
                total_triangles += accessor.count() as u32 / 3;
            }
            
            if let Some(accessor) = primitive.get(&gltf::Semantic::Positions) {
                total_vertices += accessor.count() as u32;
            }
        }
    }
    
    // Check vertex count limits
    if total_vertices > criteria.max_vertices {
        return Err(format!(
            "Model exceeds vertex limit: {} > {}",
            total_vertices, criteria.max_vertices
        ));
    }
    
    // Estimate memory usage
    let memory_mb = estimate_model_memory(total_vertices, total_triangles);
    
    // Check memory limits
    if memory_mb > criteria.max_memory_mb {
        return Err(format!(
            "Model exceeds memory limit: {:.1} MB > {:.1} MB",
            memory_mb, criteria.max_memory_mb
        ));
    }
    
    let load_time_ms = start.elapsed().as_millis() as f32;
    
    // Check mobile compatibility
    let mobile_compatible = total_vertices <= criteria.mobile_vertex_limit;
    
    // Estimate FPS impact (simplified)
    let fps_impact = (total_vertices as f32 / 10000.0).min(10.0);
    
    Ok(PerformanceMetrics {
        vertex_count: total_vertices,
        texture_size: (0, 0), // Will be filled if model has textures
        memory_usage_mb: memory_mb,
        load_time_ms,
        fps_impact,
        mobile_compatible,
    })
}

/// Validate texture assets
fn validate_texture(
    metadata: &AssetMetadata,
    criteria: &ValidationCriteria,
) -> Result<PerformanceMetrics, String> {
    let start = Instant::now();
    
    // Load image
    let img = image::open(&metadata.file_path)
        .map_err(|e| format!("Failed to load texture: {}", e))?;
    
    let (width, height) = img.dimensions();
    
    // Check texture size limits
    if width > criteria.max_texture_size.0 || height > criteria.max_texture_size.1 {
        return Err(format!(
            "Texture exceeds size limit: {}x{} > {}x{}",
            width, height, criteria.max_texture_size.0, criteria.max_texture_size.1
        ));
    }
    
    // Check power-of-two dimensions (important for older GPUs)
    if !is_power_of_two(width) || !is_power_of_two(height) {
        warn!("Texture dimensions are not power-of-two: {}x{}", width, height);
    }
    
    // Estimate memory usage
    let bytes_per_pixel = match img.color() {
        image::ColorType::L8 => 1,
        image::ColorType::La8 => 2,
        image::ColorType::Rgb8 => 3,
        image::ColorType::Rgba8 => 4,
        _ => 4,
    };
    let memory_mb = (width * height * bytes_per_pixel) as f32 / (1024.0 * 1024.0);
    
    let load_time_ms = start.elapsed().as_millis() as f32;
    
    // Check mobile compatibility
    let mobile_compatible = width <= criteria.mobile_texture_limit.0 
        && height <= criteria.mobile_texture_limit.1;
    
    // Estimate FPS impact based on texture size
    let fps_impact = ((width * height) as f32 / (1024.0 * 1024.0)).min(5.0);
    
    Ok(PerformanceMetrics {
        vertex_count: 0,
        texture_size: (width, height),
        memory_usage_mb: memory_mb,
        load_time_ms,
        fps_impact,
        mobile_compatible,
    })
}

/// Validate audio assets
fn validate_audio(
    metadata: &AssetMetadata,
    criteria: &ValidationCriteria,
) -> Result<PerformanceMetrics, String> {
    let start = Instant::now();
    
    // Get file size
    let file_metadata = fs::metadata(&metadata.file_path)
        .map_err(|e| format!("Failed to get audio file metadata: {}", e))?;
    
    let file_size_mb = file_metadata.len() as f32 / (1024.0 * 1024.0);
    
    // Simple validation based on file size
    if file_size_mb > criteria.max_memory_mb {
        return Err(format!(
            "Audio file too large: {:.1} MB > {:.1} MB",
            file_size_mb, criteria.max_memory_mb
        ));
    }
    
    let load_time_ms = start.elapsed().as_millis() as f32;
    
    Ok(PerformanceMetrics {
        vertex_count: 0,
        texture_size: (0, 0),
        memory_usage_mb: file_size_mb,
        load_time_ms,
        fps_impact: 0.1, // Minimal FPS impact for audio
        mobile_compatible: file_size_mb < 5.0, // Mobile limit for audio
    })
}

/// Validate dialogue files
fn validate_dialogue(metadata: &AssetMetadata) -> Result<PerformanceMetrics, String> {
    // Simple validation for YarnSpinner files
    let file_content = fs::read_to_string(&metadata.file_path)
        .map_err(|e| format!("Failed to read dialogue file: {}", e))?;
    
    // Check for basic YarnSpinner structure
    if !file_content.contains("title:") && !file_content.contains("---") {
        return Err("Invalid YarnSpinner dialogue format".to_string());
    }
    
    let file_size_kb = file_content.len() as f32 / 1024.0;
    
    Ok(PerformanceMetrics {
        vertex_count: 0,
        texture_size: (0, 0),
        memory_usage_mb: file_size_kb / 1024.0,
        load_time_ms: 1.0,
        fps_impact: 0.0,
        mobile_compatible: true,
    })
}

/// Calculate performance metrics for validated assets
pub fn calculate_performance_metrics(
    database: Res<GeneratedAssetsDatabase>,
    query: Query<(&AssetMetadata, &Handle<Scene>), With<ecs_mapping::ValidatingAsset>>,
    scenes: Res<Assets<Scene>>,
    meshes: Res<Assets<Mesh>>,
) {
    for (metadata, scene_handle) in query.iter() {
        if let Some(scene) = scenes.get(scene_handle) {
            let mut total_vertices = 0u32;
            let mut total_triangles = 0u32;
            
            // Count vertices in scene
            for entity in scene.world.iter_entities() {
                if let Some(mesh_handle) = entity.get::<Handle<Mesh>>() {
                    if let Some(mesh) = meshes.get(mesh_handle) {
                        if let Some(vertex_count) = mesh.count_vertices() {
                            total_vertices += vertex_count;
                        }
                    }
                }
            }
            
            info!("Asset {} has {} vertices", metadata.id, total_vertices);
        }
    }
}

/// Monitor FPS impact of loaded assets
pub fn monitor_fps_impact(
    diagnostics: Res<DiagnosticsStore>,
    database: Res<GeneratedAssetsDatabase>,
) {
    if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(fps_value) = fps.smoothed() {
            // Track FPS for performance analysis
            if fps_value < 30.0 {
                warn!("Low FPS detected: {:.1}", fps_value);
                
                // Find high-impact assets
                for (asset_id, metrics) in &database.performance_metrics {
                    if metrics.fps_impact > 5.0 {
                        warn!("High FPS impact asset: {} (impact: {:.1})", 
                            asset_id, metrics.fps_impact);
                    }
                }
            }
        }
    }
}

/// Check memory usage of loaded assets
pub fn check_memory_usage(
    database: Res<GeneratedAssetsDatabase>,
) {
    let total_memory: f32 = database.performance_metrics
        .values()
        .map(|m| m.memory_usage_mb)
        .sum();
    
    if total_memory > 200.0 {
        warn!("Total memory usage exceeds target: {:.1} MB > 200 MB", total_memory);
    }
    
    info!("Total asset memory usage: {:.1} MB", total_memory);
}

// Helper functions

fn is_power_of_two(n: u32) -> bool {
    n != 0 && (n & (n - 1)) == 0
}

fn estimate_model_memory(vertices: u32, triangles: u32) -> f32 {
    // Rough estimate: 
    // - Each vertex: position (12) + normal (12) + UV (8) = 32 bytes
    // - Each triangle: 3 indices * 4 bytes = 12 bytes
    let vertex_memory = vertices * 32;
    let index_memory = triangles * 12;
    (vertex_memory + index_memory) as f32 / (1024.0 * 1024.0)
}

fn calculate_performance_score(
    metrics: &PerformanceMetrics,
    criteria: &ValidationCriteria,
) -> f32 {
    let mut score = 1.0;
    
    // Vertex count score (for 3D models)
    if metrics.vertex_count > 0 {
        let vertex_ratio = metrics.vertex_count as f32 / criteria.max_vertices as f32;
        score *= 1.0 - vertex_ratio.min(1.0) * 0.3;
    }
    
    // Texture size score
    if metrics.texture_size.0 > 0 {
        let texture_ratio = (metrics.texture_size.0 * metrics.texture_size.1) as f32
            / (criteria.max_texture_size.0 * criteria.max_texture_size.1) as f32;
        score *= 1.0 - texture_ratio.min(1.0) * 0.2;
    }
    
    // Memory usage score
    let memory_ratio = metrics.memory_usage_mb / criteria.max_memory_mb;
    score *= 1.0 - memory_ratio.min(1.0) * 0.25;
    
    // Load time score
    let load_time_ratio = metrics.load_time_ms / criteria.max_load_time_ms;
    score *= 1.0 - load_time_ratio.min(1.0) * 0.15;
    
    // Mobile compatibility bonus
    if metrics.mobile_compatible {
        score *= 1.1;
    }
    
    // FPS impact penalty
    score *= 1.0 - (metrics.fps_impact / 10.0).min(0.5);
    
    score.clamp(0.0, 1.0)
}
