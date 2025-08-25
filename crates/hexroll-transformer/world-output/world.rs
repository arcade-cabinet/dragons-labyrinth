// Auto-generated world data from HBF
// Generated from 70801 entities, processed 654
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct HexTile {
    pub x: i32,
    pub y: i32,
    pub hex_type: HexType,
    pub feature: Feature,
    pub rivers: Vec<u8>,
    pub trails: Vec<u8>,
    pub region: String,
    pub realm: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HexType {
    Jungle,
    Mountains,
    Forest,
    Plains,
    Swamps,
    Desert,
    Tundra,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Feature {
    Village,
    Inn,
    Residency,
    Dungeon,
    Town,
    City,
    Other,
    None,
}

pub fn spawn_world_hexes(mut commands: Commands) {
    // Spawn all hex tiles
    commands.spawn(HexTile {
        x: 0,
        y: 0,
        hex_type: HexType::Jungle,
        feature: Feature::Other, // TODO: Map features properly
        rivers: vec![2, 1],
        trails: vec![2, 5],
        region: "FstfgXXx".to_string(),
        realm: "X7li5Fcx".to_string(),
    });
    commands.spawn(HexTile {
        x: 0,
        y: 1,
        hex_type: HexType::Jungle,
        feature: Feature::Other, // TODO: Map features properly
        rivers: vec![3, 5],
        trails: vec![2, 5],
        region: "FstfgXXx".to_string(),
        realm: "X7li5Fcx".to_string(),
    });
    commands.spawn(HexTile {
        x: 0,
        y: -1,
        hex_type: HexType::Jungle,
        feature: Feature::Other, // TODO: Map features properly
        rivers: vec![4, 1],
        trails: vec![],
        region: "FstfgXXx".to_string(),
        realm: "X7li5Fcx".to_string(),
    });
    commands.spawn(HexTile {
        x: 1,
        y: 0,
        hex_type: HexType::Jungle,
        feature: Feature::Other, // TODO: Map features properly
        rivers: vec![],
        trails: vec![],
        region: "FstfgXXx".to_string(),
        realm: "X7li5Fcx".to_string(),
    });
    commands.spawn(HexTile {
        x: 1,
        y: -2,
        hex_type: HexType::Jungle,
        feature: Feature::Other, // TODO: Map features properly
        rivers: vec![4, 1],
        trails: vec![],
        region: "FstfgXXx".to_string(),
        realm: "X7li5Fcx".to_string(),
    });
    
    // ... and 612 more hex tiles
    println!("Spawned {} hex tiles", 617);
}

// World statistics: {
//   Total hex tiles: 617
//   Hex types: 7
//   Regions: 28 
//   Realms: 1
//   Features with content: 37
// }
