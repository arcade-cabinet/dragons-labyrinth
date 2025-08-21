# Dragon's Labyrinth - Technical Architecture

## Overview
Dragon's Labyrinth uses Bevy with specialized crates to implement a fully AI-generated horror RPG. The technical architecture emphasizes board-based gameplay with beauty textures, hex grid pathfinding, and narrative-driven dialogue systems.

## Core Technical Components

### Hex Grid System (Hexx)
```rust
// Axial hex coordinates with pathfinding algorithms
use hexx::*;

let hex_layout = HexLayout::default();
let path = navigation_grid.find_path(start_hex, goal_hex);
let fov = navigation_grid.calculate_fov(player_hex, vision_radius);
```

**Features:**
- A* pathfinding with terrain costs
- Field of view calculations
- Ring/range operations for mount auras
- Distance calculations for proximity horror

### Board Rendering System
**Beauty Textures + Control Maps Architecture:**
- **Albedo Textures**: Seamlessly tiling materials (grass, dirt, sand, rock, water, lava)
- **Splatmaps**: RGBA channels encoding material weights per pixel
- **Overlay Masks**: Roads, bridges, hazards that override base walkability
- **ID Maps**: Reserved colors marking interactive objects with JSON metadata

```rust
// Fragment shader blends materials based on splatmap weights
let final_color = 
    grass_texture * splatmap.r +
    dirt_texture * splatmap.g + 
    rock_texture * splatmap.b +
    water_texture * splatmap.a;
```

### Navigation Grid Integration
Navigation costs derived from splatmap data with mount aura modifications:

```rust
// Base terrain costs from splatmap
let grass_cost = 1.0;
let water_cost = 10.0; // Requires mount to traverse

// Mount auras modify costs within radius
for hex in mount_aura.radius {
    if mount_type == Seastrider {
        water_cost = 1.0 + alignment_bonus;
    }
}
```

### Dialogue System (Yarn Spinner)
Narrative-heavy design using .yarn files with dread progression:

```rust
// Dialogue nodes change based on dread level
match dread_state.current_level {
    0 => "villager_peace_greeting",
    1 => "villager_nervous_greeting", 
    2 => "villager_fearful_greeting",
    3 => "villager_refuses_talk",
    4 => "villager_gone", // Fled or disappeared
}
```

### Level Editing (Yoleck)
Same binary switches to editor mode for AI-generated content:
- Load .yol files (JSON entity descriptions)
- Edit entities with egui widgets
- Test levels immediately in-engine
- Export modified .yol files for AI pipeline

### UI System (Cobweb UI)
Declarative UI with .cob scene format:
- AI generates complete UI layouts
- Built-in localization support
- Reactive primitives for game state
- Widget libraries for menus/inventory/dialogue

## Horror Progression Integration

### Dread Level Effects (0-4)
Every system responds to dread progression:

```rust
fn update_world_corruption(dread_level: u8) {
    match dread_level {
        0 => "peace: bright textures, full ambient audio",
        1 => "unease: shadows longer, bird sounds fade", 
        2 => "dread: splatmaps shift to corrupted variants",
        3 => "terror: reality distortion, false audio cues",
        4 => "horror: first-person transition, stalking audio",
    }
}
```

### Companion Trauma System
Dialogue trees branch based on trauma accumulation:
- **Einar**: Protective → Doubting → Panicking → Broken
- **Mira**: Optimistic → Forced cheer → Abandons party  
- **Sorin**: Curious → Obsessed → Ally OR Traitor
- **Tamara**: Innocent → Confused → Traumatized → Symbol

### Mount System with Alignment
Mounts modify terrain traversal based on player choices:
- **Good Alignment**: Mounts bond, larger auras, reduced costs
- **Evil Alignment**: Mounts enslaved, smaller auras, penalty costs
- **Mount Types**: Seastrider (water), RockCrusher (lava), VoidWalker (horror)

## Performance Architecture

### Mobile Optimization
- **Chunked Rendering**: Only visible hex regions loaded
- **Asynchronous Asset Loading**: Progressive texture streaming
- **ECS Efficiency**: Systems process only changed components
- **WASM Compilation**: Native performance in browser

### Memory Management
- **No Garbage Collection**: Rust ownership prevents leaks
- **Asset Pooling**: Reuse models across hex tiles
- **Texture Atlasing**: Combine materials into single textures
- **Component Sparse Sets**: Memory-efficient entity storage

## AI Generation Pipeline

### Asset Generation Workflow
1. **AI Prompt** → Generate .glb models, .png textures, .ogg audio
2. **Validation** → Check format compliance, optimization
3. **Integration** → Load into Bevy asset system with stable IDs
4. **Testing** → Automated performance and visual verification

### Idempotent Generation
```rust
// Deterministic asset IDs for repeatability
fn generate_asset_id(asset_type: &AssetType, parameters: &AssetParameters) -> String {
    let mut hasher = DefaultHasher::new();
    asset_type.hash(&mut hasher);
    parameters.hash(&mut hasher);
    format!("asset_{:x}", hasher.finish())
}
```

### Content-Addressable Storage
- Generated assets stored by content hash
- Prevents duplicate generation
- Enables incremental updates
- Supports version control integration

## Specialized Crate Integration

### Hexx (Hex Grid)
- **Coordinates**: Axial (q,r) system with conversion utilities
- **Algorithms**: A*, FOV, rings, ranges, line-of-sight
- **Mesh Generation**: Optional 3D hex column rendering

### Yarn Spinner (Dialogue)
- **Compilation**: .yarn → YarnProject resource
- **Runtime**: DialogueRunner component with event system
- **Integration**: Custom dialogue views with Cobweb UI

### Yoleck (Level Editor)
- **Format**: JSON .yol files for entity descriptions
- **Editing**: egui widgets for entity properties
- **Workflow**: Game/editor mode toggle in same binary

### Cobweb UI (Declarative UI)
- **Format**: .cob scene files with layout descriptions
- **Reactivity**: Resource mutation reactions, system events
- **Localization**: Built-in i18n support for multi-language

This technical architecture provides the foundation for a performant, fully AI-generated horror RPG that maintains narrative focus while leveraging modern Rust game development practices.