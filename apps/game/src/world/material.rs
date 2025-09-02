use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::sprite::Material2d;

#[derive(AsBindGroup, TypePath, Asset, Clone)]
pub struct HexTileMaterial {
    #[texture(1)]
    #[sampler(2)]
    pub color_texture: Option<Handle<Image>>,
    #[uniform(0)]
    pub color: Color,
    #[uniform(0)]
    pub region: Vec4,
    #[uniform(0)]
    pub atlas_size: Vec2,
    #[uniform(0)]
    pub mask_bits: u32,
    #[uniform(0)]
    pub edge_color: Color,
    #[uniform(0)]
    pub edge_strength: f32,
}

impl Default for HexTileMaterial {
    fn default() -> Self {
        Self {
            color_texture: None,
            color: Color::WHITE,
            region: Vec4::ZERO,
            atlas_size: Vec2::new(1.0,1.0),
            mask_bits: 0,
            edge_color: Color::BLACK,
            edge_strength: 0.0,
        }
    }
}

impl Material2d for HexTileMaterial {
    fn fragment_shader() -> ShaderRef {
        ShaderRef::Path("shaders/hex_mask.wgsl".into())
    }
}