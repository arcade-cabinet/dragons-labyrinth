//! Lighting values and parameters

use crate::DreadLevel;

pub struct LightingValues {
    dread_level: DreadLevel,
}

impl LightingValues {
    pub fn new(dread_level: DreadLevel) -> Self {
        Self { dread_level }
    }
    
    pub fn ambient_intensity(&self) -> f32 {
        0.5 - self.dread_level.factor() * 0.4
    }
    
    pub fn shadow_strength(&self) -> f32 {
        0.5 + self.dread_level.factor() * 0.5
    }
}
