//! Spatial container system for O(1) hex entity lookups
//! Integrates with dl_analysis container system for efficient spatial queries

use bevy::prelude::*;
use std::collections::HashMap;
use crate::world::components::*;

/// Spatial container for O(1) hex entity lookups
/// Mirrors the dl_analysis::containers::RegionContainer functionality for game runtime
#[derive(Resource, Default, Debug)]
pub struct SpatialContainer {
    /// Map hex coordinates to entities at that location
    hex_entities: HashMap<(i32, i32), Vec<Entity>>,
    /// Map region UUIDs to their entities
    region_entities: HashMap<String, Vec<Entity>>,
    /// Map settlement UUIDs to their entities
    settlement_entities: HashMap<String, Entity>,
    /// Map dungeon UUIDs to their entities
    dungeon_entities: HashMap<String, Entity>,
    /// Map faction UUIDs to their entities
    faction_entities: HashMap<String, Entity>,
    /// Map NPC UUIDs to their entities
    npc_entities: HashMap<String, Entity>,
}

impl SpatialContainer {
    /// Create a new empty spatial container
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Register an entity at a hex coordinate
    pub fn register_hex_entity(&mut self, coords: (i32, i32), entity: Entity) {
        self.hex_entities.entry(coords).or_insert_with(Vec::new).push(entity);
    }
    
    /// Get all entities at a specific hex coordinate
    pub fn get_entities_at_hex(&self, coords: (i32, i32)) -> Vec<Entity> {
        self.hex_entities.get(&coords).cloned().unwrap_or_default()
    }
    
    /// Register a region entity
    pub fn register_region_entity(&mut self, uuid: String, entity: Entity) {
        self.region_entities.entry(uuid).or_insert_with(Vec::new).push(entity);
    }
    
    /// Get all entities for a region
    pub fn get_region_entities(&self, uuid: &str) -> Vec<Entity> {
        self.region_entities.get(uuid).cloned().unwrap_or_default()
    }
    
    /// Register a settlement entity
    pub fn register_settlement_entity(&mut self, uuid: String, entity: Entity) {
        self.settlement_entities.insert(uuid, entity);
    }
    
    /// Get settlement entity by UUID
    pub fn get_settlement_entity(&self, uuid: &str) -> Option<Entity> {
        self.settlement_entities.get(uuid).copied()
    }
    
    /// Register a dungeon entity
    pub fn register_dungeon_entity(&mut self, uuid: String, entity: Entity) {
        self.dungeon_entities.insert(uuid, entity);
    }
    
    /// Get dungeon entity by UUID
    pub fn get_dungeon_entity(&self, uuid: &str) -> Option<Entity> {
        self.dungeon_entities.get(uuid).copied()
    }
    
    /// Register a faction entity
    pub fn register_faction_entity(&mut self, uuid: String, entity: Entity) {
        self.faction_entities.insert(uuid, entity);
    }
    
    /// Get faction entity by UUID
    pub fn get_faction_entity(&self, uuid: &str) -> Option<Entity> {
        self.faction_entities.get(uuid).copied()
    }
    
    /// Register an NPC entity
    pub fn register_npc_entity(&mut self, uuid: String, entity: Entity) {
        self.npc_entities.insert(uuid, entity);
    }
    
    /// Get NPC entity by UUID
    pub fn get_npc_entity(&self, uuid: &str) -> Option<Entity> {
        self.npc_entities.get(uuid).copied()
    }
    
    /// Get all entities within a radius of a hex coordinate
    pub fn get_entities_in_radius(&self, center: (i32, i32), radius: u32) -> Vec<Entity> {
        let mut entities = Vec::new();
        let radius = radius as i32;
        
        for q in (center.0 - radius)..=(center.0 + radius) {
            for r in (center.1 - radius)..=(center.1 + radius) {
                if hex_distance(center, (q, r)) <= radius {
                    entities.extend(self.get_entities_at_hex((q, r)));
                }
            }
        }
        
        entities
    }
    
