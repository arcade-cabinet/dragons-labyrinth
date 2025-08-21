# Dragon's Labyrinth

A horror-first RPG built with Bevy 0.16.1 and Rust 1.88.

## Project Structure

This project uses a modular workspace architecture with specialized crates:

### Core Crates

- **`dragons_core`** - Shared components, resources, and types used across all crates
- **`dragons_game`** - Main game executable that orchestrates all systems

### Feature Crates

- **`dragons_ui`** - UI system using Cobweb for reactive interfaces
- **`dragons_maps`** - Map generation with hex grids (hexx), tilemaps (bevy_ecs_tilemap), and procedural generation (mapgen)
- **`dragons_levels`** - Level editor and management using Yoleck
- **`dragons_ai`** - AI behaviors and pathfinding with big-brain
- **`dragons_audio`** - Advanced audio with bevy_kira_audio
- **`dragons_physics`** - Physics simulation using Avian (formerly bevy_xpbd)
- **`dragons_vfx`** - Visual effects with Hanabi particles and MotionGfx
- **`dragons_assets`** - Asset loading and management
- **`dragons_tools`** - Development and build tools

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

## Architecture

The project follows a modular ECS (Entity Component System) architecture:

- Each crate provides a Bevy plugin that can be independently developed and tested
- Core game logic is separated from engine-specific implementations
- Systems communicate through events and shared resources
- All crates use workspace dependencies for consistency

## Dependencies

Key technologies:
- **Bevy 0.16.1** - Game engine
- **Avian** - Physics engine
- **Cobweb** - Reactive UI framework
- **Yoleck** - Level editor
- **Hanabi** - Particle systems
- **MotionGfx** - Procedural animations
- **bevy_kira_audio** - Advanced audio
- **hexx** - Hexagonal grids
- **big-brain** - AI behavior trees

## License

MIT OR Apache-2.0