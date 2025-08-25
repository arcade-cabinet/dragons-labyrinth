# Dragon's Labyrinth Game Engine - ECS Migration Complete

## Overview

This crate contains the complete Bevy ECS game engine for Dragon's Labyrinth, migrated from the game-database architecture to pure ECS components. This migration eliminates the database layer and makes components the source of truth for all game data.

## Migration Summary

### What Was Done

1. **Moved Systems & Models**: All sophisticated systems and models from `crates/game-database/src/` were moved to `crates/game-engine/src/`
2. **Refactored Components**: SeaORM models converted to pure Bevy ECS components
3. **Layer Cake Architecture**: Implemented sophisticated hex tile system with Biome → Path → Feature layers
4. **Psychology Integration**: Full companion psychology system with trauma processing and therapy mechanics
5. **Hexx Integration**: Proper integration with hexx crate for hex math and coordinates
6. **AI Asset Integration**: Components designed to work with AI-generated content variants

### Architecture Overview

```
crates/game-engine/src/
├── components/          # Pure Bevy ECS components (no SeaORM)
│   ├── hex_tiles.rs    # Layer cake hex tile system
│   ├── companions.rs   # Sophisticated psychology system  
│   ├── forge.rs        # Light/dark path mechanics
│   └── ...             # All game-database models migrated
├── systems/            # ECS systems with pure component queries
│   ├── hex_rendering/  # Layer cake rendering with bevy_ecs_tilemap
│   ├── dread_progression/ # Master horror orchestrator
│   ├── companion_psychology/ # Trauma processing & therapy
│   ├── corruption/     # World transformation mechanics
│   └── ...             # All systems migrated and refactored
├── resources/          # Global game state resources
├── integration/        # bevy_integration.rs copied from game-database
└── plugins/            # Bevy plugin organization
```

## Core Systems

### 1. Layer Cake Hex Tile System

The revolutionary tile architecture eliminates complex settlement hierarchies:

- **Tile**: Base hex coordinate container (uses hexx::Hex)
- **Biome**: Base layer with adjacency validation (no lava next to snow)
- **Path**: Transparent overlay for connections (roads, bridges)
- **Feature**: Interactive overlay for content (taverns, dungeons, shrines)

#### Usage:
```rust
// Spawn a complete layer cake hex tile
let tile_entity = commands.spawn(LayerCakeHexTileBundle::new(
    Hex::new(5, 3),
    &hex_layout,
    BiomeType::Forest
)).id();

// Query layer components separately
fn layer_cake_system(
    mut query: Query<(&HexTile, &mut Biome, &mut Path, &mut Feature, &Corruption)>
) {
    for (tile, mut biome, mut path, mut feature, corruption) in query.iter_mut() {
        // Layer cake rendering logic
        // Each layer contributes to final visual
    }
}
```

### 2. Companion Psychology System

Sophisticated trauma processing with therapy mechanics:

#### Key Components:
- `CompanionPsychology`: Core psychology state and trauma tracking
- `TraumaSources`: Detailed trauma event history and triggers
- `TherapyParticipant`: Integration with therapy system
- `PsychologicalResilience`: Recovery factors and coping mechanisms

#### Usage:
```rust
// Spawn Einar with complete psychology profile
let einar = commands.spawn(CompanionBundle::new_einar(&asset_server, Vec3::ZERO)).id();

// Query psychology state
fn trauma_processing_system(
    mut psychology_query: Query<(&mut CompanionPsychology, &TraumaSources)>,
    mut trauma_events: EventReader<TraumaEvent>,
) {
    for (mut psychology, trauma_sources) in psychology_query.iter_mut() {
        // Process trauma events and update psychology state
        for trauma in trauma_events.read() {
            psychology.trauma_level += trauma.severity;
            
            // Check for breaking point
            if psychology.trauma_level >= psychology.breaking_point {
                // Companion may leave or betray
            }
        }
    }
}
```

### 3. Dread Progression Master Orchestrator

The core horror system that transforms all other systems based on dread level (0-4):

#### Key Features:
- **Global Dread State**: Affects all systems simultaneously
- **Distance-Based**: Dread increases with distance from origin
- **AI Asset Integration**: Loads appropriate horror variants
- **System Transformation**: Modifies behavior of all game systems

#### Usage:
```rust
fn dread_orchestrator_system(
    dread_state: Res<DreadState>,
    mut tile_query: Query<(&mut DreadEffects, &Corruption)>,
    mut companion_query: Query<&mut CompanionPsychology>,
) {
    for (mut dread_effects, corruption) in tile_query.iter_mut() {
        // Update tile horror effects based on global dread
        dread_effects.dread_level = calculate_tile_dread(dread_state.level, corruption.level);
        dread_effects.visual_overlay = get_horror_overlay(dread_effects.dread_level);
    }
    
    for mut psychology in companion_query.iter_mut() {
        // Dread affects companion psychology
        psychology.isolation_tendency += dread_state.level as f32 * 0.1;
    }
}
```

### 4. Infinite World Generation

Memory-optimized chunk loading for the one-world architecture:

#### Key Components:
- `ChunkCoord`: Memory-optimized chunk coordinates
- `HexTile`: Hexx integration for proper hex math
- `ChunkManager`: Resource for loading/unloading chunks

#### Usage:
```rust
fn chunk_loading_system(
    mut chunk_manager: ResMut<ChunkManager>,
    player_query: Query<&Transform, With<Player>>,
    hex_layout: Res<HexLayout>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        let player_hex = hex_layout.world_pos_to_hex(player_transform.translation.truncate());
        let player_chunk = ChunkCoord::from_hex(player_hex, 16);
        
        // Load chunks around player
        // Unload distant chunks to save memory
    }
}
```

