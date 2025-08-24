//! Camera settings

use crate::DreadLevel;

pub struct CameraSettings {
    dread_level: DreadLevel,
}

impl CameraSettings {
    pub fn new(dread_level: DreadLevel) -> Self {
        Self { dread_level }
    }
    
    pub fn shake_intensity(&self) -> f32 {
        self.dread_level.factor() * 0.5
    }
    
    pub fn field_of_view(&self) -> f32 {
        60.0 - self.dread_level.factor() * 20.0
    }
}
