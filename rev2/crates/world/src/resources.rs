use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use std::collections::BTreeSet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Poi { pub axial: String, pub kind: String }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NpcData { pub id: String, pub name: String, pub axial: Option<String> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Region {
    pub name: String,
    pub band: (u32, u32),
    pub hex_points: Vec<Poi>,
    pub npcs: Vec<NpcData>,
}

#[derive(Resource, Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorldBook {
    pub title: String,
    pub regions: Vec<Region>,
}

#[derive(Resource, Debug, Clone, Serialize, Deserialize, Default)]
pub struct LightDark { pub light: u32, pub dark: u32 }
impl LightDark { pub fn tint(&self) -> Color {
    let l = self.light as f32; let d = self.dark as f32; let t = (l - d).clamp(-10.0, 10.0) / 10.0;
    Color::rgba(0.9 + 0.1*t.max(0.0), 0.9 + 0.1*t.max(0.0), 0.9 + 0.1*t.max(0.0), 1.0)
}}

#[derive(Resource, Debug, Clone, Serialize, Deserialize, Default)]
pub struct Stats { pub melee: u32, pub stealth: u32, pub lore: u32, pub craft: u32 }

#[derive(Resource, Debug, Clone, Serialize, Deserialize, Default)]
pub struct Abilities { pub unlocked: BTreeSet<String> }

pub fn award_xp(stats: &mut Stats, key: &str, amt: u32) {
    match key {
        "melee" => stats.melee += amt,
        "stealth" => stats.stealth += amt,
        "lore" => stats.lore += amt,
        "craft" => stats.craft += amt,
        _ => {}
    }
}

pub fn check_unlocks(stats: &Stats, abilities: &mut Abilities) {
    if stats.melee >= 10 { abilities.unlocked.insert("power_strike".into()); }
    if stats.stealth >= 10 { abilities.unlocked.insert("vanish".into()); }
    if stats.lore >= 10 { abilities.unlocked.insert("omen".into()); }
    if stats.craft >= 10 { abilities.unlocked.insert("forge_blessing".into()); }
    // light/dark driven abilities would live in another resource in a fuller pass
}

#[derive(Resource, Debug, Clone, Serialize, Deserialize, Default)]
pub struct GameSave { pub stats: Stats, pub abilities: Vec<String>, pub light: u32, pub dark: u32 }
