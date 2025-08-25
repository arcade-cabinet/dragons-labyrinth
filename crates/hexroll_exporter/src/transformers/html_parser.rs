//! Parses HTML entities containing settlements, dungeons, and rich content
//!
//! These 1,013 HTML entities contain the detailed narrative content:
//! - Settlements with NPCs, shops, taverns
//! - Dungeons with rooms, encounters, treasures
//! - Rich descriptions and atmospheric text

use html5ever::parse_document;
use html5ever::tendril::TendrilSink;
use markup5ever_rcdom::{Handle, NodeData, RcDom};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::io::Cursor;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HtmlContent {
    pub uuid: String,
    pub content_type: ContentType,
    pub title: Option<String>,
    pub sections: Vec<ContentSection>,
    pub npcs: Vec<NpcData>,
    pub locations: Vec<LocationData>,
    pub items: Vec<ItemData>,
    pub raw_text: String,
    pub metadata: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContentType {
    Settlement,
    Dungeon,
    Building,
    Room,
    Encounter,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentSection {
    pub heading: Option<String>,
    pub content: String,
    pub section_type: SectionType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SectionType {
    Description,
    History,
    Rumors,
    Encounters,
    Treasure,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NpcData {
    pub name: String,
    pub occupation: Option<String>,
    pub description: Option<String>,
    pub dialogue: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationData {
    pub name: String,
    pub location_type: String,
    pub description: String,
    pub connected_to: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemData {
    pub name: String,
    pub item_type: String,
    pub description: Option<String>,
    pub value: Option<String>,
}

/// Parses HTML entities containing rich narrative content
pub struct HtmlParser {
    settlements: Vec<HtmlContent>,
    dungeons: Vec<HtmlContent>,
    other_content: Vec<HtmlContent>,
    total_npcs: usize,
    total_locations: usize,
    total_items: usize,
}

impl HtmlParser {
    pub fn new() -> Self {
        Self {
            settlements: Vec::new(),
            dungeons: Vec::new(),
            other_content: Vec::new(),
            total_npcs: 0,
            total_locations: 0,
            total_items: 0,
        }
    }

    /// Process HTML entities
    pub fn process(&mut self, entities: Vec<Value>) -> HtmlParsingResult {
        for entity in entities {
            if let Some(html_str) = self.extract_html_content(&entity) {
                if let Some(parsed_content) = self.parse_html_entity(&entity, &html_str) {
                    // Track totals
                    self.total_npcs += parsed_content.npcs.len();
                    self.total_locations += parsed_content.locations.len();
                    self.total_items += parsed_content.items.len();
                    
                    // Categorize content
                    match parsed_content.content_type {
                        ContentType::Settlement => self.settlements.push(parsed_content),
                        ContentType::Dungeon => self.dungeons.push(parsed_content),
                        _ => self.other_content.push(parsed_content),
                    }
                }
            }
        }

        HtmlParsingResult {
            settlements: self.settlements.clone(),
            dungeons: self.dungeons.clone(),
            other_content: self.other_content.clone(),
            total_npcs: self.total_npcs,
            total_locations: self.total_locations,
            total_items: self.total_items,
        }
    }

    /// Extract HTML content from entity
    fn extract_html_content(&self, entity: &Value) -> Option<String> {
        let obj = entity.as_object()?;
        
        // Look for HTML content in various fields
        if let Some(content) = obj.get("content") {
            if let Some(content_str) = content.as_str() {
                if content_str.contains("<") && content_str.contains(">") {
                    return Some(content_str.to_string());
                }
            }
        }
        
        if let Some(html) = obj.get("html").and_then(|v| v.as_str()) {
            return Some(html.to_string());
        }
        
        if let Some(body) = obj.get("body").and_then(|v| v.as_str()) {
            if body.contains("<") && body.contains(">") {
                return Some(body.to_string());
            }
        }
        
        None
    }

    /// Parse HTML entity into structured content
    fn parse_html_entity(&self, entity: &Value, html_str: &str) -> Option<HtmlContent> {
        let obj = entity.as_object()?;
        let uuid = obj.get("uuid")?.as_str()?.to_string();
        
        // Parse HTML to extract text and structure
        let (sections, raw_text) = self.parse_html_structure(html_str);
        
        // Determine content type
        let content_type = self.determine_content_type(&raw_text, &sections);
        
        // Extract title if present
        let title = self.extract_title(&sections, obj);
        
        // Extract NPCs, locations, and items
        let npcs = self.extract_npcs(&raw_text, &sections);
        let locations = self.extract_locations(&raw_text, &sections);
        let items = self.extract_items(&raw_text, &sections);
        
        // Collect metadata
        let mut metadata = HashMap::new();
        for (key, val) in obj.iter() {
            if key != "uuid" && key != "content" && key != "html" && key != "body" {
                metadata.insert(key.clone(), val.clone());
            }
        }
        
        Some(HtmlContent {
            uuid,
            content_type,
            title,
            sections,
            npcs,
            locations,
            items,
            raw_text,
            metadata,
        })
    }

    /// Parse HTML structure into sections
    fn parse_html_structure(&self, html_str: &str) -> (Vec<ContentSection>, String) {
        let mut sections = Vec::new();
        let mut raw_text = String::new();
        
        // Parse HTML using html5ever
        let dom = parse_document(RcDom::default(), Default::default())
            .from_utf8()
            .read_from(&mut Cursor::new(html_str.as_bytes()))
            .unwrap();
        
        // Walk the DOM tree to extract content
        self.walk_dom_tree(&dom.document, &mut sections, &mut raw_text);
        
        (sections, raw_text)
    }

    /// Walk DOM tree recursively
    fn walk_dom_tree(&self, handle: &Handle, sections: &mut Vec<ContentSection>, raw_text: &mut String) {
        match handle.data {
            NodeData::Text { ref contents } => {
                let text = contents.borrow().to_string();
                raw_text.push_str(&text);
                raw_text.push(' ');
            }
            NodeData::Element { ref name, .. } => {
                let tag_name = name.local.to_string();
                
                // Check for section-defining elements
                if tag_name == "h1" || tag_name == "h2" || tag_name == "h3" {
                    let mut heading_text = String::new();
                    self.extract_text_from_node(handle, &mut heading_text);
                    
                    // For now, just create a basic section with the heading
                    // TODO: Implement proper sibling traversal when we have the right API
                    if !heading_text.is_empty() {
                        sections.push(ContentSection {
                            heading: Some(heading_text.clone()),
                            content: String::new(),
                            section_type: self.determine_section_type(&heading_text, ""),
                        });
                    }
                }
            }
            _ => {}
        }
        
        // Recurse to children
        for child in handle.children.borrow().iter() {
            self.walk_dom_tree(child, sections, raw_text);
        }
    }

    /// Extract text from a DOM node
    fn extract_text_from_node(&self, handle: &Handle, text: &mut String) {
        match handle.data {
            NodeData::Text { ref contents } => {
                text.push_str(&contents.borrow());
                text.push(' ');
            }
            _ => {
                for child in handle.children.borrow().iter() {
                    self.extract_text_from_node(child, text);
                }
            }
        }
    }

    /// Determine content type from text and sections
    fn determine_content_type(&self, text: &str, _sections: &[ContentSection]) -> ContentType {
        let text_lower = text.to_lowercase();
        
        if text_lower.contains("settlement") || text_lower.contains("village") || 
           text_lower.contains("town") || text_lower.contains("city") ||
           text_lower.contains("tavern") || text_lower.contains("inn") {
            ContentType::Settlement
        } else if text_lower.contains("dungeon") || text_lower.contains("chamber") ||
                  text_lower.contains("corridor") || text_lower.contains("trap") {
            ContentType::Dungeon
        } else if text_lower.contains("shop") || text_lower.contains("store") ||
                  text_lower.contains("merchant") {
            ContentType::Building
        } else if text_lower.contains("room") || text_lower.contains("area") {
            ContentType::Room
        } else if text_lower.contains("encounter") || text_lower.contains("combat") {
            ContentType::Encounter
        } else {
            ContentType::Unknown
        }
    }

    /// Determine section type from heading and content
    fn determine_section_type(&self, heading: &str, content: &str) -> SectionType {
        let combined = format!("{} {}", heading.to_lowercase(), content.to_lowercase());
        
        if combined.contains("description") || combined.contains("appearance") {
            SectionType::Description
        } else if combined.contains("history") || combined.contains("background") {
            SectionType::History
        } else if combined.contains("rumor") || combined.contains("gossip") {
            SectionType::Rumors
        } else if combined.contains("encounter") || combined.contains("combat") {
            SectionType::Encounters
        } else if combined.contains("treasure") || combined.contains("loot") {
            SectionType::Treasure
        } else {
            SectionType::Unknown
        }
    }

    /// Extract title from sections or metadata
    fn extract_title(&self, sections: &[ContentSection], obj: &serde_json::Map<String, Value>) -> Option<String> {
        // Check metadata first
        if let Some(title) = obj.get("title").and_then(|v| v.as_str()) {
            return Some(title.to_string());
        }
        if let Some(name) = obj.get("name").and_then(|v| v.as_str()) {
            return Some(name.to_string());
        }
        
        // Check first section heading
        if let Some(first_section) = sections.first() {
            if let Some(heading) = &first_section.heading {
                return Some(heading.clone());
            }
        }
        
        None
    }

    /// Extract NPCs from text
    fn extract_npcs(&self, text: &str, _sections: &[ContentSection]) -> Vec<NpcData> {
        let mut npcs = Vec::new();
        
        // Simple pattern matching for NPCs
        // Look for patterns like "NAME, a OCCUPATION" or "NAME the TITLE"
        let lines: Vec<&str> = text.lines().collect();
        for line in lines {
            if line.contains(", a ") || line.contains(" the ") {
                // Try to extract NPC info
                if let Some(npc) = self.parse_npc_line(line) {
                    npcs.push(npc);
                }
            }
        }
        
        npcs
    }

    /// Parse a potential NPC line
    fn parse_npc_line(&self, line: &str) -> Option<NpcData> {
        // Simple heuristic: look for capitalized names followed by occupation
        let parts: Vec<&str> = line.split(", ").collect();
        if parts.len() >= 2 {
            let name = parts[0].trim();
            let rest = parts[1..].join(", ");
            
            // Check if name looks like a proper name (starts with capital)
            if name.chars().next()?.is_uppercase() {
                return Some(NpcData {
                    name: name.to_string(),
                    occupation: Some(rest),
                    description: None,
                    dialogue: Vec::new(),
                });
            }
        }
        
        None
    }

    /// Extract locations from text
    fn extract_locations(&self, text: &str, sections: &[ContentSection]) -> Vec<LocationData> {
        let mut locations = Vec::new();
        
        // Look for location patterns in sections
        for section in sections {
            if section.content.contains("room") || section.content.contains("area") ||
               section.content.contains("chamber") || section.content.contains("hall") {
                if let Some(location) = self.parse_location_section(section) {
                    locations.push(location);
                }
            }
        }
        
        // Also check raw text for location mentions
        let text_lower = text.to_lowercase();
        if text_lower.contains("north") || text_lower.contains("south") ||
           text_lower.contains("east") || text_lower.contains("west") ||
           text_lower.contains("door") || text_lower.contains("passage") {
            // Extract connections
            // This is simplified - real implementation would be more sophisticated
        }
        
        locations
    }

    /// Parse a location from a section
    fn parse_location_section(&self, section: &ContentSection) -> Option<LocationData> {
        let name = section.heading.clone().unwrap_or_else(|| "Unknown Location".to_string());
        let location_type = if section.content.contains("room") {
            "room".to_string()
        } else if section.content.contains("corridor") {
            "corridor".to_string()
        } else {
            "area".to_string()
        };
        
        Some(LocationData {
            name,
            location_type,
            description: section.content.clone(),
            connected_to: Vec::new(),
        })
    }

    /// Extract items from text
    fn extract_items(&self, text: &str, _sections: &[ContentSection]) -> Vec<ItemData> {
        let mut items = Vec::new();
        
        // Look for item patterns
        let text_lower = text.to_lowercase();
        if text_lower.contains("gold") || text_lower.contains("gp") {
            items.push(ItemData {
                name: "Gold".to_string(),
                item_type: "currency".to_string(),
                description: None,
                value: None,
            });
        }
        
        // Look for weapon/armor/potion mentions
        let item_keywords = ["sword", "axe", "armor", "shield", "potion", "scroll", "ring", "amulet"];
        for keyword in &item_keywords {
            if text_lower.contains(keyword) {
                items.push(ItemData {
                    name: keyword.to_string(),
                    item_type: "equipment".to_string(),
                    description: None,
                    value: None,
                });
            }
        }
        
        items
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct HtmlParsingResult {
    pub settlements: Vec<HtmlContent>,
    pub dungeons: Vec<HtmlContent>,
    pub other_content: Vec<HtmlContent>,
    pub total_npcs: usize,
    pub total_locations: usize,
    pub total_items: usize,
}

impl HtmlParsingResult {
    /// Generate a summary report
    pub fn summary(&self) -> String {
        format!(
            "HTML Content Parsing Summary:\n\
             - Settlements: {}\n\
             - Dungeons: {}\n\
             - Other Content: {}\n\
             - Total NPCs: {}\n\
             - Total Locations: {}\n\
             - Total Items: {}",
            self.settlements.len(),
            self.dungeons.len(),
            self.other_content.len(),
            self.total_npcs,
            self.total_locations,
            self.total_items
        )
    }

    /// Get all unique NPC names
    pub fn get_all_npc_names(&self) -> Vec<String> {
        let mut names = Vec::new();
        
        for settlement in &self.settlements {
            for npc in &settlement.npcs {
                if !names.contains(&npc.name) {
                    names.push(npc.name.clone());
                }
            }
        }
        
        for dungeon in &self.dungeons {
            for npc in &dungeon.npcs {
                if !names.contains(&npc.name) {
                    names.push(npc.name.clone());
                }
            }
        }
        
        names.sort();
        names
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_parse_simple_html() {
        let mut parser = HtmlParser::new();
        
        let entity = json!({
            "uuid": "settlement-1",
            "content": "<h1>Riverside Village</h1><p>A peaceful settlement by the river.</p>"
        });
        
        let result = parser.process(vec![entity]);
        assert_eq!(result.settlements.len(), 1);
        assert_eq!(result.settlements[0].title, Some("Riverside Village".to_string()));
    }

    #[test]
    fn test_detect_content_type() {
        let parser = HtmlParser::new();
        
        assert_eq!(
            parser.determine_content_type("A small village with a tavern", &[]),
            ContentType::Settlement
        );
        
        assert_eq!(
            parser.determine_content_type("Dark dungeon corridor with traps", &[]),
            ContentType::Dungeon
        );
    }
}
