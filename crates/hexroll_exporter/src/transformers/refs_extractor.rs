//! Extracts and processes Refs table data (locations, hexes, factions)
//!
//! The Refs table contains 1,570 reference entries that link entities together:
//! - 920 locations (settlements, dungeons, points of interest)
//! - 645 hexes (overworld map positions)
//! - 5 factions (organizations, groups)

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefEntry {
    pub uuid: String,
    pub entity_uuid: String,
    pub ref_type: RefType,
    pub metadata: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RefType {
    Location,
    Hex,
    Faction,
    Unknown(String),
}

/// Extracts references from the Refs table
pub struct RefsExtractor {
    locations: Vec<RefEntry>,
    hexes: Vec<RefEntry>,
    factions: Vec<RefEntry>,
    unknown: Vec<RefEntry>,
}

impl RefsExtractor {
    pub fn new() -> Self {
        Self {
            locations: Vec::new(),
            hexes: Vec::new(),
            factions: Vec::new(),
            unknown: Vec::new(),
        }
    }

    /// Process raw Refs table data
    pub fn process(&mut self, refs: Vec<Value>) -> RefsExtractionResult {
        for ref_value in refs {
            if let Some(ref_entry) = self.parse_ref(&ref_value) {
                match &ref_entry.ref_type {
                    RefType::Location => self.locations.push(ref_entry),
                    RefType::Hex => self.hexes.push(ref_entry),
                    RefType::Faction => self.factions.push(ref_entry),
                    RefType::Unknown(_) => self.unknown.push(ref_entry),
                }
            }
        }

        RefsExtractionResult {
            locations: self.locations.clone(),
            hexes: self.hexes.clone(),
            factions: self.factions.clone(),
            unknown: self.unknown.clone(),
            total_refs: self.locations.len() + self.hexes.len() + 
                       self.factions.len() + self.unknown.len(),
        }
    }

    /// Parse a single Ref entry
    fn parse_ref(&self, value: &Value) -> Option<RefEntry> {
        let obj = value.as_object()?;
        
        let uuid = obj.get("uuid")?.as_str()?.to_string();
        let entity_uuid = obj.get("entity_uuid")
            .or_else(|| obj.get("entityUuid"))
            .or_else(|| obj.get("target_uuid"))?
            .as_str()?
            .to_string();

        // Determine ref type based on fields or patterns
        let ref_type = self.determine_ref_type(obj);
        
        // Collect any additional metadata
        let mut metadata = HashMap::new();
        for (key, val) in obj.iter() {
            if key != "uuid" && key != "entity_uuid" && key != "entityUuid" {
                metadata.insert(key.clone(), val.clone());
            }
        }

        Some(RefEntry {
            uuid,
            entity_uuid,
            ref_type,
            metadata,
        })
    }

    /// Determine the type of reference based on its properties
    fn determine_ref_type(&self, obj: &serde_json::Map<String, Value>) -> RefType {
        // Check for specific type indicators
        if let Some(type_field) = obj.get("type").and_then(|v| v.as_str()) {
            return match type_field.to_lowercase().as_str() {
                "location" | "settlement" | "dungeon" | "poi" => RefType::Location,
                "hex" | "tile" | "map" => RefType::Hex,
                "faction" | "organization" | "group" => RefType::Faction,
                other => RefType::Unknown(other.to_string()),
            };
        }

        // Check for coordinate fields (indicates hex)
        if obj.contains_key("x") && obj.contains_key("y") {
            return RefType::Hex;
        }
        if obj.contains_key("hex_x") || obj.contains_key("hex_y") {
            return RefType::Hex;
        }

        // Check for location indicators
        if obj.contains_key("settlement_name") || obj.contains_key("dungeon_level") {
            return RefType::Location;
        }

        // Check for faction indicators
        if obj.contains_key("faction_name") || obj.contains_key("alignment") {
            return RefType::Faction;
        }

        // Default to location as most common
        RefType::Location
    }

    /// Build entity relationship map
    pub fn build_relationship_map(&self) -> HashMap<String, Vec<String>> {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        
        for location in &self.locations {
            map.entry(location.entity_uuid.clone())
                .or_insert_with(Vec::new)
                .push(location.uuid.clone());
        }
        
        for hex in &self.hexes {
            map.entry(hex.entity_uuid.clone())
                .or_insert_with(Vec::new)
                .push(hex.uuid.clone());
        }
        
        for faction in &self.factions {
            map.entry(faction.entity_uuid.clone())
                .or_insert_with(Vec::new)
                .push(faction.uuid.clone());
        }
        
        map
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct RefsExtractionResult {
    pub locations: Vec<RefEntry>,
    pub hexes: Vec<RefEntry>,
    pub factions: Vec<RefEntry>,
    pub unknown: Vec<RefEntry>,
    pub total_refs: usize,
}

impl RefsExtractionResult {
    /// Generate a summary report
    pub fn summary(&self) -> String {
        format!(
            "Refs Extraction Summary:\n\
             - Locations: {} (settlements, dungeons, POIs)\n\
             - Hexes: {} (overworld map tiles)\n\
             - Factions: {} (organizations, groups)\n\
             - Unknown: {}\n\
             - Total: {} references",
            self.locations.len(),
            self.hexes.len(),
            self.factions.len(),
            self.unknown.len(),
            self.total_refs
        )
    }

    /// Get all entity UUIDs referenced
    pub fn get_referenced_entities(&self) -> Vec<String> {
        let mut entities = Vec::new();
        
        for loc in &self.locations {
            entities.push(loc.entity_uuid.clone());
        }
        for hex in &self.hexes {
            entities.push(hex.entity_uuid.clone());
        }
        for faction in &self.factions {
            entities.push(faction.entity_uuid.clone());
        }
        
        entities.sort();
        entities.dedup();
        entities
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_parse_location_ref() {
        let extractor = RefsExtractor::new();
        
        let ref_data = json!({
            "uuid": "ref-123",
            "entity_uuid": "entity-456",
            "type": "settlement",
            "settlement_name": "Riverside Village"
        });
        
        let parsed = extractor.parse_ref(&ref_data).unwrap();
        assert_eq!(parsed.uuid, "ref-123");
        assert_eq!(parsed.entity_uuid, "entity-456");
        assert_eq!(parsed.ref_type, RefType::Location);
    }

    #[test]
    fn test_parse_hex_ref() {
        let extractor = RefsExtractor::new();
        
        let ref_data = json!({
            "uuid": "hex-ref-1",
            "entity_uuid": "map-entity-1",
            "x": 10,
            "y": 20
        });
        
        let parsed = extractor.parse_ref(&ref_data).unwrap();
        assert_eq!(parsed.ref_type, RefType::Hex);
    }

    #[test]
    fn test_process_mixed_refs() {
        let mut extractor = RefsExtractor::new();
        
        let refs = vec![
            json!({
                "uuid": "loc-1",
                "entity_uuid": "entity-1",
                "type": "settlement"
            }),
            json!({
                "uuid": "hex-1",
                "entity_uuid": "entity-2",
                "x": 5,
                "y": 10
            }),
            json!({
                "uuid": "faction-1",
                "entity_uuid": "entity-3",
                "type": "faction",
                "faction_name": "The Order"
            }),
        ];
        
        let result = extractor.process(refs);
        assert_eq!(result.locations.len(), 1);
        assert_eq!(result.hexes.len(), 1);
        assert_eq!(result.factions.len(), 1);
        assert_eq!(result.total_refs, 3);
    }
}
