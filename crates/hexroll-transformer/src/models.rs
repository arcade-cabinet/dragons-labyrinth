use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PageType {
    Hex,
    Region,
    Biome,
    Settlement,  // umbrella for City/Town/Village
    City,
    Town,
    Village,
    Inn,
    Dwelling,    // umbrella for FarmsCabins/Stronghold
    FarmsCabins,
    Stronghold,
    Dungeon,     // umbrella for Cave/Temple/Tomb
    Cave,
    Temple,
    Tomb,
    NPC,
    Faction,
    Monster,
    RumorTable,
    WeatherTable,
    Shop,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HexPage {
    pub coord: String,            // "N2", "E1" ...
    pub biome: Option<String>,
    pub title: Option<String>,
    pub region: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settlement {
    pub name: String,
    pub kind: String, // City/Town/Village
    pub population: Option<i64>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Npc {
    pub name: String,
    pub description: Option<String>,
    pub level: Option<u8>,
    pub ac: Option<String>,
    pub hp: Option<String>,
    pub speed: Option<String>,
    pub abilities: Option<[i8; 6]>, // STR DEX CON INT WIS CHA
    pub senses: Vec<String>,
    pub languages: Vec<String>,
    pub actions: Vec<String>,
    pub inventory: Vec<String>,
    pub faction_names: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RumorTable {
    pub entries: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherRow {
    pub roll: String,
    pub dry: String,
    pub wet: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherTable {
    pub rows: Vec<WeatherRow>,
    pub flood_one_in_6_weekly: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageResult {
    pub uuid: String,
    pub page_type: PageType,
    pub title: Option<String>,
    pub html: Option<String>,
    pub json: Option<serde_json::Value>,

    // typed payloads (only one or a few will be set)
    pub hex: Option<HexPage>,
    pub settlement: Option<Settlement>,
    pub npc: Option<Npc>,
    pub rumor: Option<RumorTable>,
    pub weather: Option<WeatherTable>,

    // inferred categories
    pub region: Option<String>,
    pub biome: Option<String>,
    pub dungeon_kind: Option<String>,   // Cave/Temple/Tomb
    pub dwelling_kind: Option<String>,  // FarmsCabins/Stronghold
    pub faction_kind: Option<String>,   // Cult/Militia/Syndicate
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PageType {
    Hex, Region, Biome,
    Settlement, City, Town, Village,
    Inn,
    Dwelling, FarmsCabins, Stronghold,
    Dungeon, Cave, Temple, Tomb,
    NPC, Faction, Monster,
    RumorTable, WeatherTable, Shop,
    Unknown,
}