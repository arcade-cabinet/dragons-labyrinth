# Dragon's Labyrinth Project Brief

## Core Mission

Dragon's Labyrinth is a horror RPG that inverts the traditional power fantasy. Instead of growing stronger, players grow more cursed as they journey toward an inevitable confrontation with an ancient dragon. The game uses familiar RPG mechanics to deliver genuine psychological horror through companion trauma, mathematical progression of dread, and the weight of moral choices.

## The Innovation

### Inverted Power Curve
Traditional RPGs: Weak → Strong → Victory
Dragon's Labyrinth: Hopeful → Cursed → Hunted

### Algorithmic Horror World
- **Infinite hex map**: Procedurally generated world that extends forever
- **Mathematical corruption**: Dread increases with distance from starting point (levels 1-180)
- **Companion psychology**: Sophisticated trauma system where NPCs break under pressure
- **Proximity horror**: The dragon actively hunts the player in final encounters

### Technical Architecture
- **Godot Engine**: 2.5D hex-based overworld with first-person horror sequences
- **Python Generation**: Content generation system using HBF worldbuilding data
- **Direct ECS**: No runtime database - components are source of truth
- **Layer Cake Assets**: AI-generated visual/audio assets with consistent style

## Current Development Status

### Generator Architecture (In Progress)
**Status**: Systematic refactoring of 8 subpackages to modern Python standards

**Current Reality**: 
- 8 subpackages exist but need refactoring (entities, seeds, psychology, world, maps, encounters, sprites, assets)
- Root-level files need cleanup (constants.py, types.py, models.py, protocols.py, manager.py)
- Import violations throughout (using Optional, Dict, List instead of modern syntax)
- Complex manager classes need replacement with simple run() functions

**Target Architecture**:
```
src/generator/
├── __main__.py              # ✅ Fixed - single engine orchestrator
├── statistics.py            # ✅ Good - self-contained
└── [8 subpackages]/         # 🎯 NEEDS SYSTEMATIC REFACTORING
    ├── entities/            # Recently refactored training system
    ├── seeds/
    ├── psychology/
    ├── world/
    ├── maps/
    ├── encounters/
    ├── sprites/
    └── assets/
```

### Recent Achievement: ML Training Refactoring
**Completed**: Entities training system transformation
- **From**: 375 lines of generic ML clustering/discovery
- **To**: Organized subpackage using HBF worldbuilding breakthrough data
- **Approach**: Content extraction not category detection
- **Data**: 60 organized examples (27 regions, 10 settlements, 5 factions, 18 dungeons)

### HBF Worldbuilding Breakthrough
**Major Discovery**: Successfully extracted and organized complete world data
- **Direct database access**: Bypassed complex analysis with direct SQLite queries
- **Organized categories**: Regions, settlements, factions, dungeons in memory-bank/world-building/
- **Rich content**: Complete NPCs, stats, treasure, weather, political networks
- **Training ready**: Perfect examples for ML content extraction

## Next Critical Phase: Generator Subpackage Refactoring

### Systematic Refactoring Tasks (T1-T9)
**T1-ENTITIES through T8-WORLD**: Each subpackage needs:
1. **Merge orm.py into models.py** - eliminate duplicates
2. **Replace manager classes** with simple `run(engine, logger, console)` functions  
3. **Fix import violations** - Optional → `str | None`, Dict → `dict`, List → `list`
4. **Move imports to top** - no imports inside functions
5. **Extract root content** - move subpackage-owned content from root files
6. **Remove defensive programming** - files are committed and guaranteed

**T9-CLEANUP**: 
7. **Delete root files** after content moved: constants.py, types.py, models.py, protocols.py, manager.py
8. **Keep root files**: __main__.py, statistics.py, README.md

### Architectural Standards (Non-Negotiable)
- **Modern Type System**: `dict[str, any]` not `Dict[str, Any]`, `str | None` not `Optional[str]`
- **Import Standards**: ALL imports at top, absolute imports only
- **SQLModel Architecture**: Single engine pattern, simple functions over classes
- **Subpackage Ownership**: Each subpackage owns its types/models/protocols
- **No Defensive Programming**: Repository files are committed and guaranteed

### Success Criteria
- Simple `run()` function interfaces in all subpackages
- Modern Python standards throughout
- No import violations or defensive programming
- Clean root directory with only essential files
- Working content generation pipeline

## World Design Foundation

### The Lands of Vo'il
- **600+ hex tiles** across 27 named regions
- **Political complexity**: Cities (1000+ entities) vs Villages (150 entities)
- **Faction networks**: 5 major factions with territorial disputes
- **Geographic logic**: Natural barriers, biome consistency, trade routes

### Horror Progression (Mathematical)
- **Peace** (0-20 hex distance): Everything seems normal
- **Unease** (20-40): Something feels wrong
- **Dread** (40-60): Open acknowledgment of approaching terror
- **Terror** (60-120): Reality becomes unreliable
- **Horror** (120+): Hunted by incomprehensible intelligence

### Technical Foundation Ready
- **Godot addons**: hexagon_tilemaplayer, godot-sqlite, worldbuilder integrated
- **Asset generation**: DALL-E integration with layer cake approach
- **Database architecture**: Comprehensive cross-system integration complete
- **Content pipeline**: HBF → Python generation → Godot resources

## Development Philosophy

### "The Game Is What Matters"
- Everything is tooling except the final playable experience
- Technical architecture serves the horror narrative
- Simplicity over abstraction
- Direct generation over complex pipelines

### Horror-First Design
- Every system serves the emotional journey: Peace → Unease → Dread → Terror → Horror
- Companion psychology and player curse progression are core mechanics
- Mathematical progression ensures consistent horror escalation
- Environmental storytelling through algorithmic corruption

## Current Priority

**IMMEDIATE**: Continue systematic refactoring of generator subpackages (T1-T8)
**GOAL**: Clean, modern Python architecture ready for content generation
**TIMELINE**: Complete all 8 subpackages in continuous execution mode
**SUCCESS**: Working ML training system + refactored generator = production-ready content pipeline

The project is 80% complete with solid technical foundations. The remaining work is architectural cleanup to enable the final push to a playable horror RPG experience.
