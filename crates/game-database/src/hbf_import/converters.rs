//! Converters for mapping HBF data to Dragon's Labyrinth database entities
//!
//! This module converts parsed HBF data into Dragon's Labyrinth database entities,
//! applying horror progression, corruption systems, and 180-level narrative structure.

use anyhow::Result;
use chrono::Utc;
use database_orm::*;
use sea_orm::Set;
use std::collections::HashMap;
use tracing::{debug, info};
use uuid::Uuid;
use super::database::{DragonEntities, HbfMapData};
use super::parsers::{ParsedEntities, ParsedHexFeature, ParsedSettlement, ParsedDungeonRoom, ParsedCity};

/// Convert all parsed HBF data to Dragon's Labyrinth entities
pub async fn convert_to_dragon_entities(
    map_data: &HbfMapData,
    parsed: &ParsedEntities,
) -> Result<DragonEntities> {
    info!("Converting HBF data to Dragon's Labyrinth entities");
    
    // Convert hex tiles with horror progression awareness
    let hex_tiles = convert_hex_tiles(map_data).await?;
    
    // Convert settlements with corruption integration
    let (settlements, settlement_weather) = convert_settlements(&parsed.settlements).await?;
    
    // Convert cities to settlements (treating POIs as individual settlements)
    let (city_settlements, city_weather) = convert_cities(&parsed.cities).await?;
    
    // Convert dungeon rooms to dungeons and rooms
    let (dungeons, dungeon_rooms, dungeon_doorways) = convert_dungeons(&parsed.dungeon_rooms).await?;
    
    // Convert hex features to additional settlements or dungeon entries
    let (feature_settlements, feature_dungeons) = convert_hex_features(&parsed.hex_features).await?;
    
    // Create NPCs from various sources
    let npcs = convert_npcs(&parsed).await?;
    
    // Combine all settlements
    let mut all_settlements = settlements;
    all_settlements.extend(city_settlements);
    all_settlements.extend(feature_settlements);
    
    // Combine all settlement weather
    let mut all_settlement_weather = settlement_weather;
    all_settlement_weather.extend(city_weather);
    
    // Combine all dungeons
    let mut all_dungeons = dungeons;
    all_dungeons.extend(feature_dungeons);
    
    info!("Converted: {} hex tiles, {} settlements, {} dungeons, {} rooms, {} doorways, {} NPCs",
          hex_tiles.len(), all_settlements.len(), all_dungeons.len(), 
          dungeon_rooms.len(), dungeon_doorways.len(), npcs.len());
    
    Ok(DragonEntities {
        hex_tiles,
        settlements: all_settlements,
        settlement_weather: all_settlement_weather,
        dungeons: all_dungeons,
        dungeon_rooms,
        dungeon_doorways,
        npcs,
    })
}

/// Convert HBF hex tiles to Dragon's Labyrinth hex tiles with horror integration
async fn convert_hex_tiles(map_data: &HbfMapData) -> Result<Vec<database_orm::hex_tiles::ActiveModel>> {
    let mut hex_tiles = Vec::new();
    
    for hbf_tile in &map_data.tiles {
        // Convert HBF coordinates to our hex coordinate system
        let (q, r, s) = convert_hbf_coords_to_cube(hbf_tile.x, hbf_tile.y);
        
        // Map HBF biome to our biome system
        let biome_type = map_hbf_biome_to_dragon_biome(&hbf_tile.biome);
        
        // Calculate corruption based on distance from center and biome type
        let corruption_level = calculate_initial_corruption(&hbf_tile.biome, hbf_tile.x, hbf_tile.y);
        
        // Calculate dread intensity based on biome and features
        let dread_intensity = calculate_dread_intensity(&hbf_tile.biome, &hbf_tile.feature);
        
        // Create features JSON from HBF data
        let features = serde_json::json!({
            "hbf_feature": hbf_tile.feature,
            "rivers": hbf_tile.rivers,
            "trails": hbf_tile.trails,
            "region": hbf_tile.region,
            "realm": hbf_tile.realm
        });
        
        let now = Utc::now();
        
        let tile = database_orm::hex_tiles::ActiveModel {
            id: Set(Uuid::new_v4()),
            q: Set(q),
            r: Set(r),
            s: Set(s),
            hbf_x: Set(Some(hbf_tile.x)),
            hbf_y: Set(Some(hbf_tile.y)),
            hbf_uuid: Set(Some(hbf_tile.uuid.clone())),
            biome_type: Set(biome_type),
            hbf_biome: Set(Some(hbf_tile.biome.clone())),
            tile_variant: Set("standard".to_string()),
            corruption_level: Set(corruption_level),
            dread_intensity: Set(dread_intensity),
            horror_events_count: Set(0),
            discovered: Set(false),
            fully_explored: Set(false),
            first_discovered_at: Set(None),
            last_visited_at: Set(None),
            features: Set(Some(features)),
            resources: Set(None),
            encounters: Set(None),
            atmospheric_description: Set(None),
            horror_description: Set(None),
            npcs_present: Set(None),
            companion_memories: Set(None),
            light_essence_strength: Set(0.0),
            dark_essence_strength: Set(corruption_level * 0.5),
            essence_stability: Set(1.0 - corruption_level),
            tile_asset_id: Set(None),
            ambient_audio_id: Set(None),
            weather_modifiers: Set(None),
            time_of_day_effects: Set(None),
            created_at: Set(now),
            last_modified: Set(now),
        };
        
        hex_tiles.push(tile);
    }
    
    Ok(hex_tiles)
}

