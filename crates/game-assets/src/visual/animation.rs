//! Animation timing parameters

use crate::DreadLevel;

pub struct AnimationTiming {
    dread_level: DreadLevel,
}

impl AnimationTiming {
    pub fn new(dread_level: DreadLevel) -> Self {
        Self { dread_level }
    }
    
    pub fn ui_fade(&self) -> f32 {
        0.3 + self.dread_level.factor() * 0.5
    }
    
    pub fn screen_transition(&self) -> f32 {
        0.5 + self.dread_level.factor() * 1.0
    }
}
