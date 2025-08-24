# Assets Library

Asset loading and management system for Dragon's Labyrinth.

## Overview

This crate provides a unified interface for loading and managing the three-tier asset system:
1. **Core Assets**: Sacred, hand-crafted assets (intro/outro, signature sounds)
2. **Library Assets**: CC0 collection intelligently selected by AI
3. **Generated Assets**: AI-created content filling specific gaps

## Architecture

```
assets-library/
├── core/           # Sacred assets (AI never touches)
├── library/        # CC0 collection (AI searches and selects)
├── generated/      # AI-generated content (fills gaps)
└── src/
    ├── lib.rs      # Public API
    ├── loader.rs   # Asset loading systems
    ├── cache.rs    # Asset caching
    └── metadata.rs # Asset metadata management
```

## Features

- **Database-Driven Selection**: AI searches existing assets before generating
- **Idempotent Loading**: Same inputs produce same asset references
- **Performance Optimized**: Smart caching and streaming
- **Horror-Aware**: Assets respond to dread level progression

## Asset Categories

### Core Assets (Sacred)
- Intro/outro videos
- Dragon roar
- Signature theme music
- Key story moments

### Library Assets (CC0)
- Medieval props and textures
- Environmental sounds
- Character models
- UI elements

### Generated Assets (AI)
- Horror corruption variants
- Companion trauma states
- Dread-responsive UI
- Proximity audio

## Usage

```rust
use assets_library::{AssetLibrary, AssetQuery};

// Initialize library
let library = AssetLibrary::new("assets/")?;

// Query assets by dread level
let hex_tiles = library.query_tiles(dread_level)?;

// Load specific asset
let dragon_roar = library.load_core("audio/dragon_roar.ogg")?;
```

## Build Process

The `build.rs` script:
1. Indexes all CC0 library assets
2. Creates SQLite database of metadata
3. Generates Rust code for compile-time asset references
4. Validates asset integrity

## Integration

- Works with `build-tools` for AI asset generation
- Provides assets to `game-engine` for runtime consumption
- Uses `game-database` for asset metadata storage
