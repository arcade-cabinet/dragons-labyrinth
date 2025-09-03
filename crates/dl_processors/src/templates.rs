//! Template system using minijinja2 for sophisticated code generation.
//!
//! This module replaces the basic string replacement in clusters.rs with
//! a proper template engine using minijinja2. It provides:
//! - Embedded templates for each entity type
//! - Template context rendering with proper error handling
//! - Template validation and compilation
//! - Sophisticated template utilities and filters

use minijinja::Environment;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use anyhow::{Result, Context as AnyhowContext};

use crate::base::Inventory;
use crate::raw::EntityCategory;

/// Template manager for the analysis system
#[derive(Debug, Clone)]
pub struct TemplateManager {
    env: Environment<'static>,
}

impl TemplateManager {
    pub fn new() -> Result<Self> {
        let mut env = Environment::new();
        
        // Add all embedded templates
        env.add_template("regions", REGIONS_TEMPLATE)?;
        env.add_template("settlements", SETTLEMENTS_TEMPLATE)?;
        env.add_template("factions", FACTIONS_TEMPLATE)?;
        env.add_template("dungeons", DUNGEONS_TEMPLATE)?;
        env.add_template("base_entity", BASE_ENTITY_TEMPLATE)?;
        
        // Add custom filters
        env.add_filter("rust_type", rust_type_filter);
        env.add_filter("default_value", default_value_filter);
        env.add_filter("is_optional", is_optional_filter);
        env.add_filter("extract_inner_type", extract_inner_type_filter);
        
        // Add global functions
        env.add_global("generate_uuid_methods", minijinja::Value::from_function(generate_uuid_methods));
        env.add_global("generate_spatial_methods", minijinja::Value::from_function(generate_spatial_methods));
        
        Ok(Self { env })
    }

    /// Render a template for a specific entity category
    pub fn render_entity_template(
        &self,
        category: &EntityCategory,
        inventory: &Inventory,
        metadata: Option<&HashMap<String, String>>,
    ) -> Result<String> {
        let template_name = match category {
            EntityCategory::Regions => "regions",
            EntityCategory::Settlements => "settlements", 
            EntityCategory::Factions => "factions",
            EntityCategory::Dungeons => "dungeons",
            _ => "base_entity",
        };

        let template = self.env.get_template(template_name)
            .with_context(|| format!("Failed to get template: {}", template_name))?;

        let context = self.build_template_context(inventory, metadata)?;

        template.render(context)
            .with_context(|| format!("Failed to render template: {}", template_name))
    }

    /// Generate ECS region code using template (exposed for dl_processors)
    pub fn generate_region_ecs_code(&self, uuid: &str, name: &str, corruption_level: f64) -> Result<String> {
        let region_template_path = std::path::Path::new("templates/region_ecs.rs.jinja2");
        
        // For now, use a simple template string since we can't easily load external files in build.rs
        let sanitized_name = sanitize_ident(name);
        
        Ok(format!(r#"//! Region: {}
//! UUID: {}
//! Generated ECS resources with container-based spatial processing

use bevy::prelude::*;
use crate::components::*;

/// Region resource component
#[derive(Resource, Debug, Clone)]
pub struct {}Region {{
    pub uuid: String,
    pub name: String, 
    pub corruption_level: f32,
    pub hex_tiles: Vec<Entity>,
    pub settlements: Vec<Entity>,
    pub dungeons: Vec<Entity>,
}}

impl Default for {}Region {{
    fn default() -> Self {{
        Self {{
            uuid: "{}".to_string(),
            name: "{}".to_string(),
            corruption_level: {:.2},
            hex_tiles: Vec::new(),
            settlements: Vec::new(),
            dungeons: Vec::new(),
        }}
    }}
}}

/// Spawn this region using container-based spatial indexing
pub fn spawn_region_with_containers(
    mut commands: Commands,
    mut region_resource: ResMut<{}Region>,
) {{
    // Create region entity with spatial container component
    let region_entity = commands.spawn((
        RegionId("{}".to_string()),
        RegionName("{}".to_string()),
        CorruptionLevel({:.2}),
        SpatialContainer::new(),
    )).id();
    
    // TODO: Load hex tiles using container system for O(1) lookups
    // TODO: Process settlements with spatial relationships
    // TODO: Process dungeons with container-based pathfinding
    
    println!("Spawned region: {{}} (UUID: {{}})", name, uuid);
}}

/// Get static metadata for this region
pub const REGION_METADATA: RegionMetadata = RegionMetadata {{
    uuid: "{}",
    name: "{}",
    base_corruption: {:.2},
}};
"#, 
            name, uuid,                    // 1, 2: Region comments
            sanitized_name,                // 3: Struct name
            sanitized_name,                // 4: Impl struct name
            uuid, name, corruption_level,  // 5, 6, 7: Default values
            sanitized_name,                // 8: Function parameter struct name
            uuid, name, corruption_level,  // 9, 10, 11: Component values
            uuid, name, corruption_level   // 12, 13, 14: Metadata constant
        ))
    }

