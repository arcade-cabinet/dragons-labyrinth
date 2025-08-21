// AI content generators for Dragon's Labyrinth following design bible
use serde::{Deserialize, Serialize};

/// Core game concept structure for AI generation
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameConcept {
    pub name: String,
    pub theme: String,
    pub setting: String,
    pub mood: String,
    pub visual_reference: Vec<String>,
    pub dread_progression: DreadProgression,
}

/// Dread progression system configuration
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DreadProgression {
    pub stages: Vec<DreadStage>,
}

/// Individual dread stage configuration
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DreadStage {
    pub level: u8,
    pub name: String,
    pub description: String,
    pub color_shift: ColorShift,
    pub audio_profile: AudioProfile,
    pub behavior_changes: Vec<String>,
}

/// Color transformation for visual corruption
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ColorShift {
    pub saturation_modifier: f32,
    pub brightness_modifier: f32,
    pub hue_shift: f32,
    pub corruption_overlay: Option<String>,
}

/// Audio profile for each dread stage
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AudioProfile {
    pub base_frequency: f32,
    pub distortion_level: f32,
    pub reverb_decay: f32,
    pub ambient_volume: f32,
}

/// Sprite generation request structure
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpriteRequest {
    pub name: String,
    pub description: String,
    pub style: String,
    pub size: (u32, u32),
    pub dread_variants: bool,
}

/// Tile generation request structure
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TileRequest {
    pub biome: String,
    pub corruption_level: f32,
    pub size: (u32, u32),
    pub seamless: bool,
}

/// Audio generation request structure
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AudioRequest {
    pub sound_type: String,
    pub mood: String,
    pub duration: f32,
    pub dread_level: u8,
}

/// Asset cache entry
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AssetCacheEntry {
    pub id: String,
    pub path: String,
    pub generation_parameters: serde_json::Value,
    pub timestamp: String,
    pub file_hash: String,
}

impl GameConcept {
    /// Create default Dragon's Labyrinth concept
    pub fn dragon_labyrinth_default() -> Self {
        Self {
            name: "Dragon's Labyrinth".to_string(),
            theme: "Psychological Horror RPG".to_string(),
            setting: "Ancient cursed labyrinth with escalating supernatural dread".to_string(),
            mood: "Peace → Unease → Dread → Terror → Horror".to_string(),
            visual_reference: vec![
                "Dark Souls atmosphere".to_string(),
                "Silent Hill psychological horror".to_string(),
                "Bloodborne cosmic dread".to_string(),
            ],
            dread_progression: DreadProgression::default_progression(),
        }
    }
}