    /// Find the closest entity of a specific type to a hex coordinate
    pub fn find_closest_entity_with_component<T: Component>(
        &self,
        coords: (i32, i32),
        radius: u32,
        query: &Query<Entity, With<T>>,
    ) -> Option<(Entity, i32)> {
        let mut closest: Option<(Entity, i32)> = None;
        
        for entity in self.get_entities_in_radius(coords, radius) {
            if query.contains(entity) {
                // For now, return distance 0 - in a real implementation,
                // we'd calculate the actual hex distance to the entity
                let distance = 0;
                
                match closest {
                    None => closest = Some((entity, distance)),
                    Some((_, closest_distance)) if distance < closest_distance => {
                        closest = Some((entity, distance));
                    }
                    _ => {}
                }
            }
        }
        
        closest
    }
    
    /// Get statistics about the spatial container
    pub fn get_stats(&self) -> SpatialContainerStats {
        SpatialContainerStats {
            total_hex_locations: self.hex_entities.len(),
            total_entities_at_hexes: self.hex_entities.values().map(|v| v.len()).sum(),
            total_regions: self.region_entities.len(),
            total_settlements: self.settlement_entities.len(),
            total_dungeons: self.dungeon_entities.len(),
            total_factions: self.faction_entities.len(),
            total_npcs: self.npc_entities.len(),
        }
    }
    
    /// Clear all spatial data (useful for world reloading)
    pub fn clear(&mut self) {
        self.hex_entities.clear();
        self.region_entities.clear();
        self.settlement_entities.clear();
        self.dungeon_entities.clear();
        self.faction_entities.clear();
        self.npc_entities.clear();
    }
}

/// Statistics about the spatial container contents
#[derive(Debug, Clone)]
pub struct SpatialContainerStats {
    pub total_hex_locations: usize,
    pub total_entities_at_hexes: usize,
    pub total_regions: usize,
    pub total_settlements: usize,
    pub total_dungeons: usize,
    pub total_factions: usize,
    pub total_npcs: usize,
}

/// Calculate hex distance between two coordinates
pub fn hex_distance(a: (i32, i32), b: (i32, i32)) -> i32 {
    let dq = (a.0 - b.0).abs();
    let dr = (a.1 - b.1).abs();
    let ds = ((a.0 + a.1) - (b.0 + b.1)).abs();
    (dq + dr + ds) / 2
}

/// Get neighboring hex coordinates
pub fn hex_neighbors(coords: (i32, i32)) -> [(i32, i32); 6] {
    let (q, r) = coords;
    [
        (q + 1, r),     // E
        (q + 1, r - 1), // NE 
        (q, r - 1),     // NW
        (q - 1, r),     // W
        (q - 1, r + 1), // SW
        (q, r + 1),     // SE
    ]
}

/// Get all hex coordinates within a radius
pub fn hex_coordinates_in_radius(center: (i32, i32), radius: u32) -> Vec<(i32, i32)> {
    let mut coordinates = Vec::new();
    let radius = radius as i32;
    
    for q in (center.0 - radius)..=(center.0 + radius) {
        for r in (center.1 - radius)..=(center.1 + radius) {
            if hex_distance(center, (q, r)) <= radius {
                coordinates.push((q, r));
            }
        }
    }
    
    coordinates
}

/// System to debug print spatial container stats
pub fn debug_spatial_container_stats(spatial_container: Res<SpatialContainer>) {
    if spatial_container.is_changed() {
        let stats = spatial_container.get_stats();
        println!("Spatial Container Stats: {:#?}", stats);
    }
}

/// Plugin to register spatial container systems
pub struct SpatialPlugin;

impl Plugin for SpatialPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<SpatialContainer>()
            .add_systems(Update, debug_spatial_container_stats.run_if(
                resource_exists::<SpatialContainer>
            ));
    }
}
