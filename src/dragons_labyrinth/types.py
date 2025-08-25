"""
Type definitions and aliases for Dragon's Labyrinth HBF analysis.

This module defines types following professor-pixels standards with modern Python
typing patterns, enums with auto() values, and comprehensive type coverage.
"""

from __future__ import annotations

from enum import Enum, auto
from typing import Literal, TypeAlias, Any
from pathlib import Path

import pandas as pd
from sqlalchemy.engine import Engine
import networkx as nx

# ----------------------------
# Core Path and File Types
# ----------------------------
FilePath: TypeAlias = Path | str
HBFPath: TypeAlias = Path
OutputPath: TypeAlias = Path
ConfigPath: TypeAlias = Path

# ----------------------------
# DataFrame Types
# ----------------------------
EntitiesDataFrame: TypeAlias = pd.DataFrame
ReferencesDataFrame: TypeAlias = pd.DataFrame
ClusterDataFrame: TypeAlias = pd.DataFrame
AnalysisDataFrame: TypeAlias = pd.DataFrame

# ----------------------------
# Database Types
# ----------------------------
SQLiteEngine: TypeAlias = Engine
DatabaseConnection: TypeAlias = Any  # sqlite3.Connection
CheckpointConnection: TypeAlias = Any  # SqliteSaver connection

# ----------------------------
# Graph Types
# ----------------------------
EntityGraph: TypeAlias = nx.DiGraph
RelationshipGraph: TypeAlias = nx.Graph
WorkflowGraph: TypeAlias = nx.DiGraph

# ----------------------------
# ID and Reference Types
# ----------------------------
EntityID: TypeAlias = str
ClusterID: TypeAlias = str
WorkflowID: TypeAlias = str
SliceID: TypeAlias = str
PatternID: TypeAlias = str
ComponentID: TypeAlias = str
ReferenceType: TypeAlias = str
HyperlinkRef: TypeAlias = str

# ----------------------------
# HBF Entity Types
# ----------------------------
EntityUUID: TypeAlias = str
EntityValue: TypeAlias = str
EntityContent: TypeAlias = str
HTMLContent: TypeAlias = str
JSONContent: TypeAlias = str

# ----------------------------
# Data Structure Types
# ----------------------------
EntityDict: TypeAlias = dict[str, Any]
PatternDict: TypeAlias = dict[str, Any]
ComponentDict: TypeAlias = dict[str, Any]
SummaryDict: TypeAlias = dict[str, int | dict[str, int]]
MetricsDict: TypeAlias = dict[str, float | int | None]
ConfigDict: TypeAlias = dict[str, Any]
StateDict: TypeAlias = dict[str, Any]
ReviewData: TypeAlias = dict[str, Any]

# ----------------------------
# Token and Processing Types
# ----------------------------
TokenCount: TypeAlias = int
BatchSize: TypeAlias = int
EntityCount: TypeAlias = int
ProcessingStep: TypeAlias = int

# ----------------------------
# Slice and Analysis Types
# ----------------------------
SliceType: TypeAlias = Literal[
    "region",
    "dungeon", 
    "settlement",
    "faction",
    "monster",
    "inn",
    "dwelling"
]

AnalysisDepth: TypeAlias = Literal[
    "shallow",
    "standard", 
    "deep",
    "comprehensive"
]

IntegrationLevel: TypeAlias = Literal[
    "basic",
    "horror_aware",
    "philosophy_integrated",
    "companion_responsive"
]

# ----------------------------
# Workflow and State Types
# ----------------------------
WorkflowStage: TypeAlias = Literal[
    "initialization",
    "entity_extraction", 
    "html_analysis",
    "pattern_discovery",
    "human_review",
    "component_generation",
    "rust_generation",
    "finalization"
]

ApprovalStatus: TypeAlias = Literal[
    "pending",
    "approved",
    "rejected",
    "request_changes",
    "request_deeper_analysis"
]

# ----------------------------
# Component and Generation Types
# ----------------------------
ComponentType: TypeAlias = Literal[
    "region_component",
    "biome_component",
    "weather_component",
    "dungeon_component",
    "room_component",
    "settlement_component",
    "faction_component", 
    "npc_component",
    "monster_component"
]

GenerationType: TypeAlias = Literal[
    "component_struct",
    "system_impl",
    "spawn_function", 
    "relationship_mapper"
]

# ----------------------------
# Horror Integration Types
# ----------------------------
DreadLevel: TypeAlias = Literal[0, 1, 2, 3, 4]
PhilosophyPath: TypeAlias = Literal[
    "strength",
    "harmony", 
    "light",
    "dark"
]

CompanionStress: TypeAlias = Literal[
    "none",
    "mild",
    "moderate", 
    "severe",
    "breaking_point"
]

CorruptionLevel: TypeAlias = Literal[
    "pristine",
    "tainted",
    "corrupted",
    "defiled", 
    "consumed"
]

# ----------------------------
# Enums with auto() values
# ----------------------------

class EntityType(Enum):
    """HBF entity types discovered in analysis."""
    REGION = auto()
    HEX = auto()
    BIOME = auto()
    DUNGEON = auto()
    CAVE = auto()
    TEMPLE = auto()
    TOMB = auto()
    SETTLEMENT = auto()
    CITY = auto()
    TOWN = auto()
    VILLAGE = auto()
    FACTION = auto()
    CULT = auto()
    MILITIA = auto()
    SYNDICATE = auto()
    NPC = auto()
    MONSTER = auto()
    INN = auto()
    DWELLING = auto()
    FARM = auto()
    CABIN = auto()
    STRONGHOLD = auto()
    UNKNOWN = auto()


