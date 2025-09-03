//! Common entity definitions and utilities.
//!
//! This module contains shared entity models and utilities used across
//! the analysis system. These match the Python entity models and provide
//! a consistent interface for all entity types.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::audit::AuditableType;
use crate::base::HexKey;

/// Region hex tile entity matching Python RegionHexTile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionHexTile {
    pub entity_uuid: String,
    pub hex_key: Option<HexKey>,
    pub map: Option<HashMap<String, f32>>,
    pub region_uuid: Option<String>,
    pub settlement_uuids: Vec<String>,
    pub dungeon_uuids: Vec<String>,
    pub faction_uuids: Vec<String>,
    pub biome_type: Option<String>,
    pub terrain_features: Vec<String>,
    pub special_features: Vec<String>,
    pub resource_nodes: Vec<String>,
}

impl AuditableType for RegionHexTile {
    fn audit_headers() -> Vec<String> {
        vec![
            "entity_uuid".to_string(),
            "hex_key".to_string(),
            "has_region".to_string(),
            "settlement_count".to_string(),
            "dungeon_count".to_string(),
            "faction_count".to_string(),
            "total_entities".to_string(),
            "biome_type".to_string(),
            "terrain_features_count".to_string(),
            "special_features_count".to_string(),
            "resource_nodes_count".to_string(),
            "data_completeness_score".to_string(),
        ]
    }
    
    fn audit_row(&self) -> Vec<String> {
        vec![
            self.entity_uuid.clone(),
            self.hex_key.as_ref().map(|h| h.clone()).unwrap_or("MISSING".to_string()),
            self.region_uuid.is_some().to_string(),
            self.settlement_uuids.len().to_string(),
            self.dungeon_uuids.len().to_string(),
            self.faction_uuids.len().to_string(),
            self.entity_count().to_string(),
            self.biome_type.as_ref().unwrap_or(&"UNKNOWN".to_string()).clone(),
            self.terrain_features.len().to_string(),
            self.special_features.len().to_string(),
            self.resource_nodes.len().to_string(),
            self.data_completeness_score().to_string(),
        ]
    }
    
    fn audit_category() -> String {
        "analytics".to_string()
    }
    
    fn audit_subcategory() -> String {
        "hbf_coverage".to_string()
    }
    
    fn extract_numeric_fields(&self) -> HashMap<String, f64> {
        let mut fields = HashMap::new();
        fields.insert("settlement_count".to_string(), self.settlement_uuids.len() as f64);
        fields.insert("dungeon_count".to_string(), self.dungeon_uuids.len() as f64);
        fields.insert("faction_count".to_string(), self.faction_uuids.len() as f64);
        fields.insert("total_entities".to_string(), self.entity_count() as f64);
        fields.insert("terrain_features_count".to_string(), self.terrain_features.len() as f64);
        fields.insert("special_features_count".to_string(), self.special_features.len() as f64);
        fields.insert("resource_nodes_count".to_string(), self.resource_nodes.len() as f64);
        fields.insert("data_completeness_score".to_string(), self.data_completeness_score());
        fields
    }
    
    fn custom_fields(&self) -> HashMap<String, String> {
        let mut fields = HashMap::new();
        fields.insert("has_complete_data".to_string(), (self.data_completeness_score() > 0.7).to_string());
        fields.insert("needs_attention".to_string(), (self.data_completeness_score() < 0.5).to_string());
        fields.insert("hex_coordinate_status".to_string(), 
            if self.hex_key.is_some() { "VALID".to_string() } else { "MISSING_COORDS".to_string() }
        );
        fields
    }
}

impl RegionHexTile {
    pub fn new(entity_uuid: String) -> Self {
        Self {
            entity_uuid,
            hex_key: None,
            map: None,
            region_uuid: None,
            settlement_uuids: Vec::new(),
            dungeon_uuids: Vec::new(),
            faction_uuids: Vec::new(),
            biome_type: None,
            terrain_features: Vec::new(),
            special_features: Vec::new(),
            resource_nodes: Vec::new(),
        }
    }

