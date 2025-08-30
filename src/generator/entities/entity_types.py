"""
HBF Extractors Types - SQLModel enums and type aliases following db package patterns.

Modern enums with auto() and factory methods for HBF content classification.
Uses comprehensive analysis results from hbf_analysis/ for type definitions.
"""

import re
from enum import Enum, IntEnum, StrEnum, auto
from typing import TypeAlias

# Type aliases for HBF content (using existing analysis results)
HBFEntityId: TypeAlias = str  # 8-character HBF UUID format
HexCoordinate: TypeAlias = str  # BASE, N1, E2S3, etc.
RegionName: TypeAlias = str  # Fearless Wilds, Vicious Crags, Ragthorn Woods, Heartseeker Forest
WorldName: TypeAlias = str  # The Lands of Vo'il
FeatureName: TypeAlias = str  # Watchtower, Village of Harad, etc.


# Target content categories discovered in analysis
class BiomeType(str, Enum):
    """4 biome types discovered in analysis."""
    JUNGLE = auto()      # Fearless Wilds
    MOUNTAINS = auto()   # Vicious Crags  
    FOREST = auto()      # Ragthorn Woods
    DEEP_FOREST = auto() # Heartseeker Forest
    
    @classmethod
    def from_region(cls, region: str) -> "BiomeType":
        """Convert region name to biome type."""
        region_lower = region.casefold()
        match region_lower:
            case x if "fearless wilds" in x:
                return cls.JUNGLE
            case x if "vicious crags" in x:
                return cls.MOUNTAINS
            case x if "ragthorn woods" in x:
                return cls.FOREST
            case x if "heartseeker forest" in x:
                return cls.DEEP_FOREST
            case _:
                return cls.FOREST


class DungeonType(str, Enum):
    """3 dungeon subtypes discovered in analysis."""
    CAVE = auto()     # Natural caves and lairs
    TEMPLE = auto()   # Altars and sacred sites
    TOMB = auto()     # Graveyards and dead remains
    
    @classmethod
    def from_feature(cls, feature_name: str) -> "DungeonType":
        """Classify dungeon type from feature."""
        feature_lower = feature_name.casefold()
        match feature_lower:
            case x if any(word in x for word in ["altar", "sacrificial", "temple"]):
                return cls.TEMPLE
            case x if any(word in x for word in ["graveyard", "dead", "remains", "tomb"]):
                return cls.TOMB
            case _:
                return cls.CAVE


class SettlementType(str, Enum):
    """3 settlement subtypes discovered in analysis."""
    VILLAGE = auto()  # Village of Harad, abandoned villages
    TOWN = auto()     # Caravan camps (temporary settlements)
    CITY = auto()     # Citadel of Hoviir
    
    @classmethod
    def from_feature(cls, feature_name: str) -> "SettlementType":
        """Classify settlement type from feature."""
        feature_lower = feature_name.casefold()
        match feature_lower:
            case x if "village" in x:
                return cls.VILLAGE
            case x if any(word in x for word in ["citadel", "city"]):
                return cls.CITY
            case x if "caravan" in x:
                return cls.TOWN
            case _:
                return cls.VILLAGE


class DwellingType(str, Enum):
    """2 dwelling subtypes discovered in analysis."""
    FARM_CABIN = auto()   # Farms and cabins
    STRONGHOLD = auto()   # Laboratories, schools, watchtowers
    
    @classmethod
    def from_feature(cls, feature_name: str) -> "DwellingType":
        """Classify dwelling type from feature."""
        feature_lower = feature_name.casefold()
        match feature_lower:
            case x if any(word in x for word in ["laboratory", "school", "tower", "stronghold"]):
                return cls.STRONGHOLD
            case _:
                return cls.FARM_CABIN


class FactionType(str, Enum):
    """3 faction subtypes discovered in analysis."""
    CULT = auto()      # Sacrificial grounds, ritual sites
    MILITIA = auto()   # Guards, veterans, organized fighters  
    SYNDICATE = auto() # Bandits, thieves, criminal organizations
    
    @classmethod
    def from_npcs_or_features(cls, text: str) -> "FactionType":
        """Classify faction type from NPCs or features."""
        text_lower = text.casefold()
        match text_lower:
            case x if any(word in x for word in ["sacrificial", "ritual", "cult"]):
                return cls.CULT
            case x if any(word in x for word in ["guard", "veteran", "militia", "soldier"]):
                return cls.MILITIA
            case x if any(word in x for word in ["bandit", "thief", "syndicate"]):
                return cls.SYNDICATE
            case _:
                return cls.SYNDICATE


