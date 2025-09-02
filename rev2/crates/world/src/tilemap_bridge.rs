use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

pub fn axial_to_world(q: i32, r: i32, grid_size: &TilemapGridSize, coord: HexCoordSystem) -> Vec2 {
    match coord {
        HexCoordSystem::Row => {
            let x = (q as f32 + r as f32 * 0.5) * grid_size.x;
            let y = (r as f32 * (grid_size.y * 0.75));
            Vec2::new(x, y)
        }
        HexCoordSystem::Column => {
            let x = (q as f32 * (grid_size.x * 0.75));
            let y = (r as f32 + q as f32 * 0.5) * grid_size.y;
            Vec2::new(x, y)
        }
    }
}

pub fn parse_axial(s: &str) -> (i32, i32) {
    let mut it = s.split(',');
    let q: i32 = it.next().unwrap_or("0").trim().parse().unwrap_or(0);
    let r: i32 = it.next().unwrap_or("0").trim().parse().unwrap_or(0);
    (q, r)
}
