//! Container system with spatial indexing for efficient entity lookups.
//!
//! This module provides the critical spatial indexing system matching Python containers.py:
//! - DungeonContainer with area-based spatial indexing
//! - RegionContainer with hex-based entity lookups
//! - O(1) performance HashMap indexes for spatial queries
//! - Phase 2/3 pipeline container generation

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use anyhow::Result;

use crate::base::HexKey;
use crate::dungeons::DungeonArea;

/// Container for dungeon entities with spatial indexing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DungeonContainer {
    pub dungeon_uuid: String,
    pub dungeon_name: Option<String>,
    pub areas: Vec<DungeonArea>,
    
    // Spatial indexes for O(1) lookups
    pub neighbors: HashMap<String, Vec<String>>, // area_key -> connected area_keys
    pub by_area: HashMap<String, DungeonArea>,   // area_key -> area data
    pub by_hex: HashMap<String, Vec<String>>,    // hex_key -> area_keys at that hex
    pub by_difficulty: HashMap<i32, Vec<String>>, // difficulty -> area_keys
}

impl DungeonContainer {
    pub fn new(dungeon_uuid: String) -> Self {
        Self {
            dungeon_uuid,
            dungeon_name: None,
            areas: Vec::new(),
            neighbors: HashMap::new(),
            by_area: HashMap::new(),
            by_hex: HashMap::new(),
            by_difficulty: HashMap::new(),
        }
    }

    /// Add a dungeon area to the container
    pub fn add_area(&mut self, area: DungeonArea) {
        let area_key = self.generate_area_key(&area);
        
        // Add to main areas list
        self.areas.push(area.clone());
        
        // Add to by_area index
        self.by_area.insert(area_key.clone(), area.clone());
        
        // Add to hex-based index
        if let Some(hex) = &area.entrance_hex {
            self.by_hex.entry(hex.clone()).or_insert_with(Vec::new).push(area_key.clone());
        }
        
        // Add to difficulty index
        if let Some(difficulty) = area.difficulty_level {
            self.by_difficulty.entry(difficulty).or_insert_with(Vec::new).push(area_key);
        }
    }

    /// Build all spatial indexes from current areas
    pub fn build_indexes(&mut self) {
        self.neighbors.clear();
        self.by_area.clear();
        self.by_hex.clear();
        self.by_difficulty.clear();

        // Build indexes from all areas
        for area in &self.areas {
            let area_key = self.generate_area_key(area);
            
            // Area lookup index
            self.by_area.insert(area_key.clone(), area.clone());
            
            // Hex-based spatial index
            if let Some(hex) = &area.entrance_hex {
                self.by_hex.entry(hex.clone()).or_insert_with(Vec::new).push(area_key.clone());
            }
            
            // Difficulty-based index
            if let Some(difficulty) = area.difficulty_level {
                self.by_difficulty.entry(difficulty).or_insert_with(Vec::new).push(area_key.clone());
            }
            
            // Neighbor connections
            let connected_keys: Vec<String> = area.connected_areas.iter()
                .map(|area_num| format!("area_{}", area_num))
                .collect();
            self.neighbors.insert(area_key, connected_keys);
        }
    }