    /// Calculate data completeness score for pipeline efficiency tracking
    /// This is critical for identifying the HBF coverage issues
    pub fn data_completeness_score(&self) -> f64 {
        let mut score = 0.0;
        let mut max_score = 0.0;
        
        // Critical fields (worth more)
        max_score += 0.3; // hex_key - essential for game world mapping
        if self.hex_key.is_some() { score += 0.3; }
        
        max_score += 0.3; // region_uuid - essential for region assignment
        if self.region_uuid.is_some() { score += 0.3; }
        
        max_score += 0.2; // biome_type - important for game mechanics
        if self.biome_type.is_some() { score += 0.2; }
        
        // Entity presence (worth less but indicates data richness)
        max_score += 0.1; // has entities
        if self.has_entities() { score += 0.1; }
        
        max_score += 0.1; // feature richness
        if !self.terrain_features.is_empty() || !self.special_features.is_empty() { 
            score += 0.1; 
        }
        
        score / max_score
    }

    /// Extract all referenced UUIDs from this hex tile
    pub fn extract_referenced_uuids(&self) -> Vec<String> {
        let mut uuids = Vec::new();
        
        if let Some(region_uuid) = &self.region_uuid {
            uuids.push(region_uuid.clone());
        }
        
        uuids.extend(self.settlement_uuids.iter().cloned());
        uuids.extend(self.dungeon_uuids.iter().cloned());
        uuids.extend(self.faction_uuids.iter().cloned());
        
        uuids
    }

    /// Check if this hex contains any entities
    pub fn has_entities(&self) -> bool {
        !self.settlement_uuids.is_empty() || 
        !self.dungeon_uuids.is_empty() || 
        !self.faction_uuids.is_empty()
    }

    /// Get total entity count in this hex
    pub fn entity_count(&self) -> usize {
        self.settlement_uuids.len() + 
        self.dungeon_uuids.len() + 
        self.faction_uuids.len()
    }
}

/// Settlement establishment entity matching Python SettlementEstablishment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettlementEstablishment {
    pub entity_uuid: String,
    pub settlement_name: Option<String>,
    pub settlement_type: Option<String>,
    pub population: Option<i32>,
    pub controlling_faction: Option<String>,
    pub services: Vec<String>,
    pub notable_npcs: Vec<String>,
    pub defense_level: Option<i32>,
    pub trade_goods: Vec<String>,
    pub hex_location: Option<HexKey>,
}

impl SettlementEstablishment {
    pub fn new(entity_uuid: String) -> Self {
        Self {
            entity_uuid,
            settlement_name: None,
            settlement_type: None,
            population: None,
            controlling_faction: None,
            services: Vec::new(),
            notable_npcs: Vec::new(),
            defense_level: None,
            trade_goods: Vec::new(),
            hex_location: None,
        }
    }

    /// Extract referenced UUIDs (faction, NPCs, etc.)
    pub fn extract_referenced_uuids(&self) -> Vec<String> {
        let mut uuids = Vec::new();
        
        if let Some(faction_uuid) = &self.controlling_faction {
            uuids.push(faction_uuid.clone());
        }
        
        uuids.extend(self.notable_npcs.iter().cloned());
        
        uuids
    }

    /// Check if settlement is fortified
    pub fn is_fortified(&self) -> bool {
        self.defense_level.map_or(false, |level| level > 3)
    }

    /// Get settlement size category based on population
    pub fn get_size_category(&self) -> SettlementSize {
        match self.population {
            Some(pop) if pop < 100 => SettlementSize::Hamlet,
            Some(pop) if pop < 1000 => SettlementSize::Village,
            Some(pop) if pop < 5000 => SettlementSize::Town,
            Some(pop) if pop >= 5000 => SettlementSize::City,
            Some(_) => SettlementSize::Unknown, // Catch-all for other values
            None => SettlementSize::Unknown,
        }
    }
}

