# Active Context for Dragon's Labyrinth

## Current Work Status: READY FOR GAME-DATABASE MIGRATION

### PARADIGM SHIFT COMPLETED ✅
**CRITICAL INSIGHT:** "What I am realizing is it's the wrong sort of randomization. Which is to say there is MORE that we can LEARN from it as to how WE could actually do exactly the same goddamn thing but from the start sited to OUR needs using OpenAI. I mean literally we're about to run like what, 500 different AI queries to parse THEIR stuff over to OUR stuff. So that we can use their stuff. But it's a bit goofy because we HAVE a design that's actually SIMPLER."

**PARADIGM SHIFT COMPLETED:**
- OLD: Transform 70k HBF entities into our game (complex, forced)
- NEW: Use HBF features.json as organizational templates to generate our own content (optimal)

### Infrastructure Setup Complete ✅

**Langchain/LangGraph Stack Added:**
- ✅ Complete langchain/langgraph stack in pyproject.toml
- ✅ Modernized types.py with professor-pixels standards (Type | None, auto() enums)
- ✅ Enhanced models.py with sophisticated workflow state models
- ✅ Built agent.py with sophisticated workflow orchestration
- ✅ Human-in-the-loop with structured review checkpoints
- ✅ SQLite checkpointing for durable execution and workflow resumption
- ✅ NetworkX graphs, vector stores, memory systems

### Game-Database Discovery ✅

**CRITICAL REALIZATION:** `crates/game-database` contains **2+ years of sophisticated horror RPG logic** ready to port!

**Complete Systems Available:**
- ✅ **20+ sophisticated models**: hex tiles, companions, corruption, forge, weather, etc.
- ✅ **Complete ECS systems** with horror integration and dread progression
- ✅ **Production-ready Bevy ECS integration** (bevy_integration.rs)
- ✅ **Third-party library integration**: hexx (hex math), bevy_ecs_tilemap (rendering), bevy_hanabi (particles), bevy_kira_audio (spatial audio)
- ✅ **Sophisticated horror progression logic** that transforms all systems

### Layer Cake Tile System Architecture ✅

**Revolutionary Simplification - No villages/cities/taverns needed:**
- **Tile**: Base hex coordinate container  
- **Biome**: Base layer (grassland, forest, lava) with adjacency rules
- **Path**: Transparent overlay (roads, bridges) for connections
- **Feature**: Interactive overlay (taverns, dungeons, shrines) for content

**Perfect Integration:**
- ✅ **Hex tile template** (`crates/blender-bridge/templates/hex_tile.py.j2`) supports layer cake
- ✅ **Features.json organizational patterns** teach perfect D&D content structure
- ✅ **FINAL-REVELATION infinite hex map** architecture ready
- ✅ **Biome adjacency rules** prevent lava next to snow

### IMMEDIATE NEXT PHASE: Game-Database Migration

**USER DIRECTIVE:** "finish updating core memory bank documentation comprehensively, and then git add all, git commit, and setup a new_task to get everything started by getting as much moved OUT of game-database and refactored to directly use Bevy ECS as possible"

**MIGRATION STRATEGY:**
1. **Port sophisticated systems** from game-database to game-engine (preserve 2+ years of logic)
2. **Remove SeaORM dependencies** while keeping all game logic 
3. **Convert models** to pure Bevy components
4. **Simplify settlement hierarchies** to feature overlay system
5. **Add AI content generation** workflows using features.json patterns
6. **Create asset library mirror** for AI discovery
7. **Implement layer cake tile system** with biome adjacency rules

**KEY INSIGHT:** We're **80% done** - just need to liberate existing logic from database and add infinite generation!

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
