//! Typography settings

use crate::DreadLevel;

pub struct Typography {
    dread_level: DreadLevel,
}

impl Typography {
    pub fn new(dread_level: DreadLevel) -> Self {
        Self { dread_level }
    }
    
    pub fn font_family(&self) -> &'static str {
        match self.dread_level.0 {
            0..=1 => "Roboto",
            2..=3 => "Creepster",
            4 => "Chiller",
            _ => "Roboto",
        }
    }
    
    pub fn line_height(&self) -> f32 {
        1.5 - self.dread_level.factor() * 0.3
    }
}
