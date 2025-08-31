use bevy::prelude::*;
use std::collections::HashSet;

#[derive(Resource, Default, Debug, Clone)]
pub struct Stats { pub melee: u32, pub stealth: u32, pub lore: u32, pub craft: u32, pub light: u32, pub dark: u32 }

#[derive(Resource, Default, Debug, Clone)]
pub struct Abilities { pub unlocked: HashSet<String> }

pub fn award_xp(stats: &mut Stats, kind: &str, amount: u32) {
    match kind {
        "melee" => stats.melee += amount,
        "stealth" => stats.stealth += amount,
        "lore" => stats.lore += amount,
        "craft" => stats.craft += amount,
        "light" => stats.light += amount,
        "dark" => stats.dark += amount,
        _ => (),
    }
}

pub fn check_unlocks(stats: &Stats, abilities: &mut Abilities) {
    let thresholds = [
        ("power_strike", stats.melee >= 10),
        ("vanish", stats.stealth >= 10),
        ("omen", stats.lore >= 10),
        ("forge_blessing", stats.craft >= 10),
        ("radiant_aura", stats.light >= 5),
        ("shadow_step", stats.dark >= 5),
    ];
    for (name, ok) in thresholds {
        if ok { abilities.unlocked.insert(name.into()); }
    }
}
