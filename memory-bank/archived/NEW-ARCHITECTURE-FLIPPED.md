# The FLIPPED Architecture - Assets First!

## THE OLD WAY (WRONG)
```
game-engine → uses → game-content-static + game-content-generated
game-content-generated → uses → game-content-static
```

## THE NEW WAY (RIGHT)
```
game-content-static/build.rs → generates ALL content → assets/
game-engine → loads → assets/ (that's it!)
```

## THE BEAUTIFUL SIMPLICITY

### game-content-static/build.rs
```rust
use game_content_generated::{
    generate_all_dialogue,
    generate_all_encounters,
    generate_world_progression,
};

fn main() {
    println!("cargo:rerun-if-changed=src/");
    
    // Generate ALL dialogue trees for ALL NPCs
    let dialogues = generate_all_dialogue();
    
    // Write as RON files to assets (NOT OUT_DIR!)
    for (npc_id, dialogue_tree) in dialogues {
        let path = format!("assets/dialogue/{}.ron", npc_id);
        std::fs::write(path, ron::to_string(&dialogue_tree).unwrap()).unwrap();
    }
    
    // Generate ALL encounters for entire game
    let encounters = generate_all_encounters();
    std::fs::write(
        "assets/encounters/all_encounters.ron",
        ron::to_string(&encounters).unwrap()
    ).unwrap();
    
    // Generate world progression
    let progression = generate_world_progression();
    std::fs::write(
        "assets/progression/world.ron",
        ron::to_string(&progression).unwrap()
    ).unwrap();
}
```

### The Asset Directory (EVERYTHING is here!)
```
crates/game-content-static/assets/
├── models/              # CC0 3D models (curated)
├── textures/            # CC0 textures (curated)
├── audio/               # Sound effects and music
├── fonts/               # Typography
├── dialogue/            # GENERATED dialogue trees (RON)
│   ├── companions/
│   │   ├── elena.ron
│   │   ├── marcus.ron
│   │   └── quinn.ron
│   ├── npcs/
│   │   ├── village_elder.ron
│   │   ├── blacksmith.ron
│   │   └── [200+ more NPCs].ron
│   └── bosses/
│       ├── bandit_leader.ron
│       ├── dragon.ron
│       └── [7 more bosses].ron
├── encounters/          # GENERATED encounter data (RON)
│   ├── all_encounters.ron
│   ├── boss_patterns.ron
│   └── spawn_tables.ron
├── progression/         # GENERATED progression (RON)
│   ├── world.ron
│   ├── milestones.ron
│   └── triggers.ron
└── quests/             # GENERATED quest trees (RON)
    ├── main_quest.ron
    ├── side_quests.ron
    └── companion_arcs.ron
```

### game-engine Just LOADS Assets!
```rust
use bevy::prelude::*;
use bevy_asset_ron::RonAssetPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RonAssetPlugin::<DialogueTree>::new(&["ron"]))
        .add_plugins(RonAssetPlugin::<EncounterTable>::new(&["ron"]))
        .add_plugins(RonAssetPlugin::<WorldProgression>::new(&["ron"]))
        .add_systems(Startup, load_game_content)
        .run();
}

fn load_game_content(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Load dialogue
    let elena_dialogue: Handle<DialogueTree> = 
        asset_server.load("dialogue/companions/elena.ron");
    
    // Load encounters
    let encounters: Handle<EncounterTable> = 
        asset_server.load("encounters/all_encounters.ron");
    
    // Load progression
    let world: Handle<WorldProgression> = 
        asset_server.load("progression/world.ron");
    
    // That's it! Everything is data!
}
```

## WHY RON (Rusty Object Notation)?

### Better than JSON for Rust:
```ron
// RON - Clean, Rust-native
DialogueNode(
    speaker: "Elena",
    text: "The birds have stopped singing...",
    choices: [
        (
            text: "We should investigate",
            next: "elena_agree",
            effects: [ChangeTrust(0.1)],
        ),
    ],
)

// JSON - Verbose, stringly
{
    "speaker": "Elena",
    "text": "The birds have stopped singing...",
    "choices": [{
        "text": "We should investigate",
        "next": "elena_agree",
        "effects": [{"ChangeTrust": 0.1}]
    }]
}
```

## THE 180 PROGRESSION ALGORITHM

Instead of 180 level files, ONE algorithm:

```rust
fn progression_system(
    progression: Res<CurrentProgression>,
    encounters: Res<Assets<EncounterTable>>,
    mut events: EventWriter<SpawnEncounter>,
) {
    // Every progression point, check what should happen
    match progression.current {
        1 => trigger_event("the_door"),
        10 | 30 | 50 | 70 | 90 | 110 | 130 | 150 | 170 => {
            trigger_miniboss(progression.current);
        },
        20 | 40 | 60 | 80 | 100 | 120 | 140 | 160 | 180 => {
            trigger_major_dungeon(progression.current);
        },
        p if p % 5 == 0 => {
            // Every 5 progressions, check for special events
            check_companion_events(p);
            check_mount_events(p);
            check_corruption_spread(p);
        },
        _ => {
            // Normal progression - random encounters
            let table = encounters.get(&current_biome());
            if let Some(encounter) = table.roll_encounter(progression.current) {
                events.send(SpawnEncounter(encounter));
            }
        }
    }
}
```

## THE BUILD PROCESS

### 1. Developer Changes Content Rules
```rust
// In game-content-static/src/characters.rs
pub const COMPANION_ELENA: CompanionDef = CompanionDef {
    name: "Elena",
    archetype: "Caring Shield",
    personality: Personality {
        kindness: 0.9,
        aggression: 0.2,
        loyalty: 0.8,
    },
};
```

### 2. Build.rs Runs Generation
```bash
cd crates/game-content-static
cargo build  # This triggers build.rs
```

### 3. AI Generates Content
```rust
// game-content-generated runs
Generating dialogue for Elena...
Generating dialogue for Marcus...
Generating 247 NPC conversations...
Generating 9 boss encounters...
Generating world progression...
Writing to assets/dialogue/elena.ron...
```

### 4. Assets Are Committed
```bash
git add assets/
git commit -m "Generated: Complete dialogue for all NPCs"
```

### 5. Game Engine Just Works
```bash
cd ../..
cargo run  # Loads all assets, plays game
```

## THE BENEFITS

### 1. **Single Source of Truth**
- ALL assets in one place
- Generated content is versioned
- No OUT_DIR confusion

### 2. **AI Integration**
- Generate once, commit forever
- Can regenerate any time
- Easy to review changes

### 3. **Simple Game Engine**
- Just loads assets
- No generation at runtime
- No complex dependencies

### 4. **Easy Testing**
- Can edit RON files directly
- Can test without generation
- Can mock assets easily

### 5. **Clear Separation**
- Static assets: models, textures
- Generated assets: dialogue, encounters
- Game engine: just gameplay

## THE FINAL STRUCTURE

```
crates/
├── game-content-static/
│   ├── src/             # Game rules and definitions
│   ├── assets/          # ALL game assets (static + generated)
│   └── build.rs         # Calls game-content-generated
├── game-content-generated/
│   ├── src/             # AI generation code
│   └── Cargo.toml       # OpenAI, etc
└── game-engine/
    ├── src/             # Just the game loop
    └── Cargo.toml       # Just Bevy + RON

NO OTHER CRATES NEEDED!
```

## THE BOTTOM LINE

1. **Curate assets** with Sonnet 4
2. **Generate content** into assets/
3. **Commit everything**
4. **Game engine just loads**
5. **Ship the game**

This is SO MUCH CLEANER!
