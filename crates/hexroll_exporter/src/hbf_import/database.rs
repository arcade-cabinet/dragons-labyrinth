//! HBF SQLite Analysis - discovers entity patterns to generate ORM models

use anyhow::{Context, Result};
use sea_orm::*;
use serde_json::Value;
use std::collections::HashMap;
use tracing::{info, warn};

/// Complete HBF snapshot data loaded from SQLite - for analysis only
#[derive(Debug)]
pub struct HbfSnapshot {
    pub map_data: HbfMapData,
    pub entities: HashMap<String, String>,
    pub refs: Vec<HbfRef>,
}

/// Map data structure from HBF JSON
#[derive(Debug)]
pub struct HbfMapData {
    pub tiles: Vec<HbfTile>,
    pub realms: HashMap<String, String>,
    pub regions: HashMap<String, String>,
    pub borders: HashMap<String, Vec<HbfBorder>>,
}

#[derive(Debug, Clone)]
pub struct HbfTile {
    pub x: i32,
    pub y: i32,
    pub biome: String,
    pub uuid: String,
    pub feature: String,
    pub feature_uuid: Option<String>,
    pub rivers: Vec<u8>,
    pub trails: Vec<u8>,
    pub region: Option<String>,
    pub realm: Option<String>,
}

#[derive(Debug, Clone)]
pub struct HbfBorder {
    pub hex_x: i32,
    pub hex_y: i32,
    pub borders: Vec<u8>,
}

#[derive(Debug)]
pub struct HbfRef {
    pub value: String,
    pub details: Option<String>,
    pub uuid: String,
    pub entity_type: Option<String>,
    pub icon: Option<String>,
    pub anchor: Option<String>,
}

/// Load complete HBF snapshot from SQLite file for analysis
pub async fn load_hbf_snapshot(hbf_path: &str) -> Result<HbfSnapshot> {
    info!("Loading HBF snapshot from: {}", hbf_path);
    
    // Connect to HBF database in read-only mode
    let hbf_url = format!("sqlite://{}?mode=ro", hbf_path);
    let hbf_db = Database::connect(&hbf_url).await
        .context("Failed to connect to HBF database")?;
    
    // Load map data first (contains hex grid structure)
    let map_data = load_map_data(&hbf_db).await
        .context("Failed to load map data from HBF")?;
    
    // Load all HTML entities (excluding the map entry)
    let entities = load_html_entities(&hbf_db).await
        .context("Failed to load HTML entities from HBF")?;
    
    // Load refs table (search metadata)
    let refs = load_refs_data(&hbf_db).await
        .context("Failed to load refs data from HBF")?;
    
    info!("Loaded HBF snapshot: {} tiles, {} entities, {} refs", 
          map_data.tiles.len(), entities.len(), refs.len());
    
    Ok(HbfSnapshot {
        map_data,
        entities,
        refs,
    })
}

/// Load and parse the special "map" entity containing hex grid JSON
async fn load_map_data(db: &DatabaseConnection) -> Result<HbfMapData> {
    // Query for the special map entity
    let map_result: Option<(String,)> = db
        .query_one(Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Sqlite,
            "SELECT value FROM Entities WHERE uuid = ?",
            vec!["map".into()],
        ))
        .await?
        .map(|row| (row.try_get::<String>("", "value").unwrap_or_default(),));
    
    let map_json = map_result
        .context("Map entity not found in HBF database")?
        .0;
    
    let map_value: Value = serde_json::from_str(&map_json)
        .context("Failed to parse map JSON")?;
    
    // Parse hex tiles
    let tiles_array = map_value["map"]
        .as_array()
        .context("Map tiles array not found")?;
    
    let mut tiles = Vec::new();
    for tile_value in tiles_array {
        match parse_hbf_tile(tile_value) {
            Ok(tile) => tiles.push(tile),
            Err(e) => warn!("Failed to parse tile: {}", e),
        }
    }
    
    // Parse realms
    let mut realms = HashMap::new();
    if let Some(realms_obj) = map_value["realms"].as_object() {
        for (uuid, realm_data) in realms_obj {
            if let Some(name) = realm_data["name"].as_str() {
                realms.insert(uuid.clone(), name.to_string());
            }
        }
    }
    
    // Parse regions  
    let mut regions = HashMap::new();
    if let Some(regions_obj) = map_value["regions"].as_object() {
        for (uuid, name) in regions_obj {
            if let Some(name_str) = name.as_str() {
                regions.insert(uuid.clone(), name_str.to_string());
            }
        }
    }
    
    // Parse borders
    let mut borders = HashMap::new();
    if let Some(borders_obj) = map_value["borders"].as_object() {
        for (realm_uuid, border_array) in borders_obj {
            if let Some(border_list) = border_array.as_array() {
                let mut realm_borders = Vec::new();
                for border_value in border_list {
                    if let Ok(border) = parse_hbf_border(border_value) {
                        realm_borders.push(border);
                    }
                }
                borders.insert(realm_uuid.clone(), realm_borders);
            }
        }
    }
    
    info!("Parsed {} hex tiles, {} realms, {} regions", 
          tiles.len(), realms.len(), regions.len());
    
    Ok(HbfMapData {
        tiles,
        realms,
        regions,
        borders,
    })
}

