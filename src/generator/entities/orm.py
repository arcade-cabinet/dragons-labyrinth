"""
Dragon's Labyrinth ORM Models - SQLModel tables for godot-sqlite.

All tables for the unified game.db that Godot will use directly.
Python 3.13 standards: no Optional, absolute imports, match case.
"""

from datetime import datetime

from sqlmodel import Field, SQLModel, Relationship

from generator.entities.types import (
    BiomeType,
    DungeonType,
    SettlementType,
    DwellingType,
    FactionType,
    CreatureTier,
    CoordinateType,
    DreadLevel,
    CorruptionStage,
    PhilosophyPath,
    EntityTableType,
    HBFEntityId,
    HexCoordinate,
    RegionName,
    WorldName,
    FeatureName
)


# =============================================================================
# BASE MODELS
# =============================================================================

class TimestampedModel(SQLModel):
    """Base model with timestamp tracking."""
    
    created_at: datetime = Field(default_factory=datetime.now, index=True)
    updated_at: datetime = Field(default_factory=datetime.now, index=True)
    
    # HBF tracking
    hbf_uuid: str = Field(index=True, description="Original HBF entity UUID")
    extraction_confidence: float = Field(default=0.0, description="ML extraction confidence")


# =============================================================================
# BIOME TABLE
# =============================================================================

class Biome(TimestampedModel, table=True):
    """Biome data from hex tiles."""
    
    id: int | None = Field(default=None, primary_key=True)
    
    # Core hex data
    coordinate: str = Field(index=True, description="Hex coordinate (BASE, N1, E2S3)")
    region: str = Field(index=True, description="Region name")
    world: str = Field(default="The Lands of Vo'il")
    
    # Biome classification
    biome_type: str = Field(description="jungle, mountains, forest, deep_forest")
    corruption_level: int = Field(default=0, ge=0, le=3)
    dread_level: int = Field(default=0, ge=0, le=4)
    
    # Content flags
    has_encounters: bool = Field(default=False)
    has_weather: bool = Field(default=False)
    has_npcs: bool = Field(default=False)
    has_treasure: bool = Field(default=False)
    has_boss: bool = Field(default=False)
    has_faction: bool = Field(default=False)
    
    # Environmental data
    feature_name: str | None = Field(default=None)
    environmental_description: str | None = Field(default=None)
    
    # Relationships
    monsters: list["Monster"] = Relationship(back_populates="biome")
    settlements: list["Settlement"] = Relationship(back_populates="biome")
    dungeons: list["Dungeon"] = Relationship(back_populates="biome")


# =============================================================================
# MONSTER TABLE
# =============================================================================

class Monster(TimestampedModel, table=True):
    """Creature/monster data with horror variants."""
    
    id: int | None = Field(default=None, primary_key=True)
    
    # Core stats
    name: str = Field(index=True)
    base_name: str = Field(description="Name without corruption variant")
    corruption_variant: str | None = Field(default=None)
    
    # Combat stats
    challenge_rating: float = Field(default=0.0)
    threat_level: int = Field(default=1, ge=1, le=10)
    health_points: int = Field(default=1)
    armor_class: int = Field(default=10)
    movement_speed: int = Field(default=30)
    
    # Horror progression
    dread_level: int = Field(default=0, ge=0, le=4)
    horror_impact: float = Field(default=0.0)
    trauma_triggers: str | None = Field(default=None, description="JSON list of triggers")
    
    # D&D abilities (JSON)
    abilities: str | None = Field(default=None, description="JSON dict of STR/DEX/CON/INT/WIS/CHA")
    
    # Location
    biome_id: int | None = Field(default=None, foreign_key="biome.id")
    biome: Biome | None = Relationship(back_populates="monsters")


# =============================================================================
# INN TABLE
# =============================================================================

class Inn(TimestampedModel, table=True):
    """Isolated inns for healing and rest."""
    
    id: int | None = Field(default=None, primary_key=True)
    
    # Core data
    name: str = Field(index=True)
    is_isolated: bool = Field(default=True)
    healing_available: bool = Field(default=True)
    
    # Location
    region: str = Field(index=True)
    philosophy_alignment: str = Field(default="neutral")
    
    # Services (JSON)
    services: str | None = Field(default=None, description="JSON list of available services")
    
    # NPCs
    npcs: list["NPC"] = Relationship(back_populates="inn")


# =============================================================================
# DUNGEON TABLES (Cave, Temple, Tomb)
# =============================================================================

