//! Specialized parser for dungeon content
//!
//! Focuses on extracting dungeon-specific data like rooms, encounters,
//! traps, treasures, and the connections between them.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DungeonData {
    pub uuid: String,
    pub name: String,
    pub levels: Vec<DungeonLevel>,
    pub total_rooms: usize,
    pub entrance: Option<String>,
    pub boss_rooms: Vec<String>,
    pub treasure_value: i32,
    pub danger_level: i32,
    pub theme: DungeonTheme,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DungeonLevel {
    pub level_number: i32,
    pub rooms: Vec<DungeonRoom>,
    pub connections: Vec<RoomConnection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DungeonRoom {
    pub id: String,
    pub name: String,
    pub description: String,
    pub room_type: RoomType,
    pub encounters: Vec<Encounter>,
    pub traps: Vec<Trap>,
    pub treasure: Vec<Treasure>,
    pub features: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RoomType {
    Entrance,
    Corridor,
    Chamber,
    BossRoom,
    TreasureRoom,
    Trap,
    Shrine,
    Prison,
    Library,
    Laboratory,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomConnection {
    pub from_room: String,
    pub to_room: String,
    pub connection_type: ConnectionType,
    pub locked: bool,
    pub hidden: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConnectionType {
    Door,
    Passage,
    SecretDoor,
    Stairs,
    Pit,
    Teleporter,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Encounter {
    pub encounter_type: String,
    pub creatures: Vec<String>,
    pub difficulty: String,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trap {
    pub trap_type: String,
    pub trigger: String,
    pub effect: String,
    pub detection_dc: Option<i32>,
    pub disable_dc: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Treasure {
    pub item_type: String,
    pub description: String,
    pub value: Option<i32>,
    pub magical: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DungeonTheme {
    AncientRuins,
    UndeadCrypt,
    DragonLair,
    ElementalTemple,
    DemonPortal,
    AbandonedMine,
    SunkenCitadel,
    MadWizardTower,
    Unknown,
}

/// Specialized parser for dungeon content
pub struct DungeonParser {
    dungeons: Vec<DungeonData>,
    room_count: usize,
    encounter_count: usize,
    trap_count: usize,
    treasure_count: usize,
}

impl DungeonParser {
    pub fn new() -> Self {
        Self {
            dungeons: Vec::new(),
            room_count: 0,
            encounter_count: 0,
            trap_count: 0,
            treasure_count: 0,
        }
    }

    /// Process entities that contain dungeon data
    pub fn process(&mut self, entities: Vec<Value>) -> DungeonParsingResult {
        for entity in entities {
            if let Some(dungeon) = self.parse_dungeon_entity(&entity) {
                self.room_count += dungeon.total_rooms;
                
                // Count encounters, traps, treasures
                for level in &dungeon.levels {
                    for room in &level.rooms {
                        self.encounter_count += room.encounters.len();
                        self.trap_count += room.traps.len();
                        self.treasure_count += room.treasure.len();
                    }
                }
                
                self.dungeons.push(dungeon);
            }
        }

        DungeonParsingResult {
            dungeons: self.dungeons.clone(),
            total_rooms: self.room_count,
            total_encounters: self.encounter_count,
            total_traps: self.trap_count,
            total_treasure: self.treasure_count,
            themes: self.get_unique_themes(),
        }
    }

    /// Parse a single entity into dungeon data
    fn parse_dungeon_entity(&self, entity: &Value) -> Option<DungeonData> {
        let obj = entity.as_object()?;
        let uuid = obj.get("uuid")?.as_str()?.to_string();
        
        // Check if this is a dungeon entity
        if !self.is_dungeon_entity(obj) {
            return None;
        }
        
        let name = self.extract_dungeon_name(obj);
        let theme = self.determine_theme(obj);
        let levels = self.parse_dungeon_levels(obj);
        
        let total_rooms = levels.iter().map(|l| l.rooms.len()).sum();
        let entrance = self.find_entrance_room(&levels);
        let boss_rooms = self.find_boss_rooms(&levels);
        let treasure_value = self.calculate_treasure_value(&levels);
        let danger_level = self.calculate_danger_level(&levels);
        
        Some(DungeonData {
            uuid,
            name,
            levels,
            total_rooms,
            entrance,
            boss_rooms,
            treasure_value,
            danger_level,
            theme,
        })
    }

    /// Check if entity contains dungeon data
    fn is_dungeon_entity(&self, obj: &serde_json::Map<String, Value>) -> bool {
        // Check for dungeon indicators
        if let Some(content) = obj.get("content").and_then(|v| v.as_str()) {
            let content_lower = content.to_lowercase();
            return content_lower.contains("dungeon") || 
                   content_lower.contains("chamber") ||
                   content_lower.contains("corridor") ||
                   content_lower.contains("trap");
        }
        
        // Check for dungeon-specific fields
        obj.contains_key("rooms") || 
        obj.contains_key("levels") ||
        obj.contains_key("dungeon_name") ||
        obj.contains_key("dungeon_level")
    }

    /// Extract dungeon name
    fn extract_dungeon_name(&self, obj: &serde_json::Map<String, Value>) -> String {
        if let Some(name) = obj.get("name").and_then(|v| v.as_str()) {
            return name.to_string();
        }
        if let Some(name) = obj.get("dungeon_name").and_then(|v| v.as_str()) {
            return name.to_string();
        }
        if let Some(title) = obj.get("title").and_then(|v| v.as_str()) {
            return title.to_string();
        }
        
        "Unknown Dungeon".to_string()
    }

    /// Determine dungeon theme
    fn determine_theme(&self, obj: &serde_json::Map<String, Value>) -> DungeonTheme {
        let content = obj.values()
            .filter_map(|v| v.as_str())
            .collect::<Vec<_>>()
            .join(" ")
            .to_lowercase();
        
        if content.contains("undead") || content.contains("crypt") || content.contains("tomb") {
            DungeonTheme::UndeadCrypt
        } else if content.contains("dragon") || content.contains("wyrm") {
            DungeonTheme::DragonLair
        } else if content.contains("demon") || content.contains("infernal") || content.contains("hell") {
            DungeonTheme::DemonPortal
        } else if content.contains("ancient") || content.contains("ruin") {
            DungeonTheme::AncientRuins
        } else if content.contains("elemental") || content.contains("temple") {
            DungeonTheme::ElementalTemple
        } else if content.contains("mine") || content.contains("mining") {
            DungeonTheme::AbandonedMine
        } else if content.contains("sunken") || content.contains("underwater") {
            DungeonTheme::SunkenCitadel
        } else if content.contains("wizard") || content.contains("mage") || content.contains("tower") {
            DungeonTheme::MadWizardTower
        } else {
            DungeonTheme::Unknown
        }
    }

    /// Parse dungeon levels
    fn parse_dungeon_levels(&self, obj: &serde_json::Map<String, Value>) -> Vec<DungeonLevel> {
        let mut levels = Vec::new();
        
        // Check for explicit levels structure
        if let Some(levels_value) = obj.get("levels") {
            if let Some(levels_array) = levels_value.as_array() {
                for (i, level_value) in levels_array.iter().enumerate() {
                    if let Some(level) = self.parse_single_level(level_value, i as i32 + 1) {
                        levels.push(level);
                    }
                }
            }
        }
        
        // If no levels, try to parse as single level
        if levels.is_empty() {
            if let Some(level) = self.parse_single_level(&Value::Object(obj.clone()), 1) {
                levels.push(level);
            }
        }
        
        levels
    }

    /// Parse a single dungeon level
    fn parse_single_level(&self, value: &Value, level_number: i32) -> Option<DungeonLevel> {
        let obj = value.as_object()?;
        
        let rooms = self.parse_rooms(obj);
        let connections = self.parse_connections(obj, &rooms);
        
        if rooms.is_empty() {
            return None;
        }
        
        Some(DungeonLevel {
            level_number,
            rooms,
            connections,
        })
    }

    /// Parse rooms from level data
    fn parse_rooms(&self, obj: &serde_json::Map<String, Value>) -> Vec<DungeonRoom> {
        let mut rooms = Vec::new();
        
        // Check for explicit rooms array
        if let Some(rooms_value) = obj.get("rooms") {
            if let Some(rooms_array) = rooms_value.as_array() {
                for room_value in rooms_array {
                    if let Some(room) = self.parse_single_room(room_value) {
                        rooms.push(room);
                    }
                }
            }
        }
        
        // Parse from content if no explicit rooms
        if rooms.is_empty() {
            if let Some(content) = obj.get("content").and_then(|v| v.as_str()) {
                rooms = self.parse_rooms_from_text(content);
            }
        }
        
        rooms
    }

    /// Parse a single room
    fn parse_single_room(&self, value: &Value) -> Option<DungeonRoom> {
        let obj = value.as_object()?;
        
        let id = obj.get("id")
            .or_else(|| obj.get("room_id"))
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();
            
        let name = obj.get("name")
            .or_else(|| obj.get("room_name"))
            .and_then(|v| v.as_str())
            .unwrap_or("Unnamed Room")
            .to_string();
            
        let description = obj.get("description")
            .or_else(|| obj.get("desc"))
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
            
        let room_type = self.determine_room_type(obj);
        let encounters = self.parse_encounters(obj);
        let traps = self.parse_traps(obj);
        let treasure = self.parse_treasure(obj);
        let features = self.parse_features(obj);
        
        Some(DungeonRoom {
            id,
            name,
            description,
            room_type,
            encounters,
            traps,
            treasure,
            features,
        })
    }

    /// Parse rooms from text content
    fn parse_rooms_from_text(&self, content: &str) -> Vec<DungeonRoom> {
        let mut rooms = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        
        for (i, line) in lines.iter().enumerate() {
            if line.contains("Room") || line.contains("Chamber") || line.contains("Corridor") {
                let room = DungeonRoom {
                    id: format!("room_{}", i),
                    name: line.trim().to_string(),
                    description: lines.get(i + 1).unwrap_or(&"").to_string(),
                    room_type: self.determine_room_type_from_text(line),
                    encounters: Vec::new(),
                    traps: Vec::new(),
                    treasure: Vec::new(),
                    features: Vec::new(),
                };
                rooms.push(room);
            }
        }
        
        rooms
    }

    /// Determine room type from object
    fn determine_room_type(&self, obj: &serde_json::Map<String, Value>) -> RoomType {
        if let Some(room_type) = obj.get("room_type").and_then(|v| v.as_str()) {
            return self.parse_room_type_string(room_type);
        }
        
        let content = obj.values()
            .filter_map(|v| v.as_str())
            .collect::<Vec<_>>()
            .join(" ")
            .to_lowercase();
            
        self.determine_room_type_from_text(&content)
    }

    /// Determine room type from text
    fn determine_room_type_from_text(&self, text: &str) -> RoomType {
        let text_lower = text.to_lowercase();
        
        if text_lower.contains("entrance") || text_lower.contains("entry") {
            RoomType::Entrance
        } else if text_lower.contains("corridor") || text_lower.contains("hallway") {
            RoomType::Corridor
        } else if text_lower.contains("boss") || text_lower.contains("final") {
            RoomType::BossRoom
        } else if text_lower.contains("treasure") || text_lower.contains("vault") {
            RoomType::TreasureRoom
        } else if text_lower.contains("trap") {
            RoomType::Trap
        } else if text_lower.contains("shrine") || text_lower.contains("altar") {
            RoomType::Shrine
        } else if text_lower.contains("prison") || text_lower.contains("cell") {
            RoomType::Prison
        } else if text_lower.contains("library") || text_lower.contains("archive") {
            RoomType::Library
        } else if text_lower.contains("laboratory") || text_lower.contains("workshop") {
            RoomType::Laboratory
        } else if text_lower.contains("chamber") {
            RoomType::Chamber
        } else {
            RoomType::Unknown
        }
    }

    /// Parse room type string
    fn parse_room_type_string(&self, s: &str) -> RoomType {
        match s.to_lowercase().as_str() {
            "entrance" => RoomType::Entrance,
            "corridor" => RoomType::Corridor,
            "chamber" => RoomType::Chamber,
            "boss" | "boss_room" => RoomType::BossRoom,
            "treasure" | "treasure_room" => RoomType::TreasureRoom,
            "trap" => RoomType::Trap,
            "shrine" => RoomType::Shrine,
            "prison" => RoomType::Prison,
            "library" => RoomType::Library,
            "laboratory" | "lab" => RoomType::Laboratory,
            _ => RoomType::Unknown,
        }
    }

    /// Parse encounters
    fn parse_encounters(&self, obj: &serde_json::Map<String, Value>) -> Vec<Encounter> {
        let mut encounters = Vec::new();
        
        if let Some(enc_value) = obj.get("encounters") {
            if let Some(enc_array) = enc_value.as_array() {
                for enc in enc_array {
                    if let Some(encounter) = self.parse_single_encounter(enc) {
                        encounters.push(encounter);
                    }
                }
            }
        }
        
        encounters
    }

    /// Parse single encounter
    fn parse_single_encounter(&self, value: &Value) -> Option<Encounter> {
        let obj = value.as_object()?;
        
        Some(Encounter {
            encounter_type: obj.get("type")?.as_str()?.to_string(),
            creatures: obj.get("creatures")
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
                .unwrap_or_default(),
            difficulty: obj.get("difficulty")
                .and_then(|v| v.as_str())
                .unwrap_or("medium")
                .to_string(),
            notes: obj.get("notes").and_then(|v| v.as_str()).map(|s| s.to_string()),
        })
    }

    /// Parse traps
    fn parse_traps(&self, obj: &serde_json::Map<String, Value>) -> Vec<Trap> {
        let mut traps = Vec::new();
        
        if let Some(traps_value) = obj.get("traps") {
            if let Some(traps_array) = traps_value.as_array() {
                for trap in traps_array {
                    if let Some(parsed_trap) = self.parse_single_trap(trap) {
                        traps.push(parsed_trap);
                    }
                }
            }
        }
        
        traps
    }

    /// Parse single trap
    fn parse_single_trap(&self, value: &Value) -> Option<Trap> {
        let obj = value.as_object()?;
        
        Some(Trap {
            trap_type: obj.get("type")?.as_str()?.to_string(),
            trigger: obj.get("trigger")?.as_str()?.to_string(),
            effect: obj.get("effect")?.as_str()?.to_string(),
            detection_dc: obj.get("detection_dc").and_then(|v| v.as_i64()).map(|n| n as i32),
            disable_dc: obj.get("disable_dc").and_then(|v| v.as_i64()).map(|n| n as i32),
        })
    }

    /// Parse treasure
    fn parse_treasure(&self, obj: &serde_json::Map<String, Value>) -> Vec<Treasure> {
        let mut treasures = Vec::new();
        
        if let Some(treasure_value) = obj.get("treasure") {
            if let Some(treasure_array) = treasure_value.as_array() {
                for item in treasure_array {
                    if let Some(treasure) = self.parse_single_treasure(item) {
                        treasures.push(treasure);
                    }
                }
            }
        }
        
        treasures
    }

    /// Parse single treasure item
    fn parse_single_treasure(&self, value: &Value) -> Option<Treasure> {
        let obj = value.as_object()?;
        
        Some(Treasure {
            item_type: obj.get("type")?.as_str()?.to_string(),
            description: obj.get("description")?.as_str()?.to_string(),
            value: obj.get("value").and_then(|v| v.as_i64()).map(|n| n as i32),
            magical: obj.get("magical").and_then(|v| v.as_bool()).unwrap_or(false),
        })
    }

    /// Parse room features
    fn parse_features(&self, obj: &serde_json::Map<String, Value>) -> Vec<String> {
        let mut features = Vec::new();
        
        if let Some(features_value) = obj.get("features") {
            if let Some(features_array) = features_value.as_array() {
                for feature in features_array {
                    if let Some(feature_str) = feature.as_str() {
                        features.push(feature_str.to_string());
                    }
                }
            }
        }
        
        features
    }

    /// Parse room connections
    fn parse_connections(&self, obj: &serde_json::Map<String, Value>, rooms: &[DungeonRoom]) -> Vec<RoomConnection> {
        let mut connections = Vec::new();
        
        // Check for explicit connections
        if let Some(conn_value) = obj.get("connections") {
            if let Some(conn_array) = conn_value.as_array() {
                for conn in conn_array {
                    if let Some(connection) = self.parse_single_connection(conn) {
                        connections.push(connection);
                    }
                }
            }
        }
        
        // Infer connections from room descriptions
        for (i, room) in rooms.iter().enumerate() {
            if room.description.contains("north") || room.description.contains("south") ||
               room.description.contains("east") || room.description.contains("west") {
                // Create connections to adjacent rooms
                if i + 1 < rooms.len() {
                    connections.push(RoomConnection {
                        from_room: room.id.clone(),
                        to_room: rooms[i + 1].id.clone(),
                        connection_type: ConnectionType::Passage,
                        locked: false,
                        hidden: false,
                    });
                }
            }
        }
        
        connections
    }

    /// Parse single connection
    fn parse_single_connection(&self, value: &Value) -> Option<RoomConnection> {
        let obj = value.as_object()?;
        
        Some(RoomConnection {
            from_room: obj.get("from")?.as_str()?.to_string(),
            to_room: obj.get("to")?.as_str()?.to_string(),
            connection_type: self.parse_connection_type(obj.get("type").and_then(|v| v.as_str()).unwrap_or("door")),
            locked: obj.get("locked").and_then(|v| v.as_bool()).unwrap_or(false),
            hidden: obj.get("hidden").and_then(|v| v.as_bool()).unwrap_or(false),
        })
    }

    /// Parse connection type
    fn parse_connection_type(&self, s: &str) -> ConnectionType {
        match s.to_lowercase().as_str() {
            "door" => ConnectionType::Door,
            "passage" => ConnectionType::Passage,
            "secret" | "secret_door" => ConnectionType::SecretDoor,
            "stairs" => ConnectionType::Stairs,
            "pit" => ConnectionType::Pit,
            "teleporter" | "portal" => ConnectionType::Teleporter,
            _ => ConnectionType::Passage,
        }
    }

    /// Find entrance room
    fn find_entrance_room(&self, levels: &[DungeonLevel]) -> Option<String> {
        for level in levels {
            for room in &level.rooms {
                if matches!(room.room_type, RoomType::Entrance) {
                    return Some(room.id.clone());
                }
            }
        }
        
        // Return first room if no explicit entrance
        levels.first()?.rooms.first().map(|r| r.id.clone())
    }

    /// Find boss rooms
    fn find_boss_rooms(&self, levels: &[DungeonLevel]) -> Vec<String> {
        let mut boss_rooms = Vec::new();
        
        for level in levels {
            for room in &level.rooms {
                if matches!(room.room_type, RoomType::BossRoom) {
                    boss_rooms.push(room.id.clone());
                }
            }
        }
        
        boss_rooms
    }

    /// Calculate total treasure value
    fn calculate_treasure_value(&self, levels: &[DungeonLevel]) -> i32 {
        let mut total = 0;
        
        for level in levels {
            for room in &level.rooms {
                for treasure in &room.treasure {
                    total += treasure.value.unwrap_or(0);
                }
            }
        }
        
        total
    }

    /// Calculate danger level
    fn calculate_danger_level(&self, levels: &[DungeonLevel]) -> i32 {
        let mut danger = 0;
        
        for level in levels {
            danger += level.level_number; // Higher levels are more dangerous
            
            for room in &level.rooms {
                danger += room.encounters.len() as i32 * 2;
                danger += room.traps.len() as i32 * 3;
                
                if matches!(room.room_type, RoomType::BossRoom) {
                    danger += 10;
                }
            }
        }
        
        danger
    }

    /// Get unique themes
    fn get_unique_themes(&self) -> Vec<DungeonTheme> {
        let mut themes = Vec::new();
        
        for dungeon in &self.dungeons {
            if !themes.iter().any(|t| std::mem::discriminant(t) == std::mem::discriminant(&dungeon.theme)) {
                themes.push(dungeon.theme.clone());
            }
        }
        
        themes
    }
}

#[derive(Debug, Clone)]
pub struct DungeonParsingResult {
    pub dungeons: Vec<DungeonData>,
    pub total_rooms: usize,
    pub total_encounters: usize,
    pub total_traps: usize,
    pub total_treasure: usize,
    pub themes: Vec<DungeonTheme>,
}

impl DungeonParsingResult {
    /// Generate a summary report
    pub fn summary(&self) -> String {
        let themes_str = self.themes.iter()
            .map(|t| format!("{:?}", t))
            .collect::<Vec<_>>()
            .join(", ");
            
        format!(
            "Dungeon Parsing Summary:\n\
             - Dungeons: {}\n\
             - Total Rooms: {}\n\
             - Total Encounters: {}\n\
             - Total Traps: {}\n\
             - Total Treasure Items: {}\n\
             - Themes: {}",
            self.dungeons.len(),
            self.total_rooms,
            self.total_encounters,
            self.total_traps,
            self.total_treasure,
            themes_str
        )
    }

    /// Get the most dangerous dungeon
    pub fn get_most_dangerous(&self) -> Option<&DungeonData> {
        self.dungeons.iter().max_by_key(|d| d.danger_level)
    }

    /// Get the most valuable dungeon
    pub fn get_most_valuable(&self) -> Option<&DungeonData> {
        self.dungeons.iter().max_by_key(|d| d.treasure_value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_parse_simple_dungeon() {
        let mut parser = DungeonParser::new();
        
        let entity = json!({
            "uuid": "dungeon-1",
            "name": "The Dark Crypt",
            "content": "A dungeon with chambers and corridors filled with undead",
            "rooms": [
                {
                    "id": "r1",
                    "name": "Entrance Hall",
                    "room_type": "entrance"
                },
                {
                    "id": "r2",
                    "name": "Boss Chamber",
                    "room_type": "boss_room"
                }
            ]
        });
        
        let result = parser.process(vec![entity]);
        assert_eq!(result.dungeons.len(), 1);
        assert_eq!(result.total_rooms, 2);
        assert_eq!(result.dungeons[0].theme, DungeonTheme::UndeadCrypt);
    }

    #[test]
    fn test_detect_dungeon_theme() {
        let parser = DungeonParser::new();
        let mut obj = serde_json::Map::new();
        
        obj.insert("content".to_string(), json!("Ancient ruins with crumbling walls"));
        assert_eq!(parser.determine_theme(&obj), DungeonTheme::AncientRuins);
        
        obj.insert("content".to_string(), json!("Dragon's lair filled with gold"));
        assert_eq!(parser.determine_theme(&obj), DungeonTheme::DragonLair);
    }
}