/// Convert HBF axial coordinates to cube coordinates (q, r, s)
fn convert_hbf_coords_to_cube(x: i32, y: i32) -> (i32, i32, i32) {
    // HBF uses axial coordinates, convert to cube
    let q = x;
    let r = y;
    let s = -q - r;
    (q, r, s)
}

/// Map HBF biome types to Dragon's Labyrinth biome system
fn map_hbf_biome_to_dragon_biome(hbf_biome: &str) -> String {
    match hbf_biome {
        "JungleHex" => "jungle",
        "ForestHex" => "forest", 
        "MountainHex" => "mountain",
        "PlainHex" => "plains",
        "SwampHex" => "swamp",
        "DesertHex" => "desert",
        "TundraHex" => "tundra",
        "CoastHex" => "coast",
        "OceanHex" => "ocean",
        _ => "unknown",
    }.to_string()
}

/// Calculate initial corruption level based on biome and position
fn calculate_initial_corruption(biome: &str, x: i32, y: i32) -> f32 {
    // Base corruption by biome type
    let base_corruption = match biome {
        "SwampHex" => 0.3,
        "DesertHex" => 0.2,
        "TundraHex" => 0.1,
        "MountainHex" => 0.1,
        _ => 0.05,
    };
    
    // Distance from origin influences corruption (further = more corrupt)
    let distance = ((x * x + y * y) as f64).sqrt();
    let distance_factor = (distance / 100.0).min(0.4);
    
    (base_corruption + distance_factor as f32).min(1.0)
}

/// Calculate dread intensity contribution
fn calculate_dread_intensity(biome: &str, feature: &str) -> i32 {
    let biome_dread = match biome {
        "SwampHex" => 2,
        "TundraHex" => 1,
        "MountainHex" => 1,
        _ => 0,
    };
    
    let feature_dread = match feature {
        "Dungeon" => 3,
        "Settlement" => -1, // Settlements reduce dread
        "Ruins" => 2,
        _ => 0,
    };
    
    (biome_dread + feature_dread).clamp(0, 4)
}

