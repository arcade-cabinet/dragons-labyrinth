# Dragon's Labyrinth

A horror-first RPG built with Bevy 0.16.1 and Rust 1.88.

## Project Structure

This project uses a modular workspace architecture with specialized crates:

### Core Crates

- **`dragons_core`** - Shared components, resources, and types used across all crates
- **`dragons_game`** - Main game executable that orchestrates all systems

### Feature Crates

- **`dragons_ui`** - Reactive UI system using Cobweb and cobweb-ui for modern interfaces
- **`dragons_maps`** - Advanced map generation with:
  - Hexagonal grids (hexx)
  - 2D tilemaps (bevy_ecs_tilemap)
  - 3D hexagonal tiles (bevy_clay_tiles)
  - Procedural generation (mapgen)
- **`dragons_levels`** - Level editor and management using Yoleck
- **`dragons_ai`** - AI behaviors and pathfinding with big-brain
- **`dragons_audio`** - Advanced audio with bevy_kira_audio
- **`dragons_physics`** - Physics simulation using Avian (modern Bevy physics)
- **`dragons_vfx`** - Visual effects with:
  - Particle systems (Hanabi)
  - Procedural animations (MotionGfx)
  - Screen effects and transitions
- **`dragons_assets`** - Asset loading and management
- **`dragons_tools`** - Development and build tools

## Features

### 3D Hexagonal Maps
The game supports both 2D and 3D hexagonal tile generation. Toggle between modes using the `HexagonalMapConfig` resource:

```rust
hex_config.use_3d_tiles = true; // Enable 3D hexagonal tiles
```

### Advanced Physics
Using Avian physics engine for better Bevy integration with features like:
- Collision layers for different entity types
- Force accumulation system
- Kinematic and dynamic bodies
- Custom gravity zones

### Reactive UI
Cobweb-based UI system provides:
- Reactive state management
- Component-based UI architecture
- Smooth transitions and animations

### Motion Graphics
MotionGfx integration enables:
- Procedural animations
- Easing functions
- UI element animations
- Screen effects (shake, flash, fade)

## Building

Ensure you have Rust 1.88 or later installed:

```bash
rustup update
rustup default 1.88
```

Build all crates:

```bash
cargo build --all
```

Run the game:

```bash
cargo run -p dragons_labyrinth
```

Build for release:

```bash
cargo build --release
```

## Development

### Running with hot-reload

```bash
cargo watch -x "run -p dragons_labyrinth"
```

### Running tests

```bash
cargo test --all
```

### Building for WASM

```bash
cargo build --target wasm32-unknown-unknown --release
```

### Updating Dependencies

Update to latest compatible versions:
```bash
cargo update
```

Update to latest incompatible versions:
```bash
cargo upgrade --incompatible
```

## Architecture

The project follows a modular ECS (Entity Component System) architecture:

- Each crate provides a Bevy plugin that can be independently developed and tested
- Core game logic is separated from engine-specific implementations
- Systems communicate through events and shared resources
- All crates use workspace dependencies for consistency

### Key Design Patterns

1. **Plugin-based Architecture**: Each feature is a self-contained plugin
2. **Resource-driven Configuration**: Runtime configuration through resources
3. **Event-driven Communication**: Loose coupling between systems
4. **Component Composition**: Entities built from reusable components

## Dependencies

Key technologies:
- **Bevy 0.16.1** - Game engine
- **Avian** - Modern physics engine for Bevy
- **Cobweb** - Reactive UI framework
- **Yoleck** - Level editor
- **Hanabi** - Particle systems
- **MotionGfx** - Procedural animations and motion graphics
- **bevy_kira_audio** - Advanced audio
- **hexx** - Hexagonal grids
- **bevy_clay_tiles** - 3D tile generation
- **big-brain** - AI behavior trees
- **mapgen** - Procedural map generation

## Project Status

The project is structured with Rust edition 2024 and uses the latest stable Rust toolchain (1.88). All dependencies have been upgraded to their latest versions for maximum compatibility and performance.

## License

MIT OR Apache-2.0