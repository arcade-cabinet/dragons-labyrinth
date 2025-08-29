"""
SQLModel tables and Pydantic models for world coordination system.

Combines ORM definitions with world coordination data models.
Uses SQLModel for Godot-compatible SQLite database.
"""

from datetime import datetime
from enum import Enum
from typing import Any

from pydantic import BaseModel, Field, ConfigDict
from sqlmodel import SQLModel, Field as SQLField, Relationship, Column, JSON, DateTime


# ======================================
# World Types and Enums
# ======================================

class RegionType(str, Enum):
    """Type classification for world regions"""
    WILDERNESS = "wilderness"
    SETTLEMENT = "settlement"
    DUNGEON_COMPLEX = "dungeon_complex"
    CORRUPTED_ZONE = "corrupted_zone"
    SAFE_HAVEN = "safe_haven"
    TRANSITIONAL = "transitional"

class CampaignStage(str, Enum):
    """Campaign progression stages"""
    PROLOGUE = "prologue"
    ACT_1_PEACE = "act_1_peace"
    ACT_1_UNEASE = "act_1_unease"
    ACT_2_DREAD = "act_2_dread"
    ACT_2_TERROR = "act_2_terror"
    ACT_3_HORROR = "act_3_horror"
    ACT_3_VOID = "act_3_void"
    EPILOGUE = "epilogue"

class WorldState(str, Enum):
    """Overall state of the world"""
    PRISTINE = "pristine"
    TROUBLED = "troubled"
    CORRUPTED = "corrupted"
    BROKEN = "broken"
    VOID = "void"

class PhilosophyPath(str, Enum):
    """Player's philosophical alignment"""
    LIGHT = "light"
    NEUTRAL = "neutral"
    DARK = "dark"
    PRAGMATIC = "pragmatic"
    COMPASSIONATE = "compassionate"
    RUTHLESS = "ruthless"

class RegionFeature(str, Enum):
    """Features that can be present in regions"""
    TRADE_ROUTE = "trade_route"
    ANCIENT_RUINS = "ancient_ruins"
    CORRUPTION_SOURCE = "corruption_source"
    SAFE_ZONE = "safe_zone"
    DRAGON_SIGN = "dragon_sign"
    VOID_BREACH = "void_breach"
    REFUGEE_CAMP = "refugee_camp"
    ABANDONED_SETTLEMENT = "abandoned_settlement"

class WorldComplexity(int, Enum):
    """Complexity levels for world generation"""
    SIMPLE = 1
    STANDARD = 2
    COMPLEX = 3
    EPIC = 4
    LEGENDARY = 5

class CorruptionStage(str, Enum):
    """Stages of world corruption"""
    CLEAN = "clean"
    WITHERED = "withered"
    SCORCHED = "scorched"
    VOID = "void"

class BiomeType(str, Enum):
    """Types of biomes"""
    GRASSLAND = "grassland"
    FOREST = "forest"
    MOUNTAIN = "mountain"
    DESERT = "desert"
    SWAMP = "swamp"
    TUNDRA = "tundra"
    CORRUPTED = "corrupted"

# Type aliases
RegionId = str
CampaignId = str
WorldLevel = int  # 1-180
WorldProgression = float  # 0.0-1.0
HorrorProgression = float  # 0.0-1.0
DreadLevel = int  # 0-4


# ======================================
# SQLModel ORM Tables
# ======================================

class WorldTimestampedModel(SQLModel):
    """Base model with world-specific tracking."""
    
    created_at: datetime = SQLField(default_factory=datetime.now, sa_column=Column(DateTime), index=True)
    updated_at: datetime = SQLField(default_factory=datetime.now, sa_column=Column(DateTime), index=True)
    
    # Cross-system integration tracking
    entities_integrated: bool = SQLField(default=False, description="Entities data integrated")
    seeds_integrated: bool = SQLField(default=False, description="Seeds data integrated")
    psychology_integrated: bool = SQLField(default=False, description="Psychology data integrated")
    maps_integrated: bool = SQLField(default=False, description="Maps data integrated")
    
    # World generation metadata
    generation_complexity: int = SQLField(default=2, description="WorldComplexity enum value")
    validation_score: float = SQLField(default=0.0, description="World validation score")
    cross_system_coherence: float = SQLField(default=0.0, description="Cross-system coherence score")


