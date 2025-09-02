"""Analysis models package.

This package contains the refactored analysis models following .clinerules standards
with modern Python type hints and absolute imports.
"""

# Base value objects and types
from generator.analysis.models.base import (
    HexKey,
    MapCoord,
    EdgeType,
    FieldSpec,
    EntitySpec,
    Inventory,
)

# Raw entity models
from generator.analysis.models.raw import RawEntity

# Result tracking models
from generator.analysis.models.results import (
    ModelConnections,
    GenerationResults,
    AnalysisSummary,
)

# Cluster abstractions
from generator.analysis.models.clusters import BaseEntitiesCluster

# Category-specific entity models
from generator.analysis.models.regions import (
    RegionHexTile,
    RawRegionEntities,
)
from generator.analysis.models.settlements import (
    SettlementEstablishment,
    RawSettlementEntities,
)
from generator.analysis.models.factions import (
    FactionEntity,
    RawFactionEntities,
)
from generator.analysis.models.dungeons import (
    DungeonArea,
    RawDungeonEntities,
)

# Container models for indexing
from generator.analysis.models.containers import (
    DungeonContainer,
    RegionContainer,
)

# Orchestration models
from generator.analysis.models.orchestration import RawEntities

__all__ = [
    # Base
    "HexKey",
    "MapCoord",
    "EdgeType",
    "FieldSpec",
    "EntitySpec",
    "Inventory",
    # Raw
    "RawEntity",
    # Results
    "ModelConnections",
    "GenerationResults",
    "AnalysisSummary",
    # Clusters
    "BaseEntitiesCluster",
    # Regions
    "RegionHexTile",
    "RawRegionEntities",
    # Settlements
    "SettlementEstablishment",
    "RawSettlementEntities",
    # Factions
    "FactionEntity",
    "RawFactionEntities",
    # Dungeons
    "DungeonArea",
    "RawDungeonEntities",
    # Containers
    "DungeonContainer",
    "RegionContainer",
    # Orchestration
    "RawEntities",
]
