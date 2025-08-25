//! Hex Tile ECS Components - Layer Cake System with Hexx Integration
//!
//! Pure Bevy ECS components for sophisticated hex tile management with layer cake architecture.
//! Integrates with hexx for hex math and bevy_ecs_tilemap for rendering.

use bevy::prelude::*;
use hexx::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Base hex tile component - the coordinate container
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct HexTile {
    /// Hexx coordinate for this tile
    pub coord: Hex,
    /// World position for rendering
    pub world_position: Vec3,
    /// Chunk coordinate for memory management
    pub chunk_coord: ChunkCoord,
}

impl HexTile {
    pub fn new(coord: Hex, hex_layout: &HexLayout) -> Self {
        let world_position = hex_layout.hex_to_world_pos(coord).extend(0.0);
        let chunk_coord = ChunkCoord::from_hex(coord, 16); // 16x16 hex chunks
        
        Self {
            coord,
            world_position,
            chunk_coord,
        }
    }
}

/// Biome layer - the base terrain type with adjacency validation
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct Biome {
    pub biome_type: BiomeType,
    pub variant: String,
    pub elevation: f32,
    pub moisture: f32,
    pub temperature: f32,
    /// Whether this biome can be adjacent to specific other biomes
    pub adjacency_valid: bool,
}

impl Biome {
    pub fn can_be_adjacent_to(&self, other: BiomeType) -> bool {
        use BiomeType::*;
        
        match self.biome_type {
            Lava => matches!(other, Desert | Mountain | Wasteland | CursedLand),
            Snow => matches!(other, Mountain | Forest | Plains | Tundra),
            Forest => matches!(other, Plains | Mountain | Snow | Swamp | Coast),
            Ocean => matches!(other, Coast | Plains | Forest),
            Desert => matches!(other, Lava | Wasteland | Plains | Mountain),
            Swamp => matches!(other, Forest | Plains | Coast),
            _ => true, // Most biomes are flexible
        }
    }
}

/// Path layer - transparent overlay with pathfinding and speed rules
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct Path {
    pub path_type: PathType,
    pub material: PathMaterial,
    /// Auto-generated connections to matching path types
    pub connections: Vec<Hex>,
    /// Speed modifier (+1 modifier on tiles with -1 movement penalty)
    pub speed_modifier: i32,
    /// Path quality affects durability
    pub quality: f32,
    /// Asset handle for path overlay texture
    pub overlay_asset: Option<Handle<Image>>,
}

impl Path {
    /// Pathfinding rule: connects to matching path material types
    pub fn can_connect_to(&self, other_path: &Path) -> bool {
        self.material == other_path.material && self.path_type.is_compatible_with(&other_path.path_type)
    }
    
    /// Speed modification rule
    pub fn get_movement_modifier(&self, base_biome_modifier: i32) -> i32 {
        if base_biome_modifier < 0 {
            base_biome_modifier + self.speed_modifier // Paths cancel negative terrain
        } else {
            base_biome_modifier
        }
    }
}

/// Feature layer - interactive overlay for content
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct Feature {
    pub feature_type: FeatureType,
    pub name: String,
    pub interaction_type: InteractionType,
    /// Requirements to interact (items, skills, etc.)
    pub requirements: Vec<String>,
    /// What interaction provides
    pub rewards: Vec<String>,
    /// Asset handle for feature model/texture
    pub feature_asset: Option<Handle<Scene>>,
}

/// Corruption component for horror progression
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct Corruption {
    /// Corruption level (0.0-1.0)
    pub level: f32,
    /// Entity that caused this corruption
    pub source: Option<Entity>,
    /// How fast corruption spreads from this tile
    pub spread_rate: f32,
    /// Biome-specific resistance to corruption
    pub resistance: f32,
    /// Horror events that occurred here
    pub horror_events_count: u32,
}

impl Corruption {
    pub fn dread_level(&self) -> u8 {
        match self.level {
            x if x >= 0.8 => 4,
            x if x >= 0.6 => 3,
            x if x >= 0.4 => 2,
            x if x >= 0.2 => 1,
            _ => 0,
        }
    }
}

/// Discovery state for exploration mechanics
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct DiscoveryState {
    pub discovered: bool,
    pub fully_explored: bool,
    pub exploration_progress: f32, // 0.0-1.0
    pub discovery_time: Option<f64>,
    pub last_visited: Option<f64>,
}

/// Dread effects for tile-specific horror
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct DreadEffects {
    pub dread_level: u8,
    /// Visual overlay texture name
    pub visual_overlay: String,
    /// Audio effect names to play
    pub audio_effects: Vec<String>,
    /// Behavioral modifications for NPCs/companions
    pub behavioral_modifications: HashMap<String, f32>,
    /// How much sanity this tile drains per second
    pub sanity_drain_rate: f32,
}

/// Forge system essence for light/dark mechanics
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct ForgeEssence {
    /// Light essence strength (0.0-1.0)
    pub light_strength: f32,
    /// Dark essence strength (0.0-1.0)
    pub dark_strength: f32,
    /// How stable the essence is
    pub stability: f32,
    /// Whether essence can be harvested
    pub harvestable: bool,
}

/// Environmental storytelling for atmosphere
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct AtmosphericDescription {
    /// Base atmospheric description
    pub base_description: String,
    /// Description when corrupted
    pub corruption_description: Option<String>,
    /// Weather-modified descriptions
    pub weather_descriptions: HashMap<String, String>,
    /// Time-of-day variations
    pub time_descriptions: HashMap<String, String>,
}

