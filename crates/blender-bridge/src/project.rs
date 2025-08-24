//! BlendProject management with proper caching and lifecycle handling
//! 
//! This module manages BlendProject instances, handles Python GIL acquisition,
//! and provides caching of project states for efficiency

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use pyo3::Python;
use blr::{BlendProject, import::BlendImporter, export::GltfExporter};
use crate::error::BlenderBridgeError;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use xdg::BaseDirectories;

/// Get the XDG directories for our application
fn get_xdg_dirs() -> BaseDirectories {
    BaseDirectories::with_prefix("dragons-labyrinth")
}

/// Metadata about a cached project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectMetadata {
    pub id: String,
    pub created_at: u64,
    pub last_accessed: u64,
    pub source_file: Option<String>,
    pub operations_count: usize,
}

/// A managed BlendProject with lifecycle handling
pub struct ManagedProject {
    id: String,
    metadata: ProjectMetadata,
    cache_path: PathBuf,
}

impl ManagedProject {
    /// Load an existing project from cache
    pub fn load(id: &str) -> Result<Self, BlenderBridgeError> {
        let xdg = get_xdg_dirs();
        
        // Try to find the metadata file
        let metadata_path = xdg.find_cache_file(format!("blender-projects/{}.json", id))
            .ok_or_else(|| BlenderBridgeError::BlrError(format!("Project {} not found in cache", id)))?;
        
        // Load metadata
        let metadata_json = std::fs::read_to_string(&metadata_path)?;
        let metadata: ProjectMetadata = serde_json::from_str(&metadata_json)?;
        
        // Find the blend file
        let cache_path = xdg.find_cache_file(format!("blender-projects/{}.blend", id))
            .unwrap_or_else(|| metadata_path.with_extension("blend"));
        
        Ok(Self {
            id: id.to_string(),
            metadata,
            cache_path,
        })
    }
    
    /// Create a new empty project
    pub fn new_empty() -> Result<Self, BlenderBridgeError> {
        let id = format!("project_{}", SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis());
        
        let xdg = get_xdg_dirs();
        let cache_path = xdg.place_cache_file(format!("blender-projects/{}.blend", id))
            .map_err(|e| BlenderBridgeError::BlrError(format!("Failed to create cache file: {}", e)))?;
        
        let metadata = ProjectMetadata {
            id: id.clone(),
            created_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            last_accessed: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            source_file: None,
            operations_count: 0,
        };
        
        Ok(Self {
            id,
            metadata,
            cache_path,
        })
    }
    
    /// Update metadata after an operation
    fn update_metadata(&mut self) {
        self.metadata.last_accessed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        self.metadata.operations_count += 1;
    }
    