impl DreadProgression {
    /// Create default 5-stage dread progression
    pub fn default_progression() -> Self {
        Self {
            stages: vec![
                DreadStage {
                    level: 0,
                    name: "Peace".to_string(),
                    description: "Bright world, mundane quests, friendly NPCs".to_string(),
                    color_shift: ColorShift {
                        saturation_modifier: 1.0,
                        brightness_modifier: 1.0,
                        hue_shift: 0.0,
                        corruption_overlay: None,
                    },
                    audio_profile: AudioProfile {
                        base_frequency: 220.0,
                        distortion_level: 0.0,
                        reverb_decay: 0.5,
                        ambient_volume: 0.5,
                    },
                    behavior_changes: vec!["Normal NPC behavior".to_string()],
                },
                DreadStage {
                    level: 1,
                    name: "Unease".to_string(),
                    description: "Shadows, whispers, Hollow Caretaker boss".to_string(),
                    color_shift: ColorShift {
                        saturation_modifier: 0.9,
                        brightness_modifier: 0.9,
                        hue_shift: -10.0,
                        corruption_overlay: None,
                    },
                    audio_profile: AudioProfile {
                        base_frequency: 200.0,
                        distortion_level: 0.1,
                        reverb_decay: 0.7,
                        ambient_volume: 0.4,
                    },
                    behavior_changes: vec![
                        "NPCs occasionally stare".to_string(),
                        "Subtle audio distortions".to_string(),
                    ],
                },
                DreadStage {
                    level: 2,
                    name: "Dread".to_string(),
                    description: "Swamps, ruins, economy collapse, Forsaken Knight boss".to_string(),
                    color_shift: ColorShift {
                        saturation_modifier: 0.7,
                        brightness_modifier: 0.7,
                        hue_shift: -20.0,
                        corruption_overlay: Some("rust_stains".to_string()),
                    },
                    audio_profile: AudioProfile {
                        base_frequency: 180.0,
                        distortion_level: 0.3,
                        reverb_decay: 1.0,
                        ambient_volume: 0.3,
                    },
                    behavior_changes: vec![
                        "NPCs become distrustful".to_string(),
                        "Companion Mira flees".to_string(),
                        "Environmental hazards appear".to_string(),
                    ],
                },
                DreadStage {
                    level: 3,
                    name: "Terror".to_string(),
                    description: "Reality warps, companion betrayal, moral horrors".to_string(),
                    color_shift: ColorShift {
                        saturation_modifier: 0.4,
                        brightness_modifier: 0.5,
                        hue_shift: -40.0,
                        corruption_overlay: Some("blood_stains".to_string()),
                    },
                    audio_profile: AudioProfile {
                        base_frequency: 160.0,
                        distortion_level: 0.6,
                        reverb_decay: 1.5,
                        ambient_volume: 0.2,
                    },
                    behavior_changes: vec![
                        "NPCs become hostile or flee".to_string(),
                        "Companions may betray player".to_string(),
                        "Reality distortions occur".to_string(),
                        "Moral choice consequences escalate".to_string(),
                    ],
                },
                DreadStage {
                    level: 4,
                    name: "Horror".to_string(),
                    description: "Dragon's labyrinth, stalking mechanics, final choice".to_string(),
                    color_shift: ColorShift {
                        saturation_modifier: 0.2,
                        brightness_modifier: 0.3,
                        hue_shift: -60.0,
                        corruption_overlay: Some("void_corruption".to_string()),
                    },
                    audio_profile: AudioProfile {
                        base_frequency: 140.0,
                        distortion_level: 0.9,
                        reverb_decay: 2.0,
                        ambient_volume: 0.1,
                    },
                    behavior_changes: vec![
                        "Dragon stalks player".to_string(),
                        "Reality completely unstable".to_string(),
                        "Final moral choice determines ending".to_string(),
                        "Companions lost or transformed".to_string(),
                    ],
                },
            ],
        }
    }
}

/// Content generator for sprites following design bible
pub struct SpriteGenerator {
    style_guide: Option<String>,
    cache: std::collections::HashMap<String, AssetCacheEntry>,
}

impl SpriteGenerator {
    pub fn new() -> Self {
        Self {
            style_guide: None,
            cache: std::collections::HashMap::new(),
        }
    }
    
    /// Generate style guide for consistent sprite generation
    pub fn generate_style_guide(&mut self, concept: &GameConcept) -> String {
        let style_guide = format!(
            "Style Guide for {}:\n\
            Theme: {}\n\
            Visual Style: Chess piece-like characters, simple geometric forms\n\
            Color Palette: Stage-dependent corruption levels\n\
            Art Direction: 2.5D isometric sprites with clear silhouettes\n\
            Technical: 64x64 pixel sprites, 4 corruption variants per sprite\n\
            Mood: {}\n\
            References: {}",
            concept.name,
            concept.theme,
            concept.mood,
            concept.visual_reference.join(", ")
        );
        
        self.style_guide = Some(style_guide.clone());
        style_guide
    }
    