/// Convert parsed settlements to database entities
async fn convert_settlements(
    settlements: &[ParsedSettlement],
) -> Result<(Vec<database_orm::settlements::ActiveModel>, Vec<database_orm::settlements::weather::ActiveModel>)> {
    let mut settlement_models = Vec::new();
    let mut weather_models = Vec::new();
    
    for settlement in settlements {
        let settlement_id = Uuid::new_v4();
        let now = Utc::now();
        
        // Calculate corruption based on settlement type and location
        let corruption_influence = match settlement.settlement_type.as_str() {
            "tavern" | "inn" => 0.1, // Slightly corrupting (alcohol, travelers)
            "temple" => -0.2, // Purifying
            "shop" => 0.05,
            _ => 0.0,
        };
        
        // Calculate prosperity based on settlement type
        let prosperity_level = match settlement.settlement_type.as_str() {
            "tavern" | "inn" => 6,
            "shop" => 5,
            "temple" => 4,
            _ => 3,
        };
        
        let services = serde_json::json!(settlement.services);
        let notable_features = serde_json::json!(settlement.features);
        
        let settlement_model = database_orm::settlements::ActiveModel {
            id: Set(settlement_id),
            name: Set(settlement.name.clone()),
            settlement_type: Set(settlement.settlement_type.clone()),
            hex_tile_id: Set(None), // Would need to match with hex tiles
            hbf_uuid: Set(Some(settlement.hbf_uuid.clone())),
            hbf_x: Set(settlement.coordinates.map(|(x, _)| x)),
            hbf_y: Set(settlement.coordinates.map(|(_, y)| y)),
            description: Set(settlement.description.clone()),
            population: Set(estimate_population(&settlement.settlement_type)),
            prosperity_level: Set(prosperity_level),
            corruption_influence: Set(corruption_influence),
            services: Set(Some(services)),
            notable_features: Set(Some(notable_features)),
            rumors: Set(None),
            trade_goods: Set(None),
            price_modifiers: Set(None),
            safety_rating: Set(calculate_safety_rating(&settlement.settlement_type)),
            reputation: Set(0),
            weather_data: Set(None),
            faction: Set(None),
            relationships: Set(None),
            dread_level_effects: Set(1), // Most settlements become more dangerous with dread
            corrupted_description: Set(None),
            companion_reactions: Set(None),
            discovered: Set(false),
            first_visited_at: Set(None),
            last_visited_at: Set(None),
            created_at: Set(now),
            last_modified: Set(now),
        };
        
        settlement_models.push(settlement_model);
        
        // Convert weather system if present
        if let Some(weather_system) = &settlement.weather_system {
            for season in &weather_system.seasons {
                for entry in &season.entries {
                    let weather_model = database_orm::settlements::weather::ActiveModel {
                        id: Set(Uuid::new_v4()),
                        settlement_id: Set(settlement_id),
                        season: Set(normalize_season_name(&season.name)),
                        dice_roll: Set(entry.roll.clone()),
                        weather_condition: Set(entry.condition.clone()),
                        mechanical_effects: Set(None),
                        flood_chance: Set(weather_system.special_effects.first().cloned()),
                        horror_weather_variants: Set(None),
                        created_at: Set(now),
                    };
                    weather_models.push(weather_model);
                }
            }
        }
    }
    
    Ok((settlement_models, weather_models))
}

fn estimate_population(settlement_type: &str) -> Option<i32> {
    match settlement_type {
        "tavern" | "inn" => Some(15),
        "shop" => Some(8),
        "temple" => Some(12),
        "village" => Some(50),
        "town" => Some(200),
        _ => None,
    }
}

fn calculate_safety_rating(settlement_type: &str) -> i32 {
    match settlement_type {
        "temple" => 8,
        "tavern" | "inn" => 6,
        "shop" => 5,
        _ => 4,
    }
}

fn normalize_season_name(season: &str) -> String {
    match season.to_lowercase().as_str() {
        "warm season" => "warm".to_string(),
        "dry season" => "dry".to_string(), 
        "wet season" => "wet".to_string(),
        "cold season" => "cold".to_string(),
        _ => season.to_lowercase(),
    }
}

/// Convert parsed cities to multiple settlement entities (one per POI)
async fn convert_cities(
    cities: &[ParsedCity],
) -> Result<(Vec<database_orm::settlements::ActiveModel>, Vec<database_orm::settlements::weather::ActiveModel>)> {
    let mut settlement_models = Vec::new();
    let mut weather_models = Vec::new();
    
    for city in cities {
        // Convert each POI to a settlement
        for poi in &city.points_of_interest {
            let settlement_id = Uuid::new_v4();
            let now = Utc::now();
            
            // Convert POI coordinates to hex coordinates (approximate)
            let hex_coords = convert_city_coords_to_hex(poi.coords.0, poi.coords.1);
            
            let settlement_model = database_orm::settlements::ActiveModel {
                id: Set(settlement_id),
                name: Set(poi.title.clone()),
                settlement_type: Set(poi.poi_type.clone()),
                hex_tile_id: Set(None),
                hbf_uuid: Set(Some(poi.uuid.clone())),
                hbf_x: Set(Some(hex_coords.0)),
                hbf_y: Set(Some(hex_coords.1)),
                description: Set(format!("City {}: {}", poi.poi_type, poi.title)),
                population: Set(estimate_population(&poi.poi_type)),
                prosperity_level: Set(7), // Cities are generally prosperous
                corruption_influence: Set(0.15), // Cities have moderate corruption
                services: Set(Some(serde_json::json!([poi.poi_type]))),
                notable_features: Set(Some(serde_json::json!(["city_building"]))),
                rumors: Set(None),
                trade_goods: Set(None),
                price_modifiers: Set(None),
                safety_rating: Set(7), // Cities are generally safer
                reputation: Set(0),
                weather_data: Set(None),
                faction: Set(Some("city_dwellers".to_string())),
                relationships: Set(None),
                dread_level_effects: Set(2), // Cities change significantly with dread
                corrupted_description: Set(None),
                companion_reactions: Set(None),
                discovered: Set(false),
                first_visited_at: Set(None),
                last_visited_at: Set(None),
                created_at: Set(now),
                last_modified: Set(now),
            };
            
            settlement_models.push(settlement_model);
        }
    }
    
    Ok((settlement_models, weather_models))
}

