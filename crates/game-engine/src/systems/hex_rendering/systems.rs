//! Hex Rendering Systems - ECS systems for database-driven hex visualization

use bevy::prelude::*;
use super::components::*;
use super::resources::*;
use super::biome_rules::{load_biome_rules, BiomeRules};
use crate::bevy_integration::DatabaseQuery;

/// System to load hex tiles from database into ECS
pub fn load_hex_tiles_system(
    mut commands: Commands,
    mut db_query: DatabaseQuery,
    viewport: Res<HexViewport>,
) {
    // Query database for tiles in current viewport
    // This system runs when viewport changes or at startup
    
    info!("Loading hex tiles for viewport: center=({}, {}), radius={}", 
          viewport.center.q, viewport.center.r, viewport.radius);
    
    // In full implementation, this would:
    // 1. Query database for tiles in viewport radius
    // 2. Spawn ECS entities for each tile with all components
    // 3. Set up bevy_ecs_tilemap integration
    // 4. Apply corruption visuals and effects
}

#[derive(Resource, Default, Debug, Clone)]
pub struct ActiveBiomeRules(pub Option<BiomeRules>);

pub fn apply_biome_rules_system(mut commands: Commands) {
    let rules = load_biome_rules();
    commands.insert_resource(ActiveBiomeRules(Some(rules)));
    info!("Applied biome rules for hex rendering/path costs");
}

/// System to update tile visuals based on corruption changes
pub fn update_corruption_visuals_system(
    mut query: Query<(&mut CorruptionVisuals, &mut TileVisuals), Changed<CorruptionVisuals>>,
) {
    for (corruption, mut visuals) in query.iter_mut() {
        // Update visual components when corruption changes
        visuals.base_color = corruption.color_tint;
        
        // Add corruption overlay textures
        if !corruption.corruption_overlay.is_empty() && corruption.corruption_overlay != "none" {
            if !visuals.overlay_textures.contains(&corruption.corruption_overlay) {
                visuals.overlay_textures.push(corruption.corruption_overlay.clone());
            }
        }
        
        // Update animation state based on corruption
        visuals.animation_state = if corruption.corruption_level > 0.5 {
            TileAnimationState::Pulsing
        } else if corruption.corruption_level > 0.8 {
            TileAnimationState::Flickering
        } else {
            TileAnimationState::Static
        };
    }
}

/// System to handle tile discovery and exploration
pub fn tile_discovery_system(
    mut query: Query<(&mut DiscoveryState, &HexTile)>,
    player_position: Res<PlayerPosition>,
    time: Res<Time>,
) {
    for (mut discovery, tile) in query.iter_mut() {
        let tile_pos = HexPosition::new(tile.q, tile.r);
        let distance = player_position.current.distance_to(&tile_pos);
        
        // Auto-discover tiles within sight range
        if distance <= player_position.sight_range && !discovery.discovered {
            discovery.discovered = true;
            discovery.discovery_time = Some(time.elapsed_seconds_f64());
            
            info!("Discovered tile at ({}, {}) - {}", tile.q, tile.r, tile.biome_type);
        }
        
        // Update exploration progress when player is on tile
        if distance == 0 && discovery.discovered && !discovery.fully_explored {
            discovery.exploration_progress += time.delta_seconds() * 0.1; // 10 seconds to fully explore
            
            if discovery.exploration_progress >= 1.0 {
                discovery.fully_explored = true;
                info!("Fully explored tile at ({}, {})", tile.q, tile.r);
            }
        }
    }
}

/// System to update viewport and manage tile loading/unloading
pub fn viewport_management_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut ViewportTile, &HexTile)>,
    viewport: Res<HexViewport>,
    mut db_query: DatabaseQuery,
) {
    // Update viewport tiles
    for (entity, mut viewport_tile, hex_tile) in query.iter_mut() {
        let tile_pos = HexPosition::new(hex_tile.q, hex_tile.r);
        let distance = viewport.center.distance_to(&tile_pos);
        
        let should_be_in_viewport = distance <= viewport.radius;
        
        if should_be_in_viewport != viewport_tile.in_viewport {
            viewport_tile.in_viewport = should_be_in_viewport;
            viewport_tile.distance_from_center = distance;
            
            if should_be_in_viewport {
                // Tile entered viewport - ensure it has all components
                commands.entity(entity).insert(TileSelection::default());
            } else {
                // Tile left viewport - could remove non-essential components for performance
                commands.entity(entity).remove::<TileSelection>();
            }
        }
    }
    
    // Load new tiles if viewport moved significantly
    if viewport.is_changed() {
        info!("Viewport changed, loading new tiles");
        // Would trigger database queries for new tiles
    }
}

