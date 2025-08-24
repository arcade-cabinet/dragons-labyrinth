//! Programmatic style guide for Dragon's Labyrinth
//! 
//! This crate provides the single source of truth for all visual and aesthetic
//! decisions in the game. Every system queries this for consistent styling.

pub mod scale;
pub mod color;
pub mod typography;
pub mod animation;
pub mod ui_layout;
pub mod audio;
pub mod materials;
pub mod camera;
pub mod particles;
pub mod lighting;
pub mod mdbook_theme;
pub mod characters;
pub mod traits;
pub mod progression;

use serde::{Deserialize, Serialize};

/// Global dread level that affects all style decisions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DreadLevel(pub u8);

impl DreadLevel {
    pub const PEACE: Self = Self(0);
    pub const UNEASE: Self = Self(1);
    pub const ANXIETY: Self = Self(2);
    pub const HORROR: Self = Self(3);
    pub const MADNESS: Self = Self(4);
    
    /// Interpolation factor for smooth transitions
    pub fn factor(&self) -> f32 {
        self.0 as f32 / 4.0
    }
    
    /// Inverse factor (1.0 at peace, 0.0 at madness)
    pub fn inverse_factor(&self) -> f32 {
        1.0 - self.factor()
    }
}

/// Master style guide that orchestrates all visual decisions
pub struct StyleGuide {
    dread_level: DreadLevel,
}

impl StyleGuide {
    pub fn new(dread_level: DreadLevel) -> Self {
        Self { dread_level }
    }
    
    /// Get scale system for this dread level
    pub fn scale(&self) -> scale::ScaleSystem {
        scale::ScaleSystem::new(self.dread_level)
    }
    
    /// Get color palette for this dread level
    pub fn colors(&self) -> color::ColorPalette {
        color::ColorPalette::new(self.dread_level)
    }
    
    /// Get typography settings for this dread level
    pub fn typography(&self) -> typography::Typography {
        typography::Typography::new(self.dread_level)
    }
    
    /// Get animation timings for this dread level
    pub fn animation(&self) -> animation::AnimationTiming {
        animation::AnimationTiming::new(self.dread_level)
    }
    
    /// Get UI layout parameters for this dread level
    pub fn ui_layout(&self) -> ui_layout::UILayout {
        ui_layout::UILayout::new(self.dread_level)
    }
    
    /// Get audio curves for this dread level
    pub fn audio(&self) -> audio::AudioCurves {
        audio::AudioCurves::new(self.dread_level)
    }
    
    /// Get material properties for this dread level
    pub fn materials(&self) -> materials::MaterialProperties {
        materials::MaterialProperties::new(self.dread_level)
    }
    
    /// Get camera settings for this dread level
    pub fn camera(&self) -> camera::CameraSettings {
        camera::CameraSettings::new(self.dread_level)
    }
    
    /// Get particle effect parameters for this dread level
    pub fn particles(&self) -> particles::ParticleParameters {
        particles::ParticleParameters::new(self.dread_level)
    }
    
    /// Get lighting values for this dread level
    pub fn lighting(&self) -> lighting::LightingValues {
        lighting::LightingValues::new(self.dread_level)
    }
}

/// Quick access functions for common queries
impl StyleGuide {
    /// What scale should a character model be?
    pub fn character_scale(&self) -> f32 {
        self.scale().character()
    }
    
    /// What scale should an environment prop be?
    pub fn prop_scale(&self, prop_type: &str) -> f32 {
        self.scale().prop(prop_type)
    }
    
    /// What color should UI text be?
    pub fn ui_text_color(&self) -> [f32; 4] {
        self.colors().ui_text()
    }
    
    /// How long should a UI fade animation take?
    pub fn ui_fade_duration(&self) -> f32 {
        self.animation().ui_fade()
    }
    
    /// What's the screen shake intensity?
    pub fn screen_shake_intensity(&self) -> f32 {
        self.camera().shake_intensity()
    }
    
    /// How much decay/corruption should be applied?
    pub fn decay_intensity(&self) -> f32 {
        self.dread_level.factor()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_dread_interpolation() {
        assert_eq!(DreadLevel::PEACE.factor(), 0.0);
        assert_eq!(DreadLevel::MADNESS.factor(), 1.0);
        assert_eq!(DreadLevel(2).factor(), 0.5);
    }
    
    #[test]
    fn test_style_guide_creation() {
        let guide = StyleGuide::new(DreadLevel::ANXIETY);
        assert!(guide.character_scale() > 0.0);
    }
}