    /// Import a model file into this project
    pub fn import_model(&mut self, path: &Path, scale: f32) -> Result<(), BlenderBridgeError> {
        Python::with_gil(|py| {
            // Create or load the project
            let _project = BlendProject::empty(py)
                .map_err(|e| BlenderBridgeError::BlrError(e.to_string()))?;
            
            // Create the appropriate importer
            let mut importer = BlendImporter::from_filepath_extension(path)
                .map_err(|e| BlenderBridgeError::UnsupportedFormat {
                    file: path.to_path_buf(),
                    error: e.to_string(),
                })?;
            
            // Determine if we need post-import scaling
            let needs_post_scale = match &mut importer {
                BlendImporter::Obj(imp) => {
                    imp.global_scale = scale;
                    false
                },
                BlendImporter::Fbx(imp) => {
                    imp.global_scale = scale;
                    false
                },
                BlendImporter::Stl(imp) => {
                    imp.global_scale = scale;
                    false
                },
                BlendImporter::Abc(imp) => {
                    imp.scale = scale;
                    false
                },
                BlendImporter::Usd(imp) => {
                    imp.scale = scale;
                    false
                },
                BlendImporter::Ply(_imp) => {
                    // PlyImporter doesn't have a scale field, we'll handle it post-import
                    tracing::info!("PLY importer doesn't support direct scaling, will apply transform");
                    true
                },
                BlendImporter::Gltf(_) => {
                    // GLTF doesn't support scale in blr
                    tracing::info!("GLTF importer doesn't support direct scaling, will apply transform");
                    true
                },
                _ => {
                    tracing::warn!("Unknown importer type, scale may not be applied");
                    true
                }
            };
            
            // Import the file
            importer.import(path)
                .map_err(|e| BlenderBridgeError::ImportFailed {
                    file: path.to_path_buf(),
                    error: e.to_string(),
                })?;
            
            // If scale wasn't applied directly, apply it as a transform
            if scale != 1.0 && needs_post_scale {
                // Apply scale transform to all objects
                let scale_script = format!(r#"
import bpy
for obj in bpy.context.scene.objects:
    obj.scale = ({}, {}, {})
bpy.ops.object.transform_apply(location=False, rotation=False, scale=True)
"#, scale, scale, scale);
                
                py.run(&scale_script, None, None)
                    .map_err(|e| BlenderBridgeError::BlrError(format!("Failed to apply scale: {}", e)))?;
            }
            
            Ok::<(), BlenderBridgeError>(())
        })?;
        
        // Update metadata
        self.metadata.source_file = Some(path.to_string_lossy().to_string());
        self.update_metadata();
        
        Ok(())
    }
    
    /// Export the project to GLB
    pub fn export_glb(&mut self, output_path: &Path) -> Result<PathBuf, BlenderBridgeError> {
        let result_path = Python::with_gil(|py| {
            let project = BlendProject::empty(py)
                .map_err(|e| BlenderBridgeError::BlrError(e.to_string()))?;
            
            let exporter = GltfExporter::default();
            let result_path = project.export(exporter, output_path)
                .map_err(|e| BlenderBridgeError::ExportFailed {
                    file: output_path.to_path_buf(),
                    error: e.to_string(),
                })?;
            Ok::<PathBuf, BlenderBridgeError>(result_path)
        })?;
        
        self.update_metadata();
        Ok(result_path)
    }
    
    /// Execute a Python script in the project context
    pub fn execute_script(&mut self, script: &str) -> Result<(), BlenderBridgeError> {
        Python::with_gil(|py| {
            py.run(script, None, None)
                .map_err(|e| BlenderBridgeError::ScriptExecutionFailed(e.to_string()))?;
            Ok::<(), BlenderBridgeError>(())
        })?;
        
        self.update_metadata();
        Ok(())
    }
    
    /// Save the project to disk for later reuse
    pub fn save(&self) -> Result<(), BlenderBridgeError> {
        // Save metadata alongside the blend file
        let xdg = get_xdg_dirs();
        let metadata_path = xdg.place_cache_file(format!("blender-projects/{}.json", self.id))
            .map_err(|e| BlenderBridgeError::BlrError(format!("Failed to save metadata: {}", e)))?;
        
        let metadata_json = serde_json::to_string_pretty(&self.metadata)?;
        std::fs::write(metadata_path, metadata_json)?;
        
        // The actual .blend file would be saved by Blender's save operation
        Ok(())
    }
}

/// Project manager that handles multiple projects and their lifecycle
pub struct ProjectManager {
    projects: Arc<Mutex<HashMap<String, ManagedProject>>>,
    max_cached_projects: usize,
}

impl ProjectManager {
    /// Create a new project manager
    pub fn new() -> Self {
        Self {
            projects: Arc::new(Mutex::new(HashMap::new())),
            max_cached_projects: 10,
        }
    }
    
