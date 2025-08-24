//! Corruption Engine - Database-driven corruption spread and horror progression
//!
//! This system manages the spread of corruption across hex tiles and integrates
//! with Dragon's Labyrinth's core horror progression mechanics.

use anyhow::Result;
use database_orm::*;
use sea_orm::{DatabaseConnection, EntityTrait, ColumnTrait, QueryFilter};
use std::collections::HashMap;
use tracing::{debug, info, warn};
use uuid::Uuid;
use rand::Rng;
use super::HexPosition;

pub struct CorruptionEngine {
    db: DatabaseConnection,
    corruption_sources: HashMap<HexPosition, f32>, // position -> corruption intensity
    spread_rate: f32,
    dread_thresholds: [f32; 5], // Corruption levels for dread levels 0-4
}

impl CorruptionEngine {
    pub async fn new(db: &DatabaseConnection) -> Result<Self> {
        let mut engine = Self {
            db: db.clone(),
            corruption_sources: HashMap::new(),
            spread_rate: 0.1, // Base corruption spread per day
            dread_thresholds: [0.0, 0.2, 0.4, 0.6, 0.8], // Dread level boundaries
        };
        
        // Load existing corruption sources from database
        engine.load_corruption_sources().await?;
        
        info!("Corruption engine initialized with {} corruption sources",
              engine.corruption_sources.len());
        
        Ok(engine)
    }
    
    /// Spread corruption from sources across the world
    pub async fn process_corruption_spread(&mut self, time_passed_hours: u32) -> Result<CorruptionSpreadResult> {
        let days_passed = time_passed_hours as f32 / 24.0;
        let spread_amount = self.spread_rate * days_passed;
        
        let mut affected_tiles = Vec::new();
        let mut dread_level_changes = Vec::new();
        
        // Get all tiles for corruption processing
        let tiles = hex_tiles::Entity::find().all(&self.db).await?;
        
        for tile in tiles {
            let position = HexPosition::new(tile.q, tile.r);
            let current_corruption = tile.corruption_level;
            let old_dread_level = self.calculate_dread_level(current_corruption);
            
            // Calculate new corruption level
            let corruption_influence = self.calculate_corruption_influence(position, &tile).await?;
            let new_corruption = (current_corruption + corruption_influence * spread_amount).clamp(0.0, 1.0);
            
            if (new_corruption - current_corruption).abs() > 0.01 {
                // Update tile corruption in database
                hex_tiles::Entity::update_many()
                    .col_expr(hex_tiles::Column::CorruptionLevel, sea_orm::sea_query::Expr::value(new_corruption))
                    .col_expr(hex_tiles::Column::LastModified, sea_orm::sea_query::Expr::current_timestamp())
                    .filter(hex_tiles::Column::Id.eq(tile.id))
                    .exec(&self.db)
                    .await?;
                
                affected_tiles.push(CorruptionChange {
                    position,
                    old_corruption: current_corruption,
                    new_corruption,
                    change_amount: new_corruption - current_corruption,
                });
                
                // Check for dread level change
                let new_dread_level = self.calculate_dread_level(new_corruption);
                if new_dread_level != old_dread_level {
                    dread_level_changes.push(DreadLevelChange {
                        position,
                        old_dread_level,
                        new_dread_level,
                    });
                    
                    // Update dread intensity in database
                    hex_tiles::Entity::update_many()
                        .col_expr(hex_tiles::Column::DreadIntensity, sea_orm::sea_query::Expr::value(new_dread_level))
                        .filter(hex_tiles::Column::Id.eq(tile.id))
                        .exec(&self.db)
                        .await?;
                }
            }
        }
        
        info!("Corruption spread: {} tiles affected, {} dread level changes", 
              affected_tiles.len(), dread_level_changes.len());
        
        Ok(CorruptionSpreadResult {
            affected_tiles,
            dread_level_changes,
            total_spread_amount: spread_amount,
        })
    }
    
