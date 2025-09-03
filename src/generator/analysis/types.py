"""
Primitive types and enums for entity processing.

Following .clinerules standards:
- Modern Python types (dict/list not Dict/List)
- Enums with auto() for entity classifications
- Type aliases for clarity
"""

from __future__ import annotations

from enum import Enum, auto
from typing import TypeAlias


# Type aliases
EntityId: TypeAlias = str
HexCoordinate: TypeAlias = tuple[int, int]  # (q, r) axial coordinates
RegionName: TypeAlias = str
SettlementName: TypeAlias = str
DungeonName: TypeAlias = str


class SettlementScale(str, Enum):
    """Settlement scale classifications from analysis."""
    VILLAGE = "village"
    TOWN = "town" 
    CITY = "city"
    METROPOLIS = "metropolis"


class ServiceType(str, Enum):
    """Settlement service types from JSON analysis."""
    COMMERCE = "commerce"
    LODGING = "lodging"
    CRAFTING = "crafting"
    MEDICAL = "medical"
    RELIGIOUS = "religious"
    DEFENSE = "defense"
    GOVERNMENT = "government"
    LEARNING = "learning"


class DungeonType(str, Enum):
    """Dungeon type classifications."""
    CRYPT = "crypt"
    CAVE = "cave"
    TEMPLE = "temple"
    LAIR = "lair"
    HIDEOUT = "hideout"
    PIT = "pit"
    GENERIC = "generic"


class ComplexityLevel(str, Enum):
    """Structural complexity levels from JSON analysis."""
    SIMPLE = "simple"
    MODERATE = "moderate"
    COMPLEX = "complex"
    VERY_COMPLEX = "very_complex"


class NavigationDifficulty(str, Enum):
    """Navigation difficulty from JSON dungeon analysis."""
    TRIVIAL = "trivial"
    EASY = "easy"
    MODERATE = "moderate"
    HARD = "hard"


class ExplorationDifficulty(str, Enum):
    """Exploration difficulty with enhanced JSON patterns."""
    TRIVIAL = "trivial"
    EASY = "easy"
    MODERATE = "moderate"
    HARD = "hard"
    EXTREME = "extreme"
    NIGHTMARE = "nightmare"  # From JSON enhancement


class BiomeType(str, Enum):
    """Dragon's Labyrinth biome types from JSON map analysis."""
    WET_MEADOW = "wet_meadow"
    ASHEN_FOREST = "ashen_forest"
    FLOODED_VILLAGE = "flooded_village"
    BLACK_SWAMP = "black_swamp"
    FUNGAL_CATHEDRAL = "fungal_cathedral"
    RUST_PLAINS = "rust_plains"
    FAMINE_FIELDS = "famine_fields"
    BONE_FOREST = "bone_forest"
    DRAGON_SCAR = "dragon_scar"
    ABYSSAL_CHASM = "abyssal_chasm"
    # Legacy biomes
    FOREST = "forest"
    DESERT = "desert"
    MOUNTAIN = "mountain"
    PLAINS = "plains"
    SWAMP = "swamp"
    TUNDRA = "tundra"


class CorruptionLevel(int, Enum):
    """Corruption levels matching Dragon's Labyrinth progression bands."""
    PEACE = 0      # Band 1-20
    UNEASE = 1     # Band 21-40  
    DREAD = 2      # Band 41-60
    TERROR = 3     # Band 61-120
    HORROR = 4     # Band 121-180
    DRAGON = 5     # Dragon-touched regions


class ProcessorType(str, Enum):
    """Entity processor types."""
    SETTLEMENTS = "settlements"
    DUNGEONS = "dungeons"
    REGIONS = "regions"
    FACTIONS = "factions"
    BIOMES = "biomes"
