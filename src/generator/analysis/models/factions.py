"""Faction-specific entity models and clusters.

Following .clinerules standards with modern Python type hints.
"""

from __future__ import annotations

from typing import Any

from pydantic import BaseModel, Field

from generator.analysis.models.clusters import BaseEntitiesCluster


# ---- Faction entity models ----

class FactionEntity(BaseModel):
    """Faction entity model."""
    entity_uuid: str = Field(..., description="UUID from filename")
    faction_name: str | None = Field(None, description="Name of the faction")
    stronghold_hex: str | None = Field(None, description="Hex coordinate of stronghold")
    controlled_regions: list[str] = Field(default_factory=list, description="Region UUIDs under control")
    controlled_settlements: list[str] = Field(default_factory=list, description="Settlement UUIDs under control")
    member_uuids: list[str] = Field(default_factory=list, description="Member NPC UUIDs")
    allied_factions: list[str] = Field(default_factory=list, description="Allied faction UUIDs")
    enemy_factions: list[str] = Field(default_factory=list, description="Enemy faction UUIDs")

    @classmethod
    def extract_uuid_connections(cls) -> dict[str, str]:
        """Extract UUID connection types."""
        return {
            "entity_uuid": "faction",
            "controlled_regions": "region",
            "controlled_settlements": "settlement",
            "member_uuids": "npc",
            "allied_factions": "faction",
            "enemy_factions": "faction",
        }


# ---- Faction cluster specialization ----

class RawFactionEntities(BaseEntitiesCluster):
    """Faction-specific cluster for AI model generation."""
    
    def inventory_schema(self) -> dict[str, Any]:
        """Schema for faction inventory extraction."""
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
        """Prompt for faction analysis."""
        return (
            "Analyze the supplied HTML/JSON snippets related to *factions*.\n"
            "Identify UUID fields that represent links to regions/settlements/dungeons/etc.\n"
            "Look for political relationships, territorial control, member information.\n"
            "Focus on alliance/enemy relationships, stronghold locations, influence zones.\n"
            "Return JSON only."
        )

    def model_template(self) -> str:
        """Template for faction model generation."""
        return '''# Generated Pydantic models for factions
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
