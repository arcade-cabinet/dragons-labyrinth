# Active Development Context

## Current Architecture

### Stack Overview
- **Game Engine**: Rust + Bevy 0.16.1 (replaced Godot)
- **Content Pipeline**: Python + OpenAI (markdown → JSON → game)
- **Rendering**: 2D hex-based with Material2dPlugin
- **Data Flow**: No database, direct JSON loading at runtime

### Why We Pivoted from Godot
- **Complexity**: Godot's node system was overkill for hex-based game
- **Performance**: Rust/Bevy provides better control and speed
- **Architecture**: ECS pattern fits our component-heavy design better
- **Workflow**: Hot-reload JSON simpler than Godot resource management

## Active Work Areas

### 1. Python AI Pipeline (`ai/` directory)
**Status**: Core structure complete, needs schema implementation

**Files**:
- `ai.py`: Main orchestrator with 6 commands (canonize, plan, expand, image-plan, images, narrative)
- `schemas.py`: Pydantic models for all game data structures
- `prompts.py`: System prompts for creative and image generation
- `images.py`: DALL-E integration for tileset generation
- `dialogue.py`: NPC dialogue and questline expansion
- `atlas.py`: Atlas/map generation utilities
- `util.py`: Helper functions and path management

**Workflow**:
1. Edit markdown in `content/` (Architecture.md, Themes.md)
2. Run `python -m ai canonize` to convert to canon.json
3. Run `python -m ai plan` to generate world plan
4. Run `python -m ai expand` to create region details
5. Run `python -m ai image-plan` to design assets
6. Run `python -m ai images` to generate tilesets
7. Run `python -m ai narrative` to expand dialogue

### 2. Rust/Bevy Game (`apps/game/` and `crates/world/`)
**Status**: Basic systems working, needs feature completion

**Core Systems**:
- **World Plugin**: Main game logic orchestrator
- **Hex Movement**: Q/W/E/A/S/D navigation implemented
- **Hot Reload**: R key reloads worldbook.json
- **Shop System**: T key opens/closes shop UI
- **Dungeon System**: Enter/Esc for dungeon transitions
- **Lighting**: Ambient light cycles over time

**Architecture**:
```rust
// Main game loop
App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(Material2dPlugin::<HexTileMaterial>)
    .add_plugins(WorldPlugin)
    .add_systems(Startup, setup_camera)
    .add_systems(Startup, load_worldbook)
    .run()
```

**Key Components** (`crates/world/src/`):
- `plugin.rs`: Main world plugin definition
- `resources.rs`: WorldBook, PlayerState, Lighting, GameMode
- `systems/`: Movement, encounters, shops, dungeons, lighting
- `components.rs`: ECS components for entities
- `hex.rs`: Hex grid math and utilities
- `material.rs`: Shader materials for hex tiles

### 3. Content Structure (`build/` directory)
**Generated Files**:
- `build/master/canon.json`: Core game rules from Architecture.md
- `build/master/themes.json`: Art bible from Themes.md
- `build/world/plan.json`: High-level world structure
- `build/world/region_*.json`: Detailed region data
- `build/world/worldbook.json`: Complete world data (loaded by game)
- `build/features/shops/*.json`: Shop inventories
- `build/narrative/`: Dialogue and questline bundles

## Immediate Tasks

### Critical Path to Playable
1. **Complete Schema Definitions** (`ai/schemas.py`)
   - [ ] GameCanon, ThemeBible, WorldPlan models
   - [ ] RegionBible, WorldBook, ImagePlan models
   - [ ] BiomeTileset, TileVariant, IconJob models

2. **Implement Image Generation** (`ai/images.py`)
   - [ ] DALL-E API integration
   - [ ] Tileset generation for each biome
   - [ ] POI icon generation
   - [ ] Asset organization in game directory

3. **Core Game Mechanics** (`crates/world/`)
   - [ ] Combat system with inverted economy
   - [ ] Companion trauma tracking
   - [ ] Forge redemption mechanic
   - [ ] Band-based progression gates

4. **Content Generation**
   - [ ] Run full pipeline for all 5 bands
   - [ ] Generate all biome tilesets
   - [ ] Create complete NPC dialogue
   - [ ] Design signature encounters

## Current Session Focus

### What We Just Did
- Fixed git remote from godot-open-rpg to dragons-labyrinth
- Reviewed entire new codebase structure
- Understood pivot from Godot to Rust/Bevy
- Started rewriting memory-bank for new architecture

### What We're Doing Now
- Completing memory-bank rewrite
- Documenting new technical architecture
- Preparing for force push to correct repo

### Next Immediate Steps
1. Finish rewriting remaining memory-bank files
2. Force push to git@github.com:jbcom/dragons-labyrinth.git
3. Begin implementing missing schemas
4. Test full AI generation pipeline
5. Polish core game mechanics

## Key Decisions Made

### Architecture Choices
- **No Database**: Direct JSON loading simpler and faster
- **ECS Pattern**: Better for component-heavy game design
- **Hot Reload**: Rapid iteration via R key
- **AI Pipeline**: Markdown source of truth for all content

### Design Principles
- **Content First**: Markdown drives everything
- **Simple Tools**: Each script does one thing well
- **Fast Iteration**: Hot reload for instant testing
- **Clear Separation**: AI generation vs game runtime

## Known Issues

### Technical Debt
- Missing schema implementations in Python
- Image generation not connected
- Combat system not implemented
- Companion system placeholder only

### Content Gaps
- No generated tilesets yet
- Missing NPC dialogue
- Questlines not expanded
- Encounters undefined

## Success Metrics

### Short Term (This Week)
- [ ] Complete AI pipeline can generate full world
- [ ] Game loads and displays generated world
- [ ] Player can navigate all hex tiles
- [ ] Basic combat encounter triggers

### Medium Term (This Month)
- [ ] All 5 progression bands playable
- [ ] Companion trauma system working
- [ ] Forge mechanic implemented
- [ ] First 60 levels polished

### Long Term (Launch)
- [ ] Complete 180-level experience
- [ ] All biomes and POIs rendered
- [ ] Full narrative content
- [ ] Performance optimized
