"""
Entity SQLModel definitions for Dragon's Labyrinth database.

Contains EntityRecord and related models with comprehensive validation.
"""

from __future__ import annotations

import re
from datetime import datetime
from typing import Any

from pydantic import validator, root_validator
from sqlmodel import SQLModel, Field

from dragons_labyrinth.db.types import (
    EntityId, EntityType, DreadLevel, CorruptionStage, PipelineStage,
    PhilosophyPath, ActContext, RegionContext, ContentHash, EntityData, TagList
)


class EntityRecord(SQLModel, table=True):
    """Entities table: All pipeline data (tiles, NPCs, world data, everything)"""
    __tablename__ = "entities"
    
    # Primary identification with type-safe constraints
    entity_id: EntityId = Field(primary_key=True, description="Unique entity identifier")
    entity_type: EntityType = Field(description="Type of entity (biome_spec, npc_spec, etc.)")
    entity_name: str | None = Field(default=None, min_length=1, max_length=200, description="Human-readable entity name")
    
    # Content storage with validation
    entity_data: EntityData = Field(description="Complete entity data as JSON")
    data_checksum: ContentHash | None = Field(default=None, description="SHA256 checksum of entity_data for idempotency")
    
    # Pipeline context with validation
    source_pipeline: str = Field(min_length=1, description="Pipeline that created this entity")
    dependent_entities: TagList | None = Field(default=None, description="List of entity IDs this depends on")
    
    # Categorization with validation
    category: str | None = Field(default=None, description="High-level category (world, tiles, characters, encounters)")
    subcategory: str | None = Field(default=None, description="Specific subcategory within type")
    tags: TagList | None = Field(default=None, description="Searchable tags for entity discovery")
    
    # Game context with enum validation
    region_context: RegionContext | None = Field(default=None, description="Region this entity belongs to")
    dread_level: DreadLevel | None = Field(default=None, description="Associated dread level (0-4)")
    philosophy_path: PhilosophyPath | None = Field(default=None, description="Associated philosophy path")
    act_context: ActContext | None = Field(default=None, description="Game act context (1-3)")
    corruption_stage: CorruptionStage | None = Field(default=None, description="Corruption stage (clean, withered, scorched, void)")
    
    # Lifecycle
    created_at: datetime = Field(default_factory=datetime.now, description="Creation timestamp")
    updated_at: datetime = Field(default_factory=datetime.now, description="Last update")
    version: int = Field(default=1, ge=1, description="Entity version for updates")
    
    # Status
    validated: bool = Field(default=True, description="Whether entity passed validation")
    active: bool = Field(default=True, description="Whether entity is active")
    
    @validator("entity_type", pre=True)
    def validate_entity_type(cls, v: str | EntityType) -> EntityType:
        """Convert string to EntityType enum"""
        if isinstance(v, EntityType):
            return v
        return EntityType.from_string(v)
    
    @validator("dread_level", pre=True)
    def validate_dread_level(cls, v: int | DreadLevel | None) -> DreadLevel | None:
        """Convert int to DreadLevel enum"""
        if v is None:
            return v
        if isinstance(v, DreadLevel):
            return v
        return DreadLevel.from_value(v)
    
    @validator("philosophy_path", pre=True)
    def validate_philosophy_path(cls, v: str | PhilosophyPath | None) -> PhilosophyPath | None:
        """Convert string to PhilosophyPath enum"""
        if v is None:
            return v
        if isinstance(v, PhilosophyPath):
            return v
        return PhilosophyPath.from_string(v)
    
    @validator("act_context", pre=True)
    def validate_act_context(cls, v: int | ActContext | None) -> ActContext | None:
        """Convert int to ActContext enum"""
        if v is None:
            return v
        if isinstance(v, ActContext):
            return v
        return ActContext.from_value(v)
    
    @validator("corruption_stage", pre=True)
    def validate_corruption_stage(cls, v: str | CorruptionStage | None) -> CorruptionStage | None:
        """Convert string to CorruptionStage enum"""
        if v is None:
            return v
        if isinstance(v, CorruptionStage):
            return v
        return CorruptionStage.from_string(v)
    
    @validator("region_context", pre=True)
    def validate_region_context(cls, v: str | RegionContext | None) -> RegionContext | None:
        """Convert string to RegionContext enum"""
        if v is None:
            return v
        if isinstance(v, RegionContext):
            return v
        return RegionContext.from_string(v)
    
    @validator("data_checksum")
    def validate_data_checksum(cls, v: str | None) -> str | None:
        """Validate data checksum format if provided"""
        if v is None:
            return v
        if not re.match(r"^[a-f0-9]{64}$", v.lower()):
            raise ValueError("Data checksum must be a valid SHA256 hex string")
        return v.lower()
    
    @root_validator
    def validate_entity_consistency(cls, values):
        """Validate entity consistency and relationships"""
        entity_type = values.get("entity_type")
        category = values.get("category")
        dread_level = values.get("dread_level")
        
        # Ensure category matches entity type expectations
        if entity_type and category:
            expected_categories = {
                EntityType.BIOME_SPEC: ["world", "tiles"],
                EntityType.NPC_SPEC: ["characters"],
                EntityType.ENCOUNTER_SPEC: ["encounters"],
                EntityType.ITEM_SPEC: ["items"],
                EntityType.LOCATION_SPEC: ["world", "locations"]
            }
            if entity_type in expected_categories:
                if category not in expected_categories[entity_type]:
                    raise ValueError(f"Category '{category}' not valid for entity type '{entity_type}'")
        
        return values


class EntityQuery(SQLModel):
    """Query parameters for entity search"""
    
    entity_type: str | None = Field(default=None, description="Filter by entity type")
    category: str | None = Field(default=None, description="Filter by category")
    subcategory: str | None = Field(default=None, description="Filter by subcategory")
    
    region_context: str | None = Field(default=None, description="Filter by region")
    dread_level: int | None = Field(default=None, description="Filter by dread level")
    philosophy_path: str | None = Field(default=None, description="Filter by philosophy path")
    act_context: int | None = Field(default=None, description="Filter by act")
    corruption_stage: str | None = Field(default=None, description="Filter by corruption stage")
    
    tags: list[str] = Field(default_factory=list, description="Filter by tags (AND logic)")
    active_only: bool = Field(default=True, description="Only return active entities")
    validated_only: bool = Field(default=True, description="Only return validated entities")
    
    limit: int | None = Field(default=None, description="Maximum results to return")
    offset: int = Field(default=0, description="Results offset for pagination")