    /// Get all areas at a specific hex coordinate (O(1) lookup)
    pub fn get_areas_at_hex(&self, hex_key: &HexKey) -> Vec<&DungeonArea> {
        self.by_hex.get(hex_key)
            .map(|area_keys| {
                area_keys.iter()
                    .filter_map(|key| self.by_area.get(key))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get areas by difficulty level (O(1) lookup)
    pub fn get_areas_by_difficulty(&self, difficulty: i32) -> Vec<&DungeonArea> {
        self.by_difficulty.get(&difficulty)
            .map(|area_keys| {
                area_keys.iter()
                    .filter_map(|key| self.by_area.get(key))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get connected areas for a given area (O(1) lookup)
    pub fn get_connected_areas(&self, area_key: &str) -> Vec<&DungeonArea> {
        self.neighbors.get(area_key)
            .map(|connected_keys| {
                connected_keys.iter()
                    .filter_map(|key| self.by_area.get(key))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Find path between two areas using connection graph
    pub fn find_path(&self, from_area: &str, to_area: &str) -> Option<Vec<String>> {
        // Simple BFS pathfinding using the neighbor indexes
        use std::collections::{VecDeque, HashSet};
        
        if from_area == to_area {
            return Some(vec![from_area.to_string()]);
        }

        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut parent = HashMap::new();
        
        queue.push_back(from_area.to_string());
        visited.insert(from_area.to_string());
        
        while let Some(current) = queue.pop_front() {
            if let Some(neighbors) = self.neighbors.get(&current) {
                for neighbor in neighbors {
                    if !visited.contains(neighbor) {
                        visited.insert(neighbor.clone());
                        parent.insert(neighbor.clone(), current.clone());
                        queue.push_back(neighbor.clone());
                        
                        if neighbor == to_area {
                            // Reconstruct path
                            let mut path = Vec::new();
                            let mut current = to_area.to_string();
                            
                            while let Some(prev) = parent.get(&current) {
                                path.push(current.clone());
                                current = prev.clone();
                            }
                            path.push(from_area.to_string());
                            path.reverse();
                            
                            return Some(path);
                        }
                    }
                }
            }
        }
        
        None // No path found
    }

    /// Generate a consistent area key from area data
    fn generate_area_key(&self, area: &DungeonArea) -> String {
        if let Some(area_num) = area.area_number {
            format!("area_{}", area_num)
        } else {
            format!("area_{}", area.entity_uuid)
        }
    }

    /// Get statistics about the dungeon
    pub fn get_statistics(&self) -> DungeonStatistics {
        let total_areas = self.areas.len();
        let areas_with_monsters = self.areas.iter()
            .filter(|area| !area.monsters.is_empty())
            .count();
        let areas_with_treasure = self.areas.iter()
            .filter(|area| area.treasure.is_some())
            .count();
        let areas_with_traps = self.areas.iter()
            .filter(|area| !area.traps.is_empty())
            .count();
        
        let average_difficulty = if total_areas > 0 {
            let total_difficulty: i32 = self.areas.iter()
                .filter_map(|area| area.difficulty_level)
                .sum();
            let difficulty_count = self.areas.iter()
                .filter(|area| area.difficulty_level.is_some())
                .count();
            if difficulty_count > 0 {
                Some(total_difficulty as f32 / difficulty_count as f32)
            } else {
                None
            }
        } else {
            None
        };

        DungeonStatistics {
            total_areas,
            areas_with_monsters,
            areas_with_treasure,
            areas_with_traps,
            average_difficulty,
        }
    }
}

/// Statistics about a dungeon container
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DungeonStatistics {
    pub total_areas: usize,
    pub areas_with_monsters: usize,
    pub areas_with_treasure: usize,
    pub areas_with_traps: usize,
    pub average_difficulty: Option<f32>,
}

/// Container for region entities with hex-based spatial indexing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionContainer {
    pub region_uuid: String,
    pub region_name: Option<String>,
    
    // Entity collections
    pub hex_tiles: Vec<RegionHexTile>,
    pub settlements: Vec<SettlementEstablishment>,
    pub factions: Vec<FactionEntity>,
    pub dungeon_containers: Vec<DungeonContainer>,
    
    // Spatial indexes for O(1) lookups - matches Python containers.py pattern
    pub by_hex: HashMap<HexKey, HashMap<String, Vec<String>>>, // hex -> {entity_type: [uuids]}
    pub by_settlement: HashMap<String, RegionHexTile>, // settlement_uuid -> containing hex_tile
    pub by_faction: HashMap<String, Vec<String>>, // faction_uuid -> hex_keys where present
    pub by_dungeon: HashMap<String, Vec<String>>, // dungeon_uuid -> hex_keys where present
}

impl RegionContainer {
    pub fn new(region_uuid: String) -> Self {
        Self {
            region_uuid,
            region_name: None,
            hex_tiles: Vec::new(),
            settlements: Vec::new(),
            factions: Vec::new(),
            dungeon_containers: Vec::new(),
            by_hex: HashMap::new(),
            by_settlement: HashMap::new(),
            by_faction: HashMap::new(),
            by_dungeon: HashMap::new(),
        }
    }

    /// Build all spatial indexes from current entities (matches Python build_indexes)
    pub fn build_indexes(&mut self) {
        self.by_hex.clear();
        self.by_settlement.clear();
        self.by_faction.clear();
        self.by_dungeon.clear();

        // Index hex tiles
        for tile in &self.hex_tiles {
            if let Some(hex_key) = &tile.hex_key {
                let hex_entry = self.by_hex.entry(hex_key.clone()).or_insert_with(HashMap::new);
                
                // Add tile itself
                hex_entry.entry("tiles".to_string()).or_insert_with(Vec::new)
                    .push(tile.entity_uuid.clone());
                
                // Index settlements in this hex
                for settlement_uuid in &tile.settlement_uuids {
                    hex_entry.entry("settlements".to_string()).or_insert_with(Vec::new)
                        .push(settlement_uuid.clone());
                    self.by_settlement.insert(settlement_uuid.clone(), tile.clone());
                }
                
                // Index dungeons in this hex
                for dungeon_uuid in &tile.dungeon_uuids {
                    hex_entry.entry("dungeons".to_string()).or_insert_with(Vec::new)
                        .push(dungeon_uuid.clone());
                    self.by_dungeon.entry(dungeon_uuid.clone()).or_insert_with(Vec::new)
                        .push(hex_key.clone());
                }
                
                // Index factions in this hex
                for faction_uuid in &tile.faction_uuids {
                    hex_entry.entry("factions".to_string()).or_insert_with(Vec::new)
                        .push(faction_uuid.clone());
                    self.by_faction.entry(faction_uuid.clone()).or_insert_with(Vec::new)
                        .push(hex_key.clone());
                }
            }
        }
    }

    /// Get all entities at a specific hex coordinate (O(1) lookup matching Python)
    pub fn get_entities_at_hex(&self, hex_key: &HexKey) -> HashMap<String, Vec<EntityReference>> {
        let mut result: HashMap<String, Vec<EntityReference>> = HashMap::new();
        
        if let Some(hex_data) = self.by_hex.get(hex_key) {
            // Get hex tiles
            if let Some(tile_uuids) = hex_data.get("tiles") {
                let tiles: Vec<EntityReference> = self.hex_tiles.iter()
                    .filter(|tile| tile_uuids.contains(&tile.entity_uuid))
                    .map(|tile| EntityReference {
                        uuid: tile.entity_uuid.clone(),
                        entity_type: "hex_tile".to_string(),
                        name: tile.hex_key.clone(),
                    })
                    .collect();
                result.insert("tiles".to_string(), tiles);
            }
            
            // Get settlements
            if let Some(settlement_uuids) = hex_data.get("settlements") {
                let settlements: Vec<EntityReference> = self.settlements.iter()
                    .filter(|settlement| settlement_uuids.contains(&settlement.entity_uuid))
                    .map(|settlement| EntityReference {
                        uuid: settlement.entity_uuid.clone(),
                        entity_type: "settlement".to_string(),
                        name: settlement.settlement_name.clone(),
                    })
                    .collect();
                result.insert("settlements".to_string(), settlements);
            }
            
            // Get dungeons
            if let Some(dungeon_uuids) = hex_data.get("dungeons") {
                let dungeons: Vec<EntityReference> = self.dungeon_containers.iter()
                    .filter(|dungeon| dungeon_uuids.contains(&dungeon.dungeon_uuid))
                    .map(|dungeon| EntityReference {
                        uuid: dungeon.dungeon_uuid.clone(),
                        entity_type: "dungeon".to_string(),
                        name: dungeon.dungeon_name.clone(),
                    })
                    .collect();
                result.insert("dungeons".to_string(), dungeons);
            }
            
            // Get factions
            if let Some(faction_uuids) = hex_data.get("factions") {
                let factions: Vec<EntityReference> = self.factions.iter()
                    .filter(|faction| faction_uuids.contains(&faction.entity_uuid))
                    .map(|faction| EntityReference {
                        uuid: faction.entity_uuid.clone(),
                        entity_type: "faction".to_string(),
                        name: faction.faction_name.clone(),
                    })
                    .collect();
                result.insert("factions".to_string(), factions);
            }
        }
        
        result
    }

    /// Get hex tiles within a radius of a given hex
    pub fn get_hex_tiles_in_radius(&self, center_hex: &HexKey, radius: u32) -> Vec<&RegionHexTile> {
        // This would use proper hex math - for now, simple string matching
        self.hex_tiles.iter()
            .filter(|tile| {
                if let Some(hex_key) = &tile.hex_key {
                    // Simple distance check - would need proper hex coordinate math
                    hex_key == center_hex || radius > 0
                } else {
                    false
                }
            })
            .collect()
    }

    /// Find all hexes where a specific faction is present
    pub fn get_faction_territory(&self, faction_uuid: &str) -> Vec<HexKey> {
        self.by_faction.get(faction_uuid)
            .cloned()
            .unwrap_or_default()
    }

    /// Get all dungeons accessible from a hex
    pub fn get_dungeons_near_hex(&self, hex_key: &HexKey) -> Vec<&DungeonContainer> {
        if let Some(hex_data) = self.by_hex.get(hex_key) {
            if let Some(dungeon_uuids) = hex_data.get("dungeons") {
                return self.dungeon_containers.iter()
                    .filter(|dungeon| dungeon_uuids.contains(&dungeon.dungeon_uuid))
                    .collect();
            }
        }
        Vec::new()
    }
}

impl DungeonContainer {
    /// Get dungeons near hex coordinates (build.rs API)
    pub fn get_dungeons_near_hex(&self, coords: (i32, i32), radius: i32) -> Vec<String> {
        // Simple implementation - return dungeon UUIDs near coordinates
        // In a real implementation, this would use proper hex math
        vec![self.dungeon_uuid.clone()]
    }

    /// Get pathfinding data for an area (build.rs API)
    pub fn get_pathfinding_data(&self, area_uuid: &str) -> PathfindingData {
        // Return pathfinding data for the area
        PathfindingData {
            nodes: vec![(0, 0), (1, 0), (0, 1)], // Default nodes
            connections: vec![((0, 0), (1, 0)), ((1, 0), (0, 1))], // Default connections
        }
    }
}

/// Pathfinding data structure for dungeon areas
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathfindingData {
    pub nodes: Vec<(i32, i32)>,
    pub connections: Vec<((i32, i32), (i32, i32))>,
}

/// Reference to an entity in spatial queries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityReference {
    pub uuid: String,
    pub entity_type: String,
    pub name: Option<String>,
}

/// Placeholder entity types - these should match the actual entity models
/// TODO: These will be replaced by proper entity models from other modules

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionHexTile {
    pub entity_uuid: String,
    pub hex_key: Option<String>,
    pub region_uuid: Option<String>,
    pub settlement_uuids: Vec<String>,
    pub dungeon_uuids: Vec<String>,
    pub faction_uuids: Vec<String>,
    pub biome_type: Option<String>,
    pub terrain_features: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettlementEstablishment {
    pub entity_uuid: String,
    pub settlement_name: Option<String>,
    pub settlement_type: Option<String>,
    pub population: Option<i32>,
    pub controlling_faction: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactionEntity {
    pub entity_uuid: String,
    pub faction_name: Option<String>,
    pub faction_type: Option<String>,
    pub allegiances: Vec<String>,
    pub territories: Vec<String>,
}

/// Container generation utilities for Phase 2/3 pipeline
pub mod generation {
    use super::*;
    use std::path::PathBuf;

    /// Generate dungeon containers from individual dungeon areas (Phase 2)
    pub fn generate_dungeon_containers(
        areas_by_dungeon: HashMap<String, Vec<DungeonArea>>,
        output_dir: &Path,
    ) -> Result<Vec<DungeonContainer>> {
        let mut containers = Vec::new();

        for (dungeon_uuid, areas) in areas_by_dungeon {
            let mut container = DungeonContainer::new(dungeon_uuid.clone());
            
            // Set dungeon name from first area
            if let Some(first_area) = areas.first() {
                container.dungeon_name = first_area.dungeon_name.clone();
            }
            
            // Add all areas
            for area in areas {
                container.add_area(area);
            }
            
            // Build spatial indexes
            container.build_indexes();
            
            // Write container to file
            let container_file = output_dir.join(format!("dungeon_{}.ron", dungeon_uuid));
            let container_content = ron::ser::to_string_pretty(&container, Default::default())?;
            std::fs::write(container_file, container_content)?;
            
            containers.push(container);
        }

        Ok(containers)
    }

    /// Generate region containers from individual entities (Phase 3)
    pub fn generate_region_containers(
        hex_tiles: Vec<RegionHexTile>,
        settlements: Vec<SettlementEstablishment>,
        factions: Vec<FactionEntity>,
        dungeon_containers: Vec<DungeonContainer>,
        output_dir: &Path,
    ) -> Result<Vec<RegionContainer>> {
        // Group entities by region
        let mut regions_map: HashMap<String, Vec<RegionHexTile>> = HashMap::new();
        
        for tile in hex_tiles {
            if let Some(region_uuid) = &tile.region_uuid {
                regions_map.entry(region_uuid.clone()).or_insert_with(Vec::new).push(tile);
            }
        }

        let mut containers = Vec::new();

        for (region_uuid, tiles) in regions_map {
            let mut container = RegionContainer::new(region_uuid.clone());
            container.hex_tiles = tiles;
            
            // Add settlements and factions (simplified - would need proper region association)
            container.settlements = settlements.clone();
            container.factions = factions.clone();
            container.dungeon_containers = dungeon_containers.clone();
            
            // Build spatial indexes
            container.build_indexes();
            
            // Write container to file
            let container_file = output_dir.join(format!("region_{}.ron", region_uuid));
            let container_content = ron::ser::to_string_pretty(&container, Default::default())?;
            std::fs::write(container_file, container_content)?;
            
            containers.push(container);
        }

        Ok(containers)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dungeon_container_creation() {
        let container = DungeonContainer::new("test-dungeon".to_string());
        assert_eq!(container.dungeon_uuid, "test-dungeon");
        assert!(container.areas.is_empty());
    }

    #[test]
    fn test_dungeon_spatial_indexing() {
        let mut container = DungeonContainer::new("test".to_string());
        
        let mut area = DungeonArea::new("area1".to_string());
        area.area_number = Some(1);
        area.entrance_hex = Some("W2S51".to_string());
        area.difficulty_level = Some(5);
        
        container.add_area(area);
        container.build_indexes();
        
        // Test hex lookup
        let areas_at_hex = container.get_areas_at_hex("W2S51");
        assert_eq!(areas_at_hex.len(), 1);
        
        // Test difficulty lookup
        let level5_areas = container.get_areas_by_difficulty(5);
        assert_eq!(level5_areas.len(), 1);
    }

    #[test]
    fn test_region_container_creation() {
        let container = RegionContainer::new("test-region".to_string());
        assert_eq!(container.region_uuid, "test-region");
        assert!(container.hex_tiles.is_empty());
    }

    #[test]
    fn test_pathfinding() {
        let mut container = DungeonContainer::new("test".to_string());
        
        // Create connected areas: 1 -> 2 -> 3
        let mut area1 = DungeonArea::new("area1".to_string());
        area1.area_number = Some(1);
        area1.connected_areas = vec![2];
        
        let mut area2 = DungeonArea::new("area2".to_string());
        area2.area_number = Some(2);
        area2.connected_areas = vec![1, 3];
        
        let mut area3 = DungeonArea::new("area3".to_string());
        area3.area_number = Some(3);
        area3.connected_areas = vec![2];
        
        container.add_area(area1);
        container.add_area(area2);
        container.add_area(area3);
        container.build_indexes();
        
        let path = container.find_path("area_1", "area_3");
        assert!(path.is_some());
        let path = path.unwrap();
        assert_eq!(path.len(), 3); // area_1 -> area_2 -> area_3
    }
}
