//! Hex Rendering Resources - ECS resources for hex visualization

use bevy::prelude::*;
use std::collections::HashMap;
use crate::systems::HexPosition;

/// Hex rendering settings and configuration
#[derive(Resource, Debug, Clone)]
pub struct HexRenderingSettings {
    pub tile_size: f32,
    pub corruption_intensity: f32,
    pub animation_speed: f32,
    pub fog_of_war: bool,
    pub show_grid: bool,
    pub show_coordinates: bool,
    pub performance_mode: bool,
}

impl Default for HexRenderingSettings {
    fn default() -> Self {
        Self {
            tile_size: 1.0,
            corruption_intensity: 1.0,
            animation_speed: 1.0,
            fog_of_war: true,
            show_grid: false,
            show_coordinates: false,
            performance_mode: false,
        }
    }
}

/// Texture atlas for tile rendering
#[derive(Resource, Debug, Clone)]
pub struct TileTextureAtlas {
    pub biome_textures: HashMap<String, Handle<Image>>,
    pub overlay_textures: HashMap<String, Handle<Image>>,
    pub feature_textures: HashMap<String, Handle<Image>>,
    pub corruption_textures: HashMap<String, Handle<Image>>,
}

impl Default for TileTextureAtlas {
    fn default() -> Self {
        Self {
            biome_textures: HashMap::new(),
            overlay_textures: HashMap::new(),
            feature_textures: HashMap::new(),
            corruption_textures: HashMap::new(),
        }
    }
}

/// Camera viewport tracking
#[derive(Resource, Debug, Clone)]
pub struct HexViewport {
    pub center: HexPosition,
    pub radius: u32,
    pub zoom_level: f32,
}

impl Default for HexViewport {
    fn default() -> Self {
        Self {
            center: HexPosition::new(0, 0),
            radius: 10,
            zoom_level: 1.0,
        }
    }
}

impl HexViewport {
    pub fn move_to(&mut self, new_center: HexPosition) {
        self.center = new_center;
    }
    
    pub fn zoom(&mut self, zoom_delta: f32) {
        self.zoom_level = (self.zoom_level + zoom_delta).clamp(0.5, 3.0);
        
        // Adjust radius based on zoom
        self.radius = match self.zoom_level {
            x if x < 0.8 => 5,   // Zoomed in
            x if x > 2.0 => 20,  // Zoomed out
            _ => 10,             // Normal
        };
    }
}

/// Player position tracking
#[derive(Resource, Debug, Clone)]
pub struct PlayerPosition {
    pub current: HexPosition,
    pub previous: HexPosition,
    pub sight_range: u32,
    pub movement_points: f32,
}

impl Default for PlayerPosition {
    fn default() -> Self {
        Self {
            current: HexPosition::new(0, 0),
            previous: HexPosition::new(0, 0),
            sight_range: 3,
            movement_points: 30.0,
        }
    }
}

impl PlayerPosition {
    pub fn move_to(&mut self, new_position: HexPosition) {
        self.previous = self.current.clone();
        self.current = new_position;
    }
    
    pub fn can_see(&self, position: &HexPosition) -> bool {
        self.current.distance_to(position) <= self.sight_range
    }
}

/// Global weather state for rendering
#[derive(Resource, Debug, Clone)]
pub struct GlobalWeatherState {
    pub season: String,
    pub time_of_day: String,
    pub global_weather: String,
    pub weather_intensity: f32,
}

impl Default for GlobalWeatherState {
    fn default() -> Self {
        Self {
            season: "warm".to_string(),
            time_of_day: "day".to_string(),
            global_weather: "Clear".to_string(),
            weather_intensity: 1.0,
        }
    }
}

/// Mouse/cursor world position
#[derive(Resource, Debug, Clone, Default)]
pub struct CursorWorldPosition {
    pub world_pos: Option<Vec3>,
    pub hex_pos: Option<HexPosition>,
}

impl CursorWorldPosition {
    pub fn update(&mut self, world_pos: Option<Vec3>) {
        self.world_pos = world_pos;
        self.hex_pos = self.to_hex();
    }
    
