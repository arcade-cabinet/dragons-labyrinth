"""Generation result models for AI model generation tracking.

Following .clinerules standards with modern Python type hints.
"""

from __future__ import annotations

from datetime import datetime
from pydantic import BaseModel, Field


class ModelConnections(BaseModel):
    """Connection information surfaced by generated models."""
    uuid_fields: list[str] = Field(default_factory=list)
    connection_fields: list[str] = Field(default_factory=list)
    import_path: str = ""
    exported_classes: list[str] = Field(default_factory=list)


class GenerationResults(BaseModel):
    """Results from AI model generation."""
    models_generated: list[str] = Field(default_factory=list)
    types_generated: list[str] = Field(default_factory=list)
    protocols_generated: list[str] = Field(default_factory=list)
    analysis_notes: list[str] = Field(default_factory=list)
    token_usage: dict[str, int] = Field(default_factory=dict)
    connections: ModelConnections | None = None
    success: bool = False


class AnalysisSummary(BaseModel):
    """Summary of analysis results for reporting."""
    total_entities_processed: int
    regions_count: int
    settlements_count: int
    factions_count: int
    dungeons_count: int
    uncategorized_count: int
    processing_timestamp: datetime = Field(default_factory=datetime.now)
    ready_for_processors: bool = True
