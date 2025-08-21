# Dragon's Labyrinth - System Patterns

## Core Architecture

### Entity Component System (ECS)
The game uses Bevy's ECS architecture where:
- **Entities**: Unique IDs representing game objects
- **Components**: Data attached to entities (position, health, etc.)
- **Systems**: Functions that process components
- **Resources**: Global singleton data (DreadState, HexWorld, etc.)

### System Organization

```
src/
├── components/     # Data structures
├── systems/        # Logic processors  
├── resources/      # Global state
├── generators/     # Content generation
├── dialogue/       # Narrative system
├── board/          # Board rendering
└── hex_board/      # Hex grid logic
```

## Key Design Patterns

### 1. Dread-Driven Transformation
Every system responds to the global dread level:

```rust
fn any_system(
    dread: Res<DreadState>,
    mut query: Query<&mut Component>
) {
    for mut component in query.iter_mut() {
        component.transform_by_dread(dread.level);
    }
}
```

**Pattern Rules:**
- Systems MUST check dread level
- Behavior changes are discrete per level (0-4)
- Transitions should feel jarring, not smooth
- Later stages override earlier behaviors completely

### 2. Component Composition Over Inheritance
Instead of complex hierarchies, use component combinations:

```rust
// DON'T: Giant monolithic components
struct NPC {
    health: f32,
    dialogue: DialogueTree,
    ai: AIBehavior,
    inventory: Vec<Item>,
    // ... 50 more fields
}

// DO: Composable components
commands.spawn((
    Health(100.0),
    DialogueCapable { tree: "villager" },
    FleeAtDread(2),  // Flees at dread level 2+
    Inventory::default(),
));
```

### 3. Event-Driven Narrative
Narrative progression through events:

```rust
#[derive(Event)]
struct NarrativeEvent {
    event_type: NarrativeEventType,
    actor: Entity,
    context: NarrativeContext,
}

fn narrative_system(
    mut events: EventReader<NarrativeEvent>,
    mut dread: ResMut<DreadState>,
) {
    for event in events.read() {
        match event.event_type {
            NarrativeEventType::BossDefeated => dread.advance(),
            NarrativeEventType::CompanionTrauma => // Handle trauma
            NarrativeEventType::MoralChoice => // Record choice
        }
    }
}
```

### 4. Proximity-Based Horror
Distance calculations drive horror mechanics:

```rust
fn proximity_horror_system(
    player: Query<&HexPosition, With<Player>>,
    dragon: Query<&HexPosition, With<Dragon>>,
    mut audio: ResMut<ProximityAudio>,
) {
    let distance = hex_distance(player_pos, dragon_pos);
    audio.set_intensity(1.0 / distance.max(1.0));
}
```

### 5. Staged System Execution
Systems run in specific order for consistency:

```rust
app.add_systems(Update, (
    // Input first
    handle_input,
    // Then movement
    update_positions,
    // Then game logic
    check_collisions,
    update_dread_progression,
    // Then reactions
    apply_world_corruption,
    update_companion_trauma,
    // Finally rendering
    update_visuals,
).chain());
```

## Component Relationships

### Hex Grid Components
```rust
#[derive(Component)]
struct HexPosition(Hex);  // Axial coordinates

#[derive(Component)]
struct WorldPosition(Vec3);  // Converted 3D position

#[derive(Component)]
struct HexTerrain {
    base_cost: f32,
    terrain_type: TerrainType,
    corrupted: bool,
}
```

### Companion Components
```rust
#[derive(Component)]
struct Companion {
    name: String,
    trauma: f32,  // 0.0 to 1.0
    loyalty: f32,  // Affects betrayal
    dialogue_state: String,
}

#[derive(Component)]
struct TraumaResponse {
    threshold: f32,
    response: TraumaResponseType,
}

enum TraumaResponseType {
    Flee,           // Mira at dread 2
    Break,          // Einar at trauma > 0.8
    Betray,         // Sorin at loyalty < 0.3
    Traumatized,    // Tamara's progression
}
```

### Horror Components
```rust
#[derive(Component)]
struct SanityAffecting {
    radius: f32,
    intensity: f32,
}

#[derive(Component)]
struct Hallucination {
    false_audio: Option<AudioSource>,
    visual_distortion: DistortionType,
}

#[derive(Component)]
struct Stalking {
    target: Entity,
    aggression: f32,
}
```

## Resource Management

