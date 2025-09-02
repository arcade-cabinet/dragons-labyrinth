"""Base value objects and shared abstractions for analysis package.

Following .clinerules standards with modern Python type hints.
"""

from __future__ import annotations

from enum import Enum
from pydantic import BaseModel, Field


# ---- Value objects ----

HexKey = str  # Canonical key like "W2S51"


class MapCoord(BaseModel):
    """Map coordinate with optional hex ID."""
    x: float | None = Field(None)
    y: float | None = Field(None)
    hex_id: str | None = Field(None, description="Map hex id when present")


class EdgeType(str, Enum):
    """Edge type enumeration for entity relationships."""
    settlement_in_hex = "settlement_in_hex"
    dungeon_in_hex = "dungeon_in_hex"
    area_connects_to_area = "area_connects_to_area"
    faction_controls_region = "faction_controls_region"
    faction_controls_settlement = "faction_controls_settlement"


# ---- Inventory types for structured outputs ----

class FieldSpec(BaseModel):
    """Field specification for entity model generation."""
    name: str
    type: str
    required: bool
    description: str | None = None
    is_uuid: bool | None = None
    is_connection: bool | None = None
    edge_type: EdgeType | None = None


class EntitySpec(BaseModel):
    """Entity specification for model generation."""
    name: str
    description: str | None = None
    fields: list[FieldSpec] = Field(default_factory=list)


class Inventory(BaseModel):
    """Inventory of entities and connections for model generation."""
    entities: list[EntitySpec] = Field(default_factory=list)
    connections: dict[str, str] = Field(default_factory=dict)  # field_name -> target_entity_kind
    notes: list[str] = Field(default_factory=list)
