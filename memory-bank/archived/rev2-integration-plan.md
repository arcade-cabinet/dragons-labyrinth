# Rev2 Integration Plan - Three Crate Architecture

## Overview
The `rev2` directory presents a clean architecture integrating three powerful crates:
1. **bevy_ecs_tilemap** - Hex tilemap rendering (partially integrated)
2. **mapgen 0.6.0** - Procedural dungeon generation algorithms
3. **bevy-agent 0.1.0** - AI-powered Rust code generation (optional)

## Current State vs Rev2 Vision

### Current Architecture
```
Python AI Pipeline → JSON Data → Rust/Bevy Game
- ai/ generates worldbuilding from markdown
- Manual Rust code for game mechanics
- Custom hex implementation
- bevy_ecs_tilemap partially integrated
```

### Rev2 Architecture
```
Python AI (worldbuilding) + bevy-agent (Rust code) → Game
- Python handles content/worldbuilding
- bevy-agent generates Rust code dynamically
- mapgen provides dungeon algorithms
- bevy_ecs_tilemap handles all hex rendering
```

## Key Components from Rev2

### 1. Procgen Crate (`crates/procgen`)
```rust
// Uses mapgen for dungeon generation
pub fn generate_hex_dungeon(w: usize, h: usize) -> HexDungeon {
    MapBuilder::new(w, h)
        .with(NoiseGenerator::uniform())
        .with(CellularAutomata::new())
        .with(AreaStartingPosition::new(XStart::CENTER, YStart::CENTER))
        .build()
}
```
**Benefits:**
- Professional dungeon generation algorithms
- Cellular automata for organic caves
- Noise generation for terrain variety
- Starting position control

### 2. Tilemap Bridge (`crates/world/src/tilemap_bridge.rs`)
```rust
// Clean utilities for hex coordinate conversion
pub fn axial_to_world(q: i32, r: i32, grid_size: &TilemapGridSize, coord: HexCoordSystem) -> Vec2
pub fn parse_axial(s: &str) -> (i32, i32)
```
**Benefits:**
- Centralized hex math utilities
- Clean separation from game logic
- Works with bevy_ecs_tilemap natively

### 3. Agent Bridge (`tools/agent_bridge`)
```rust
// Optional AI code generation
let agent = BevyAIAgent::new(cfg).await?;
let resp = agent
    .request("Generate hex biome tiles and quests for a Bevy hex RPG.")
    .with_model(ModelType::GPT4)
    .execute()
    .await?;
```
**Benefits:**
- Generate Rust code on demand
- Reduce manual coding for repetitive tasks
- AI understands Bevy patterns

## Integration Strategy

### Phase 1: Add mapgen for Dungeons
```toml
# Add to Cargo.toml
[workspace.dependencies]
mapgen = "0.6.0"
```

Create `crates/procgen`:
- Move dungeon generation logic here
- Use mapgen algorithms for:
  - Dungeon layouts (cellular automata)
  - Cave systems (noise generation)
  - Building interiors (room placement)

### Phase 2: Consolidate Tilemap Utilities
Move all hex utilities to `tilemap_bridge.rs`:
- Axial to world conversion
- Hex distance calculations
- Neighbor finding
- Mouse picking for hex tiles

### Phase 3: Optional bevy-agent Integration
For complex features, use bevy-agent to generate code:
```bash
# Generate combat system
bevy-agent add "inverted HP combat system where attacks cost health"

# Generate companion system
bevy-agent add "companion trauma tracking with breaking points"

# Generate forge redemption
bevy-agent add "forge system where players trade HP for second chances"
```

## Recommended Workflow

### 1. Content Generation (Python AI)
```bash
python -m ai expand  # Generate world data
python -m ai images  # Generate tilesets
```

### 2. Procedural Generation (Rust/mapgen)
```rust
// Use mapgen for runtime dungeon generation
let dungeon = procgen::generate_hex_dungeon(50, 50);
```

### 3. Code Generation (bevy-agent, optional)
```bash
# When adding new features
BEVY_AGENT_AUTOGEN=1 cargo run -p game
```

## Implementation Priority

### Immediate (This Session)
1. ✅ Add mapgen dependency
2. ✅ Create procgen crate with dungeon generation
3. ✅ Move hex utilities to tilemap_bridge
4. ✅ Test dungeon generation with worldbook POIs

### Next Session
1. ⏳ Integrate procgen dungeons with combat encounters
2. ⏳ Use mapgen for shrine/forge interiors
3. ⏳ Connect dungeon generation to worldbook locations

### Future (Optional)
1. ❌ Set up bevy-agent for code generation
2. ❌ Create templates for common patterns
3. ❌ Automate repetitive coding tasks

## Benefits of This Architecture

### Separation of Concerns
- **Python AI**: Worldbuilding, narrative, asset generation
- **mapgen**: Procedural dungeon algorithms
- **bevy-agent**: Rust code generation (optional)
- **Game code**: Core mechanics and integration

### Flexibility
- Can use AI for content OR code generation
- Dungeons generated at runtime or build time
- Mix handcrafted and procedural content

### Maintainability
- Clear module boundaries
- Each crate has single responsibility
- Easy to upgrade individual components

## Migration Path from Current Code

### Current Files to Refactor
```
crates/world/src/
├── hex.rs → tilemap_bridge.rs (merge with rev2 version)
├── systems/
│   ├── tilemap_spawn.rs → Use procgen for dungeons
│   ├── tilemap_movement.rs → Use tilemap_bridge utilities
│   └── combat.rs → Keep, but consider bevy-agent for expansion
```

### New Structure
```
crates/
├── procgen/           # NEW: Dungeon generation
│   └── src/
│       └── lib.rs     # mapgen integration
├── world/
│   └── src/
│       ├── tilemap_bridge.rs  # Hex utilities
│       └── systems/            # Game systems
└── tools/
    └── agent_bridge/  # Optional AI code gen
```

## Example Integration

### Using All Three Together
```rust
// 1. Load worldbook data (Python-generated)
let worldbook = load_worldbook();

// 2. Generate dungeon for POI (mapgen)
for poi in worldbook.regions[0].hex_points {
    if poi.kind == "dungeon" {
        let dungeon = procgen::generate_hex_dungeon(30, 30);
        spawn_dungeon_at(poi.axial, dungeon);
    }
}

// 3. Optional: Use bevy-agent for complex features
// Run: bevy-agent add "boss fight with multiple phases"
```

## Conclusion

The rev2 architecture provides a clean separation between:
- **Content generation** (Python AI)
- **Procedural generation** (mapgen)  
- **Code generation** (bevy-agent)

This allows us to leverage the best tool for each job while maintaining a clean, modular architecture. The immediate priority should be integrating mapgen for dungeon generation, as this provides the most immediate value without requiring significant refactoring.
