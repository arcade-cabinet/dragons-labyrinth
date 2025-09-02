# Rev2 Complete Integration Plan - Dragon's Labyrinth

## Overview
This document outlines the complete plan for integrating modern Bevy libraries into Dragon's Labyrinth, focusing on physics-based movement with Avian2d, professional tilemap rendering with bevy_ecs_tilemap, and procedural dungeon generation with mapgen.

## Current Situation Analysis

### What We Were Trying to Do (Wrong Approach)
- Fixing compilation errors without refactoring the architecture
- Keeping old sprite-based systems while adding new libraries
- Not leveraging the physics engine properly
- Trying to maintain backward compatibility with legacy code

### Why This Approach Failed
- Old systems (SpriteBundle, manual movement) conflict with new physics-based approach
- UI/Text systems need complete overhaul for Bevy 0.16
- Material2dPlugin conflicts with modern rendering approach
- No clear separation between physics entities and visual representation

## Correct Architecture Plan

### 1. Physics-First Design (Avian2d)

#### Core Concepts
```rust
// Every game entity has physics components
Entity {
    // Physics (Avian2d)
    RigidBody::Dynamic/Kinematic,
    Collider::circle(radius),
    LinearVelocity,
    AngularVelocity,
    
    // Game Logic
    Player/Npc/Creature,
    AxialPos { q, r },  // Logical hex position
    
    // Visual (bevy_ecs_tilemap or sprites)
    TilePos,  // For tilemap entities
    Sprite,   // For character sprites
}
```

#### Movement System Refactor
```rust
// OLD: Direct transform manipulation
fn movement_system(mut transform: Query<&mut Transform>) {
    transform.translation.x += dx;  // BAD
}

// NEW: Physics-based movement
fn movement_system(mut velocity: Query<&mut LinearVelocity, With<Player>>) {
    velocity.0 = Vec2::new(dx, dy);  // Let physics handle it
}
```

#### Collision System
```rust
// Use Avian2d's collision events
fn handle_collisions(
    mut collision_events: EventReader<CollisionStarted>,
    players: Query<Entity, With<Player>>,
    enemies: Query<Entity, With<Creature>>,
) {
    for CollisionStarted(e1, e2) in collision_events.read() {
        // Handle player-enemy collisions
        // Trigger combat, dialogue, etc.
    }
}
```

### 2. Tilemap Rendering (bevy_ecs_tilemap)

#### Architecture
```rust
// Tilemap is purely visual, not physics
TilemapEntity {
    TilemapId,
    TilemapSize { x: 100, y: 100 },
    TileStorage,  // Efficient tile storage
}

// Individual tiles
TileEntity {
    TilePos { x, y },  // Grid position
    TileTexture(index),  // Atlas index
    TileVisible,
    TileFlip,
}
```

#### Hex Coordinate Mapping
```rust
// Convert between axial hex and tilemap grid
fn axial_to_tile_pos(q: i32, r: i32) -> TilePos {
    // Use offset coordinates for rectangular tilemap
    let col = q;
    let row = r + (q - (q & 1)) / 2;
    TilePos { x: col as u32, y: row as u32 }
}
```

#### Layer System
```rust
enum MapLayer {
    Terrain = 0,   // Base biome tiles
    Features = 1,  // POIs, buildings
    Fog = 2,       // Fog of war
    UI = 3,        // Selection, highlights
}
```

### 3. Procedural Dungeons (mapgen)

#### Integration Pattern
```rust
// Generate dungeon when entering POI
fn enter_dungeon(
    player_pos: Query<&AxialPos, With<Player>>,
    pois: Query<(&AxialPos, &POIType)>,
) {
    if player_at_dungeon_poi() {
        // Generate dungeon
        let dungeon = procgen::generate_hex_dungeon(30, 30);
        
        // Convert to tilemap
        spawn_dungeon_tilemap(dungeon);
        
        // Add physics colliders for walls
        spawn_dungeon_colliders(dungeon);
    }
}
```

### 4. System Architecture

#### Plugin Structure
```rust
// Main game plugin
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // Physics
            .add_plugins(Avian2dPlugin::default())
            .add_plugins(PhysicsDebugPlugin::default())
            
            // Rendering
            .add_plugins(TilemapPlugin)
            
            // Game Systems
            .add_plugins(WorldPlugin)
            .add_plugins(CombatPlugin)
            .add_plugins(CompanionPlugin)
            .add_plugins(ForgePlugin);
    }
}
```

#### Component Organization
```rust
// crates/world/src/components.rs
// Physics components (from Avian2d)
pub use avian2d::prelude::*;

// Game components
#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct AxialPos { pub q: i32, pub r: i32 }

#[derive(Component)]
pub struct Health { pub current: i32, pub max: i32 }

// Tilemap components (from bevy_ecs_tilemap)
pub use bevy_ecs_tilemap::prelude::*;
```

## Refactoring Steps

### Phase 1: Clean Slate
1. **Remove all old rendering code**
   - Delete Material2dPlugin usage
   - Remove SpriteBundle references
   - Clean up manual transform manipulation

2. **Set up physics foundation**
   ```rust
   // Every entity gets physics components
   commands.spawn((
       RigidBody::Dynamic,
       Collider::circle(16.0),
       Player,
       AxialPos { q: 0, r: 0 },
   ));
   ```