/// Settlement size categories
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SettlementSize {
    Hamlet,
    Village, 
    Town,
    City,
    Unknown,
}

/// Faction entity matching Python FactionEntity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactionEntity {
    pub entity_uuid: String,
    pub faction_name: Option<String>,
    pub faction_type: Option<String>,
    pub allegiances: Vec<String>,
    pub enemies: Vec<String>,
    pub territories: Vec<HexKey>,
    pub leader: Option<String>,
    pub goals: Vec<String>,
    pub resources: Vec<String>,
    pub influence_level: Option<i32>,
}

impl FactionEntity {
    pub fn new(entity_uuid: String) -> Self {
        Self {
            entity_uuid,
            faction_name: None,
            faction_type: None,
            allegiances: Vec::new(),
            enemies: Vec::new(),
            territories: Vec::new(),
            leader: None,
            goals: Vec::new(),
            resources: Vec::new(),
            influence_level: None,
        }
    }

    /// Extract referenced UUIDs (allies, enemies, leader, etc.)
    pub fn extract_referenced_uuids(&self) -> Vec<String> {
        let mut uuids = Vec::new();
        
        uuids.extend(self.allegiances.iter().cloned());
        uuids.extend(self.enemies.iter().cloned());
        
        if let Some(leader_uuid) = &self.leader {
            uuids.push(leader_uuid.clone());
        }
        
        uuids
    }

    /// Check if faction controls a specific hex
    pub fn controls_hex(&self, hex_key: &HexKey) -> bool {
        self.territories.contains(hex_key)
    }

    /// Get faction power level category
    pub fn get_power_level(&self) -> FactionPower {
        match self.influence_level {
            Some(level) if level < 3 => FactionPower::Minor,
            Some(level) if level < 7 => FactionPower::Moderate,
            Some(level) if level >= 7 => FactionPower::Major,
            Some(_) => FactionPower::Unknown, // Catch-all for other values
            None => FactionPower::Unknown,
        }
    }

    /// Check if faction is at war with another
    pub fn is_enemy(&self, other_faction_uuid: &str) -> bool {
        self.enemies.contains(&other_faction_uuid.to_string())
    }

    /// Check if faction is allied with another
    pub fn is_ally(&self, other_faction_uuid: &str) -> bool {
        self.allegiances.contains(&other_faction_uuid.to_string())
    }
}

/// Faction power level categories
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum FactionPower {
    Minor,
    Moderate,
    Major,
    Unknown,
}

/// Common entity traits and utilities
pub trait Entity {
    /// Get the entity's UUID
    fn entity_uuid(&self) -> &str;
    
    /// Extract all UUIDs this entity references
    fn extract_referenced_uuids(&self) -> Vec<String>;
    
    /// Get the entity type name
    fn entity_type(&self) -> &'static str;
}

impl Entity for RegionHexTile {
    fn entity_uuid(&self) -> &str {
        &self.entity_uuid
    }
    
    fn extract_referenced_uuids(&self) -> Vec<String> {
        self.extract_referenced_uuids()
    }
    
    fn entity_type(&self) -> &'static str {
        "region_hex_tile"
    }
}

impl Entity for SettlementEstablishment {
    fn entity_uuid(&self) -> &str {
        &self.entity_uuid
    }
    
    fn extract_referenced_uuids(&self) -> Vec<String> {
        self.extract_referenced_uuids()
    }
    
    fn entity_type(&self) -> &'static str {
        "settlement_establishment"
    }
}

impl Entity for FactionEntity {
    fn entity_uuid(&self) -> &str {
        &self.entity_uuid
    }
    
    fn extract_referenced_uuids(&self) -> Vec<String> {
        self.extract_referenced_uuids()
    }
    
    fn entity_type(&self) -> &'static str {
        "faction_entity"
    }
}

