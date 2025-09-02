//! Base value objects and shared abstractions for analysis package.
//! 
//! Mirrors the Python base.py with modern Rust types and follows .clinerules standards.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Canonical hex key like "W2S51"
pub type HexKey = String;

/// Map coordinate with optional hex ID
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MapCoord {
    pub x: Option<f32>,
    pub y: Option<f32>,
    /// Map hex id when present
    pub hex_id: Option<String>,
}

impl MapCoord {
    pub fn new(x: Option<f32>, y: Option<f32>, hex_id: Option<String>) -> Self {
        Self { x, y, hex_id }
    }
}

/// Edge type enumeration for entity relationships
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum EdgeType {
    #[serde(rename = "settlement_in_hex")]
    SettlementInHex,
    #[serde(rename = "dungeon_in_hex")]
    DungeonInHex,
    #[serde(rename = "area_connects_to_area")]
    AreaConnectsToArea,
    #[serde(rename = "faction_controls_region")]
    FactionControlsRegion,
    #[serde(rename = "faction_controls_settlement")]
    FactionControlsSettlement,
}

impl EdgeType {
    pub fn as_str(&self) -> &'static str {
        match self {
            EdgeType::SettlementInHex => "settlement_in_hex",
            EdgeType::DungeonInHex => "dungeon_in_hex",
            EdgeType::AreaConnectsToArea => "area_connects_to_area",
            EdgeType::FactionControlsRegion => "faction_controls_region",
            EdgeType::FactionControlsSettlement => "faction_controls_settlement",
        }
    }
}

/// Field specification for entity model generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldSpec {
    pub name: String,
    pub field_type: String,
    pub required: bool,
    pub description: Option<String>,
    pub is_uuid: Option<bool>,
    pub is_connection: Option<bool>,
    pub edge_type: Option<EdgeType>,
}

impl FieldSpec {
    pub fn new(name: String, field_type: String, required: bool) -> Self {
        Self {
            name,
            field_type,
            required,
            description: None,
            is_uuid: None,
            is_connection: None,
            edge_type: None,
        }
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn with_uuid_flag(mut self, is_uuid: bool) -> Self {
        self.is_uuid = Some(is_uuid);
        self
    }

    pub fn with_connection(mut self, is_connection: bool, edge_type: Option<EdgeType>) -> Self {
        self.is_connection = Some(is_connection);
        self.edge_type = edge_type;
        self
    }
}

/// Entity specification for model generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntitySpec {
    pub name: String,
    pub description: Option<String>,
    pub fields: Vec<FieldSpec>,
}

impl EntitySpec {
    pub fn new(name: String) -> Self {
        Self {
            name,
            description: None,
            fields: Vec::new(),
        }
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn add_field(mut self, field: FieldSpec) -> Self {
        self.fields.push(field);
        self
    }
}

/// Inventory of entities and connections for model generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inventory {
    pub entities: Vec<EntitySpec>,
    /// field_name -> target_entity_kind
    pub connections: HashMap<String, String>,
    pub notes: Vec<String>,
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            connections: HashMap::new(),
            notes: Vec::new(),
        }
    }

    pub fn add_entity(mut self, entity: EntitySpec) -> Self {
        self.entities.push(entity);
        self
    }

    pub fn add_connection(mut self, field_name: String, target_entity: String) -> Self {
        self.connections.insert(field_name, target_entity);
        self
    }

    pub fn add_note(mut self, note: String) -> Self {
        self.notes.push(note);
        self
    }
}

impl Default for Inventory {
    fn default() -> Self {
        Self::new()
    }
}

/// Known entity categories from the HBF database
pub const KNOWN_REGIONS: &[&str] = &[
    "Aurora Bushes", "Black Shield Timberlands", "Blood Blade Fields", "Bonecrusher Plains",
    "Darkfall Dunes", "Darkfall Plains", "Fallen Star Steppe", "Fearless Wilds", 
    "Firefly Cliffs", "Goblinchaser Jungle", "Goblinchaser Wilderness", "Goldenswan Timberlands",
    "Goldseekers Cliffs", "Grey Mist Snowlands", "Heartseeker Forest", "Heartseeker Moors",
    "Hells Gate Desert", "Holloweye Wilderness", "Iceborn Wilderness", "Javelin Plains",
    "Javelin Wetlands", "Moonwatcher Wetlands", "Nightmare Desert", "Ragthorn Meadows",
    "Ragthorn Woods", "Thunderwave Woodlands", "Vicious Crags",
];