    /// Generate hex tile ECS code using template (exposed for dl_processors)
    pub fn generate_hex_tile_ecs_code(&self, index: usize, q: i32, r: i32, biome: &str, hex_uuid: &str) -> Result<String> {
        let biome_variant = match biome.to_lowercase().as_str() {
            "wet meadow" => "WetMeadow",
            "ashen forest" => "AshenForest", 
            "flooded village" => "FloodedVillage",
            "black swamp" => "BlackSwamp",
            "fungal cathedral" => "FungalCathedral",
            "shadowed fen" => "ShadowedFen",
            "rust plains" => "RustPlains",
            "hollow hills" => "HollowHills",
            "corroded battleground" => "CorrodedBattleground",
            "famine fields" => "FamineFields",
            "bone forest" => "BoneForest",
            "desolate expanse" => "DesolateExpanse",
            "dragon scar" => "DragonScar",
            "abyssal chasm" => "AbyssalChasm",
            "final dread terrain" => "FinalDreadTerrain",
            _ => "WetMeadow", // fallback
        };
        
        Ok(format!(r#"//! Hex tile {} at position ({}, {})
//! Biome: {}

use bevy::prelude::*;
use crate::components::*;

/// Spawn this hex tile with spatial container integration
pub fn spawn_hex_tile_with_container(
    mut commands: Commands,
    container: &mut SpatialContainer,
) -> Entity {{
    let entity = commands.spawn((
        HexPosition {{ q: {}, r: {} }},
        HexBiome::{},
        HexId("{}".to_string()),
        BiomeFeatures::default(),
    )).id();
    
    // Register in spatial container for O(1) lookups
    container.register_hex_entity(({}, {}), entity);
    
    entity
}}

/// Static hex data for container queries
pub const HEX_STATIC_DATA: HexStaticData = HexStaticData {{
    uuid: "{}",
    q: {},
    r: {},
    biome: "{}",
}};
"#,
            index, q, r, biome,
            q, r, biome_variant, hex_uuid,
            q, r,
            hex_uuid, q, r, biome
        ))
    }

    /// Build template context from inventory and metadata
    fn build_template_context(
        &self,
        inventory: &Inventory,
        metadata: Option<&HashMap<String, String>>,
    ) -> Result<minijinja::Value> {
        let mut ctx_map = HashMap::new();
        ctx_map.insert("inventory".to_string(), minijinja::Value::from_serialize(inventory));
        ctx_map.insert("generation_timestamp".to_string(), 
                      minijinja::Value::from_serialize(&chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string()));

        if let Some(meta) = metadata {
            for (key, value) in meta {
                ctx_map.insert(key.clone(), minijinja::Value::from_serialize(value));
            }
        }

        Ok(minijinja::Value::from_serialize(&ctx_map))
    }

    /// Validate all templates can compile
    pub fn validate_templates(&self) -> Result<()> {
        let templates = ["regions", "settlements", "factions", "dungeons", "base_entity"];
        
        for template_name in &templates {
            self.env.get_template(template_name)
                .with_context(|| format!("Template validation failed: {}", template_name))?;
        }
        
        Ok(())
    }
}