/// Entity relationship types for connection tracking
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RelationshipType {
    /// Settlement is controlled by faction
    SettlementControlled,
    /// Settlement is located in hex
    SettlementInHex,
    /// Dungeon entrance is in hex
    DungeonInHex,
    /// Faction has presence in hex
    FactionInHex,
    /// Factions are allied
    FactionAlliance,
    /// Factions are enemies
    FactionEnmity,
    /// Entity contains or references another
    Contains,
    /// Generic reference/connection
    References,
}

/// Entity connection for tracking relationships
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityConnection {
    pub from_uuid: String,
    pub to_uuid: String,
    pub relationship_type: RelationshipType,
    pub metadata: Option<HashMap<String, String>>,
}

impl EntityConnection {
    pub fn new(
        from_uuid: String, 
        to_uuid: String, 
        relationship_type: RelationshipType
    ) -> Self {
        Self {
            from_uuid,
            to_uuid,
            relationship_type,
            metadata: None,
        }
    }

    pub fn with_metadata(mut self, metadata: HashMap<String, String>) -> Self {
        self.metadata = Some(metadata);
        self
    }

    /// Check if this connection involves a specific entity
    pub fn involves(&self, entity_uuid: &str) -> bool {
        self.from_uuid == entity_uuid || self.to_uuid == entity_uuid
    }

    /// Get the other entity in this connection
    pub fn get_other(&self, entity_uuid: &str) -> Option<&str> {
        if self.from_uuid == entity_uuid {
            Some(&self.to_uuid)
        } else if self.to_uuid == entity_uuid {
            Some(&self.from_uuid)
        } else {
            None
        }
    }
}

/// Utility functions for working with entities
pub mod utils {
    use super::*;
    
    /// Extract all connections from a collection of entities
    pub fn extract_entity_connections<T: Entity>(entities: &[T]) -> Vec<EntityConnection> {
        let mut connections = Vec::new();
        
        for entity in entities {
            let from_uuid = entity.entity_uuid().to_string();
            
            for to_uuid in entity.extract_referenced_uuids() {
                let connection = EntityConnection::new(
                    from_uuid.clone(),
                    to_uuid,
                    RelationshipType::References,
                );
                connections.push(connection);
            }
        }
        
        connections
    }

