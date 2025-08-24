//! Template-based Asset Generation for Dragon's Labyrinth
//! 
//! Uses minijinja2 templates to generate Blender Python scripts from TOML requests
//! Supports dual perspective system: 2.5D overworld + 3D FPS dungeons

use anyhow::Result;
use minijinja::{Environment, context};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use toml;

/// Template-based asset processor
pub struct TemplateProcessor {
    template_dir: PathBuf,
    texture_base_path: PathBuf,
}

impl TemplateProcessor {
    /// Create new template processor
    pub fn new<P: AsRef<Path>>(template_dir: P, texture_base: P) -> Result<Self> {
        let template_path = template_dir.as_ref().to_path_buf();
        
        // Verify template directory exists
        if !template_path.exists() {
            return Err(anyhow::anyhow!("Template directory does not exist: {}", template_path.display()));
        }
        
        Ok(Self {
            template_dir: template_path,
            texture_base_path: texture_base.as_ref().to_path_buf(),
        })
    }
    
    /// Load template on-demand to avoid lifetime issues
    fn load_template(&self, template_name: &str) -> Result<String> {
        let template_file = format!("{}.py.j2", template_name);
        let template_path = self.template_dir.join(&template_file);
        
        if template_path.exists() {
            std::fs::read_to_string(&template_path)
                .map_err(|e| anyhow::anyhow!("Failed to read template {}: {}", template_file, e))
        } else {
            Err(anyhow::anyhow!("Template not found: {}", template_file))
        }
    }
    
    /// Generate Blender script from TOML asset request
    pub fn generate_script_from_toml<P: AsRef<Path>>(
        &self,
        toml_path: P,
        output_base: P,
    ) -> Result<Vec<GeneratedScript>> {
        let toml_content = std::fs::read_to_string(toml_path)?;
        let request: AssetRequest = toml::from_str(&toml_content)?;
        
        let output_base = output_base.as_ref();
        let mut scripts = Vec::new();
        
        // Process hex tiles for overworld
        if let Some(ref tiles) = request.tiles {
            for (name, tile_data) in tiles {
                let script = self.generate_hex_tile_script(&name, &tile_data, &request, output_base)?;
                scripts.push(script);
            }
        }
        
        // Process companions for both perspectives
        if let Some(ref companions) = request.companions {
            for (name, companion_data) in companions {
                let script = self.generate_companion_script(&name, &companion_data, &request, output_base)?;
                scripts.push(script);
            }
        }
        
        // Process dungeons for FPS perspective
        if let Some(ref dungeons) = request.dungeons {
            for (name, dungeon_data) in dungeons {
                let script = self.generate_dungeon_script(&name, &dungeon_data, &request, output_base)?;
                scripts.push(script);
            }
        }
        
        // Process weapons for FPS perspective
        if let Some(ref weapons) = request.weapons {
            for (name, weapon_data) in weapons {
                let script = self.generate_weapon_script(&name, &weapon_data, &request, output_base)?;
                scripts.push(script);
            }
        }
        
        Ok(scripts)
    }
    
    /// Generate hex tile script using appropriate template
    fn generate_hex_tile_script(
        &self,
        name: &str,
        tile_data: &HexTileData,
        request: &AssetRequest,
        output_base: &Path,
    ) -> Result<GeneratedScript> {
        // Choose template based on perspective
        let template_name = if tile_data.perspective.as_deref() == Some("fps") {
            "hex_tile"  // 3D detail for FPS exploration
        } else {
            "overworld_tiles"  // 2.5D optimized for top-down
        };
        
        let template_content = self.load_template(template_name)?;
        let mut env = Environment::new();
        env.add_template(template_name, &template_content)?;
        let template = env.get_template(template_name)?;
        
        let output_path = output_base.join("tiles").join(format!("{}.glb", name));
        
        let script_content = template.render(context! {
            name => name,
            description => tile_data.description,
            base_geometry => tile_data.base_geometry,
            primary_texture => tile_data.primary_texture,
            detail_textures => tile_data.detail_textures.clone().unwrap_or_default(),
            height_variation => tile_data.height_variation,
            corruption_level => tile_data.corruption_level.unwrap_or(0.0),
            corruption_variants => tile_data.corruption_variants.clone().unwrap_or_default(),
            texture_base_path => self.texture_base_path.display().to_string(),
            output_path => output_path.display().to_string(),
            generation => request.generation,
        })?;
        
        Ok(GeneratedScript {
            name: name.to_string(),
            asset_type: "hex_tile".to_string(),
            perspective: tile_data.perspective.clone().unwrap_or("overworld".to_string()),
            script_content,
            output_path,
        })
    }
    
