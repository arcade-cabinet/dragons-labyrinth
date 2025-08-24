//! Asset intelligence system for analyzing and cataloging CC0 assets
//! This module analyzes what we HAVE and identifies what we NEED

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use anyhow::Result;

/// Complete asset manifest with semantic understanding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetManifest {
    /// All discovered assets with their metadata
    pub assets: Vec<AssetEntry>,
    
    /// Semantic categorization by game purpose
    pub categories: HashMap<String, Vec<AssetId>>,
    
    /// AI-analyzed gaps in our asset library
    pub gaps: Vec<AssetGap>,
    
    /// Generated prompts for missing assets
    pub generation_queue: Vec<GenerationPrompt>,
    
    /// Statistics about the library
    pub stats: AssetStats,
}

/// Individual asset with enriched metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetEntry {
    pub id: AssetId,
    pub path: PathBuf,
    pub asset_type: AssetType,
    pub source: AssetSource,
    pub tags: HashSet<String>,
    pub semantic_category: Option<String>,
    pub game_purpose: Option<GamePurpose>,
    pub dread_compatibility: Vec<u8>, // Which dread levels this works for
    pub style_match_score: f32, // How well it matches our style guide
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct AssetId(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AssetType {
    Model3D { 
        format: String,
        poly_count: Option<u32>,
        has_materials: bool,
    },
    Texture {
        format: String,
        resolution: (u32, u32),
        texture_type: TextureType,
    },
    Font {
        family: String,
        weight: String,
        style: String,
    },
    Audio {
        format: String,
        duration: f32,
        sample_rate: u32,
    },
    Animation {
        format: String,
        frame_count: u32,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TextureType {
    Diffuse,
    Normal,
    Roughness,
    Metallic,
    Emission,
    Occlusion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AssetSource {
    Kenney { collection: String },
    Quaternius { collection: String },
    Generated { agent: String, timestamp: i64 },
    Custom { author: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GamePurpose {
    Character { role: String },
    Environment { biome: String },
    Prop { category: String },
    UI { element: String },
    Effect { type_: String },
    Mount { species: String },
}

/// Identified gap in our asset library
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetGap {
    pub category: String,
    pub description: String,
    pub priority: Priority,
    pub required_for: Vec<String>, // Game systems that need this
    pub suggested_approach: GenerationApproach,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Priority {
    Critical,  // Blocks gameplay
    High,      // Important for experience
    Medium,    // Nice to have
    Low,       // Polish
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GenerationApproach {
    BlenderScript { template: String },
    AIGeneration { agent: String },
    Modification { base_asset: AssetId, operations: Vec<String> },
    Combination { assets: Vec<AssetId>, method: String },
}

/// Prompt for generating missing assets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationPrompt {
    pub id: String,
    pub gap_id: String,
    pub prompt_type: PromptType,
    pub prompt_text: String,
    pub target_specs: TargetSpecs,
    pub dependencies: Vec<AssetId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PromptType {
    BlenderPython,
    DialogueTree,
    MapLayout,
    UIConfiguration,
    AudioSpec,
    MaterialDefinition,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetSpecs {
    pub scale: f32,
    pub style_guide_params: HashMap<String, serde_json::Value>,
    pub dread_levels: Vec<u8>,
    pub output_format: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetStats {
    pub total_assets: usize,
    pub by_type: HashMap<String, usize>,
    pub by_source: HashMap<String, usize>,
    pub coverage_percentage: f32, // How much of our needed assets we have
    pub gaps_count: usize,
    pub queue_size: usize,
}

/// Asset intelligence analyzer
pub struct AssetIntelligence {
    manifest: AssetManifest,
    design_bible: DesignRequirements,
}

impl AssetIntelligence {
    /// Create a new intelligence system
    pub fn new() -> Self {
        Self {
            manifest: AssetManifest {
                assets: Vec::new(),
                categories: HashMap::new(),
                gaps: Vec::new(),
                generation_queue: Vec::new(),
                stats: AssetStats {
                    total_assets: 0,
                    by_type: HashMap::new(),
                    by_source: HashMap::new(),
                    coverage_percentage: 0.0,
                    gaps_count: 0,
                    queue_size: 0,
                },
            },
            design_bible: DesignRequirements::load(),
        }
    }
    
    /// Analyze a directory of assets
    pub fn analyze_directory(&mut self, path: &Path) -> Result<()> {
        // Scan for all assets
        self.scan_assets(path)?;
        
        // Categorize semantically
        self.categorize_assets()?;
        
        // Identify gaps against design bible
        self.identify_gaps()?;
        
        // Generate prompts for missing assets
        self.generate_prompts()?;
        
        // Update statistics
        self.update_stats();
        
        Ok(())
    }
    
    /// Save manifest for consumption by game-database
    pub fn save_manifest(&self, path: &Path) -> Result<()> {
        let json = serde_json::to_string_pretty(&self.manifest)?;
        std::fs::write(path, json)?;
        Ok(())
    }
    
    /// Load a previously saved manifest
    pub fn load_manifest(path: &Path) -> Result<AssetManifest> {
        let json = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str(&json)?)
    }
    
    fn scan_assets(&mut self, root: &Path) -> Result<()> {
        use walkdir::WalkDir;
        
        for entry in WalkDir::new(root)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            
            // Determine asset type from extension
            if let Some(asset_type) = self.identify_asset_type(path) {
                let id = AssetId(self.generate_asset_id(path));
                let source = self.identify_source(path);
                
                let asset = AssetEntry {
                    id: id.clone(),
                    path: path.to_path_buf(),
                    asset_type,
                    source,
                    tags: HashSet::new(),
                    semantic_category: None,
                    game_purpose: None,
                    dread_compatibility: vec![0, 1, 2, 3, 4], // Default: works for all
                    style_match_score: 0.5, // Default: neutral
                };
                
                self.manifest.assets.push(asset);
            }
        }
        
        Ok(())
    }
    
    fn identify_asset_type(&self, path: &Path) -> Option<AssetType> {
        let ext = path.extension()?.to_str()?;
        
        match ext.to_lowercase().as_str() {
            "fbx" | "obj" | "gltf" | "glb" => Some(AssetType::Model3D {
                format: ext.to_string(),
                poly_count: None,
                has_materials: path.with_extension("mtl").exists(),
            }),
            "png" | "jpg" | "jpeg" | "tga" => Some(AssetType::Texture {
                format: ext.to_string(),
                resolution: (1024, 1024), // TODO: Read actual resolution
                texture_type: self.guess_texture_type(path),
            }),
            "ttf" | "otf" | "woff" | "woff2" => Some(AssetType::Font {
                family: path.file_stem()?.to_str()?.to_string(),
                weight: "Regular".to_string(),
                style: "Normal".to_string(),
            }),
            "mp3" | "ogg" | "wav" => Some(AssetType::Audio {
                format: ext.to_string(),
                duration: 0.0, // TODO: Read actual duration
                sample_rate: 44100,
            }),
            _ => None,
        }
    }
    
    fn guess_texture_type(&self, path: &Path) -> TextureType {
        let name = path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_lowercase();
        
        if name.contains("normal") || name.contains("nm") {
            TextureType::Normal
        } else if name.contains("rough") {
            TextureType::Roughness
        } else if name.contains("metal") {
            TextureType::Metallic
        } else if name.contains("emission") || name.contains("emissive") {
            TextureType::Emission
        } else if name.contains("ao") || name.contains("occlusion") {
            TextureType::Occlusion
        } else {
            TextureType::Diffuse
        }
    }
    
    fn identify_source(&self, path: &Path) -> AssetSource {
        let path_str = path.to_string_lossy().to_lowercase();
        
        if path_str.contains("kenney") {
            AssetSource::Kenney {
                collection: self.extract_collection(&path_str, "kenney"),
            }
        } else if path_str.contains("quaternius") {
            AssetSource::Quaternius {
                collection: self.extract_collection(&path_str, "quaternius"),
            }
        } else {
            AssetSource::Custom {
                author: "unknown".to_string(),
            }
        }
    }
    
    fn extract_collection(&self, path: &str, source: &str) -> String {
        // Extract collection name from path
        if let Some(start) = path.find(source) {
            let after_source = &path[start + source.len()..];
            after_source.split('/').nth(1).unwrap_or("unknown").to_string()
        } else {
            "unknown".to_string()
        }
    }
    
    fn generate_asset_id(&self, path: &Path) -> String {
        // Generate a stable ID from the path
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        path.hash(&mut hasher);
        format!("asset_{:x}", hasher.finish())
    }
    
    fn categorize_assets(&mut self) -> Result<()> {
        // TODO: Use AI to semantically categorize assets
        // For now, use simple heuristics
        
        for asset in &mut self.manifest.assets {
            // Categorize based on path and filename
            let path_str = asset.path.to_string_lossy().to_lowercase();
            
            if path_str.contains("character") || path_str.contains("npc") {
                asset.semantic_category = Some("character".to_string());
                asset.game_purpose = Some(GamePurpose::Character {
                    role: "npc".to_string(),
                });
            } else if path_str.contains("environment") || path_str.contains("terrain") {
                asset.semantic_category = Some("environment".to_string());
                asset.game_purpose = Some(GamePurpose::Environment {
                    biome: "generic".to_string(),
                });
            } else if path_str.contains("ui") || path_str.contains("interface") {
                asset.semantic_category = Some("ui".to_string());
                asset.game_purpose = Some(GamePurpose::UI {
                    element: "generic".to_string(),
                });
            }
            
            // Add to category index
            if let Some(ref category) = asset.semantic_category {
                self.manifest.categories
                    .entry(category.clone())
                    .or_insert_with(Vec::new)
                    .push(asset.id.clone());
            }
        }
        
        Ok(())
    }
    
    fn identify_gaps(&mut self) -> Result<()> {
        // Compare what we have against what the design bible requires
        
        // Check each required asset from the design bible
        for required in &self.design_bible.required_assets {
            // Look for matching assets in our manifest
            let found_count = self.manifest.assets.iter()
                .filter(|a| a.semantic_category == Some(required.category.clone()))
                .count();
            
            if found_count < required.count {
                self.manifest.gaps.push(AssetGap {
                    category: required.category.clone(),
                    description: format!("Need {} more {} (have {}/{})", 
                        required.count - found_count, required.description, found_count, required.count),
                    priority: Priority::High,
                    required_for: vec!["core_gameplay".to_string()],
                    suggested_approach: GenerationApproach::AIGeneration {
                        agent: "specialized_agent".to_string(),
                    },
                });
            }
        }
        
        Ok(())
    }
    
    fn generate_prompts(&mut self) -> Result<()> {
        // Generate specific prompts for each gap
        
        for gap in &self.manifest.gaps {
            let prompt = GenerationPrompt {
                id: format!("prompt_{}", gap.category),
                gap_id: gap.category.clone(),
                prompt_type: PromptType::BlenderPython,
                prompt_text: format!("Generate {} for Dragon's Labyrinth", gap.description),
                target_specs: TargetSpecs {
                    scale: 1.0,
                    style_guide_params: HashMap::new(),
                    dread_levels: vec![0, 1, 2, 3, 4],
                    output_format: "gltf".to_string(),
                },
                dependencies: Vec::new(),
            };
            
            self.manifest.generation_queue.push(prompt);
        }
        
        Ok(())
    }
    
    fn update_stats(&mut self) {
        let stats = &mut self.manifest.stats;
        
        stats.total_assets = self.manifest.assets.len();
        stats.gaps_count = self.manifest.gaps.len();
        stats.queue_size = self.manifest.generation_queue.len();
        
        // Count by type
        for asset in &self.manifest.assets {
            let type_name = match &asset.asset_type {
                AssetType::Model3D { .. } => "model3d",
                AssetType::Texture { .. } => "texture",
                AssetType::Font { .. } => "font",
                AssetType::Audio { .. } => "audio",
                AssetType::Animation { .. } => "animation",
            };
            *stats.by_type.entry(type_name.to_string()).or_insert(0) += 1;
        }
        
        // Count by source
        for asset in &self.manifest.assets {
            let source_name = match &asset.source {
                AssetSource::Kenney { .. } => "kenney",
                AssetSource::Quaternius { .. } => "quaternius",
                AssetSource::Generated { .. } => "generated",
                AssetSource::Custom { .. } => "custom",
            };
            *stats.by_source.entry(source_name.to_string()).or_insert(0) += 1;
        }
        
        // Calculate coverage
        let total_needed = stats.total_assets + stats.gaps_count;
        stats.coverage_percentage = if total_needed > 0 {
            (stats.total_assets as f32 / total_needed as f32) * 100.0
        } else {
            0.0
        };
    }
}

/// Design requirements loaded from our design bible
#[derive(Debug, Clone)]
struct DesignRequirements {
    required_assets: Vec<RequiredAsset>,
}

#[derive(Debug, Clone)]
struct RequiredAsset {
    category: String,
    description: String,
    count: usize,
    variations_per_dread: usize,
}

impl DesignRequirements {
    fn load() -> Self {
        // TODO: Load from actual design bible
        Self {
            required_assets: vec![
                RequiredAsset {
                    category: "companion_models".to_string(),
                    description: "3D models for each companion".to_string(),
                    count: 12,
                    variations_per_dread: 5,
                },
                RequiredAsset {
                    category: "hex_tiles".to_string(),
                    description: "Hex tile variations for board".to_string(),
                    count: 20,
                    variations_per_dread: 5,
                },
                // ... more requirements
            ],
        }
    }
}
