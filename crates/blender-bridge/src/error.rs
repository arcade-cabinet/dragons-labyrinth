//! Error types for blender-bridge operations

use std::path::PathBuf;
use thiserror::Error;

/// Comprehensive error type for blender-bridge operations
#[derive(Error, Debug)]
pub enum BlenderBridgeError {
    #[error("Source file not found: {0}")]
    SourceNotFound(PathBuf),
    
    #[error("MTL file missing for OBJ: {obj_file:?}, expected: {expected_mtl:?}")]
    MissingMtlFile {
        obj_file: PathBuf,
        expected_mtl: PathBuf,
    },
    
    #[error("Missing texture files referenced in MTL {mtl_file:?}: {missing_files:?}")]
    MissingTextures {
        mtl_file: PathBuf,
        missing_files: Vec<PathBuf>,
    },
    
    #[error("Unsupported file format for {file:?}: {error}")]
    UnsupportedFormat {
        file: PathBuf,
        error: String,
    },
    
    #[error("Invalid file path (non-UTF8): {0:?}")]
    InvalidPath(PathBuf),
    
    #[error("Import failed for {file:?}: {error}")]
    ImportFailed {
        file: PathBuf,
        error: String,
    },
    
    #[error("Export failed for {file:?}: {error}")]
    ExportFailed {
        file: PathBuf,
        error: String,
    },
    
    #[error("Blender operation failed: {0}")]
    BlenderOperationFailed(String),
    
    #[error("Manifest operation failed: {0}")]
    ManifestError(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("BPY script execution failed: {0}")]
    ScriptExecutionFailed(String),
    
    #[error("BLR operation failed: {0}")]
    BlrError(String),
    
    #[error("Batch operation failed: {message}")]
    BatchFailed {
        message: String,
    },
}