    /// Generate companion script
    fn generate_companion_script(
        &self,
        name: &str,
        companion_data: &CompanionData,
        request: &AssetRequest,
        output_base: &Path,
    ) -> Result<GeneratedScript> {
        let template_content = self.load_template("companion")?;
        let mut env = Environment::new();
        env.add_template("companion", &template_content)?;
        let template = env.get_template("companion")?;
        
        let output_path = output_base.join("companions").join(format!("{}.glb", name));
        
        let script_content = template.render(context! {
            name => name,
            description => companion_data.description,
            base_geometry => companion_data.base_geometry,
            primary_texture => companion_data.primary_texture,
            clothing_textures => companion_data.clothing_textures.clone().unwrap_or_default(),
            emotion_overlays => companion_data.emotion_overlays.clone().unwrap_or_default(),
            equipment_textures => companion_data.equipment_textures.clone().unwrap_or_default(),
            trauma_level => companion_data.trauma_level,
            shader_effects => companion_data.shader_effects.clone().unwrap_or_default(),
            texture_base_path => self.texture_base_path.display().to_string(),
            output_path => output_path.display().to_string(),
            generation => request.generation,
        })?;
        
        Ok(GeneratedScript {
            name: name.to_string(),
            asset_type: "companion".to_string(),
            perspective: "fps".to_string(),  // Companions need FPS detail
            script_content,
            output_path,
        })
    }
    
    /// Generate dungeon script for FPS rooms
    fn generate_dungeon_script(
        &self,
        name: &str,
        dungeon_data: &DungeonData,
        request: &AssetRequest,
        output_base: &Path,
    ) -> Result<GeneratedScript> {
        let template_content = self.load_template("fps_dungeon_room")?;
        let mut env = Environment::new();
        env.add_template("fps_dungeon_room", &template_content)?;
        let template = env.get_template("fps_dungeon_room")?;
        
        let output_path = output_base.join("dungeons").join(format!("{}.glb", name));
        
        let script_content = template.render(context! {
            name => name,
            description => dungeon_data.description,
            base_geometry => dungeon_data.base_geometry,
            floor_texture => dungeon_data.floor_texture,
            wall_texture => dungeon_data.wall_texture,
            ceiling_texture => dungeon_data.ceiling_texture,
            platform_texture => dungeon_data.platform_texture,
            lighting_style => dungeon_data.lighting_style,
            ambient_effect => dungeon_data.ambient_effect,
            texture_base_path => self.texture_base_path.display().to_string(),
            output_path => output_path.display().to_string(),
            generation => request.generation,
        })?;
        
        Ok(GeneratedScript {
            name: name.to_string(),
            asset_type: "dungeon".to_string(),
            perspective: "fps".to_string(),
            script_content,
            output_path,
        })
    }
    
    /// Generate weapon script
    fn generate_weapon_script(
        &self,
        name: &str,
        weapon_data: &WeaponData,
        request: &AssetRequest,
        output_base: &Path,
    ) -> Result<GeneratedScript> {
        // Weapons need FPS detail for first-person usage - reuse companion template for now
        let template_content = self.load_template("companion")?;
        let mut env = Environment::new();
        env.add_template("companion", &template_content)?;
        let template = env.get_template("companion")?;
        
        let output_path = output_base.join("weapons").join(format!("{}.glb", name));
        
        let script_content = template.render(context! {
            name => name,
            description => weapon_data.description,
            base_geometry => weapon_data.base_geometry,
            primary_texture => weapon_data.primary_texture.clone().unwrap_or_default(),
            texture_base_path => self.texture_base_path.display().to_string(),
            output_path => output_path.display().to_string(),
            generation => request.generation,
        })?;
        
        Ok(GeneratedScript {
            name: name.to_string(),
            asset_type: "weapon".to_string(),
            perspective: "fps".to_string(),
            script_content,
            output_path,
        })
    }
}

/// Generated Blender script with metadata
#[derive(Debug, Clone)]
pub struct GeneratedScript {
    pub name: String,
    pub asset_type: String,
    pub perspective: String,  // "overworld", "fps"
    pub script_content: String,
    pub output_path: PathBuf,
}