/// Weather effects specific to this tile
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct TileWeather {
    pub current_condition: String,
    pub visibility_modifier: f32,
    pub movement_modifier: f32,
    pub weather_overlay: Option<String>,
}

/// Chunk coordinate for memory-optimized loading
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect, Serialize, Deserialize)]
pub struct ChunkCoord {
    pub q: i32,
    pub r: i32,
}

impl ChunkCoord {
    pub fn from_hex(hex: Hex, chunk_size: u32) -> Self {
        let chunk_size = chunk_size as i32;
        Self {
            q: hex.q.div_euclid(chunk_size),
            r: hex.r.div_euclid(chunk_size),
        }
    }
    
    pub fn hex_positions(&self, chunk_size: u32) -> Vec<Hex> {
        let chunk_size = chunk_size as i32;
        let start_q = self.q * chunk_size;
        let start_r = self.r * chunk_size;
        
        let mut positions = Vec::new();
        for q in start_q..(start_q + chunk_size) {
            for r in start_r..(start_r + chunk_size) {
                positions.push(Hex::new(q, r));
            }
        }
        positions
    }
}

// Enums for layer cake system

#[derive(Debug, Clone, PartialEq, Eq, Hash, Reflect, Serialize, Deserialize)]
pub enum BiomeType {
    Plains,
    Forest,
    Mountain,
    Desert,
    Snow,
    Tundra,
    Swamp,
    Coast,
    Ocean,
    Lava,
    Wasteland,
    CursedLand,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Reflect, Serialize, Deserialize)]
pub enum PathType {
    None,
    Trail,
    Road,
    Bridge,
    Tunnel,
    MagicPortal,
}

impl PathType {
    /// Path compatibility rules for connections
    pub fn is_compatible_with(&self, other: &PathType) -> bool {
        match (self, other) {
            (PathType::None, PathType::None) => false,
            (PathType::None, _) | (_, PathType::None) => false,
            (PathType::MagicPortal, PathType::MagicPortal) => true,
            (PathType::MagicPortal, _) | (_, PathType::MagicPortal) => false,
            _ => true, // Most path types can connect
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Reflect, Serialize, Deserialize)]
pub enum PathMaterial {
    None,
    Wood,
    Stone,
    Brick,
    Metal,
    Magic,
    Dirt,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Reflect, Serialize, Deserialize)]
pub enum FeatureType {
    None,
    Tavern,
    Dungeon,
    Shrine,
    Village,
    Ruins,
    Tower,
    Cave,
    Spring,
    Battlefield,
    DragonLair,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Reflect, Serialize, Deserialize)]
pub enum InteractionType {
    None,
    Rest,
    Trade,
    Explore,
    Combat,
    Puzzle,
    Story,
    Forge,
}

/// Bundle for spawning complete layer cake hex tiles
#[derive(Bundle)]
pub struct LayerCakeHexTileBundle {
    pub hex_tile: HexTile,
    pub biome: Biome,
    pub path: Path,
    pub feature: Feature,
    pub corruption: Corruption,
    pub discovery_state: DiscoveryState,
    pub dread_effects: DreadEffects,
    pub forge_essence: ForgeEssence,
    pub atmospheric_description: AtmosphericDescription,
    pub tile_weather: TileWeather,
    
    // Bevy rendering components
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}

impl LayerCakeHexTileBundle {
    pub fn new(coord: Hex, hex_layout: &HexLayout, biome_type: BiomeType) -> Self {
        let hex_tile = HexTile::new(coord, hex_layout);
        let transform = Transform::from_translation(hex_tile.world_position);
        
        Self {
            hex_tile,
            biome: Biome {
                biome_type,
                variant: "default".to_string(),
                elevation: 0.0,
                moisture: 0.5,
                temperature: 0.5,
                adjacency_valid: true,
            },
            path: Path {
                path_type: PathType::None,
                connections: Vec::new(),
                quality: 1.0,
                maintenance_level: 1.0,
                overlay_asset: None,
            },
            feature: Feature {
                feature_type: FeatureType::None,
                name: "Empty".to_string(),
                interaction_type: InteractionType::None,
                requirements: Vec::new(),
                rewards: Vec::new(),
                feature_asset: None,
            },
            corruption: Corruption {
                level: 0.0,
                source: None,
                spread_rate: 0.0,
                resistance: 0.2,
                horror_events_count: 0,
            },
            discovery_state: DiscoveryState {
                discovered: false,
                fully_explored: false,
                exploration_progress: 0.0,
                discovery_time: None,
                last_visited: None,
            },
            dread_effects: DreadEffects {
                dread_level: 0,
                visual_overlay: "none".to_string(),
                audio_effects: Vec::new(),
                behavioral_modifications: HashMap::new(),
                sanity_drain_rate: 0.0,
            },
            forge_essence: ForgeEssence {
                light_strength: 0.1,
                dark_strength: 0.0,
                stability: 1.0,
                harvestable: false,
            },
            atmospheric_description: AtmosphericDescription {
                base_description: format!("A peaceful {} tile", format!("{:?}", biome_type).to_lowercase()),
                corruption_description: None,
                weather_descriptions: HashMap::new(),
                time_descriptions: HashMap::new(),
            },
            tile_weather: TileWeather {
                current_condition: "Clear".to_string(),
                visibility_modifier: 1.0,
                movement_modifier: 1.0,
                weather_overlay: None,
            },
            transform,
            global_transform: GlobalTransform::default(),
            visibility: Visibility::default(),
            inherited_visibility: InheritedVisibility::default(),
            view_visibility: ViewVisibility::default(),
        }
    }
}
