"""Dungeon-specific entity models and clusters.

Following .clinerules standards with modern Python type hints.
"""

from __future__ import annotations

from typing import Any

from pydantic import BaseModel, Field

from generator.analysis.models.clusters import BaseEntitiesCluster


# ---- Dungeon entity models ----

class DungeonArea(BaseModel):
    """Dungeon area entity model."""
    entity_uuid: str = Field(..., description="UUID from filename")
    dungeon_name: str | None = Field(None, description="Name of the dungeon")
    area_number: int | None = Field(None, description="Area number within dungeon")
    area_description: str | None = Field(None, description="Area description")
    entrance_hex: str | None = Field(None, description="Hex coordinate of dungeon entrance")
    connected_areas: list[int] = Field(default_factory=list, description="Connected area numbers")
    monsters: list[dict[str, Any]] = Field(default_factory=list, description="Monster encounters")
    treasure: dict[str, Any] | None = Field(None, description="Treasure data")
    traps: list[dict[str, Any]] = Field(default_factory=list, description="Trap data")

    @classmethod
    def extract_uuid_connections(cls) -> dict[str, str]:
        """Extract UUID connection types."""
        return {
            "entity_uuid": "dungeon_area",
        }


# ---- Dungeon cluster specialization ----

class RawDungeonEntities(BaseEntitiesCluster):
    """Dungeon-specific cluster for AI model generation."""
    
    def inventory_schema(self) -> dict[str, Any]:
        """Schema for dungeon inventory extraction."""
        return {
            "type": "object",
            "properties": {
                "entities": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "properties": {
                            "name": {"type": "string"},
                            "description": {"type": "string"},
                            "fields": {
                                "type": "array",
                                "items": {
                                    "type": "object",
                                    "properties": {
                                        "name": {"type": "string"},
                                        "type": {"type": "string"},
                                        "required": {"type": "boolean"},
                                        "description": {"type": "string"},
                                        "is_uuid": {"type": "boolean"},
                                        "is_connection": {"type": "boolean"},
                                    },
                                    "required": ["name", "type", "required"],
                                    "additionalProperties": False,
                                },
                            },
                        },
                        "required": ["name", "fields"],
                        "additionalProperties": False,
                    },
                },
                "connections": {"type": "object", "additionalProperties": {"type": "string"}},
                "notes": {"type": "array", "items": {"type": "string"}},
            },
            "required": ["entities"],
            "additionalProperties": False,
        }

    def analysis_prompt(self) -> str:
        """Prompt for dungeon analysis."""
        return (
            "Analyze the supplied HTML/JSON snippets related to *dungeons*.\n"
            "Detect area/monster/treasure relationships, but only emit field/connection metadata.\n"
            "Look for area layouts, encounter data, treasure distribution.\n"
            "Focus on monster stat blocks, environmental descriptions, area connections.\n"
            "Return JSON only."
        )

    def model_template(self) -> str:
        """Template for dungeon model generation."""
        return '''# Generated Pydantic models for dungeons
from __future__ import annotations

from typing import Any
from pydantic import BaseModel, Field

{% for e in inventory.entities %}
class {{ e.name }}(BaseModel):
    """{{ e.description or e.name }}"""
{% for f in e.fields %}
    {{ f.name }}: {{ f.type }}{% if not f.required %} | None{% endif %} = {% if f.required %}Field(...{% else %}Field(None{% endif %}, description="{{ f.description or '' }}")
{% endfor %}

    @classmethod
    def extract_uuid_connections(cls) -> dict[str, str]:
        """Extract UUID connection types."""
        return {
{% for f in e.fields if f.is_uuid or f.is_connection %}
            '{{ f.name }}': '{{ inventory.connections.get(f.name, 'unknown') }}',
{% endfor %}
        }

{% endfor %}
'''
