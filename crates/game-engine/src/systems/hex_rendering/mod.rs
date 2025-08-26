//! Hex Rendering System - Full ECS implementation with bevy_ecs_tilemap integration
//!
//! Database-driven hex tile visualization powered by HBF data,
//! integrated as Bevy ECS components, systems, and resources.

use bevy::prelude::*;

pub mod components;
pub mod systems;
pub mod resources;
pub mod biome_rules;

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
                apply_biome_rules_system,
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
// Removed legacy DB-coupled rendering system; ECS plugins handle rendering.

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
}
