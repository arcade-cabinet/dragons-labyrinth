# Quick Reference for Dragon's Labyrinth Development

## 🗂️ Where Everything Lives

### Core Game Files
```bash
# Main game entry point
/workspace/crates/game/src/main.rs

# Original game systems (reference these!)
/workspace/crates/game/src/systems.rs      # Movement, combat logic
/workspace/crates/game/src/generators.rs   # Dungeon generation
/workspace/crates/game/src/cutscenes.rs    # State machine example

# Design documents and assets
/workspace/crates/game/attached_assets/    # Screenshots, design docs
/workspace/crates/assets/assets/           # Game assets
```

### Crate Responsibilities
```rust
dragons_core    → Components & Resources (Player, Enemy, Health, etc.)
dragons_game    → Main game loop and system coordination
dragons_ui      → All UI (menus, HUD, inventory)
dragons_maps    → Map generation and hex grid logic
dragons_levels  → Level editor and serialization
dragons_ai      → Enemy behaviors and pathfinding
dragons_audio   → Sound effects and music
dragons_physics → Collision and physics
dragons_vfx     → Particles and visual effects
dragons_save    → Save/load game state
dragons_assets  → Asset loading and caching
dragons_tools   → Dev tools and utilities
```

## 🔧 Common Commands

```bash
# Build everything
cargo build --all

# Run the game
cargo run -p dragons_labyrinth

# Check for errors without building
cargo check --all

# Run with hot reload
cargo watch -x "run -p dragons_labyrinth"

# Build for release
cargo build --release

# Update dependencies
cargo update
```

## 📦 Key Dependencies Already Setup

### Hex Grid Operations
```rust
use hexx::{Hex, HexLayout, HexOrientation};

// Create hex
let hex = Hex::new(q, r);

// Get neighbors
let neighbors = hex.all_neighbors();

// Distance between hexes
let distance = hex1.distance(hex2);

// Line between hexes
let line = hex1.line_to(hex2);
```

### Physics (Avian)
```rust
use avian3d::prelude::*;

commands.spawn((
    RigidBody::Dynamic,
    Collider::sphere(0.5),
    LinearVelocity(Vec3::ZERO),
));
```

### UI (Cobweb)
```rust
use bevy_cobweb::prelude::*;

// Reactive UI example
let button = commands.spawn_button()
    .with_text("Start Game")
    .on_click(|mut state: ResMut<GameState>| {
        *state = GameState::Playing;
    });
```

### VFX (Hanabi + MotionGfx)
```rust
use bevy_hanabi::prelude::*;
use motiongfx_bevy::prelude::*;

// Spawn particle effect
commands.spawn(ParticleEffectBundle {
    effect: effects.fire.clone(),
    transform: Transform::from_translation(pos),
    ..default()
});
```

### Save System
```rust
use dragons_save::{SaveGameEvent, LoadGameEvent, SaveSlot};

// Save game
events.send(SaveGameEvent {
    slot: SaveSlot::Manual(1),
    description: Some("Before boss fight".into()),
});

// Load game
events.send(LoadGameEvent {
    slot: SaveSlot::Manual(1),
});
```

## 🎮 ECS Patterns

### Component Bundles
```rust
#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    health: Health,
    sanity: Sanity,
    hex_pos: HexPosition,
    sprite: SpriteBundle,
}
```

### System Organization
```rust
// Input → Logic → Physics → Render
app.add_systems(Update, (
    handle_input,
    update_movement,
    apply_physics,
    render_sprites,
).chain());
```

### Event Pattern
```rust
#[derive(Event)]
struct DamageEvent {
    target: Entity,
    amount: f32,
}

// Send event
damage_events.send(DamageEvent { target, amount: 10.0 });

// Handle event
fn handle_damage(
    mut events: EventReader<DamageEvent>,
    mut health: Query<&mut Health>,
) {
    for event in events.read() {
        if let Ok(mut hp) = health.get_mut(event.target) {
            hp.current -= event.amount;
        }
    }
}
```

## 🐛 Debug Tips

### Quick Debug Rendering
```rust
// Draw debug sphere
gizmos.sphere(position, Quat::IDENTITY, 0.5, Color::RED);

// Draw debug line
gizmos.line(start, end, Color::GREEN);

// Draw debug text
gizmos.text_2d(position, "Debug Info", Color::WHITE);
```

### Logging
```rust
use bevy::log::{info, warn, error, debug};

info!("Game started");
debug!("Player position: {:?}", position);
warn!("Low health: {}", health);
error!("Failed to load asset: {}", path);
```

### Inspector
```rust
// Add to any entity for runtime inspection
#[derive(Component, Reflect)]
#[reflect(Component)]
struct DebugMe;
```

## 🎨 Asset Paths

```rust
// Load texture
let texture = asset_server.load("sprites/player.png");

// Load sound
let sound = asset_server.load("sounds/footstep.ogg");

// Load font
let font = asset_server.load("fonts/main.ttf");
```

## 🔥 Performance Tips

1. **Use Changed/Added filters**
   ```rust
   Query<&Transform, Changed<Transform>>
   ```

2. **Batch operations**
   ```rust
   commands.spawn_batch(entities_to_spawn);
   ```

3. **Use resources for singletons**
   ```rust
   #[derive(Resource)]
   struct GameSettings { ... }
   ```

4. **Profile with tracy**
   ```rust
   #[profiling::function]
   fn expensive_function() { ... }
   ```

## 📝 TODO Markers

Use these in code for quick navigation:
```rust
todo!("Implement this feature");      // Compiles but panics
unimplemented!("Not yet done");       // Same as todo!
unreachable!("Should never happen");  // For impossible states

// TODO: Add sound effects here
// FIXME: Optimize this loop
// HACK: Temporary solution
// NOTE: Important information
```

## 🚀 Go Build!

You have everything you need. The crates are set up, dependencies are installed, and the architecture is solid. Focus on making it playable, not perfect.

**Your first file to create**: `/workspace/crates/game/src/states.rs`

Start typing. Ship code. Make games.