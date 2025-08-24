//! Pure Rust 3D model conversion using blr crate
//! 
//! Provides equivalent functionality to the Python bpy_processor.py but in pure Rust

use std::path::Path;
use std::time::Instant;
use blr::import::BlendImporter;
use crate::error::BlenderBridgeError;
use serde::{Deserialize, Serialize};

/// Statistics about a converted model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelStats {
    pub vertex_count: u32,
    pub face_count: u32,
    pub file_size: u64,
    pub export_time_ms: u64,
}

/// Result of a conversion operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionResult {
    pub success: bool,
    pub input_file: String,
    pub output_file: String,
    pub stats: Option<ModelStats>,
    pub error: Option<String>,
    pub skipped: bool,
}

/// Convert a single model file to GLB with comprehensive statistics
///
/// This replicates the Python `convert_file_to_glb` functionality using pure Rust + blr
pub fn convert_file_to_glb_enhanced(
    src: &Path, 
    dst: &Path,
    scale: f32
) -> Result<ConversionResult, BlenderBridgeError> {
    let _start_time = Instant::now();
    
    // Validate source file exists
    if !src.exists() {
        return Ok(ConversionResult {
            success: false,
            input_file: src.to_string_lossy().to_string(),
            output_file: dst.to_string_lossy().to_string(),
            stats: None,
            error: Some("Source file not found".to_string()),
            skipped: false,
        });
    }
    
    // For OBJ files, validate MTL dependency
    if let Some(ext) = src.extension().and_then(|e| e.to_str()) {
        if ext.to_ascii_lowercase() == "obj" {
            let mtl_path = src.with_extension("mtl");
            if !mtl_path.exists() {
                return Ok(ConversionResult {
                    success: false,
                    input_file: src.to_string_lossy().to_string(),
                    output_file: dst.to_string_lossy().to_string(),
                    stats: None,
                    error: Some(format!("Missing MTL file: {}", mtl_path.display())),
                    skipped: false,
                });
            }
        }
    }
    
    // Ensure destination directory exists
    if let Some(parent) = dst.parent() {
        std::fs::create_dir_all(parent)?;
    }
    
    // Use the project management system for conversion
    match crate::project::convert_with_project(src, dst, scale) {
        Ok(result_path) => {
            let metadata = std::fs::metadata(&result_path)?;
            let export_time_ms = _start_time.elapsed().as_millis() as u64;
            
            Ok(ConversionResult {
                success: true,
                input_file: src.to_string_lossy().to_string(),
                output_file: result_path.to_string_lossy().to_string(),
                stats: Some(ModelStats {
                    vertex_count: 0,  // Would require querying Blender
                    face_count: 0,
                    file_size: metadata.len(),
                    export_time_ms,
                }),
                error: None,
                skipped: false,
            })
        }
        Err(e) => Ok(ConversionResult {
            success: false,
            input_file: src.to_string_lossy().to_string(),
            output_file: dst.to_string_lossy().to_string(),
            stats: None,
            error: Some(e.to_string()),
            skipped: false,
        })
    }
}

/// Legacy function maintained for backward compatibility
pub fn convert_file_to_glb(src: &Path, dst: &Path) -> Result<(), BlenderBridgeError> {
    let result = convert_file_to_glb_enhanced(src, dst, 1.0)?;
    if result.success {
        Ok(())
    } else {
        Err(BlenderBridgeError::BlrError(
            result.error.unwrap_or_else(|| "Unknown conversion error".to_string())
        ))
    }
}

// NOTE: Direct import->export using blr is not currently supported
// The blr API requires using BlendProject for proper import/export workflows
// For procedural generation, use the bpy_generation module instead



/// Collect file statistics 
fn collect_file_stats(export_path: &Path, export_time_ms: u64) -> Result<ModelStats, BlenderBridgeError> {
    // Get file size
    let file_size = if export_path.exists() {
        export_path.metadata()
            .map(|m| m.len())
            .unwrap_or(0)
    } else {
        0
    };
    
    // Since blr doesn't seem to expose scene statistics directly,
    // we'll return basic stats with zero vertex/face counts for now
    Ok(ModelStats {
        vertex_count: 0, // Would need blr API to get actual counts
        face_count: 0,   // Would need blr API to get actual counts
        file_size,
        export_time_ms,
    })
}

/// Check if a file format is supported for conversion
pub fn is_supported_format(path: &Path) -> bool {
    // Try to create an importer - if it succeeds, it's supported
    BlendImporter::from_filepath_extension(path).is_ok()
}

