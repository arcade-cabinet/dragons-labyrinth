//! Hex Rendering System - Full ECS implementation with bevy_ecs_tilemap integration
//!
//! Database-driven hex tile visualization powered by HBF data,
//! integrated as Bevy ECS components, systems, and resources.

use bevy::prelude::*;
use sea_orm::DatabaseConnection;

pub mod components;
pub mod systems;
pub mod resources;

pub use components::*;
pub use systems::*;
pub use resources::*;

/// Hex rendering system plugin for Bevy ECS integration
pub struct HexRenderingPlugin;

impl Plugin for HexRenderingPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add resources
            .init_resource::<HexViewport>()
            .init_resource::<PlayerPosition>()
            .init_resource::<GlobalWeatherState>()
            .init_resource::<CursorWorldPosition>()
            .init_resource::<HexRenderingSettings>()
            .init_resource::<TileTextureAtlas>()
            
            // Add systems
            .add_systems(Startup, (
                setup_hex_tilemap_system,
                load_tile_textures_system,
            ))
            .add_systems(Update, (
                // Core rendering
                load_hex_tiles_system,
                viewport_management_system,
                
                // Visual updates
                update_corruption_visuals_system,
                weather_effects_system,
                corruption_particle_system,
                
                // Player interaction
                tile_discovery_system,
                tile_interaction_system,
                
                // Performance optimization
                tile_culling_system,
                animation_update_system,
            ))
            
            // Register component reflection for debugging
            .register_type::<HexTile>()
            .register_type::<TileVisuals>()
            .register_type::<CorruptionVisuals>()
            .register_type::<TileFeatures>()
            .register_type::<DiscoveryState>()
            .register_type::<TileWeather>()
            .register_type::<TileInteraction>()
            .register_type::<ViewportTile>()
            .register_type::<TileSelection>()
            .register_type::<TileAudio>()
            .register_type::<PathfindingData>();
    }
}

/// Legacy hex rendering system for compatibility
pub struct HexRenderingSystem {
    db: DatabaseConnection,
    biome_texture_map: HashMap<String, String>,
    corruption_overlays: HashMap<i32, String>, // dread_level -> overlay_texture
}

impl HexRenderingSystem {
    pub async fn new(db: &DatabaseConnection) -> Result<Self> {
        let biome_texture_map = Self::create_biome_texture_map();
        let corruption_overlays = Self::create_corruption_overlay_map();
        
        info!("Hex rendering system initialized with {} biome textures and {} corruption overlays",
              biome_texture_map.len(), corruption_overlays.len());
        
        Ok(Self {
            db: db.clone(),
            biome_texture_map,
            corruption_overlays,
        })
    }
    
    /// Get tiles within viewport for rendering
    pub async fn get_tiles_in_viewport(&self, viewport: Viewport) -> Result<Vec<TileRenderData>> {
        debug!("Querying tiles in viewport: center=({}, {}), radius={}", 
               viewport.center.q, viewport.center.r, viewport.radius);
        
        // Calculate hex positions within radius
        let hex_positions = self.calculate_hex_positions_in_radius(viewport.center, viewport.radius);
        
        // Query database for tiles at these positions
        let tiles = hex_tiles::Entity::find()
            .filter(hex_tiles::Column::Q.is_in(hex_positions.iter().map(|p| p.q).collect::<Vec<_>>()))
            .filter(hex_tiles::Column::R.is_in(hex_positions.iter().map(|p| p.r).collect::<Vec<_>>()))
            .all(&self.db)
            .await?;
        
        // Convert to render data
        let mut render_data = Vec::new();
        for tile in tiles {
            let render_tile = self.convert_tile_to_render_data(&tile)?;
            render_data.push(render_tile);
        }
        
        debug!("Found {} tiles for rendering", render_data.len());
        Ok(render_data)
    }
    
    /// Get a single tile for detailed inspection
    pub async fn get_tile_at_position(&self, position: HexPosition) -> Result<Option<TileRenderData>> {
        let tile = hex_tiles::Entity::find()
            .filter(hex_tiles::Column::Q.eq(position.q))
            .filter(hex_tiles::Column::R.eq(position.r))
            .filter(hex_tiles::Column::S.eq(position.s))
            .one(&self.db)
            .await?;
        
        if let Some(tile) = tile {
            Ok(Some(self.convert_tile_to_render_data(&tile)?))
        } else {
            Ok(None)
        }
    }
    
    /// Update tile corruption and re-calculate render data
    pub async fn update_tile_corruption(&self, position: HexPosition, new_corruption: f32) -> Result<TileRenderData> {
        // Update in database
        hex_tiles::Entity::update_many()
            .col_expr(hex_tiles::Column::CorruptionLevel, sea_orm::sea_query::Expr::value(new_corruption))
            .col_expr(hex_tiles::Column::LastModified, sea_orm::sea_query::Expr::current_timestamp())
            .filter(hex_tiles::Column::Q.eq(position.q))
            .filter(hex_tiles::Column::R.eq(position.r))
            .filter(hex_tiles::Column::S.eq(position.s))
            .exec(&self.db)
            .await?;
        
        // Return updated render data
        self.get_tile_at_position(position).await?.ok_or_else(|| anyhow::anyhow!("Tile not found after update"))
    }
    
    /// Get tiles by biome type for biome-specific operations
    pub async fn get_tiles_by_biome(&self, biome_type: &str) -> Result<Vec<TileRenderData>> {
        let tiles = hex_tiles::Entity::find()
            .filter(hex_tiles::Column::BiomeType.eq(biome_type))
            .all(&self.db)
            .await?;
        
        tiles.into_iter()
            .map(|tile| self.convert_tile_to_render_data(&tile))
            .collect()
    }
    
