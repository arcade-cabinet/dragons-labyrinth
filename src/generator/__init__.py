"""
Generator Package

Modern Python generator system with simple run() interfaces.
Follows .clinerules architectural patterns with clean subpackage organization.
"""

# Import run() functions from all subpackages
from .seeds import run as seeds_run
from .psychology import run as psychology_run
from .world import run as world_run
from .maps import run as maps_run
from .encounters import run as encounters_run
from .sprites import run as sprites_run
from .assets import run as assets_run

# Import backwards compatibility functions
from .seeds import get_emotional_seeds_data, get_horror_progression_data
from .psychology import get_all_companion_profiles
from .world import get_all_regions, get_campaign_state, get_world_state
from .maps import get_all_hex_tiles, get_map_regions, calculate_hex_distance
from .encounters import get_all_encounters, get_encounters_by_region, get_encounters_by_dread_level
from .sprites import get_all_characters, get_companions, get_character_roster
from .assets import get_all_assets, get_assets_by_category, get_openai_generation_metrics

__all__ = [
    # Subpackage run functions
    "seeds_run",
    "psychology_run", 
    "world_run",
    "maps_run",
    "encounters_run",
    "sprites_run",
    "assets_run",
    
    # Backwards compatibility functions
    "get_emotional_seeds_data",
    "get_horror_progression_data", 
    "get_all_companion_profiles",
    "get_all_regions",
    "get_campaign_state",
    "get_world_state",
    "get_all_hex_tiles",
    "get_map_regions", 
    "calculate_hex_distance",
    "get_all_encounters",
    "get_encounters_by_region",
    "get_encounters_by_dread_level",
    "get_all_characters",
    "get_companions",
    "get_character_roster",
    "get_all_assets",
    "get_assets_by_category",
    "get_openai_generation_metrics"
]