class CreatureTier(str, Enum):
    """Creature tiers based on CR analysis."""
    COMMON = auto()     # CR 0-1
    UNCOMMON = auto()   # CR 2-4
    BOSS = auto()       # CR 5-10
    LEGENDARY = auto()  # CR 10+
    
    @classmethod
    def from_cr(cls, cr_value: float) -> "CreatureTier":
        """Determine tier from challenge rating."""
        match cr_value:
            case x if x <= 1:
                return cls.COMMON
            case x if x <= 4:
                return cls.UNCOMMON
            case x if x <= 10:
                return cls.BOSS
            case _:
                return cls.LEGENDARY


class CoordinateType(str, Enum):
    """Hex coordinate complexity types."""
    BASE = auto()              # BASE
    SIMPLE_DIRECTIONAL = auto() # N1, S1, E1, W1
    COMPLEX_DIRECTIONAL = auto() # E1N2, W1S1, E6S43
    UNKNOWN = auto()
    
    @classmethod
    def from_coordinate(cls, coord: str) -> "CoordinateType":
        """Classify coordinate complexity."""
        if coord == "BASE":
            return cls.BASE
        elif re.match(r'^[NSEW]\d+$', coord):
            return cls.SIMPLE_DIRECTIONAL
        elif re.match(r'^[NSEW]\d+[NSEW]\d+$', coord):
            return cls.COMPLEX_DIRECTIONAL
        else:
            return cls.UNKNOWN


class ContentFlag(str, Enum):
    """Content type flags for hex tiles."""
    HAS_ENCOUNTERS = auto()
    HAS_WEATHER = auto()
    HAS_NPCS = auto()
    HAS_TREASURE = auto()
    HAS_SHOPS = auto()
    HAS_RUMORS = auto()
    HAS_BOSS_CREATURE = auto()
    HAS_FACTION_PRESENCE = auto()


# Entity-specific enums moved from db/types.py

class DreadLevel(IntEnum):
    """Dread level classifications for horror progression."""
    PEACEFUL = 0
    UNSETTLING = 1
    TERRIFYING = 2
    HORRIFYING = 3
    VOID = 4
    
    @classmethod
    def from_corruption(cls, corruption_level: int) -> "DreadLevel":
        """Map corruption level to dread level."""
        match corruption_level:
            case 0:
                return cls.PEACEFUL
            case 1:
                return cls.UNSETTLING
            case 2:
                return cls.TERRIFYING
            case 3:
                return cls.HORRIFYING
            case _:
                return cls.VOID


class CorruptionStage(StrEnum):
    """Corruption stage classifications for world state."""
    CLEAN = auto()
    WITHERED = auto()
    SCORCHED = auto()
    VOID = auto()
    
    @classmethod
    def from_dread(cls, dread_level: DreadLevel) -> "CorruptionStage":
        """Map dread level to corruption stage."""
        match dread_level:
            case DreadLevel.PEACEFUL:
                return cls.CLEAN
            case DreadLevel.UNSETTLING:
                return cls.WITHERED
            case DreadLevel.TERRIFYING | DreadLevel.HORRIFYING:
                return cls.SCORCHED
            case DreadLevel.VOID:
                return cls.VOID
            case _:
                return cls.CLEAN


class PhilosophyPath(StrEnum):
    """Philosophy path alignments for Dragon's Labyrinth."""
    STRENGTH = auto()
    HARMONY = auto()
    LIGHT = auto()
    DARK = auto()
    NEUTRAL = auto()
    
    @classmethod
    def from_content(cls, content: str) -> "PhilosophyPath":
        """Detect philosophy from content."""
        content_lower = content.lower()
        match content_lower:
            case x if "strength" in x or "power" in x:
                return cls.STRENGTH
            case x if "harmony" in x or "peace" in x:
                return cls.HARMONY
            case x if "light" in x or "holy" in x:
                return cls.LIGHT
            case x if "dark" in x or "shadow" in x:
                return cls.DARK
            case _:
                return cls.NEUTRAL


class EntityTableType(StrEnum):
    """All Dragon's Labyrinth entity table types."""
    BIOME = auto()
    MONSTER = auto()
    INN = auto()
    CAVE = auto()
    TEMPLE = auto()
    TOMB = auto()
    CITY = auto()
    TOWN = auto()
    VILLAGE = auto()
    FARMS_CABINS = auto()
    STRONGHOLD = auto()
    CULT = auto()
    MILITIA = auto()
    SYNDICATE = auto()
    HTML_ENTITY = auto()
    JSON_ENTITY = auto()
    UNKNOWN = auto()

# Alias for backwards compatibility
EntityType = EntityTableType
