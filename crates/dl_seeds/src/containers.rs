//! Container system for spatial processing and entity relationships.
//!
//! This module provides efficient spatial indexing for Dragon's Labyrinth
//! entities using container-based algorithms for O(1) lookups.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Spatial container for hex-based entity lookups
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HexContainer {
    /// Map from hex coordinates to entity lists
    pub hex_entities: HashMap<String, Vec<String>>,
    /// Spatial regions for performance
    pub region_bounds: HashMap<String, Vec<String>>,
}

impl HexContainer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_entity(&mut self, hex_key: String, entity_uuid: String) {
        self.hex_entities
            .entry(hex_key)
            .or_insert_with(Vec::new)
            .push(entity_uuid);
    }

    pub fn get_entities_at_hex(&self, hex_key: &str) -> Vec<&String> {
        self.hex_entities
            .get(hex_key)
            .map(|entities| entities.iter().collect())
            .unwrap_or_else(Vec::new)
    }
}

/// Container for dungeon area relationships
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DungeonContainer {
    /// Map from dungeon UUID to area UUIDs
    pub dungeon_areas: HashMap<String, Vec<String>>,
    /// Map from area UUID to connected area UUIDs
    pub area_connections: HashMap<String, Vec<String>>,
}

impl DungeonContainer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_dungeon_area(&mut self, dungeon_uuid: String, area_uuid: String) {
        self.dungeon_areas
            .entry(dungeon_uuid)
            .or_insert_with(Vec::new)
            .push(area_uuid);
    }

    pub fn connect_areas(&mut self, area1: String, area2: String) {
        self.area_connections
            .entry(area1.clone())
            .or_insert_with(Vec::new)
            .push(area2.clone());
        self.area_connections
            .entry(area2)
            .or_insert_with(Vec::new)
            .push(area1);
    }
}

/// Entity clustering container for AI analysis
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ClusteringContainer {
    /// Entity clusters organized by category
    pub category_clusters: HashMap<String, Vec<String>>,
    /// Cluster metadata for analysis
    pub cluster_metadata: HashMap<String, ClusterMeta>,
}

impl ClusteringContainer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_entity_to_cluster(&mut self, cluster_id: String, entity_uuid: String) {
        self.category_clusters
            .entry(cluster_id)
            .or_insert_with(Vec::new)
            .push(entity_uuid);
    }

    pub fn set_cluster_meta(&mut self, cluster_id: String, meta: ClusterMeta) {
        self.cluster_metadata.insert(cluster_id, meta);
    }

    pub fn get_cluster_entities(&self, cluster_id: &str) -> Vec<&String> {
        self.category_clusters
            .get(cluster_id)
            .map(|entities| entities.iter().collect())
            .unwrap_or_else(Vec::new)
    }
}

/// Metadata about an entity cluster
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterMeta {
    pub category: String,
    pub size: usize,
    pub analysis_complete: bool,
    pub ai_summary: Option<String>,
}

impl ClusterMeta {
    pub fn new(category: String, size: usize) -> Self {
        Self {
            category,
            size,
            analysis_complete: false,
            ai_summary: None,
        }
    }
}