## AI Asset Integration

Components are designed to work seamlessly with AI-generated content:

### Dread-Level Asset Variants
```rust
#[derive(Component)]
pub struct CompanionVisuals {
    pub trauma_visuals: HashMap<u8, Handle<Scene>>, // 0-4 trauma states
    pub current_visual: Handle<Scene>,
}

// System automatically switches to appropriate trauma visual
fn visual_progression_system(
    mut companion_query: Query<(&CompanionPsychology, &mut CompanionVisuals)>
) {
    for (psychology, mut visuals) in companion_query.iter_mut() {
        let trauma_level = (psychology.trauma_level as u8).min(4);
        if let Some(trauma_visual) = visuals.trauma_visuals.get(&trauma_level) {
            visuals.current_visual = trauma_visual.clone();
        }
    }
}
```

## Performance Optimizations

### 1. Memory-Optimized Chunks
- Only loads visible hex tiles (16x16 chunks)
- LRU eviction for distant chunks
- Maximum memory limits configurable

### 2. Component Queries
- Uses `Changed<T>` filters to avoid unnecessary work
- Batched processing for large tile updates
- Efficient queries for dread progression

### 3. Asset Streaming
- Preloads current + next dread level assets
- Unloads previous dread level to save memory
- Smart caching for frequently accessed assets

## Integration with Existing Systems

### Hexx Integration
```rust
use hexx::*;

// Components use hexx::Hex directly
pub struct HexTile {
    pub coord: Hex,  // Direct hexx integration
    pub world_position: Vec3,
}

// Systems use hexx math functions
fn hex_distance_system(query: Query<&HexTile>) {
    for tile in query.iter() {
        let distance = tile.coord.distance_to(Hex::ZERO);
        // Use distance for dread calculations
    }
}
```

### Bevy ECS Tilemap Integration
- Layer cake renders through bevy_ecs_tilemap
- Base biome + path overlay + feature overlay
- Corruption overlays based on dread level

## Migration Benefits

### Before (SeaORM Database)
- Complex database queries at runtime
- ORM overhead and translation layers
- Database dependency for all game data
- Async complexity throughout codebase

### After (Pure Bevy ECS)
- Direct component queries (no database)
- Everything compiled and type-safe
- Components ARE the source of truth
- Synchronous, predictable performance

## Key Files

### Components
- `hex_tiles.rs`: Complete layer cake system with hexx integration
- `companions.rs`: Sophisticated psychology system with trauma processing
- `forge.rs`: Light/dark path mechanics and sentimental items
- All other game-database models converted to pure ECS

### Systems  
- `hex_rendering/`: Layer cake rendering with corruption overlays
- `dread_progression/`: Master horror orchestrator affecting all systems
- `companion_psychology/`: Trauma processing, therapy, and recovery
- `corruption/`: World transformation and horror progression

### Integration
- `integration/bevy_integration.rs`: Production-ready Bevy ECS integration from game-database
- Sophisticated event-driven sync between systems
- Component mapping and system registration

## Development Workflow

### Adding New Components
1. Create component in appropriate `components/` file
2. Add to relevant Bundle for easy spawning
3. Register reflection for debugging: `.register_type::<MyComponent>()`
4. Create systems that query the component

### Adding New Systems
1. Create system function with appropriate queries
2. Add to plugin in `systems/mod.rs`
3. Use `.chain()` for systems that depend on each other
4. Add events for inter-system communication

### AI Asset Integration
1. Add asset handles to visual components
2. Create variants for each dread level (0-4)
3. Systems automatically switch based on dread state
4. Preload adjacent dread levels for smooth transitions

## Testing

### Component Testing
```rust
#[test]
fn test_layer_cake_tile() {
    let hex_layout = HexLayout::default();
    let tile = LayerCakeHexTileBundle::new(Hex::new(0, 0), &hex_layout, BiomeType::Forest);
    
    assert_eq!(tile.biome.biome_type, BiomeType::Forest);
    assert!(tile.biome.can_be_adjacent_to(BiomeType::Plains));
    assert!(!tile.biome.can_be_adjacent_to(BiomeType::Lava));
}
```

### System Testing
```rust
#[test]
fn test_trauma_processing() {
    let mut app = App::new();
    app.add_systems(Update, trauma_processing_system);
    
    let companion = app.world.spawn(CompanionBundle::new_einar(&asset_server, Vec3::ZERO)).id();
    
    // Send trauma event
    app.world.send_event(TraumaEvent {
        severity: 1.0,
        // ... other fields
    });
    
    app.update();
    
    // Verify trauma was processed
    let psychology = app.world.get::<CompanionPsychology>(companion).unwrap();
    assert!(psychology.trauma_level > 0.0);
}
```

## Future Improvements

### 1. World Generation
- Implement biome generation algorithms
- Add procedural feature placement
- Create dragon proximity effects

### 2. AI Integration
- Connect to AI content generation pipelines
- Implement dynamic asset loading based on world state
- Add procedural dialogue generation

### 3. Performance
- Optimize chunk loading/unloading
- Add multithreaded world generation
- Implement asset streaming optimizations

---

## Migration Status: ✅ COMPLETE

The game-database to pure Bevy ECS migration is complete. All sophisticated systems have been preserved and enhanced:

- **2+ years of horror RPG logic** preserved and improved
- **Layer cake tile system** implemented with hexx integration  
- **Sophisticated companion psychology** with trauma processing
- **Master dread orchestrator** affecting all systems
- **AI asset integration** ready for content generation
- **Memory-optimized infinite world** ready for chunk loading

The codebase is now significantly simpler, more performant, and easier to extend. Components are the source of truth, eliminating database complexity while preserving all the sophisticated game mechanics.
