//! Blender Bridge - Idempotent 3D Model Conversion
//! 
//! This crate provides robust 3D model conversion using Blender, with proper
//! OBJ+MTL correlation, comprehensive error handling, and idempotent operations.

pub mod conversion;
pub mod error;
pub mod hashing;
pub mod manifest;
pub mod batch;
pub mod project;
pub mod ron_models;
pub mod simple_gltf;
pub mod template_processor;

// Re-export main types for convenience
pub use error::BlenderBridgeError;
pub use conversion::{
    is_supported_format, validate_obj_dependencies,
    ModelStats, ConversionResult, BatchProcessor,
    convert_file_to_glb
};
pub use ron_models::{generate_model_from_ron, ron_model_exists};
pub use hashing::{obj_family_hash, file_content_hash, parse_mtl_for_textures};
pub use manifest::{ConversionManifest, ManifestEntry};

// BlenderBatchProcessor will be exported after it's defined

use std::path::Path;
use serde::{Deserialize, Serialize};

/// A conversion job specifying source file and desired output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionJob {
    /// Path to source file (OBJ, FBX, GLTF, etc.)
    pub src: String,
    /// Desired output filename (can include subdirectories)
    pub dst_filename: String,
    /// Scale factor (currently unused but maintained for API compatibility)
    #[serde(default = "default_scale")]
    pub scale: f32,
}

fn default_scale() -> f32 {
    1.0
}

/// Result of a single conversion operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobResult {
    pub success: bool,
    pub input: String,
    pub output_file: Option<String>,
    pub skipped: bool,
    pub error: Option<String>,
}

/// Summary of batch conversion operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionSummary {
    pub success: bool,
    pub converted: usize,
    pub total: usize,
    pub results: Vec<JobResult>,
}

/// Main entry point: Convert multiple model files to GLB with idempotent tracking
///
/// This function:
/// 1. Validates all OBJ files have corresponding MTL files
/// 2. Computes content hashes for change detection
/// 3. Skips conversions when files haven't changed
/// 4. Performs Blender conversions for new/changed files
/// 5. Updates manifest to track conversions
///
/// # Arguments
/// * `files` - List of conversion jobs
/// * `output_dir` - Directory for GLB outputs
/// * `manifest_path` - Path to conversion tracking manifest
///
/// # Returns
/// Summary of conversion results including success/failure counts
pub fn convert_model_files_to_glb(
    files: &[ConversionJob],
    output_dir: &str,
    manifest_path: &str,
) -> ConversionSummary {
    let output_path = Path::new(output_dir);
    let manifest_file = Path::new(manifest_path);
    
    // Load existing manifest
    let mut manifest = match ConversionManifest::load_from_file(manifest_file) {
        Ok(m) => m,
        Err(e) => {
            tracing::warn!("Failed to load manifest, starting fresh: {}", e);
            ConversionManifest::default()
        }
    };
    
    // Clean up stale entries
    manifest.cleanup_stale_entries();
    
    let mut results = Vec::new();
    let mut converted_count = 0;
    
    // Ensure output directory exists
    if let Err(e) = std::fs::create_dir_all(output_path) {
        return ConversionSummary {
            success: false,
            converted: 0,
            total: files.len(),
            results: vec![JobResult {
                success: false,
                input: String::new(),
                output_file: None,
                skipped: false,
                error: Some(format!("Failed to create output directory: {}", e)),
            }],
        };
    }
    
    // Process each conversion job
    for job in files {
        let result = process_conversion_job(job, output_path, &mut manifest);
        
        if result.success && !result.skipped {
            converted_count += 1;
        }
        
        results.push(result);
    }
    
    // Save updated manifest
    if let Err(e) = manifest.save_to_file(manifest_file) {
        tracing::error!("Failed to save manifest: {}", e);
    }
    
    ConversionSummary {
        success: true,
        converted: converted_count,
        total: files.len(),
        results,
    }
}

