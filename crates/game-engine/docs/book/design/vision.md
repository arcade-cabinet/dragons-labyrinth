# Dragon's Labyrinth - Project Brief

## Vision Statement
Dragon's Labyrinth is a **horror-first RPG** disguised as an adventure game. Players begin by opening their front door on a beautiful morning and end in first-person darkness, hunted by an ancient intelligence. This is not an RPG with horror elements - it's a horror experience that uses RPG mechanics to deliver its narrative.

## ARCHITECTURAL REVOLUTION (January 2025)

### The Clean Separation Principle
**Build-Time AI Generation (Python) ↔ Runtime Game Engine (Rust)**

- **Python Agentic Workflows**: Sophisticated AI agents using LangGraph + SQLite for durable content generation
- **Rust Game Engine**: Pure runtime consumption of generated assets with zero AI generation code
- **Database-Driven Asset Selection**: AI searches CC0 library before generating, achieving 80% reuse + 20% targeted generation

### Three-Tier Asset Strategy (REVOLUTIONARY)
1. **Core Assets (Sacred)**: Hand-crafted intro/outro videos, signature sounds - AI never touches
2. **Library Assets (CC0 Collection)**: Existing medieval props, textures, sounds - AI searches and uses intelligently  
3. **Generated Assets (AI Fills Gaps)**: Horror corruption variants, companion trauma states, dread-responsive UI

## Core Requirements

### Technical Foundation
- **Platform**: Rust with Bevy 0.16.1 game engine
- **Architecture**: Entity Component System (ECS) with <100 line systems
- **Deployment**: Native + WebAssembly for web/mobile
- **Performance**: 60 FPS with 10,000+ hex tiles, 30 FPS mobile

### Revolutionary Asset Strategy
- **Smart Asset Reuse**: 80% CC0 library leverage + 20% targeted AI generation
- **Agentic Workflows**: 5 specialized AI agents (Maps, Levels, UI, Dialogue, Audio)
- **Database-First Selection**: Semantic search of existing assets before AI generation
- **Human-in-the-Loop**: Review and approval workflows built into each agent
- **Idempotent Generation**: Same inputs produce identical outputs with SQLite checkpointing
- **Resource Efficiency**: Massive cost savings through intelligent asset reuse

### Specialized AI Agent Architecture
- **MapsAgent**: Hexx integration, hex world generation, biome corruption progression
- **LevelsAgent**: Yoleck integration, encounter placement, interactive object positioning
- **UIAgent**: Cobweb integration, horror-responsive interface degradation
- **DialogueAgent**: YarnSpinner integration, companion arcs, moral choice branching
- **AudioAgent**: Freesound integration, proximity horror, spatial audio systems

### Gameplay Architecture
- **Hex Grid World**: Hexx crate for pathfinding and navigation
- **Board Rendering**: Beauty textures with splatmaps and control maps
- **Dialogue System**: YarnSpinner for narrative-heavy interactions (bevy_yarnspinner crate)
- **Level Editing**: Yoleck for in-engine content creation and encounter placement
- **UI Framework**: Cobweb UI for declarative, horror-responsive interface design

## Narrative Structure (Enhanced with AI Generation)

### Emotional Progression
**Peace → Unease → Dread → Terror → Horror**

Each stage fundamentally transforms all game systems through AI-generated variants:
- **Stage 0 (Peace)**: Beautiful world, helpful NPCs, bright textures from CC0 library
- **Stage 1 (Unease)**: AI-generated shadow overlays, whispered audio, Hollow Caretaker boss
- **Stage 2 (Dread)**: AI corruption masks on existing swamp assets, economic collapse dialogs
- **Stage 3 (Terror)**: Reality distortion shaders, companion betrayal voice lines, moral horror UI
- **Stage 4 (Horror)**: Complete AI transformation - dragon stalking, first-person nightmare mode

### Companion Arcs (AI-Enhanced Progression)
- **Einar**: Loyal friend with AI-generated trauma progression states (visual + audio)
- **Mira**: Optimist with AI-generated departure dialogue and emotional voice variants
- **Sorin**: Scholar with AI-generated betrayal path and corrupted academic appearance
- **Tamara**: Baker's apprentice with AI-generated innocence loss visual progression

### Boss Encounters (AI-Generated Moral Complexity)
Each boss offers AI-generated meaningful moral choices:
- **Empathy vs brutality**: Dynamic dialogue trees generated per player history
- **Forgiveness vs execution**: AI-generated consequence visualizations
- **Understanding vs destruction**: Companion reaction variants based on trauma levels

Choices influence through AI systems:
- **Companion morale**: AI-generated trauma response animations
- **Available endings**: AI-generated narrative path variations
- **Dragon proximity**: AI-generated escalating dread audio/visual effects

## Success Criteria

### Technical Metrics (Maintained)
- ✅ 60 FPS with 10,000+ rendered hex tiles
- ✅ < 200MB memory usage for full game  
- ✅ < 2 second area load times
- ✅ 30 FPS on mid-range mobile devices
- ✅ No memory leaks or garbage collection issues

