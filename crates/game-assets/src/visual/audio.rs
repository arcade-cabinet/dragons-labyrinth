//! Audio volume curves and parameters

use crate::DreadLevel;

pub struct AudioCurves {
    dread_level: DreadLevel,
}

impl AudioCurves {
    pub fn new(dread_level: DreadLevel) -> Self {
        Self { dread_level }
    }
    
    pub fn master_volume(&self) -> f32 {
        1.0 - self.dread_level.factor() * 0.2
    }
    
    pub fn ambient_volume(&self) -> f32 {
        0.3 + self.dread_level.factor() * 0.4
    }
}
