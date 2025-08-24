use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde_json::json;

use crate::loader::{HexrollSnapshot, RefRecord};

/// Export all contents of a Hexroll snapshot into a directory hierarchy.
///
/// The exporter will create the following structure under `out_dir`:
///
/// * `map.json` – the raw map JSON as found in the HBF file.
/// * `entities/` – a directory containing one `.html` file per entity
///    (excluding the `map` entry).  The filename is the entity UUID with a
///    `.html` extension.
/// * `refs.json` – an array of objects representing the search references.
///
/// If the output directory does not exist it will be created.  Existing files
/// will be overwritten.
pub fn export_snapshot(snapshot: &HexrollSnapshot, out_dir: impl AsRef<Path>) -> Result<()> {
    let out_dir = out_dir.as_ref();
    fs::create_dir_all(out_dir).with_context(|| format!("failed to create output directory {}", out_dir.display()))?;
    // Write the map JSON
    let map_json_path = out_dir.join("map.json");
    let map_json = json!({
        "map": snapshot.map.tiles.iter().map(|t| {
            json!({
                "x": t.x,
                "y": t.y,
                "type": t.biome,
                "uuid": t.uuid,
                "feature": t.feature,
                "feature_uuid": t.feature_uuid,
                "rivers": t.rivers,
                "trails": t.trails,
                "region": t.region,
                "realm": t.realm,
            })
        }).collect::<Vec<_>>(),
        "realms": snapshot.map.realms,
        "regions": snapshot.map.regions,
        "borders": snapshot.map.borders.iter().map(|(realm, borders)| {
            let arr = borders.iter().map(|b| {
                json!({
                    "hex_x": b.hex_x,
                    "hex_y": b.hex_y,
                    "borders": b.borders,
                })
            }).collect::<Vec<_>>();
            (realm.clone(), arr)
        }).collect::<serde_json::Map<String, serde_json::Value>>(),
    });
    let mut file = File::create(&map_json_path)
        .with_context(|| format!("failed to create {}", map_json_path.display()))?;
    serde_json::to_writer_pretty(&mut file, &map_json)
        .with_context(|| format!("failed to write {}", map_json_path.display()))?;
    // Entities directory
    let entities_dir = out_dir.join("entities");
    fs::create_dir_all(&entities_dir).with_context(|| format!("failed to create {}", entities_dir.display()))?;
    for (uuid, html) in &snapshot.entities {
        let mut path = entities_dir.join(uuid);
        path.set_extension("html");
        let mut f = File::create(&path)
            .with_context(|| format!("failed to create entity file {}", path.display()))?;
        f.write_all(html.as_bytes())
            .with_context(|| format!("failed to write entity file {}", path.display()))?;
    }
    // Refs JSON
    let refs_json_path = out_dir.join("refs.json");
    let mut refs_vec = Vec::with_capacity(snapshot.refs.len());
    for r in &snapshot.refs {
        refs_vec.push(json!({
            "value": r.value,
            "details": r.details,
            "uuid": r.uuid,
            "type": r.entity_type,
            "icon": r.icon,
            "anchor": r.anchor,
        }));
    }
    let mut rf = File::create(&refs_json_path)
        .with_context(|| format!("failed to create {}", refs_json_path.display()))?;
    serde_json::to_writer_pretty(&mut rf, &refs_vec)
        .with_context(|| format!("failed to write {}", refs_json_path.display()))?;
    Ok(())
}