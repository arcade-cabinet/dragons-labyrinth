"""
SQLModel tables and Pydantic models for maps and hex grid system.

Combines ORM definitions with hex grid and spatial data models.
Uses SQLModel for Godot-compatible SQLite database.
"""

from datetime import datetime
from enum import Enum
from typing import Any

from pydantic import BaseModel, Field, ConfigDict
from sqlmodel import SQLModel, Field as SQLField, Relationship, Column, JSON


# ======================================
# Maps Types and Enums
# ======================================

class CoordinateType(str, Enum):
    """Type of hex coordinate"""
    BASE = "base"
    CARDINAL = "cardinal"  # N, S, E, W
    DIAGONAL = "diagonal"  # NE, NW, SE, SW
    COMPLEX = "complex"    # N2E3, etc.

class TileType(str, Enum):
    """Base tile types for hex grid"""
    GRASS = "grass"
    FOREST = "forest"
    MOUNTAIN = "mountain"
    WATER = "water"
    DESERT = "desert"
    SWAMP = "swamp"
    CORRUPTED = "corrupted"
    VOID = "void"

class CorruptionLevel(int, Enum):
    """Corruption levels for hex tiles"""
    CLEAN = 0
    SLIGHT = 1
    MODERATE = 2
    HEAVY = 3
    ABSOLUTE = 4

class MapLayer(str, Enum):
    """Tilemap layers for Godot integration"""
    BASE_TERRAIN = "base_terrain"
    CORRUPTION = "corruption"
    ENTITIES = "entities"
    TRAVEL_ROUTES = "travel_routes"
    FOG_OF_WAR = "fog_of_war"

class HexAdjacency(str, Enum):
    """Types of hex adjacency relationships"""
    ADJACENT = "adjacent"      # Direct neighbor
    DIAGONAL = "diagonal"      # Diagonal neighbor
    DISTANT = "distant"        # Multiple hexes away

class TravelRoute(str, Enum):
    """Types of travel routes between hexes"""
    NATURAL_PASS = "natural_pass"
    ROAD = "road"
    TRAIL = "trail"
    RIVER = "river"
    BRIDGE = "bridge"
    TUNNEL = "tunnel"
    CORRUPTED_PATH = "corrupted_path"
    BLOCKED = "blocked"

class HexGridSize(int, Enum):
    """Standard hex grid sizes"""
    SMALL = 50
    MEDIUM = 100
    LARGE = 200
    MASSIVE = 500

class BiomeType(str, Enum):
    """Types of biomes from entities"""
    GRASSLAND = "grassland"
    FOREST = "forest"
    MOUNTAIN = "mountain"
    DESERT = "desert"
    SWAMP = "swamp"
    TUNDRA = "tundra"
    CORRUPTED = "corrupted"

class RegionType(str, Enum):
    """Region types from world system"""
    WILDERNESS = "wilderness"
    SETTLEMENT = "settlement"
    DUNGEON_COMPLEX = "dungeon_complex"
    CORRUPTED_ZONE = "corrupted_zone"
    SAFE_HAVEN = "safe_haven"

# Type aliases
HexCoordinate = str
HexDirection = str
DreadLevel = int  # 0-4
HorrorProgression = float  # 0.0-1.0


# ======================================
# SQLModel ORM Tables
# ======================================

class MapsTimestampedModel(SQLModel):
    """Base model with maps-specific tracking."""

    created_at: datetime = SQLField(default_factory=datetime.now, index=True)
    updated_at: datetime = SQLField(default_factory=datetime.now, index=True)

    # Cross-system integration tracking
    entities_placed: bool = SQLField(default=False, description="Entities placed on map")
    world_coordinated: bool = SQLField(default=False, description="World regions coordinated")
    psychology_overlaid: bool = SQLField(default=False, description="Psychology corruption applied")

    # Hex grid validation
    coordinate_validated: bool = SQLField(default=False, description="Hex coordinate validated")
    adjacency_calculated: bool = SQLField(default=False, description="Adjacency relationships calculated")


