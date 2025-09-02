use bevy::prelude::*;
use crate::resources::{GameSave, Stats, Abilities, LightDark};

pub fn save_on_key(
    keys: Res<ButtonInput<KeyCode>>,
    stats: Res<Stats>,
    abilities: Res<Abilities>,
    align: Res<LightDark>,
) {
    if keys.just_pressed(KeyCode::KeyS) {
        let gs = GameSave { stats: stats.clone(), abilities: abilities.unlocked.iter().cloned().collect(), light: align.light, dark: align.dark };
        if let Ok(text) = serde_json::to_string_pretty(&gs) {
            std::fs::create_dir_all("build/save").ok();
            std::fs::write("build/save/save.json", text).ok();
            info!("Saved build/save/save.json");
        }
    }
}
