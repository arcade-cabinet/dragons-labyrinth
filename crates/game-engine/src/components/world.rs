//! World and environment components

use bevy::prelude::*;
use hexx::Hex;
use serde::{Deserialize, Serialize};

/// Hex tile component - the building block of our world
#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct HexTile {
    pub hex: Hex,
    pub terrain: TerrainType,
    pub biome: BiomeType,
    pub elevation: f32,
    pub corruption_level: f32, // 0.0 = pristine, 1.0 = fully corrupted
    pub passable: bool,
    pub visibility: TileVisibility,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum TerrainType {
    Grass,
    Forest,
    Hills,
    Mountains,
    Water,
    Swamp,
    Desert,
    Corrupted,
    Void,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum BiomeType {
    Meadow,
    DarkForest,
    Wetlands,
    Highlands,
    Wasteland,
    Village,
    Ruins,
    BossArena,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum TileVisibility {
    Hidden,      // Never seen
    Revealed,    // Seen but not currently visible
    Visible,     // Currently in view
    Hallucinated, // False vision from low sanity
}

/// Environmental conditions that affect gameplay
#[derive(Component, Clone, Debug)]
pub struct Environment {
    pub weather: Weather,
    pub time_of_day: TimeOfDay,
    pub ambient_dread: f32, // Area's base dread contribution
    pub fog_density: f32,
}

#[derive(Clone, Copy, Debug)]
pub enum Weather {
    Clear,
    Foggy,
    Raining,
    Storming,
    AshFall,    // High dread weather
    BloodRain,  // Maximum dread weather
}

#[derive(Clone, Copy, Debug)]
pub enum TimeOfDay {
    Dawn,
    Morning,
    Noon,
    Afternoon,
    Dusk,
    Night,
    WitchingHour, // Special horror time
}

/// Points of interest on the map
#[derive(Component, Clone, Debug)]
pub struct PointOfInterest {
    pub name: String,
    pub description: String,
    pub poi_type: PoiType,
    pub discovered: bool,
}

#[derive(Clone, Debug)]
pub enum PoiType {
    Village,
    Dungeon,
    Shrine,
    ForgeLocation,
    CompanionEncounter,
    BossArena,
    SecretArea,
}

/// Procedural generation seed for areas
#[derive(Component, Clone, Debug)]
pub struct MapSeed {
    pub seed: u64,
    pub biome_distribution: Vec<(BiomeType, f32)>,
    pub corruption_points: Vec<Hex>,
    pub poi_locations: Vec<(Hex, PoiType)>,
}
