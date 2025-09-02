# Unified World Generation Architecture Plan

## Current State Analysis

### What Works ✅
- **World Crate Pipeline**: `crates/world/src/generated/` structure ready for Python-generated Rust code
- **Builder Integration**: `WorldGenerator` writes modular Rust code (resources, systems, components)
- **ECS Integration**: Generated modules properly integrate with Bevy ECS
- **HBF Rich Data**: 70,801 entities from transformer.py successfully extracted and clustered

### What's Broken ❌
- **Generator System Fragmentation**: Seeds and entities systems both target non-existent SQLite/Godot
- **Duplicated Generation Logic**: Two separate systems doing similar Python → Rust transformation
- **Architecture Mismatch**: Python generators not connected to working world crate pipeline

## Unified Architecture Solution

### New Flow Design
```
HBF Database (70,801 entities) + Literature Sources
    ↓
Python Processing (src/generator/)
    ↓ [UNIFIED GENERATION]
World Crate (crates/world/src/generated/)
    ↓
ECS Components & Systems
    ↓
Game Runtime
```

### Unified World Crate Structure
```
crates/world/src/generated/
├── mod.rs              # Main registration
├── resources.rs        # Biome maps, world data, seeds configurations
├── components.rs       # HexTile, Settlement, Faction, Dungeon, Seeds components
├── systems/
│   ├── mod.rs         # System registration
│   ├── hex_gen.rs     # Hex generation from HBF regions
│   ├── settlement.rs  # Settlement placement and data
│   ├── faction.rs     # Faction territorial control
│   ├── dungeon.rs     # Dungeon placement and encounters
│   ├── narrative.rs   # Seeds-based narrative systems
│   ├── motifs.rs      # Visual motif application
│   └── horror.rs      # Horror progression systems
└── queries.rs         # Complex queries for world data
```

## Implementation Plan

### Phase 1: Create Unified Generator System
1. **Create `src/generator/world/`** - New unified world generator
2. **Move Best Logic** - Extract working parts from both seeds + entities
3. **Unified Models** - Rust-compatible data structures for both domains
4. **World Code Templates** - Jinja2 templates for all world crate modules

### Phase 2: Route Both Systems to World Crate
1. **Seeds → World**: Route seeds extraction to world crate generation
2. **Entities → World**: Route HBF processing to world crate generation  
3. **Preserve Good Logic**: Keep transformer.py clustering, literature extraction patterns
4. **Single Generation Command**: One command generates complete world crate

### Phase 3: Enhanced World Crate Modules
1. **Rich Resources**: World data, biome configurations, narrative seeds, HBF entities
2. **Advanced Components**: Settlement, Faction, Dungeon, NarrativeSeed, MotifSeed components
3. **Intelligent Systems**: Settlement placement, faction territories, dungeon generation, horror progression
4. **Complex Queries**: Spatial queries, faction relationships, narrative context

## Technical Benefits

### Eliminates Duplication
- **Single Generation Path**: One Python → Rust pipeline instead of two
- **Shared Templates**: Reusable Jinja2 templates for all Rust generation
- **Unified Models**: One set of Rust-compatible data structures
- **Single Testing**: Test one pipeline instead of multiple

### Leverages Existing Infrastructure  
- **Working World Crate**: Uses existing `crates/world/src/generated/` structure
- **Builder Integration**: Works with existing `WorldGenerator` patterns
- **ECS Ready**: Generates components and systems that integrate immediately
- **Game Compatible**: No changes needed to game loading logic

### Maximizes Data Utilization
- **Full HBF Integration**: All 70,801 entities properly utilized
- **Rich Seeds Data**: Literature-extracted narrative patterns integrated
- **Spatial Relationships**: HBF spatial data + seeds thematic data combined
- **Horror Progression**: Both mechanical (entities) and atmospheric (seeds) horror

## Data Flow Redesign

### Input Data Sources
1. **HBF Database**: `memory-bank/world-output/nTR8nJOW.hbf`
   - 27 regions, 10 settlements, 5 factions, 18 dungeons
   - Spatial coordinates, relationships, encounter data

2. **Literature Sources**: Books, NLTK corpora, linguistic databases
   - Narrative patterns, motifs, semantic concepts
   - Emotional progressions, linguistic patterns

### Processing Pipeline
1. **HBF Extraction**: `transformer.py` clustering (keep this - it works excellently)
2. **Literature Processing**: Seeds extraction from multiple sources
3. **Data Fusion**: Combine spatial HBF data with thematic seeds data
4. **World Generation**: Generate complete world crate with all systems

### Output Integration
1. **ECS Components**: HexTile, Settlement, Faction, Dungeon, NarrativeSeed, etc.
2. **Resource Data**: Biome maps, world configuration, narrative seeds
3. **System Logic**: Placement algorithms, progression systems, horror mechanics
4. **Game Runtime**: Immediate integration with existing game systems

## Implementation Priority

### Immediate (Next Session)
1. **Create `src/generator/world/` package** - Unified world generation system
2. **Design unified data models** - Rust-compatible structures for both domains
3. **Create world crate templates** - Jinja2 templates for all generated modules
4. **Route seeds system** - Point to world crate instead of SQLite

### This Phase (Unified Generation)
1. **Route entities system** - HBF processing → world crate generation
2. **Test full pipeline** - Python → World crate → ECS → Game
3. **Data fusion logic** - Combine HBF spatial + seeds thematic data
4. **Remove broken systems** - Eliminate SQLite/database dependencies

### Next Phase (Enhanced World)
1. **Advanced ECS systems** - Settlement placement, faction territories
2. **Horror progression integration** - Both mechanical and atmospheric
3. **Narrative system integration** - Seeds-driven storytelling
4. **Complete testing** - Full pipeline validation

## Success Criteria

### Technical Achievements
- [x] Unified generation pipeline: `src/generator/world/` → `crates/world/src/generated/`
- [ ] Both HBF entities (70,801) and literature seeds processed into world crate
- [ ] Generated world crate compiles and integrates with game
- [ ] Single command generates complete world with both data sources
- [ ] No SQLite or database dependencies remain

### Data Integration Success
- [ ] HBF regions properly generate hex tiles with spatial relationships
- [ ] Settlements placed with correct scale, services, and faction relationships
- [ ] Factions have territorial control and political dynamics
- [ ] Dungeons generate with appropriate encounter rules and treasure
- [ ] Narrative seeds create atmospheric horror progression
- [ ] Motifs enhance visual and thematic consistency

### Game Integration Success
- [ ] World loads with rich generated content
- [ ] Player movement through meaningful spatial relationships
- [ ] Encounters reflect both mechanical and narrative design
- [ ] Horror progression works across all systems
- [ ] Performance remains smooth with complex generated world

## Architecture Benefits Summary

This unified approach eliminates the architectural mismatch by:
1. **Single Target**: Both systems generate for the same world crate
2. **Leveraged Infrastructure**: Uses existing working world generation pipeline  
3. **Data Fusion**: Combines the best of spatial HBF data with thematic seeds
4. **Reduced Complexity**: One generation path instead of multiple competing systems
5. **Better Integration**: Generated code immediately usable by ECS systems

The result will be a rich, cohesive game world that leverages both the mechanical depth of HBF worldbuilding data AND the atmospheric richness of literature-extracted narrative seeds.
