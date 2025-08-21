# Dragon's Labyrinth - Horror-First RPG

## Overview
Dragon's Labyrinth is a complete horror RPG that follows the emotional arc Peace → Unease → Dread → Terror → Horror. The game begins as a bright adventure with the player delivering bread, but gradually transforms into psychological horror culminating in a confrontation with an ancient dragon in its labyrinth.

## Project Architecture

### Core Structure
- **CLI Interface**: Python-only toolchain (no external dependencies, stdlib only)
- **Component-based**: Each system <100 lines, clear I/O, no cross-coupling
- **Idempotent generation**: Deterministic IDs, stable APIs, reproducible outputs
- **Performance Limitation**: JavaScript/WebGL fundamentally insufficient for complex 3D with many models

### Performance Architecture Issue
Current React/Three.js implementation faces fundamental limitations:
- Interpreted JavaScript cannot handle complex 3D model management efficiently
- Garbage collection causes memory leaks with frequent model instantiation
- WebGL lacks direct memory control needed for mobile 3D performance
- **Solution**: Migrate to Bevy Engine (Rust) with WebAssembly deployment for proper performance

### Output Structure
```
/generated_game/
  ├── Cargo.toml      # Bevy project configuration
  ├── src/
  │   ├── main.rs     # Entry point
  │   ├── systems/    # ECS systems
  │   ├── components/ # Game components
  │   └── resources/  # Game resources
  ├── assets/
  │   ├── models/     # .glb files with vertex colors
  │   ├── audio/      # .ogg files from generation
  │   └── textures/   # Generated textures
  └── metadata/       # JSON metadata for all generated content
```

### Systems Integration
- **Narrative Orchestration**: All systems respond to dread level (0-4)
- **Sanity System**: Hallucinations and false audio cues
- **Proximity Horror**: Dragon presence through sound/environment
- **Choice Consequences**: Boss encounters influence endings

## Key Components

### Stage Progression
1. **Peace (0)**: Bright world, mundane quests, friendly NPCs
2. **Unease (1)**: Shadows, whispers, Hollow Caretaker boss
3. **Dread (2)**: Swamps, ruins, economy collapse, Forsaken Knight boss
4. **Terror (3)**: Reality warps, companion betrayal, moral horrors
5. **Horror (4)**: Dragon's labyrinth, stalking mechanics, final choice

### Companions
- **Einar**: Loyal friend who breaks under pressure
- **Mira**: Optimist who abandons party in Dread stage
- **Sorin**: Scholar who becomes traitor boss if not handled properly
- **Tamara**: Innocent baker's apprentice, represents lost innocence

### Boss Encounters
- Each boss offers meaningful choices (empathy vs brutality, forgiveness vs execution)
- Choices influence companion morale, available endings, and dragon proximity
- Final dragon encounter has three endings: Acceptance, Defiance, Understanding

## User Preferences
- Focus on complete game implementation over tooling
- Component-based architecture with clear separation
- Performance optimization for smooth gameplay
- Rich narrative integration across all systems
- **Documentation Integration**: All project documentation must be properly organized and accessible for reference during implementation
- **Design Bible Compliance**: All implementation must follow the design bible principles exactly
- **Bevy/Rust Architecture**: Migration from JavaScript to compiled Rust for proper performance

## Recent Changes
- **Initial Setup (2025-01-21)**: Received complete handoff documentation including biomes, companions, monsters, quests, and Blender scripts
- **Architecture Planning**: Established focus on Dragon's Labyrinth as primary deliverable, not generation tools
- **Movement System (2025-01-24)**: Replaced WASD controls with tap-to-move pathfinding for better mobile compatibility and modern gameplay
- **Hexagonal Grid**: Implemented proper hexagonal world with 6-directional movement ("hexagons are the bestagons")
- **Visual Improvements**: Added chess piece-style character sprites and stage-based world corruption effects
- **Performance Issue Identified (2025-01-21)**: JavaScript/React/Three.js architecture fundamentally insufficient for complex 3D game with multiple models due to interpreted language limitations and garbage collection
- **Architecture Migration (2025-01-21)**: Transitioning to Bevy Engine (Rust) with WebAssembly deployment for proper 2.5D performance and memory management
- **Design Bible Integration**: Aligning with zero dependencies, idempotent generation, and component-based architecture principles from project design bible
- **Build System Setup (2025-01-21)**: Created build_tools crate with AI-powered content generators using openai_dive and freesound-rs for structured output generation
- **Asset Pipeline**: Established idempotent build system with caching and automatic generation of .cob UI files, .yol level files, and ECS prefabs
- **Rust Upgrade (2025-08-21)**: Successfully upgraded from Rust 1.77.2 to Rust 1.88.0 for modern Bevy 0.16.1 compatibility
- **Bevy Migration**: Updated to Bevy 0.16.1 with latest features and performance improvements
- **Modern Ecosystem**: Using latest versions of hexx 0.21, dashmap 6.1, and other Rust crates for optimal performance
- **Map Generation Integration (2025-08-21)**: Added mapgen 0.6 and bevy_ecs_tilemap 0.16 for superior procedural map generation with algorithms like BSP, cellular automata, and drunkard's walk
- **Dual Database Architecture (2025-08-21)**: Implemented SeaORM with dual-database system - game.db for distributed ECS content (read-only) and player.db stored in XDG directories for player saves (read-write)
- **FMV Cutscenes Added (2025-08-21)**: Integrated emotionally-driven FMV cutscenes (villager-intro.mp4, traveler-portal.mp4) that create the transition from first-person 3D to top-down 2.5D gameplay, establishing the "wonderland" horror atmosphere

## Implementation Status
- [x] Core Bevy ECS architecture setup
- [x] Design bible integration and documentation structure
- [x] Component definitions (HexTile, Companion, Player, Quest, NPC)
- [x] Resource management (DreadState, HexWorld, NarrativeState)
- [x] Core systems (DreadProgression, CompanionTrauma, WorldCorruption)
- [x] Dual-database architecture (game.db for content, player.db for saves)
- [x] FMV cutscene system for emotional intro/outro transitions
- [ ] Asset generation pipeline integration
- [ ] Idempotent generation system
- [ ] Audio system with Freesound integration
- [ ] Performance targets validation (60 FPS, <200MB memory)
- [ ] WebAssembly deployment pipeline

## Documentation Structure
- `docs/design_bible.md` - Complete project design bible
- `docs/biomes_reference.md` - Environmental system specifications
- `docs/companions_reference.md` - Character system details
- `docs/implementation_checklist.md` - Development progress tracking
- `docs/technical_architecture.md` - Specialized crate integration and board rendering
- `bevy_migration_plan.md` - Architecture transition planning

## Specialized Crate Integration
- **Hexx**: Hex grid system with A* pathfinding, FOV, and navigation
- **Yoleck**: Level editor with JSON .yol files for AI-generated content
- **Yarn Spinner**: Dialogue system with .yarn files and dread progression
- **Cobweb UI**: Declarative UI with .cob scene format for AI-generated interfaces