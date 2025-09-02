use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct WorldBook { pub plan: Plan, pub regions: Vec<Region> }

#[derive(Debug, Clone, Deserialize)]
pub struct Plan { pub title: String, pub starting_hex: String, pub region_bands: Vec<RegionBand>, pub global_pillars: Vec<String> }

#[derive(Debug, Clone, Deserialize)]
pub struct RegionBand { pub band: String, pub name: String, pub theme_summary: String, pub biome_palette: Vec<String>, pub tone: String }

#[derive(Debug, Clone, Deserialize)]
pub struct Region {
    pub band: String,
    pub name: String,
    pub mood_board: Vec<String>,
    pub biomes: Vec<String>,
    pub hex_points: Vec<HexPOI>,
    pub quests: Vec<Quest>,
    pub npcs: Vec<NpcData>,
    pub creatures: Vec<CreatureData>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HexPOI { pub axial: String, pub kind: String, pub blurb: String }

#[derive(Debug, Clone, Deserialize)]
pub struct Quest {
    pub id: String, pub title: String, pub summary: String, pub r#type: String,
    pub steps: Vec<String>, pub success_outcome: String, pub failure_outcome: String
}

#[derive(Debug, Clone, Deserialize)]
pub struct NpcData { pub id: String, pub name: String, pub role: String }

#[derive(Debug, Clone, Deserialize)]
pub struct CreatureData { pub id: String, pub name: String, pub tags: Vec<String>, pub cr_hint: String }

