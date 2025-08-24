//! Scale system for consistent sizing across all game elements
//! 
//! CRITICAL: These values must match Blender export settings!

use crate::DreadLevel;
use glam::Vec3;
use serde::{Deserialize, Serialize};

/// Base unit scale for the entire game
/// 1.0 = 1 meter in Blender = 1 unit in game
pub const BASE_UNIT_SCALE: f32 = 1.0;

/// Hex tile size (must match board generation)
pub const HEX_TILE_RADIUS: f32 = 1.0;
pub const HEX_TILE_HEIGHT: f32 = 0.2;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScaleSystem {
    dread_level: DreadLevel,
}

impl ScaleSystem {
    pub fn new(dread_level: DreadLevel) -> Self {
        Self { dread_level }
    }
    
    /// Character model scales
    pub fn character(&self) -> f32 {
        // Characters slightly shrink as horror increases (feeling small)
        match self.dread_level.0 {
            0 => 1.8,  // Normal human height
            1 => 1.75,
            2 => 1.7,
            3 => 1.65,
            4 => 1.6,  // Hunched, cowering
            _ => 1.8,
        }
    }
    
    /// Companion/mount scales
    pub fn mount(&self, mount_type: &str) -> f32 {
        let base = match mount_type {
            "war_horse" => 2.2,
            "wolf" => 1.4,
            "dragon" => 4.5,
            "spider" => 1.8,
            _ => 2.0,
        };
        
        // Mounts grow more imposing with dread
        base * (1.0 + self.dread_level.factor() * 0.3)
    }
    
    /// Environment prop scales
    pub fn prop(&self, prop_type: &str) -> f32 {
        let base = match prop_type {
            "tree" => 4.0,
            "rock" => 1.5,
            "house" => 3.5,
            "chest" => 0.8,
            "door" => 2.1,
            "torch" => 0.3,
            _ => 1.0,
        };
        
        // Environment warps with dread
        let distortion = match self.dread_level.0 {
            0 => 1.0,
            1 => 1.05,
            2 => 1.1,
            3 => 1.2,
            4 => 1.35, // Everything looms
            _ => 1.0,
        };
        
        base * distortion
    }
    
    /// UI element scales (screen-space)
    pub fn ui_element(&self, element_type: &str) -> f32 {
        let base = match element_type {
            "button" => 1.0,
            "health_bar" => 1.2,
            "inventory_slot" => 0.8,
            "dialogue_box" => 1.5,
            "notification" => 0.9,
            _ => 1.0,
        };
        
        // UI becomes more intrusive with dread
        base * (1.0 + self.dread_level.factor() * 0.2)
    }
    
    /// Text scales for different UI contexts
    pub fn text_scale(&self, context: &str) -> f32 {
        let base = match context {
            "title" => 2.0,
            "heading" => 1.5,
            "body" => 1.0,
            "tooltip" => 0.8,
            "debug" => 0.6,
            _ => 1.0,
        };
        
        // Text becomes harder to read at high dread
        let clarity = 1.0 - (self.dread_level.factor() * 0.1);
        base * clarity
    }
    
    /// Weapon scales
    pub fn weapon(&self, weapon_type: &str) -> f32 {
        let base = match weapon_type {
            "sword" => 1.0,
            "axe" => 1.2,
            "bow" => 1.1,
            "staff" => 1.8,
            "dagger" => 0.6,
            _ => 1.0,
        };
        
        // Weapons feel heavier at high dread
        base * (1.0 + self.dread_level.factor() * 0.15)
    }
    
    /// Particle effect scales
    pub fn particle(&self, effect_type: &str) -> f32 {
        let base = match effect_type {
            "fire" => 0.5,
            "smoke" => 2.0,
            "magic" => 1.0,
            "blood" => 0.3,
            "dust" => 0.1,
            _ => 1.0,
        };
        
        // Particles become more chaotic
        base * (1.0 + self.dread_level.factor() * 0.5)
    }
    
    /// Get Vec3 scale for 3D objects
    pub fn vec3_uniform(&self, scale: f32) -> Vec3 {
        Vec3::splat(scale)
    }
    
    /// Get non-uniform scale with dread distortion
    pub fn vec3_distorted(&self, base_scale: f32) -> Vec3 {
        let distortion = self.dread_level.factor();
        Vec3::new(
            base_scale * (1.0 + distortion * 0.1),
            base_scale * (1.0 - distortion * 0.05), // Squash vertically
            base_scale * (1.0 + distortion * 0.15), // Stretch depth
        )
    }
    
    /// Blender export settings for consistent pipeline
    pub fn blender_export_settings() -> BlenderExportSettings {
        BlenderExportSettings {
            unit_scale: BASE_UNIT_SCALE,
            apply_modifiers: true,
            apply_transforms: true,
            forward_axis: "-Z",
            up_axis: "Y",
            use_mesh_modifiers: true,
            export_materials: true,
        }
    }
}

/// Settings that must be used in Blender for consistent export
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlenderExportSettings {
    pub unit_scale: f32,
    pub apply_modifiers: bool,
    pub apply_transforms: bool,
    pub forward_axis: &'static str,
    pub up_axis: &'static str,
    pub use_mesh_modifiers: bool,
    pub export_materials: bool,
}

/// Camera distance scales for different view contexts
pub fn camera_distance(view_type: &str, dread_level: DreadLevel) -> f32 {
    let base = match view_type {
        "overview" => 20.0,
        "tactical" => 15.0,
        "exploration" => 10.0,
        "dialogue" => 5.0,
        "cinematic" => 8.0,
        _ => 12.0,
    };
    
    // Camera pulls in closer at high dread (claustrophobia)
    base * (1.0 - dread_level.factor() * 0.3)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_scale_consistency() {
        let scale = ScaleSystem::new(DreadLevel::PEACE);
        assert_eq!(scale.character(), 1.8);
        
        let scale_horror = ScaleSystem::new(DreadLevel::MADNESS);
        assert!(scale_horror.character() < scale.character());
    }
    
    #[test]
    fn test_blender_settings() {
        let settings = ScaleSystem::blender_export_settings();
        assert_eq!(settings.unit_scale, BASE_UNIT_SCALE);
        assert_eq!(settings.forward_axis, "-Z");
        assert_eq!(settings.up_axis, "Y");
    }
}