### Narrative Metrics (AI-Enhanced)
- **Horror progression**: Feels inevitable through AI-generated environmental storytelling
- **Companion attachment**: AI-generated dialogue creates genuine emotional bonds
- **Moral dilemmas**: AI-generated choice consequences feel weighty and permanent
- **Ending satisfaction**: AI-generated narrative conclusions reflect player journey
- **Dragon presence**: AI-generated proximity audio creates authentic dread

### Content Metrics (Revolutionary Efficiency)
- **80% Asset Reuse**: Professional CC0 library assets used intelligently
- **20% AI Generation**: Targeted horror variants and game-specific content
- **Idempotent Results**: Deterministic generation with SQLite checkpointing
- **Component Systems**: <100 lines each, AI-generated content consumed cleanly
- **Zero Embedded Generators**: Complete separation of build-time vs runtime
- **Mobile Performance**: Direct GLTF loading, optimized asset selection

## Project Scope (Updated)

### In Scope
- **Complete 5-stage horror progression** with AI-generated corruption variants
- **4 companion characters** with AI-generated trauma progression arcs
- **3 major boss encounters + dragon finale** with AI-generated moral choice systems
- **Hex-based world** with AI-enhanced biome corruption (Hexx + MapsAgent)
- **Proximity horror audio** system with AI-generated spatial intensity
- **Sanity/hallucination mechanics** with AI-generated false audio/visual effects
- **Multiple endings** with AI-generated narrative path variations

### Enhanced Scope (Agentic AI)
- **Durable workflow execution** with LangGraph checkpointing and recovery
- **Human-in-the-loop review** for all AI-generated content approval
- **Database-driven asset selection** with semantic search and reuse optimization  
- **Batch 3D asset generation** using Blender-MCP job queue patterns
- **Cross-agent coordination** for consistent horror progression across all systems

### Out of Scope (Maintained)
- Multiplayer functionality
- User-generated content tools  
- Voice acting (text-based dialogue with AI-generated emotional variants)
- Procedural quest generation (narrative is hand-crafted, enhanced by AI)
- Save game cloud sync
- Achievement system

## Development Philosophy (Revolutionary Update)

### Horror-First Principle (AI-Enhanced)
Every feature, system, and asset must serve the horror narrative. AI agents understand horror progression and generate content that enhances the growing dread and emotional journey. If AI-generated content doesn't serve the horror experience, it gets rejected in human-in-the-loop review.

### Component Independence (Maintained)
Each system works standalone and in combination. No system should require another to function, only enhance when combined. AI-generated content is consumed through clean interfaces, maintaining system independence.

### Performance by Design (Enhanced)
Mobile optimization isn't an afterthought - every architectural decision considers mobile performance from the start. AI asset selection prioritizes performance-optimized CC0 assets, with targeted generation only when necessary.

### Narrative as Orchestrator (AI-Amplified)
The dread level (0-4) is the master orchestrator. All AI agents are dread-aware and generate content that responds to and amplifies the current narrative stage. The horror progression drives every generation decision.

### Resource Intelligence Principle (NEW)
AI agents search existing CC0 assets before generating new content. Professional quality assets are reused intelligently, with AI generation focused on horror-specific variants and game-unique elements. This achieves massive cost savings while maintaining or improving quality.

### Clean Separation Doctrine (CRITICAL)
Build-time AI generation is completely separate from runtime game logic. Python agentic workflows handle all content creation, Rust game engine handles pure performance and gameplay. This eliminates the architectural mistakes of embedded generators and creates a maintainable, scalable system.

## Production Timeline (3 Weeks to Revolutionary Game)

### Week 1: Architectural Foundation
- **Clean Separation**: Remove all embedded generators, create pure Rust runtime
- **Asset Database**: Index CC0 library with semantic search capabilities
- **Agentic Framework**: Implement LangGraph workflows with SQLite checkpointing

### Week 2: Specialized AI Agents
- **Domain Agents**: Deploy Maps, Levels, UI, Dialogue, Audio agents
- **Integration Testing**: Validate all Bevy ecosystem crate compatibility
- **First Stage**: Complete Peace → Unease progression with AI enhancement

### Week 3: Production Polish
- **Complete Generation**: All 5 dread stages with AI-enhanced horror progression
- **Performance Validation**: 60 FPS with complete AI-generated + library assets
- **Production Deploy**: WebAssembly build with zero placeholder content

## Risk Mitigation

### Technical Risks (Addressed)
- **AI Generation Reliability**: Human-in-the-loop review prevents poor quality output
- **Performance Impact**: Database-first asset selection ensures optimized content
- **Integration Complexity**: Clean separation simplifies Python/Rust coordination

### Content Risks (Mitigated)
- **Asset Quality Consistency**: CC0 library provides professional baseline quality
- **Horror Effectiveness**: Specialized agents understand horror progression requirements
- **Narrative Coherence**: AI agents coordinate through shared dread progression context

### Timeline Risks (Managed)
- **Workflow Durability**: LangGraph checkpointing enables recovery from failures
- **Parallelization**: Multiple specialized agents can work simultaneously
- **Fallback Strategy**: CC0 library provides backup content if AI generation fails

This architectural revolution transforms Dragon's Labyrinth from a traditionally developed game into an AI-enhanced horror experience that intelligently leverages existing assets while generating precisely targeted content for maximum emotional impact.