    /// Add a new corruption source (void rift, cursed artifact, etc.)
    pub async fn add_corruption_source(&mut self, position: HexPosition, intensity: f32) -> Result<()> {
        self.corruption_sources.insert(position, intensity);
        
        // Update the hex tile immediately
        hex_tiles::Entity::update_many()
            .col_expr(hex_tiles::Column::CorruptionLevel, 
                     sea_orm::sea_query::Expr::col(hex_tiles::Column::CorruptionLevel).add(intensity))
            .col_expr(hex_tiles::Column::LastModified, sea_orm::sea_query::Expr::current_timestamp())
            .filter(hex_tiles::Column::Q.eq(position.q))
            .filter(hex_tiles::Column::R.eq(position.r))
            .filter(hex_tiles::Column::S.eq(position.s))
            .exec(&self.db)
            .await?;
        
        info!("Added corruption source at ({}, {}) with intensity {}", 
              position.q, position.r, intensity);
        
        Ok(())
    }
    
    /// Attempt to purify corruption at a location
    pub async fn purify_corruption(&mut self, position: HexPosition, purification_power: f32) -> Result<PurificationResult> {
        let tile = hex_tiles::Entity::find()
            .filter(hex_tiles::Column::Q.eq(position.q))
            .filter(hex_tiles::Column::R.eq(position.r))
            .filter(hex_tiles::Column::S.eq(position.s))
            .one(&self.db)
            .await?;
        
        let tile = match tile {
            Some(t) => t,
            None => return Ok(PurificationResult::NoTileFound),
        };
        
        let current_corruption = tile.corruption_level;
        
        if current_corruption < 0.1 {
            return Ok(PurificationResult::AlreadyPure);
        }
        
        // Calculate purification effectiveness
        let corruption_resistance = self.calculate_corruption_resistance(&tile);
        let effective_purification = purification_power * (1.0 - corruption_resistance);
        
        let new_corruption = (current_corruption - effective_purification).max(0.0);
        let corruption_removed = current_corruption - new_corruption;
        
        // Update database
        hex_tiles::Entity::update_many()
            .col_expr(hex_tiles::Column::CorruptionLevel, sea_orm::sea_query::Expr::value(new_corruption))
            .col_expr(hex_tiles::Column::DreadIntensity, sea_orm::sea_query::Expr::value(self.calculate_dread_level(new_corruption)))
            .col_expr(hex_tiles::Column::LastModified, sea_orm::sea_query::Expr::current_timestamp())
            .filter(hex_tiles::Column::Q.eq(position.q))
            .filter(hex_tiles::Column::R.eq(position.r))
            .filter(hex_tiles::Column::S.eq(position.s))
            .exec(&self.db)
            .await?;
        
        info!("Purified corruption at ({}, {}): {:.2} -> {:.2} (-{:.2})", 
              position.q, position.r, current_corruption, new_corruption, corruption_removed);
        
        Ok(PurificationResult::Success {
            corruption_removed,
            new_corruption_level: new_corruption,
            effectiveness: effective_purification / purification_power,
        })
    }
    
    /// Get corruption levels in an area for visualization
    pub async fn get_corruption_map(&self, center: HexPosition, radius: u32) -> Result<HashMap<HexPosition, f32>> {
        let mut corruption_map = HashMap::new();
        
        // Get tiles in radius
        for q in (center.q - radius as i32)..=(center.q + radius as i32) {
            for r in (center.r - radius as i32)..=(center.r + radius as i32) {
                let position = HexPosition::new(q, r);
                let distance = center.distance_to(&position);
                
                if distance <= radius {
                    let tile = hex_tiles::Entity::find()
                        .filter(hex_tiles::Column::Q.eq(q))
                        .filter(hex_tiles::Column::R.eq(r))
                        .filter(hex_tiles::Column::S.eq(-q - r))
                        .one(&self.db)
                        .await?;
                    
                    if let Some(tile) = tile {
                        corruption_map.insert(position, tile.corruption_level);
                    }
                }
            }
        }
        
        Ok(corruption_map)
    }
    
    /// Get areas requiring immediate attention due to high corruption
    pub async fn get_critical_corruption_areas(&self) -> Result<Vec<CorruptionHotspot>> {
        let critical_tiles = hex_tiles::Entity::find()
            .filter(hex_tiles::Column::CorruptionLevel.gte(0.7))
            .all(&self.db)
            .await?;
        
        let mut hotspots = Vec::new();
        
        for tile in critical_tiles {
            // Check for nearby settlements or important features
            let threat_level = self.calculate_threat_level(&tile).await?;
            
            hotspots.push(CorruptionHotspot {
                position: HexPosition::new(tile.q, tile.r),
                corruption_level: tile.corruption_level,
                threat_level,
                biome: tile.biome_type,
                nearby_settlements: self.get_nearby_settlements(HexPosition::new(tile.q, tile.r)).await?,
            });
        }
        
        // Sort by threat level
        hotspots.sort_by(|a, b| b.threat_level.partial_cmp(&a.threat_level).unwrap());
        
        Ok(hotspots)
    }
    