class AnalysisStatus(Enum):
    """Status of slice analysis operations."""
    NOT_STARTED = auto()
    EXTRACTING_ENTITIES = auto()
    ANALYZING_HTML = auto()
    DISCOVERING_PATTERNS = auto()
    AWAITING_REVIEW = auto()
    GENERATING_COMPONENTS = auto()
    FINALIZING = auto()
    COMPLETED = auto()
    FAILED = auto()
    CANCELLED = auto()


class WorkflowEvent(Enum):
    """Events that can occur during workflow execution."""
    WORKFLOW_START = auto()
    NODE_ENTRY = auto()
    NODE_EXIT = auto()
    HUMAN_INTERRUPT = auto()
    HUMAN_RESPONSE = auto()
    ERROR_OCCURRED = auto()
    CHECKPOINT_SAVED = auto()
    WORKFLOW_COMPLETE = auto()
    WORKFLOW_FAILED = auto()


class IntegrationPoint(Enum):
    """Horror RPG integration points for HBF data."""
    DREAD_AMPLIFICATION = auto()
    WEATHER_CORRUPTION = auto()
    COMPANION_STRESS = auto()
    PHILOSOPHY_RESONANCE = auto()
    POPULATION_DECAY = auto()
    SOCIAL_BREAKDOWN = auto()
    CONSPIRACY_MECHANICS = auto()
    TRUST_DEGRADATION = auto()
    CLAUSTROPHOBIA_TRIGGERS = auto()
    CIVILIZATION_COMFORT = auto()


class PatternCategory(Enum):
    """Categories of patterns discovered in HBF data."""
    GEOGRAPHIC = auto()
    STRUCTURAL = auto()
    SOCIAL = auto()
    MECHANICAL = auto()
    NARRATIVE = auto()
    PROBABILISTIC = auto()
    RELATIONAL = auto()
    TEMPORAL = auto()


class OutputFormat(Enum):
    """Output formats for generated content."""
    RUST_CODE = auto()
    JSON_DATA = auto()
    TOML_CONFIG = auto()
    YAML_SPEC = auto()
    MARKDOWN_DOC = auto()


class ValidationLevel(Enum):
    """Validation levels for generated content."""
    SYNTAX_CHECK = auto()
    TYPE_CHECK = auto()
    INTEGRATION_CHECK = auto()
    HORROR_ALIGNMENT_CHECK = auto()
    COMPREHENSIVE = auto()


# ----------------------------
# Collection Types
# ----------------------------
EntityCollection: TypeAlias = list[EntityDict]
PatternCollection: TypeAlias = list[PatternDict]
ComponentCollection: TypeAlias = list[ComponentDict]
IntegrationPointList: TypeAlias = list[str]
HyperlinkCollection: TypeAlias = list[HyperlinkRef]
ProbabilityTableCollection: TypeAlias = list[dict[str, Any]]

# ----------------------------
# Search and Query Types
# ----------------------------
SearchQuery: TypeAlias = str
SearchFilter: TypeAlias = dict[str, Any]
SearchResult: TypeAlias = dict[str, Any]
QueryPattern: TypeAlias = str

# ----------------------------
# Memory and Cache Types
# ----------------------------
CacheKey: TypeAlias = str
CacheValue: TypeAlias = Any
MemoryStore: TypeAlias = dict[str, Any]
CheckpointState: TypeAlias = dict[str, Any]

# ----------------------------
# Error and Result Types
# ----------------------------
ErrorMessage: TypeAlias = str
SuccessResult: TypeAlias = dict[str, Any]
ValidationError: TypeAlias = dict[str, str]
ProcessingResult: TypeAlias = dict[str, Any]

__all__ = [
    # Path types
    "FilePath", "HBFPath", "OutputPath", "ConfigPath",
    # DataFrame types
    "EntitiesDataFrame", "ReferencesDataFrame", "ClusterDataFrame", "AnalysisDataFrame",
    # Database types
    "SQLiteEngine", "DatabaseConnection", "CheckpointConnection",
    # Graph types  
    "EntityGraph", "RelationshipGraph", "WorkflowGraph",
    # ID types
    "EntityID", "ClusterID", "WorkflowID", "SliceID", "PatternID", "ComponentID",
    "ReferenceType", "HyperlinkRef",
    # HBF types
    "EntityUUID", "EntityValue", "EntityContent", "HTMLContent", "JSONContent", 
    # Data structure types
    "EntityDict", "PatternDict", "ComponentDict", "SummaryDict", "MetricsDict",
    "ConfigDict", "StateDict", "ReviewData",
    # Token types
    "TokenCount", "BatchSize", "EntityCount", "ProcessingStep",
    # Slice types
    "SliceType", "AnalysisDepth", "IntegrationLevel",
    # Workflow types
    "WorkflowStage", "ApprovalStatus",
    # Component types
    "ComponentType", "GenerationType", 
    # Horror integration types
    "DreadLevel", "PhilosophyPath", "CompanionStress", "CorruptionLevel",
    # Enums
    "EntityType", "AnalysisStatus", "WorkflowEvent", "IntegrationPoint",
    "PatternCategory", "OutputFormat", "ValidationLevel",
    # Collection types
    "EntityCollection", "PatternCollection", "ComponentCollection",
    "IntegrationPointList", "HyperlinkCollection", "ProbabilityTableCollection",
    # Search types
    "SearchQuery", "SearchFilter", "SearchResult", "QueryPattern",
    # Memory types
    "CacheKey", "CacheValue", "MemoryStore", "CheckpointState",
    # Error types
    "ErrorMessage", "SuccessResult", "ValidationError", "ProcessingResult",
]
