# Rev2 Integration Attempt - ARCHIVED

## Date: August 31, 2025

## What We Tried
Attempted to integrate bevy_ecs_tilemap, Avian2d physics, and mapgen into the existing codebase by:
1. Adding dependencies to Cargo.toml
2. Creating procgen crate with mapgen
3. Moving tilemap_bridge utilities
4. Trying to fix 66+ compilation errors

## Why It Failed
- Tried to patch old code instead of refactoring
- Mixed physics and non-physics movement systems
- SpriteBundle and Material2dPlugin incompatible with Bevy 0.16
- UI/Text types not properly imported
- No clear separation between physics, rendering, and game logic

## What We Learned
**Need physics-first architecture!** Can't retrofit physics onto transform-based movement.

## Compilation Errors Encountered
- `SpriteBundle` not found (needs proper imports or is deprecated)
- `MaterialMesh2dBundle` and `Mesh2dHandle` missing
- `NodeBundle`, `Style`, `TextStyle` import issues
- `Color::rgba` should be `Color::srgba`
- WorldBook missing Resource derive
- Stats missing Serialize/Deserialize

## Files Modified (Now Broken)
- crates/world/src/systems/spawn.rs
- crates/world/src/systems/movement.rs
- crates/world/src/systems/encounters.rs
- crates/world/src/systems/ui.rs
- crates/world/src/systems/shops.rs
- crates/world/src/systems/dialogue.rs
- crates/world/src/systems/quests.rs
- crates/world/src/resources.rs
- crates/world/src/abilities.rs
- crates/world/src/alignment.rs

## Correct Approach (See rev2-complete-integration-plan.md)
1. Don't patch - refactor with physics-first design
2. Create minimal prototype first
3. Migrate systems gradually
4. Keep physics, rendering, and logic separated

## Status: ABANDONED
This approach was abandoned in favor of a complete physics-first refactor.