    /// Find entities that reference a specific UUID
    pub fn find_referencing_entities<'a, T: Entity>(
        entities: &'a [T], 
        target_uuid: &str
    ) -> Vec<&'a T> {
        entities.iter()
            .filter(|entity| {
                entity.extract_referenced_uuids()
                    .contains(&target_uuid.to_string())
            })
            .collect()
    }

    /// Group entities by a key function
    pub fn group_entities_by<T, K, F>(entities: Vec<T>, key_fn: F) -> HashMap<K, Vec<T>>
    where
        K: std::hash::Hash + Eq,
        F: Fn(&T) -> K,
    {
        let mut groups: HashMap<K, Vec<T>> = HashMap::new();
        
        for entity in entities {
            let key = key_fn(&entity);
            groups.entry(key).or_insert_with(Vec::new).push(entity);
        }
        
        groups
    }

    /// Filter entities by hex location
    pub fn filter_entities_by_hex<'a, T>(
        entities: &'a [T],
        hex_key: &HexKey,
        location_fn: fn(&T) -> Option<&HexKey>,
    ) -> Vec<&'a T> {
        entities.iter()
            .filter(|entity| {
                location_fn(entity).map_or(false, |hex| hex == hex_key)
            })
            .collect()
    }

    /// Calculate distance between two hex coordinates (simplified)
    pub fn hex_distance(hex1: &HexKey, hex2: &HexKey) -> Option<u32> {
        // This is a placeholder - would need proper hex coordinate parsing
        // and distance calculation using axial coordinates
        if hex1 == hex2 {
            Some(0)
        } else {
            // Parse hex coordinates and calculate actual distance
            // For now, return None to indicate we can't calculate
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_region_hex_tile_creation() {
        let tile = RegionHexTile::new("test-uuid".to_string());
        assert_eq!(tile.entity_uuid, "test-uuid");
        assert!(!tile.has_entities());
        assert_eq!(tile.entity_count(), 0);
    }

    #[test]
    fn test_settlement_size_categories() {
        let mut settlement = SettlementEstablishment::new("test-uuid".to_string());
        
        settlement.population = Some(50);
        assert_eq!(settlement.get_size_category(), SettlementSize::Hamlet);
        
        settlement.population = Some(500);
        assert_eq!(settlement.get_size_category(), SettlementSize::Village);
        
        settlement.population = Some(2000);
        assert_eq!(settlement.get_size_category(), SettlementSize::Town);
        
        settlement.population = Some(10000);
        assert_eq!(settlement.get_size_category(), SettlementSize::City);
    }

    #[test]
    fn test_faction_relationships() {
        let mut faction = FactionEntity::new("faction1".to_string());
        faction.allegiances.push("faction2".to_string());
        faction.enemies.push("faction3".to_string());
        
        assert!(faction.is_ally("faction2"));
        assert!(faction.is_enemy("faction3"));
        assert!(!faction.is_ally("faction3"));
        assert!(!faction.is_enemy("faction2"));
    }

    #[test]
    fn test_entity_connections() {
        let connection = EntityConnection::new(
            "entity1".to_string(),
            "entity2".to_string(),
            RelationshipType::References,
        );
        
        assert!(connection.involves("entity1"));
        assert!(connection.involves("entity2"));
        assert!(!connection.involves("entity3"));
        
        assert_eq!(connection.get_other("entity1"), Some("entity2"));
        assert_eq!(connection.get_other("entity2"), Some("entity1"));
        assert_eq!(connection.get_other("entity3"), None);
    }

    #[test]
    fn test_referenced_uuids_extraction() {
        let mut tile = RegionHexTile::new("tile1".to_string());
        tile.settlement_uuids.push("settlement1".to_string());
        tile.faction_uuids.push("faction1".to_string());
        
        let uuids = tile.extract_referenced_uuids();
        assert!(uuids.contains(&"settlement1".to_string()));
        assert!(uuids.contains(&"faction1".to_string()));
    }

    #[test]
    fn test_settlement_fortification() {
        let mut settlement = SettlementEstablishment::new("test".to_string());
        
        settlement.defense_level = Some(2);
        assert!(!settlement.is_fortified());
        
        settlement.defense_level = Some(5);
        assert!(settlement.is_fortified());
    }

    #[test]
    fn test_faction_power_levels() {
        let mut faction = FactionEntity::new("test".to_string());
        
        faction.influence_level = Some(2);
        assert_eq!(faction.get_power_level(), FactionPower::Minor);
        
        faction.influence_level = Some(5);
        assert_eq!(faction.get_power_level(), FactionPower::Moderate);
        
        faction.influence_level = Some(8);
        assert_eq!(faction.get_power_level(), FactionPower::Major);
    }
    
    #[test]
    fn test_hex_tile_audit_functionality() {
        let mut tile = RegionHexTile::new("test-uuid".to_string());
        
        // Empty tile should have low completeness score
        assert!(tile.data_completeness_score() < 0.5);
        
        // Add critical data
        tile.hex_key = Some("q1r2".to_string());
        tile.region_uuid = Some("region1".to_string());
        tile.biome_type = Some("grassland".to_string());
        
        // Should have high completeness score now
        assert!(tile.data_completeness_score() > 0.7);
        
        // Test audit row generation
        let audit_row = tile.audit_row();
        assert_eq!(audit_row[0], "test-uuid"); // entity_uuid
        assert_eq!(audit_row[1], "q1r2"); // hex_key
        assert_eq!(audit_row[7], "grassland"); // biome_type
    }
}
