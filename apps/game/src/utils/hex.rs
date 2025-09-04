use bevy::prelude::*;
use dl_types::world::HexCoord;

pub fn hex_to_world(hex: HexCoord) -> Vec3 {
    let size = 32.0; // Half the hex tile size
    let x = size * (3.0_f32.sqrt() * hex.q as f32 + 3.0_f32.sqrt() / 2.0 * hex.r as f32);
    let z = size * (3.0 / 2.0 * hex.r as f32);
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
    HexCoord::new(hex.q + dir.q, hex.r + dir.r)
}

pub fn hex_ring(center: HexCoord, radius: i32) -> Vec<HexCoord> {
    if radius == 0 {
        return vec![center];
    }
    
    let mut results = Vec::new();
    let dir_4 = hex_direction(4);
    let mut hex = HexCoord::new(center.q + dir_4.q * radius, center.r + dir_4.r * radius);
    
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
    let distance = hex_distance(start, end);
    let mut results = Vec::new();
    
    for i in 0..=distance {
        let t = i as f32 / distance.max(1) as f32;
        let lerp_q = start.q as f32 * (1.0 - t) + end.q as f32 * t;
        let lerp_r = start.r as f32 * (1.0 - t) + end.r as f32 * t;
        
        results.push(world_to_hex(Vec3::new(lerp_q * 32.0, 0.0, lerp_r * 32.0)));
    }
    
    results
}

pub fn hex_range(center: HexCoord, range: i32) -> Vec<HexCoord> {
    let mut results = Vec::new();
    for q in -range..=range {
        for r in (-range).max(-q - range)..=range.min(-q + range) {
            results.push(HexCoord::new(center.q + q, center.r + r));
        }
    }
    results
}

pub fn hex_distance(a: HexCoord, b: HexCoord) -> i32 {
    ((a.q - b.q).abs() + (a.q + a.r - b.q - b.r).abs() + (a.r - b.r).abs()) / 2
}

pub fn hex_neighbors(hex: HexCoord) -> Vec<HexCoord> {
    vec![
        HexCoord::new(hex.q + 1, hex.r),
        HexCoord::new(hex.q + 1, hex.r - 1),
        HexCoord::new(hex.q, hex.r - 1),
        HexCoord::new(hex.q - 1, hex.r),
        HexCoord::new(hex.q - 1, hex.r + 1),
        HexCoord::new(hex.q, hex.r + 1),
    ]
}

pub fn hex_neighbors_within_range(center: HexCoord, range: i32) -> Vec<HexCoord> {
    let mut neighbors = Vec::new();
    for q in -range..=range {
        for r in (-range).max(-q-range)..=range.min(-q+range) {
            let neighbor = HexCoord::new(center.q + q, center.r + r);
            if neighbor != center {
                neighbors.push(neighbor);
            }
        }
    }
    neighbors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_distance() {
        let a = HexCoord::new(0, 0);
        let b = HexCoord::new(3, 0);
        assert_eq!(hex_distance(a, b), 3);
    }

    #[test]
    fn test_hex_neighbors() {
        let hex = HexCoord::new(0, 0);
        let neighbors = hex_neighbors(hex);
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