    /// Record a horror event and its corruption impact
    pub async fn record_horror_event(&mut self, position: HexPosition, event_type: &str, impact: f32) -> Result<()> {
        // Increase corruption at event location
        let current_tile = hex_tiles::Entity::find()
            .filter(hex_tiles::Column::Q.eq(position.q))
            .filter(hex_tiles::Column::R.eq(position.r))
            .filter(hex_tiles::Column::S.eq(position.s))
            .one(&self.db)
            .await?;
        
        if let Some(tile) = current_tile {
            let new_corruption = (tile.corruption_level + impact).min(1.0);
            let new_dread = self.calculate_dread_level(new_corruption);
            
            hex_tiles::Entity::update_many()
                .col_expr(hex_tiles::Column::CorruptionLevel, sea_orm::sea_query::Expr::value(new_corruption))
                .col_expr(hex_tiles::Column::DreadIntensity, sea_orm::sea_query::Expr::value(new_dread))
                .col_expr(hex_tiles::Column::HorrorEventsCount, 
                         sea_orm::sea_query::Expr::col(hex_tiles::Column::HorrorEventsCount).add(1))
                .col_expr(hex_tiles::Column::LastModified, sea_orm::sea_query::Expr::current_timestamp())
                .filter(hex_tiles::Column::Id.eq(tile.id))
                .exec(&self.db)
                .await?;
            
            info!("Horror event '{}' at ({}, {}) increased corruption by {:.2}", 
                  event_type, position.q, position.r, impact);
        }
        
        Ok(())
    }
    
    /// Private helper methods
    
    async fn load_corruption_sources(&mut self) -> Result<()> {
        // Load existing high-corruption areas as sources
        let corruption_sources = hex_tiles::Entity::find()
            .filter(hex_tiles::Column::CorruptionLevel.gte(0.8))
            .all(&self.db)
            .await?;
        
        for tile in corruption_sources {
            let position = HexPosition::new(tile.q, tile.r);
            self.corruption_sources.insert(position, tile.corruption_level);
        }
        
        Ok(())
    }
    
    async fn calculate_corruption_influence(&self, position: HexPosition, tile: &hex_tiles::Model) -> Result<f32> {
        let mut total_influence = 0.0;
        
        // Influence from nearby corruption sources
        for (source_pos, intensity) in &self.corruption_sources {
            let distance = position.distance_to(source_pos) as f32;
            if distance <= 5.0 { // Corruption spreads within 5 hexes
                let influence = intensity * (1.0 - (distance / 5.0));
                total_influence += influence;
            }
        }
        
        // Biome resistance to corruption
        let biome_resistance = match tile.biome_type.as_str() {
            "temple" => -0.3, // Temples resist corruption
            "plains" => 0.0,
            "forest" => 0.1,
            "swamp" => 0.3, // Swamps amplify corruption
            "mountain" => -0.1,
            _ => 0.0,
        };
        
        total_influence += biome_resistance;
        
        // Horror events increase corruption vulnerability
        let horror_event_modifier = (tile.horror_events_count as f32 * 0.05).min(0.5);
        total_influence += horror_event_modifier;
        
        Ok(total_influence)
    }
    
    fn calculate_dread_level(&self, corruption: f32) -> i32 {
        for (level, threshold) in self.dread_thresholds.iter().enumerate().rev() {
            if corruption >= *threshold {
                return level as i32;
            }
        }
        0
    }
    
    fn calculate_corruption_resistance(&self, tile: &hex_tiles::Model) -> f32 {
        // Some biomes resist purification
        match tile.biome_type.as_str() {
            "swamp" => 0.4, // Hard to purify swamps
            "mountain" => 0.1, // Mountains are easier to purify
            "temple" => 0.0, // Temples purify easily
            _ => 0.2,
        }
    }
    