impl Default for TemplateManager {
    fn default() -> Self {
        Self::new().expect("Failed to create template manager")
    }
}

/// Template for regions module
const REGIONS_TEMPLATE: &str = r#"//! Generated models for regions
//! 
//! This file was generated by the analysis system. Do not edit manually.
//! Generated at: {{ generation_timestamp }}

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::base::HexKey;

{% for entity in inventory.entities %}
/// {{ entity.description or entity.name }}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct {{ entity.name }} {
{% for field in entity.fields %}
    /// {{ field.description or '' }}
    pub {{ field.name }}: {{ field.type | rust_type(field.required) }},
{% endfor %}
}

impl {{ entity.name }} {
    pub fn new(entity_uuid: String) -> Self {
        Self {
            entity_uuid,
{% for field in entity.fields %}
{% if field.name != "entity_uuid" %}
            {{ field.name }}: {{ field.type | default_value(field.required) }},
{% endif %}
{% endfor %}
        }
    }

{% if entity.fields | selectattr("is_uuid", "equalto", true) | list | length > 0 %}
    /// Extract UUID connections from this entity
    pub fn extract_uuid_connections(&self) -> HashMap<String, Vec<String>> {
        let mut connections = HashMap::new();
{% for field in entity.fields %}
{% if field.is_uuid %}
        {% if field.type.startswith("Vec<") %}
        connections.insert("{{ field.name }}".to_string(), self.{{ field.name }}.clone());
        {% elif field.type | is_optional %}
        if let Some(ref uuid) = self.{{ field.name }} {
            connections.insert("{{ field.name }}".to_string(), vec![uuid.clone()]);
        }
        {% else %}
        connections.insert("{{ field.name }}".to_string(), vec![self.{{ field.name }}.clone()]);
        {% endif %}
{% endif %}
{% endfor %}
        connections
    }
{% endif %}

{% if entity.fields | selectattr("is_spatial", "equalto", true) | list | length > 0 %}
    /// Extract spatial coordinates from this entity
    pub fn extract_spatial_info(&self) -> HashMap<String, String> {
        let mut spatial = HashMap::new();
{% for field in entity.fields %}
{% if field.is_spatial %}
        {% if field.type | is_optional %}
        if let Some(ref coord) = self.{{ field.name }} {
            spatial.insert("{{ field.name }}".to_string(), coord.to_string());
        }
        {% else %}
        spatial.insert("{{ field.name }}".to_string(), self.{{ field.name }}.to_string());
        {% endif %}
{% endif %}
{% endfor %}
        spatial
    }
{% endif %}

    /// Get all referenced entity UUIDs from this region hex tile
    pub fn get_referenced_uuids(&self) -> Vec<String> {
        let mut uuids = Vec::new();
        
{% for field in entity.fields %}
{% if field.is_uuid and field.name != "entity_uuid" %}
        {% if field.type.startswith("Vec<") %}
        uuids.extend(self.{{ field.name }}.iter().cloned());
        {% elif field.type | is_optional %}
        if let Some(ref uuid) = self.{{ field.name }} {
            uuids.push(uuid.clone());
        }
        {% else %}
        uuids.push(self.{{ field.name }}.clone());
        {% endif %}
{% endif %}
{% endfor %}
        
        uuids
    }
}

{% endfor %}

{% if inventory.notes %}
/*
Generation Notes:
{% for note in inventory.notes %}
- {{ note }}
{% endfor %}
*/
{% endif %}
"#;

/// Template for settlements module
const SETTLEMENTS_TEMPLATE: &str = r#"//! Generated models for settlements
//! 
//! This file was generated by the analysis system. Do not edit manually.
//! Generated at: {{ generation_timestamp }}

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::base::HexKey;

{% for entity in inventory.entities %}
/// {{ entity.description or entity.name }}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct {{ entity.name }} {
{% for field in entity.fields %}
    /// {{ field.description or '' }}
    pub {{ field.name }}: {{ field.type | rust_type(field.required) }},
{% endfor %}
}