class Regions(WorldTimestampedModel, table=True):
    """World regions coordinating entities, psychology, and map data."""
    
    id: int | None = SQLField(default=None, primary_key=True)
    
    # Region identification
    region_id: str = SQLField(index=True, unique=True, description="Unique region identifier")
    region_name: str = SQLField(index=True, description="Human-readable region name")
    region_type: str = SQLField(index=True, description="RegionType enum value")
    
    # World positioning
    campaign_stage: str = SQLField(index=True, description="CampaignStage enum value")
    level_range_min: int = SQLField(ge=1, le=180, description="Minimum level for this region")
    level_range_max: int = SQLField(ge=1, le=180, description="Maximum level for this region")
    
    # Horror progression integration
    base_dread_level: int = SQLField(ge=0, le=4, description="Base dread level from psychology")
    corruption_stage: str = SQLField(default="CLEAN", description="CorruptionStage enum value")
    horror_escalation_rate: float = SQLField(default=0.1, ge=0.0, le=1.0)
    
    # Entity integration
    primary_biome_type: str = SQLField(description="Primary BiomeType from entities")
    settlement_count: int = SQLField(default=0, description="Number of settlements from entities")
    dungeon_count: int = SQLField(default=0, description="Number of dungeons from entities")
    npc_count: int = SQLField(default=0, description="Number of NPCs from entities")
    
    # Seeds integration
    narrative_themes: str = SQLField(default="[]", sa_column=Column(JSON), description="Narrative themes from seeds")
    emotional_patterns: str = SQLField(default="[]", sa_column=Column(JSON), description="Emotional patterns from seeds")
    motif_influences: str = SQLField(default="[]", sa_column=Column(JSON), description="Motif influences from seeds")
    
    # Philosophy integration
    dominant_philosophy: str = SQLField(default="NEUTRAL", description="Dominant PhilosophyPath")
    philosophy_strongholds: str = SQLField(default="{}", sa_column=Column(JSON), description="Philosophy -> locations mapping")
    moral_choice_density: float = SQLField(default=0.5, ge=0.0, le=1.0)
    
    # Regional features
    features: str = SQLField(default="[]", sa_column=Column(JSON), description="List of RegionFeature enum values")
    travel_difficulty: float = SQLField(default=1.0, ge=0.1, le=5.0, description="Travel difficulty multiplier")
    companion_safety: float = SQLField(default=0.5, ge=0.0, le=1.0, description="Safety for companions")
    
    # Godot integration
    godot_scene_path: str | None = SQLField(default=None, description="Path to generated .tscn")
    godot_resource_path: str | None = SQLField(default=None, description="Path to generated .tres")
    tileset_reference: str | None = SQLField(default=None, description="Reference to map tileset")
    
    # Relationships
    campaigns: list["Campaigns"] = Relationship(back_populates="regions")


class Campaigns(WorldTimestampedModel, table=True):
    """Campaign progression structures coordinating all systems."""
    
    id: int | None = SQLField(default=None, primary_key=True)
    
    # Campaign identification
    campaign_id: str = SQLField(index=True, unique=True, description="Unique campaign identifier")
    campaign_name: str = SQLField(description="Human-readable campaign name")
    campaign_stage: str = SQLField(index=True, description="Current CampaignStage")
    
    # Progression tracking
    current_level: int = SQLField(default=1, ge=1, le=180, description="Current campaign level")
    current_world_state: str = SQLField(default="PRISTINE", description="WorldState enum value")
    overall_progression: float = SQLField(default=0.0, ge=0.0, le=1.0, description="0-1 campaign completion")
    
    # Act structure
    act_1_completed: bool = SQLField(default=False, description="Act 1 (Peace to Terror) complete")
    act_2_completed: bool = SQLField(default=False, description="Act 2 (Terror to Madness) complete")
    act_3_completed: bool = SQLField(default=False, description="Act 3 (Madness to Void) complete")
    
    # Cross-system integration
    active_regions: str = SQLField(default="[]", sa_column=Column(JSON), description="List of active region IDs")
    available_regions: str = SQLField(default="[]", sa_column=Column(JSON), description="List of available region IDs")
    unlocked_regions: str = SQLField(default="[]", sa_column=Column(JSON), description="List of unlocked region IDs")
    
    # Psychology integration
    player_psychology_state: str = SQLField(default="{}", sa_column=Column(JSON), description="Player psychological state")
    active_companions: str = SQLField(default="[]", sa_column=Column(JSON), description="List of active companion IDs")
    companion_states: str = SQLField(default="{}", sa_column=Column(JSON), description="Companion ID -> state mapping")
    total_trauma_events: int = SQLField(default=0, description="Total trauma events across campaign")
    
    # Philosophy tracking
    player_philosophy_path: str = SQLField(default="NEUTRAL", description="Player's philosophy path")
    philosophy_choices_made: str = SQLField(default="{}", sa_column=Column(JSON), description="Philosophy -> choice count")
    moral_reputation: float = SQLField(default=0.0, ge=-1.0, le=1.0, description="Overall moral standing")
    
    # World state tracking
    corruption_spread: float = SQLField(default=0.0, ge=0.0, le=1.0, description="World corruption level")
    dragon_awakening_level: float = SQLField(default=0.0, ge=0.0, le=1.0, description="Dragon awakening progress")
    void_influence: float = SQLField(default=0.0, ge=0.0, le=1.0, description="Void corruption influence")
    social_collapse_level: float = SQLField(default=0.0, ge=0.0, le=1.0, description="Social order breakdown")
    
    # Narrative tracking
    major_revelations: str = SQLField(default="[]", sa_column=Column(JSON), description="List of major story revelations")
    story_beats_completed: str = SQLField(default="[]", sa_column=Column(JSON), description="List of completed story beats")
    ending_trajectory: str = SQLField(default="unknown", description="Which ending player is heading toward")
    
    # Performance metrics
    total_playtime_hours: float = SQLField(default=0.0, description="Total campaign playtime")
    average_session_length: float = SQLField(default=0.0, description="Average session length in hours")
    difficulty_modifiers: str = SQLField(default="{}", sa_column=Column(JSON), description="Applied difficulty modifiers")
    
    # Relationships
    region_id: int | None = SQLField(default=None, foreign_key="regions.id")
    regions: Regions | None = Relationship(back_populates="campaigns")


