use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct HexCoord {
    pub x: i32,
    pub y: i32,
}

impl HexCoord {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    
    pub fn zero() -> Self {
        Self { x: 0, y: 0 }
    }
    
    pub fn distance(&self, other: &HexCoord) -> i32 {
        ((self.x - other.x).abs() + (self.x + self.y - other.x - other.y).abs() + (self.y - other.y).abs()) / 2
    }
    
    pub fn neighbors(&self) -> Vec<HexCoord> {
        vec![
            HexCoord::new(self.x + 1, self.y),
            HexCoord::new(self.x + 1, self.y - 1),
            HexCoord::new(self.x, self.y - 1),
            HexCoord::new(self.x - 1, self.y),
            HexCoord::new(self.x - 1, self.y + 1),
            HexCoord::new(self.x, self.y + 1),
        ]
    }
    
    pub fn neighbors_within_range(&self, range: i32) -> Vec<HexCoord> {
        let mut neighbors = Vec::new();
        for x in -range..=range {
            for y in (-range).max(-x-range)..=range.min(-x+range) {
                let neighbor = HexCoord::new(self.x + x, self.y + y);
                if neighbor != *self {
                    neighbors.push(neighbor);
                }
            }
        }
        neighbors
    }
    
    pub fn to_world(&self) -> Vec3 {
        hex_to_world(*self)
    }
    
    pub fn from_world(world_pos: Vec3) -> Self {
        world_to_hex(world_pos)
    }
}

pub fn hex_to_world(hex: HexCoord) -> Vec3 {
    let size = 32.0; // Half the hex tile size
    let x = size * (3.0_f32.sqrt() * hex.x as f32 + 3.0_f32.sqrt() / 2.0 * hex.y as f32);
    let z = size * (3.0 / 2.0 * hex.y as f32);
    Vec3::new(x, 0.0, z)
}

pub fn world_to_hex(world_pos: Vec3) -> HexCoord {
    let size = 32.0;
    let q = (2.0 / 3.0 * world_pos.x) / size;
    let r = (-1.0 / 3.0 * world_pos.x + 3.0_f32.sqrt() / 3.0 * world_pos.z) / size;
    
    cube_to_hex(cube_round(q, -q - r, r))
}

fn cube_round(x: f32, y: f32, z: f32) -> (i32, i32, i32) {
    let rx = x.round();
    let ry = y.round();
    let rz = z.round();
    
    let x_diff = (rx - x).abs();
    let y_diff = (ry - y).abs();
    let z_diff = (rz - z).abs();
    
    if x_diff > y_diff && x_diff > z_diff {
        (-(ry + rz) as i32, ry as i32, rz as i32)
    } else if y_diff > z_diff {
        (rx as i32, -(rx + rz) as i32, rz as i32)
    } else {
        (rx as i32, ry as i32, -(rx + ry) as i32)
    }
}

fn cube_to_hex(cube: (i32, i32, i32)) -> HexCoord {
    HexCoord::new(cube.0, cube.2)
}

pub fn hex_direction(direction: i32) -> HexCoord {
    let directions = [
        HexCoord::new(1, 0),
        HexCoord::new(1, -1),
        HexCoord::new(0, -1),
        HexCoord::new(-1, 0),
        HexCoord::new(-1, 1),
        HexCoord::new(0, 1),
    ];
    
    directions[(direction % 6) as usize]
}

pub fn hex_neighbor(hex: HexCoord, direction: i32) -> HexCoord {
    let dir = hex_direction(direction);
    HexCoord::new(hex.x + dir.x, hex.y + dir.y)
}

pub fn hex_ring(center: HexCoord, radius: i32) -> Vec<HexCoord> {
    if radius == 0 {
        return vec![center];
    }
    
    let mut results = Vec::new();
    let mut hex = HexCoord::new(center.x + hex_direction(4).x * radius, center.y + hex_direction(4).y * radius);
    
    for direction in 0..6 {
        for _step in 0..radius {
            results.push(hex);
            hex = hex_neighbor(hex, direction);
        }
    }
    
    results
}

pub fn hex_spiral(center: HexCoord, radius: i32) -> Vec<HexCoord> {
    let mut results = vec![center];
    for k in 1..=radius {
        results.extend(hex_ring(center, k));
    }
    results
}

pub fn hex_line(start: HexCoord, end: HexCoord) -> Vec<HexCoord> {
    let distance = start.distance(&end);
    let mut results = Vec::new();
    
    for i in 0..=distance {
        let t = i as f32 / distance.max(1) as f32;
        let lerp_x = start.x as f32 * (1.0 - t) + end.x as f32 * t;
        let lerp_y = start.y as f32 * (1.0 - t) + end.y as f32 * t;
        
        results.push(world_to_hex(Vec3::new(lerp_x * 32.0, 0.0, lerp_y * 32.0)));
    }
    
    results
}

pub fn hex_range(center: HexCoord, range: i32) -> Vec<HexCoord> {
    let mut results = Vec::new();
    for x in -range..=range {
        for y in (-range).max(-x - range)..=range.min(-x + range) {
            results.push(HexCoord::new(center.x + x, center.y + y));
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_distance() {
        let a = HexCoord::new(0, 0);
        let b = HexCoord::new(3, 0);
        assert_eq!(a.distance(&b), 3);
    }

    #[test]
    fn test_hex_neighbors() {
        let hex = HexCoord::new(0, 0);
        let neighbors = hex.neighbors();
        assert_eq!(neighbors.len(), 6);
        assert!(neighbors.contains(&HexCoord::new(1, 0)));
        assert!(neighbors.contains(&HexCoord::new(0, 1)));
    }

    #[test]
    fn test_world_hex_conversion() {
        let hex = HexCoord::new(1, 1);
        let world = hex_to_world(hex);
        let back_to_hex = world_to_hex(world);
        assert_eq!(hex, back_to_hex);
    }
}