class HexTilesLegacy(MapsTimestampedModel, table=True):
    """Individual hex tiles with cross-system coordination. [LEGACY - REPLACED BY SIMPLE 5-TABLE ARCHITECTURE]"""
    __tablename__ = "legacy_hex_tiles"

    id: int | None = SQLField(default=None, primary_key=True)

    # Hex identification
    hex_coordinate: str = SQLField(index=True, unique=True, description="Hex coordinate (BASE, N1, E2S3)")
    coordinate_type: str = SQLField(description="CoordinateType enum value")

    # Spatial positioning
    grid_x: int = SQLField(description="Grid X position")
    grid_y: int = SQLField(description="Grid Y position")
    distance_from_base: int = SQLField(default=0, description="Distance from BASE hex")

    # Terrain from entities
    base_tile_type: str = SQLField(index=True, description="TileType enum value from entities biome")
    biome_type: str = SQLField(index=True, description="BiomeType from entities subpackage")

    # Corruption from psychology
    corruption_level: int = SQLField(default=0, ge=0, le=4, description="CorruptionLevel from psychology")
    dread_level: int = SQLField(default=0, ge=0, le=4, description="DreadLevel from psychology")
    horror_intensity: float = SQLField(default=0.0, ge=0.0, le=1.0, description="Horror intensity from psychology")

    # World coordination
    region_id: str | None = SQLField(default=None, description="Region ID from world subpackage")
    region_fk: int | None = SQLField(default=None, foreign_key="mapregions.id", index=True, description="FK to MapRegions table")
    world_level_min: int = SQLField(default=1, description="Minimum level for this hex")
    world_level_max: int = SQLField(default=180, description="Maximum level for this hex")

    # Entity placement
    has_settlement: bool = SQLField(default=False, description="Has settlement from entities")
    has_dungeon: bool = SQLField(default=False, description="Has dungeon from entities")
    has_npc: bool = SQLField(default=False, description="Has NPC from entities")
    entity_count: int = SQLField(default=0, description="Total entities on this hex")

    # Travel and accessibility
    travel_routes: list[str] = SQLField(default_factory=list, sa_column=Column(JSON), description="List of TravelRoute enum values")
    travel_difficulty: float = SQLField(default=1.0, description="Travel difficulty multiplier")
    companion_safety: float = SQLField(default=0.5, description="Safety for companions from psychology")

    # Godot integration
    tilemap_tile_id: int | None = SQLField(default=None, description="Godot tilemap tile ID")
    tileset_reference: str | None = SQLField(default=None, description="Reference to Godot tileset")
    layer_data: dict[str, Any] = SQLField(default_factory=dict, sa_column=Column(JSON), description="MapLayer -> tile data")
    godot_synced: bool = SQLField(default=False, description="Exported to Godot SQLite")
    godot_synced_at: datetime | None = SQLField(default=None, description="Last sync to Godot SQLite")
    
    # World hooks for Godot integration
    world_hooks: str = SQLField(default="{}", sa_column=Column(JSON), description="Spatial data and integration hooks for Godot")

    # Note: Relationships removed for simple 5-table architecture
    # Legacy complex relationships are no longer needed


class HexAdjacencyTable(MapsTimestampedModel, table=True):
    """Hex adjacency relationships with travel costs."""
    __tablename__ = "hex_adjacency"
    
    id: int | None = SQLField(default=None, primary_key=True)
    
    # Adjacency relationship
    source_hex_id: int = SQLField(foreign_key="legacy_hex_tiles.id")
    target_hex_id: int = SQLField(foreign_key="legacy_hex_tiles.id")
    # Note: Relationships removed for simple 5-table architecture
    
    # Relationship properties
    adjacency_type: str = SQLField(description="HexAdjacency enum value")
    distance: int = SQLField(default=1, description="Distance in hex steps")
    direction: str | None = SQLField(default=None, description="HexDirection enum value")
    
    # Travel properties
    travel_cost: float = SQLField(default=1.0, description="Movement cost multiplier")
    route_type: str = SQLField(default="natural_pass", description="TravelRoute enum value")
    passable: bool = SQLField(default=True, description="Whether path is passable")
    
    # Cross-system modifiers
    corruption_modifier: float = SQLField(default=0.0, description="Corruption travel penalty")
    companion_safety_modifier: float = SQLField(default=0.0, description="Companion safety modifier")
    philosophy_modifier: float = SQLField(default=0.0, description="Philosophy path modifier")


