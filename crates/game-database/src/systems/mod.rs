//! Game Systems Module - The Actual Game Engine
//!
//! This module contains the core game systems that are driven by database queries.
//! With 70k+ entities from HBF integration, the database IS the game engine.

use anyhow::Result;
use database_orm::*;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod hex_rendering;
pub mod combat_engine;
pub mod settlement_systems;
pub mod weather_engine;
pub mod faction_systems;
pub mod dungeon_systems;
pub mod encounter_spawning;
pub mod corruption_engine;

/// Core game systems coordinator
pub struct GameSystems {
    pub hex_renderer: hex_rendering::HexRenderingSystem,
    pub combat_engine: combat_engine::CombatEngine,
    pub settlement_systems: settlement_systems::SettlementSystems,
    pub weather_engine: weather_engine::WeatherEngine,
    pub faction_systems: faction_systems::FactionSystems,
    pub dungeon_systems: dungeon_systems::DungeonSystems,
    pub encounter_spawning: encounter_spawning::EncounterSpawning,
    pub corruption_engine: corruption_engine::CorruptionEngine,
}

impl GameSystems {
    pub async fn new(db: &DatabaseConnection) -> Result<Self> {
        Ok(Self {
            hex_renderer: hex_rendering::HexRenderingSystem::new(db).await?,
            combat_engine: combat_engine::CombatEngine::new(db).await?,
            settlement_systems: settlement_systems::SettlementSystems::new(db).await?,
            weather_engine: weather_engine::WeatherEngine::new(db).await?,
            faction_systems: faction_systems::FactionSystems::new(db).await?,
            dungeon_systems: dungeon_systems::DungeonSystems::new(db).await?,
            encounter_spawning: encounter_spawning::EncounterSpawning::new(db).await?,
            corruption_engine: corruption_engine::CorruptionEngine::new(db).await?,
        })
    }
}

/// Common types used across game systems
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HexPosition {
    pub q: i32,
    pub r: i32,
    pub s: i32,
}

impl HexPosition {
    pub fn new(q: i32, r: i32) -> Self {
        Self { q, r, s: -q - r }
    }
    
    pub fn from_hbf_coords(x: i32, y: i32) -> Self {
        Self::new(x, y)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Viewport {
    pub center: HexPosition,
    pub radius: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TileRenderData {
    pub position: HexPosition,
    pub biome_type: String,
    pub corruption_level: f32,
    pub dread_intensity: i32,
    pub texture_id: String,
    pub overlay_effects: Vec<String>,
    pub features: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatEncounter {
    pub creatures: Vec<CreatureInstance>,
    pub environment: EncounterEnvironment,
    pub tactical_map: Option<TacticalMap>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatureInstance {
    pub name: String,
    pub current_hp: i32,
    pub max_hp: i32,
    pub armor_class: i32,
    pub abilities: CreatureAbilities,
    pub actions: Vec<CreatureAction>,
    pub position: Option<HexPosition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatureAbilities {
    pub strength: i32,
    pub dexterity: i32,
    pub constitution: i32,
    pub intelligence: i32,
    pub wisdom: i32,
    pub charisma: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatureAction {
    pub name: String,
    pub description: String,
    pub attack_bonus: Option<i32>,
    pub damage_formula: Option<String>,
    pub save_dc: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncounterEnvironment {
    pub terrain: String,
    pub weather: WeatherCondition,
    pub lighting: String,
    pub hazards: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TacticalMap {
    pub grid_size: (u32, u32),
    pub terrain_features: Vec<TerrainFeature>,
    pub cover_points: Vec<HexPosition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerrainFeature {
    pub position: HexPosition,
    pub feature_type: String,
    pub blocks_movement: bool,
    pub provides_cover: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherCondition {
    pub condition: String,
    pub visibility_modifier: f32,
    pub movement_modifier: f32,
    pub combat_effects: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InnInterface {
    pub settlement_name: String,
    pub npcs: Vec<NpcData>,
    pub services: Vec<String>,
    pub current_weather: WeatherCondition,
    pub room_rate: i32,
    pub rumors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NpcData {
    pub id: Uuid,
    pub name: String,
    pub role: String,
    pub personality: String,
    pub disposition: i32,
    pub dialogue_options: Vec<String>,
    pub services_offered: Vec<String>,
    pub trade_goods: TradeData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeData {
    pub buys: Vec<String>,
    pub sells: Vec<String>,
    pub price_modifiers: std::collections::HashMap<String, f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactionData {
    pub id: Uuid,
    pub name: String,
    pub faction_type: String, // "cult", "militia", "merchant_guild", etc.
    pub influence_areas: Vec<HexPosition>,
    pub relationships: std::collections::HashMap<Uuid, i32>, // faction_id -> relationship (-10 to +10)
    pub goals: Vec<String>,
    pub resources: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DungeonLayout {
    pub id: Uuid,
    pub name: String,
    pub total_rooms: i32,
    pub current_room: Option<Uuid>,
    pub rooms: Vec<DungeonRoomData>,
    pub discovered_rooms: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DungeonRoomData {
    pub id: Uuid,
    pub area_number: i32,
    pub title: String,
    pub description: String,
    pub doorways: Vec<DoorwayData>,
    pub features: Vec<String>,
    pub encounters: Vec<String>,
    pub discovered: bool,
    pub cleared: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DoorwayData {
    pub direction: String,
    pub material: String,
    pub condition: String,
    pub locked: bool,
    pub leads_to: Option<i32>, // area number
    pub key_location: Option<String>,
}