    /// Get tiles with high corruption for corruption spread mechanics
    pub async fn get_corrupted_tiles(&self, min_corruption: f32) -> Result<Vec<TileRenderData>> {
        let tiles = hex_tiles::Entity::find()
            .filter(hex_tiles::Column::CorruptionLevel.gte(min_corruption))
            .all(&self.db)
            .await?;
        
        tiles.into_iter()
            .map(|tile| self.convert_tile_to_render_data(&tile))
            .collect()
    }
    
    /// Convert database tile to render data
    fn convert_tile_to_render_data(&self, tile: &hex_tiles::Model) -> Result<TileRenderData> {
        let position = HexPosition::new(tile.q, tile.r);
        
        // Get base texture from biome
        let texture_id = self.biome_texture_map
            .get(&tile.biome_type)
            .unwrap_or(&"unknown".to_string())
            .clone();
        
        // Determine overlay effects
        let mut overlay_effects = Vec::new();
        
        // Corruption overlay
        if tile.corruption_level > 0.1 {
            if let Some(overlay) = self.corruption_overlays.get(&tile.dread_intensity) {
                overlay_effects.push(overlay.clone());
            }
        }
        
        // Add feature-based effects
        if let Some(features) = &tile.features {
            if let Some(feature_obj) = features.as_object() {
                if let Some(rivers) = feature_obj.get("rivers") {
                    if let Some(river_array) = rivers.as_array() {
                        if !river_array.is_empty() {
                            overlay_effects.push("river_overlay".to_string());
                        }
                    }
                }
                if let Some(trails) = feature_obj.get("trails") {
                    if let Some(trail_array) = trails.as_array() {
                        if !trail_array.is_empty() {
                            overlay_effects.push("trail_overlay".to_string());
                        }
                    }
                }
            }
        }
        
        // Extract feature names
        let mut feature_names = Vec::new();
        if let Some(features) = &tile.features {
            if let Some(feature_obj) = features.as_object() {
                if let Some(hbf_feature) = feature_obj.get("hbf_feature") {
                    if let Some(feature_str) = hbf_feature.as_str() {
                        if feature_str != "Other" {
                            feature_names.push(feature_str.to_string());
                        }
                    }
                }
            }
        }
        
        Ok(TileRenderData {
            position,
            biome_type: tile.biome_type.clone(),
            corruption_level: tile.corruption_level,
            dread_intensity: tile.dread_intensity,
            texture_id,
            overlay_effects,
            features: feature_names,
        })
    }
    
    /// Calculate hex positions within radius using cube coordinates
    fn calculate_hex_positions_in_radius(&self, center: HexPosition, radius: u32) -> Vec<HexPosition> {
        let mut positions = Vec::new();
        let radius = radius as i32;
        
        for q in -radius..=radius {
            let r1 = (-radius).max(-q - radius);
            let r2 = radius.min(-q + radius);
            
            for r in r1..=r2 {
                let s = -q - r;
                positions.push(HexPosition {
                    q: center.q + q,
                    r: center.r + r,
                    s: center.s + s,
                });
            }
        }
        
        positions
    }
    
    /// Create biome texture mapping
    fn create_biome_texture_map() -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("jungle".to_string(), "jungle_tile".to_string());
        map.insert("forest".to_string(), "forest_tile".to_string());
        map.insert("mountain".to_string(), "mountain_tile".to_string());
        map.insert("plains".to_string(), "plains_tile".to_string());
        map.insert("swamp".to_string(), "swamp_tile".to_string());
        map.insert("desert".to_string(), "desert_tile".to_string());
        map.insert("tundra".to_string(), "tundra_tile".to_string());
        map.insert("coast".to_string(), "coast_tile".to_string());
        map.insert("ocean".to_string(), "ocean_tile".to_string());
        map.insert("unknown".to_string(), "default_tile".to_string());
        map
    }
    
    /// Create corruption overlay mapping
    fn create_corruption_overlay_map() -> HashMap<i32, String> {
        let mut map = HashMap::new();
        map.insert(0, "clean_overlay".to_string());
        map.insert(1, "slight_corruption_overlay".to_string());
        map.insert(2, "moderate_corruption_overlay".to_string());
        map.insert(3, "heavy_corruption_overlay".to_string());
        map.insert(4, "extreme_corruption_overlay".to_string());
        map
    }
}

/// Extension methods for HBF coordinate conversion
impl HexPosition {
    /// Convert from HBF coordinates stored in database
    pub fn from_hbf_tile(tile: &hex_tiles::Model) -> Option<Self> {
        if let (Some(hbf_x), Some(hbf_y)) = (tile.hbf_x, tile.hbf_y) {
            Some(Self::from_hbf_coords(hbf_x, hbf_y))
        } else {
            Some(Self::new(tile.q, tile.r))
        }
    }
    
    /// Calculate distance between two hex positions
    pub fn distance_to(&self, other: &HexPosition) -> u32 {
        ((self.q - other.q).abs() + (self.r - other.r).abs() + (self.s - other.s).abs() / 2) as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_hex_distance() {
        let pos1 = HexPosition::new(0, 0);
        let pos2 = HexPosition::new(1, 1);
        assert_eq!(pos1.distance_to(&pos2), 2);
    }
    
    #[test]
    fn test_radius_calculation() {
        let system = HexRenderingSystem {
            db: todo!(), // Would need mock DB for full test
            biome_texture_map: HashMap::new(),
            corruption_overlays: HashMap::new(),
        };
        
        let center = HexPosition::new(0, 0);
        let positions = system.calculate_hex_positions_in_radius(center, 1);
        assert_eq!(positions.len(), 7); // Center + 6 neighbors
    }
}