impl {{ entity.name }} {
    pub fn new(entity_uuid: String) -> Self {
        Self {
            entity_uuid,
{% for field in entity.fields %}
{% if field.name != "entity_uuid" %}
            {{ field.name }}: {{ field.type | default_value(field.required) }},
{% endif %}
{% endfor %}
        }
    }

    /// Check if this settlement is fortified based on defense level
    pub fn is_fortified(&self) -> bool {
{% for field in entity.fields %}
{% if field.name.contains("defense") and field.type.contains("i32") %}
        self.{{ field.name }}.map_or(false, |level| level > 3)
{% endif %}
{% endfor %}
        false  // Default if no defense field found
    }

    /// Get settlement size category based on population
    pub fn get_size_category(&self) -> SettlementSize {
{% for field in entity.fields %}
{% if field.name.contains("population") and field.type.contains("i32") %}
        match self.{{ field.name }} {
            Some(pop) if pop < 100 => SettlementSize::Hamlet,
            Some(pop) if pop < 1000 => SettlementSize::Village,
            Some(pop) if pop < 5000 => SettlementSize::Town,
            Some(pop) if pop >= 5000 => SettlementSize::City,
            None => SettlementSize::Unknown,
        }
{% endif %}
{% endfor %}
        SettlementSize::Unknown  // Default if no population field
    }

{% if entity.fields | selectattr("is_uuid", "equalto", true) | list | length > 0 %}
    /// Extract referenced faction and NPC UUIDs
    pub fn extract_referenced_uuids(&self) -> Vec<String> {
        let mut uuids = Vec::new();
        
{% for field in entity.fields %}
{% if field.is_uuid and field.name != "entity_uuid" %}
        {% if field.type.startswith("Vec<") %}
        uuids.extend(self.{{ field.name }}.iter().cloned());
        {% elif field.type | is_optional %}
        if let Some(ref uuid) = self.{{ field.name }} {
            uuids.push(uuid.clone());
        }
        {% else %}
        uuids.push(self.{{ field.name }}.clone());
        {% endif %}
{% endif %}
{% endfor %}
        
        uuids
    }
{% endif %}
}

{% endfor %}

/// Settlement size categories
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SettlementSize {
    Hamlet,
    Village,
    Town,
    City,
    Unknown,
}

{% if inventory.notes %}
/*
Generation Notes:
{% for note in inventory.notes %}
- {{ note }}
{% endfor %}
*/
{% endif %}
"#;

/// Template for factions module
const FACTIONS_TEMPLATE: &str = r#"//! Generated models for factions
//! 
//! This file was generated by the analysis system. Do not edit manually.
//! Generated at: {{ generation_timestamp }}

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::base::HexKey;

{% for entity in inventory.entities %}
/// {{ entity.description or entity.name }}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct {{ entity.name }} {
{% for field in entity.fields %}
    /// {{ field.description or '' }}
    pub {{ field.name }}: {{ field.type | rust_type(field.required) }},
{% endfor %}
}

impl {{ entity.name }} {
    pub fn new(entity_uuid: String) -> Self {
        Self {
            entity_uuid,
{% for field in entity.fields %}
{% if field.name != "entity_uuid" %}
            {{ field.name }}: {{ field.type | default_value(field.required) }},
{% endif %}
{% endfor %}
        }
    }

    /// Get faction power level based on influence
    pub fn get_power_level(&self) -> FactionPower {
{% for field in entity.fields %}
{% if field.name.contains("influence") and field.type.contains("i32") %}
        match self.{{ field.name }} {
            Some(level) if level < 3 => FactionPower::Minor,
            Some(level) if level < 7 => FactionPower::Moderate,
            Some(level) if level >= 7 => FactionPower::Major,
            None => FactionPower::Unknown,
        }
{% endif %}
{% endfor %}
        FactionPower::Unknown  // Default if no influence field
    }