    async fn calculate_threat_level(&self, tile: &hex_tiles::Model) -> Result<f32> {
        let base_threat = tile.corruption_level;
        
        // Increase threat if near settlements
        let nearby_settlements = self.get_nearby_settlements(HexPosition::new(tile.q, tile.r)).await?;
        let settlement_threat_modifier = nearby_settlements.len() as f32 * 0.2;
        
        // Increase threat if on travel routes (has trails)
        let mut route_threat_modifier = 0.0;
        if let Some(features) = &tile.features {
            if let Some(feature_obj) = features.as_object() {
                if let Some(trails) = feature_obj.get("trails") {
                    if let Some(trail_array) = trails.as_array() {
                        if !trail_array.is_empty() {
                            route_threat_modifier = 0.3;
                        }
                    }
                }
            }
        }
        
        Ok(base_threat + settlement_threat_modifier + route_threat_modifier)
    }
    
    async fn get_nearby_settlements(&self, position: HexPosition) -> Result<Vec<String>> {
        let settlements = settlements::Entity::find()
            .all(&self.db)
            .await?;
        
        let mut nearby = Vec::new();
        
        for settlement in settlements {
            if let (Some(hbf_x), Some(hbf_y)) = (settlement.hbf_x, settlement.hbf_y) {
                let settlement_pos = HexPosition::from_hbf_coords(hbf_x, hbf_y);
                let distance = position.distance_to(&settlement_pos);
                
                if distance <= 10 { // Within 10 hexes
                    nearby.push(settlement.name);
                }
            }
        }
        
        Ok(nearby)
    }
}

/// Corruption visualization and effects
impl CorruptionEngine {
    /// Get visual corruption effects for rendering
    pub async fn get_corruption_visual_effects(&self, position: HexPosition) -> Result<CorruptionVisualEffects> {
        let tile = hex_tiles::Entity::find()
            .filter(hex_tiles::Column::Q.eq(position.q))
            .filter(hex_tiles::Column::R.eq(position.r))
            .filter(hex_tiles::Column::S.eq(position.s))
            .one(&self.db)
            .await?;
        
        let tile = match tile {
            Some(t) => t,
            None => return Ok(CorruptionVisualEffects::default()),
        };
        
        let dread_level = self.calculate_dread_level(tile.corruption_level);
        
        Ok(CorruptionVisualEffects {
            corruption_level: tile.corruption_level,
            dread_level,
            visual_overlay: self.get_corruption_overlay(dread_level),
            particle_effects: self.get_corruption_particles(tile.corruption_level),
            color_tint: self.get_corruption_color_tint(tile.corruption_level),
            environmental_changes: self.get_environmental_changes(dread_level),
        })
    }
    
    /// Get corruption effects on NPCs in the area
    pub async fn apply_corruption_to_npcs(&self, position: HexPosition, radius: u32) -> Result<Vec<NpcCorruptionEffect>> {
        let mut effects = Vec::new();
        
        // Get NPCs in the area
        let npcs = npcs::Entity::find()
            .all(&self.db)
            .await?;
        
        for npc in npcs {
            if let (Some(npc_x), Some(npc_y)) = (npc.hbf_x, npc.hbf_y) {
                let npc_pos = HexPosition::from_hbf_coords(npc_x, npc_y);
                let distance = position.distance_to(&npc_pos);
                
                if distance <= radius {
                    // Get corruption level at NPC location
                    let corruption_level = self.get_corruption_at_position(npc_pos).await?;
                    
                    // Calculate corruption effect on NPC
                    let corruption_increase = corruption_level * npc.corruption_susceptibility * 0.1;
                    let new_corruption = (npc.current_corruption_level + corruption_increase).min(1.0);
                    
                    if corruption_increase > 0.01 {
                        // Update NPC corruption
                        npcs::Entity::update_many()
                            .col_expr(npcs::Column::CurrentCorruptionLevel, sea_orm::sea_query::Expr::value(new_corruption))
                            .filter(npcs::Column::Id.eq(npc.id))
                            .exec(&self.db)
                            .await?;
                        
                        effects.push(NpcCorruptionEffect {
                            npc_id: npc.id,
                            npc_name: npc.name,
                            corruption_increase,
                            new_corruption_level: new_corruption,
                            behavioral_changes: self.get_corruption_behavioral_changes(new_corruption),
                        });
                    }
                }
            }
        }
        
        Ok(effects)
    }
    
