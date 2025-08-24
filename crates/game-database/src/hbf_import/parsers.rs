//! HTML parsers for different HBF entity types
//! 
//! This module contains specialized parsers for the various entity types found in HBF data,
//! including full D&D 5e stat blocks, city GeoJSON data, encounter tables, and weather systems.

use anyhow::{Context, Result};
use scraper::{Html, Selector};
use serde_json::Value;
use std::collections::HashMap;
use tracing::{debug, warn};
use super::database::{HbfRef};

/// All parsed entities from HBF HTML content
#[derive(Debug, Default)]
pub struct ParsedEntities {
    pub hex_features: Vec<ParsedHexFeature>,
    pub settlements: Vec<ParsedSettlement>,
    pub cities: Vec<ParsedCity>,
    pub dungeon_rooms: Vec<ParsedDungeonRoom>,
    pub creatures: Vec<ParsedCreature>,
    pub encounter_tables: Vec<ParsedEncounterTable>,
    pub weather_systems: Vec<ParsedWeatherSystem>,
}

#[derive(Debug, Clone)]
pub struct ParsedHexFeature {
    pub hbf_uuid: String,
    pub name: String,
    pub feature_type: String, // "watchtower", "ruins", "bridge", etc.
    pub coordinates: Option<(i32, i32)>,
    pub description: String,
    pub encounter_table: Option<ParsedEncounterTable>,
    pub weather_system: Option<ParsedWeatherSystem>,
    pub special_features: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ParsedSettlement {
    pub hbf_uuid: String,
    pub name: String,
    pub settlement_type: String, // "tavern", "inn", "shop", "temple"
    pub coordinates: Option<(i32, i32)>,
    pub description: String,
    pub weather_system: Option<ParsedWeatherSystem>,
    pub services: Vec<String>,
    pub features: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ParsedCity {
    pub hbf_uuid: String,
    pub name: String,
    pub coordinates: Option<(i32, i32)>,
    pub geojson_data: Value, // Raw GeoJSON for city layout
    pub buildings: Vec<CityBuilding>,
    pub roads: Vec<CityRoad>,
    pub points_of_interest: Vec<CityPoi>,
}

#[derive(Debug, Clone)]
pub struct CityBuilding {
    pub coords: Vec<(f64, f64)>, // Building polygon coordinates
    pub uuid: Option<String>,    // Link to detailed building data
}

#[derive(Debug, Clone)]
pub struct CityRoad {
    pub coords: Vec<(f64, f64)>, // Road line coordinates  
    pub width: f64,
}

#[derive(Debug, Clone)]
pub struct CityPoi {
    pub coords: (f64, f64),
    pub title: String,
    pub uuid: String,
    pub poi_type: String, // Derived from title: "blacksmith", "witch", "music", etc.
}

#[derive(Debug, Clone)]
pub struct ParsedDungeonRoom {
    pub hbf_uuid: String,
    pub title: String,
    pub room_type: String, // "corridor", "chamber", "crypt", etc.
    pub area_number: Option<i32>,
    pub parent_dungeon: Option<String>,
    pub description: String,
    pub doorways: Vec<ParsedDoorway>,
    pub features: Vec<String>,
    pub wandering_monsters: Option<ParsedEncounterTable>,
    pub coordinates: Option<(i32, i32)>,
}

#[derive(Debug, Clone)]
pub struct ParsedDoorway {
    pub direction: String,    // "north", "south", "east", "west"
    pub material: String,     // "wooden", "iron", "bronze", "marble"
    pub shape: String,        // "rectangular", "round", "arched"  
    pub condition: String,    // "normal", "stuck", "broken", "barricaded", "locked"
    pub locked: bool,
    pub key_location: Option<String>, // Reference to area with key
    pub magical: bool,
}

#[derive(Debug, Clone)]
pub struct ParsedCreature {
    pub hbf_uuid: String,
    pub name: String,
    pub creature_type: String, // "undead", "beast", "humanoid", etc.
    pub challenge_rating: String,
    pub armor_class: i32,
    pub hit_points: String, // Formula like "9d8 + 18"
    pub speed: HashMap<String, i32>, // "walk": 30, "fly": 50, etc.
    pub ability_scores: CreatureAbilities,
    pub saving_throws: HashMap<String, i32>,
    pub skills: HashMap<String, i32>,
    pub damage_resistances: Vec<String>,
    pub damage_immunities: Vec<String>,
    pub condition_immunities: Vec<String>,
    pub senses: Vec<String>,
    pub languages: Vec<String>,
    pub special_abilities: Vec<SpecialAbility>,
    pub actions: Vec<CreatureAction>,
    pub alignment: String,
}

#[derive(Debug, Clone)]
pub struct CreatureAbilities {
    pub strength: i32,
    pub dexterity: i32,
    pub constitution: i32,
    pub intelligence: i32,
    pub wisdom: i32,
    pub charisma: i32,
}

#[derive(Debug, Clone)]
pub struct SpecialAbility {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct CreatureAction {
    pub name: String,
    pub description: String,
    pub attack_bonus: Option<i32>,
    pub damage_formula: Option<String>,
    pub damage_type: Option<String>,
    pub range: Option<String>,
    pub save_dc: Option<i32>,
    pub save_ability: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ParsedEncounterTable {
    pub title: String,
    pub probability: String, // "1 in 6", "2 in 6", etc.
    pub encounters: Vec<EncounterEntry>,
}

#[derive(Debug, Clone)]
pub struct EncounterEntry {
    pub roll: String,        // "1", "2", "3-4", etc.
    pub creature_name: String,
    pub quantity: Option<String>, // "(7)", "(5)", etc.
    pub creature_data: Option<ParsedCreature>,
}

#[derive(Debug, Clone)]
pub struct ParsedWeatherSystem {
    pub title: String,
    pub seasons: Vec<WeatherSeason>,
    pub special_effects: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct WeatherSeason {
    pub name: String, // "Warm Season", "Dry Season", "Wet Season", "Cold Season"
    pub entries: Vec<WeatherEntry>,
}

#[derive(Debug, Clone)]
pub struct WeatherEntry {
    pub roll: String,    // "2", "3-4", "5-9", etc.
    pub condition: String, // "Clear", "Rainy", "Stormy", etc.
}

/// Parse all HTML entities into structured data
pub async fn parse_all_entities(
    entities: &HashMap<String, String>,
    refs: &[HbfRef],
) -> Result<ParsedEntities> {
    let mut parsed = ParsedEntities::default();
    
    let refs_by_uuid: HashMap<String, &HbfRef> = refs
        .iter()
        .map(|r| (r.uuid.clone(), r))
        .collect();
    
    debug!("Parsing {} HTML entities", entities.len());
    
    for (uuid, html) in entities {
        let document = Html::parse_document(html);
        
        // Try to determine entity type and parse accordingly
        if is_city_data(html) {
            if let Ok(city) = parse_city_data(uuid, html) {
                parsed.cities.push(city);
            }
        } else if is_dungeon_room(&document) {
            if let Ok(room) = parse_dungeon_room(uuid, &document) {
                parsed.dungeon_rooms.push(room);
            }
        } else if is_hex_feature(&document) {
            if let Ok(feature) = parse_hex_feature(uuid, &document) {
                parsed.hex_features.push(feature);
            }
        } else if is_settlement(&document) {
            if let Ok(settlement) = parse_settlement(uuid, &document) {
                parsed.settlements.push(settlement);
            }
        }
        
        // Extract monster stat blocks regardless of entity type
        let creatures = parse_creatures_from_html(&document);
        parsed.creatures.extend(creatures);
        
        // Extract encounter tables
        if let Some(encounter_table) = parse_encounter_table(&document) {
            parsed.encounter_tables.push(encounter_table);
        }
        
        // Extract weather systems
        if let Some(weather) = parse_weather_system(&document) {
            parsed.weather_systems.push(weather);
        }
    }
    
    debug!("Parsed {} cities, {} hex features, {} settlements, {} dungeon rooms, {} creatures", 
           parsed.cities.len(), parsed.hex_features.len(), parsed.settlements.len(), 
           parsed.dungeon_rooms.len(), parsed.creatures.len());
    
    Ok(parsed)
}

/// Check if content is GeoJSON city data
fn is_city_data(html: &str) -> bool {
    html.trim().starts_with('{') && html.contains("\"map_data\"") && html.contains("\"FeatureCollection\"")
}

/// Parse GeoJSON city data
fn parse_city_data(uuid: &str, json_str: &str) -> Result<ParsedCity> {
    let city_data: Value = serde_json::from_str(json_str)
        .context("Failed to parse city JSON")?;
    
    let map_data = &city_data["map_data"];
    let poi_data = city_data["poi"].as_array().unwrap_or(&vec![]);
    
    let mut buildings = Vec::new();
    let mut roads = Vec::new();
    let mut points_of_interest = Vec::new();
    
    // Parse buildings
    if let Some(features) = map_data["features"].as_array() {
        for feature in features {
            if feature["id"] == "buildings" {
                if let Some(coords) = feature["coordinates"].as_array() {
                    for building_coords in coords {
                        if let Some(polygon) = building_coords.as_array() {
                            if let Some(coords_array) = polygon.get(0) {
                                if let Some(coords_array) = coords_array.as_array() {
                                    let coords: Vec<(f64, f64)> = coords_array
                                        .iter()
                                        .filter_map(|coord| {
                                            if let Some(arr) = coord.as_array() {
                                                if arr.len() >= 2 {
                                                    let x = arr[0].as_f64()?;
                                                    let y = arr[1].as_f64()?;
                                                    Some((x, y))
                                                } else {
                                                    None
                                                }
                                            } else {
                                                None
                                            }
                                        })
                                        .collect();
                                    
                                    if !coords.is_empty() {
                                        let building_uuid = if polygon.len() > 1 {
                                            polygon[1].as_str().map(|s| s.to_string())
                                        } else {
                                            None
                                        };
                                        
                                        buildings.push(CityBuilding {
                                            coords,
                                            uuid: building_uuid,
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            } else if feature["id"] == "roads" {
                if let Some(geometries) = feature["geometries"].as_array() {
                    for road in geometries {
                        if let Some(coords) = road["coordinates"].as_array() {
                            let road_coords: Vec<(f64, f64)> = coords
                                .iter()
                                .filter_map(|coord| {
                                    if let Some(arr) = coord.as_array() {
                                        if arr.len() >= 2 {
                                            let x = arr[0].as_f64()?;
                                            let y = arr[1].as_f64()?;
                                            Some((x, y))
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                })
                                .collect();
                            
                            let width = road["width"].as_f64().unwrap_or(1.0);
                            
                            roads.push(CityRoad {
                                coords: road_coords,
                                width,
                            });
                        }
                    }
                }
            }
        }
    }
    
    // Parse POIs
    for poi in poi_data {
        if let (Some(coords), Some(title), Some(poi_uuid)) = (
            poi["coords"].as_object(),
            poi["title"].as_str(),
            poi["uuid"].as_str(),
        ) {
            if let (Some(x), Some(y)) = (coords["x"].as_f64(), coords["y"].as_f64()) {
                let poi_type = determine_poi_type(title);
                points_of_interest.push(CityPoi {
                    coords: (x, y),
                    title: title.to_string(),
                    uuid: poi_uuid.to_string(),
                    poi_type,
                });
            }
        }
    }
    
    Ok(ParsedCity {
        hbf_uuid: uuid.to_string(),
        name: "Generated City".to_string(), // Would need better name extraction
        coordinates: None,
        geojson_data: city_data,
        buildings,
        roads,
        points_of_interest,
    })
}

fn determine_poi_type(title: &str) -> String {
    let title_lower = title.to_lowercase();
    if title_lower.contains("blacksmith") {
        "blacksmith"
    } else if title_lower.contains("shop") {
        "shop"
    } else if title_lower.contains("tavern") || title_lower.contains("lodge") {
        "tavern"
    } else if title_lower.contains("witch") {
        "witch"
    } else if title_lower.contains("market") {
        "market"
    } else if title_lower.contains("tailor") {
        "tailor"
    } else if title_lower.contains("herbalist") {
        "herbalist"
    } else if title_lower.contains("veterinarian") {
        "veterinarian"
    } else {
        "unknown"
    }.to_string()
}

/// Check if HTML represents a hex feature (watchtower, ruins, etc.)
fn is_hex_feature(document: &Html) -> bool {
    let title_selector = Selector::parse("#title span").unwrap();
    if let Some(title_element) = document.select(&title_selector).next() {
        let title = title_element.inner_html().to_lowercase();
        title.contains("watchtower") || title.contains("ruins") || title.contains("bridge") ||
        title.contains("tower") || title.contains("shrine")
    } else {
        false
    }
}

/// Check if HTML represents a settlement
fn is_settlement(document: &Html) -> bool {
    let title_selector = Selector::parse("#title span").unwrap();
    if let Some(title_element) = document.select(&title_selector).next() {
        let title = title_element.inner_html().to_lowercase();
        title.contains("tavern") || title.contains("inn") || title.contains("shop") || 
        title.contains("temple") || title.contains("market")
    } else {
        false
    }
}

/// Check if HTML represents a dungeon room
fn is_dungeon_room(document: &Html) -> bool {
    let content = document.html().to_lowercase();
    content.contains("area #") || content.contains("doorways") || 
    content.contains("corridor") || content.contains("chamber") || content.contains("crypt")
}

/// Parse hex feature (watchtower, ruins, etc.)
fn parse_hex_feature(uuid: &str, document: &Html) -> Result<ParsedHexFeature> {
    let title_selector = Selector::parse("#title span").unwrap();
    let name = document
        .select(&title_selector)
        .next()
        .map(|el| el.inner_html().trim().to_string())
        .unwrap_or_else(|| "Unknown Feature".to_string());
    
    let feature_type = if name.to_lowercase().contains("watchtower") {
        "watchtower"
    } else if name.to_lowercase().contains("ruins") {
        "ruins"
    } else if name.to_lowercase().contains("bridge") {
        "bridge"
    } else if name.to_lowercase().contains("tower") {
        "tower"
    } else if name.to_lowercase().contains("shrine") {
        "shrine"
    } else {
        "feature"
    }.to_string();
    
    let coordinates = extract_coordinates(document);
    let description = extract_description(document);
    let encounter_table = parse_encounter_table(document);
    let weather_system = parse_weather_system(document);
    let special_features = extract_features(document);
    
    Ok(ParsedHexFeature {
        hbf_uuid: uuid.to_string(),
        name,
        feature_type,
        coordinates,
        description,
        encounter_table,
        weather_system,
        special_features,
    })
}

/// Parse settlement (tavern, inn, shop, etc.)
fn parse_settlement(uuid: &str, document: &Html) -> Result<ParsedSettlement> {
    let title_selector = Selector::parse("#title span").unwrap();
    let name = document
        .select(&title_selector)
        .next()
        .map(|el| el.inner_html().trim_matches('"').to_string())
        .unwrap_or_else(|| "Unknown Settlement".to_string());
    
    let settlement_type = if name.to_lowercase().contains("tavern") {
        "tavern"
    } else if name.to_lowercase().contains("inn") {
        "inn"
    } else if name.to_lowercase().contains("shop") {
        "shop"
    } else if name.to_lowercase().contains("temple") {
        "temple"
    } else if name.to_lowercase().contains("market") {
        "market"
    } else {
        "settlement"
    }.to_string();
    
    let coordinates = extract_coordinates(document);
    let description = extract_description(document);
    let weather_system = parse_weather_system(document);
    let services = extract_services(document);
    let features = extract_features(document);
    
    Ok(ParsedSettlement {
        hbf_uuid: uuid.to_string(),
        name,
        settlement_type,
        coordinates,
        description,
        weather_system,
        services,
        features,
    })
}

/// Parse dungeon room with full detail
fn parse_dungeon_room(uuid: &str, document: &Html) -> Result<ParsedDungeonRoom> {
    let title_selector = Selector::parse("#title span").unwrap();
    let title = document
        .select(&title_selector)
        .next()
        .map(|el| el.inner_html().trim().to_string())
        .unwrap_or_else(|| "Unknown Room".to_string());
    
    let room_type = if title.to_lowercase().contains("corridor") {
        "corridor"
    } else if title.to_lowercase().contains("chamber") {
        "chamber"
    } else if title.to_lowercase().contains("crypt") {
        "crypt"
    } else if title.to_lowercase().contains("hall") {
        "hall"
    } else if title.to_lowercase().contains("vault") {
        "vault"
    } else {
        "room"
    }.to_string();
    
    let area_number = extract_area_number(document);
    let parent_dungeon = extract_parent_dungeon(document);
    let coordinates = extract_coordinates(document);
    let description = extract_description(document);
    let doorways = parse_doorways(document);
    let features = extract_features(document);
    let wandering_monsters = parse_encounter_table(document);
    
    Ok(ParsedDungeonRoom {
        hbf_uuid: uuid.to_string(),
        title,
        room_type,
        area_number,
        parent_dungeon,
        description,
        doorways,
        features,
        wandering_monsters,
        coordinates,
    })
}

/// Extract coordinates from map-coords element
fn extract_coordinates(document: &Html) -> Option<(i32, i32)> {
    let coord_selector = Selector::parse(".map-coords").unwrap();
    document.select(&coord_selector).next().and_then(|el| {
        let x: i32 = el.value().attr("x")?.parse().ok()?;
        let y: i32 = el.value().attr("y")?.parse().ok()?;
        Some((x, y))
    })
}

/// Extract area number from breadcrumbs
fn extract_area_number(document: &Html) -> Option<i32> {
    let breadcrumb_selector = Selector::parse(".breadcrumbs").unwrap();
    document.select(&breadcrumb_selector).next().and_then(|el| {
        let text = el.inner_html();
        if let Some(area_start) = text.find("Area # ") {
            text[area_start + 7..].chars()
                .take_while(|c| c.is_ascii_digit())
                .collect::<String>()
                .parse().ok()
        } else {
            None
        }
    })
}

/// Extract parent dungeon name from breadcrumbs
fn extract_parent_dungeon(document: &Html) -> Option<String> {
    let breadcrumb_selector = Selector::parse(".breadcrumbs a[href*='/location/']").unwrap();
    document.select(&breadcrumb_selector)
        .last()
        .map(|el| el.inner_html().trim().to_string())
}

/// Extract main description from HTML
fn extract_description(document: &Html) -> String {
    let desc_selector = Selector::parse("blockquote, p").unwrap();
    document
        .select(&desc_selector)
        .map(|el| el.inner_html().trim())
        .filter(|s| !s.is_empty() && !s.contains("<h5>") && !s.contains("class='condensed'"))
        .collect::<Vec<_>>()
        .join(" ")
}

/// Extract general features from HTML
fn extract_features(document: &Html) -> Vec<String> {
    let feature_selector = Selector::parse("ul li").unwrap();
    document
        .select(&feature_selector)
        .map(|el| el.inner_html().trim().to_string())
        .filter(|s| {
            !s.is_empty() && 
            !s.contains("side -") && // Skip doorway entries
            !s.contains("btn-spawn-dice") && // Skip dice roll entries
            !s.contains("statblock") // Skip stat block entries
        })
        .collect()
}

/// Extract services from settlement HTML
fn extract_services(document: &Html) -> Vec<String> {
    // For now, return empty - would need more analysis of settlement patterns
    Vec::new()
}

/// Parse doorways with full details including locks and keys
fn parse_doorways(document: &Html) -> Vec<ParsedDoorway> {
    let doorway_selector = Selector::parse("h5:contains('Doorways') + ul li").unwrap();
    
    document
        .select(&doorway_selector)
        .filter_map(|el| parse_single_doorway(&el.inner_html()))
        .collect()
}

fn parse_single_doorway(doorway_html: &str) -> Option<ParsedDoorway> {
    // Parse complex doorway text with locking and key information
    let direction = if doorway_html.contains("<strong>N</strong>") {
        "north"
    } else if doorway_html.contains("<strong>S</strong>") {
        "south"
    } else if doorway_html.contains("<strong>E</strong>") {
        "east"
    } else if doorway_html.contains("<strong>W</strong>") {
        "west"
    } else {
        return None;
    }.to_string();
    
    let material = if doorway_html.contains("wooden") {
        "wood"
    } else if doorway_html.contains("iron") {
        "iron"
    } else if doorway_html.contains("bronze") {
        "bronze"
    } else if doorway_html.contains("marble") {
        "marble"
    } else if doorway_html.contains("stone") {
        "stone"
    } else {
        "unknown"
    }.to_string();
    
    let shape = if doorway_html.contains("arched") {
        "arched"
    } else if doorway_html.contains("rectangular") {
        "rectangular"
    } else if doorway_html.contains("round") {
        "round"
    } else {
        "rectangular"
    }.to_string();
    
    let locked = doorway_html.contains("Locked");
    let magical = doorway_html.contains("&#128863;"); // Magic symbol
    
    let condition = if doorway_html.contains("Stuck") {
        "stuck"
    } else if doorway_html.contains("Half-broken") || doorway_html.contains("Broken") {
        "broken"
    } else if doorway_html.contains("Barricaded") {
        "barricaded"
    } else if locked {
        "locked"
    } else {
        "normal"
    }.to_string();
    
    // Extract key location if present
    let key_location = if doorway_html.contains("The key is in") {
        // Parse "area 13" type references
        if let Some(area_start) = doorway_html.find("area ") {
            let area_text = &doorway_html[area_start + 5..];
            area_text.chars()
                .take_while(|c| c.is_ascii_digit())
                .collect::<String>()
                .parse::<i32>()
                .ok()
                .map(|n| format!("area_{}", n))
        } else {
            None
        }
    } else {
        None
    };
    
    Some(ParsedDoorway {
        direction,
        material,
        shape,
        condition,
        locked,
        key_location,
        magical,
    })
}

/// Parse encounter tables with full creature data
fn parse_encounter_table(document: &Html) -> Option<ParsedEncounterTable> {
    // Look for encounter table headers
    let encounter_header_selector = Selector::parse("h5:contains('encounter'), h5:contains('Monsters')").unwrap();
    let encounter_header = document.select(&encounter_header_selector).next()?;
    
    let title = encounter_header.inner_html().trim().to_string();
    
    // Extract probability text
    let probability = extract_encounter_probability(document);
    
    // Parse encounter table rows
    let table_selector = Selector::parse("table").unwrap();
    let table = document.select(&table_selector).next()?;
    
    let row_selector = Selector::parse("tr").unwrap();
    let rows: Vec<_> = table.select(&row_selector).collect();
    
    let mut encounters = Vec::new();
    
    // Skip header row
    for row in rows.iter().skip(1) {
        if let Some(encounter) = parse_encounter_row(row) {
            encounters.push(encounter);
        }
    }
    
    if encounters.is_empty() {
        return None;
    }
    
    Some(ParsedEncounterTable {
        title,
        probability,
        encounters,
    })
}

fn extract_encounter_probability(document: &Html) -> String {
    let prob_selector = Selector::parse("p:contains('chance')").unwrap();
    if let Some(prob_element) = document.select(&prob_selector).next() {
        let text = prob_element.inner_html();
        if text.contains("1 in 6") {
            "1_in_6"
        } else if text.contains("2 in 6") {
            "2_in_6"
        } else if text.contains("1d6") {
            "1d6"
        } else {
            "unknown"
        }.to_string()
    } else {
        "unknown".to_string()
    }
}

fn parse_encounter_row(row: &scraper::ElementRef) -> Option<EncounterEntry> {
    let cell_selector = Selector::parse("td").unwrap();
    let cells: Vec<_> = row.select(&cell_selector).collect();
    
    if cells.len() >= 2 {
        let roll = cells[0].inner_html().trim().to_string();
        let creature_text = cells[1].inner_html().trim().to_string();
        
        // Extract creature name and quantity
        let (creature_name, quantity) = parse_creature_name_and_quantity(&creature_text);
        
        // Extract creature data from stat block if present
        let creature_data = if cells.len() > 2 {
            parse_creature_from_cell(&cells[2])
        } else {
            None
        };
        
        Some(EncounterEntry {
            roll,
            creature_name,
            quantity,
            creature_data,
        })
    } else {
        None
    }
}

fn parse_creature_name_and_quantity(text: &str) -> (String, Option<String>) {
    // Parse text like "Blood Hawks (7)" or "Ettercap"
    if let Some(paren_start) = text.find('(') {
        let name = text[..paren_start].trim().to_string();
        let quantity_text = &text[paren_start..];
        if let Some(paren_end) = quantity_text.find(')') {
            let quantity = quantity_text[1..paren_end].trim().to_string();
            (name, Some(quantity))
        } else {
            (text.trim().to_string(), None)
        }
    } else {
        (text.trim().to_string(), None)
    }
}

/// Parse creatures from stat blocks in HTML
fn parse_creatures_from_html(document: &Html) -> Vec<ParsedCreature> {
    let statblock_selector = Selector::parse(".statblock").unwrap();
    document
        .select(&statblock_selector)
        .filter_map(|block| parse_single_creature(block))
        .collect()
}

fn parse_single_creature(statblock: scraper::ElementRef) -> Option<ParsedCreature> {
    // Extract basic info from statblock-top-row
    let top_row_selector = Selector::parse(".statblock-top-row div").unwrap();
    let top_row_divs: Vec<_> = statblock.select(&top_row_selector).collect();
    
    let mut challenge_rating = "0".to_string();
    let mut armor_class = 10;
    let mut hit_points = "1d4".to_string();
    let mut speed = HashMap::new();
    
    for div in top_row_divs {
        let text = div.inner_html();
        if text.contains("CR:") {
            if let Some(cr_start) = text.find("CR:</span> ") {
                challenge_rating = text[cr_start + 11..].split_whitespace().next().unwrap_or("0").to_string();
            }
        } else if text.contains("AC:") {
            if let Some(ac_start) = text.find("AC:</span> ") {
                if let Ok(ac) = text[ac_start + 11..].split_whitespace().next().unwrap_or("10").parse() {
                    armor_class = ac;
                }
            }
        } else if text.contains("HP:") {
            if let Some(hp_start) = text.find("HP:</span>  (") {
                if let Some(hp_end) = text[hp_start + 13..].find(')') {
                    hit_points = text[hp_start + 13..hp_start + 13 + hp_end].to_string();
                }
            }
        } else if text.contains("Speed:") {
            speed = parse_speed_text(&text);
        }
    }
    
    // Parse ability scores
    let ability_scores = parse_ability_scores(&statblock);
    
    // Parse special abilities
    let special_abilities = parse_special_abilities(&statblock);
    
    // Parse actions
    let actions = parse_creature_actions(&statblock);
    
    // Extract other attributes
    let alignment = extract_alignment(&statblock);
    let languages = extract_languages(&statblock);
    let senses = extract_senses(&statblock);
    let damage_immunities = extract_damage_immunities(&statblock);
    
    Some(ParsedCreature {
        hbf_uuid: "".to_string(), // Will be set by caller
        name: "Unknown".to_string(), // Will be extracted from encounter context
        creature_type: "unknown".to_string(),
        challenge_rating,
        armor_class,
        hit_points,
        speed,
        ability_scores,
        saving_throws: HashMap::new(),
        skills: HashMap::new(),
        damage_resistances: Vec::new(),
        damage_immunities,
        condition_immunities: Vec::new(),
        senses,
        languages,
        special_abilities,
        actions,
        alignment,
    })
}

fn parse_creature_from_cell(cell: &scraper::ElementRef) -> Option<ParsedCreature> {
    let statblock_selector = Selector::parse(".statblock").unwrap();
    if let Some(statblock) = cell.select(&statblock_selector).next() {
        parse_single_creature(statblock)
    } else {
        None
    }
}

fn parse_speed_text(text: &str) -> HashMap<String, i32> {
    let mut speed = HashMap::new();
    
    if let Some(speed_start) = text.find("Speed:</span> ") {
        let speed_text = &text[speed_start + 14..];
        
        // Parse "Walk 30 ft. Fly 50 ft. (hover)" etc.
        for part in speed_text.split('.') {
            let part = part.trim();
            if part.contains("Walk") {
                if let Some(num) = extract_number_from_text(part) {
                    speed.insert("walk".to_string(), num);
                }
            } else if part.contains("Fly") {
                if let Some(num) = extract_number_from_text(part) {
                    speed.insert("fly".to_string(), num);
                }
            } else if part.contains("Swim") {
                if let Some(num) = extract_number_from_text(part) {
                    speed.insert("swim".to_string(), num);
                }
            } else if part.contains("Climb") {
                if let Some(num) = extract_number_from_text(part) {
                    speed.insert("climb".to_string(), num);
                }
            }
        }
    }
    
    speed
}

fn extract_number_from_text(text: &str) -> Option<i32> {
    text.chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse()
        .ok()
}

fn parse_ability_scores(statblock: &scraper::ElementRef) -> CreatureAbilities {
    let table_selector = Selector::parse(".statblock-table tr").unwrap();
    let rows: Vec<_> = statblock.select(&table_selector).collect();
    
    if rows.len() >= 2 {
        let score_row = &rows[1];
        let cell_selector = Selector::parse("td").unwrap();
        let cells: Vec<_> = score_row.select(&cell_selector).collect();
        
        if cells.len() >= 6 {
            return CreatureAbilities {
                strength: extract_ability_score(&cells[0]),
                dexterity: extract_ability_score(&cells[1]),
                constitution: extract_ability_score(&cells[2]),
                intelligence: extract_ability_score(&cells[3]),
                wisdom: extract_ability_score(&cells[4]),
                charisma: extract_ability_score(&cells[5]),
            };
        }
    }
    
    // Default values
    CreatureAbilities {
        strength: 10,
        dexterity: 10,
        constitution: 10,
        intelligence: 10,
        wisdom: 10,
        charisma: 10,
    }
}

fn extract_ability_score(cell: &scraper::ElementRef) -> i32 {
    let text = cell.inner_html();
    text.split_whitespace()
        .next()
        .and_then(|s| s.parse().ok())
        .unwrap_or(10)
}

fn parse_special_abilities(statblock: &scraper::ElementRef) -> Vec<SpecialAbility> {
    // Parse abilities from the section between ability scores and actions
    let mut abilities = Vec::new();
    
    let content_selector = Selector::parse(".statblock-container ul li").unwrap();
    for li in statblock.select(&content_selector) {
        let text = li.inner_html();
        if let Some(colon_pos) = text.find(':') {
            let name = text[..colon_pos].trim_start_matches("<strong>").trim_end_matches("</strong>").to_string();
            let description = text[colon_pos + 1..].trim().to_string();
            
            if !name.is_empty() && !description.is_empty() {
                abilities.push(SpecialAbility { name, description });
            }
        }
    }
    
    abilities
}

fn parse_creature_actions(statblock: &scraper::ElementRef) -> Vec<CreatureAction> {
    let mut actions = Vec::new();
    
    // Look for Actions section
    let actions_selector = Selector::parse("h6:contains('Actions') + div ul li").unwrap();
    for li in statblock.select(&actions_selector) {
        let text = li.inner_html();
        if let Some(colon_pos) = text.find(':') {
            let name = text[..colon_pos].trim_start_matches("<strong>").trim_end_matches("</strong>").to_string();
            let description = text[colon_pos + 1..].trim().to_string();
            
            // Extract attack bonus, damage, etc. from description
            let attack_bonus = extract_attack_bonus(&description);
            let damage_formula = extract_damage_formula(&description);
            let save_dc = extract_save_dc(&description);
            
            actions.push(CreatureAction {
                name,
                description,
                attack_bonus,
                damage_formula,
                damage_type: None,
                range: None,
                save_dc,
                save_ability: None,
            });
        }
    }
    
    actions
}

fn extract_attack_bonus(text: &str) -> Option<i32> {
    if let Some(plus_pos) = text.find("+") {
        let after_plus = &text[plus_pos + 1..];
        after_plus.chars()
            .take_while(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse()
            .ok()
    } else {
        None
    }
}

fn extract_damage_formula(text: &str) -> Option<String> {
    // Look for dice formulas like "2d6 + 4"
    let dice_pattern = regex::Regex::new(r"\d+d\d+(?:\s*[+\-]\s*\d+)?").unwrap();
    dice_pattern.find(text).map(|m| m.as_str().to_string())
}

fn extract_save_dc(text: &str) -> Option<i32> {
    if let Some(dc_pos) = text.find("DC") {
        let after_dc = &text[dc_pos + 2..];
        after_dc.chars()
            .skip_while(|c| !c.is_ascii_digit())
            .take_while(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse()
            .ok()
    } else {
        None
    }
}

fn extract_alignment(statblock: &scraper::ElementRef) -> String {
    let alignment_selector = Selector::parse("ul li:contains('Alignment')").unwrap();
    if let Some(alignment_li) = statblock.select(&alignment_selector).next() {
        let text = alignment_li.inner_html();
        if let Some(colon_pos) = text.find(':') {
            text[colon_pos + 1..].trim().to_string()
        } else {
            "Unknown".to_string()
        }
    } else {
        "Unknown".to_string()
    }
}

fn extract_languages(statblock: &scraper::ElementRef) -> Vec<String> {
    let lang_selector = Selector::parse("ul li:contains('Languages')").unwrap();
    if let Some(lang_li) = statblock.select(&lang_selector).next() {
        let text = lang_li.inner_html();
        if let Some(colon_pos) = text.find(':') {
            text[colon_pos + 1..].split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect()
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    }
}

fn extract_senses(statblock: &scraper::ElementRef) -> Vec<String> {
    let senses_selector = Selector::parse("ul li:contains('Senses')").unwrap();
    if let Some(senses_li) = statblock.select(&senses_selector).next() {
        let text = senses_li.inner_html();
        if let Some(colon_pos) = text.find(':') {
            text[colon_pos + 1..].split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect()
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    }
}

fn extract_damage_immunities(statblock: &scraper::ElementRef) -> Vec<String> {
    let immunity_selector = Selector::parse("ul li:contains('Immunities')").unwrap();
    if let Some(immunity_li) = statblock.select(&immunity_selector).next() {
        let text = immunity_li.inner_html();
        if let Some(colon_pos) = text.find(':') {
            text[colon_pos + 1..].split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect()
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    }
}

/// Parse weather system with seasonal variations
fn parse_weather_system(document: &Html) -> Option<ParsedWeatherSystem> {
    let weather_header_selector = Selector::parse("h5:contains('Weather')").unwrap();
    let weather_header = document.select(&weather_header_selector).next()?;
    
    let title = weather_header.inner_html().trim().to_string();
    
    // Look for weather table
    let table_selector = Selector::parse("table.condensed").unwrap();
    let table = document.select(&table_selector).next()?;
    
    let header_row_selector = Selector::parse("tr th").unwrap();
    let headers: Vec<_> = table.select(&header_row_selector).map(|th| th.inner_html().trim().to_string()).collect();
    
    let row_selector = Selector::parse("tr").unwrap();
    let rows: Vec<_> = table.select(&row_selector).collect();
    
    let mut seasons = Vec::new();
    
    // Create seasons based on headers (skip first column which is dice roll)
    for (i, header) in headers.iter().skip(1).enumerate() {
        let mut entries = Vec::new();
        
        // Extract weather entries for this season
        for row in rows.iter().skip(1) { // Skip header row
            let cell_selector = Selector::parse("td").unwrap();
            let cells: Vec<_> = row.select(&cell_selector).collect();
            
            if cells.len() > i + 1 {
                let roll = cells[0].inner_html().trim().to_string();
                let condition = cells[i + 1].inner_html().trim().to_string();
                
                entries.push(WeatherEntry { roll, condition });
            }
        }
        
        if !entries.is_empty() {
            seasons.push(WeatherSeason {
                name: header.clone(),
                entries,
            });
        }
    }
    
    // Extract special effects (flood chances, etc.)
    let special_effects = extract_weather_special_effects(document);
    
    if seasons.is_empty() {
        return None;
    }
    
    Some(ParsedWeatherSystem {
        title,
        seasons,
        special_effects,
    })
}

fn extract_weather_special_effects(document: &Html) -> Vec<String> {
    let small_selector = Selector::parse("small").unwrap();
    document
        .select(&small_selector)
        .map(|el| el.inner_html().trim().to_string())
        .filter(|s| s.contains("flood") || s.contains("chance"))
        .collect()
}