fn convert_city_coords_to_hex(city_x: f64, city_y: f64) -> (i32, i32) {
    // Convert city local coordinates to approximate hex coordinates
    // This is a rough approximation - would need better mapping
    ((city_x / 10.0) as i32, (city_y / 10.0) as i32)
}

/// Convert dungeon rooms to full dungeon structures
async fn convert_dungeons(
    rooms: &[ParsedDungeonRoom],
) -> Result<(
    Vec<database_orm::dungeons::ActiveModel>,
    Vec<database_orm::dungeons::rooms::ActiveModel>,
    Vec<database_orm::dungeons::doorways::ActiveModel>,
)> {
    let mut dungeons = Vec::new();
    let mut dungeon_rooms = Vec::new();
    let mut doorways = Vec::new();
    
    // Group rooms by parent dungeon
    let mut dungeons_by_name: HashMap<String, Vec<&ParsedDungeonRoom>> = HashMap::new();
    
    for room in rooms {
        let dungeon_name = room.parent_dungeon.clone().unwrap_or_else(|| "Unknown Dungeon".to_string());
        dungeons_by_name.entry(dungeon_name).or_insert_with(Vec::new).push(room);
    }
    
    // Create dungeon entities for each group
    for (dungeon_name, dungeon_rooms_list) in dungeons_by_name {
        let dungeon_id = Uuid::new_v4();
        let now = Utc::now();
        
        // Calculate dungeon properties from rooms
        let total_rooms = dungeon_rooms_list.len() as i32;
        let danger_level = calculate_dungeon_danger(&dungeon_rooms_list);
        let corruption_intensity = calculate_dungeon_corruption(&dungeon_rooms_list);
        
        // Determine dungeon type from rooms
        let dungeon_type = determine_dungeon_type(&dungeon_rooms_list);
        
        let dungeon_model = database_orm::dungeons::ActiveModel {
            id: Set(dungeon_id),
            name: Set(dungeon_name.clone()),
            dungeon_type: Set(dungeon_type),
            hex_tile_id: Set(None),
            hbf_uuid: Set(Some(format!("dungeon_{}", dungeon_name.replace(' ', "_")))),
            hbf_x: Set(dungeon_rooms_list.first().and_then(|r| r.coordinates.map(|(x, _)| x))),
            hbf_y: Set(dungeon_rooms_list.first().and_then(|r| r.coordinates.map(|(_, y)| y))),
            description: Set(format!("A {} with {} rooms", dungeon_type, total_rooms)),
            total_rooms: Set(total_rooms),
            levels: Set(1), // Assume single level for now
            estimated_size: Set(estimate_dungeon_size(total_rooms)),
            danger_level: Set(danger_level),
            recommended_level: Set((danger_level + 1) / 2),
            corruption_intensity: Set(corruption_intensity),
            themes: Set(Some(serde_json::json!(["undead", "ancient"]))),
            special_features: Set(None),
            environmental_hazards: Set(None),
            treasure_hints: Set(None),
            boss_encounters: Set(None),
            lore: Set(None),
            story_connections: Set(None),
            dread_level_effects: Set(3), // Dungeons are heavily affected by dread
            corrupted_description: Set(None),
            discovered: Set(false),
            partially_explored: Set(false),
            fully_cleared: Set(false),
            first_entered_at: Set(None),
            last_visited_at: Set(None),
            cleared_at: Set(None),
            created_at: Set(now),
            last_modified: Set(now),
        };
        
        dungeons.push(dungeon_model);
        
        // Convert individual rooms
        for room in dungeon_rooms_list {
            let room_id = Uuid::new_v4();
            
            let room_model = database_orm::dungeons::rooms::ActiveModel {
                id: Set(room_id),
                dungeon_id: Set(dungeon_id),
                area_number: Set(room.area_number.unwrap_or(1)),
                title: Set(room.title.clone()),
                room_type: Set(room.room_type.clone()),
                hbf_uuid: Set(Some(room.hbf_uuid.clone())),
                description: Set(room.description.clone()),
                doorways: Set(Some(serde_json::json!(room.doorways))),
                features: Set(Some(serde_json::json!(room.features))),
                encounters: Set(room.wandering_monsters.as_ref().map(|enc| serde_json::json!(enc))),
                treasure: Set(None),
                lighting: Set(Some("dim".to_string())),
                atmosphere: Set(Some(determine_room_atmosphere(&room.description))),
                environmental_effects: Set(None),
                discovered: Set(false),
                searched: Set(false),
                cleared: Set(false),
                first_entered_at: Set(None),
                last_visited_at: Set(None),
                created_at: Set(now),
                last_modified: Set(now),
            };
            
            dungeon_rooms.push(room_model);
            
            // Convert doorways
            for doorway in &room.doorways {
                let doorway_model = database_orm::dungeons::doorways::ActiveModel {
                    id: Set(Uuid::new_v4()),
                    room_id: Set(room_id),
                    direction: Set(doorway.direction.clone()),
                    material: Set(doorway.material.clone()),
                    shape: Set(doorway.shape.clone()),
                    condition: Set(doorway.condition.clone()),
                    leads_to_room_id: Set(None),
                    leads_to_area_number: Set(None),
                    locked: Set(doorway.locked),
                    trapped: Set(false),
                    secret: Set(false),
                    magical: Set(doorway.magical),
                    unlock_method: Set(doorway.key_location.clone()),
                    trap_description: Set(None),
                    discovered: Set(false),
                    opened: Set(false),
                    trap_triggered: Set(false),
                    created_at: Set(now),
                };
                
                doorways.push(doorway_model);
            }
        }
    }
    
    Ok((dungeons, dungeon_rooms, doorways))
}