class WorldStateTable(WorldTimestampedModel, table=True):
    """Global world state coordinating all systems in real-time."""
    __tablename__ = "world_state"
    
    id: int | None = SQLField(default=None, primary_key=True)
    
    # World identification
    world_instance_id: str = SQLField(index=True, unique=True, description="Unique world instance")
    world_name: str = SQLField(default="Dragon's Labyrinth", description="World name")
    world_version: str = SQLField(default="1.0", description="World generation version")
    
    # Current state
    current_world_state: str = SQLField(index=True, description="WorldState enum value")
    global_dread_level: int = SQLField(default=0, ge=0, le=4, description="Global dread level")
    world_corruption: float = SQLField(default=0.0, ge=0.0, le=1.0, description="Overall world corruption")
    
    # Time and progression
    world_time_elapsed: float = SQLField(default=0.0, description="In-world time elapsed in days")
    real_time_elapsed: float = SQLField(default=0.0, description="Real-world time elapsed in hours")
    events_triggered: int = SQLField(default=0, description="Total world events triggered")
    
    # Cross-system state
    total_entities_active: int = SQLField(default=0, description="Active entities in world")
    active_horror_sources: int = SQLField(default=0, description="Active horror sources from psychology")
    narrative_threads_active: int = SQLField(default=0, description="Active narrative threads from seeds")
    regions_discovered: int = SQLField(default=0, description="Total regions discovered")
    
    # Dragon system
    dragon_location: str | None = SQLField(default=None, description="Current dragon location")
    dragon_awakeness: float = SQLField(default=0.0, ge=0.0, le=1.0, description="Dragon awakeness level")
    dragon_aggression: float = SQLField(default=0.0, ge=0.0, le=1.0, description="Dragon aggression level")
    dragon_corruption_radius: float = SQLField(default=0.0, description="Dragon's corruption influence radius")
    
    # Void system
    void_breaches: int = SQLField(default=0, description="Number of void breaches in world")
    void_corruption_zones: str = SQLField(default="[]", sa_column=Column(JSON), description="List of void-corrupted areas")
    reality_stability: float = SQLField(default=1.0, ge=0.0, le=1.0, description="Reality stability level")
    temporal_anomalies: int = SQLField(default=0, description="Number of time distortions")
    
    # Social systems
    civilization_stability: float = SQLField(default=1.0, ge=0.0, le=1.0, description="Civilizational stability")
    trade_route_integrity: float = SQLField(default=1.0, ge=0.0, le=1.0, description="Trade system functionality")
    political_alliances: str = SQLField(default="{}", sa_column=Column(JSON), description="Political alliance states")
    refugee_populations: int = SQLField(default=0, description="Displaced population count")


