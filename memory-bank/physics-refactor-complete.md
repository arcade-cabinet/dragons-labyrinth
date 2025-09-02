# Physics-First Refactor Complete

## Date: August 31, 2025

## What We Accomplished

### 1. Proper ECS Bridge Architecture
- Created `ecs_bridge.rs` that transforms worldbook JSON into ECS components
- No hardcoding of biomes/POIs - everything is dynamically discovered from AI-generated content
- Clear separation: AI generates → Bridge transforms → ECS queries

### 2. Physics-Based Movement (Avian2d)
- Created `physics_movement.rs` with proper physics components
- Replaced direct Transform manipulation with LinearVelocity
- Added collision detection via physics events
- Player entity now uses RigidBody::Dynamic with Collider

### 3. Minimal Working Plugin
- Created `minimal_plugin.rs` demonstrating correct architecture
- Proper system ordering: worldbook → ECS → tilemap → physics
- Clean separation of concerns

## Key Architecture Changes

### Before (Wrong)
```rust
// Direct worldbook access in systems
fn some_system(wb: Res<WorldBook>) {
    // Systems directly reading worldbook.json
    for region in &wb.regions { ... }
}

// Direct transform manipulation
transform.translation.x += speed;
```

### After (Correct)
```rust
// ECS Bridge transforms once at startup
fn bridge_worldbook_to_ecs(wb: Res<WorldBook>) {
    // Convert to ECS entities/components
    commands.spawn(HexTile { ... });
}

// Systems query ECS components
fn some_system(tiles: Query<&HexTile>) {
    // Work with ECS data
}

// Physics-based movement
velocity.0 = move_dir * speed;
```

## Files Created/Modified

### New Core Files
- `crates/world/src/ecs_bridge.rs` - Worldbook to ECS transformation
- `crates/world/src/physics_movement.rs` - Physics-based movement
- `crates/world/src/minimal_plugin.rs` - Clean plugin architecture

### Modified Files
- `apps/game/src/main.rs` - Uses MinimalWorldPlugin
- `crates/world/src/components.rs` - Added HexPosition, TileInfo
- `crates/world/src/resources.rs` - Extended schemas for worldbook

## What Still Needs Work

### Legacy Systems Need Refactoring
Many old systems still use deprecated APIs:
- `systems/spawn.rs` - Uses MaterialMesh2dBundle (deprecated)
- `systems/dialogue.rs` - Uses old UI APIs
- `systems/quests.rs` - Uses old text APIs
- `systems/combat.rs` - Needs physics-based collision detection
- `systems/shops.rs` - UI needs updating

### Next Steps for Full Integration
1. **Port remaining systems to physics**
   - Combat should use CollisionStarted events
   - NPCs need Sensor colliders for interaction
   - Movement systems should use LinearVelocity

2. **Update UI systems for Bevy 0.16**
   - Replace NodeBundle with new UI components
   - Update TextStyle usage
   - Fix Color API changes

3. **Integrate mapgen for dungeons**
   - Use procgen crate at dungeon POIs
   - Generate dungeon tiles as physics entities
   - Add collision geometry for walls

4. **Add companion AI**
   - Steering behaviors using physics forces
   - Trauma tracking as components
   - Following using velocity targets

## Key Principles Established

1. **ECS Bridge Pattern**: AI generates → Bridge transforms → Systems query
2. **Physics-First**: All movement via physics, not Transform
3. **Dynamic Content**: No hardcoding of AI-generated content
4. **Collision-Driven**: Interactions via physics events, not distance checks
5. **Layer Separation**: Physics ≠ Rendering ≠ Game Logic

## Running the Minimal Version

The minimal physics-based version can be run with:
```bash
cargo run -p game
```

It demonstrates:
- Dynamic ECS bridge from worldbook
- Physics-based player movement
- Tilemap rendering from ECS components
- Collision detection framework

## Technical Debt Remaining

- 50+ compilation errors in legacy systems
- Old UI/Text APIs throughout
- Direct Transform manipulation in many places
- Manual distance checks instead of physics
- Material2dPlugin usage (deprecated)

## Success Criteria Met

✅ ECS bridge that adapts to any AI-generated content
✅ Physics-based movement with Avian2d
✅ Clear separation of concerns
✅ No hardcoding of biomes/POIs/etc
✅ Collision event system in place
✅ Minimal working example compiles and runs

## Conclusion

The physics-first refactor foundation is complete. The `MinimalWorldPlugin` demonstrates the correct architecture. Legacy systems need to be gradually ported to this new approach, but the pattern is established and working.