    pub fn to_hex(&self) -> Option<HexPosition> {
        // Convert world position to hex coordinates
        if let Some(pos) = self.world_pos {
            // Proper hex conversion math would go here
            // For now, simple approximation
            Some(HexPosition::new(pos.x as i32, pos.z as i32))
        } else {
            None
        }
    }
}

/// Rendering performance tracking
#[derive(Resource, Debug, Clone, Default)]
pub struct RenderingStats {
    pub tiles_rendered: u32,
    pub tiles_culled: u32,
    pub corruption_effects_active: u32,
    pub weather_effects_active: u32,
    pub discovery_updates: u32,
}

impl RenderingStats {
    pub fn reset_frame_stats(&mut self) {
        self.tiles_rendered = 0;
        self.tiles_culled = 0;
        self.corruption_effects_active = 0;
        self.weather_effects_active = 0;
    }
    
    pub fn record_tile_rendered(&mut self) {
        self.tiles_rendered += 1;
    }
    
    pub fn record_tile_culled(&mut self) {
        self.tiles_culled += 1;
    }
    
    pub fn rendering_efficiency(&self) -> f32 {
        if self.tiles_rendered + self.tiles_culled == 0 {
            1.0
        } else {
            self.tiles_rendered as f32 / (self.tiles_rendered + self.tiles_culled) as f32
        }
    }
}

/// Animation controller for tile effects
#[derive(Resource, Debug, Clone)]
pub struct TileAnimationController {
    pub corruption_pulse_speed: f32,
    pub river_flow_speed: f32,
    pub void_crack_flicker_speed: f32,
    pub global_animation_scale: f32,
}

impl Default for TileAnimationController {
    fn default() -> Self {
        Self {
            corruption_pulse_speed: 2.0,
            river_flow_speed: 1.0,
            void_crack_flicker_speed: 5.0,
            global_animation_scale: 1.0,
        }
    }
}

/// Tilemap integration state
#[derive(Resource, Debug, Clone)]
pub struct TilemapState {
    pub tilemap_entity: Option<Entity>,
    pub layer_entities: HashMap<String, Entity>, // layer_name -> entity
    pub tile_entities: HashMap<HexPosition, Entity>, // position -> tile entity
    pub needs_rebuild: bool,
}

impl Default for TilemapState {
    fn default() -> Self {
        Self {
            tilemap_entity: None,
            layer_entities: HashMap::new(),
            tile_entities: HashMap::new(),
            needs_rebuild: false,
        }
    }
}

impl TilemapState {
    pub fn mark_for_rebuild(&mut self) {
        self.needs_rebuild = true;
    }
    
    pub fn add_tile_entity(&mut self, position: HexPosition, entity: Entity) {
        self.tile_entities.insert(position, entity);
    }
    
    pub fn remove_tile_entity(&mut self, position: &HexPosition) -> Option<Entity> {
        self.tile_entities.remove(position)
    }
    
    pub fn get_tile_entity(&self, position: &HexPosition) -> Option<Entity> {
        self.tile_entities.get(position).copied()
    }
}

/// Fog of war state
#[derive(Resource, Debug, Clone, Default)]
pub struct FogOfWarState {
    pub revealed_tiles: HashMap<HexPosition, f32>, // position -> reveal_intensity (0.0 to 1.0)
    pub permanent_fog: bool, // Whether fog returns after moving away
}

impl FogOfWarState {
    pub fn reveal_tile(&mut self, position: HexPosition, intensity: f32) {
        self.revealed_tiles.insert(position, intensity.clamp(0.0, 1.0));
    }
    
    pub fn is_revealed(&self, position: &HexPosition) -> bool {
        self.revealed_tiles.get(position).unwrap_or(&0.0) > &0.1
    }
    
    pub fn get_reveal_intensity(&self, position: &HexPosition) -> f32 {
        *self.revealed_tiles.get(position).unwrap_or(&0.0)
    }
    
    pub fn fade_fog(&mut self, fade_rate: f32) {
        if self.permanent_fog {
            for (_, intensity) in self.revealed_tiles.iter_mut() {
                *intensity = (*intensity - fade_rate).max(0.0);
            }
        }
    }
}