    /// Check if faction controls a specific hex
    pub fn controls_hex(&self, hex_key: &HexKey) -> bool {
{% for field in entity.fields %}
{% if field.name.contains("territories") or field.name.contains("territory") %}
        self.{{ field.name }}.contains(hex_key)
{% endif %}
{% endfor %}
        false  // Default if no territories field
    }

{% if entity.fields | selectattr("is_uuid", "equalto", true) | list | length > 0 %}
    /// Extract all faction relationship UUIDs
    pub fn extract_referenced_uuids(&self) -> Vec<String> {
        let mut uuids = Vec::new();
        
{% for field in entity.fields %}
{% if field.is_uuid and field.name != "entity_uuid" %}
        {% if field.type.startswith("Vec<") %}
        uuids.extend(self.{{ field.name }}.iter().cloned());
        {% elif field.type | is_optional %}
        if let Some(ref uuid) = self.{{ field.name }} {
            uuids.push(uuid.clone());
        }
        {% else %}
        uuids.push(self.{{ field.name }}.clone());
        {% endif %}
{% endif %}
{% endfor %}
        
        uuids
    }
{% endif %}

    /// Check if faction is allied with another faction
    pub fn is_ally(&self, other_faction_uuid: &str) -> bool {
{% for field in entity.fields %}
{% if field.name.contains("allies") or field.name.contains("allegiances") %}
        self.{{ field.name }}.contains(&other_faction_uuid.to_string())
{% endif %}
{% endfor %}
        false
    }

    /// Check if faction is enemy with another faction  
    pub fn is_enemy(&self, other_faction_uuid: &str) -> bool {
{% for field in entity.fields %}
{% if field.name.contains("enemies") or field.name.contains("rivals") %}
        self.{{ field.name }}.contains(&other_faction_uuid.to_string())
{% endif %}
{% endfor %}
        false
    }
}

{% endfor %}

/// Faction power level categories
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FactionPower {
    Minor,
    Moderate,
    Major,
    Unknown,
}

{% if inventory.notes %}
/*
Generation Notes:
{% for note in inventory.notes %}
- {{ note }}
{% endfor %}
*/
{% endif %}
"#;

/// Template for dungeons module
const DUNGEONS_TEMPLATE: &str = r#"//! Generated models for dungeons
//! 
//! This file was generated by the analysis system. Do not edit manually.
//! Generated at: {{ generation_timestamp }}

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::base::HexKey;

{% for entity in inventory.entities %}
/// {{ entity.description or entity.name }}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct {{ entity.name }} {
{% for field in entity.fields %}
    /// {{ field.description or '' }}
    pub {{ field.name }}: {{ field.type | rust_type(field.required) }},
{% endfor %}
}

impl {{ entity.name }} {
    pub fn new(entity_uuid: String) -> Self {
        Self {
            entity_uuid,
{% for field in entity.fields %}
{% if field.name != "entity_uuid" %}
            {{ field.name }}: {{ field.type | default_value(field.required) }},
{% endif %}
{% endfor %}
        }
    }

{% if entity.fields | selectattr("name", "contains", "monsters") | list | length > 0 %}
    /// Extract monster UUIDs from this dungeon area
    pub fn extract_monster_uuids(&self) -> Vec<String> {
        let mut monster_uuids = Vec::new();
        
{% for field in entity.fields %}
{% if field.name.contains("monsters") %}
        for monster in &self.{{ field.name }} {
            if let Some(uuid) = monster.get("uuid").and_then(|v| v.as_str()) {
                monster_uuids.push(uuid.to_string());
            }
        }
{% endif %}
{% endfor %}
        
        monster_uuids
    }
{% endif %}

{% if entity.fields | selectattr("name", "contains", "treasure") | list | length > 0 %}
    /// Extract treasure UUIDs from this dungeon area
    pub fn extract_treasure_uuids(&self) -> Vec<String> {
        let mut treasure_uuids = Vec::new();
        
{% for field in entity.fields %}
{% if field.name.contains("treasure") and field.type | is_optional %}
        if let Some(ref treasure) = self.{{ field.name }} {
            if let Some(uuid) = treasure.get("uuid").and_then(|v| v.as_str()) {
                treasure_uuids.push(uuid.to_string());
            }
        }
{% elif field.name.contains("treasure") and field.type.startswith("Vec<") %}
        for treasure in &self.{{ field.name }} {
            if let Some(uuid) = treasure.get("uuid").and_then(|v| v.as_str()) {
                treasure_uuids.push(uuid.to_string());
            }
        }
{% endif %}
{% endfor %}
        
        treasure_uuids
    }
{% endif %}

