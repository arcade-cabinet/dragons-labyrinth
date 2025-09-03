# Analysis Models Package

## Purpose
This package houses the new modular model set for Dragons Labyrinth analysis, split into focused modules following .clinerules standards with modern Python type hints and absolute imports.

## Module Organization

### Core Infrastructure
- **`base.py`**: Common value objects (HexKey, MapCoord), edge taxonomy (EdgeType enum), and inventory types (FieldSpec, EntitySpec, Inventory)
- **`raw.py`**: RawEntity model with smart clustering logic, factory methods, and file writing capabilities
- **`results.py`**: Generation tracking models (ModelConnections, GenerationResults, AnalysisSummary)
- **`clusters.py`**: BaseEntitiesCluster abstraction with two-stage AI generation (inventory → code)

### Category-Specific Models
- **`regions.py`**: RegionHexTile entity model and RawRegionEntities cluster
- **`settlements.py`**: SettlementEstablishment entity model and RawSettlementEntities cluster
- **`factions.py`**: FactionEntity model and RawFactionEntities cluster
- **`dungeons.py`**: DungeonArea model and RawDungeonEntities cluster

### Integration & Orchestration
- **`orchestration.py`**: RawEntities master orchestrator for 3-phase pipeline
- **`containers.py`**: Integration containers (DungeonContainer, RegionContainer) with spatial indexes (`by_hex`, `by_area`, `neighbors`)

## Edge Taxonomy
We normalize edges so containers can index/query efficiently:

- `settlement_in_hex`: settlement -> RegionHexTile
- `dungeon_in_hex`: dungeon -> RegionHexTile
- `area_connects_to_area`: DungeonArea -> DungeonArea (neighbors)
- `faction_controls_region`: faction -> region
- `faction_controls_settlement`: faction -> settlement
- (extensible via EdgeType enum)

## Two-Stage Generation Process

### Stage A: Inventory Analysis
- Uses OpenAI Structured Outputs with strict JSON schema
- Analyzes HTML/JSON samples to infer field inventory
- No code generation, only JSON metadata extraction
- Respects sample thresholds (HTML: 10, JSON: 5 by default)

### Stage B: Code Generation
- Renders deterministic Pydantic models from Stage A inventory
- Uses inline Jinja2 templates for consistent output
- Generates `extract_uuid_connections()` methods for relationship tracking
- Follows .clinerules standards (no Optional, modern type hints)

## 3-Phase Pipeline

1. **Phase 1**: Individual category models (regions, settlements, factions, dungeons)
2. **Phase 2**: Dungeon container integration (areas → complexes with navigation)
3. **Phase 3**: Regional container integration (all entities with spatial indexing)

## Key Features

- **Smart Clustering**: RawEntity automatically routes to appropriate category
- **Factory Methods**: `RawEntity.create()` computes fields during instantiation
- **Spatial Indexing**: Containers maintain hex-based and area-based lookups
- **Relationship Tracking**: UUID connections with typed edges
- **Clean Separation**: Each module has single, focused responsibility
- **Absolute Imports**: No wildcards, explicit paths throughout
