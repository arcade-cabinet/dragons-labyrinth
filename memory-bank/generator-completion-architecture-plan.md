# Dragon's Labyrinth Generator - Complete Architecture Plan

## Date: 2025-08-28
## Status: SEEDS COMPLETE - READY FOR COMPREHENSIVE REFACTOR

## Current State
- ✅ **Entities Subpackage**: Complete ML-first entity extraction system
- ✅ **Seeds Subpackage**: Complete pandas-based unified sources + ML extraction
- ✅ **Database Manager**: Slim coordinator connecting entities + seeds
- ✅ **Clean Code**: Removed defensive programming, proper error handling

## Next Phase: Database Unification & ML Integration

### 1. Psychology Subpackage → db/psychology/
**Move**: `src/generator/psychology/` → `src/generator/db/psychology/`

**Architecture Requirements**:
- Follow **entities pattern**: manager.py, extractors.py, orm.py, errors.py, types.py, protocols.py
- **ML-First Integration**: Use BOTH entities data AND seeds data for psychology generation
- **Psychology Tables**: 
  - CompanionProfiles (trauma, behaviors, dialogue patterns)
  - HorrorProgression (5-stage dread escalation)
  - PlayerPsychology (corruption mechanics, sanity states)
- **Cross-System Queries**: Psychology manager queries entities for NPCs, seeds for emotional patterns
- **Clean Error Handling**: Proper exceptions, no defensive programming

### 2. World Subpackage → db/world/
**Move**: `src/generator/world/` → `src/generator/db/world/`

**Architecture Requirements**:
- World-level coordination using ALL other subpackages
- **World Tables**: 
  - Regions (assembled from hex tiles + entities)
  - Campaigns (progression through horror stages)
  - WorldState (global corruption, player progress)
- **Dependencies**: entities (for NPCs/locations), seeds (for themes), psychology (for horror)
- **Godot Integration**: Generate .tres resources for world assembly

### 3. Maps Subpackage Creation → db/maps/
**Merge**: `tiles/` + existing `db/hex_tiles/` → `db/maps/`

**Architecture Requirements**:
- **Maps Tables**:
  - HexTiles (coordinates, biomes, corruption)
  - Regions (assembled hex clusters)
  - Dungeons (procedural dungeon data)
- **Coordinate System**: Proper hex grid with adjacency rules
- **Map Generation**: Use entities data for placement, psychology for horror zones
- **Godot Output**: Generate map files directly for hex tilemap addon

### 4. Encounters & Sprites → db/encounters/ & db/sprites/
**Move**: `encounters/`, `sprites/` → `db/encounters/`, `db/sprites/`

**Architecture Requirements**:
- **Encounters Tables**: CombatEncounters, NPCInteractions, QuestEvents
- **Sprites Tables**: CharacterSprites, MonsterSprites, ItemSprites
- **Cross-Dependencies**: Use entities for NPCs, psychology for behaviors, maps for locations
- **AI Integration**: Connect to DALL-E MCP for sprite generation

### 5. Assets Subpackage Completion → db/assets/
**Complete**: `db/assets/` as blob storage system

**Architecture Requirements**:
- **Assets Table**: Store ALL generated 2D assets as SQLite blobs
- **AI Integration**: Direct DALL-E → SQLite pipeline with data integrity
- **Idempotency**: Checksums prevent duplicate generation
- **Separation**: Only AI-generated assets in SQLite, static assets (3D models, fonts) remain in filesystem
- **Godot Loader**: Extract blobs to temporary files for Godot loading

## Root Generator Refactor

### 6. World Building Integration → Root Generator
**Merge**: `src/generator/world_building/` → `src/generator/` (root level)

**Integration Points**:
- Merge world building logic into root generator package
- Update orchestrator to initialize db subpackage properly
- Add HBF loading flag: `--hbf-db path/to/database.db`

### 7. Final Orchestrator
**Root orchestrator becomes**:
```python
# A) Initialize db subpackage (all subpackages coordinate)
db_manager = DatabaseManager()

# B) Optional HBF loading from --hbf-db flag
if hbf_path:
    db_manager.load_hbf_data(hbf_path)

# C) Run full ML pipeline: sources → entities → seeds → psychology → world
results = db_manager.run_full_pipeline(ml_processor)

# D) Generate Godot code/resources
godot_generator.generate_all(results)
```

## Data Flow Architecture

### ML Pipeline Flow
```
1. Sources (pandas loaders) → Raw text data in SQLite
2. Entities (ML extractors) → NPCs, locations, items from sources
3. Seeds (ML extractors) → Narrative, motifs, emotional patterns from sources  
4. Psychology (ML integration) → Use entities + seeds → Generate horror progression
5. World (coordination) → Use entities + psychology + maps → Generate regions/campaigns
6. Maps (coordinate assembly) → Use entities + world → Generate hex grids + dungeons
7. Assets (AI generation) → Use all data → Generate sprites/textures → Store as blobs
```

### Cross-System Dependencies
- **Psychology** ← entities (NPCs), seeds (emotional patterns)
- **World** ← entities (locations), psychology (horror stages), maps (regions)
- **Maps** ← entities (placement), psychology (corruption zones)  
- **Encounters** ← entities (NPCs), psychology (behaviors), maps (locations)
- **Sprites** ← entities (characters), psychology (expressions), encounters (contexts)
- **Assets** ← ALL systems (universal AI generation)

## Implementation Priority

### Phase 1: Psychology Integration
1. Move psychology → db/psychology/
2. Create proper ORM tables
3. Implement ML integration with entities + seeds data
4. Build cross-system query capabilities

### Phase 2: World & Maps Unification  
1. Move world → db/world/
2. Merge tiles → db/maps/
3. Implement coordinate system
4. Build region assembly

### Phase 3: Complete Subpackage Migration
1. Move encounters, sprites → db/
2. Complete assets blob storage
3. Implement AI generation pipeline

### Phase 4: Root Integration
1. Merge world_building into root
2. Update orchestrator for db initialization
3. Add HBF loading capability
4. Implement full Godot generation

## Benefits of This Architecture

### Clean Separation
- **Database layer**: All game data coordinated in SQLite
- **Generator layer**: Orchestration + Godot output
- **No scattered systems**: Everything under db/ with clear dependencies

### ML-First Throughout
- Every subpackage uses ML for content generation
- Cross-system ML queries enable rich content generation
- Psychology system leverages ALL other systems' data

### Godot-Ready Output
- Direct .tres/.gd/.tscn generation
- Proper hex tilemap integration
- Assets stored efficiently in SQLite with Godot loader

### Scalable Architecture
- Each subpackage self-contained but interconnected
- Easy to add new content types
- Clean dependency graph prevents circular imports

## Success Metrics
- **Single SQLite database** contains all game content
- **Cross-system ML queries** generate rich interconnected content
- **Direct Godot integration** with proper file generation
- **Clean architecture** with proper error handling throughout
- **AI asset pipeline** generating and storing content efficiently

This architecture creates a complete, interconnected content generation system where every component leverages every other component's data through ML to generate rich, coherent game content directly for Godot.