    /// Get or create a project for a specific operation
    pub fn get_or_create_project(&self, id: Option<&str>) -> Result<String, BlenderBridgeError> {
        let mut projects = self.projects.lock().unwrap();
        
        if let Some(id) = id {
            // Check if already in memory
            if projects.contains_key(id) {
                return Ok(id.to_string());
            }
            
            // Try to load from cache
            if let Ok(project) = ManagedProject::load(id) {
                let project_id = project.id.clone();
                projects.insert(project_id.clone(), project);
                return Ok(project_id);
            }
        }
        
        // Create new project
        let project = ManagedProject::new_empty()?;
        let project_id = project.id.clone();
        
        // Check if we need to evict old projects
        if projects.len() >= self.max_cached_projects {
            // Find and remove the least recently used project
            if let Some(lru_id) = projects.iter()
                .min_by_key(|(_, p)| p.metadata.last_accessed)
                .map(|(id, _)| id.clone()) {
                
                // Save before evicting
                if let Some(project) = projects.get(&lru_id) {
                    let _ = project.save(); // Ignore save errors during eviction
                }
                projects.remove(&lru_id);
            }
        }
        
        projects.insert(project_id.clone(), project);
        Ok(project_id)
    }
    
    /// Convert a model file using a managed project
    pub fn convert_model(&self, src: &Path, dst: &Path, scale: f32) -> Result<PathBuf, BlenderBridgeError> {
        let project_id = self.get_or_create_project(None)?;
        let mut projects = self.projects.lock().unwrap();
        
        if let Some(project) = projects.get_mut(&project_id) {
            project.import_model(src, scale)?;
            project.export_glb(dst)
        } else {
            Err(BlenderBridgeError::BlrError("Project not found".to_string()))
        }
    }
    
    /// Execute a BPY script and export to GLB
    pub fn execute_script_to_glb(&self, script: &str, output_path: &Path) -> Result<PathBuf, BlenderBridgeError> {
        let project_id = self.get_or_create_project(None)?;
        let mut projects = self.projects.lock().unwrap();
        
        if let Some(project) = projects.get_mut(&project_id) {
            project.execute_script(script)?;
            project.export_glb(output_path)
        } else {
            Err(BlenderBridgeError::BlrError("Project not found".to_string()))
        }
    }
    
    /// List all cached projects
    pub fn list_cached_projects(&self) -> Vec<String> {
        let xdg = get_xdg_dirs();
        let mut project_ids = Vec::new();
        
        // List all JSON metadata files
        for metadata_file in xdg.list_cache_files("blender-projects") {
            if let Some(filename) = metadata_file.file_name() {
                if let Some(name_str) = filename.to_str() {
                    if name_str.ends_with(".json") {
                        let id = name_str.trim_end_matches(".json");
                        project_ids.push(id.to_string());
                    }
                }
            }
        }
        
        project_ids
    }
    
    /// Clean up old cached projects
    pub fn cleanup_cache(&self) -> Result<usize, BlenderBridgeError> {
        let xdg = get_xdg_dirs();
        let mut removed = 0;
        
        // Remove projects older than 7 days
        let cutoff = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() - (7 * 24 * 60 * 60);
        
        // List all cache files in the blender-projects directory
        for cache_file in xdg.list_cache_files("blender-projects") {
            if let Ok(metadata) = std::fs::metadata(&cache_file) {
                if let Ok(modified) = metadata.modified() {
                    if let Ok(duration) = modified.duration_since(UNIX_EPOCH) {
                        if duration.as_secs() < cutoff {
                            if std::fs::remove_file(&cache_file).is_ok() {
                                removed += 1;
                                tracing::debug!("Removed old cache file: {:?}", cache_file);
                            }
                        }
                    }
                }
            }
        }
        
        Ok(removed)
    }
}

lazy_static::lazy_static! {
    /// Global project manager instance
    static ref GLOBAL_PROJECT_MANAGER: ProjectManager = ProjectManager::new();
}

/// Get the global project manager
pub fn get_project_manager() -> &'static ProjectManager {
    &GLOBAL_PROJECT_MANAGER
}

/// High-level function to convert a model with proper project management
pub fn convert_with_project(src: &Path, dst: &Path, scale: f32) -> Result<PathBuf, BlenderBridgeError> {
    get_project_manager().convert_model(src, dst, scale)
}

/// High-level function to execute a BPY script with proper project management
pub fn execute_script_with_project(script: &str, output_path: &Path) -> Result<PathBuf, BlenderBridgeError> {
    get_project_manager().execute_script_to_glb(script, output_path)
}
