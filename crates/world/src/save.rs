use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use std::fs;

use crate::abilities::{Stats, Abilities};
use crate::alignment::LightDark;

#[derive(Serialize, Deserialize)]
struct SaveData { stats: Stats, abilities: Vec<String>, light: u32, dark: u32 }

pub fn save_system(keys: Res<ButtonInput<KeyCode>>, stats: Res<Stats>, abilities: Res<Abilities>, align: Res<LightDark>) {
    if keys.just_pressed(KeyCode::KeyS) {
        let data = SaveData { stats: stats.clone(), abilities: abilities.unlocked.iter().cloned().collect(), light: align.light, dark: align.dark };
        let json = serde_json::to_string_pretty(&data).unwrap();
        std::fs::create_dir_all("build/save").ok();
        fs::write("build/save/save.json", json).ok();
        info!("Saved to build/save/save.json");
    }
}

pub fn load_system(mut commands: Commands) {
    if let Ok(raw) = std::fs::read_to_string("build/save/save.json") {
        if let Ok(sd) = serde_json::from_str::<SaveData>(&raw) {
            commands.insert_resource(sd.stats);
            let mut ab = Abilities::default();
            for a in sd.abilities { ab.unlocked.insert(a); }
            commands.insert_resource(ab);
            commands.insert_resource(LightDark{ light: sd.light, dark: sd.dark });
            info!("Loaded save.json");
        }
    }
}