    async fn get_corruption_at_position(&self, position: HexPosition) -> Result<f32> {
        let tile = hex_tiles::Entity::find()
            .filter(hex_tiles::Column::Q.eq(position.q))
            .filter(hex_tiles::Column::R.eq(position.r))
            .filter(hex_tiles::Column::S.eq(position.s))
            .one(&self.db)
            .await?;
        
        Ok(tile.map(|t| t.corruption_level).unwrap_or(0.0))
    }
    
    fn get_corruption_overlay(&self, dread_level: i32) -> String {
        match dread_level {
            0 => "none".to_string(),
            1 => "faint_shadows".to_string(),
            2 => "dark_veins".to_string(),
            3 => "writhing_darkness".to_string(),
            4 => "void_tendrils".to_string(),
            _ => "none".to_string(),
        }
    }
    
    fn get_corruption_particles(&self, corruption_level: f32) -> Vec<String> {
        let mut particles = Vec::new();
        
        if corruption_level > 0.2 {
            particles.push("dark_motes".to_string());
        }
        if corruption_level > 0.5 {
            particles.push("whispers_visual".to_string());
        }
        if corruption_level > 0.8 {
            particles.push("void_cracks".to_string());
        }
        
        particles
    }
    
    fn get_corruption_color_tint(&self, corruption_level: f32) -> (f32, f32, f32, f32) {
        // RGBA color tint based on corruption level
        let red_tint = corruption_level * 0.3;
        let green_reduction = corruption_level * 0.5;
        let blue_reduction = corruption_level * 0.4;
        
        (1.0 + red_tint, 1.0 - green_reduction, 1.0 - blue_reduction, 1.0)
    }
    
    fn get_environmental_changes(&self, dread_level: i32) -> Vec<String> {
        match dread_level {
            0 => Vec::new(),
            1 => vec!["wilted_plants".to_string()],
            2 => vec!["twisted_trees".to_string(), "dead_grass".to_string()],
            3 => vec!["cracked_earth".to_string(), "poisoned_water".to_string()],
            4 => vec!["void_rifts".to_string(), "reality_distortion".to_string()],
            _ => Vec::new(),
        }
    }
    
    fn get_corruption_behavioral_changes(&self, corruption_level: f32) -> Vec<String> {
        let mut changes = Vec::new();
        
        if corruption_level > 0.3 {
            changes.push("paranoid".to_string());
        }
        if corruption_level > 0.5 {
            changes.push("aggressive".to_string());
        }
        if corruption_level > 0.7 {
            changes.push("irrational".to_string());
        }
        if corruption_level > 0.9 {
            changes.push("hostile".to_string());
        }
        
        changes
    }
}

#[derive(Debug, Clone)]
pub struct CorruptionSpreadResult {
    pub affected_tiles: Vec<CorruptionChange>,
    pub dread_level_changes: Vec<DreadLevelChange>,
    pub total_spread_amount: f32,
}

#[derive(Debug, Clone)]
pub struct CorruptionChange {
    pub position: HexPosition,
    pub old_corruption: f32,
    pub new_corruption: f32,
    pub change_amount: f32,
}

#[derive(Debug, Clone)]
pub struct DreadLevelChange {
    pub position: HexPosition,
    pub old_dread_level: i32,
    pub new_dread_level: i32,
}

#[derive(Debug, Clone)]
pub struct CorruptionHotspot {
    pub position: HexPosition,
    pub corruption_level: f32,
    pub threat_level: f32,
    pub biome: String,
    pub nearby_settlements: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum PurificationResult {
    Success {
        corruption_removed: f32,
        new_corruption_level: f32,
        effectiveness: f32,
    },
    AlreadyPure,
    NoTileFound,
}

#[derive(Debug, Clone, Default)]
pub struct CorruptionVisualEffects {
    pub corruption_level: f32,
    pub dread_level: i32,
    pub visual_overlay: String,
    pub particle_effects: Vec<String>,
    pub color_tint: (f32, f32, f32, f32), // RGBA
    pub environmental_changes: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct NpcCorruptionEffect {
    pub npc_id: Uuid,
    pub npc_name: String,
    pub corruption_increase: f32,
    pub new_corruption_level: f32,
    pub behavioral_changes: Vec<String>,
}