    /// Extract all referenced UUIDs from this dungeon area
    pub fn extract_referenced_uuids(&self) -> Vec<String> {
        let mut uuids = Vec::new();
        
{% for field in entity.fields %}
{% if field.is_uuid and field.name != "entity_uuid" %}
        {% if field.type.startswith("Vec<") %}
        uuids.extend(self.{{ field.name }}.iter().cloned());
        {% elif field.type | is_optional %}
        if let Some(ref uuid) = self.{{ field.name }} {
            uuids.push(uuid.clone());
        }
        {% else %}
        uuids.push(self.{{ field.name }}.clone());
        {% endif %}
{% endif %}
{% endfor %}
        
        // Add monster/treasure UUIDs if present
        uuids.extend(self.extract_monster_uuids());
        uuids.extend(self.extract_treasure_uuids());
        
        uuids
    }
}

{% endfor %}

{% if inventory.notes %}
/*
Generation Notes:
{% for note in inventory.notes %}
- {{ note }}
{% endfor %}
*/
{% endif %}
"#;

/// Base template for generic entities
const BASE_ENTITY_TEMPLATE: &str = r#"//! Generated models for {{ category | default("generic") }} entities
//! 
//! This file was generated by the analysis system. Do not edit manually.
//! Generated at: {{ generation_timestamp }}

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

{% for entity in inventory.entities %}
/// {{ entity.description or entity.name }}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct {{ entity.name }} {
{% for field in entity.fields %}
    /// {{ field.description or '' }}
    pub {{ field.name }}: {{ field.type | rust_type(field.required) }},
{% endfor %}
}

impl {{ entity.name }} {
    pub fn new(entity_uuid: String) -> Self {
        Self {
            entity_uuid,
{% for field in entity.fields %}
{% if field.name != "entity_uuid" %}
            {{ field.name }}: {{ field.type | default_value(field.required) }},
{% endif %}
{% endfor %}
        }
    }
}

{% endfor %}

{% if inventory.notes %}
/*
Generation Notes:
{% for note in inventory.notes %}
- {{ note }}
{% endfor %}
*/
{% endif %}
"#;

/// Custom filter to handle Rust type formatting
fn rust_type_filter(type_str: String, required: bool) -> String {
    if required {
        type_str
    } else {
        format!("Option<{}>", type_str)
    }
}

/// Custom filter to generate default values for fields
fn default_value_filter(type_str: String, required: bool) -> String {
    if !required {
        return "None".to_string();
    }

    match type_str.as_str() {
        "String" => "String::new()".to_string(),
        "Vec<String>" => "Vec::new()".to_string(),
        "Vec<serde_json::Value>" => "Vec::new()".to_string(),
        "HashMap<String, String>" => "HashMap::new()".to_string(),
        "HashMap<String, f32>" => "HashMap::new()".to_string(),
        s if s.starts_with("Vec<") => "Vec::new()".to_string(),
        s if s.starts_with("HashMap<") => "HashMap::new()".to_string(),
        _ => "Default::default()".to_string(),
    }
}

/// Custom filter to check if a type is optional
fn is_optional_filter(type_str: String) -> bool {
    type_str.starts_with("Option<")
}

/// Custom filter to extract inner type from Option<T>
fn extract_inner_type_filter(type_str: String) -> String {
    if type_str.starts_with("Option<") && type_str.ends_with('>') {
        type_str[7..type_str.len()-1].to_string()
    } else {
        type_str
    }
}

