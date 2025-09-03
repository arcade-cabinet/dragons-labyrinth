"""Region-specific entity models and clusters.

Following .clinerules standards with modern Python type hints.
"""

from __future__ import annotations

from typing import Any

from pydantic import BaseModel, Field

from generator.analysis.models.clusters import BaseEntitiesCluster


# ---- Region entity models ----

class RegionHexTile(BaseModel):
    """Region hex tile entity model."""
    entity_uuid: str = Field(..., description="UUID from filename")
    hex_key: str | None = Field(None, description="Canonical hex key like 'W2S51'")
    map: dict[str, Any] | None = Field(None, description="{'x': float, 'y': float, 'hex_id': str | None}")
    region_uuid: str | None = Field(None, description="UUID of region if distinct")
    settlement_uuids: list[str] = Field(default_factory=list)
    dungeon_uuids: list[str] = Field(default_factory=list)
    faction_uuids: list[str] = Field(default_factory=list)

    @classmethod
    def extract_uuid_connections(cls) -> dict[str, str]:
        """Extract UUID connection types."""
        return {
            "entity_uuid": "region_hex_tile",
            "region_uuid": "region",
            "settlement_uuids": "settlement",
            "dungeon_uuids": "dungeon",
            "faction_uuids": "faction",
        }


# ---- Region cluster specialization ----

class RawRegionEntities(BaseEntitiesCluster):
    """Region-specific cluster for AI model generation."""
    
    def inventory_schema(self) -> dict[str, Any]:
        """Schema for region inventory extraction."""
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
        """Prompt for region analysis."""
        return (
            "Analyze the supplied HTML/JSON snippets related to *regions*.\n"
            "Return a JSON object with an 'entities' array describing data models.\n"
            "Focus on names, descriptions, field names/types, and which fields are UUIDs or connections.\n"
            "Look for hex coordinates, map positions, settlements, dungeons, and faction references.\n"
            "If uncertain, omit rather than invent."
        )

    def model_template(self) -> str:
        """Template for region model generation."""
        return '''# Generated Pydantic models for regions
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