### Phase 2: Tilemap Integration
1. **Create tilemap from worldbook**
   ```rust
   fn spawn_world_tilemap(
       mut commands: Commands,
       worldbook: Res<WorldBook>,
       asset_server: Res<AssetServer>,
   ) {
       let map_size = TilemapSize { x: 100, y: 100 };
       let mut tile_storage = TileStorage::empty(map_size);
       let tilemap_entity = commands.spawn_empty().id();
       
       // Spawn tiles from worldbook regions
       for region in &worldbook.regions {
           for hex in &region.hex_points {
               let tile_pos = axial_to_tile_pos(hex.q, hex.r);
               let tile_entity = spawn_tile(&mut commands, tile_pos, hex.biome);
               tile_storage.set(&tile_pos, tile_entity);
           }
       }
   }
   ```

### Phase 3: Physics-Based Movement
1. **Hex movement with physics**
   ```rust
   fn hex_movement_system(
       input: Res<ButtonInput<KeyCode>>,
       mut player: Query<(&mut LinearVelocity, &AxialPos), With<Player>>,
   ) {
       let mut direction = Vec2::ZERO;
       
       if input.pressed(KeyCode::KeyQ) { // Northwest
           direction = hex_direction_to_world(HexDirection::Northwest);
       }
       // ... other directions
       
       velocity.0 = direction * MOVE_SPEED;
   }
   ```

### Phase 4: Collision-Based Interactions
1. **Replace proximity checks with collisions**
   ```rust
   fn setup_interaction_zones(
       mut commands: Commands,
       npcs: Query<(Entity, &Transform), With<Npc>>,
   ) {
       for (entity, transform) in &npcs {
           commands.entity(entity).insert((
               Sensor,  // Avian2d sensor collider
               Collider::circle(32.0),  // Interaction radius
               CollisionLayers::new(Layer::Interaction, Layer::Player),
           ));
       }
   }
   ```

## Benefits of This Approach

### 1. Physics Engine Benefits
- **Automatic collision detection** - No manual distance checks
- **Smooth movement** - Physics interpolation
- **Pushback/knockback** - Built-in physics responses
- **Performance** - Spatial partitioning for free

### 2. Tilemap Benefits  
- **Efficient rendering** - Batched tile rendering
- **Chunk loading** - Built-in chunking system
- **Mouse picking** - Tile selection out of the box
- **Layers** - Proper z-ordering

### 3. Architecture Benefits
- **Clear separation** - Physics, logic, and rendering separated
- **Testability** - Each system can be tested independently
- **Extensibility** - Easy to add new features
- **Performance** - Leverages Bevy's parallelism

## Migration Priority

### Immediate (Do First)
1. Create new physics-based player entity
2. Implement hex movement with LinearVelocity
3. Set up basic tilemap from worldbook
4. Add collision detection for walls

### Next Phase
1. Convert NPCs to physics entities
2. Add interaction sensors
3. Implement fog of war layer
4. Add dungeon generation at POIs

### Later
1. Combat system with physics
2. Companion following with steering behaviors
3. Particle effects for abilities
4. Advanced tilemap features (animated tiles, etc.)

## Code Organization

### New File Structure
```
crates/world/src/
├── physics/
│   ├── mod.rs           # Physics setup
│   ├── movement.rs      # Movement systems
│   ├── collisions.rs    # Collision handlers
│   └── layers.rs        # Collision layers
├── tilemap/
│   ├── mod.rs           # Tilemap setup
│   ├── spawn.rs         # Tile spawning
│   ├── layers.rs        # Layer management
│   └── hex_utils.rs     # Hex/tile conversion
├── gameplay/
│   ├── combat.rs        # Combat systems
│   ├── dialogue.rs      # Dialogue system
│   ├── companions.rs    # Companion AI
│   └── forge.rs         # Forge mechanics
└── procgen/
    ├── dungeon.rs       # Dungeon generation
    └── world.rs         # World generation
```

## Testing Strategy

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_hex_to_world_position() {
        // Test coordinate conversions
    }
    
    #[test]
    fn test_collision_layers() {
        // Test layer interactions
    }
}
```

### Integration Tests
1. Spawn player, move with physics
2. Load worldbook, create tilemap
3. Enter dungeon POI, generate dungeon
4. Trigger combat via collision

## Common Pitfalls to Avoid

### Don't
- Mix physics and non-physics movement
- Manually check distances when collisions would work
- Use transforms directly when you have physics
- Create massive tilemaps without chunking

### Do
- Use physics for all moving entities
- Leverage collision events
- Let physics engine handle interpolation
- Use tilemap layers effectively

## Success Metrics

### Technical
- 60 FPS with full physics simulation
- <100ms dungeon generation
- Smooth hex movement
- Accurate collision detection

### Gameplay
- Intuitive movement
- Clear visual feedback
- Responsive interactions
- No physics glitches

## Next Steps

1. **Stop trying to fix the current broken build**
2. **Create a minimal physics prototype**
   - Just player + movement + tilemap
3. **Gradually migrate systems**
   - One at a time, testing each
4. **Document patterns as we go**
   - Update this plan with learnings

This approach will be much more productive than trying to patch the existing code. We're essentially rebuilding the game systems on a modern foundation rather than retrofitting old code.