class Dungeon(TimestampedModel, table=True):
    """Base dungeon coordinator table."""
    
    id: int | None = Field(default=None, primary_key=True)
    
    # Core data
    name: str = Field(index=True)
    dungeon_type: str = Field(index=True, description="cave, temple, or tomb")
    
    # Horror elements
    corruption_level: int = Field(default=0, ge=0, le=3)
    dread_level: int = Field(default=0, ge=0, le=4)
    
    # Location
    region: str = Field(index=True)
    biome_id: int | None = Field(default=None, foreign_key="biome.id")
    biome: Biome | None = Relationship(back_populates="dungeons")
    
    # Content
    has_boss: bool = Field(default=False)
    has_treasure: bool = Field(default=False)
    has_traps: bool = Field(default=False)


class Cave(TimestampedModel, table=True):
    """Natural cave dungeons."""
    
    id: int | None = Field(default=None, primary_key=True)
    dungeon_id: int = Field(foreign_key="dungeon.id")
    
    # Cave-specific
    depth_levels: int = Field(default=1)
    has_underground_water: bool = Field(default=False)
    creature_lair: bool = Field(default=False)


class Temple(TimestampedModel, table=True):
    """Sacred temple dungeons."""
    
    id: int | None = Field(default=None, primary_key=True)
    dungeon_id: int = Field(foreign_key="dungeon.id")
    
    # Temple-specific
    deity_alignment: str = Field(default="neutral")
    has_altar: bool = Field(default=True)
    blessed_or_cursed: str = Field(default="neutral")


class Tomb(TimestampedModel, table=True):
    """Burial tomb dungeons."""
    
    id: int | None = Field(default=None, primary_key=True)
    dungeon_id: int = Field(foreign_key="dungeon.id")
    
    # Tomb-specific
    undead_present: bool = Field(default=True)
    ancient_curse: bool = Field(default=False)
    burial_goods_value: int = Field(default=0)


# =============================================================================
# SETTLEMENT TABLES (City, Town, Village)
# =============================================================================

class Settlement(TimestampedModel, table=True):
    """Base settlement coordinator table."""
    
    id: int | None = Field(default=None, primary_key=True)
    
    # Core data
    name: str = Field(index=True)
    settlement_type: str = Field(index=True, description="city, town, or village")
    
    # Population
    population_size: int = Field(default=100)
    population_state: str = Field(default="peaceful")
    
    # Location
    region: str = Field(index=True)
    biome_id: int | None = Field(default=None, foreign_key="biome.id")
    biome: Biome | None = Relationship(back_populates="settlements")
    
    # Services (JSON)
    features: str | None = Field(default=None, description="JSON list of features")
    
    # NPCs
    npcs: list["NPC"] = Relationship(back_populates="settlement")


class City(TimestampedModel, table=True):
    """Major city settlements."""
    
    id: int | None = Field(default=None, primary_key=True)
    settlement_id: int = Field(foreign_key="settlement.id")
    
    # City-specific
    has_walls: bool = Field(default=True)
    districts: int = Field(default=3)
    trade_hub: bool = Field(default=True)


class Town(TimestampedModel, table=True):
    """Medium town settlements."""
    
    id: int | None = Field(default=None, primary_key=True)
    settlement_id: int = Field(foreign_key="settlement.id")
    
    # Town-specific
    market_day: str | None = Field(default=None)
    trade_goods: str | None = Field(default=None, description="JSON list")
    
    
class Village(TimestampedModel, table=True):
    """Small village settlements."""
    
    id: int | None = Field(default=None, primary_key=True)
    settlement_id: int = Field(foreign_key="settlement.id")
    
    # Village-specific
    agricultural: bool = Field(default=True)
    elder_name: str | None = Field(default=None)


# =============================================================================
# DWELLING TABLES (Farms/Cabins, Strongholds)
# =============================================================================

class FarmsCabins(TimestampedModel, table=True):
    """Rural dwellings - farms and cabins."""
    
    id: int | None = Field(default=None, primary_key=True)
    
    # Core data
    name: str = Field(index=True)
    dwelling_type: str = Field(default="farm")
    
    # Location
    region: str = Field(index=True)
    isolated: bool = Field(default=True)
    
    # Production
    crops: str | None = Field(default=None, description="JSON list")
    livestock: str | None = Field(default=None, description="JSON list")


