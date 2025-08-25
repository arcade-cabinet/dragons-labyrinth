# Active Context for Dragon's Labyrinth

## Current Work Status: MAJOR ARCHITECTURAL BREAKTHROUGH

### HBF Paradigm Shift Completed ✅
**CRITICAL DISCOVERY:** After comprehensive analysis of professor-pixels langchain/langgraph architecture and features.json content patterns, we've achieved a complete paradigm shift:

**OLD APPROACH (ABANDONED):**
- Transform 70k HBF entities into our game
- Parse complex HTML relationships  
- Force their content into our systems

**NEW APPROACH (IMPLEMENTED):**
- Use HBF features.json as organizational templates
- Generate our own content with AI workflows
- Perfect integration with horror RPG systems

### Infrastructure Setup Complete ✅

**Langchain/LangGraph Integration:**
- ✅ Added complete langchain stack to pyproject.toml
- ✅ Modernized types.py with professor-pixels standards
- ✅ Enhanced models.py with sophisticated workflow models
- ✅ Built agent.py with durable workflow orchestration
- ✅ Human-in-the-loop with structured review checkpoints
- ✅ SQLite checkpointing for workflow resumption
- ✅ Memory systems with NetworkX graphs

**Standards Alignment:**
- ✅ Type | None (not Optional[Type])
- ✅ list[Type] (not List[Type])  
- ✅ dict[K,V] (not Dict[K,V])
- ✅ auto() enum values (not string values)
- ✅ Field(description="...") for all Pydantic fields
- ✅ ConfigDict usage (not Config class)

### Game-Database System Discovery ✅

**CRITICAL REALIZATION:** `crates/game-database` contains **2+ years of sophisticated horror RPG logic** - NOT a simple ORM layer!

**Complete Systems Ready to Port:**
- ✅ **Hex rendering system** with corruption/dread integration
- ✅ **Companion psychology** with trauma processing and therapy
- ✅ **Dread progression** master orchestrator (transforms all systems)
- ✅ **Forge system** with light/dark paths and sentimental items
- ✅ **Weather/corruption/combat** systems with horror integration
- ✅ **Complete Bevy ECS architecture** with event-driven sync
- ✅ **Third-party integration** (hexx, bevy_ecs_tilemap, bevy_hanabi, etc.)

### Layer Cake Tile System Designed ✅

**Revolutionary Simplification:**
- **Tile**: 6-sided container with coordinates
- **Biome**: Base layer (grassland, forest, lava) with gameplay effects
- **Path**: Transparent overlay (wooden planks, stone roads) for connections
- **Feature**: Interactive overlay (tavern, dungeon, shrine) for content

**Perfect Integration:**
- ✅ **Hex tile template** already supports layer cake rendering
- ✅ **Biome adjacency rules** for smart terrain generation
- ✅ **Features.json patterns** for AI content generation
- ✅ **FINAL-REVELATION infinite map** architecture

### Next Major Phase: Game-Database Migration

**Migration Strategy:**
1. **Port sophisticated systems** from game-database to game-engine
2. **Remove SeaORM dependencies** (keep all game logic)
3. **Convert models** to pure Bevy components
4. **Simplify settlement hierarchies** to feature overlays
5. **Add AI content generation** workflows using features.json patterns
6. **Create asset library mirror** for AI discovery

**Key Insight:** We're **80% done** - just need to liberate existing logic from database and add infinite generation!

## Recent Changes & Learnings

### Python 3.13 Compatibility
- Fixed xdg_base_dirs usage (functions need to be called)
- Fixed audioop removal by installing audioop-lts package
- PyDub now working correctly with full audio processing capabilities
- PyOgg works but requires system Opus libraries

### Transition Design Philosophy
Each transition now:
- Tests specific philosophy (Strength/Harmony in Act 1, Light/Dark in Act 2)
- Reveals dragon truth differently based on path
- Creates permanent consequences
- Scales from solo to armies
- Has three versions (To/From/Void)

### Audio System Architecture
- Mechanical prompts drive audio generation
- AI enhances specifications with horror-aware details
- Music21 generates compositions
- Freesound provides environmental audio
- Everything exports to Godot-ready OGG format

## Important Patterns & Preferences

### Narrative Structure
- Dual paths create fundamentally different experiences
- Every choice locks out alternatives permanently
- Horror escalates through emotional phases, not just danger
- The journey IS the game - destination is anticlimactic by design

### Second Chances Philosophy (NEW)
- No permanent lockout of cool features/abilities
- Forge failure gives legendary gear, not nothing
- Mythic abilities can be earned pre-dragon, post-dragon, or on return journey
- Those who succeed early get different rewards (eternal companion)
- Creates risk/reward without permanent punishment

### Technical Patterns
- Jinja2 templates with embedded GDScript
- Trait system as connective tissue
- Idempotent generation (can regenerate without breaking saves)
- Zero external dependencies in generated code

### Design Philosophy
- Show consequences through mechanics, not cutscenes
- Make players complicit in their transformation
- Use audio to reinforce emotional journey
- Every system reinforces the horror narrative

## Current Focus
The generator is now fully operational! Successfully generated first Godot files:
- door_scene.gd (4 files) - The peaceful beginning
- peaceful_quests.gd (3 files) - Morning light mechanics
- hex_exploration attempted (needs template fix)

Next: Test generated content in Godot and refine templates based on output quality.
