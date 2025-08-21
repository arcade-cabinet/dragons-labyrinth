# Dragon's Labyrinth - Technical Context

## Technology Stack

### Core Technologies
- **Language**: Rust (Edition 2021, Version 1.88+)
- **Game Engine**: Bevy 0.16.1
- **Build Targets**: Native (Linux/Windows/Mac) + WebAssembly
- **Graphics**: wgpu (cross-platform graphics API)
- **Audio**: Bevy Audio (Rodio backend)

### Key Dependencies

#### Game Systems
- **hexx** (0.21): Hexagonal grid math, pathfinding, FOV
- **bevy_ecs_tilemap** (0.16): Efficient tile rendering
- **mapgen** (0.6): Procedural map generation
- **bevy-yarn-spinner**: Dialogue system (pending integration)
- **yoleck**: Level editor (pending integration)
- **cobweb-ui**: Declarative UI (pending integration)

#### Core Libraries
- **serde** (1.0): Serialization for save games
- **serde_json** (1.0): JSON data format
- **rand** (0.8): Random number generation
- **fastrand** (2.0): Fast RNG for non-crypto uses

#### Database & Storage
- **diesel** (2.2): ORM for game database
- **rusqlite** (0.32): SQLite integration
- **diesel_migrations** (2.2): Database migrations

#### Development Tools
- **wasm-bindgen**: WebAssembly bindings
- **web-sys**: Web API access
- **console_error_panic_hook**: WASM debugging
- **wasm-bindgen-futures**: Async support in WASM

## Development Setup

### Prerequisites
```bash
# Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default stable
rustup target add wasm32-unknown-unknown

# WASM tools
cargo install wasm-bindgen-cli
cargo install basic-http-server

# Database tools (optional)
cargo install diesel_cli --no-default-features --features sqlite
```

### Project Structure
```
dragons-labyrinth/
├── src/
│   ├── main.rs              # Entry point
│   ├── components/          # ECS components
│   ├── systems/            # Game systems
│   ├── resources/          # Global resources
│   ├── generators/         # Content generation
│   ├── dialogue/           # Narrative system
│   ├── board/              # Board rendering
│   ├── hex_board/          # Hex grid logic
│   └── assets/             # Asset management
├── assets/
│   ├── models/            # 3D models (.glb)
│   ├── textures/          # Images (.png)
│   ├── audio/             # Sounds (.ogg)
│   ├── fonts/             # Typography (.ttf)
│   └── ui/                # UI elements
├── memory-bank/           # Documentation
├── target/                # Build outputs
└── Cargo.toml            # Dependencies
```

### Build Commands

#### Native Development
```bash
# Debug build (fast compile, slow runtime)
cargo run

# Release build (slow compile, fast runtime)
cargo build --release
./target/release/dragons_labyrinth

# Run with features
cargo run --features debug_ui
```

#### WebAssembly Build
```bash
# Build WASM module
./build_wasm.sh

# Or manually:
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./out/ --target web \
    ./target/wasm32-unknown-unknown/release/dragons_labyrinth.wasm

# Serve locally
python3 -m http.server 8000
# Visit http://localhost:8000
```

## Technical Constraints

### Performance Requirements
- **Frame Rate**: 60 FPS (desktop), 30 FPS (mobile)
- **Memory**: < 200MB total usage
- **Load Time**: < 2 seconds per area
- **Hex Tiles**: Support 10,000+ simultaneous

### Platform Constraints