/// System to handle tile selection and interaction
pub fn tile_interaction_system(
    mut query: Query<(&mut TileSelection, &TileInteraction, &HexTile)>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    cursor_position: Res<CursorWorldPosition>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        // Convert mouse position to hex coordinates
        if let Some(hex_pos) = cursor_position.to_hex() {
            // Find clicked tile and handle selection
            for (mut selection, interaction, tile) in query.iter_mut() {
                let tile_pos = HexPosition::new(tile.q, tile.r);
                
                if tile_pos.q == hex_pos.q && tile_pos.r == hex_pos.r {
                    selection.is_selected = !selection.is_selected;
                    
                    if selection.is_selected {
                        info!("Selected tile at ({}, {}) - {}", tile.q, tile.r, tile.biome_type);
                        // Could trigger interaction menu or movement command
                    }
                } else {
                    selection.is_selected = false; // Deselect other tiles
                }
            }
        }
    }
}

/// System to update weather effects on tiles
pub fn weather_effects_system(
    mut query: Query<(&mut TileWeather, &mut TileVisuals, &HexTile)>,
    weather_state: Res<GlobalWeatherState>,
) {
    for (mut weather, mut visuals, tile) in query.iter_mut() {
        // Update weather condition based on global weather state
        if let Some(new_condition) = weather_state.get_weather_for_tile(&HexPosition::new(tile.q, tile.r)) {
            if weather.current_condition != new_condition.condition {
                weather.current_condition = new_condition.condition.clone();
                weather.visibility_modifier = new_condition.visibility_modifier;
                weather.movement_modifier = new_condition.movement_modifier;
                
                // Update visual overlay for weather
                weather.weather_overlay = match new_condition.condition.as_str() {
                    "Rainy" => Some("rain_overlay".to_string()),
                    "Foggy" => Some("fog_overlay".to_string()),
                    "Snowy" => Some("snow_overlay".to_string()),
                    _ => None,
                };
                
                // Add weather overlay to visuals
                if let Some(overlay) = &weather.weather_overlay {
                    if !visuals.overlay_textures.contains(overlay) {
                        visuals.overlay_textures.push(overlay.clone());
                    }
                }
            }
        }
    }
}

/// System to spawn particle effects for corrupted tiles
pub fn corruption_particle_system(
    mut commands: Commands,
    query: Query<(Entity, &CorruptionVisuals, &Transform), Changed<CorruptionVisuals>>,
) {
    for (entity, corruption, transform) in query.iter() {
        // Spawn particle effects based on corruption level
        for effect_name in &corruption.particle_effects {
            match effect_name.as_str() {
                "dark_motes" => {
                    // Spawn dark particle effect
                    commands.entity(entity).with_children(|parent| {
                        parent.spawn((
                            ParticleEffect::new(effect_name.clone()),
                            Transform::from_translation(transform.translation),
                        ));
                    });
                }
                "void_cracks" => {
                    // Spawn void crack visual effect
                    commands.entity(entity).with_children(|parent| {
                        parent.spawn((
                            ParticleEffect::new(effect_name.clone()),
                            Transform::from_translation(transform.translation),
                        ));
                    });
                }
                _ => {}
            }
        }
    }
}

// Helper resources and components

#[derive(Resource, Debug, Clone)]
pub struct HexViewport {
    pub center: HexPosition,
    pub radius: u32,
}

#[derive(Resource, Debug, Clone)]
pub struct PlayerPosition {
    pub current: HexPosition,
    pub sight_range: u32,
}

#[derive(Resource, Debug, Clone)]
pub struct GlobalWeatherState {
    pub season: String,
    pub time_of_day: String,
}

impl GlobalWeatherState {
    pub fn get_weather_for_tile(&self, _position: &HexPosition) -> Option<WeatherCondition> {
        // Would query weather system for tile-specific weather
        Some(WeatherCondition {
            condition: "Clear".to_string(),
            visibility_modifier: 1.0,
            movement_modifier: 1.0,
            combat_effects: Vec::new(),
        })
    }
}

#[derive(Resource, Debug, Clone)]
pub struct CursorWorldPosition {
    pub world_pos: Option<Vec3>,
}

impl CursorWorldPosition {
    pub fn to_hex(&self) -> Option<HexPosition> {
        // Convert world position to hex coordinates
        if let Some(pos) = self.world_pos {
            // Would use proper hex conversion math
            Some(HexPosition::new(pos.x as i32, pos.z as i32))
        } else {
            None
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct ParticleEffect {
    pub effect_type: String,
}

impl ParticleEffect {
    pub fn new(effect_type: String) -> Self {
        Self { effect_type }
    }
}

use crate::systems::{HexPosition, WeatherCondition};