class RegionalProgression(WorldTimestampedModel, table=True):
    """Regional progression tracking coordinating with psychology horror curves."""
    
    id: int | None = SQLField(default=None, primary_key=True)
    
    # Region reference
    region_id: str = SQLField(index=True, description="Reference to Regions table")
    progression_name: str = SQLField(description="Name of progression curve")
    
    # Level-based progression
    level_curve_data: str = SQLField(sa_column=Column(JSON), description="Level -> horror/corruption mapping")
    dread_progression_curve: str = SQLField(sa_column=Column(JSON), description="Level -> dread level mapping")
    corruption_timeline: str = SQLField(sa_column=Column(JSON), description="Time -> corruption events")
    
    # Psychology integration
    companion_trauma_curve: str = SQLField(default="{}", sa_column=Column(JSON), description="Trauma accumulation by level")
    player_psychology_curve: str = SQLField(default="{}", sa_column=Column(JSON), description="Player psychology changes")
    horror_escalation_points: str = SQLField(default="[]", sa_column=Column(JSON), description="Major horror escalation levels")
    
    # Narrative integration
    story_beat_mapping: str = SQLField(sa_column=Column(JSON), description="Level -> story beats from seeds")
    revelation_points: str = SQLField(default="[]", sa_column=Column(JSON), description="Levels with major revelations")
    philosophy_choice_points: str = SQLField(default="[]", sa_column=Column(JSON), description="Levels with philosophy choices")


class WorldGenerationMetrics(WorldTimestampedModel, table=True):
    """Metrics for world generation and cross-system coordination."""
    
    id: int | None = SQLField(default=None, primary_key=True)
    
    # Generation session
    generation_type: str = SQLField(index=True, description="Type of world generation")
    generation_complexity: int = SQLField(description="WorldComplexity enum value")
    
    # Performance metrics
    total_generation_time: float = SQLField(default=0.0, description="Total generation time in seconds")
    entities_processing_time: float = SQLField(default=0.0, description="Time spent processing entities")
    seeds_processing_time: float = SQLField(default=0.0, description="Time spent processing seeds")
    psychology_processing_time: float = SQLField(default=0.0, description="Time spent on psychology")
    coordination_time: float = SQLField(default=0.0, description="Time spent on cross-system coordination")
    
    # Quality metrics
    world_coherence_score: float = SQLField(default=0.0, ge=0.0, le=1.0, description="Overall world coherence")
    cross_system_integration_score: float = SQLField(default=0.0, ge=0.0, le=1.0, description="Integration quality")
    narrative_consistency_score: float = SQLField(default=0.0, ge=0.0, le=1.0, description="Narrative consistency")
    gameplay_balance_score: float = SQLField(default=0.0, ge=0.0, le=1.0, description="Gameplay balance")
    
    # Generation results
    regions_generated: int = SQLField(default=0, description="Number of regions generated")
    campaigns_created: int = SQLField(default=0, description="Number of campaigns created")
    godot_resources_generated: int = SQLField(default=0, description="Number of Godot resources created")
    
    # Error tracking
    generation_errors: int = SQLField(default=0, description="Number of generation errors")
    coordination_failures: int = SQLField(default=0, description="Number of coordination failures")
    validation_warnings: int = SQLField(default=0, description="Number of validation warnings")


# ======================================
# Pydantic Data Models
# ======================================

class RegionData(BaseModel):
    """Complete region data structure"""
    region_id: str
    name: str
    region_type: RegionType
    campaign_stage: CampaignStage
    level_range: tuple[int, int]
    
    # Horror progression
    dread_level: DreadLevel
    corruption_stage: CorruptionStage
    horror_escalation_rate: float
    
    # Cross-system data
    primary_biome: BiomeType
    entity_counts: dict[str, int]
    narrative_themes: list[str]
    emotional_patterns: list[str]
    features: list[RegionFeature]
    
    # Gameplay
    travel_difficulty: float
    companion_safety: float
    moral_choice_density: float
    
    model_config = ConfigDict(extra="forbid")


class CampaignProgress(BaseModel):
    """Campaign progression state"""
    campaign_id: str
    name: str
    stage: CampaignStage
    
    # Level progression
    current_level: WorldLevel
    overall_progression: WorldProgression
    
    # Act completion
    acts_completed: dict[str, bool]
    
    # Cross-system state
    active_regions: list[str]
    player_psychology: dict[str, Any]
    companion_states: dict[str, str]
    philosophy_path: PhilosophyPath
    
    # World impact
    corruption_spread: float
    dragon_awakening: float
    ending_trajectory: str
    
    model_config = ConfigDict(extra="forbid")


class WorldSystemState(BaseModel):
    """Complete world system state"""
    world_id: str
    name: str
    state: WorldState
    
    # Global metrics
    global_dread_level: DreadLevel
    world_corruption: float
    reality_stability: float
    
    # Time tracking
    world_time_days: float
    real_time_hours: float
    
    # Dragon system
    dragon_location: str | None
    dragon_awakeness: float
    dragon_aggression: float
    
    # Social systems
    civilization_stability: float
    refugee_populations: int
    
    model_config = ConfigDict(extra="forbid")