fn calculate_dungeon_danger(rooms: &[&ParsedDungeonRoom]) -> i32 {
    // Base danger on number of rooms and encounter complexity
    let room_count_danger = (rooms.len() / 5).min(8) as i32;
    let encounter_danger = rooms
        .iter()
        .filter(|r| r.wandering_monsters.is_some())
        .count()
        .min(6) as i32;
    
    (room_count_danger + encounter_danger).min(10)
}

fn calculate_dungeon_corruption(rooms: &[&ParsedDungeonRoom]) -> f32 {
    // Dungeons are inherently corrupted, more so with undead encounters
    let base_corruption = 0.4;
    
    let undead_encounters = rooms
        .iter()
        .filter(|r| {
            r.wandering_monsters.as_ref()
                .map(|enc| enc.encounters.iter().any(|e| e.creature_name.to_lowercase().contains("undead") ||
                                                       e.creature_name.to_lowercase().contains("ghost") ||
                                                       e.creature_name.to_lowercase().contains("wight") ||
                                                       e.creature_name.to_lowercase().contains("specter")))
                .unwrap_or(false)
        })
        .count() as f32;
    
    let undead_factor = (undead_encounters / rooms.len() as f32) * 0.3;
    
    (base_corruption + undead_factor).min(1.0)
}

fn determine_dungeon_type(rooms: &[&ParsedDungeonRoom]) -> String {
    let crypt_count = rooms.iter().filter(|r| r.room_type.contains("crypt")).count();
    let corridor_count = rooms.iter().filter(|r| r.room_type.contains("corridor")).count();
    
    if crypt_count > rooms.len() / 2 {
        "crypt"
    } else if corridor_count > rooms.len() / 3 {
        "labyrinth"
    } else {
        "ruins"
    }.to_string()
}

fn estimate_dungeon_size(room_count: i32) -> String {
    match room_count {
        1..=5 => "small",
        6..=15 => "medium",
        16..=30 => "large",
        _ => "massive",
    }.to_string()
}

fn determine_room_atmosphere(description: &str) -> String {
    let desc_lower = description.to_lowercase();
    if desc_lower.contains("cold") || desc_lower.contains("ice") {
        "cold"
    } else if desc_lower.contains("damp") || desc_lower.contains("water") {
        "damp"
    } else if desc_lower.contains("scorch") || desc_lower.contains("burn") {
        "scorched"
    } else if desc_lower.contains("spore") || desc_lower.contains("mushroom") {
        "fungal"
    } else {
        "eerie"
    }.to_string()
}

