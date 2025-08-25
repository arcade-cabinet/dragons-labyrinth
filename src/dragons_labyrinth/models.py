"""
Pydantic models for Dragon's Labyrinth HBF analysis
"""

import sqlite3
import logging
from pathlib import Path
from typing import Any

from pydantic import BaseModel, Field, ConfigDict
from rich.console import Console
import pandas as pd


class HBFConfig(BaseModel):
    """Configuration for HBF loading and processing"""
    model_config = ConfigDict(arbitrary_types_allowed=True)
    
    hbf_path: Path = Field(description="Path to HBF database file")
    output_dir: Path = Field(description="Output directory for results")
    batch_size: int = Field(default=1000, description="Batch size for processing")
    
    def model_post_init(self, __context: Any) -> None:
        """Ensure paths are Path objects and output dir exists"""
        self.hbf_path = Path(self.hbf_path)
        self.output_dir = Path(self.output_dir)
        self.output_dir.mkdir(parents=True, exist_ok=True)


class OrchestratorState(BaseModel):
    """
    Shared state for all HBF orchestrator components.
    Passed to all mixins to provide standardized access to data and utilities.
    """
    model_config = ConfigDict(arbitrary_types_allowed=True)
    
    # Core data
    entities_df: pd.DataFrame | None = Field(default=None, description="Entities DataFrame")
    refs_df: pd.DataFrame | None = Field(default=None, description="References DataFrame")
    
    # Database connections
    sqlite_conn: Any = Field(default=None, description="Raw SQLite connection")  # sqlite3.Connection
    sqlalchemy_engine: Any = Field(default=None, description="SQLAlchemy engine")
    
    # Utilities
    log: Any = Field(default=None, description="Logger instance")  # logging.Logger
    console: Any = Field(default=None, description="Rich console instance")  # Console
    
    # Configuration
    config: HBFConfig = Field(description="HBF configuration")


class EntityData(BaseModel):
    """Parsed entity data from HBF"""
    model_config = ConfigDict(extra="allow")
    
    entity_type: str = Field(default="unknown", description="Type of entity")
    name: str | None = Field(default=None, description="Entity name")
    content: str | None = Field(default=None, description="HTML content")
    description: str | None = Field(default=None, description="Entity description")
    
    # Relationships
    faction: str | None = Field(default=None, description="Faction reference")
    leader: str | None = Field(default=None, description="Leader reference")
    location: str | None = Field(default=None, description="Location reference")
    parent: str | None = Field(default=None, description="Parent entity reference")
    dungeon: str | None = Field(default=None, description="Dungeon reference")
    region: str | None = Field(default=None, description="Region reference")
    hex: str | None = Field(default=None, description="Hex reference")
    
    # List fields
    members: list[str] = Field(default_factory=list, description="Member references")
    collaborators: list[str] = Field(default_factory=list, description="Collaborator references")
    connections: list[str] = Field(default_factory=list, description="Connection references")
    rooms: list[str] = Field(default_factory=list, description="Room references")
    items: list[str] = Field(default_factory=list, description="Item references")


class Entity(BaseModel):
    """Complete entity from HBF database"""
    uuid: str = Field(description="Entity UUID")
    value: str = Field(description="Raw JSON value")
    data: EntityData | dict = Field(default_factory=dict, description="Parsed entity data")
    entity_type: str = Field(default="unknown", description="Entity type extracted from data")


class Reference(BaseModel):
    """Reference between entities"""
    uuid: str = Field(description="Source entity UUID")
    value: str = Field(description="Target entity UUID")
    ref_type: str = Field(default="unknown", description="Type of reference")


class Cluster(BaseModel):
    """Cluster of entities for batch processing"""
    cluster_id: str = Field(description="Unique cluster identifier")
    entity_type: str = Field(description="Type of entities in cluster")
    entity_ids: list[str] = Field(default_factory=list, description="List of entity UUIDs")
    total_tokens: int = Field(default=0, description="Total token count")
    entity_count: int = Field(default=0, description="Number of entities")


class GraphMetrics(BaseModel):
    """Metrics for entity relationship graph"""
    node_count: int = Field(description="Total number of nodes")
    edge_count: int = Field(description="Total number of edges")
    density: float = Field(description="Graph density")
    components: int = Field(description="Number of weakly connected components")
    avg_in_degree: float | None = Field(default=None, description="Average in-degree")
    max_in_degree: int | None = Field(default=None, description="Maximum in-degree")
    avg_out_degree: float | None = Field(default=None, description="Average out-degree")
    max_out_degree: int | None = Field(default=None, description="Maximum out-degree")


class HBFSummary(BaseModel):
    """Summary statistics for HBF database"""
    total_entities: int = Field(description="Total number of entities")
    entities_with_content: int = Field(description="Entities with non-empty content")
    empty_entities: int = Field(description="Entities with empty content")
    unique_entity_types: int = Field(description="Number of unique entity types")
    total_references: int = Field(description="Total number of references")
    type_distribution: dict[str, int] = Field(
        default_factory=dict,
        description="Distribution of entity types"
    )
