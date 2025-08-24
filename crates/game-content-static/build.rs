//! Build script using blender-bridge to:
//! 1. Convert all OBJ/FBX to GLB
//! 2. Run our Blender Python scripts for procedural assets
//! 3. Clean up old formats after successful conversion

use blender_bridge::{ConversionJob, convert_model_files_to_glb, generate_model_from_ron, ron_model_exists};
use glob::glob;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=blender/");
    println!("cargo:rerun-if-changed=assets/");
    
    // Always convert and clean up
    convert_legacy_models();
    
    // Always run procedural generation
    run_blender_scripts();
}

fn convert_legacy_models() {
    println!("cargo:warning=Converting legacy models to GLB...");
    
    // Find all OBJ and FBX files
    let mut jobs = Vec::new();
    
    for pattern in &["assets/**/*.obj", "assets/**/*.fbx"] {
        let files: Vec<PathBuf> = glob(pattern)
            .expect("Failed to read glob pattern")
            .filter_map(Result::ok)
            .collect();
        
        for file_path in files {
            // For OBJ files, ensure MTL exists
            if file_path.extension().and_then(|e| e.to_str()) == Some("obj") {
                let mtl_path = file_path.with_extension("mtl");
                if !mtl_path.exists() {
                    println!("cargo:warning=Skipping {} - no MTL file found", file_path.display());
                    continue;
                }
                println!("cargo:warning=Found OBJ+MTL pair: {}", file_path.display());
            }
            
            // Build output path (relative to assets/)
            let relative_path = file_path.strip_prefix("assets/").unwrap_or(&file_path);
            let glb_path = PathBuf::from("assets").join(relative_path).with_extension("glb");
            
            // Skip if GLB already exists and is newer than source
            if glb_path.exists() {
                let src_time = std::fs::metadata(&file_path)
                    .and_then(|m| m.modified())
                    .ok();
                let glb_time = std::fs::metadata(&glb_path)
                    .and_then(|m| m.modified())
                    .ok();
                
                // For OBJ, also check MTL modification time
                if file_path.extension().and_then(|e| e.to_str()) == Some("obj") {
                    let mtl_path = file_path.with_extension("mtl");
                    let mtl_time = std::fs::metadata(&mtl_path)
                        .and_then(|m| m.modified())
                        .ok();
                    
                    if let (Some(glb), Some(src), Some(mtl)) = (glb_time, src_time, mtl_time) {
                        if glb > src && glb > mtl {
                            continue; // GLB is newer than both OBJ and MTL
                        }
                    }
                } else if let (Some(glb), Some(src)) = (glb_time, src_time) {
                    if glb > src {
                        continue; // GLB is newer than source
                    }
                }
            }
            
            jobs.push(ConversionJob {
                src: file_path.to_string_lossy().to_string(),
                dst_filename: glb_path.file_name()
                    .unwrap()
                    .to_string_lossy()
                    .to_string(),
                scale: 1.0,
            });
        }
    }
    
    if !jobs.is_empty() {
        println!("cargo:warning=Converting {} models...", jobs.len());
        
        let summary = convert_model_files_to_glb(
            &jobs,
            "assets",
            "assets/conversion_manifest.json"
        );
        
        println!("cargo:warning=Converted {}/{} models", summary.converted, summary.total);
        
        // Clean up successfully converted files (only if CLEANUP_MODELS is set)
        let should_cleanup = std::env::var("CLEANUP_MODELS").is_ok();
        let mut cleaned = 0;
        
        for result in summary.results {
            if result.success && !result.skipped && should_cleanup {
                // Successfully converted - remove source files
                let src_path = PathBuf::from(&result.input);
                
                // For OBJ files, also remove MTL and any referenced textures
                if src_path.extension().and_then(|e| e.to_str()) == Some("obj") {
                    let mtl_path = src_path.with_extension("mtl");
                    if mtl_path.exists() {
                        if let Err(e) = std::fs::remove_file(&mtl_path) {
                            println!("cargo:warning=Failed to remove {}: {}", mtl_path.display(), e);
                        } else {
                            println!("cargo:warning=Removed {}", mtl_path.display());
                        }
                    }
                }
                
                // Remove the source file
                if let Err(e) = std::fs::remove_file(&src_path) {
                    println!("cargo:warning=Failed to remove {}: {}", src_path.display(), e);
                } else {
                    cleaned += 1;
                    println!("cargo:warning=Removed {}", src_path.display());
                }
            } else if !result.success && !result.skipped {
                println!("cargo:warning=Failed to convert {}: {:?}", result.input, result.error);
            }
        }
        
        if cleaned > 0 {
            println!("cargo:warning=Cleaned up {} source files after successful conversion", cleaned);
        }
    }
}

fn run_blender_scripts() {
    println!("cargo:warning=Generating procedural models from RON definitions...");
    
    // Find all RON model definitions
    let ron_models: Vec<PathBuf> = glob("assets/ron/**/*.ron")
        .expect("Failed to read glob pattern")
        .filter_map(Result::ok)
        .collect();
    
    for ron_path in ron_models {
        let model_name = ron_path.file_stem().unwrap().to_str().unwrap();
        let category = ron_path.parent().unwrap().file_name().unwrap().to_str().unwrap();
        
        // Output goes to assets/models/{category}/{model_name}.glb
        let output_dir = format!("assets/models/{}", category);
        std::fs::create_dir_all(&output_dir).ok();
        
        let output_path = PathBuf::from(format!("{}/{}.glb", output_dir, model_name));
        
        // Skip if output already exists and is newer than RON
        if output_path.exists() {
            let ron_time = std::fs::metadata(&ron_path)
                .and_then(|m| m.modified())
                .ok();
            let glb_time = std::fs::metadata(&output_path)
                .and_then(|m| m.modified())
                .ok();
            
            if let (Some(glb), Some(ron)) = (glb_time, ron_time) {
                if glb > ron {
                    println!("cargo:warning=✓ {} is up to date", output_path.display());
                    continue;
                }
            }
        }
        
        println!("cargo:warning=Generating {} from RON...", model_name);
        
        // Generate the model from RON
        match generate_model_from_ron(&ron_path, &output_path) {
            Ok(()) => println!("cargo:warning=✓ Generated {}", output_path.display()),
            Err(e) => println!("cargo:warning=✗ Failed to generate {}: {}", model_name, e),
        }
    }
}