fn parse_hbf_tile(value: &Value) -> Result<HbfTile> {
    Ok(HbfTile {
        x: value["x"].as_i64().unwrap_or(0) as i32,
        y: value["y"].as_i64().unwrap_or(0) as i32,
        biome: value["type"].as_str().unwrap_or("Unknown").to_string(),
        uuid: value["uuid"].as_str().unwrap_or("").to_string(),
        feature: value["feature"].as_str().unwrap_or("Other").to_string(),
        feature_uuid: value["feature_uuid"].as_str().map(|s| s.to_string()),
        rivers: value["rivers"].as_array()
            .map(|arr| arr.iter().filter_map(|v| v.as_u64().map(|n| n as u8)).collect())
            .unwrap_or_default(),
        trails: value["trails"].as_array()
            .map(|arr| arr.iter().filter_map(|v| v.as_u64().map(|n| n as u8)).collect())
            .unwrap_or_default(),
        region: value["region"].as_str().map(|s| s.to_string()),
        realm: value["realm"].as_str().map(|s| s.to_string()),
    })
}

fn parse_hbf_border(value: &Value) -> Result<HbfBorder> {
    Ok(HbfBorder {
        hex_x: value["hex_x"].as_i64().unwrap_or(0) as i32,
        hex_y: value["hex_y"].as_i64().unwrap_or(0) as i32,
        borders: value["borders"].as_array()
            .map(|arr| arr.iter().filter_map(|v| v.as_u64().map(|n| n as u8)).collect())
            .unwrap_or_default(),
    })
}

/// Load all HTML entities from the Entities table (excluding map)
async fn load_html_entities(db: &DatabaseConnection) -> Result<HashMap<String, String>> {
    let mut entities = HashMap::new();
    
    let results: Vec<(String, String)> = db
        .query_all(Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Sqlite,
            "SELECT uuid, value FROM Entities WHERE uuid != 'map' AND value IS NOT NULL AND value != ''",
            vec![],
        ))
        .await?
        .into_iter()
        .filter_map(|row| {
            let uuid = row.try_get::<String>("", "uuid").ok()?;
            let value = row.try_get::<String>("", "value").ok()?;
            if !value.trim().is_empty() {
                Some((uuid, value))
            } else {
                None
            }
        })
        .collect();
    
    for (uuid, value) in results {
        entities.insert(uuid, value);
    }
    
    info!("Loaded {} non-empty HTML entities", entities.len());
    Ok(entities)
}

/// Load refs data for search/metadata
async fn load_refs_data(db: &DatabaseConnection) -> Result<Vec<HbfRef>> {
    let results: Vec<HbfRef> = db
        .query_all(Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Sqlite,
            "SELECT value, details, uuid, type, icon, anchor FROM Refs",
            vec![],
        ))
        .await?
        .into_iter()
        .filter_map(|row| {
            Some(HbfRef {
                value: row.try_get::<String>("", "value").ok()?,
                details: row.try_get::<String>("", "details").ok(),
                uuid: row.try_get::<String>("", "uuid").ok()?,
                entity_type: row.try_get::<String>("", "type").ok(),
                icon: row.try_get::<String>("", "icon").ok(),
                anchor: row.try_get::<String>("", "anchor").ok(),
            })
        })
        .collect();
    
    info!("Loaded {} refs entries", results.len());
    Ok(results)
}
