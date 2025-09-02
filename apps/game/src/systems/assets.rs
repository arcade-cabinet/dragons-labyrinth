use bevy::prelude::*;
use crate::resources::{AssetHandles, GameState};
use bevy::asset::LoadState;

pub fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut asset_handles: ResMut<AssetHandles>,
) {
    // Load tilemap texture atlas
    asset_handles.tilemap_texture = Some(asset_server.load("textures/tilemap.png"));
    
    // Load biome textures
    asset_handles.biome_textures.insert("grassland".to_string(), 
        asset_server.load("textures/biomes/grassland.png"));
    asset_handles.biome_textures.insert("forest".to_string(), 
        asset_server.load("textures/biomes/forest.png"));
    asset_handles.biome_textures.insert("mountain".to_string(), 
        asset_server.load("textures/biomes/mountain.png"));
    asset_handles.biome_textures.insert("desert".to_string(), 
        asset_server.load("textures/biomes/desert.png"));
    asset_handles.biome_textures.insert("swamp".to_string(), 
        asset_server.load("textures/biomes/swamp.png"));
    asset_handles.biome_textures.insert("water".to_string(), 
        asset_server.load("textures/biomes/water.png"));
    
    // Load feature models
    asset_handles.feature_models.insert("tavern".to_string(), 
        asset_server.load("models/tavern.glb"));
    asset_handles.feature_models.insert("tower".to_string(), 
        asset_server.load("models/tower.glb"));
    asset_handles.feature_models.insert("spring".to_string(), 
        asset_server.load("models/spring.glb"));
    asset_handles.feature_models.insert("ruins".to_string(), 
        asset_server.load("models/ruins.glb"));
    asset_handles.feature_models.insert("blacksmith".to_string(), 
        asset_server.load("models/blacksmith.glb"));
    
    // Load character models
    asset_handles.character_models.insert("player".to_string(),
        asset_server.load("models/characters/player.glb"));
    asset_handles.character_models.insert("companion_male".to_string(),
        asset_server.load("models/characters/companion_male.glb"));
    asset_handles.character_models.insert("companion_female".to_string(),
        asset_server.load("models/characters/companion_female.glb"));
    
    // Load dialogue files
    asset_handles.dialogue_files.insert("intro".to_string(),
        asset_server.load("dialogue/intro.yarn"));
    
    // Load audio files
    asset_handles.audio_files.insert("ambient_peace".to_string(),
        asset_server.load("audio/ambient_peace.ogg"));
    asset_handles.audio_files.insert("ambient_dread".to_string(),
        asset_server.load("audio/ambient_dread.ogg"));
    
    info!("Assets loading initiated");
}

pub fn asset_loading_system(
    asset_server: Res<AssetServer>,
    asset_handles: Res<AssetHandles>,
    mut game_state: ResMut<NextState<crate::game::GameStateEnum>>,
) {
    let mut all_loaded = true;
    
    // Check tilemap texture
    if let Some(handle) = &asset_handles.tilemap_texture {
        if asset_server.load_state(handle.id()) != LoadState::Loaded {
            all_loaded = false;
        }
    }
    
    // Check biome textures
    for (name, handle) in &asset_handles.biome_textures {
        if asset_server.load_state(handle.id()) != LoadState::Loaded {
            all_loaded = false;
            break;
        }
    }
    
    // Check feature models
    for (name, handle) in &asset_handles.feature_models {
        if asset_server.load_state(handle.id()) != LoadState::Loaded {
            all_loaded = false;
            break;
        }
    }
    
    // If all critical assets are loaded, transition to main menu
    if all_loaded {
        info!("All assets loaded, transitioning to main menu");
        game_state.set(crate::game::GameStateEnum::MainMenu);
    }
}

pub fn setup_asset_fallbacks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut asset_handles: ResMut<AssetHandles>,
) {
    // Create fallback meshes and materials for when assets fail to load
    let cube_mesh = meshes.add(Mesh::from(Cuboid::new(1.0, 1.0, 1.0)));
    let default_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.8, 0.7, 0.6),
        ..default()
    });
    
    asset_handles.fallback_mesh = Some(cube_mesh);
    asset_handles.fallback_material = Some(default_material);
}