### Global Resources
```rust
#[derive(Resource)]
struct DreadState {
    level: u8,  // 0-4
    progress: f32,  // 0.0-1.0 within level
    triggers_remaining: Vec<DreadTrigger>,
}

#[derive(Resource)]
struct HexWorld {
    tiles: HashMap<Hex, Entity>,
    corruption_seeds: Vec<Hex>,
    walkable_cache: HashMap<Hex, bool>,
}

#[derive(Resource)]
struct NarrativeState {
    choices_made: Vec<MoralChoice>,
    companion_states: HashMap<String, CompanionStatus>,
    available_endings: Vec<EndingType>,
}
```

### Asset Resources
```rust
#[derive(Resource)]
struct GeneratedAssets {
    models: HashMap<AssetId, Handle<Scene>>,
    textures: HashMap<AssetId, Handle<Image>>,
    audio: HashMap<AssetId, Handle<AudioSource>>,
}
```

## System Communication Patterns

### 1. Resource Mutation
Systems communicate through shared resources:
```rust
// System A writes
fn trigger_dread(mut dread: ResMut<DreadState>) {
    dread.advance();
}

// System B reads
fn react_to_dread(dread: Res<DreadState>) {
    if dread.is_changed() {
        // React to change
    }
}
```

### 2. Event Broadcasting
Decouple systems with events:
```rust
// Publisher
fn detect_boss_defeat(
    mut events: EventWriter<BossDefeatedEvent>
) {
    events.send(BossDefeatedEvent { boss: "hollow_caretaker" });
}

// Subscriber
fn handle_boss_defeat(
    mut events: EventReader<BossDefeatedEvent>
) {
    for event in events.read() {
        // Process defeat
    }
}
```

### 3. Component Markers
Use marker components for queries:
```rust
#[derive(Component)]
struct Corrupted;  // Marker

fn corrupt_entity(mut commands: Commands, entity: Entity) {
    commands.entity(entity).insert(Corrupted);
}

fn process_corrupted(
    query: Query<Entity, With<Corrupted>>
) {
    // Only processes corrupted entities
}
```

## Performance Patterns

### 1. Query Filtering
Minimize iteration with precise queries:
```rust
// DON'T: Check everything
fn system(query: Query<&Transform>) {
    for transform in query.iter() {
        if let Some(npc) = // Additional checks
    }
}

// DO: Filter in query
fn system(query: Query<&Transform, (With<NPC>, Without<Dead>)>) {
    for transform in query.iter() {
        // Already filtered
    }
}
```

### 2. Change Detection
Only process when needed:
```rust
fn expensive_system(
    query: Query<&Component, Changed<Component>>
) {
    // Only runs for changed components
}
```

### 3. System Ordering
Explicit dependencies prevent race conditions:
```rust
app.add_systems(Update, (
    movement_system,
    collision_system.after(movement_system),
    damage_system.after(collision_system),
));
```

## Integration Patterns

### Hexx Integration
```rust
fn hex_system(
    hex_layout: Res<HexLayout>,
    query: Query<(&HexPosition, &mut WorldPosition)>
) {
    for (hex_pos, mut world_pos) in query.iter_mut() {
        world_pos.0 = hex_layout.hex_to_world_pos(hex_pos.0);
    }
}
```

### Yarn Spinner Integration
```rust
fn dialogue_system(
    yarn_project: Res<YarnProject>,
    mut dialogue_runner: ResMut<DialogueRunner>,
    dread: Res<DreadState>,
) {
    let node = match dread.level {
        0 => "villager_peace",
        1 => "villager_nervous",
        _ => "villager_fled",
    };
    dialogue_runner.start_node(node);
}
```

## Critical Implementation Paths

### 1. Dread Progression Path
```
Player Action → Narrative Event → Dread Check → 
Level Advance → System Updates → World Transform
```

### 2. Combat Resolution Path
```
Attack Input → Validation → Damage Calculation →
Companion Reaction → Trauma Update → Dialogue Change
```

### 3. Horror Proximity Path
```
Position Update → Distance Calculation → 
Audio Intensity → Sanity Effect → Hallucination Spawn
```

### 4. Companion Arc Path
```
Dread Level → Trauma Accumulation → Threshold Check →
Arc Event → Dialogue Update → Potential Departure
```

These patterns ensure consistent, performant, and maintainable code across the entire Dragon's Labyrinth codebase.