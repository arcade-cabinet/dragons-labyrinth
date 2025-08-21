# Dragon's Labyrinth - Horror-First RPG

## Overview
Dragon's Labyrinth is a complete horror RPG that follows the emotional arc Peace → Unease → Dread → Terror → Horror. The game begins as a bright adventure with the player delivering bread, but gradually transforms into psychological horror culminating in a confrontation with an ancient dragon in its labyrinth.

## Project Architecture

### Core Structure
- **CLI Interface**: Python-only toolchain (no external dependencies, stdlib only)
- **Component-based**: Each system <100 lines, clear I/O, no cross-coupling
- **Idempotent generation**: Deterministic IDs, stable APIs, reproducible outputs
- **Performance-first**: 60 FPS desktop, 30 FPS mobile target

### Output Structure
```
/generated_game/
  ├── project.godot
  ├── scenes/
  ├── scripts/
  ├── assets/
  │   ├── models/     # .glb files with vertex colors
  │   ├── audio/      # .ogg files from generation
  │   └── textures/   # SVG-based textures
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

## Recent Changes
- **Initial Setup (2025-01-21)**: Received complete handoff documentation including biomes, companions, monsters, quests, and Blender scripts
- **Architecture Planning**: Established focus on Dragon's Labyrinth as primary deliverable, not generation tools

## Implementation Status
- [ ] Core CLI structure
- [ ] Godot project generation
- [ ] Component systems (biomes, companions, quests, monsters)
- [ ] Asset generation pipeline
- [ ] Stage progression mechanics
- [ ] Audio system integration
- [ ] Performance optimization