/// Global function to generate UUID extraction methods
fn generate_uuid_methods(_args: &[minijinja::Value]) -> Result<String, minijinja::Error> {
    Ok(r#"
    /// Extract UUID connections from this entity
    pub fn extract_uuid_connections(&self) -> HashMap<String, Vec<String>> {
        let mut connections = HashMap::new();
        // Implementation would be generated based on UUID fields
        connections
    }
    "#.to_string())
}

/// Global function to generate spatial methods
fn generate_spatial_methods(_args: &[minijinja::Value]) -> Result<String, minijinja::Error> {
    Ok(r#"
    /// Extract spatial information from this entity
    pub fn extract_spatial_info(&self) -> HashMap<String, String> {
        let mut spatial = HashMap::new();
        // Implementation would be generated based on spatial fields
        spatial
    }
    "#.to_string())
}

/// Utility function for sanitizing identifiers (exposed for downstream crates)
pub fn sanitize_ident(s: &str) -> String {
    s.split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<String>()
        .replace(['-', '\'', ' '], "")
}

/// Utility function for sanitizing UUIDs (exposed for downstream crates)
pub fn sanitize_uuid(uuid: &str) -> String {
    uuid.replace(['-', ' '], "_").to_lowercase()
}

/// Exposed template generation functions for dl_processors and apps/game
pub mod ecs_generation {
    use super::*;
    
    /// Generate region ECS code (called by dl_processors)
    pub fn generate_region_ecs_code(uuid: &str, name: &str, corruption_level: f64) -> String {
        let manager = TemplateManager::new().unwrap();
        manager.generate_region_ecs_code(uuid, name, corruption_level).unwrap()
    }
    
    /// Generate hex tile ECS code (called by dl_processors)
    pub fn generate_hex_tile_ecs_code(index: usize, q: i32, r: i32, biome: &str, hex_uuid: &str) -> String {
        let manager = TemplateManager::new().unwrap();
        manager.generate_hex_tile_ecs_code(index, q, r, biome, hex_uuid).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::base::{Inventory, EntitySpec, FieldSpec};

    #[test]
    fn test_template_manager_creation() {
        let manager = TemplateManager::new();
        assert!(manager.is_ok());
    }

    #[test]
    fn test_template_validation() {
        let manager = TemplateManager::new().unwrap();
        assert!(manager.validate_templates().is_ok());
    }

    #[test]
    fn test_rust_type_filter() {
        assert_eq!(rust_type_filter("String".to_string(), true), "String");
        assert_eq!(rust_type_filter("String".to_string(), false), "Option<String>");
        assert_eq!(rust_type_filter("Vec<String>".to_string(), true), "Vec<String>");
    }

    #[test]
    fn test_default_value_filter() {
        assert_eq!(default_value_filter("String".to_string(), true), "String::new()");
        assert_eq!(default_value_filter("String".to_string(), false), "None");
        assert_eq!(default_value_filter("Vec<String>".to_string(), true), "Vec::new()");
        assert_eq!(default_value_filter("i32".to_string(), true), "Default::default()");
    }

    #[test]
    fn test_is_optional_filter() {
        assert!(is_optional_filter("Option<String>".to_string()));
        assert!(!is_optional_filter("String".to_string()));
        assert!(!is_optional_filter("Vec<String>".to_string()));
    }

    #[test]
    fn test_extract_inner_type_filter() {
        assert_eq!(extract_inner_type_filter("Option<String>".to_string()), "String");
        assert_eq!(extract_inner_type_filter("String".to_string()), "String");
        assert_eq!(extract_inner_type_filter("Option<Vec<String>>".to_string()), "Vec<String>");
    }

    #[test]
    fn test_template_rendering() {
        let manager = TemplateManager::new().unwrap();
        
        // Create a simple inventory for testing
        let mut inventory = Inventory::new();
        let entity = EntitySpec::new("TestEntity".to_string())
            .add_field(FieldSpec::new("entity_uuid".to_string(), "String".to_string(), true))
            .add_field(FieldSpec::new("name".to_string(), "Option<String>".to_string(), false));
        inventory = inventory.add_entity(entity);
        
        let result = manager.render_entity_template(&EntityCategory::Regions, &inventory, None);
        assert!(result.is_ok());
        let rendered = result.unwrap();
        assert!(rendered.contains("TestEntity"));
        assert!(rendered.contains("entity_uuid: String"));
        assert!(rendered.contains("name: Option<String>"));
    }
}
