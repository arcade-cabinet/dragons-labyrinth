use bevy::prelude::*;
use crate::material::HexTileMaterial;
use crate::resources::Lighting;

pub fn cycle_ambient(time: Res<Time>, mut lighting: ResMut<Lighting>, mut mats: ResMut<Assets<HexTileMaterial>>) {
    let t = (time.elapsed_seconds() * 0.05).sin() * 0.5 + 0.5;
    lighting.ambient = 0.4 + 0.4 * t;
    for (_h, m) in mats.iter_mut() {
        m.color.set_a(lighting.ambient as f32);
    }
}
