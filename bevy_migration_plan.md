# Dragon's Labyrinth - Bevy Migration Plan

## Architecture Transition

### From JavaScript/React/Three.js to Bevy (Rust)

**Current Issues:**
- Memory leaks from interpreted JavaScript 
- Garbage collection performance spikes
- No direct memory control for 3D models
- WebGL limitations for complex scenes

**Bevy Solutions:**
- Compiled Rust performance
- Direct memory management
- Entity Component System (ECS) architecture
- Native WebAssembly compilation
- Excellent 2.5D rendering capabilities

## Implementation Strategy

### Phase 1: Core ECS Architecture
```rust
// Core components matching design bible principles
#[derive(Component)]
struct HexTile {
    q: i32,
    r: i32,
    tile_type: TileType,
    dread_level: u8,  // 0-4 following design bible
}

#[derive(Component)]
struct Companion {
    name: String,
    trauma_level: f32,
    current_state: CompanionState,
}

#[derive(Component)]
struct DreadSystem {
    current_level: u8,  // Peace(0) → Horror(4)
    progression_triggers: Vec<DreadTrigger>,
}
```

### Phase 2: Asset Integration
- Generate models using AI → .glb format
- Freesound audio integration → .ogg format  
- Idempotent asset generation pipeline
- Component-addressable asset system

### Phase 3: Narrative Systems
```rust
// Narrative orchestration through ECS
fn update_dread_progression(
    mut dread_query: Query<&mut DreadSystem>,
    companion_query: Query<&Companion>,
    tile_query: Query<&HexTile>,
) {
    // Horror-first design: Everything responds to dread level
    // NPCs refuse to talk, world darkens, companions develop trauma
}
```

### Phase 4: WebAssembly Deployment
- Bevy native WASM compilation
- Mobile-optimized performance
- Browser deployment with full feature set

## Performance Targets (Design Bible Requirements)

- ✅ 60 FPS with 10,000 rendered hex tiles
- ✅ < 200MB memory for full game  
- ✅ < 2 second area load times
- ✅ 30 FPS on mid-range mobile devices

## Design Bible Alignment

### Zero External Dependencies
- ✅ AI-generated models and assets only
- ✅ Freesound CC0 audio integration
- ✅ No purchased asset libraries

### Idempotent Generation  
- ✅ Deterministic component IDs
- ✅ Stable API contracts
- ✅ Versioned output system

### Component-Based Architecture
- ✅ ECS naturally enforces component isolation
- ✅ <100 lines per system
- ✅ Clear input/output contracts

### Horror-First Design
- ✅ Dread level drives all systems
- ✅ Peace → Unease → Dread → Terror → Horror
- ✅ Companion trauma and world corruption

## Migration Steps

1. **Setup Bevy Project Structure**
   - Cargo.toml with WebAssembly target
   - Core ECS systems for hex world
   - Asset loading pipeline

2. **Port Hexagonal World System**
   - HexTile components with proper coordinates
   - Efficient rendering without memory leaks
   - Material system for tile variations

3. **Implement Narrative Systems**
   - DreadProgressionSystem
   - CompanionTraumaSystem  
   - WorldCorruptionSystem

4. **Asset Generation Pipeline**
   - AI model generation → Bevy asset loading
   - Freesound integration with proper licensing
   - Idempotent generation with version control

5. **WebAssembly Deployment**
   - WASM build configuration
   - Browser optimization
   - Mobile performance testing

## Benefits of Bevy Architecture

1. **Performance**: Compiled Rust vs interpreted JavaScript
2. **Memory Safety**: No garbage collection issues
3. **ECS Architecture**: Perfect for component-based design
4. **2.5D Excellence**: Bevy's strength for this style
5. **WebAssembly**: Native compilation to web
6. **Horror Integration**: Systems naturally respond to narrative state

This migration will solve the fundamental performance issues while perfectly aligning with the design bible's component-based, horror-first architecture.