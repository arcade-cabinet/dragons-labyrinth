# Dragon's Labyrinth - Agent Handoff Document

## 🚀 YOLO Mode Instructions for Next Agent

### Current State (2025-01-21)
The project has been fully refactored into a modular Bevy 0.16.1 workspace with 12 specialized crates. All dependencies are latest versions, Rust 1.88 with edition 2024. **The project builds with zero errors.**

### 🎯 Your Mission: Build the Playable Prototype

Work independently to create a playable vertical slice. Don't ask for permission - just build. The architecture is ready, now make it fun.

## 📋 Priority Task Queue (Do These In Order)

### Phase 1: Core Game Loop (2-3 hours)
```bash
# You're implementing the basic game state machine and hex movement
```

1. **Game States** (`/workspace/crates/game/src/states.rs`)
   - Create state machine: MainMenu → Loading → InGame → Paused
   - Add state transitions with fade effects
   - Implement pause menu (ESC key)
   - Add quick save/load hotkeys (F5/F9)

2. **Hex Movement System** (`/workspace/crates/game/src/movement.rs`)
   ```rust
   // Player moves on hex grid with these controls:
   // Q,W,E,A,S,D for 6 hex directions
   // Mouse click for pathfinding movement
   // Shift+Click for running
   ```
   - Implement hex coordinate system
   - Add smooth movement interpolation
   - Create movement cost system (terrain types)
   - Add turn-based movement points

3. **Camera Controller** (`/workspace/crates/game/src/camera.rs`)
   - Follow player with smooth damping
   - Mouse wheel zoom (2D/3D modes)
   - Tab key to toggle 2D/3D hex view
   - Edge scrolling for free camera mode

### Phase 2: Combat System (2-3 hours)

4. **Turn-Based Combat** (`/workspace/crates/game/src/combat.rs`)
   ```rust
   // Combat flow:
   // 1. Initiative roll (speed-based)
   // 2. Player turn: Move → Action → End
   // 3. Enemy turns (AI controlled)
   // 4. Environmental effects
   ```
   - Create combat state machine
   - Implement attack range checking (melee/ranged)
   - Add damage calculation with dice rolls
   - Create combat log UI element

5. **Basic Enemy AI** (`/workspace/crates/ai/src/behaviors.rs`)
   - Patrol behavior (follow waypoints)
   - Chase behavior (A* pathfinding to player)
   - Attack behavior (when in range)
   - Flee behavior (when health < 25%)

### Phase 3: Content & Polish (3-4 hours)

6. **First Playable Map** (`/workspace/crates/maps/src/presets.rs`)
   ```rust
   // Create a 20x20 hex map with:
   // - Starting village (safe zone)
   // - Forest path (tutorial combat)
   // - Corrupted grove (first boss)
   // - Hidden treasure caches
   ```

7. **UI Implementation** (`/workspace/crates/ui/src/game_ui.rs`)
   - Health/Sanity bars
   - Inventory grid (I key)
   - Quest tracker
   - Minimap with fog of war
   - Dialogue system with choices

8. **Horror Atmosphere** (`/workspace/crates/game/src/atmosphere.rs`)
   - Implement dread system (increases over time)
   - Dynamic lighting (torches, spells)
   - Fog of war with vision radius
   - Sanity effects (screen distortion, whispers)
   - Corruption spreading on tiles

### Phase 4: Game Feel (2-3 hours)

9. **Juice & Polish**
   - Screen shake on damage
   - Particle effects for attacks
   - Hit-stop for impact feel
   - Death animations
   - Victory/defeat screens

10. **Audio Integration**
    - Footstep sounds (per terrain)
    - Combat sounds (swoosh, impact)
    - Ambient horror music
    - UI feedback sounds
    - Dynamic music intensity

## 🛠️ Technical Implementation Guide

### File Structure You'll Create
```
/workspace/crates/game/src/
├── states.rs       # Game state machine
├── movement.rs     # Hex movement system
├── camera.rs       # Camera controller
├── combat.rs       # Combat system
├── atmosphere.rs   # Horror systems
└── input.rs        # Input handling

/workspace/crates/maps/src/
├── presets.rs      # Premade maps
└── hex_utils.rs    # Hex math helpers

/workspace/crates/ui/src/
├── game_ui.rs      # In-game UI
├── main_menu.rs    # Main menu
└── inventory.rs    # Inventory system
```

