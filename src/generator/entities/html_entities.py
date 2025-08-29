"""
HTML Entities Table - HTML fragments requiring ML analysis.

Handles HBF entities that are HTML/text fragments requiring sophisticated
ML pattern analysis to extract Dragon's Labyrinth game content.
"""

from __future__ import annotations

import json
from datetime import datetime
from enum import Enum, auto
from typing import Any

from sqlmodel import SQLModel, Field, Column, Text, JSON
from sqlalchemy import Index


class HTMLContentType(str, Enum):
    """Types of HTML content detected from HBF."""
    HEX_TILE = auto()           # Hex coordinate with content
    CREATURE_BLOCK = auto()     # D&D stat blocks
    NPC_DESCRIPTION = auto()    # Character descriptions
    TREASURE_HOARD = auto()     # Treasure and items
    SETTLEMENT_DETAIL = auto()  # Village/town/city descriptions
    DUNGEON_ROOM = auto()       # Cave/temple/tomb descriptions
    FACTION_INFO = auto()       # Cult/militia/syndicate data
    NARRATIVE_TEXT = auto()     # Story elements and lore
    UNKNOWN = auto()            # Unclassified content


class HTMLAnalysisStatus(str, Enum):
    """ML analysis processing status."""
    PENDING = auto()            # Not yet analyzed
    IN_PROGRESS = auto()        # Currently being analyzed
    COMPLETED = auto()          # Analysis finished
    FAILED = auto()             # Analysis failed
    ROUTED = auto()             # Routed to specialized extractor


class HTMLEntityRecord(SQLModel, table=True):
    """
    HTML entities requiring ML analysis for Dragon's Labyrinth extraction.
    
    These are HBF entities that contain HTML/text fragments requiring
    sophisticated pattern analysis to extract game-relevant content.
    """
    __tablename__ = "html_entities"
    
    # Primary key using HBF UUID
    hbf_uuid: str = Field(primary_key=True, regex=r'^[a-zA-Z0-9]{8}$')
    
    # HTML content
    html_content: str = Field(..., sa_column=Column(Text))
    cleaned_text: str = Field(default="", sa_column=Column(Text), description="HTML stripped text")
    
    # ML Analysis status
    analysis_status: HTMLAnalysisStatus = Field(default=HTMLAnalysisStatus.PENDING)
    content_type: HTMLContentType = Field(default=HTMLContentType.UNKNOWN)
    
    # ML Analysis results
    pattern_matches: dict[str, Any] = Field(default_factory=dict, sa_column=Column(JSON))
    extracted_entities: dict[str, Any] = Field(default_factory=dict, sa_column=Column(JSON))
    horror_classification: dict[str, Any] = Field(default_factory=dict, sa_column=Column(JSON))
    
    # Dragon's Labyrinth integration
    region_detected: str | None = Field(default=None)  # Fearless Wilds, Vicious Crags, etc.
    dread_level_hint: int = Field(default=0, ge=0, le=4)
    philosophy_indicators: dict[str, float] = Field(default_factory=dict, sa_column=Column(JSON))
    
    # Routing after analysis
    extraction_targets: list[str] = Field(default_factory=list, sa_column=Column(JSON))
    priority_score: float = Field(default=0.0, ge=0.0, le=1.0)
    
    # Processing tracking
    analysis_started_at: datetime | None = Field(default=None)
    analysis_completed_at: datetime | None = Field(default=None)
    processing_errors: list[str] = Field(default_factory=list, sa_column=Column(JSON))
    
    # Raw entity tracking (required columns)
    source_entity_id: str = Field(...)
    extraction_timestamp: datetime = Field(default_factory=datetime.now)
    raw_content: str = Field(..., sa_column=Column(Text))
    confidence_score: float = Field(default=0.0, ge=0.0, le=1.0)
    
    # Indexes for ML processing workflow
    __table_args__ = (
        Index('idx_html_analysis_status', 'analysis_status'),
        Index('idx_html_content_type', 'content_type'),
        Index('idx_html_region_dread', 'region_detected', 'dread_level_hint'),
        Index('idx_html_priority', 'priority_score'),
        Index('idx_html_routing', 'extraction_targets'),
    )
