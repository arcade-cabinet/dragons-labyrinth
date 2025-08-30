# generator/entities/training/registry.py
from __future__ import annotations

from generator.entities.training import regions as _regions
from generator.entities.training import factions as _factions
from generator.entities.training import dungeons as _dungeons
from generator.entities.training import settlements as _settlements
from generator.entities.training import patterns as _patterns
# If you keep creatures/biomes elsewhere, import them similarly.

def _norm(items: list[str] | set[str]) -> set[str]:
    return {i.strip().lower() for i in items if i and isinstance(i, str)}

def list_regions() -> set[str]:
    return _norm(getattr(_regions, "REGIONS", []) or getattr(_regions, "ALL", []))

def list_biomes() -> set[str]:
    # If you store biomes in patterns or regions, adapt this accordingly.
    # Many projects keep BIOMES in patterns; try both:
    biomes = set()
    if hasattr(_patterns, "BIOMES"):
        biomes |= _norm(_patterns.BIOMES)
    if hasattr(_regions, "BIOMES"):
        biomes |= _norm(_regions.BIOMES)
    return biomes

def list_factions() -> set[str]:
    return _norm(getattr(_factions, "FACTIONS", []) or getattr(_factions, "ALL", []))

def list_dungeons() -> set[str]:
    return _norm(getattr(_dungeons, "DUNGEONS", []) or getattr(_dungeons, "ALL", []))

def list_settlements() -> set[str]:
    return _norm(getattr(_settlements, "SETTLEMENTS", []) or getattr(_settlements, "ALL", []))

def list_patterns() -> set[str]:
    # catch-all generic gameplay/world patterns
    return _norm(getattr(_patterns, "ALL", []) or [])