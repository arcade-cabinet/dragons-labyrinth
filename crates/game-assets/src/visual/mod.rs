//! Visual style system - how the game looks at different dread levels
//!
//! All visual decisions are dread-responsive, creating the horror progression
//! through gradual visual degradation and corruption.

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

use crate::DreadLevel;

/// Master visual style orchestrator
pub struct VisualStyle {
    dread_level: DreadLevel,
}

impl VisualStyle {
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
impl VisualStyle {
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
