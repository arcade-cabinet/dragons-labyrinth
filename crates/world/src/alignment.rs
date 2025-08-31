use bevy::prelude::*;

#[derive(Resource, Debug, Clone)]
pub struct LightDark { pub light: u32, pub dark: u32 }

impl Default for LightDark {
    fn default() -> Self { Self { light: 0, dark: 0 } }
}

impl LightDark {
    pub fn tint(&self) -> Color {
        let total = (self.light + self.dark).max(1) as f32;
        let l = self.light as f32 / total;
        let d = self.dark as f32 / total;
        Color::rgba(0.9 + 0.1*l, 0.9 - 0.2*d, 0.9 - 0.2*d, 1.0)
    }
}
