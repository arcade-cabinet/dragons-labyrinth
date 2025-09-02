use bevy::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct HexTile { pub q: i32, pub r: i32, pub biome: String, pub distance_band: String }