class MapRegions(MapsTimestampedModel, table=True):
    """Map regions coordinating with world subpackage."""
    
    id: int | None = SQLField(default=None, primary_key=True)
    
    # Region identification
    map_region_id: str = SQLField(index=True, unique=True, description="Map region identifier")
    world_region_id: str = SQLField(index=True, description="Corresponding world region ID")
    region_name: str = SQLField(index=True, description="Human-readable region name")
    
    # Spatial bounds
    min_hex_x: int = SQLField(description="Minimum X coordinate")
    max_hex_x: int = SQLField(description="Maximum X coordinate")
    min_hex_y: int = SQLField(description="Minimum Y coordinate")
    max_hex_y: int = SQLField(description="Maximum Y coordinate")
    center_coordinate: str = SQLField(index=True, description="Central hex coordinate")
    
    # Region properties
    region_type: str = SQLField(description="RegionType from world subpackage")
    total_hexes: int = SQLField(default=0, description="Total hexes in region")
    explored_hexes: int = SQLField(default=0, description="Player-explored hexes")
    
    # Cross-system integration
    dominant_biome: str = SQLField(description="Primary BiomeType from entities")
    corruption_average: float = SQLField(default=0.0, description="Average corruption from psychology")
    entity_density: float = SQLField(default=0.0, description="Entities per hex")
    
    # Godot tilemap integration
    tilemap_scene_path: str | None = SQLField(default=None, description="Generated tilemap .tscn path")
    tileset_path: str | None = SQLField(default=None, description="Tileset .tres resource path")
    fog_of_war_enabled: bool = SQLField(default=True, description="Whether fog of war is active")


class TileSets(MapsTimestampedModel, table=True):
    """Godot tileset resources coordinated with all subpackages."""
    
    id: int | None = SQLField(default=None, primary_key=True)
    
    # Tileset identification
    tileset_id: str = SQLField(index=True, unique=True, description="Tileset identifier")
    tileset_name: str = SQLField(description="Human-readable name")
    
    # Godot integration
    tileset_resource_path: str = SQLField(description="Path to .tres tileset resource")
    texture_atlas_path: str = SQLField(description="Path to tileset texture atlas")
    tile_size: int = SQLField(default=64, description="Tile size in pixels")
    
    # Cross-system tile mapping
    biome_tile_mapping: dict[str, list[int]] = SQLField(default_factory=dict, sa_column=Column(JSON), description="BiomeType -> tile IDs")
    corruption_tile_mapping: dict[str, list[int]] = SQLField(default_factory=dict, sa_column=Column(JSON), description="CorruptionLevel -> tile IDs")
    entity_tile_mapping: dict[str, list[int]] = SQLField(default_factory=dict, sa_column=Column(JSON), description="Entity types -> tile IDs")
    
    # Tileset properties
    total_tiles: int = SQLField(default=0, description="Total tiles in tileset")
    animated_tiles: int = SQLField(default=0, description="Number of animated tiles")
    corruption_variants: int = SQLField(default=0, description="Corruption variant tiles")
    
    # Hex compatibility
    hex_compatible: bool = SQLField(default=True, description="Compatible with hex grid")
    hexagon_tilemaplayer_ready: bool = SQLField(default=False, description="Ready for hexagon addon")