/// Convert hex features to settlements or dungeons as appropriate
async fn convert_hex_features(
    features: &[ParsedHexFeature],
) -> Result<(Vec<database_orm::settlements::ActiveModel>, Vec<database_orm::dungeons::ActiveModel>)> {
    let mut settlements = Vec::new();
    let mut dungeons = Vec::new();
    
    for feature in features {
        match feature.feature_type.as_str() {
            "watchtower" | "shrine" => {
                // Convert to outpost-type settlement
                let settlement = convert_feature_to_settlement(feature).await?;
                settlements.push(settlement);
            }
            "ruins" => {
                // Convert to dungeon
                let dungeon = convert_feature_to_dungeon(feature).await?;
                dungeons.push(dungeon);
            }
            _ => {
                // Default to settlement
                let settlement = convert_feature_to_settlement(feature).await?;
                settlements.push(settlement);
            }
        }
    }
    
    Ok((settlements, dungeons))
}

async fn convert_feature_to_settlement(feature: &ParsedHexFeature) -> Result<database_orm::settlements::ActiveModel> {
    let now = Utc::now();
    
    Ok(database_orm::settlements::ActiveModel {
        id: Set(Uuid::new_v4()),
        name: Set(feature.name.clone()),
        settlement_type: Set("outpost".to_string()),
        hex_tile_id: Set(None),
        hbf_uuid: Set(Some(feature.hbf_uuid.clone())),
        hbf_x: Set(feature.coordinates.map(|(x, _)| x)),
        hbf_y: Set(feature.coordinates.map(|(_, y)| y)),
        description: Set(feature.description.clone()),
        population: Set(Some(5)),
        prosperity_level: Set(3),
        corruption_influence: Set(0.05),
        services: Set(Some(serde_json::json!(["shelter"]))),
        notable_features: Set(Some(serde_json::json!(feature.special_features))),
        rumors: Set(None),
        trade_goods: Set(None),
        price_modifiers: Set(None),
        safety_rating: Set(5),
        reputation: Set(0),
        weather_data: Set(None),
        faction: Set(None),
        relationships: Set(None),
        dread_level_effects: Set(1),
        corrupted_description: Set(None),
        companion_reactions: Set(None),
        discovered: Set(false),
        first_visited_at: Set(None),
        last_visited_at: Set(None),
        created_at: Set(now),
        last_modified: Set(now),
    })
}

async fn convert_feature_to_dungeon(feature: &ParsedHexFeature) -> Result<database_orm::dungeons::ActiveModel> {
    let now = Utc::now();
    
    Ok(database_orm::dungeons::ActiveModel {
        id: Set(Uuid::new_v4()),
        name: Set(feature.name.clone()),
        dungeon_type: Set("ruins".to_string()),
        hex_tile_id: Set(None),
        hbf_uuid: Set(Some(feature.hbf_uuid.clone())),
        hbf_x: Set(feature.coordinates.map(|(x, _)| x)),
        hbf_y: Set(feature.coordinates.map(|(_, y)| y)),
        description: Set(feature.description.clone()),
        total_rooms: Set(3), // Estimate for features
        levels: Set(1),
        estimated_size: Set("small".to_string()),
        danger_level: Set(4),
        recommended_level: Set(2),
        corruption_intensity: Set(0.6),
        themes: Set(Some(serde_json::json!(["ancient", "forgotten"]))),
        special_features: Set(Some(serde_json::json!(feature.special_features))),
        environmental_hazards: Set(None),
        treasure_hints: Set(None),
        boss_encounters: Set(None),
        lore: Set(None),
        story_connections: Set(None),
        dread_level_effects: Set(2),
        corrupted_description: Set(None),
        discovered: Set(false),
        partially_explored: Set(false),
        fully_cleared: Set(false),
        first_entered_at: Set(None),
        last_visited_at: Set(None),
        cleared_at: Set(None),
        created_at: Set(now),
        last_modified: Set(now),
    })
}

/// Convert various parsed entities to NPCs
async fn convert_npcs(parsed: &ParsedEntities) -> Result<Vec<database_orm::npcs::ActiveModel>> {
    let mut npcs = Vec::new();
    
    // Convert creatures to NPCs where appropriate
    for creature in &parsed.creatures {
        if is_npc_creature(&creature.creature_type, &creature.alignment) {
            let npc = convert_creature_to_npc(creature).await?;
            npcs.push(npc);
        }
    }
    
    // Convert city POIs to NPCs where appropriate
    for city in &parsed.cities {
        for poi in &city.points_of_interest {
            if should_create_npc_for_poi(&poi.poi_type) {
                let npc = convert_poi_to_npc(poi, city).await?;
                npcs.push(npc);
            }
        }
    }
    
    Ok(npcs)
}

fn is_npc_creature(creature_type: &str, alignment: &str) -> bool {
    // Only convert humanoid creatures with non-evil alignments to NPCs
    creature_type == "humanoid" && !alignment.to_lowercase().contains("evil")
}

