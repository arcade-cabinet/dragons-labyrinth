# Game Engine

The main game engine crate for Dragon's Labyrinth - a horror-first RPG built with Bevy.

## Overview

This crate contains the complete runtime game engine, merging all gameplay systems into a unified architecture. It consumes AI-generated assets from the build process but contains NO generation code itself, maintaining clean separation between build-time and runtime.

## Architecture

```
game-engine/
├── src/
│   ├── main.rs           # Entry point
│   ├── lib.rs            # Library exports
│   ├── components/       # ECS components (merged from core)
│   ├── resources/        # Global game resources
│   ├── systems/          # Game systems (NO generators)
│   ├── ai/              # NPC AI behaviors
│   ├── audio/           # Spatial audio systems
│   ├── levels/          # Level management
│   ├── maps/            # Hex world systems
│   ├── physics/         # Physics integration
│   ├── save/            # Save game systems
│   ├── ui/              # Horror-responsive UI
│   └── vfx/             # Visual effects
```

## Features

- **Horror Progression**: 5-stage emotional journey from Peace to Horror
- **Companion System**: 4 companions with trauma and betrayal mechanics
- **Hex-Based World**: Using Hexx for pathfinding and navigation
- **Proximity Horror**: Dragon stalking with spatial audio
- **Moral Choices**: Decisions that affect companions and endings
- **First-Person Mode**: Perspective shift in the labyrinth finale

## Dependencies

- **Bevy 0.16.1**: Core game engine
- **Hexx**: Hexagonal grid mathematics
- **bevy_ecs_tilemap**: Efficient tile rendering
- **avian3d/avian2d**: Physics systems
- **bevy_hanabi**: Particle effects
- **bevy_kira_audio**: Advanced audio

## Usage

### Running the Game

```bash
# Debug mode
cargo run

# Release mode
cargo run --release

# WebAssembly build
./build_wasm.sh
```

### Architecture Principles

1. **NO Embedded Generators**: All assets are consumed, never generated at runtime
2. **Component Independence**: Each system works standalone
3. **Dread-Driven**: All systems respond to horror progression
4. **Performance First**: Mobile-optimized from the start

## Horror Progression

The game progresses through 5 dread levels:

- **0 - Peace**: Beautiful world, helpful NPCs
- **1 - Unease**: Subtle wrongness, shadows lengthening
- **2 - Dread**: Visible corruption, companions stressed
- **3 - Terror**: Reality breaking, betrayals begin
- **4 - Horror**: Dragon hunting, first-person nightmare

## Integration

This crate integrates with:
- `game-database`: For persistent game state
- `assets-library`: For asset loading and management
