use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use std::collections::HashMap;

// Asset collections following design bible zero dependencies principle
#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    // Hex tile models - AI generated .glb files
    #[asset(path = "models/hex_grass.glb")]
    pub hex_grass: Handle<Scene>,
    
    #[asset(path = "models/hex_forest.glb")]
    pub hex_forest: Handle<Scene>,
    
    #[asset(path = "models/hex_stone.glb")]
    pub hex_stone: Handle<Scene>,
    
    #[asset(path = "models/hex_water.glb")]
    pub hex_water: Handle<Scene>,
    
    #[asset(path = "models/hex_corrupted.glb")]
    pub hex_corrupted: Handle<Scene>,
    
    #[asset(path = "models/hex_void.glb")]
    pub hex_void: Handle<Scene>,
    
    // Special structures
    #[asset(path = "models/labyrinth_portal.glb")]
    pub labyrinth_portal: Handle<Scene>,
    
    // Companion models
    #[asset(path = "models/companions/einar.glb")]
    pub companion_einar: Handle<Scene>,
    
    #[asset(path = "models/companions/mira.glb")]
    pub companion_mira: Handle<Scene>,
    
    #[asset(path = "models/companions/sorin.glb")]
    pub companion_sorin: Handle<Scene>,
    
    #[asset(path = "models/companions/tamara.glb")]
    pub companion_tamara: Handle<Scene>,
    
    // Audio collections per dread level (Freesound CC0)
    #[asset(path = "audio/ambient/peace.ogg")]
    pub ambient_peace: Handle<AudioSource>,
    
    #[asset(path = "audio/ambient/unease.ogg")]
    pub ambient_unease: Handle<AudioSource>,
    
    #[asset(path = "audio/ambient/dread.ogg")]
    pub ambient_dread: Handle<AudioSource>,
    
    #[asset(path = "audio/ambient/terror.ogg")]
    pub ambient_terror: Handle<AudioSource>,
    
    #[asset(path = "audio/ambient/horror.ogg")]
    pub ambient_horror: Handle<AudioSource>,
    
    // Dragon proximity audio
    #[asset(path = "audio/dragon/distant_breath.ogg")]
    pub dragon_distant: Handle<AudioSource>,
    
    #[asset(path = "audio/dragon/approaching.ogg")]
    pub dragon_approaching: Handle<AudioSource>,
    
    #[asset(path = "audio/dragon/close.ogg")]
    pub dragon_close: Handle<AudioSource>,
}

// Asset generation system following idempotent principles
#[derive(Resource)]
pub struct AssetGenerator {
    pub generation_queue: Vec<AssetRequest>,
    pub generated_assets: HashMap<String, String>, // ID -> Path mapping
}

#[derive(Clone, Debug)]
pub struct AssetRequest {
    pub asset_type: AssetType,
    pub id: String,
    pub parameters: AssetParameters,
}

#[derive(Clone, Debug)]
pub enum AssetType {
    HexTile,
    Companion,
    Audio,
    Icon,
    Environment,
}

#[derive(Clone, Debug)]
pub struct AssetParameters {
    pub dread_level: u8,
    pub style: String,
    pub variations: u8,
    pub quality: AssetQuality,
}

#[derive(Clone, Debug)]
pub enum AssetQuality {
    Low,    // Mobile optimized
    Medium, // Balanced
    High,   // Desktop quality
}

impl Default for AssetGenerator {
    fn default() -> Self {
        Self {
            generation_queue: Vec::new(),
            generated_assets: HashMap::new(),
        }
    }
}

impl AssetGenerator {
    // Generate deterministic ID for idempotent generation
    pub fn generate_asset_id(&self, asset_type: &AssetType, parameters: &AssetParameters) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        format!("{:?}", asset_type).hash(&mut hasher);
        parameters.dread_level.hash(&mut hasher);
        parameters.style.hash(&mut hasher);
        parameters.variations.hash(&mut hasher);
        format!("{:?}", parameters.quality).hash(&mut hasher);
        