fn should_create_npc_for_poi(poi_type: &str) -> bool {
    match poi_type {
        "blacksmith" | "witch" | "tailor" | "herbalist" | "veterinarian" => true,
        _ => false,
    }
}

async fn convert_creature_to_npc(creature: &super::parsers::ParsedCreature) -> Result<database_orm::npcs::ActiveModel> {
    let now = Utc::now();
    
    Ok(database_orm::npcs::ActiveModel {
        id: Set(Uuid::new_v4()),
        name: Set(creature.name.clone()),
        role: Set("traveler".to_string()),
        race: Set("human".to_string()),
        hex_tile_id: Set(None),
        settlement_id: Set(None),
        dungeon_id: Set(None),
        hbf_uuid: Set(Some(creature.hbf_uuid.clone())),
        hbf_x: Set(None),
        hbf_y: Set(None),
        description: Set("A wandering individual".to_string()),
        personality: Set(None),
        background: Set(None),
        level: Set(Some(1)),
        hit_points: Set(Some(creature.armor_class)), // Rough approximation
        armor_class: Set(Some(creature.armor_class)),
        ability_scores: Set(Some(serde_json::json!({
            "strength": creature.ability_scores.strength,
            "dexterity": creature.ability_scores.dexterity,
            "constitution": creature.ability_scores.constitution,
            "intelligence": creature.ability_scores.intelligence,
            "wisdom": creature.ability_scores.wisdom,
            "charisma": creature.ability_scores.charisma
        }))),
        equipment: Set(None),
        disposition: Set(0),
        reputation_awareness: Set(0),
        dialogue_options: Set(None),
        rumors_known: Set(None),
        services_offered: Set(None),
        trade_goods: Set(None),
        price_modifiers: Set(None),
        wealth_level: Set(3),
        relationships: Set(None),
        faction: Set(None),
        behavior_type: Set("neutral".to_string()),
        daily_schedule: Set(None),
        interaction_triggers: Set(None),
        corruption_susceptibility: Set(0.3),
        current_corruption_level: Set(0.0),
        dread_level_effects: Set(1),
        corrupted_description: Set(None),
        corruption_triggers: Set(None),
        companion_reactions: Set(None),
        companion_memories: Set(None),
        quest_connections: Set(None),
        story_importance: Set(None),
        alive: Set(true),
        encountered: Set(false),
        first_met_at: Set(None),
        last_interaction_at: Set(None),
        times_interacted: Set(0),
        mobile: Set(true),
        movement_pattern: Set(None),
        created_at: Set(now),
        last_modified: Set(now),
    })
}

async fn convert_poi_to_npc(poi: &super::parsers::CityPoi, city: &ParsedCity) -> Result<database_orm::npcs::ActiveModel> {
    let now = Utc::now();
    let hex_coords = convert_city_coords_to_hex(poi.coords.0, poi.coords.1);
    
    let role = match poi.poi_type.as_str() {
        "blacksmith" => "craftsperson",
        "witch" => "mystic",
        "tailor" => "artisan", 
        "herbalist" => "healer",
        "veterinarian" => "animal_healer",
        _ => "citizen",
    };
    
    let services = match poi.poi_type.as_str() {
        "blacksmith" => vec!["weapon_repair", "armor_crafting"],
        "witch" => vec!["potion_brewing", "fortune_telling"],
        "tailor" => vec!["clothing_repair", "custom_clothing"],
        "herbalist" => vec!["healing_potions", "herb_knowledge"],
        "veterinarian" => vec!["animal_healing", "mount_care"],
        _ => vec!["general_services"],
    };
    
    Ok(database_orm::npcs::ActiveModel {
        id: Set(Uuid::new_v4()),
        name: Set(poi.title.clone()),
        role: Set(role.to_string()),
        race: Set("human".to_string()),
        hex_tile_id: Set(None),
        settlement_id: Set(None),
        dungeon_id: Set(None),
        hbf_uuid: Set(Some(poi.uuid.clone())),
        hbf_x: Set(Some(hex_coords.0)),
        hbf_y: Set(Some(hex_coords.1)),
        description: Set(format!("A {} operating in the city", role)),
        personality: Set(Some(generate_personality_for_role(role))),
        background: Set(Some(format!("Has worked as a {} for many years", role))),
        level: Set(Some(3)),
        hit_points: Set(Some(20)),
        armor_class: Set(Some(12)),
        ability_scores: Set(Some(generate_ability_scores_for_role(role))),
        equipment: Set(Some(generate_equipment_for_role(role))),
        disposition: Set(5), // Neutral to positive
        reputation_awareness: Set(3),
        dialogue_options: Set(Some(serde_json::json!(["services", "local_news", "trade"]))),
        rumors_known: Set(Some(serde_json::json!(["city_gossip"]))),
        services_offered: Set(Some(serde_json::json!(services))),
        trade_goods: Set(Some(generate_trade_goods_for_role(role))),
        price_modifiers: Set(None),
        wealth_level: Set(6), // City dwellers are moderately wealthy
        relationships: Set(None),
        faction: Set(Some("city_guild".to_string())),
        behavior_type: Set("friendly".to_string()),
        daily_schedule: Set(Some(serde_json::json!({
            "morning": "work",
            "afternoon": "work", 
            "evening": "rest"
        }))),
        interaction_triggers: Set(None),
        corruption_susceptibility: Set(0.2),
        current_corruption_level: Set(0.05),
        dread_level_effects: Set(2),
        corrupted_description: Set(None),
        corruption_triggers: Set(None),
        companion_reactions: Set(None),
        companion_memories: Set(None),
        quest_connections: Set(None),
        story_importance: Set(None),
        alive: Set(true),
        encountered: Set(false),
        first_met_at: Set(None),
        last_interaction_at: Set(None),
        times_interacted: Set(0),
        mobile: Set(false), // City NPCs stay in place
        movement_pattern: Set(None),
        created_at: Set(now),
        last_modified: Set(now),
    })
}