#### WebAssembly Limitations
- No threading (use Bevy's async systems)
- Limited memory (4GB maximum)
- No file system access (use browser storage)
- Audio restrictions (user interaction required)

#### Mobile Considerations
- Touch input only (no hover states)
- Limited GPU memory
- Battery usage optimization needed
- Variable screen sizes/ratios

### Asset Constraints
- **Models**: .glb format, < 100k vertices
- **Textures**: Power-of-2 dimensions, < 2048x2048
- **Audio**: .ogg format, compressed
- **Total Size**: < 50MB initial download

## Development Patterns

### Bevy-Specific Patterns

#### System Registration
```rust
app.add_systems(Startup, setup_system)
   .add_systems(Update, (
       input_system,
       movement_system,
       collision_system,
   ).chain())  // Explicit ordering
   .add_systems(FixedUpdate, physics_system);
```

#### Resource Management
```rust
// Initialize resources
app.init_resource::<GameState>()
   .insert_resource(Settings::default());

// Access in systems
fn system(settings: Res<Settings>, mut state: ResMut<GameState>) {
    // Use resources
}
```

#### Component Queries
```rust
// Basic query
Query<&Transform, With<Player>>

// Mutable query
Query<&mut Health, (With<Enemy>, Without<Dead>)>

// Complex query
Query<(Entity, &Name, &mut Position), 
      (Changed<Position>, With<Visible>)>
```

### Error Handling
```rust
// Use Result for fallible operations
fn load_asset(path: &str) -> Result<Handle<Image>, AssetError> {
    // ...
}

// Graceful degradation
match load_asset("texture.png") {
    Ok(handle) => use_texture(handle),
    Err(_) => use_default_texture(),
}
```

### Performance Optimization

#### Entity Pooling
```rust
// Reuse entities instead of spawn/despawn
fn recycle_entity(
    mut commands: Commands,
    entity: Entity,
) {
    commands.entity(entity)
        .remove::<Active>()
        .insert(Pooled);
}
```

#### Batch Operations
```rust
// Process multiple entities together
fn batch_system(
    mut query: Query<(&mut Transform, &Velocity)>,
) {
    query.par_iter_mut().for_each(|(mut transform, velocity)| {
        transform.translation += velocity.0;
    });
}
```

## Tool Usage

### Cargo Commands
```bash
# Check compilation without building
cargo check

# Run tests
cargo test

# Format code
cargo fmt

# Lint code
cargo clippy

# Update dependencies
cargo update

# Audit for security issues
cargo audit
```

### Database Management
```bash
# Run migrations
diesel migration run

# Revert migration
diesel migration revert

# Create new migration
diesel migration generate add_companions
```

### Asset Pipeline
```bash
# Optimize models (external tool)
gltf-transform optimize input.glb output.glb

# Convert audio
ffmpeg -i input.wav -c:a libvorbis -q:a 4 output.ogg

# Generate texture atlases
# (Custom tool needed)
```

## Debugging & Profiling

### Debug Features
```rust
#[cfg(feature = "debug")]
fn debug_system(
    mut gizmos: Gizmos,
    query: Query<&Transform, With<Enemy>>,
) {
    for transform in query.iter() {
        gizmos.sphere(transform.translation, 1.0, Color::RED);
    }
}
```

### Performance Profiling
```rust
// Use Bevy's built-in diagnostics
app.add_plugins(FrameTimeDiagnosticsPlugin)
   .add_plugins(EntityCountDiagnosticsPlugin);

// Custom timing
let start = Instant::now();
expensive_operation();
info!("Operation took: {:?}", start.elapsed());
```

### WASM Debugging
```javascript
// In browser console
console.log(Module._get_game_state());
performance.measure("frame");
```

## Integration Points

### Asset Loading
```rust
fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let texture = asset_server.load("textures/grass.png");
    let model = asset_server.load("models/tree.glb#Scene0");
    let sound = asset_server.load("audio/wind.ogg");
}
```

### Save System
```rust
#[derive(Serialize, Deserialize)]
struct SaveGame {
    dread_level: u8,
    player_position: (i32, i32),
    companion_states: Vec<CompanionSave>,
}

fn save_game(world: &World) -> Result<(), SaveError> {
    let save_data = extract_save_data(world);
    let json = serde_json::to_string(&save_data)?;
    // Write to platform-specific storage
}
```

### Platform-Specific Code
```rust
#[cfg(target_arch = "wasm32")]
fn platform_storage() -> Box<dyn Storage> {
    Box::new(BrowserStorage::new())
}

#[cfg(not(target_arch = "wasm32"))]
fn platform_storage() -> Box<dyn Storage> {
    Box::new(FileStorage::new())
}
```

## Deployment Configuration

### Native Release
```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
strip = true
```

### WASM Optimization
```toml
[profile.wasm-release]
inherits = "release"
opt-level = "z"  # Size optimization
```

### CI/CD Pipeline
```yaml
# GitHub Actions example
- name: Build WASM
  run: |
    cargo build --release --target wasm32-unknown-unknown
    wasm-bindgen --out-dir dist --target web target/wasm32-unknown-unknown/release/*.wasm
    wasm-opt -Oz dist/*.wasm -o dist/optimized.wasm
```

## Known Technical Debt

### Current Issues
- Splatmap shader not implemented
- Yarn Spinner integration pending
- Audio spatialization incomplete
- Save system not implemented

### Future Improvements
- Implement LOD system for distant objects
- Add texture streaming for large worlds
- Optimize hex grid queries with spatial indexing
- Implement predictive asset loading

### Performance Bottlenecks
- Hex grid pathfinding for many units
- Corruption spread calculation
- Dynamic lighting updates
- Large dialogue tree parsing

## External Resources

### Documentation
- [Bevy Book](https://bevyengine.org/learn/book/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [WebAssembly Guide](https://rustwasm.github.io/docs/book/)
- [Hexx Docs](https://docs.rs/hexx/)

### Community
- [Bevy Discord](https://discord.gg/bevy)
- [Rust GameDev](https://gamedev.rs/)
- [/r/rust_gamedev](https://reddit.com/r/rust_gamedev)

### Tools
- [Bevy Inspector](https://github.com/jakobhellermann/bevy-inspector-egui)
- [Bevy Asset Loader](https://github.com/NiklasEi/bevy_asset_loader)
- [Bevy Rapier](https://rapier.rs/) (physics, if needed)