pub const KNOWN_SETTLEMENTS: &[&str] = &[
    "Village of Harad", "City of Headsmen", "Town of Tinder", "City of Palemoon",
    "Town of Devilville", "Village of Ashamar", "Village of Balaal", "Village of Dokar",
    "Village of Dorith", "Village of Headbone", "Village of Kothian",
];

pub const KNOWN_FACTIONS: &[&str] = &[
    "The Defiled Wolves", "Fists of Justice", "The Crimson Order", "Shadow Stalkers", "Iron Brotherhood",
];

pub const KNOWN_DUNGEONS: &[&str] = &[
    "Bowel of the Raging Pits", "Den of the Raging Pits", "Caverns of the Burning Souls",
    "Caverns of the Infernal Lich", "Crypt of the Corrupted Order", "Crypt of the Infernal Blades",
    "Crypt of the Mourning Goblin", "Crypt of the Unholy Goblin", "Crypt of the Violent Ogre",
    "Hideout of the Corrupted Order", "Hideout of the Unspoken Desire", 
    "Lair of the Foresaken Desire", "Lair of the Mourning Hopes", 
    "Shrine of the Infernal Blades", "Shrine of the Infernal Desire",
    "Temple of the Violent Ogre", "Tomb of the Cursed Pits", "Tomb of the Grey Ogre",
];

/// AI generation thresholds
pub const HTML_ENTITIES_SAMPLE_THRESHOLD: usize = 10;
pub const JSON_ENTITIES_SAMPLE_THRESHOLD: usize = 5;

/// Default OpenAI model for analysis
pub const DEFAULT_MODEL: &str = "gpt-4o-2024-08-06";

/// Hex coordinate parsing utilities
pub mod hex_utils {
    use super::HexKey;
    use regex::Regex;
    use std::sync::OnceLock;

    static HEX_PATTERN: OnceLock<Regex> = OnceLock::new();

    fn hex_regex() -> &'static Regex {
        HEX_PATTERN.get_or_init(|| {
            Regex::new(r"(?i)hex\s+([NSEW]\d+[NSEW]\d+)").expect("Valid hex regex")
        })
    }

    /// Extract hex coordinate from content like "Hex W2S51"
    pub fn extract_hex_coordinate(content: &str) -> Option<HexKey> {
        hex_regex()
            .captures(content)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str().to_uppercase())
    }

    /// Convert hex coordinate to axial coordinates (q, r)
    pub fn hex_to_axial(hex_key: &str) -> Option<(i32, i32)> {
        // This would need proper implementation based on the HBF coordinate system
        // For now, return placeholder
        if hex_key.is_empty() {
            None
        } else {
            Some((0, 0))
        }
    }
}

/// UUID extraction utilities
pub mod uuid_utils {
    use regex::Regex;
    use std::sync::OnceLock;

    static UUID_PATTERN: OnceLock<Regex> = OnceLock::new();

    fn uuid_regex() -> &'static Regex {
        UUID_PATTERN.get_or_init(|| {
            Regex::new(r"[0-9a-fA-F]{8}-?[0-9a-fA-F]{4}-?[0-9a-fA-F]{4}-?[0-9a-fA-F]{4}-?[0-9a-fA-F]{12}|[0-9a-zA-Z]{8}")
                .expect("Valid UUID regex")
        })
    }

    /// Extract all UUIDs from content
    pub fn extract_uuids(content: &str) -> Vec<String> {
        uuid_regex()
            .find_iter(content)
            .map(|m| m.as_str().to_string())
            .collect()
    }

    /// Extract UUID from filename pattern like "entity_YVyOmKIy.html"
    pub fn extract_uuid_from_filename(filename: &str) -> Option<String> {
        if let Some(captures) = Regex::new(r"entity_([^.]+)")
            .ok()?
            .captures(filename)
        {
            captures.get(1).map(|m| m.as_str().to_string())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_coordinate_extraction() {
        let content = "This is content about Hex W2S51 with some data";
        let hex = hex_utils::extract_hex_coordinate(content);
        assert_eq!(hex, Some("W2S51".to_string()));
    }

    #[test]
    fn test_uuid_extraction() {
        let content = "Some content with UUID YVyOmKIy and more stuff";
        let uuids = uuid_utils::extract_uuids(content);
        assert!(!uuids.is_empty());
    }

    #[test]
    fn test_field_spec_builder() {
        let field = FieldSpec::new("test_field".to_string(), "String".to_string(), true)
            .with_description("Test description".to_string())
            .with_uuid_flag(true);
        
        assert_eq!(field.name, "test_field");
        assert_eq!(field.field_type, "String");
        assert!(field.required);
        assert_eq!(field.description, Some("Test description".to_string()));
        assert_eq!(field.is_uuid, Some(true));
    }
}
