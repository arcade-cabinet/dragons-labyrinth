# The TRUE Architecture - Radical Simplification

## THE REVELATION
We've been cargo-culting game dev tools when we already have EVERYTHING WE NEED!

## WHAT WE ACTUALLY NEED

### For Overworld:
```rust
use hexx::{Hex, HexLayout};

pub struct OverworldMap {
    hexes: HashMap<Hex, TerrainType>,
    entities: HashMap<Hex, Vec<Entity>>,
    fog_of_war: HashSet<Hex>,
}
```
**THAT'S IT!** Hexx gives us pathfinding, neighbors, distance, line-of-sight!

### For 3D Dungeons:
```rust
use bevy::prelude::*;

pub struct DungeonScene {
    layout: Vec<Vec<TileType>>,  // Simple 2D array
    walls: Vec<Entity>,           // Spawn wall meshes
    enemies: Vec<Entity>,         // Spawn enemy entities
}
```
**THAT'S IT!** Bevy's transform system handles everything!

### For Dialogue:
```rust
pub struct DialogueTree {
    nodes: HashMap<NodeId, DialogueNode>,
    current: NodeId,
}

pub struct DialogueNode {
    speaker: String,
    text: String,
    choices: Vec<(String, NodeId)>,
    conditions: Vec<Condition>,
}
```
**THAT'S IT!** No yarn files, no external tools!

### For UI:
```rust
use bevy::ui::*;

pub fn spawn_dialogue_ui(commands: &mut Commands, dialogue: &DialogueNode) {
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Px(200.0),
            position_type: PositionType::Absolute,
            bottom: Val::Px(0.0),
            ..default()
        },
        background_color: Color::rgba(0.0, 0.0, 0.0, 0.8).into(),
        ..default()
    }).with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            dialogue.text.clone(),
            TextStyle {
                font_size: 20.0,
                color: Color::WHITE,
                ..default()
            }
        ));
    });
}
```
**THAT'S IT!** Bevy UI is sufficient for our needs!

## WHAT WE'RE DELETING

### ❌ Yoleck
- We don't have "levels" to edit
- We have a progression system
- DELETE ALL YOLECK REFERENCES

### ❌ YarnSpinner  
- We generate dialogue trees directly
- No need for .yarn files
- DELETE ALL YARNSPINNER REFERENCES

### ❌ Cobweb UI
- Bevy's UI is fine for dialogue boxes
- We need custom horror UI anyway
- DELETE ALL COBWEB REFERENCES

### ❌ Complex External Formats
- No .yarn files
- No .cob files
- No .yoleck files
- Just Rust structs!

## THE ACTUAL MINIMAL STACK

```toml
[dependencies]
# Core
bevy = "0.16"

# Hex map
hexx = "0.24"

# Physics (for 3D sections)
avian3d = "0.2"

# Map generation  
mapgen = "0.5"

# Noise for procedural content
noise = "0.10"

# Pathfinding
pathfinding = "4.11"

# Database for content
sqlx = { version = "0.8", features = ["sqlite", "runtime-tokio"] }

# Serialization
serde = "1.0"
serde_json = "1.0"

# THAT'S IT!
```

## WHAT AI GENERATES

Instead of .yarn files, AI generates:
```rust
pub fn generate_companion_dialogue() -> DialogueDatabase {
    DialogueDatabase {
        elena: HashMap::from([
            ("first_meeting", DialogueTree { ... }),
            ("wolf_encounter", DialogueTree { ... }),
            ("death_scar", DialogueTree { ... }),
        ]),
        marcus: HashMap::from([...]),
        quinn: HashMap::from([...]),
    }
}
```

Instead of .yoleck levels, AI generates:
```rust
pub fn generate_world_progression() -> ProgressionDatabase {
    ProgressionDatabase {
        milestones: vec![
            Milestone { 
                progression: 1, 
                event: "the_door",
                triggers: vec![...],
            },
            Milestone {
                progression: 10,
                event: "first_miniboss",
                triggers: vec![...],
            },
        ],
        encounter_tables: HashMap::from([...]),
    }
}
```

## THE BEAUTIFUL SIMPLICITY

### 1. Hex Overworld (Hexx)
```rust
fn move_on_hex_map(
    mut query: Query<&mut Hex, With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if let Ok(mut hex) = query.get_single_mut() {
        if input.just_pressed(KeyCode::W) {
            *hex = *hex + Hex::new(0, -1);
        }
        // Check encounters, trigger events, update fog
    }
}
```

### 2. 3D Dungeons (Bevy)
```rust
fn enter_dungeon(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // Spawn walls
    for wall in dungeon.walls {
        commands.spawn(PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 3.0, 1.0)),
            transform: Transform::from_translation(wall.position),
            ..default()
        });
    }
}
```

### 3. Dialogue (Pure Rust)
```rust
fn show_dialogue(
    dialogue: &DialogueNode,
    ui: &mut Commands,
) {
    spawn_dialogue_box(ui, &dialogue.text);
    for (choice_text, next_node) in &dialogue.choices {
        spawn_choice_button(ui, choice_text, *next_node);
    }
}
```

## WHY THIS IS 1000x BETTER

### 1. **We Control Everything**
- No external file formats
- No version conflicts
- No missing features
- No documentation diving

### 2. **AI Can Generate Directly**
- Rust structs, not yarn files
- Direct serialization
- Type safety
- Immediate compilation

### 3. **Faster Development**
- No tool learning curve
- No integration headaches
- No format conversions
- Just write Rust

### 4. **Perfect for Our Game**
- Horror UI degradation built-in
- Dread system integrated
- Philosophy tracking native
- Death scars visible

## THE REAL ARCHITECTURE

```
game-engine/
├── src/
│   ├── main.rs              // Just Bevy app setup
│   ├── overworld.rs         // Hex movement (Hexx)
│   ├── dungeon.rs           // 3D sections (Bevy)
│   ├── dialogue.rs          // Conversation system
│   ├── progression.rs       // 1-180 journey tracker
│   ├── encounters.rs        // Spawn system
│   └── ui.rs               // Horror-responsive UI
└── Cargo.toml              // Minimal deps

game-content-static/
├── src/
│   ├── world_layout.rs     // The hex map structure
│   ├── progression_arc.rs  // 180-point journey
│   ├── encounter_tables.rs // What spawns where
│   └── dialogue_trees.rs   // All conversations
└── Cargo.toml

game-content-generated/
├── src/
│   ├── generate_world.rs   // Creates hex map
│   ├── generate_dialogue.rs // Creates conversations
│   └── generate_dungeons.rs // Creates 3D layouts
└── Cargo.toml
```

## THE PATH FORWARD

1. **DELETE** all Yoleck code
2. **DELETE** all YarnSpinner code  
3. **DELETE** all Cobweb code
4. **BUILD** simple progression system
5. **BUILD** simple dialogue system
6. **BUILD** simple encounter system
7. **SHIP** the actual game

## THE BOTTOM LINE

We've been trying to use:
- A platformer level editor (Yoleck)
- A visual novel dialogue system (YarnSpinner)
- A traditional UI framework (Cobweb)

For a game that needs:
- A progression tracker
- A dialogue database
- A horror UI system

**LET'S BUILD WHAT WE ACTUALLY NEED!**
