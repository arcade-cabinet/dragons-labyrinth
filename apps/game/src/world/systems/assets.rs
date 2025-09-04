use bevy::prelude::*;
use crate::world::state::AssetHandles;
use bevy::asset::LoadState;
use std::fs;
use std::path::Path;

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

    // After initiating loads, audit assets and write report
    generate_missing_assets_report();
}

pub fn asset_loading_system(
    asset_server: Res<AssetServer>,
    asset_handles: Res<AssetHandles>,
    mut game_state: ResMut<NextState<crate::game::GameStateEnum>>,
) {
    let mut all_loaded = true;
    
    // Check tilemap texture
    if let Some(handle) = &asset_handles.tilemap_texture {
        if !matches!(asset_server.load_state(handle.id()), LoadState::Loaded) {
            all_loaded = false;
        }
    }
    
    // Check biome textures
    for (name, handle) in &asset_handles.biome_textures {
        if !matches!(asset_server.load_state(handle.id()), LoadState::Loaded) {
            all_loaded = false;
            break;
        }
    }
    
    // Check feature models
    for (name, handle) in &asset_handles.feature_models {
        if !matches!(asset_server.load_state(handle.id()), LoadState::Loaded) {
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

fn generate_missing_assets_report() {
    // Scan code for known asset references and check presence in assets directory
    // This is a lightweight static list matching our current code paths
    let mut missing: Vec<String> = Vec::new();
    let root = Path::new("assets");

    // GLB models referenced in systems/components
    let model_paths = [
        // Biomes
        "models/biomes/core/grassland.glb",
        "models/biomes/core/forest.glb",
        "models/biomes/core/mountain.glb",
        "models/biomes/core/desert.glb",
        "models/biomes/core/swamp.glb",
        "models/biomes/core/water.glb",
        "models/biomes/core/snow.glb",
        "models/biomes/core/lava.glb",
        "models/biomes/core/void.glb",
        // Transitional
        "models/biomes/transitional/forest_grassland.glb",
        "models/biomes/transitional/mountain_forest.glb",
        "models/biomes/transitional/desert_mountain.glb",
        "models/biomes/transitional/swamp_water.glb",
        "models/biomes/transitional/snow_mountain.glb",
        // Corrupted
        "models/biomes/corrupted/grassland.glb",
        "models/biomes/corrupted/forest.glb",
        "models/biomes/corrupted/mountain.glb",
        "models/biomes/corrupted/desert.glb",
        "models/biomes/corrupted/swamp.glb",
        "models/biomes/corrupted/water.glb",
        "models/biomes/corrupted/snow.glb",
        // Void
        "models/biomes/void/grassland.glb",
        "models/biomes/void/forest.glb",
        "models/biomes/void/mountain.glb",
        "models/biomes/void/desert.glb",
        "models/biomes/void/swamp.glb",
        "models/biomes/void/water.glb",
        "models/biomes/void/snow.glb",
        "models/biomes/void/lava.glb",
        // Features
        "models/tavern.glb",
        "models/tower.glb",
        "models/spring.glb",
        "models/ruins.glb",
        "models/blacksmith.glb",
        // Characters
        "models/characters/player/male_base.glb",
        "models/characters/player/female_base.glb",
        "models/characters/monsters/wolf.glb",
        "models/characters/monsters/bandit.glb",
        "models/characters/monsters/goblin.glb",
        "models/characters/monsters/skeleton_warrior.glb",
        "models/characters/monsters/corrupted_beast.glb",
        "models/characters/monsters/dark_wizard.glb",
        "models/characters/monsters/void_creature.glb",
        "models/characters/monsters/dragon_spawn.glb",
        "models/characters/monsters/nightmare_entity.glb",
        "models/characters/npcs/villager.glb",
        "models/characters/npcs/merchant.glb",
        "models/characters/npcs/innkeeper.glb",
        "models/characters/npcs/guard.glb",
        "models/characters/npcs/blacksmith.glb",
        "models/characters/npcs/priest.glb",
        "models/characters/npcs/noble.glb",
        "models/characters/npcs/hermit.glb",
        "models/characters/npcs/quest_giver.glb",
    ];

    for rel in model_paths.iter() {
        if !root.join(rel).exists() {
            missing.push(format!("GLB: {}", rel));
        }
    }

    // UI and dialogue assets
    let cob_paths = [
        "ui/splash_screen.cob",
        "ui/main_menu.cob",
        "ui/character_creator.cob",
    ];
    for rel in cob_paths.iter() {
        if !root.join(rel).exists() {
            missing.push(format!("COB: {}", rel));
        }
    }

    let yarn_paths = [
        "dialogue/intro.yarn",
        "dialogue/villager_generic.yarn",
        "dialogue/merchant.yarn",
        "dialogue/innkeeper.yarn",
        "dialogue/guard.yarn",
        "dialogue/blacksmith.yarn",
        "dialogue/priest.yarn",
        "dialogue/noble.yarn",
        "dialogue/hermit.yarn",
        "dialogue/quest_giver.yarn",
    ];
    for rel in yarn_paths.iter() {
        if !root.join(rel).exists() {
            missing.push(format!("YARN: {}", rel));
        }
    }

    // PNG textures used at startup
    let png_paths = [
        "textures/tilemap.png",
        "textures/tilemap_extended.png",
    ];
    for rel in png_paths.iter() {
        if !root.join(rel).exists() {
            missing.push(format!("PNG: {}", rel));
        }
    }

    // Fonts referenced in UI
    let font_paths = [
        "fonts/Creepster-Regular.ttf",
        "fonts/MedievalSharp-Regular.ttf",
        "fonts/PressStart2P-Regular.ttf",
    ];
    for rel in font_paths.iter() {
        if !root.join(rel).exists() {
            missing.push(format!("FONT: {}", rel));
        }
    }

    // Write report
    let report_path = Path::new("apps/game/assets/MISSING_ASSETS.md");
    if let Some(parent) = report_path.parent() { let _ = fs::create_dir_all(parent); }
    let mut out = String::new();
    out.push_str("# Missing Assets Report\n\n");
    out.push_str("This report is generated at startup to document required assets absent from assets/. Replit pipeline can populate these.\n\n");
    if missing.is_empty() {
        out.push_str("All referenced assets are present.\n");
    } else {
        for m in missing {
            out.push_str("- ");
            out.push_str(&m);
            out.push('\n');
        }
    }
    let _ = fs::write(report_path, out);
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
