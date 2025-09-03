"""Settlement-specific entity models and clusters.

Following .clinerules standards with modern Python type hints.
"""

from __future__ import annotations

from typing import Any

from pydantic import BaseModel, Field

from generator.analysis.models.clusters import BaseEntitiesCluster


# ---- Settlement entity models ----

class SettlementEstablishment(BaseModel):
    """Settlement establishment entity model."""
    entity_uuid: str = Field(..., description="UUID from filename")
    settlement_name: str | None = Field(None, description="Name of the settlement")
    establishment_type: str | None = Field(None, description="Type of establishment (shop, tavern, etc)")
    hex_key: str | None = Field(None, description="Hex coordinate if present")
    region_uuid: str | None = Field(None, description="Parent region UUID")
    faction_uuids: list[str] = Field(default_factory=list)
    npc_uuids: list[str] = Field(default_factory=list)
    inventory: dict[str, Any] | None = Field(None, description="Shop inventory if applicable")

    @classmethod
    def extract_uuid_connections(cls) -> dict[str, str]:
        """Extract UUID connection types."""
        return {
            "entity_uuid": "settlement_establishment",
            "region_uuid": "region",
            "faction_uuids": "faction",
            "npc_uuids": "npc",
        }


# ---- Settlement cluster specialization ----

class RawSettlementEntities(BaseEntitiesCluster):
    """Settlement-specific cluster for AI model generation."""
    
    def inventory_schema(self) -> dict[str, Any]:
        """Schema for settlement inventory extraction."""
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
        """Prompt for settlement analysis."""
        return (
            "Analyze the supplied HTML/JSON snippets related to *settlements*.\n"
            "Return a JSON object with 'entities' describing normalized fields and UUID connections.\n"
            "Look for establishments (shops, taverns), NPCs, faction memberships, and economic data.\n"
            "Note that faction info may be in spoiler tags.\n"
            "Do not output code; output JSON only."
        )

    def model_template(self) -> str:
        """Template for settlement model generation."""
        return '''# Generated Pydantic models for settlements
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
