//! Material properties that change with dread

use crate::DreadLevel;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialProperties {
    dread_level: DreadLevel,
}

impl MaterialProperties {
    pub fn new(dread_level: DreadLevel) -> Self {
        Self { dread_level }
    }
    
    /// Roughness values for PBR materials
    pub fn roughness(&self, material_type: &str) -> f32 {
        let base = match material_type {
            "metal" => 0.2,
            "wood" => 0.8,
            "stone" => 0.9,
            "fabric" => 0.95,
            "water" => 0.0,
            "glass" => 0.0,
            _ => 0.5,
        };
        
        // Everything becomes rougher/dirtier with dread
        (base + self.dread_level.factor() * 0.3).min(1.0)
    }
    
    /// Metallic values for PBR materials
    pub fn metallic(&self, material_type: &str) -> f32 {
        let base = match material_type {
            "metal" => 1.0,
            "wood" => 0.0,
            "stone" => 0.0,
            "fabric" => 0.0,
            "water" => 0.0,
            "glass" => 0.0,
            _ => 0.0,
        };
        
        // Metals tarnish with dread
        base * (1.0 - self.dread_level.factor() * 0.5)
    }
    
    /// Emission intensity for glowing materials
    pub fn emission(&self, material_type: &str) -> f32 {
        let base = match material_type {
            "fire" => 3.0,
            "magic" => 2.0,
            "eyes" => 1.0,
            "torch" => 1.5,
            "crystal" => 0.5,
            _ => 0.0,
        };
        
        // Unnatural glow increases with dread
        base * (1.0 + self.dread_level.factor() * 2.0)
    }
    
    /// Normal map intensity
    pub fn normal_strength(&self) -> f32 {
        // Surface detail becomes more pronounced
        1.0 + self.dread_level.factor() * 1.0
    }
    
    /// Transparency/opacity
    pub fn opacity(&self, material_type: &str) -> f32 {
        let base = match material_type {
            "glass" => 0.2,
            "water" => 0.7,
            "fog" => 0.3,
            "ghost" => 0.5,
            _ => 1.0,
        };
        
        // Things become less clear with dread
        base + self.dread_level.factor() * 0.3
    }
    
    /// Texture tiling scale
    pub fn texture_scale(&self) -> f32 {
        // Textures tile more (becoming more repetitive/maddening)
        1.0 + self.dread_level.factor() * 0.5
    }
    
    /// Shader distortion amount
    pub fn distortion(&self) -> f32 {
        self.dread_level.factor() * 0.2
    }
}
