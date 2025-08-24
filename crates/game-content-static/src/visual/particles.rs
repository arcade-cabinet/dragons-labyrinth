//! Particle effect parameters

use crate::DreadLevel;

pub struct ParticleParameters {
    dread_level: DreadLevel,
}

impl ParticleParameters {
    pub fn new(dread_level: DreadLevel) -> Self {
        Self { dread_level }
    }
    
    pub fn emission_rate(&self) -> f32 {
        10.0 + self.dread_level.factor() * 50.0
    }
    
    pub fn lifetime(&self) -> f32 {
        2.0 + self.dread_level.factor() * 3.0
    }
}