### Key Systems to Wire Together

1. **Hex Grid Math** (Already have hexx crate)
   ```rust
   use hexx::{Hex, HexLayout, HexOrientation};
   
   // Convert world pos to hex
   let hex = layout.world_pos_to_hex(world_pos);
   
   // Get neighbors
   let neighbors = hex.all_neighbors();
   ```

2. **ECS Patterns to Use**
   ```rust
   // Player entity
   commands.spawn((
       Player,
       HexPosition(Hex::ZERO),
       Health(100.0),
       Sanity(100.0),
       MovementPoints(3),
   ));
   
   // Enemy entity
   commands.spawn((
       Enemy,
       AIAgent { behavior: Patrol },
       HexPosition(Hex::new(5, 5)),
       Health(50.0),
   ));
   ```

3. **State Transitions**
   ```rust
   fn menu_to_game_transition(
       mut next_state: ResMut<NextState<GameState>>,
   ) {
       next_state.set(GameState::Loading);
       // Load resources...
       next_state.set(GameState::InGame);
   }
   ```

### Testing Your Progress

After each phase, test with:
```bash
cargo run -p dragons_labyrinth
```

Expected milestones:
- **Hour 1**: Player moves on hex grid
- **Hour 2**: Can enter combat with enemy
- **Hour 3**: Enemy fights back with AI
- **Hour 4**: Full game loop with win/lose
- **Hour 5**: Polished with effects and sound

## 🎮 Controls to Implement

```
Movement:
- QWEASD or Mouse: Move on hex grid
- Shift: Run (2x movement cost)
- Tab: Toggle 2D/3D view

Combat:
- Left Click: Select target
- Right Click: Cancel action
- Space: End turn
- 1-9: Use abilities

UI:
- I: Inventory
- M: Map
- J: Journal/Quests
- ESC: Pause menu
- F5: Quick save
- F9: Quick load
```

## 🔥 Pro Tips for YOLO Mode

1. **Don't Over-Engineer**
   - Get it working first, optimize later
   - Use `todo!()` for non-critical features
   - Copy-paste is fine for prototyping

2. **Use What's Already There**
   - Maps crate has hex generation
   - Physics crate has collision
   - VFX crate has particles
   - Save crate handles persistence

3. **Quick Wins**
   - Start with 2D only, add 3D later
   - Use colored squares for art initially
   - Hardcode the first map
   - Use print debugging liberally

4. **If Stuck**
   - Check `/workspace/crates/game/attached_assets/` for design docs
   - The original code in generators.rs has dungeon generation
   - Use bevy examples for reference
   - Make it work, make it right, make it fast (in that order)

## 🚨 Critical Path Only

**Your success metric**: A player can start the game, move around a hex map, fight an enemy, and either win or die. Everything else is optional.

**Time budget**: 8-10 hours of focused work

**What to skip**:
- Perfect code architecture
- Comprehensive error handling  
- Unit tests (for now)
- Asset loading (use primitives)
- Network features
- Advanced AI
- Multiple enemy types
- Complex items/inventory

## 🎯 Definition of Done

You've succeeded when:
1. ✅ Player spawns on hex map
2. ✅ Player can move with keyboard/mouse
3. ✅ At least one enemy exists
4. ✅ Combat works (player and enemy can damage each other)
5. ✅ Player can win (enemy dies) or lose (player dies)
6. ✅ Can save/load game state
7. ✅ Main menu exists with Start/Load/Quit
8. ✅ The game feels spooky (fog, lighting, sounds)

## 💪 You Got This!

The foundation is solid. The architecture is clean. All systems are wired up and compile. Now go make it fun. Ship the prototype. Don't ask permission. Just build.

Remember: A playable bad game is infinitely better than a perfect architecture with no gameplay.

**Start with Phase 1, Task 1: Game States. GO!**

---

*P.S. - If you finish early, the 3D hex view toggle would be really cool. The code is already in `/workspace/crates/maps/src/lib.rs` starting at line 536.*