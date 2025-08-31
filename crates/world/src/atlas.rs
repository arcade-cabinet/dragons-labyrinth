use bevy::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize)]
pub struct AtlasMeta { pub tiles: HashMap<String, AtlasRect>, pub tile_size: u32, pub width: u32, pub height: u32 }

#[derive(Debug, Clone, Deserialize, Copy)]
pub struct AtlasRect { pub u: u32, pub v: u32, pub w: u32, pub h: u32 }

#[derive(Resource, Default)]
pub struct AtlasIndex { pub atlas_image: Handle<Image>, pub meta: Option<AtlasMeta> }

impl AtlasIndex {
    pub fn rect_of(&self, key: &str) -> Option<AtlasRect> { self.meta.as_ref()?.tiles.get(key).copied() }
    pub fn size(&self) -> Option<(f32,f32)> { self.meta.as_ref().map(|m| (m.width as f32, m.height as f32)) }
}
