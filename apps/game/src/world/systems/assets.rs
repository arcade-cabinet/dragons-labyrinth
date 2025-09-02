use bevy::prelude::*;
use crate::world::resources::{AssetHandles, GameState};
use bevy::asset::LoadState;

pub fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut asset_handles: ResMut<AssetHandles>,
) {
    // Load tilemap texture atlas
    asset_handles.tilemap_texture = Some(asset_server.load("textures/tilemap.png"));
    
    // Biome textures not yet present - rely on tilemap for now
    
    // Feature models not yet available - use fallbacks when spawning
    
    // Character models not yet available - use fallbacks when spawning
    
    // Load dialogue files
    asset_handles.dialogue_files.insert("intro".to_string(),
        asset_server.load("dialogue/intro.yarn"));
    
    // Audio handled by dedicated systems with concrete paths
    
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
