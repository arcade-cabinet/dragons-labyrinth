//! Asset loading and management for Dragon's Labyrinth

use bevy::prelude::*;
use std::path::PathBuf;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameAssets>();
    }
}

#[derive(Resource, Default)]
pub struct GameAssets {
    pub textures: Vec<Handle<Image>>,
    pub audio: Vec<Handle<AudioSource>>,
    pub models: Vec<Handle<Scene>>,
}

/// Get the assets directory path
pub fn assets_dir() -> PathBuf {
    if cfg!(debug_assertions) {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets")
    } else {
        PathBuf::from("assets")
    }
}