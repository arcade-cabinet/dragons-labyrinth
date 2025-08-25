//! Parses JSON entities containing overworld map data
//!
//! These 47 JSON entities contain the hex grid map data with terrain types,
//! points of interest, and connections between locations.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapData {
    pub uuid: String,
    pub realm_name: Option<String>,
    pub grid_type: GridType,
    pub hexes: Vec<HexTile>,
    pub metadata: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GridType {
    Hexagonal,
    Square,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HexTile {
    pub x: i32,
    pub y: i32,
    pub terrain: TerrainType,
    pub features: Vec<String>,
    pub location_uuid: Option<String>,
    pub connections: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TerrainType {
    Plains,
    Forest,
    Mountains,
    Hills,
    Swamp,
    Desert,
    Water,
    Tundra,
    Wasteland,
    Unknown(String),
}

/// Parses JSON entities containing map data
pub struct JsonParser {
    maps: Vec<MapData>,
    hex_count: usize,
    terrain_stats: HashMap<String, usize>,
}

impl JsonParser {
    pub fn new() -> Self {
        Self {
            maps: Vec::new(),
            hex_count: 0,
            terrain_stats: HashMap::new(),
        }
    }

    /// Process JSON entities
    pub fn process(&mut self, entities: Vec<Value>) -> JsonParsingResult {
        for entity in entities {
            if let Some(json_content) = self.extract_json_content(&entity) {
                if let Some(map_data) = self.parse_map_data(&entity, &json_content) {
                    self.hex_count += map_data.hexes.len();
                    
                    // Track terrain statistics
                    for hex in &map_data.hexes {
                        let terrain_name = format!("{:?}", hex.terrain);
                        *self.terrain_stats.entry(terrain_name).or_insert(0) += 1;
                    }
                    
                    self.maps.push(map_data);
                }
            }
        }

        JsonParsingResult {
            maps: self.maps.clone(),
            total_hexes: self.hex_count,
            terrain_distribution: self.terrain_stats.clone(),
            realm_count: self.count_unique_realms(),
        }
    }

    /// Extract JSON content from entity
    fn extract_json_content(&self, entity: &Value) -> Option<Value> {
        let obj = entity.as_object()?;
        
        // Look for JSON content in various possible fields
        if let Some(content) = obj.get("json_content") {
            return Some(content.clone());
        }
        if let Some(content) = obj.get("content") {
            if let Some(content_str) = content.as_str() {
                // Try to parse as JSON
                if let Ok(parsed) = serde_json::from_str::<Value>(content_str) {
                    return Some(parsed);
                }
            }
            return Some(content.clone());
        }
        if let Some(data) = obj.get("data") {
            return Some(data.clone());
        }
        
        // Check if the entire entity is the JSON data
        if obj.contains_key("hexes") || obj.contains_key("grid") || obj.contains_key("map") {
            return Some(entity.clone());
        }
        
        None
    }

    /// Parse map data from JSON content
    fn parse_map_data(&self, entity: &Value, content: &Value) -> Option<MapData> {
        let entity_obj = entity.as_object()?;
        let uuid = entity_obj.get("uuid")?.as_str()?.to_string();
        
        // Extract realm name if present
        let realm_name = content.get("realm")
            .or_else(|| content.get("realm_name"))
            .or_else(|| content.get("name"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        
        // Determine grid type
        let grid_type = self.determine_grid_type(content);
        
        // Parse hex tiles
        let hexes = self.parse_hexes(content)?;
        
        // Collect metadata
        let mut metadata = HashMap::new();
        if let Some(obj) = content.as_object() {
            for (key, val) in obj.iter() {
                if key != "hexes" && key != "grid" && key != "tiles" {
                    metadata.insert(key.clone(), val.clone());
                }
            }
        }
        
        Some(MapData {
            uuid,
            realm_name,
            grid_type,
            hexes,
            metadata,
        })
    }

    /// Determine the grid type from content
    fn determine_grid_type(&self, content: &Value) -> GridType {
        if let Some(grid_type) = content.get("grid_type").and_then(|v| v.as_str()) {
            return match grid_type.to_lowercase().as_str() {
                "hex" | "hexagonal" => GridType::Hexagonal,
                "square" | "grid" => GridType::Square,
                _ => GridType::Unknown,
            };
        }
        
        // Default to hexagonal for HexRoll data
        GridType::Hexagonal
    }

    /// Parse hex tiles from various possible formats
    fn parse_hexes(&self, content: &Value) -> Option<Vec<HexTile>> {
        // Try different field names
        let hex_data = content.get("hexes")
            .or_else(|| content.get("tiles"))
            .or_else(|| content.get("grid"))
            .or_else(|| content.get("map"))?;
        
        let mut hexes = Vec::new();
        
        // Handle array of hexes
        if let Some(hex_array) = hex_data.as_array() {
            for hex_value in hex_array {
                if let Some(hex) = self.parse_single_hex(hex_value) {
                    hexes.push(hex);
                }
            }
        }
        // Handle object with coordinate keys (e.g., {"0,0": {...}, "0,1": {...}})
        else if let Some(hex_obj) = hex_data.as_object() {
            for (coord, hex_value) in hex_obj.iter() {
                if let Some(hex) = self.parse_hex_from_coord(coord, hex_value) {
                    hexes.push(hex);
                }
            }
        }
        
        if hexes.is_empty() {
            None
        } else {
            Some(hexes)
        }
    }

    /// Parse a single hex tile
    fn parse_single_hex(&self, value: &Value) -> Option<HexTile> {
        let obj = value.as_object()?;
        
        let x = obj.get("x")?.as_i64()? as i32;
        let y = obj.get("y")?.as_i64()? as i32;
        
        let terrain = self.parse_terrain(obj);
        let features = self.parse_features(obj);
        let location_uuid = obj.get("location")
            .or_else(|| obj.get("location_uuid"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let connections = self.parse_connections(obj);
        
        Some(HexTile {
            x,
            y,
            terrain,
            features,
            location_uuid,
            connections,
        })
    }

    /// Parse hex from coordinate string key
    fn parse_hex_from_coord(&self, coord: &str, value: &Value) -> Option<HexTile> {
        // Parse coordinates from string like "10,20" or "x10y20"
        let (x, y) = if coord.contains(',') {
            let parts: Vec<&str> = coord.split(',').collect();
            if parts.len() == 2 {
                (parts[0].parse::<i32>().ok()?, parts[1].parse::<i32>().ok()?)
            } else {
                return None;
            }
        } else if coord.contains('x') && coord.contains('y') {
            // Handle format like "x10y20"
            let coord_lower = coord.to_lowercase();
            let x_pos = coord_lower.find('x')? + 1;
            let y_pos = coord_lower.find('y')?;
            let x = coord_lower[x_pos..y_pos].parse::<i32>().ok()?;
            let y = coord_lower[y_pos + 1..].parse::<i32>().ok()?;
            (x, y)
        } else {
            return None;
        };
        
        let obj = value.as_object()?;
        let terrain = self.parse_terrain(obj);
        let features = self.parse_features(obj);
        let location_uuid = obj.get("location")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let connections = self.parse_connections(obj);
        
        Some(HexTile {
            x,
            y,
            terrain,
            features,
            location_uuid,
            connections,
        })
    }

    /// Parse terrain type
    fn parse_terrain(&self, obj: &serde_json::Map<String, Value>) -> TerrainType {
        if let Some(terrain_str) = obj.get("terrain").and_then(|v| v.as_str()) {
            match terrain_str.to_lowercase().as_str() {
                "plains" | "grassland" => TerrainType::Plains,
                "forest" | "woods" => TerrainType::Forest,
                "mountains" | "mountain" => TerrainType::Mountains,
                "hills" | "hill" => TerrainType::Hills,
                "swamp" | "marsh" => TerrainType::Swamp,
                "desert" | "sand" => TerrainType::Desert,
                "water" | "ocean" | "sea" | "lake" => TerrainType::Water,
                "tundra" | "snow" | "ice" => TerrainType::Tundra,
                "wasteland" | "waste" => TerrainType::Wasteland,
                other => TerrainType::Unknown(other.to_string()),
            }
        } else {
            TerrainType::Unknown("unspecified".to_string())
        }
    }

    /// Parse hex features
    fn parse_features(&self, obj: &serde_json::Map<String, Value>) -> Vec<String> {
        let mut features = Vec::new();
        
        if let Some(features_value) = obj.get("features") {
            if let Some(features_array) = features_value.as_array() {
                for feature in features_array {
                    if let Some(feature_str) = feature.as_str() {
                        features.push(feature_str.to_string());
                    }
                }
            } else if let Some(feature_str) = features_value.as_str() {
                features.push(feature_str.to_string());
            }
        }
        
        // Check for individual feature fields
        if obj.get("road").and_then(|v| v.as_bool()).unwrap_or(false) {
            features.push("road".to_string());
        }
        if obj.get("river").and_then(|v| v.as_bool()).unwrap_or(false) {
            features.push("river".to_string());
        }
        if obj.get("poi").is_some() {
            features.push("point_of_interest".to_string());
        }
        
        features
    }

    /// Parse hex connections
    fn parse_connections(&self, obj: &serde_json::Map<String, Value>) -> Vec<String> {
        let mut connections = Vec::new();
        
        if let Some(conn_value) = obj.get("connections") {
            if let Some(conn_array) = conn_value.as_array() {
                for conn in conn_array {
                    if let Some(conn_str) = conn.as_str() {
                        connections.push(conn_str.to_string());
                    }
                }
            }
        }
        
        connections
    }

    /// Count unique realms
    fn count_unique_realms(&self) -> usize {
        let mut realms = Vec::new();
        for map in &self.maps {
            if let Some(realm) = &map.realm_name {
                if !realms.contains(realm) {
                    realms.push(realm.clone());
                }
            }
        }
        realms.len()
    }
}

#[derive(Debug, Clone)]
pub struct JsonParsingResult {
    pub maps: Vec<MapData>,
    pub total_hexes: usize,
    pub terrain_distribution: HashMap<String, usize>,
    pub realm_count: usize,
}

impl JsonParsingResult {
    /// Generate a summary report
    pub fn summary(&self) -> String {
        let mut terrain_summary = String::new();
        for (terrain, count) in &self.terrain_distribution {
            terrain_summary.push_str(&format!("    - {}: {}\n", terrain, count));
        }
        
        format!(
            "JSON Map Parsing Summary:\n\
             - Maps processed: {}\n\
             - Total hexes: {}\n\
             - Unique realms: {}\n\
             - Terrain distribution:\n{}",
            self.maps.len(),
            self.total_hexes,
            self.realm_count,
            terrain_summary
        )
    }

    /// Get all location UUIDs referenced in maps
    pub fn get_location_references(&self) -> Vec<String> {
        let mut locations = Vec::new();
        
        for map in &self.maps {
            for hex in &map.hexes {
                if let Some(loc) = &hex.location_uuid {
                    if !locations.contains(loc) {
                        locations.push(loc.clone());
                    }
                }
            }
        }
        
        locations
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_parse_hex_array() {
        let mut parser = JsonParser::new();
        
        let entity = json!({
            "uuid": "map-1",
            "hexes": [
                {
                    "x": 0,
                    "y": 0,
                    "terrain": "plains",
                    "features": ["road"]
                },
                {
                    "x": 1,
                    "y": 0,
                    "terrain": "forest"
                }
            ]
        });
        
        let result = parser.process(vec![entity]);
        assert_eq!(result.maps.len(), 1);
        assert_eq!(result.total_hexes, 2);
    }

    #[test]
    fn test_parse_coordinate_map() {
        let mut parser = JsonParser::new();
        
        let entity = json!({
            "uuid": "map-2",
            "grid": {
                "0,0": {
                    "terrain": "mountains",
                    "location": "dungeon-1"
                },
                "1,0": {
                    "terrain": "hills"
                }
            }
        });
        
        let result = parser.process(vec![entity]);
        assert_eq!(result.maps.len(), 1);
        assert_eq!(result.total_hexes, 2);
        
        let locations = result.get_location_references();
        assert_eq!(locations.len(), 1);
        assert_eq!(locations[0], "dungeon-1");
    }
}