/// Process a single conversion job with proper error handling
fn process_conversion_job(
    job: &ConversionJob,
    output_dir: &Path,
    manifest: &mut ConversionManifest,
) -> JobResult {
    let src_path = Path::new(&job.src);
    
    // Validate source file
    if !src_path.exists() {
        return JobResult {
            success: false,
            input: job.src.clone(),
            output_file: None,
            skipped: false,
            error: Some(format!("Source file not found: {}", job.src)),
        };
    }
    
    // Check if format is supported
    if !is_supported_format(src_path) {
        return JobResult {
            success: false,
            input: job.src.clone(),
            output_file: None,
            skipped: false,
            error: Some(format!("Unsupported file format: {}", job.src)),
        };
    }
    
    // For OBJ files, validate MTL and texture dependencies
    if let Some(ext) = src_path.extension().and_then(|e| e.to_str()) {
        if ext.to_ascii_lowercase() == "obj" {
            if let Err(e) = validate_obj_dependencies(src_path) {
                return JobResult {
                    success: false,
                    input: job.src.clone(),
                    output_file: None,
                    skipped: false,
                    error: Some(format!("OBJ dependency validation failed: {}", e)),
                };
            }
        }
    }
    
    // Compute content hash
    let current_hash = match compute_file_hash(src_path) {
        Ok(hash) => hash,
        Err(e) => {
            return JobResult {
                success: false,
                input: job.src.clone(),
                output_file: None,
                skipped: false,
                error: Some(format!("Failed to compute hash: {}", e)),
            };
        }
    };
    
    // Build destination path
    let mut dst_path = output_dir.to_path_buf();
    for component in Path::new(&job.dst_filename).components() {
        dst_path.push(component);
    }
    
    // Ensure destination ends with .glb
    if dst_path.extension().and_then(|e| e.to_str()) != Some("glb") {
        dst_path.set_extension("glb");
    }
    
    // Check if conversion is needed
    let src_key = src_path.to_string_lossy().to_string();
    if !manifest.needs_conversion(&src_key, &current_hash) {
        return JobResult {
            success: true,
            input: job.src.clone(),
            output_file: manifest.get_destination(&src_key).map(|s| s.to_string()),
            skipped: true,
            error: None,
        };
    }
    
    // Perform conversion
    match convert_file_to_glb(src_path, &dst_path) {
        Ok(()) => {
            // Record successful conversion
            manifest.record_conversion(
                src_key,
                current_hash,
                dst_path.to_string_lossy().to_string(),
            );
            
            JobResult {
                success: true,
                input: job.src.clone(),
                output_file: Some(dst_path.to_string_lossy().to_string()),
                skipped: false,
                error: None,
            }
        }
        Err(e) => {
            JobResult {
                success: false,
                input: job.src.clone(),
                output_file: None,
                skipped: false,
                error: Some(e.to_string()),
            }
        }
    }
}

/// Compute appropriate content hash for a file based on its type
fn compute_file_hash(path: &Path) -> Result<String, BlenderBridgeError> {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    
    match ext.as_str() {
        "obj" => obj_family_hash(path),
        _ => file_content_hash(path),
    }
}

/// Enhanced batch conversion with manifest tracking
///
/// This combines the BatchProcessor with manifest-based idempotency
pub fn convert_batch_with_manifest(
    jobs: &[ConversionJob],
    output_dir: &Path,
    manifest_path: &Path,
) -> Result<Vec<ConversionResult>, BlenderBridgeError> {
    let mut manifest = match ConversionManifest::load_from_file(manifest_path) {
        Ok(m) => m,
        Err(_) => ConversionManifest::default(),
    };
    
    manifest.cleanup_stale_entries();
    
    let mut results = Vec::new();
    std::fs::create_dir_all(output_dir)?;
    
    for job in jobs {
        let src_path = Path::new(&job.src);
        let dst_path = output_dir.join(&job.dst_filename);
        
        // Check if conversion is needed (idempotency)
        let current_hash = compute_file_hash(src_path)?;
        let src_key = src_path.to_string_lossy().to_string();
        
        if !manifest.needs_conversion(&src_key, &current_hash) {
            results.push(ConversionResult {
                success: true,
                input_file: job.src.clone(),
                output_file: manifest.get_destination(&src_key)
                    .unwrap_or(&job.dst_filename).to_string(),
                stats: None,
                error: None,
                skipped: true,
            });
            continue;
        }
        
        // Perform enhanced conversion with statistics
        let result = conversion::convert_file_to_glb_enhanced(
            src_path, 
            &dst_path, 
            job.scale
        )?;
        
        // Update manifest if conversion was successful
        if result.success {
            manifest.record_conversion(
                src_key,
                current_hash,
                dst_path.to_string_lossy().to_string(),
            );
        }
        
        results.push(result);
    }
    
    // Save updated manifest
    manifest.save_to_file(manifest_path)?;
    
    Ok(results)
}
