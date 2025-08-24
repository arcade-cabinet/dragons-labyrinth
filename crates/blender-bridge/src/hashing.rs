//! Content hashing for idempotent conversion
//! 
//! Provides hashing functions to detect when OBJ+MTL+textures have changed

use std::path::{Path, PathBuf};
use std::fs;
use sha2::{Digest, Sha256};
use crate::error::BlenderBridgeError;

/// Parse an OBJ material file (.mtl) and return a list of texture file paths
///
/// Scans for various texture map directives (map_Kd, map_Ks, etc.) and collects
/// the referenced image files that actually exist on disk.
pub fn parse_mtl_for_textures(mtl_path: &Path) -> Result<Vec<PathBuf>, BlenderBridgeError> {
    // Common texture map keys in MTL files
    const TEXTURE_KEYS: &[&str] = &[
        "map_Kd",    // Diffuse texture map
        "map_Ks",    // Specular texture map  
        "map_Ka",    // Ambient texture map
        "map_Ns",    // Specular highlight texture map
        "map_d",     // Alpha/opacity texture map
        "map_bump",  // Bump map
        "bump",      // Bump map (alternate)
        "norm",      // Normal map
        "map_norm",  // Normal map (alternate)
        "map_Pr",    // Roughness map (PBR)
        "map_Pm",    // Metallic map (PBR)
        "map_Ke",    // Emissive map
        "disp",      // Displacement map
        "map_disp",  // Displacement map (alternate)
    ];
    
    let mut textures = Vec::new();
    let contents = fs::read_to_string(mtl_path)
        .map_err(|e| BlenderBridgeError::Io(e))?;
    
    for line in contents.lines() {
        let trimmed = line.trim();
        
        // Skip comments and empty lines
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        
        for &key in TEXTURE_KEYS {
            if trimmed.starts_with(key) {
                let parts: Vec<&str> = trimmed.split_whitespace().collect();
                if parts.len() >= 2 {
                    // Get the texture filename (last part after the key)
                    if let Some(filename) = parts.last() {
                        let texture_path = mtl_path
                            .parent()
                            .unwrap_or_else(|| Path::new(""))
                            .join(filename);
                        
                        // Only include textures that actually exist
                        if texture_path.exists() {
                            textures.push(texture_path);
                        } else {
                            tracing::warn!(
                                "Texture referenced in MTL not found: {} (from {})", 
                                texture_path.display(),
                                mtl_path.display()
                            );
                        }
                    }
                }
                break; // Found matching key, move to next line
            }
        }
    }
    
    Ok(textures)
}

/// Compute a fast hash for a file based on name, modification time and size
pub fn quick_hash(path: &Path) -> Result<String, BlenderBridgeError> {
    let mut hasher = Sha256::new();
    
    // Hash the filename
    let name = path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or_default();
    hasher.update(name.as_bytes());
    
    // Hash file metadata (size and modification time)
    if let Ok(metadata) = path.metadata() {
        hasher.update(metadata.len().to_string().as_bytes());
        
        if let Ok(modified) = metadata.modified() {
            if let Ok(duration) = modified.duration_since(std::time::SystemTime::UNIX_EPOCH) {
                hasher.update(duration.as_nanos().to_string().as_bytes());
            }
        }
    }
    
    let digest = hasher.finalize();
    Ok(format!("{:x}", &digest)[..16].to_string())
}

/// Compute a comprehensive content hash for an OBJ family (OBJ + MTL + textures)
///
/// This produces a unique digest that includes:
/// - The OBJ file itself
/// - The corresponding MTL file (if present)
/// - All texture files referenced by the MTL
///
/// This enables idempotent conversion - if nothing in the family has changed,
/// we can skip reconversion.
pub fn obj_family_hash(obj_path: &Path) -> Result<String, BlenderBridgeError> {
    let mut hasher = Sha256::new();
    
    // Hash the OBJ file itself
    if obj_path.exists() {
        let obj_hash = quick_hash(obj_path)?;
        hasher.update(obj_hash.as_bytes());
    } else {
        return Err(BlenderBridgeError::SourceNotFound(obj_path.to_path_buf()));
    }
    
    // Hash the MTL file if present
    let mtl_path = obj_path.with_extension("mtl");
    if mtl_path.exists() {
        let mtl_hash = quick_hash(&mtl_path)?;
        hasher.update(mtl_hash.as_bytes());
        
        // Hash all referenced textures
        let textures = parse_mtl_for_textures(&mtl_path)?;
        for texture_path in textures {
            if texture_path.exists() {
                let tex_hash = quick_hash(&texture_path)?;
                hasher.update(tex_hash.as_bytes());
            }
        }
    } else {
        // For OBJ files, MTL is required
        return Err(BlenderBridgeError::MissingMtlFile {
            obj_file: obj_path.to_path_buf(),
            expected_mtl: mtl_path,
        });
    }
    
    let digest = hasher.finalize();
    Ok(format!("{:x}", &digest)[..16].to_string())
}

/// Compute content hash for non-OBJ files (FBX, GLTF, etc.)
pub fn file_content_hash(path: &Path) -> Result<String, BlenderBridgeError> {
    quick_hash(path)
}