fn generate_personality_for_role(role: &str) -> String {
    match role {
        "craftsperson" => "Practical and hardworking, takes pride in quality work",
        "mystic" => "Mysterious and wise, speaks in riddles",
        "artisan" => "Creative and detail-oriented, perfectionist",
        "healer" => "Caring and knowledgeable, always ready to help",
        "animal_healer" => "Patient and gentle, has strong bond with animals",
        _ => "Friendly and helpful, typical city dweller",
    }.to_string()
}

fn generate_ability_scores_for_role(role: &str) -> serde_json::Value {
    match role {
        "craftsperson" => serde_json::json!({
            "strength": 14, "dexterity": 12, "constitution": 13,
            "intelligence": 12, "wisdom": 11, "charisma": 10
        }),
        "mystic" => serde_json::json!({
            "strength": 8, "dexterity": 10, "constitution": 12,
            "intelligence": 15, "wisdom": 16, "charisma": 14
        }),
        "artisan" => serde_json::json!({
            "strength": 10, "dexterity": 16, "constitution": 12,
            "intelligence": 14, "wisdom": 12, "charisma": 11
        }),
        "healer" => serde_json::json!({
            "strength": 9, "dexterity": 11, "constitution": 13,
            "intelligence": 14, "wisdom": 16, "charisma": 12
        }),
        _ => serde_json::json!({
            "strength": 10, "dexterity": 10, "constitution": 10,
            "intelligence": 10, "wisdom": 10, "charisma": 10
        }),
    }
}

fn generate_equipment_for_role(role: &str) -> serde_json::Value {
    match role {
        "craftsperson" => serde_json::json!(["smith_tools", "leather_apron", "work_clothes"]),
        "mystic" => serde_json::json!(["spell_components", "mystical_robes", "crystal_ball"]),
        "artisan" => serde_json::json!(["sewing_kit", "fine_clothes", "measuring_tools"]),
        "healer" => serde_json::json!(["herbalism_kit", "healing_supplies", "simple_robes"]),
        _ => serde_json::json!(["common_clothes", "basic_tools"]),
    }
}

fn generate_trade_goods_for_role(role: &str) -> serde_json::Value {
    match role {
        "craftsperson" => serde_json::json!({
            "buys": ["metal_ingots", "coal", "gems"],
            "sells": ["weapons", "armor", "tools"]
        }),
        "mystic" => serde_json::json!({
            "buys": ["rare_components", "ancient_texts"],
            "sells": ["potions", "scrolls", "fortune_telling"]
        }),
        "artisan" => serde_json::json!({
            "buys": ["fine_fabrics", "dyes", "thread"],
            "sells": ["clothing", "tapestries", "repairs"]
        }),
        "healer" => serde_json::json!({
            "buys": ["herbs", "medicinal_plants"],
            "sells": ["healing_potions", "medical_supplies"]
        }),
        _ => serde_json::json!({
            "buys": ["common_goods"],
            "sells": ["general_supplies"]
        }),
    }
}
