//! Color palette system that adapts to dread levels

use crate::DreadLevel;
use palette::{Srgba, Hsva, IntoColor};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorPalette {
    dread_level: DreadLevel,
}

impl ColorPalette {
    pub fn new(dread_level: DreadLevel) -> Self {
        Self { dread_level }
    }
    
    /// Primary color shifts from blue (calm) to red (danger)
    pub fn primary(&self) -> [f32; 4] {
        let hue = 210.0 - (self.dread_level.factor() * 210.0); // Blue to red
        let saturation = 0.7 + (self.dread_level.factor() * 0.3);
        let value = 0.9 - (self.dread_level.factor() * 0.3);
        
        let color: Srgba = Hsva::new(hue, saturation, value, 1.0).into_color();
        [color.red, color.green, color.blue, color.alpha]
    }
    
    /// Secondary color for accents
    pub fn secondary(&self) -> [f32; 4] {
        match self.dread_level.0 {
            0 => [0.49, 0.83, 0.13, 1.0], // Green (growth)
            1 => [0.96, 0.65, 0.14, 1.0], // Orange (caution)
            2 => [0.85, 0.37, 0.31, 1.0], // Red-orange (warning)
            3 => [0.55, 0.27, 0.52, 1.0], // Purple (corruption)
            4 => [0.13, 0.13, 0.13, 1.0], // Near black (void)
            _ => [0.5, 0.5, 0.5, 1.0],
        }
    }
    
    /// Background colors for UI
    pub fn background(&self) -> [f32; 4] {
        let brightness = 0.95 - (self.dread_level.factor() * 0.85);
        [brightness, brightness, brightness * 0.98, 1.0]
    }
    
    /// Fog/atmosphere color
    pub fn fog(&self) -> [f32; 4] {
        match self.dread_level.0 {
            0 => [0.8, 0.85, 0.9, 0.3],   // Light blue mist
            1 => [0.7, 0.7, 0.65, 0.4],   // Gray fog
            2 => [0.5, 0.45, 0.4, 0.6],   // Brown haze
            3 => [0.3, 0.2, 0.25, 0.8],   // Dark purple
            4 => [0.05, 0.0, 0.05, 0.95], // Impenetrable darkness
            _ => [0.5, 0.5, 0.5, 0.5],
        }
    }
    
    /// Blood/damage color
    pub fn damage(&self) -> [f32; 4] {
        let intensity = 0.7 + (self.dread_level.factor() * 0.3);
        [intensity, 0.0, 0.0, 1.0]
    }
    
    /// Healing/positive color
    pub fn healing(&self) -> [f32; 4] {
        let purity = 1.0 - (self.dread_level.factor() * 0.5);
        [0.0, purity, purity * 0.3, 1.0]
    }
    
    /// Magic/energy color
    pub fn magic(&self) -> [f32; 4] {
        match self.dread_level.0 {
            0 => [0.3, 0.6, 1.0, 1.0],  // Bright blue
            1 => [0.5, 0.4, 0.9, 1.0],  // Purple-blue
            2 => [0.7, 0.3, 0.7, 1.0],  // Purple
            3 => [0.8, 0.2, 0.4, 1.0],  // Crimson
            4 => [0.9, 0.1, 0.1, 1.0],  // Blood red
            _ => [0.5, 0.5, 0.5, 1.0],
        }
    }
    
    /// UI text color
    pub fn ui_text(&self) -> [f32; 4] {
        let brightness = 0.1 + (self.dread_level.factor() * 0.8);
        let opacity = 1.0 - (self.dread_level.factor() * 0.2);
        [brightness, brightness, brightness, opacity]
    }
    
    /// UI disabled element color
    pub fn ui_disabled(&self) -> [f32; 4] {
        [0.5, 0.5, 0.5, 0.5]
    }
    
    /// Corruption overlay color
    pub fn corruption(&self) -> [f32; 4] {
        let intensity = self.dread_level.factor();
        [intensity * 0.3, 0.0, intensity * 0.2, intensity * 0.5]
    }
    
    /// Companion-specific colors
    pub fn companion_color(&self, companion_type: &str) -> [f32; 4] {
        let base = match companion_type {
            "therapist" => [0.2, 0.4, 0.8, 1.0],    // Calming blue
            "philosopher" => [0.6, 0.3, 0.7, 1.0],  // Thoughtful purple
            "warrior" => [0.8, 0.2, 0.2, 1.0],      // Strong red
            "mystic" => [0.3, 0.7, 0.6, 1.0],       // Mystical teal
            _ => [0.5, 0.5, 0.5, 1.0],
        };
        
        // Desaturate with dread
        let desaturation = self.dread_level.factor() * 0.5;
        [
            base[0] * (1.0 - desaturation) + 0.5 * desaturation,
            base[1] * (1.0 - desaturation) + 0.5 * desaturation,
            base[2] * (1.0 - desaturation) + 0.5 * desaturation,
            base[3],
        ]
    }
    
    /// Material tint colors
    pub fn material_tint(&self, material_type: &str) -> [f32; 4] {
        let base = match material_type {
            "wood" => [0.55, 0.35, 0.15, 1.0],
            "stone" => [0.5, 0.5, 0.5, 1.0],
            "metal" => [0.7, 0.7, 0.75, 1.0],
            "fabric" => [0.6, 0.5, 0.4, 1.0],
            "flesh" => [0.9, 0.7, 0.6, 1.0],
            _ => [0.5, 0.5, 0.5, 1.0],
        };
        
        // Add corruption tint
        let corruption = self.corruption();
        [
            base[0] * (1.0 - corruption[3]) + corruption[0] * corruption[3],
            base[1] * (1.0 - corruption[3]) + corruption[1] * corruption[3],
            base[2] * (1.0 - corruption[3]) + corruption[2] * corruption[3],
            base[3],
        ]
    }
    
    /// Lighting colors
    pub fn ambient_light(&self) -> [f32; 4] {
        let intensity = 0.5 - (self.dread_level.factor() * 0.4);
        match self.dread_level.0 {
            0 => [intensity, intensity, intensity * 1.1, 1.0], // Slight blue
            1 => [intensity, intensity, intensity, 1.0],       // Neutral
            2 => [intensity * 1.1, intensity, intensity * 0.9, 1.0], // Slight yellow
            3 => [intensity * 1.2, intensity * 0.9, intensity * 0.9, 1.0], // Reddish
            4 => [intensity * 0.5, 0.0, intensity * 0.3, 1.0], // Purple darkness
            _ => [intensity, intensity, intensity, 1.0],
        }
    }
    
    /// Convert to hex string for CSS/web
    pub fn to_hex(color: [f32; 4]) -> String {
        format!("#{:02x}{:02x}{:02x}",
            (color[0] * 255.0) as u8,
            (color[1] * 255.0) as u8,
            (color[2] * 255.0) as u8,
        )
    }
}