class Stronghold(TimestampedModel, table=True):
    """Fortified dwellings - towers, keeps, fortresses."""
    
    id: int | None = Field(default=None, primary_key=True)
    
    # Core data
    name: str = Field(index=True)
    stronghold_type: str = Field(default="tower")
    
    # Location
    region: str = Field(index=True)
    
    # Defense
    fortified: bool = Field(default=True)
    garrison_size: int = Field(default=10)
    lord_name: str | None = Field(default=None)


# =============================================================================
# FACTION TABLES (Cult, Militia, Syndicate)
# =============================================================================

class Cult(TimestampedModel, table=True):
    """Dark cult factions."""
    
    id: int | None = Field(default=None, primary_key=True)
    
    # Core data
    name: str = Field(index=True)
    deity_or_entity: str | None = Field(default=None)
    
    # Philosophy
    philosophy: str = Field(default="dark")
    corruption_level: int = Field(default=2, ge=0, le=3)
    
    # Activities
    ritual_type: str | None = Field(default=None)
    sacrifice_required: bool = Field(default=False)


class Militia(TimestampedModel, table=True):
    """Military/guard factions."""
    
    id: int | None = Field(default=None, primary_key=True)
    
    # Core data
    name: str = Field(index=True)
    
    # Organization
    commander_name: str | None = Field(default=None)
    troop_size: int = Field(default=20)
    
    # Alignment
    philosophy: str = Field(default="neutral")
    faction_state: str = Field(default="defensive")


class Syndicate(TimestampedModel, table=True):
    """Criminal syndicate factions."""
    
    id: int | None = Field(default=None, primary_key=True)
    
    # Core data
    name: str = Field(index=True)
    
    # Organization
    boss_name: str | None = Field(default=None)
    operation_type: str | None = Field(default=None)
    
    # Activities
    territory: str | None = Field(default=None)
    rival_faction: str | None = Field(default=None)


# =============================================================================
# NPC TABLE
# =============================================================================

class NPC(TimestampedModel, table=True):
    """Non-player characters with psychology."""
    
    id: int | None = Field(default=None, primary_key=True)
    
    # Core data
    name: str = Field(index=True)
    level: int = Field(default=1)
    class_name: str | None = Field(default=None)
    
    # Companion system
    companion_type: str | None = Field(default=None)
    can_be_companion: bool = Field(default=False)
    
    # Psychology
    baseline_trauma: int = Field(default=0, ge=0, le=10)
    current_stress: int = Field(default=0, ge=0, le=10)
    philosophy_lean: str = Field(default="neutral")
    
    # Location
    settlement_id: int | None = Field(default=None, foreign_key="settlement.id")
    settlement: Settlement | None = Relationship(back_populates="npcs")
    
    inn_id: int | None = Field(default=None, foreign_key="inn.id")
    inn: Inn | None = Relationship(back_populates="npcs")
    
    # Inventory
    has_sentimental_items: bool = Field(default=False)
    treasures: list["Treasure"] = Relationship(back_populates="owner")


# =============================================================================
# TREASURE TABLE
# =============================================================================

class Treasure(TimestampedModel, table=True):
    """Items and treasures with sentimental value."""
    
    id: int | None = Field(default=None, primary_key=True)
    
    # Core data
    name: str = Field(index=True)
    treasure_value: int = Field(default=0)
    
    # Sentimental system
    is_sentimental: bool = Field(default=False)
    emotional_weight: float = Field(default=0.0)
    forge_reagent_type: str | None = Field(default=None)
    
    # Material progression
    material_tier: str = Field(default="wood")
    
    # Ownership
    owner_id: int | None = Field(default=None, foreign_key="npc.id")
    owner: NPC | None = Relationship(back_populates="treasures")


# =============================================================================
# FALLBACK TABLES FOR UNCLASSIFIED CONTENT
# =============================================================================

class HTMLEntity(TimestampedModel, table=True):
    """Fallback for HTML content that couldn't be classified."""
    
    id: int | None = Field(default=None, primary_key=True)
    html_content: str = Field()
    
    # ML analysis results (JSON)
    ml_features: str | None = Field(default=None)
    pattern_scores: str | None = Field(default=None)
    cluster_id: int | None = Field(default=None)
    is_anomaly: bool = Field(default=False)


class JSONEntity(TimestampedModel, table=True):
    """Fallback for JSON content that couldn't be classified."""
    
    id: int | None = Field(default=None, primary_key=True)
    json_data: str = Field()
    
    # JSON metadata
    json_type: str | None = Field(default=None)
    json_keys: str | None = Field(default=None, description="JSON list of top-level keys")