        format!("asset_{:x}", hasher.finish())
    }
    
    // Queue asset for generation following design bible principles
    pub fn queue_asset_generation(&mut self, asset_type: AssetType, parameters: AssetParameters) -> String {
        let id = self.generate_asset_id(&asset_type, &parameters);
        
        // Check if already generated (idempotent)
        if !self.generated_assets.contains_key(&id) {
            let request = AssetRequest {
                asset_type,
                id: id.clone(),
                parameters,
            };
            self.generation_queue.push(request);
        }
        
        id
    }
    
    // Generate hex tile variations for biome system
    pub fn generate_biome_tiles(&mut self, dread_level: u8) {
        let base_tiles = vec!["grass", "forest", "stone", "water"];
        
        for tile_type in base_tiles {
            let parameters = AssetParameters {
                dread_level,
                style: tile_type.to_string(),
                variations: 3, // Multiple variations per type
                quality: AssetQuality::Medium,
            };
            
            self.queue_asset_generation(AssetType::HexTile, parameters);
        }
        
        // Add corruption tiles for higher dread levels
        if dread_level >= 2 {
            let corrupted_params = AssetParameters {
                dread_level,
                style: "corrupted".to_string(),
                variations: 2,
                quality: AssetQuality::Medium,
            };
            self.queue_asset_generation(AssetType::HexTile, corrupted_params);
        }
        
        // Add void tiles for horror stage
        if dread_level >= 4 {
            let void_params = AssetParameters {
                dread_level,
                style: "void".to_string(),
                variations: 1,
                quality: AssetQuality::Medium,
            };
            self.queue_asset_generation(AssetType::HexTile, void_params);
        }
    }
    
    // Generate companion models with trauma variations
    pub fn generate_companion_models(&mut self) {
        let companions = vec!["einar", "mira", "sorin", "tamara"];
        
        for companion in companions {
            for trauma_level in 0..4 {
                let parameters = AssetParameters {
                    dread_level: trauma_level,
                    style: companion.to_string(),
                    variations: 1,
                    quality: AssetQuality::Medium,
                };
                
                self.queue_asset_generation(AssetType::Companion, parameters);
            }
        }
    }
    
    // Generate audio assets using Freesound API
    pub fn generate_audio_collection(&mut self) {
        // Ambient tracks per dread level
        for dread_level in 0..5 {
            let stage_name = match dread_level {
                0 => "peace",
                1 => "unease", 
                2 => "dread",
                3 => "terror",
                4 => "horror",
                _ => "unknown",
            };
            
            let parameters = AssetParameters {
                dread_level,
                style: format!("ambient_{}", stage_name),
                variations: 2, // Primary and alternate tracks
                quality: AssetQuality::Medium,
            };
            
            self.queue_asset_generation(AssetType::Audio, parameters);
        }
        
        // Dragon proximity audio
        let dragon_audio = vec!["distant", "approaching", "close", "breathing"];
        for audio_type in dragon_audio {
            let parameters = AssetParameters {
                dread_level: 4, // Horror stage
                style: format!("dragon_{}", audio_type),
                variations: 3,
                quality: AssetQuality::High, // Important for horror effect
            };
            
            self.queue_asset_generation(AssetType::Audio, parameters);
        }
    }
}

// Asset loading states for progressive loading
#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum AssetLoadingState {
    #[default]
    Loading,
    Ready,
    Failed,
}

// Plugin for asset management
pub struct AssetManagementPlugin;

impl Plugin for AssetManagementPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<AssetLoadingState>()
            .init_resource::<AssetGenerator>()
            .add_systems(OnEnter(AssetLoadingState::Loading), setup_asset_generation)
            .add_systems(Update, 
                (check_asset_loading, process_generation_queue)
                    .run_if(in_state(AssetLoadingState::Loading))
            );
    }
}

fn setup_asset_generation(mut asset_generator: ResMut<AssetGenerator>) {
    info!("Setting up asset generation following design bible principles");
    
    // Generate all required assets for game
    for dread_level in 0..5 {
        asset_generator.generate_biome_tiles(dread_level);
    }
    
    asset_generator.generate_companion_models();
    asset_generator.generate_audio_collection();
    
    info!("Queued {} assets for generation", asset_generator.generation_queue.len());
}

fn process_generation_queue(mut asset_generator: ResMut<AssetGenerator>) {
    // Process asset generation queue
    // In real implementation, this would interface with AI generation APIs
    while let Some(request) = asset_generator.generation_queue.pop() {
        info!("Processing asset generation: {:?}", request.id);
        
        // Simulate asset path generation
        let asset_path = match request.asset_type {
            AssetType::HexTile => format!("models/tiles/{}_{}.glb", request.parameters.style, request.id),
            AssetType::Companion => format!("models/companions/{}_{}.glb", request.parameters.style, request.id),
            AssetType::Audio => format!("audio/{}_{}.ogg", request.parameters.style, request.id),
            AssetType::Icon => format!("icons/{}_{}.svg", request.parameters.style, request.id),
            AssetType::Environment => format!("models/environment/{}_{}.glb", request.parameters.style, request.id),
        };
        
        asset_generator.generated_assets.insert(request.id, asset_path);
    }
}

fn check_asset_loading(
    asset_generator: Res<AssetGenerator>,
    mut next_state: ResMut<NextState<AssetLoadingState>>,
) {
    // Check if all assets are loaded and ready
    if asset_generator.generation_queue.is_empty() {
        info!("Asset generation complete, transitioning to Ready state");
        next_state.set(AssetLoadingState::Ready);
    }
}