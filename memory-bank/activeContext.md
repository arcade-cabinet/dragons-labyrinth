# Dragon's Labyrinth - Active Context

## Current Status
**Date**: December 2024
**Phase**: Core Implementation (Post-Migration)
**Bevy Version**: 0.16.1
**Rust Edition**: 2021

## Recent Changes

### Migration from JavaScript to Rust/Bevy
- Successfully migrated from React/Three.js to Bevy 0.16.1
- Resolved memory leak issues from JavaScript implementation
- Achieved 60+ FPS with 10,000 hex tiles (was crashing at 1,000 in JS)
- Implemented proper ECS architecture replacing object-oriented approach
- WebAssembly build working for web deployment

### Core Systems Implemented
- ✅ Hex grid system using Hexx crate
- ✅ Basic ECS components (Player, Companion, HexTile, etc.)
- ✅ Dread state management (0-4 progression)
- ✅ Camera system (isometric view)
- ✅ Basic lighting that responds to dread
- ✅ Initial companion spawning
- ✅ Database integration for game state

### Documentation Structure
- Migrated from `docs/` to `memory-bank/` structure
- Created `.clinerules` for AI assistant memory management
- Added `.cursor/rules/00-loader.mdc` for development patterns
- Preserved design bible and reference documentation

## Current Work Focus

### Immediate Priority: Asset Generation Pipeline
Need to implement the AI asset generation system for:
1. Hex tile models (.glb format)
2. Character models for companions
3. Environment props and decorations
4. UI elements (SVG-based)

### Board Rendering System
Implement beauty texture + splatmap architecture:
- Base terrain textures (grass, dirt, rock, water)
- Splatmap blending in shaders
- Overlay masks for roads/bridges
- ID maps for interactive objects

### Narrative Integration
Wire up the dialogue system:
- Integrate Yarn Spinner for dialogue trees
- Connect dialogue to dread progression
- Implement companion conversation system
- Create NPC interaction framework

## Next Steps

### Short Term (This Week)
1. [ ] Implement basic hex tile rendering with textures
2. [ ] Create splatmap shader for terrain blending  
3. [ ] Add hex interaction system (click/tap to move)
4. [ ] Implement basic pathfinding with Hexx
5. [ ] Create first biome (Peace stage meadow)

### Medium Term (Next 2 Weeks)
1. [ ] Integrate Yarn Spinner for dialogue
2. [ ] Implement companion AI behaviors
3. [ ] Create world corruption system
4. [ ] Add proximity audio system foundation
5. [ ] Implement first boss encounter (Hollow Caretaker)

### Long Term (Next Month)
1. [ ] Complete all 5 narrative stages
2. [ ] Implement all companion arcs
3. [ ] Create dragon stalking mechanics
4. [ ] Add first-person labyrinth mode
5. [ ] Polish for web deployment

## Active Decisions

### Architecture Choices
- **Hexx for hex grid**: Proven, efficient, good algorithms
- **Bevy 0.16.1**: Latest stable with needed features
- **ECS over OOP**: Better performance, cleaner architecture
- **WebAssembly primary**: Instant play, no download required

### Asset Strategy
- **Runtime generation**: Generate models/textures at runtime where possible
- **Caching layer**: Store generated assets in browser storage
- **Deterministic seeds**: Ensure reproducible generation
- **Progressive loading**: Load assets as needed, not upfront

### Performance Optimizations
- **Frustum culling**: Only render visible hex tiles
- **LOD system**: Reduce detail for distant objects
- **Texture atlasing**: Combine textures to reduce draw calls
- **Entity pooling**: Reuse entities instead of spawn/despawn

## Important Patterns

### Dread-Driven Systems
Every system checks `DreadState` resource:
```rust
fn system(dread: Res<DreadState>) {
    match dread.level {
        0 => // Peace behavior
        1 => // Unease behavior
        2 => // Dread behavior
        3 => // Terror behavior
        4 => // Horror behavior
    }
}
```

### Component Composition
Small, focused components that combine:
```rust
// Instead of MonolithicNPC component:
entity.spawn((
    NPC,
    DialogueCapable,
    DreadReactive,
    Mortal,
    FleeCapable,
))
```

### Event-Driven Narrative
Use Bevy events for narrative beats:
```rust
EventWriter<NarrativeEvent>
EventReader<CompanionEvent>
EventWriter<DreadProgressionEvent>
```

## Learnings

### From JavaScript Implementation
- **Memory management critical**: JS garbage collection killed performance
- **Component size matters**: Large components cause cache misses
- **Asset loading strategy**: Preloading everything doesn't work
- **Mobile first**: Desktop performance doesn't guarantee mobile

### From Bevy Migration
- **ECS is natural fit**: Horror progression maps perfectly to systems
- **Rust ownership helps**: Prevents accidental state corruption
- **WASM works well**: Near-native performance in browser
- **Bevy ecosystem rich**: Many crates solve common problems

## Current Blockers

### Technical
- Need shader expertise for splatmap blending
- Yarn Spinner integration documentation sparse
- WebAssembly audio can be tricky

### Content
- No asset generation pipeline yet
- Need Freesound integration approach
- Require biome design specifications

### Design
- Exact boss encounter mechanics undefined
- Companion dialogue trees need writing
- Labyrinth layout generation approach needed

## Resource Links

### Documentation
- [Bevy 0.16.1 Book](https://bevyengine.org/learn/book/)
- [Hexx Documentation](https://docs.rs/hexx/)
- [Yarn Spinner Rust](https://github.com/YarnSpinnerTool/YarnSpinner-Rust)

### Project Resources
- GitHub: https://github.com/jbcom/dragons-labyrinth
- Design Bible: `memory-bank/design_bible.md`
- Technical Architecture: `memory-bank/technical_architecture.md`

## Configuration Notes

### Build Commands
- Native: `cargo run`
- WASM: `./build_wasm.sh`
- Release: `cargo build --release`

### Development Setup
- Rust 1.88+ required
- wasm-bindgen-cli for web builds
- Python 3 for local web server

### Performance Targets
- Desktop: 60 FPS minimum
- Mobile: 30 FPS minimum  
- Memory: < 200MB usage
- Load time: < 2 seconds