    /// Generate sprite with dread variants
    pub fn generate_sprite(&mut self, request: &SpriteRequest) -> Result<Vec<String>, String> {
        // Check cache first
        let cache_key = format!("{}_{}", request.name, request.style);
        if let Some(cached) = self.cache.get(&cache_key) {
            return Ok(vec![cached.path.clone()]);
        }
        
        // Generate sprite variants
        let mut variants = Vec::new();
        
        if request.dread_variants {
            // Generate 5 corruption variants (Peace through Horror)
            for dread_level in 0..=4 {
                let variant_description = format!(
                    "{} - {} - Dread Level {}: {}",
                    request.description,
                    request.style,
                    dread_level,
                    match dread_level {
                        0 => "Clean, bright, normal appearance",
                        1 => "Subtle shadows, slightly muted colors",
                        2 => "Visible corruption, rust/decay elements",
                        3 => "Heavy corruption, disturbing details",
                        4 => "Complete corruption, nightmarish transformation",
                        _ => "Unknown corruption level",
                    }
                );
                
                let variant_path = format!("sprites/{}_{}.png", request.name, dread_level);
                variants.push(variant_path);
            }
        } else {
            let base_path = format!("sprites/{}.png", request.name);
            variants.push(base_path);
        }
        
        // Cache the result
        let cache_entry = AssetCacheEntry {
            id: cache_key.clone(),
            path: variants[0].clone(),
            generation_parameters: serde_json::to_value(request).unwrap(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            file_hash: "placeholder_hash".to_string(),
        };
        self.cache.insert(cache_key, cache_entry);
        
        Ok(variants)
    }
}

/// Content generator for tiles following design bible
pub struct TileGenerator {
    biome_definitions: std::collections::HashMap<String, BiomeDefinition>,
}

#[derive(Clone, Debug)]
pub struct BiomeDefinition {
    pub name: String,
    pub base_colors: Vec<String>,
    pub texture_patterns: Vec<String>,
    pub corruption_effects: Vec<String>,
}

impl TileGenerator {
    pub fn new() -> Self {
        let mut biome_definitions = std::collections::HashMap::new();
        
        // Define biomes for each dread stage
        biome_definitions.insert("meadow".to_string(), BiomeDefinition {
            name: "Peaceful Meadow".to_string(),
            base_colors: vec!["#4CAF50".to_string(), "#8BC34A".to_string()],
            texture_patterns: vec!["grass".to_string(), "flowers".to_string()],
            corruption_effects: vec!["none".to_string()],
        });
        
        biome_definitions.insert("forest".to_string(), BiomeDefinition {
            name: "Darkening Forest".to_string(),
            base_colors: vec!["#2E7D32".to_string(), "#1B5E20".to_string()],
            texture_patterns: vec!["dense_trees".to_string(), "shadows".to_string()],
            corruption_effects: vec!["withered_leaves".to_string()],
        });
        
        biome_definitions.insert("swamp".to_string(), BiomeDefinition {
            name: "Corrupted Swamp".to_string(),
            base_colors: vec!["#5D4037".to_string(), "#3E2723".to_string()],
            texture_patterns: vec!["murky_water".to_string(), "dead_trees".to_string()],
            corruption_effects: vec!["toxic_pools".to_string(), "rust_stains".to_string()],
        });
        
        biome_definitions.insert("ruins".to_string(), BiomeDefinition {
            name: "Ancient Ruins".to_string(),
            base_colors: vec!["#616161".to_string(), "#424242".to_string()],
            texture_patterns: vec!["cracked_stone".to_string(), "moss_covered".to_string()],
            corruption_effects: vec!["blood_stains".to_string(), "eldritch_symbols".to_string()],
        });
        
        biome_definitions.insert("labyrinth".to_string(), BiomeDefinition {
            name: "Dragon's Labyrinth".to_string(),
            base_colors: vec!["#212121".to_string(), "#000000".to_string()],
            texture_patterns: vec!["obsidian_walls".to_string(), "void_cracks".to_string()],
            corruption_effects: vec!["reality_distortion".to_string(), "dragon_presence".to_string()],
        });
        
        Self { biome_definitions }
    }
    
    /// Generate tile texture with corruption level
    pub fn generate_tile(&self, request: &TileRequest) -> Result<String, String> {
        let biome = self.biome_definitions.get(&request.biome)
            .ok_or_else(|| format!("Unknown biome: {}", request.biome))?;
        
        let corruption_stage = (request.corruption_level * 4.0) as usize;
        
        let tile_description = format!(
            "Hexagonal tile for {} biome, corruption level {:.1}, {} texture pattern",
            biome.name,
            request.corruption_level,
            biome.texture_patterns.first().unwrap_or(&"default".to_string())
        );
        
        let tile_path = format!(
            "tiles/{}_{}.png", 
            request.biome, 
            corruption_stage
        );
        
        // Return path where tile would be generated
        Ok(tile_path)
    }
}

/// Default implementations
impl Default for SpriteGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for TileGenerator {
    fn default() -> Self {
        Self::new()
    }
}