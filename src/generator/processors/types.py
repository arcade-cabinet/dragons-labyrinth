"""
Processor-specific type definitions for HBF data extraction and analysis.

Contains types needed for processing real HBF data, following .clinerules
standards with modern Python typing. Focus on ANALYSIS not INVENTION.
"""

from __future__ import annotations

from enum import Enum
from typing import TypeAlias

# Type aliases for better semantic meaning
EntityId: TypeAlias = str
HexCoordinate: TypeAlias = tuple[int, int]


class ProcessorType(str, Enum):
    """Processor type identifiers."""
    REGIONS = "regions"
    SETTLEMENTS = "settlements"
    DUNGEONS = "dungeons"
    FACTIONS = "factions"
    META = "meta"
    JSON_CITIES = "json_cities"
    JSON_DUNGEONS = "json_dungeons"
    JSON_HAZARDS = "json_hazards"
    JSON_MAP = "json_map"


class BiomeType(str, Enum):
    """
    Biome types from constants.py BIOMES - the 7 actual biomes in the game.
    NO CUSTOM BIOME MAPPING - use only these canonical biomes.
    """
    DESERT = "Desert"
    FOREST = "Forest"
    JUNGLE = "Jungle"
    MOUNTAINS = "Mountains"
    PLAINS = "Plains"
    SWAMPS = "Swamps"
    TUNDRA = "Tundra"


class SettlementType(str, Enum):
    """Settlement types found in HBF data analysis."""
    VILLAGE = "village"
    TOWN = "town"
    CITY = "city"


class DungeonType(str, Enum):
    """Dungeon types derived from HBF entity analysis."""
    CRYPT = "crypt"
    CAVE = "cave"
    TEMPLE = "temple"
    LAIR = "lair"
    PIT = "pit"
    UNKNOWN = "unknown"


class ThreatLevel(str, Enum):
    """
    Threat levels based on ACTUAL enemy counts from HBF data.
    NOT INVENTED - calculated from real enemy entities found.
    """
    MINIMAL = "minimal"      # 0-2 enemies
    LOW = "low"             # 3-5 enemies
    MODERATE = "moderate"   # 6-10 enemies
    HIGH = "high"           # 11-15 enemies
    EXTREME = "extreme"     # 16+ enemies


class ComplexityLevel(str, Enum):
    """
    Complexity levels based on ACTUAL room mentions from HBF data.
    NOT INVENTED - calculated from real structure analysis.
    """
    SIMPLE = "simple"       # 1-2 rooms mentioned
    MODERATE = "moderate"   # 3-5 rooms mentioned
    COMPLEX = "complex"     # 6-10 rooms mentioned
    ELABORATE = "elaborate" # 11+ rooms mentioned


class EconomicLevel(str, Enum):
    """
    Economic activity levels based on ACTUAL service/trade mentions.
    NOT INVENTED - counted from real establishments found.
    """
    NONE = "none"           # 0 economic mentions
    MINIMAL = "minimal"     # 1-2 economic mentions
    LOW = "low"             # 3-5 economic mentions
    MODERATE = "moderate"   # 6-10 economic mentions
    HIGH = "high"           # 11-15 economic mentions
    VERY_HIGH = "very_high" # 16+ economic mentions


class ServiceType(str, Enum):
    """Service types found in settlement analysis."""
    COMMERCE = "commerce"
    LODGING = "lodging"
    CRAFTING = "crafting"
    MEDICAL = "medical"
    RELIGIOUS = "religious"
    DEFENSE = "defense"
    GOVERNMENT = "government"
    LEARNING = "learning"
    UNKNOWN = "unknown"


class FactionType(str, Enum):
    """Faction types derived from HBF name pattern analysis."""
    CULT = "cult"
    MILITIA = "militia"
    SYNDICATE = "syndicate"
    GUILD = "guild"
    UNKNOWN = "unknown"


class HostilityLevel(str, Enum):
    """
    Hostility levels based on ACTUAL faction name analysis.
    NOT INVENTED - derived from faction naming patterns.
    """
    FRIENDLY = "friendly"
    NEUTRAL = "neutral"
    AGGRESSIVE = "aggressive"
    HOSTILE = "hostile"
    UNKNOWN = "unknown"
