# Narrative Storage Architecture

## The Problem
Dragon's Labyrinth has MASSIVE narrative content:
- 12 companion archetypes × multiple dialogue trees
- Branching quests with moral choices
- Dynamic relationship graphs
- Context-sensitive dialogue based on dread/trauma/trust

## What Stays in Build-Tools (AI Generation)

### 1. YarnSpinner Dialogue Generation
```rust
// build-tools generates .yarn files with:
- Companion dialogue trees (12 archetypes × 5 dread levels × N situations)
- Quest dialogues with branching
- NPC conversations
- Hallucination/breakdown dialogue
```

### 2. Narrative Graph Generation
```rust
// Generate Cobweb-compatible story graphs:
- Main narrative branches
- Companion story arcs
- Side quest chains
- Ending variations
```

## Storage Architecture

### SQLite for Narrative Content (game.db)
```sql
-- Core narrative tables
CREATE TABLE dialogues (
    id TEXT PRIMARY KEY,
    yarn_file TEXT,        -- Path to .yarn file
    character TEXT,
    dread_level INTEGER,
    context TEXT,          -- "first_meeting", "breakdown", etc.
    prerequisites JSON     -- Required flags/states
);

CREATE TABLE quests (
    id TEXT PRIMARY KEY,
    title TEXT,
    description TEXT,
    stages JSON,           -- Complex quest stage data
    moral_weight TEXT,
    dread_requirement INTEGER,
    generated_by TEXT      -- Which AI agent created this
);

CREATE TABLE story_nodes (
    id TEXT PRIMARY KEY,
    content TEXT,
    branches JSON,
    requirements JSON,
    cobweb_graph TEXT      -- Reference to graph structure
);

CREATE TABLE companion_dialogues (
    companion_archetype TEXT,
    trauma_level REAL,
    context TEXT,
    dialogue_id TEXT,
    FOREIGN KEY (dialogue_id) REFERENCES dialogues(id)
);
```

### YarnSpinner Integration
```rust
// In game-engine/src/systems/narrative.rs
pub struct NarrativeDatabase {
    db: SqliteConnection,
    yarn_runner: YarnRunner,
    active_dialogues: HashMap<Entity, String>,
}

impl NarrativeDatabase {
    pub fn load_dialogue_for_context(
        &mut self,
        character: &str,
        dread: u8,
        trauma: f32,
        context: &str,
    ) -> Option<YarnProgram> {
        // Query database for appropriate dialogue
        // Load .yarn file
        // Inject current game state variables
    }
}
```

## What Moves to Direct Implementation

### 1. Map Generation
- Use `mapgen` algorithms directly
- No AI needed - procedural rules work fine
- Corruption overlays based on dread level

### 2. Audio Processing
- Sonnet writes the integration code during asset curation
- Use existing audio files from CC0 assets
- Layer and mix based on game state

### 3. Visual Assets
- Sonnet identifies what's missing during curation
- Generates .bpy scripts for Blender if needed
- Direct integration into ECS

## Build-Tools Becomes Narrative-Focused

```rust
// crates/build-tools/src/agents/
dialogue.rs    // Generates YarnSpinner files
quests.rs      // Creates quest chains
relationships.rs // Companion relationship trees
story_graph.rs // Cobweb narrative graphs

// Output structure:
target/
└── generated/
    ├── dialogues/     # .yarn files
    ├── quests/        # .json quest definitions
    ├── graphs/        # .cobweb story graphs
    └── manifest.json  # What was generated
```

## The Key Insight

**Heavy AI Generation Needed For:**
- Dialogue (too much to write manually)
- Quest variations (moral complexity)
- Relationship dynamics (12 companions × interactions)
- Narrative branching (exponential possibilities)

**Direct Implementation For:**
- Maps (procedural algorithms work)
- Audio (mix existing assets)
- Visual corruption (shaders + math)
- Physics/gameplay (standard patterns)

## Database as Content Management

The SQLite database becomes our **content management system**:
- AI generates content → stored in DB
- Game queries DB for appropriate content
- YarnSpinner/Cobweb reference DB entries
- Easy to patch/update narrative post-launch

This is why `game.db` ships with the game - it's not just metadata, it's the actual narrative content of the game!
