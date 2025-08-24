use std::collections::HashMap;

use anyhow::{Context, Result};
use bevy::prelude::*;
use rusqlite::{Connection, Row};
use serde::Deserialize;

/// A single hex tile extracted from the map JSON.
#[derive(Debug, Clone)]
pub struct MapTile {
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

/// Represents a single border entry for realm boundaries.
#[derive(Debug, Clone)]
pub struct BorderEntry {
    pub hex_x: i32,
    pub hex_y: i32,
    pub borders: Vec<u8>,
}

/// All data extracted from the `map` JSON entry.
#[derive(Debug)]
pub struct MapData {
    pub tiles: Vec<MapTile>,
    pub realms: HashMap<String, String>,
    pub regions: HashMap<String, String>,
    pub borders: HashMap<String, Vec<BorderEntry>>, 
}

/// Raw HTML pages for all entities (excluding the `map` entry itself).
pub type HtmlEntities = HashMap<String, String>;

/// A single row from the `Refs` table.  Use this to build search indices.
#[derive(Debug)]
pub struct RefRecord {
    pub value: String,
    pub details: Option<String>,
    pub uuid: String,
    pub entity_type: Option<String>,
    pub icon: Option<String>,
    pub anchor: Option<String>,
}

/// Top‑level snapshot data loaded from an HBF file.
#[derive(Debug)]
pub struct HexrollSnapshot {
    pub map: MapData,
    pub entities: HtmlEntities,
    pub refs: Vec<RefRecord>,
}

/// Deserialize target for the `map` JSON.  This mirrors the structure of the
/// JSON stored in the `Entities` table.  We use `serde` to parse it.
#[derive(Debug, Deserialize)]
struct RawMap {
    map: Vec<RawTile>,
    realms: HashMap<String, RawRealm>,
    regions: HashMap<String, String>,
    borders: HashMap<String, Vec<RawBorder>>, 
}

#[derive(Debug, Deserialize)]
struct RawTile {
    x: i32,
    y: i32,
    #[serde(rename = "type")]
    type_: String,
    uuid: String,
    feature: String,
    #[serde(default, rename = "feature_uuid")]
    feature_uuid: Option<String>,
    #[serde(default)]
    rivers: Vec<u8>,
    #[serde(default)]
    trails: Vec<u8>,
    #[serde(default)]
    region: Option<String>,
    #[serde(default)]
    realm: Option<String>,
}

#[derive(Debug, Deserialize)]
struct RawRealm {
    name: String,
}

#[derive(Debug, Deserialize)]
struct RawBorder {
    hex_x: i32,
    hex_y: i32,
    borders: Vec<u8>,
}

/// Reads a Hexroll Backpack file from disk and returns a populated
/// `HexrollSnapshot`.  The loader opens the SQLite database, extracts the map
/// JSON and parses it, then loads all HTML entity pages and search
/// references.  Errors are propagated via the `anyhow` crate.
pub fn load_snapshot(path: impl AsRef<std::path::Path>) -> Result<HexrollSnapshot> {
    let conn = Connection::open(path.as_ref())
        .with_context(|| format!("failed to open HBF file at {}", path.as_ref().display()))?;

    // Read the map JSON.  There should be exactly one row whose uuid is "map".
    let map_json: String = conn
        .query_row(
            "SELECT value FROM Entities WHERE uuid = 'map'",
            [],
            |row| row.get(0),
        )
        .context("map entry not found in Entities table")?;
    let raw_map: RawMap = serde_json::from_str(&map_json)
        .context("failed to parse map JSON from Entities.value")?;
    // Convert raw map into our strongly typed structures.
    let tiles = raw_map
        .map
        .into_iter()
        .map(|raw| MapTile {
            x: raw.x,
            y: raw.y,
            biome: raw.type_,
            uuid: raw.uuid,
            feature: raw.feature,
            feature_uuid: raw.feature_uuid,
            rivers: raw.rivers,
            trails: raw.trails,
            region: raw.region,
            realm: raw.realm,
        })
        .collect();
    let realms = raw_map
        .realms
        .into_iter()
        .map(|(k, v)| (k, v.name))
        .collect();
    let borders = raw_map
        .borders
        .into_iter()
        .map(|(realm, entries)| {
            let converted = entries
                .into_iter()
                .map(|e| BorderEntry {
                    hex_x: e.hex_x,
                    hex_y: e.hex_y,
                    borders: e.borders,
                })
                .collect::<Vec<_>>();
            (realm, converted)
        })
        .collect();
    let map_data = MapData {
        tiles,
        realms,
        regions: raw_map.regions,
        borders,
    };

    // Load all HTML entities except the map entry.  We avoid streaming all
    // 70k rows into memory if possible by reading in chunks.
    let mut stmt = conn.prepare("SELECT uuid, value FROM Entities WHERE uuid != 'map'")?;
    let entity_iter = stmt.query_map([], |row| {
        let uuid: String = row.get(0)?;
        let value: String = row.get(1)?;
        Ok((uuid, value))
    })?;
    let mut entities = HashMap::new();
    for res in entity_iter {
        let (uuid, value) = res?;
        entities.insert(uuid, value);
    }

    // Load search references.
    let mut ref_stmt = conn.prepare("SELECT value, details, uuid, type, icon, anchor FROM Refs")?;
    let ref_iter = ref_stmt.query_map([], |row| build_ref_record(row))?;
    let mut refs = Vec::new();
    for res in ref_iter {
        refs.push(res?);
    }

    Ok(HexrollSnapshot {
        map: map_data,
        entities,
        refs,
    })
}

fn build_ref_record(row: &Row<'_>) -> rusqlite::Result<RefRecord> {
    Ok(RefRecord {
        value: row.get(0)?,
        details: row.get::<_, Option<String>>(1)?,
        uuid: row.get(2)?,
        entity_type: row.get::<_, Option<String>>(3)?,
        icon: row.get::<_, Option<String>>(4)?,
        anchor: row.get::<_, Option<String>>(5)?,
    })
}

/// A Bevy resource that stores the loaded Hexroll snapshot.  Resources live
/// globally in the ECS and can be accessed by any system via a `Res<T>` or
/// `ResMut<T>` parameter.
#[derive(Resource, Debug)]
pub struct HexrollResource(pub HexrollSnapshot);

/// A component attached to each spawned tile entity.  Storing the UUID makes
/// it easy to look up additional information or link back to the HTML page.
#[derive(Component, Debug, Clone)]
pub struct HexTileComponent {
    pub uuid: String,
    pub feature_uuid: Option<String>,
    pub biome: String,
    pub region: Option<String>,
    pub realm: Option<String>,
    pub rivers: Vec<u8>,
    pub trails: Vec<u8>,
}

/// A Bevy plugin that loads a Hexroll snapshot at startup and spawns an entity
/// for each hex tile.  It inserts a `HexrollResource` containing the entire
/// snapshot, so other systems can access the raw HTML or search references.
pub struct HexrollPlugin<P: AsRef<std::path::Path> + Send + Sync + 'static>(pub P);

impl<P: AsRef<std::path::Path> + Send + Sync + 'static> Plugin for HexrollPlugin<P> {
    fn build(&self, app: &mut App) {
        let path = self.0.as_ref().to_owned();
        app.add_startup_system(move |mut commands: Commands| {
            match load_snapshot(&path) {
                Ok(snapshot) => {
                    let map = snapshot.map.tiles.clone();
                    commands.insert_resource(HexrollResource(snapshot));
                    for tile in map {
                        commands.spawn((
                            HexTileComponent {
                                uuid: tile.uuid.clone(),
                                feature_uuid: tile.feature_uuid.clone(),
                                biome: tile.biome.clone(),
                                region: tile.region.clone(),
                                realm: tile.realm.clone(),
                                rivers: tile.rivers.clone(),
                                trails: tile.trails.clone(),
                            },
                        ));
                    }
                }
                Err(err) => {
                    error!("Failed to load Hexroll snapshot: {}", err);
                }
            }
        });
    }
}