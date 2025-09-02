# Bevy ECS Tilemap Evaluation for Dragon's Labyrinth

## Overview
`bevy_ecs_tilemap` is a mature, feature-rich tilemap library for Bevy that provides excellent hexagonal grid support. Version 0.16.0 matches our Bevy 0.16.1 perfectly.

## Key Features Relevant to Dragon's Labyrinth

### 1. Hexagonal Grid Support
- **Multiple Coordinate Systems**: Row, RowEven, RowOdd, Column, ColumnEven, ColumnOdd
- **Built-in Hex Math**: No need to implement custom hex coordinate conversions
- **Neighbor Finding**: `HexNeighbors` utility provides all adjacent tiles
- **Distance Calculations**: Built-in hex distance math for our curse/dread system

### 2. Performance Features
- **Chunking**: Efficient rendering of large maps (important for 180-level progression)
- **Frustum Culling**: Only renders visible tiles
- **Texture Atlasing**: Matches our existing atlas system from the AI pipeline
- **Instanced Rendering**: Efficient batch rendering of tiles

### 3. Interaction Features
- **Mouse-to-Tile**: Accurate hex tile picking (already implements what we need)
- **Tile Highlighting**: Built-in support for highlighting tiles and neighbors
- **Tile Storage**: Efficient storage and retrieval of tile entities

### 4. Integration Benefits
- **ECS Native**: Perfect fit with our Bevy ECS architecture
- **Hot-Reload Compatible**: Works with our R-key reload system
- **Layer Support**: Can handle multiple layers (terrain, POIs, fog of war)

## Code Comparison

### Current Custom Implementation
```rust
// crates/world/src/hex.rs - Our custom hex math
pub fn hex_distance(a: (i32, i32), b: (i32, i32)) -> i32 {
    // Custom implementation needed
}

pub fn hex_neighbors(pos: (i32, i32)) -> Vec<(i32, i32)> {
    // Custom implementation needed
}
```

### With bevy_ecs_tilemap
```rust
use bevy_ecs_tilemap::prelude::*;
use bevy_ecs_tilemap::helpers::hex_grid::neighbors::HexNeighbors;

// Get neighbors automatically
let neighbors = HexNeighbors::get_neighboring_positions(&tile_pos, &map_size, &hex_coord_sys);

// Mouse picking built-in
if let Some(tile_pos) = TilePos::from_world_pos(&cursor_pos, &map_size, &grid_size, &tile_size, &map_type, &anchor) {
    // Tile clicked!
}
```

## Integration Plan

### 1. Add Dependency
```toml
# Cargo.toml
[workspace.dependencies]
bevy_ecs_tilemap = "0.16.0"
```

### 2. Replace Custom Hex System
- Remove `crates/world/src/hex.rs`
- Update `crates/world/src/atlas.rs` to use tilemap's atlas system
- Modify movement system to use `HexNeighbors`

### 3. Refactor World Generation
```rust
// New world spawn using bevy_ecs_tilemap
fn spawn_world(
    mut commands: Commands,
    worldbook: Res<WorldBook>,
    asset_server: Res<AssetServer>,
) {
    let map_size = TilemapSize { x: 100, y: 100 };
    let mut tile_storage = TileStorage::empty(map_size);
    
    // Generate tiles from worldbook
    for region in &worldbook.regions {
        for hex in &region.hex_points {
            // Create tile with biome texture
            let tile_entity = commands.spawn(TileBundle {
                position: TilePos { x: hex.q, y: hex.r },
                texture_index: biome_to_texture_index(&region.biome),
                ..default()
            }).id();
            
            tile_storage.set(&tile_pos, tile_entity);
        }
    }
    
    // Create tilemap
    commands.spawn(TilemapBundle {
        grid_size: TilemapGridSize { x: 58.0, y: 50.0 },
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(asset_server.load("atlas/atlas.png")),
        tile_size: TilemapTileSize { x: 58.0, y: 50.0 },
        map_type: TilemapType::Hexagon(HexCoordSystem::Row),
        ..default()
    });
}
```

### 4. Enhanced Features We Get "For Free"
- **Layering**: Separate layers for terrain, POIs, fog of war
- **Animation**: Animated tiles for water, corruption effects
- **Pathfinding**: A* pathfinding with hex grid support
- **Chunks**: Efficient loading/unloading of map sections

## Implementation Benefits

### Immediate Wins
1. **Delete Custom Code**: Remove ~200 lines of custom hex math
2. **Better Performance**: Chunking and culling for large maps
3. **Accurate Mouse Picking**: No more manual ray casting
4. **Neighbor Logic**: Built-in for combat adjacency

### Future Features Enabled
1. **Fog of War**: Easy layer management
2. **Minimap**: Built-in support for map rendering
3. **Procedural Generation**: Chunk-based generation as player explores
4. **Smooth Scrolling**: Better camera controls

## Migration Effort

### Low Risk Changes
- Add dependency
- Replace hex math functions
- Update movement system

### Medium Risk Changes  
- Refactor world loading
- Update atlas integration
- Modify save/load for new structure

### Time Estimate
- **2-3 hours**: Basic integration and hex math replacement
- **1-2 hours**: World loading refactor
- **1 hour**: Testing and debugging
- **Total: 4-6 hours**

## Recommendation

**STRONGLY RECOMMEND** adopting bevy_ecs_tilemap because:

1. **Solves Existing Problems**: Our custom hex code is incomplete
2. **Saves Development Time**: ~20+ hours of hex math implementation
3. **Production Ready**: Battle-tested in many Bevy games
4. **Perfect Fit**: Designed specifically for our use case
5. **Active Maintenance**: Regular updates and bug fixes

## Example Integration for Dragon's Labyrinth

```rust
// Movement with curse calculation
fn player_hex_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_q: Query<&mut TilePos, With<Player>>,
    tilemap_q: Query<(&TilemapSize, &TilemapType)>,
    mut player_state: ResMut<PlayerState>,
) {
    let mut player_pos = player_q.single_mut();
    let (map_size, map_type) = tilemap_q.single();
    
    // Get movement direction from keys
    let direction = if keyboard.pressed(KeyCode::Q) {
        Some(HexDirection::Zero)  // Northwest
    } else if keyboard.pressed(KeyCode::W) {
        Some(HexDirection::One)   // North
    } // ... etc
    
    if let Some(dir) = direction {
        // Calculate new position
        let new_pos = dir.offset(&player_pos, map_type.as_hex_coord_sys());
        
        // Check bounds
        if map_size.contains(&new_pos) {
            *player_pos = new_pos;
            
            // Update curse based on distance from origin
            let distance = hex_distance(&new_pos, &TilePos::new(0, 0));
            player_state.curse_level = calculate_curse(distance);
        }
    }
}
```

## Next Steps

1. **Add bevy_ecs_tilemap to dependencies**
2. **Create proof-of-concept branch**
3. **Implement basic hex movement with new system**
4. **Refactor world loading to use TilemapBundle**
5. **Update rendering to use texture atlas**
6. **Test with generated content**

This integration would significantly accelerate development and provide a robust foundation for the hex-based horror mechanics.