class MapGenerationMetrics(MapsTimestampedModel, table=True):
    """Metrics for map generation and cross-system coordination."""
    
    id: int | None = SQLField(default=None, primary_key=True)
    
    # Generation session
    generation_type: str = SQLField(index=True, description="Type of map generation")
    hex_grid_size: int = SQLField(description="HexGridSize enum value")
    
    # Performance metrics
    coordinate_parsing_time: float = SQLField(default=0.0, description="Time spent parsing coordinates")
    entity_placement_time: float = SQLField(default=0.0, description="Time spent placing entities")
    corruption_overlay_time: float = SQLField(default=0.0, description="Time spent applying corruption")
    tilemap_generation_time: float = SQLField(default=0.0, description="Time spent generating tilemaps")
    
    # Results
    hexes_generated: int = SQLField(default=0, description="Number of hex tiles generated")
    regions_mapped: int = SQLField(default=0, description="Number of regions mapped")
    tilesets_created: int = SQLField(default=0, description="Number of tilesets created")
    
    # Quality metrics
    coordinate_accuracy: float = SQLField(default=0.0, ge=0.0, le=1.0, description="Coordinate parsing accuracy")
    placement_coherence: float = SQLField(default=0.0, ge=0.0, le=1.0, description="Entity placement coherence")
    cross_system_integration: float = SQLField(default=0.0, ge=0.0, le=1.0, description="Integration quality")
    
    # Error tracking
    coordinate_errors: int = SQLField(default=0, description="Coordinate parsing errors")
    placement_errors: int = SQLField(default=0, description="Entity placement errors")
    generation_errors: int = SQLField(default=0, description="General generation errors")


# ======================================
# Pydantic Data Models
# ======================================

class HexTileData(BaseModel):
    """Complete hex tile data structure"""
    coordinate: HexCoordinate
    coordinate_type: CoordinateType
    grid_position: tuple[int, int]
    distance_from_base: int
    
    # Terrain data
    tile_type: TileType
    biome_type: BiomeType
    
    # Cross-system data
    corruption_level: CorruptionLevel
    dread_level: DreadLevel
    horror_intensity: HorrorProgression
    
    # Entity placement
    has_settlement: bool
    has_dungeon: bool
    has_npc: bool
    entity_count: int
    
    # Travel data
    travel_routes: list[TravelRoute]
    travel_difficulty: float
    companion_safety: float
    
    # World integration
    region_id: str | None
    level_range: tuple[int, int]
    
    model_config = ConfigDict(extra="forbid")


class HexRegionData(BaseModel):
    """Map region data structure"""
    region_id: str
    name: str
    region_type: RegionType
    
    # Spatial bounds
    bounds: dict[str, int]  # min_x, max_x, min_y, max_y
    center_coordinate: HexCoordinate
    total_hexes: int
    
    # Cross-system data
    dominant_biome: BiomeType
    corruption_average: float
    entity_density: float
    
    # Exploration
    explored_hexes: int
    fog_of_war_enabled: bool
    
    model_config = ConfigDict(extra="forbid")


class TileSetData(BaseModel):
    """Tileset configuration for Godot"""
    tileset_id: str
    name: str
    resource_path: str
    texture_atlas_path: str
    tile_size: int
    
    # Tile mappings
    biome_mapping: dict[str, list[int]]
    corruption_mapping: dict[str, list[int]]
    entity_mapping: dict[str, list[int]]
    
    # Properties
    total_tiles: int
    animated_tiles: int
    hex_compatible: bool
    
    model_config = ConfigDict(extra="forbid")


class MapGenerationConfig(BaseModel):
    """Configuration for map generation"""
    grid_size: HexGridSize
    generation_type: str
    
    # Cross-system integration
    integrate_entities: bool = True
    apply_corruption: bool = True
    coordinate_world_regions: bool = True
    
    # Godot integration
    generate_tilemaps: bool = True
    create_tilesets: bool = True
    enable_fog_of_war: bool = True
    
    model_config = ConfigDict(extra="forbid")


class HexGrid(BaseModel):
    """Complete hex grid structure"""
    base_coordinate: HexCoordinate = "BASE"
    grid_size: HexGridSize
    total_hexes: int
    
    # Hex tiles
    hexes: dict[HexCoordinate, HexTileData]
    regions: dict[str, HexRegionData]
    
    # Adjacency data
    adjacency_map: dict[HexCoordinate, list[HexCoordinate]]
    
    # Cross-system integration
    entities_integrated: bool = False
    psychology_applied: bool = False
    world_coordinated: bool = False
    
    model_config = ConfigDict(extra="forbid")