/// Simplified TOML structures for template processing
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AssetRequest {
    pub metadata: RequestMetadata,
    pub tiles: Option<HashMap<String, HexTileData>>,
    pub companions: Option<HashMap<String, CompanionData>>,
    pub dungeons: Option<HashMap<String, DungeonData>>,
    pub weapons: Option<HashMap<String, WeaponData>>,
    pub generation: GenerationParams,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RequestMetadata {
    pub category: String,
    pub output_format: String,
    pub target_scale: f32,
    pub texture_base_path: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HexTileData {
    pub description: String,
    pub base_geometry: String,
    pub primary_texture: String,
    pub detail_textures: Option<Vec<String>>,
    pub height_variation: Option<f32>,
    pub corruption_level: Option<f32>,
    pub corruption_variants: Option<Vec<CorruptionVariant>>,
    pub perspective: Option<String>,  // "overworld" or "fps"
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CompanionData {
    pub description: String,
    pub base_geometry: String,
    pub primary_texture: String,
    pub clothing_textures: Option<Vec<String>>,
    pub emotion_overlays: Option<Vec<String>>,
    pub equipment_textures: Option<Vec<String>>,
    pub trauma_level: Option<i32>,
    pub shader_effects: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DungeonData {
    pub description: String,
    pub base_geometry: String,
    pub floor_texture: Option<String>,
    pub wall_texture: Option<String>,
    pub ceiling_texture: Option<String>,
    pub platform_texture: Option<String>,
    pub lighting_style: Option<String>,
    pub ambient_effect: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WeaponData {
    pub description: String,
    pub base_geometry: String,
    pub primary_texture: Option<String>,
    pub blade_texture: Option<String>,
    pub handle_texture: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorruptionVariant {
    pub level: i32,
    pub texture: Option<String>,
    pub overlay: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GenerationParams {
    pub subdivision_level: Option<u32>,
    pub edge_smoothing: Option<bool>,
    pub optimize_for_mobile: Option<bool>,
    pub include_collision_mesh: Option<bool>,
    pub include_animations: Option<bool>,
}

/// Execute generated scripts with Blender
pub fn execute_blender_scripts(scripts: &[GeneratedScript]) -> Result<Vec<ExecutionResult>> {
    let mut results = Vec::new();
    
    for script in scripts {
        println!("Executing Blender script for {} ({})", script.name, script.perspective);
        
        // Write script to temporary file
        let temp_script = std::env::temp_dir().join(format!("{}_script.py", script.name));
        std::fs::write(&temp_script, &script.script_content)?;
        
        // Execute with Blender
        let output = std::process::Command::new("blender")
            .args(&["--background", "--python", temp_script.to_str().unwrap()])
            .output()?;
        
        let success = output.status.success();
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        
        results.push(ExecutionResult {
            script_name: script.name.clone(),
            success,
            output_file: if success { Some(script.output_path.clone()) } else { None },
            stdout: stdout.to_string(),
            stderr: stderr.to_string(),
        });
        
        // Clean up temp file
        let _ = std::fs::remove_file(temp_script);
    }
    
    Ok(results)
}

#[derive(Debug, Clone)]
pub struct ExecutionResult {
    pub script_name: String,
    pub success: bool,
    pub output_file: Option<PathBuf>,
    pub stdout: String,
    pub stderr: String,
}

/// Process all TOML files in a directory and generate scripts
pub fn process_toml_directory<P: AsRef<Path>>(
    toml_dir: P,
    template_dir: P,
    texture_base: P,
    output_base: P,
) -> Result<Vec<GeneratedScript>> {
    let processor = TemplateProcessor::new(template_dir, texture_base)?;
    let output_base_path = output_base.as_ref();
    let mut all_scripts = Vec::new();
    
    for entry in std::fs::read_dir(toml_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.extension().and_then(|s| s.to_str()) == Some("toml") {
            let scripts = processor.generate_script_from_toml(path, output_base_path.to_path_buf())?;
            all_scripts.extend(scripts);
        }
    }
    
    Ok(all_scripts)
}

/// Generate and execute all assets from TOML requests
pub fn generate_all_assets<P: AsRef<Path>>(
    toml_dir: P,
    template_dir: P,
    texture_base: P,
    output_base: P,
) -> Result<GenerationSummary> {
    println!("🎨 Generating assets with template-based pipeline...");
    
    // Generate scripts
    let scripts = process_toml_directory(&toml_dir, &template_dir, &texture_base, &output_base)?;
    println!("📝 Generated {} Blender scripts", scripts.len());
    
    // Execute scripts
    let results = execute_blender_scripts(&scripts)?;
    
    // Summarize results
    let successful = results.iter().filter(|r| r.success).count();
    let failed = results.len() - successful;
    
    let overworld_assets = results.iter()
        .filter(|r| r.success && scripts.iter().any(|s| s.name == r.script_name && s.perspective == "overworld"))
        .count();
    
    let fps_assets = results.iter()
        .filter(|r| r.success && scripts.iter().any(|s| s.name == r.script_name && s.perspective == "fps"))
        .count();
    
    println!("✅ Generation complete: {} successful ({} overworld, {} FPS), {} failed", 
             successful, overworld_assets, fps_assets, failed);
    
    Ok(GenerationSummary {
        total_requested: scripts.len(),
        successful_generations: successful,
        failed_generations: failed,
        overworld_assets,
        fps_assets,
        results,
    })
}

#[derive(Debug, Clone)]
pub struct GenerationSummary {
    pub total_requested: usize,
    pub successful_generations: usize,
    pub failed_generations: usize,
    pub overworld_assets: usize,
    pub fps_assets: usize,
    pub results: Vec<ExecutionResult>,
}