/// Validate that an OBJ file has its required MTL and texture dependencies
pub fn validate_obj_dependencies(obj_path: &Path) -> Result<Vec<std::path::PathBuf>, BlenderBridgeError> {
    if !obj_path.exists() {
        return Err(BlenderBridgeError::SourceNotFound(obj_path.to_path_buf()));
    }
    
    let mtl_path = obj_path.with_extension("mtl");
    if !mtl_path.exists() {
        return Err(BlenderBridgeError::MissingMtlFile {
            obj_file: obj_path.to_path_buf(),
            expected_mtl: mtl_path,
        });
    }
    
    // Parse MTL file for texture dependencies
    let textures = crate::hashing::parse_mtl_for_textures(&mtl_path)?;
    
    // Verify all referenced textures exist
    let mut missing_textures = Vec::new();
    for tex_path in &textures {
        if !tex_path.exists() {
            missing_textures.push(tex_path.clone());
        }
    }
    
    if !missing_textures.is_empty() {
        return Err(BlenderBridgeError::MissingTextures {
            mtl_file: mtl_path,
            missing_files: missing_textures,
        });
    }
    
    Ok(textures)
}

/// Batch processor for multiple conversion jobs
///
/// This provides similar functionality to Python's BPYBatchManager
pub struct BatchProcessor {
    pub total_converted: usize,
    pub total_failed: usize,
    pub results: Vec<ConversionResult>,
}

impl BatchProcessor {
    pub fn new() -> Self {
        Self {
            total_converted: 0,
            total_failed: 0,
            results: Vec::new(),
        }
    }
    
    /// Process multiple BPY scripts (not fully implemented due to blr limitations)
    pub fn execute_bpy_batch(
        &mut self,
        scripts: &[(String, String)], // (script_content, filename)
        output_dir: &Path,
    ) -> Result<(), BlenderBridgeError> {
        std::fs::create_dir_all(output_dir)?;
        
        for (_script_content, filename) in scripts {
            let output_file = if filename.ends_with(".glb") {
                output_dir.join(filename)
            } else {
                output_dir.join(format!("{}.glb", filename))
            };
            
            // Scripts are now handled via RON models
            let result = ConversionResult {
                success: false,
                input_file: filename.clone(),
                output_file: output_file.to_string_lossy().to_string(),
                stats: None,
                error: Some("BPY scripts are no longer supported. Use RON models instead.".to_string()),
                skipped: true,
            };
            
            if result.success {
                self.total_converted += 1;
            } else {
                self.total_failed += 1;
            }
            
            self.results.push(result);
        }
        
        Ok(())
    }
    
    /// Process multiple file conversions
    pub fn convert_batch(
        &mut self,
        files: &[(std::path::PathBuf, std::path::PathBuf, f32)], // (src, dst, scale)
    ) -> Result<(), BlenderBridgeError> {
        for (src, dst, scale) in files {
            let result = convert_file_to_glb_enhanced(src, dst, *scale)?;
            
            if result.success {
                self.total_converted += 1;
            } else {
                self.total_failed += 1;
            }
            
            self.results.push(result);
        }
        
        Ok(())
    }
    
    /// Get conversion summary
    pub fn summary(&self) -> String {
        format!(
            "Batch complete: {}/{} successful",
            self.total_converted,
            self.total_converted + self.total_failed
        )
    }
}

impl Default for BatchProcessor {
    fn default() -> Self {
        Self::new()
    }
}

/// Simplified conversion function that matches the Python API
/// 
/// This replicates the Python organize_cc0_library.py convert_model_files_to_glb function
pub fn convert_model_files_to_glb(
    files: &[crate::ConversionJob],
    output_dir: &str,
    _scale: f32,
    _manifest_path: &str,
) -> Result<Vec<ConversionResult>, BlenderBridgeError> {
    let output_path = Path::new(output_dir);
    let mut results = Vec::new();
    
    // Ensure output directory exists
    std::fs::create_dir_all(output_path)?;
    
    // Process each conversion job
    for job in files {
        let src_path = Path::new(&job.src);
        let mut dst_path = output_path.join(&job.dst_filename);
        
        // Ensure destination ends with .glb
        if dst_path.extension().and_then(|e| e.to_str()) != Some("glb") {
            dst_path.set_extension("glb");
        }
        
        let result = convert_file_to_glb_enhanced(src_path, &dst_path, job.scale)?;
        results.push(result);
    }
    
    Ok(results)
}