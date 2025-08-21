// Build tools crate for Dragon's Labyrinth AI content generation
use async_openai::{
    types::{
        ChatCompletionRequestMessage, CreateChatCompletionRequestArgs,
        CreateImageRequestArgs, ImageModel, ImageQuality, ImageResponseFormat,
        ImageSize, Role,
    },
    Client,
};
use imageproc::drawing::*;
use palette::{IntoColor, Lab, Srgb};
use petgraph::{Graph, Directed};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use zstd::stream::{decode_all, encode_all};

/// Core build system following design bible principles
#[derive(Debug, Clone)]
pub struct BuildTools {
    pub openai_client: Client,
    pub asset_cache: AssetCache,
    pub style_guide: StyleGuide,
    pub dependency_graph: Graph<AssetNode, AssetDependency, Directed>,
    pub generation_queue: Vec<GenerationTask>,
}

/// Asset cache with compression and hashing
#[derive(Debug, Clone, Default)]
pub struct AssetCache {
    pub entries: HashMap<String, CacheEntry>,
    pub style_embeddings: HashMap<String, Vec<f32>>,
    pub generation_metrics: HashMap<String, GenerationMetrics>,
}

/// Style guide for consistent asset generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StyleGuide {
    pub color_palette: Vec<String>,      // 16-bit era palette (16-256 colors)
    pub sprite_dimensions: (u32, u32),   // Standard sprite size
    pub tile_dimensions: (u32, u32),     // Standard tile size
    pub outline_style: OutlineStyle,
    pub shading_technique: ShadingStyle,
    pub perspective: PerspectiveStyle,
    pub visual_constraints: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OutlineStyle {
    None,
    SinglePixelBlack,
    DoublePixelBlack,
    ColoredOutline(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ShadingStyle {
    Flat,
    CellShaded,
    PixelPerfect,
    Dithered,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PerspectiveStyle {
    TopDown,
    Isometric45,
    ThreeQuarter,
    SideView,
}

/// Asset dependency graph node
#[derive(Debug, Clone)]
pub struct AssetNode {
    pub id: String,
    pub asset_type: AssetType,
    pub generation_priority: u8,
    pub style_parent: Option<String>,
}

/// Asset dependency edge
#[derive(Debug, Clone)]
pub struct AssetDependency {
    pub dependency_type: DependencyType,
    pub strength: f32, // 0.0 to 1.0
}

#[derive(Debug, Clone)]
pub enum DependencyType {
    StyleInheritance,
    ColorPalette,
    ThematicConsistency,
    SizeConstraint,
}

/// Generation task with priority and dependencies
#[derive(Debug, Clone)]
pub struct GenerationTask {
    pub id: String,
    pub task_type: GenerationTaskType,
    pub prompt: String,
    pub constraints: GenerationConstraints,
    pub dependencies: Vec<String>,
    pub priority: u8,
}

#[derive(Debug, Clone)]
pub enum GenerationTaskType {
    StyleGuide,
    CharacterSprite,
    TileTexture,
    UIElement,
    AudioSFX,
    BackgroundMusic,
    DialogueAudio,
}

/// Cache entry with compression
#[derive(Debug, Clone)]
pub struct CacheEntry {
    pub id: String,
    pub data: Vec<u8>, // Compressed data
    pub metadata: CacheMetadata,
    pub hash: String,
}

#[derive(Debug, Clone)]
pub struct CacheMetadata {
    pub generation_time: chrono::DateTime<chrono::Utc>,
    pub prompt_hash: String,
    pub file_size: usize,
    pub asset_type: AssetType,
    pub style_compliance: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AssetType {
    Sprite,
    Tile,
    UI,
    Audio,
    Code,
    Metadata,
}

/// Generation constraints following 16-bit era
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationConstraints {
    pub max_colors: u8,
    pub dimensions: (u32, u32),
    pub file_format: String,
    pub style_adherence: f32, // 0.0 to 1.0
    pub dread_level: Option<u8>, // For Dragon's Labyrinth progression
}

/// Generation quality metrics
#[derive(Debug, Clone)]
pub struct GenerationMetrics {
    pub success_rate: f32,
    pub average_quality: f32,
    pub style_consistency: f32,
    pub generation_time: f32,
    pub retry_count: u32,
}

impl BuildTools {
    /// Initialize build tools with OpenAI client
    pub async fn new(api_key: String) -> Result<Self, Box<dyn std::error::Error>> {
        let client = Client::new().with_api_key(api_key);
        
        let style_guide = StyleGuide {
            color_palette: vec![
                "#000000".to_string(), "#FFFFFF".to_string(),
                "#FF0000".to_string(), "#00FF00".to_string(),
                "#0000FF".to_string(), "#FFFF00".to_string(),
                "#FF00FF".to_string(), "#00FFFF".to_string(),
                "#800000".to_string(), "#008000".to_string(),
                "#000080".to_string(), "#808000".to_string(),
                "#800080".to_string(), "#008080".to_string(),
                "#C0C0C0".to_string(), "#808080".to_string(),
            ],
            sprite_dimensions: (64, 64),
            tile_dimensions: (32, 32),
            outline_style: OutlineStyle::SinglePixelBlack,
            shading_technique: ShadingStyle::PixelPerfect,
            perspective: PerspectiveStyle::Isometric45,
            visual_constraints: vec![
                "No anti-aliasing".to_string(),
                "Pixel-perfect edges".to_string(),
                "Limited color palette".to_string(),
                "Consistent lighting angle".to_string(),
            ],
        };
        
        Ok(Self {
            openai_client: client,
            asset_cache: AssetCache::default(),
            style_guide,
            dependency_graph: Graph::new(),
            generation_queue: Vec::new(),
        })
    }
    
    /// Generate style guide for the game
    pub async fn generate_style_guide(&mut self, game_concept: &str) -> Result<StyleGuide, Box<dyn std::error::Error>> {
        let prompt = format!(
            "Create a comprehensive pixel art style guide for a horror RPG called 'Dragon's Labyrinth'. \
            Game concept: {}. \
            \nRequirements:\
            \n- 16-bit era aesthetic (1990-1995)\
            \n- 3/4 isometric view\
            \n- Limited color palette (16-256 colors)\
            \n- Pixel-perfect art style\
            \n- Horror atmosphere that escalates from peaceful to terrifying\
            \n\nProvide specific RGB color codes, sprite dimensions, and art direction rules.",
            game_concept
        );
        
        let request = CreateChatCompletionRequestArgs::default()
            .model("gpt-4o")
            .messages([ChatCompletionRequestMessage {
                role: Role::User,
                content: Some(prompt),
                name: None,
                tool_calls: None,
                tool_call_id: None,
            }])
            .build()?;
        
        let response = self.openai_client.chat().create(request).await?;
        
        if let Some(choice) = response.choices.first() {
            if let Some(content) = &choice.message.content {
                // Parse the response and update style guide
                // For now, return the existing style guide
                info!("Generated style guide: {}", content);
                return Ok(self.style_guide.clone());
            }
        }
        
        Err("Failed to generate style guide".into())
    }
    
    /// Generate sprite with style consistency
    pub async fn generate_sprite(&mut self, request: &SpriteGenerationRequest) -> Result<String, Box<dyn std::error::Error>> {
        // Check cache first
        let cache_key = self.compute_cache_key(&request.to_prompt());
        if let Some(entry) = self.asset_cache.entries.get(&cache_key) {
            let decompressed = decode_all(&entry.data[..])?;
            return Ok(String::from_utf8(decompressed)?);
        }
        
        // Build prompt with style constraints
        let style_prompt = self.build_style_constrained_prompt(&request.description);
        
        let image_request = CreateImageRequestArgs::default()
            .model(ImageModel::DallE3)
            .prompt(style_prompt)
            .size(ImageSize::S1024x1024)
            .quality(ImageQuality::Standard)
            .response_format(ImageResponseFormat::Url)
            .n(1)
            .build()?;
        
        let response = self.openai_client.images().create(image_request).await?;
        
        if let Some(image) = response.data.first() {
            if let Some(url) = &image.url {
                // Download and process the image
                let processed_sprite = self.post_process_sprite(url, &request.constraints).await?;
                
                // Cache the result
                self.cache_asset(&cache_key, &processed_sprite, AssetType::Sprite).await?;
                
                return Ok(processed_sprite);
            }
        }
        
        Err("Failed to generate sprite".into())
    }
    
    /// Post-process sprite for style consistency
    async fn post_process_sprite(&self, url: &str, constraints: &GenerationConstraints) -> Result<String, Box<dyn std::error::Error>> {
        // Download image
        let response = reqwest::get(url).await?;
        let image_bytes = response.bytes().await?;
        
        // Load image
        let img = image::load_from_memory(&image_bytes)?;
        let mut rgba_img = img.to_rgba8();
        
        // Apply palette quantization
        self.quantize_to_palette(&mut rgba_img, &self.style_guide.color_palette)?;
        
        // Apply pixel art filters
        self.apply_pixel_perfect_filter(&mut rgba_img);
        
        // Add outline if specified
        if matches!(self.style_guide.outline_style, OutlineStyle::SinglePixelBlack) {
            self.add_pixel_outline(&mut rgba_img);
        }
        
        // Resize to target dimensions
        let resized = image::imageops::resize(
            &rgba_img,
            constraints.dimensions.0,
            constraints.dimensions.1,
            image::imageops::FilterType::Nearest,
        );
        
        // Convert to base64 for caching
        let mut buffer = Vec::new();
        resized.write_to(&mut std::io::Cursor::new(&mut buffer), image::ImageFormat::Png)?;
        Ok(base64::encode(buffer))
    }
    
    /// Quantize image to style guide palette
    fn quantize_to_palette(&self, img: &mut image::RgbaImage, palette: &[String]) -> Result<(), Box<dyn std::error::Error>> {
        let palette_colors: Vec<Srgb<u8>> = palette
            .iter()
            .filter_map(|hex| {
                if hex.len() == 7 && hex.starts_with('#') {
                    let r = u8::from_str_radix(&hex[1..3], 16).ok()?;
                    let g = u8::from_str_radix(&hex[3..5], 16).ok()?;
                    let b = u8::from_str_radix(&hex[5..7], 16).ok()?;
                    Some(Srgb::new(r, g, b))
                } else {
                    None
                }
            })
            .collect();
        
        for pixel in img.pixels_mut() {
            let current = Srgb::new(pixel[0], pixel[1], pixel[2]);
            let current_lab: Lab = current.into_color();
            
            // Find closest palette color using perceptual distance
            let closest = palette_colors
                .iter()
                .min_by(|&&a, &&b| {
                    let a_lab: Lab = a.into_color();
                    let b_lab: Lab = b.into_color();
                    
                    let dist_a = (current_lab.l - a_lab.l).powi(2) +
                                (current_lab.a - a_lab.a).powi(2) +
                                (current_lab.b - b_lab.b).powi(2);
                    let dist_b = (current_lab.l - b_lab.l).powi(2) +
                                (current_lab.a - b_lab.a).powi(2) +
                                (current_lab.b - b_lab.b).powi(2);
                    
                    dist_a.partial_cmp(&dist_b).unwrap_or(std::cmp::Ordering::Equal)
                })
                .unwrap_or(&palette_colors[0]);
            
            pixel[0] = closest.red;
            pixel[1] = closest.green;
            pixel[2] = closest.blue;
        }
        
        Ok(())
    }
    
    /// Apply pixel-perfect filtering
    fn apply_pixel_perfect_filter(&self, img: &mut image::RgbaImage) {
        // Remove anti-aliasing by snapping colors to nearest palette values
        // This ensures crisp, pixel-perfect edges
        
        for pixel in img.pixels_mut() {
            // Simple threshold-based edge sharpening
            if pixel[3] < 128 { // Alpha threshold
                pixel[3] = 0;
            } else {
                pixel[3] = 255;
            }
        }
    }
    
    /// Add pixel-perfect outline
    fn add_pixel_outline(&self, img: &mut image::RgbaImage) {
        let (width, height) = img.dimensions();
        let mut outlined = img.clone();
        
        for y in 1..height-1 {
            for x in 1..width-1 {
                let current = img.get_pixel(x, y);
                
                // Check if this is an edge pixel
                if current[3] > 0 { // If pixel is not transparent
                    let neighbors = [
                        img.get_pixel(x-1, y),   // Left
                        img.get_pixel(x+1, y),   // Right
                        img.get_pixel(x, y-1),   // Up
                        img.get_pixel(x, y+1),   // Down
                    ];
                    
                    // Add outline if any neighbor is transparent
                    if neighbors.iter().any(|p| p[3] == 0) {
                        // Add black outline
                        for dy in -1..=1 {
                            for dx in -1..=1 {
                                let nx = (x as i32 + dx) as u32;
                                let ny = (y as i32 + dy) as u32;
                                
                                if nx < width && ny < height {
                                    let neighbor = img.get_pixel(nx, ny);
                                    if neighbor[3] == 0 {
                                        outlined.put_pixel(nx, ny, image::Rgba([0, 0, 0, 255]));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        *img = outlined;
    }
    
    /// Cache asset with compression
    async fn cache_asset(&mut self, key: &str, data: &str, asset_type: AssetType) -> Result<(), Box<dyn std::error::Error>> {
        let compressed = encode_all(data.as_bytes(), 3)?;
        
        let entry = CacheEntry {
            id: key.to_string(),
            data: compressed,
            metadata: CacheMetadata {
                generation_time: chrono::Utc::now(),
                prompt_hash: blake3::hash(data.as_bytes()).to_string(),
                file_size: data.len(),
                asset_type,
                style_compliance: 0.95, // Placeholder
            },
            hash: blake3::hash(data.as_bytes()).to_string(),
        };
        
        self.asset_cache.entries.insert(key.to_string(), entry);
        Ok(())
    }
    
    /// Build style-constrained prompt
    fn build_style_constrained_prompt(&self, base_description: &str) -> String {
        format!(
            "{}\n\
            CRITICAL STYLE REQUIREMENTS:\
            \n- 16-bit pixel art style from 1990-1995 era\
            \n- Isometric 3/4 view perspective\
            \n- Limited color palette: {}\
            \n- Single pixel black outline on all sprites\
            \n- No anti-aliasing or gradients\
            \n- Pixel-perfect edges and shapes\
            \n- Dimensions: {}x{} pixels\
            \n- Background should be transparent\
            \nAVOID: realistic rendering, 3D effects, modern art styles, gradients, anti-aliasing",
            base_description,
            self.style_guide.color_palette.join(", "),
            self.style_guide.sprite_dimensions.0,
            self.style_guide.sprite_dimensions.1
        )
    }
    
    /// Compute cache key for generation request
    fn compute_cache_key(&self, prompt: &str) -> String {
        blake3::hash(prompt.as_bytes()).to_string()
    }
}

/// Sprite generation request
#[derive(Debug, Clone)]
pub struct SpriteGenerationRequest {
    pub name: String,
    pub description: String,
    pub constraints: GenerationConstraints,
    pub dread_level: Option<u8>,
}

impl SpriteGenerationRequest {
    pub fn to_prompt(&self) -> String {
        format!("{}: {}", self.name, self.description)
    }
}

/// Default style guide for Dragon's Labyrinth
impl Default for StyleGuide {
    fn default() -> Self {
        Self {
            color_palette: vec![
                "#000000".to_string(), "#FFFFFF".to_string(),
                "#2D2D2D".to_string(), "#747474".to_string(),
                "#8B0000".to_string(), "#FF0000".to_string(),
                "#006400".to_string(), "#00FF00".to_string(),
                "#000080".to_string(), "#0000FF".to_string(),
                "#800080".to_string(), "#FF00FF".to_string(),
                "#B8860B".to_string(), "#FFFF00".to_string(),
                "#8B4513".to_string(), "#FFA500".to_string(),
            ],
            sprite_dimensions: (64, 64),
            tile_dimensions: (32, 32),
            outline_style: OutlineStyle::SinglePixelBlack,
            shading_technique: ShadingStyle::PixelPerfect,
            perspective: PerspectiveStyle::Isometric45,
            visual_constraints: vec![
                "16-bit era aesthetic".to_string(),
                "Pixel-perfect rendering".to_string(),
                "Limited color palette".to_string(),
                "Consistent isometric perspective".to_string(),
                "Single pixel outlines".to_string(),
            ],
        }
    }
}