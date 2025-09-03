//! Hex coordinate system types

use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};
use crate::audit::AuditableType;
use std::collections::HashMap;

/// Hex coordinate using axial coordinate system (q, r)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component, Serialize, Deserialize)]
pub struct HexCoord {
    pub q: i32,  // Column
    pub r: i32,  // Row
}

impl AuditableType for HexCoord {
    fn audit_headers() -> Vec<String> {
        vec!["q".to_string(), "r".to_string(), "distance_from_origin".to_string()]
    }
    
    fn audit_row(&self) -> Vec<String> {
        vec![
            self.q.to_string(),
            self.r.to_string(), 
            self.distance_from_origin().to_string(),
        ]
    }
    
    fn audit_category() -> String {
        "world".to_string()
    }
    
    fn audit_subcategory() -> String {
        "coordinates".to_string()
    }
    
    fn extract_numeric_fields(&self) -> HashMap<String, f64> {
        let mut fields = HashMap::new();
        fields.insert("q".to_string(), self.q as f64);
        fields.insert("r".to_string(), self.r as f64);
        fields.insert("distance_from_origin".to_string(), self.distance_from_origin() as f64);
        fields
    }
}

impl HexCoord {
    /// Create new hex coordinate
    pub fn new(q: i32, r: i32) -> Self {
        Self { q, r }
    }
    
    /// Origin coordinate (0, 0)
    pub fn origin() -> Self {
        Self::new(0, 0)
    }
    
    /// Calculate distance from origin
    pub fn distance_from_origin(&self) -> u32 {
        self.distance_to(&Self::origin())
    }
    
    /// Calculate distance between two hex coordinates
    pub fn distance_to(&self, other: &HexCoord) -> u32 {
        ((self.q - other.q).abs() + (self.q + self.r - other.q - other.r).abs() + (self.r - other.r).abs()) as u32 / 2
    }
    
    /// Get all 6 neighbors in hex grid
    pub fn neighbors(&self) -> [HexCoord; 6] {
        [
            HexCoord::new(self.q + 1, self.r - 1), // Northeast
            HexCoord::new(self.q + 1, self.r),     // East
            HexCoord::new(self.q, self.r + 1),     // Southeast
            HexCoord::new(self.q - 1, self.r + 1), // Southwest
            HexCoord::new(self.q - 1, self.r),     // West  
            HexCoord::new(self.q, self.r - 1),     // Northwest
        ]
    }
    
    /// Get neighbor in specific direction (0-5)
    pub fn neighbor(&self, direction: usize) -> HexCoord {
        self.neighbors()[direction % 6]
    }
    
    /// Convert to cube coordinates (for some algorithms)
    pub fn to_cube(&self) -> (i32, i32, i32) {
        let x = self.q;
        let z = self.r;
        let y = -x - z;
        (x, y, z)
    }
    
    /// Convert from cube coordinates
    pub fn from_cube(x: i32, y: i32, z: i32) -> Self {
        debug_assert_eq!(x + y + z, 0);
        Self::new(x, z)
    }
}

/// Hex direction constants for movement
pub mod directions {
    pub const NORTHEAST: usize = 0;
    pub const EAST: usize = 1;
    pub const SOUTHEAST: usize = 2;
    pub const SOUTHWEST: usize = 3;
    pub const WEST: usize = 4;
    pub const NORTHWEST: usize = 5;
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_hex_distance() {
        let origin = HexCoord::origin();
        let test_coord = HexCoord::new(3, 2);
        
        assert_eq!(origin.distance_to(&test_coord), 5);
        assert_eq!(test_coord.distance_from_origin(), 5);
    }
    
    #[test]
    fn test_hex_neighbors() {
        let coord = HexCoord::new(0, 0);
        let neighbors = coord.neighbors();
        
        assert_eq!(neighbors[directions::NORTHEAST], HexCoord::new(1, -1));
        assert_eq!(neighbors[directions::EAST], HexCoord::new(1, 0));
        assert_eq!(neighbors[directions::SOUTHEAST], HexCoord::new(0, 1));
    }
    
    #[test]
    fn test_cube_conversion() {
        let hex = HexCoord::new(1, 2);
        let (x, y, z) = hex.to_cube();
        assert_eq!(x + y + z, 0);
        
        let converted_back = HexCoord::from_cube(x, y, z);
        assert_eq!(hex, converted_back);
    }
}
