use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::Path;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use ron;

#[derive(Debug, Serialize, Deserialize)]
struct DungeonRoom {
    title: String,
    room_type: String, // Corridor, Crypt, Chamber, etc.
    doorways: Vec<Doorway>,
    description: String,
    features: Vec<String>,
    area_number: Option<u32>,
    parent_location: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Doorway {
    direction: String, // N, S, E, W
    material: String,  // wooden, iron, bronze, marble
    shape: String,     // rectangular, round
    condition: String, // Stuck, Half-broken, Barricaded
}

#[derive(Debug, Serialize, Deserialize)]
struct Settlement {
    name: String,
    settlement_type: String, // Tavern, Inn, Shop, Temple
    hex_coordinates: Option<HexCoord>,
    weather_table: Option<WeatherTable>,
    features: Vec<String>,
    region: Option<String>,
    realm: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct HexCoord {
    hex_id: String,
    x: i32,
    y: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct WeatherTable {
    dry_season: Vec<WeatherEntry>,
    wet_season: Vec<WeatherEntry>,
    flood_chance: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct WeatherEntry {
    roll_range: String,
    condition: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Patterns {
    dungeon_rooms: Vec<DungeonRoom>,
    settlements: Vec<Settlement>,
}

fn main() {
    println!("cargo:rerun-if-changed=import/");
    println!("cargo:rerun-if-changed=patterns/");
    
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let src_dir = Path::new(&out_dir).join("../../../src");
    
    // Create generated directory
    let generated_dir = src_dir.join("generated");
    fs::create_dir_all(&generated_dir).expect("Failed to create generated directory");
    
    // Extract patterns from HTML files
    let patterns = extract_patterns_from_html();
    
    // Save patterns as RON files
    save_patterns_to_ron(&patterns);
    
    // Generate Rust code
    generate_rust_code(&patterns, &generated_dir);
}

fn extract_patterns_from_html() -> Patterns {
    let entities_dir = Path::new("import/hbf_export_data/entities");
    let mut patterns = Patterns {
        dungeon_rooms: Vec::new(),
        settlements: Vec::new(),
    };
    
    if !entities_dir.exists() {
        eprintln!("Warning: import directory not found, skipping pattern extraction");
        return patterns;
    }
    
    let html_files: Vec<_> = fs::read_dir(entities_dir)
        .expect("Failed to read entities directory")
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension()?.to_str()? == "html" {
                Some(path)
            } else {
                None
            }
        })
        .collect();
    
    println!("Found {} HTML files to analyze", html_files.len());
    
    // Sample first 500 files to identify patterns
    for (i, file_path) in html_files.iter().take(500).enumerate() {
        if i % 100 == 0 {
            println!("Processing file {} of {}", i + 1, html_files.len().min(500));
        }
        
        if let Ok(content) = fs::read_to_string(file_path) {
            let document = Html::parse_document(&content);
            
            // Check if it's a dungeon room
            if let Some(room) = parse_dungeon_room(&document) {
                patterns.dungeon_rooms.push(room);
            }
            
            // Check if it's a settlement
            if let Some(settlement) = parse_settlement(&document) {
                patterns.settlements.push(settlement);
            }
        }
    }
    
    println!("Extracted {} dungeon rooms and {} settlements", 
             patterns.dungeon_rooms.len(), patterns.settlements.len());
    
    patterns
}

fn parse_dungeon_room(document: &Html) -> Option<DungeonRoom> {
    let title_selector = Selector::parse("#title span").ok()?;
    let title_element = document.select(&title_selector).next()?;
    let title = title_element.inner_html();
    
    // Check if it's a dungeon room type
    if !title.contains("Corridor") && !title.contains("Crypt") && !title.contains("Chamber") {
        return None;
    }
    
    let room_type = if title.contains("Corridor") {
        "Corridor"
    } else if title.contains("Crypt") {
        "Crypt" 
    } else if title.contains("Chamber") {
        "Chamber"
    } else {
        "Unknown"
    }.to_string();
    
    // Extract doorways
    let mut doorways = Vec::new();
    let doorway_selector = Selector::parse("h5:contains('Doorways') + ul li").ok()?;
    for doorway_element in document.select(&doorway_selector) {
        if let Some(doorway) = parse_doorway(&doorway_element.inner_html()) {
            doorways.push(doorway);
        }
    }
    
    // Extract description
    let desc_selector = Selector::parse("h5:contains('Description') + blockquote").ok()?;
    let description = document.select(&desc_selector)
        .next()
        .map(|el| el.inner_html().trim().to_string())
        .unwrap_or_default();
    
    // Extract features
    let mut features = Vec::new();
    let feature_selector = Selector::parse("ul li").ok()?;
    for feature_element in document.select(&feature_selector) {
        let text = feature_element.inner_html();
        if !text.contains("side -") { // Skip doorway entries
            features.push(text.trim().to_string());
        }
    }
    
    // Extract area number from breadcrumbs
    let breadcrumb_selector = Selector::parse(".breadcrumbs").ok()?;
    let area_number = document.select(&breadcrumb_selector)
        .next()
        .and_then(|el| {
            let text = el.inner_html();
            if let Some(area_start) = text.find("Area # ") {
                text[area_start + 7..].chars()
                    .take_while(|c| c.is_ascii_digit())
                    .collect::<String>()
                    .parse().ok()
            } else {
                None
            }
        });
    
    Some(DungeonRoom {
        title,
        room_type,
        doorways,
        description,
        features,
        area_number,
        parent_location: None,
    })
}

fn parse_doorway(doorway_text: &str) -> Option<Doorway> {
    // Parse text like: "<strong>W</strong> side - rectangular wooden door (<strong>Stuck</strong>)"
    let direction = if doorway_text.contains("<strong>N</strong>") {
        "N"
    } else if doorway_text.contains("<strong>S</strong>") {
        "S" 
    } else if doorway_text.contains("<strong>E</strong>") {
        "E"
    } else if doorway_text.contains("<strong>W</strong>") {
        "W"
    } else {
        return None;
    }.to_string();
    
    let material = if doorway_text.contains("wooden") {
        "wooden"
    } else if doorway_text.contains("iron") {
        "iron"
    } else if doorway_text.contains("bronze") {
        "bronze"
    } else if doorway_text.contains("marble") {
        "marble"
    } else {
        "unknown"
    }.to_string();
    
    let shape = if doorway_text.contains("rectangular") {
        "rectangular"
    } else if doorway_text.contains("round") {
        "round"
    } else {
        "rectangular" // default
    }.to_string();
    
    let condition = if doorway_text.contains("Stuck") {
        "Stuck"
    } else if doorway_text.contains("Half-broken") {
        "Half-broken"
    } else if doorway_text.contains("Barricaded") {
        "Barricaded"
    } else {
        "Normal"
    }.to_string();
    
    Some(Doorway {
        direction,
        material,
        shape, 
        condition,
    })
}

fn parse_settlement(document: &Html) -> Option<Settlement> {
    let title_selector = Selector::parse("#title span").ok()?;
    let title_element = document.select(&title_selector).next()?;
    let title = title_element.inner_html();
    
    // Check if it's a settlement type
    if !title.contains("Tavern") && !title.contains("Inn") && !title.contains("Shop") && !title.contains("Temple") {
        return None;
    }
    
    let settlement_type = if title.contains("Tavern") {
        "Tavern"
    } else if title.contains("Inn") {
        "Inn"
    } else if title.contains("Shop") {
        "Shop"
    } else if title.contains("Temple") {
        "Temple"
    } else {
        "Unknown"
    }.to_string();
    
    // Extract hex coordinates
    let coord_selector = Selector::parse(".map-coords").ok()?;
    let hex_coordinates = document.select(&coord_selector).next().and_then(|el| {
        let hex_id = el.value().attr("hex")?.to_string();
        let x: i32 = el.value().attr("x")?.parse().ok()?;
        let y: i32 = el.value().attr("y")?.parse().ok()?;
        Some(HexCoord { hex_id, x, y })
    });
    
    // Extract weather table if present
    let weather_table = parse_weather_table(document);
    
    // Extract features
    let mut features = Vec::new();
    let feature_selector = Selector::parse("p").ok()?;
    for feature_element in document.select(&feature_selector) {
        let text = feature_element.inner_html();
        if !text.is_empty() && !text.contains("<h5>") {
            features.push(text.trim().to_string());
        }
    }
    
    Some(Settlement {
        name: title.trim_matches('"').to_string(),
        settlement_type,
        hex_coordinates,
        weather_table,
        features,
        region: None,
        realm: None,
    })
}

fn parse_weather_table(document: &Html) -> Option<WeatherTable> {
    let table_selector = Selector::parse("table.condensed").ok()?;
    let table = document.select(&table_selector).next()?;
    
    let row_selector = Selector::parse("tr").ok()?;
    let rows: Vec<_> = table.select(&row_selector).collect();
    
    if rows.len() < 2 {
        return None; // Need header + data rows
    }
    
    let mut dry_season = Vec::new();
    let mut wet_season = Vec::new();
    
    // Skip header row
    for row in rows.iter().skip(1) {
        let cell_selector = Selector::parse("td").ok()?;
        let cells: Vec<_> = row.select(&cell_selector).map(|c| c.inner_html().trim().to_string()).collect();
        
        if cells.len() >= 3 {
            let roll_range = cells[0].clone();
            let dry_condition = cells[1].clone();
            let wet_condition = cells[2].clone();
            
            dry_season.push(WeatherEntry {
                roll_range: roll_range.clone(),
                condition: dry_condition,
            });
            
            wet_season.push(WeatherEntry {
                roll_range,
                condition: wet_condition,
            });
        }
    }
    
    // Look for flood chance note
    let flood_selector = Selector::parse("small").ok()?;
    let flood_chance = document.select(&flood_selector)
        .find(|el| el.inner_html().contains("flood"))
        .map(|el| el.inner_html());
    
    Some(WeatherTable {
        dry_season,
        wet_season,
        flood_chance,
    })
}

fn save_patterns_to_ron(patterns: &Patterns) {
    let patterns_dir = Path::new("patterns");
    fs::create_dir_all(patterns_dir).expect("Failed to create patterns directory");
    
    // Save dungeon rooms
    let dungeon_ron = ron::ser::to_string_pretty(&patterns.dungeon_rooms, ron::ser::PrettyConfig::default())
        .expect("Failed to serialize dungeon rooms");
    fs::write(patterns_dir.join("dungeon_rooms.ron"), dungeon_ron)
        .expect("Failed to write dungeon_rooms.ron");
    
    // Save settlements
    let settlement_ron = ron::ser::to_string_pretty(&patterns.settlements, ron::ser::PrettyConfig::default())
        .expect("Failed to serialize settlements");
    fs::write(patterns_dir.join("settlements.ron"), settlement_ron)
        .expect("Failed to write settlements.ron");
    
    println!("Saved patterns to RON files");
}

fn generate_rust_code(patterns: &Patterns, generated_dir: &Path) {
    // Generate ECS components
    let mut components_file = fs::File::create(generated_dir.join("components.rs"))
        .expect("Failed to create components.rs");
    
    write!(components_file, r#"
// Auto-generated ECS components from HBF patterns
use bevy::prelude::*;
use serde::{{Deserialize, Serialize}};

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct DungeonRoom {{
    pub title: String,
    pub room_type: RoomType,
    pub doorways: Vec<Doorway>,
    pub description: String,
    pub features: Vec<String>,
    pub area_number: Option<u32>,
    pub parent_location: Option<String>,
}}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RoomType {{
    Corridor,
    Crypt,
    Chamber,
    Unknown,
}}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Doorway {{
    pub direction: Direction,
    pub material: DoorMaterial,
    pub shape: DoorShape,
    pub condition: DoorCondition,
}}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Direction {{
    North,
    South,
    East, 
    West,
}}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DoorMaterial {{
    Wooden,
    Iron,
    Bronze,
    Marble,
    Unknown,
}}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DoorShape {{
    Rectangular,
    Round,
}}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DoorCondition {{
    Normal,
    Stuck,
    HalfBroken,
    Barricaded,
}}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Settlement {{
    pub name: String,
    pub settlement_type: SettlementType,
    pub hex_coordinates: Option<HexCoord>,
    pub weather_table: Option<WeatherTable>,
    pub features: Vec<String>,
    pub region: Option<String>,
    pub realm: Option<String>,
}}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SettlementType {{
    Tavern,
    Inn,
    Shop,
    Temple,
    Unknown,
}}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HexCoord {{
    pub hex_id: String,
    pub x: i32,
    pub y: i32,
}}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherTable {{
    pub dry_season: Vec<WeatherEntry>,
    pub wet_season: Vec<WeatherEntry>,
    pub flood_chance: Option<String>,
}}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherEntry {{
    pub roll_range: String,
    pub condition: String,
}}

// Statistics
pub const EXTRACTED_DUNGEON_ROOMS: usize = {};
pub const EXTRACTED_SETTLEMENTS: usize = {};
"#, patterns.dungeon_rooms.len(), patterns.settlements.len())
        .expect("Failed to write components.rs");
    
    // Generate systems
    let mut systems_file = fs::File::create(generated_dir.join("systems.rs"))
        .expect("Failed to create systems.rs");
    
    write!(systems_file, r#"
// Auto-generated systems from HBF patterns  
use bevy::prelude::*;
use super::components::*;

pub struct HbfParserPlugin;

impl Plugin for HbfParserPlugin {{
    fn build(&self, app: &mut App) {{
        app
            .add_systems(Startup, load_hbf_data)
            .add_systems(Update, (
                dungeon_room_system,
                settlement_system,
            ));
    }}
}}

fn load_hbf_data(mut commands: Commands) {{
    info!("Loading HBF data with {{}} rooms and {{}} settlements", 
          EXTRACTED_DUNGEON_ROOMS, EXTRACTED_SETTLEMENTS);
    
    // TODO: Load actual data from RON files
    // This would be implemented to spawn entities from the extracted patterns
}}

fn dungeon_room_system(query: Query<&DungeonRoom>) {{
    // System for processing dungeon rooms
    for room in query.iter() {{
        // Room logic here
    }}
}}

fn settlement_system(query: Query<&Settlement>) {{
    // System for processing settlements
    for settlement in query.iter() {{
        // Settlement logic here  
    }}
}}
"#).expect("Failed to write systems.rs");
    
    // Generate mod.rs
    let mut mod_file = fs::File::create(generated_dir.join("mod.rs"))
        .expect("Failed to create mod.rs");
    
    write!(mod_file, r#"
//! Auto-generated HBF parser components and systems
//! 
//! This module contains ECS components and systems generated from
//! parsing the HBF export HTML files.

pub mod components;
pub mod systems;

pub use components::*;
pub use systems::HbfParserPlugin;

/// Re-export commonly used types
pub mod prelude {{
    pub use super::components::{{DungeonRoom, Settlement, RoomType, SettlementType}};
    pub use super::systems::HbfParserPlugin;
}}
"#).expect("Failed to write mod.rs");
    
    println!("Generated Rust code in src/generated/");
}
