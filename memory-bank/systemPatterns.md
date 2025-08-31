# System Patterns & Architecture

## Core Architecture Pattern

### Content-Driven Development
```
Markdown (source of truth)
    ↓
AI Generation (Python + OpenAI)
    ↓
JSON Data (build artifacts)
    ↓
Game Runtime (Rust/Bevy)
```

## Technical Patterns

### 1. ECS (Entity Component System) Pattern
**Framework**: Bevy 0.16.1

**Core Concepts**:
- **Entities**: Just IDs that group components
- **Components**: Pure data structs (position, health, etc.)
- **Systems**: Functions that query and modify components
- **Resources**: Global singletons (WorldBook, PlayerState)

**Example Pattern**:
```rust
// Component
#[derive(Component)]
struct HexPosition { q: i32, r: i32 }

// System
fn movement_system(
    mut query: Query<&mut HexPosition, With<Player>>,
    input: Res<Input<KeyCode>>
) {
    // Logic here
}

// Registration
app.add_systems(Update, movement_system)
```

### 2. Hot-Reload Pattern
**Purpose**: Instant content testing without restart

**Implementation**:
- R key triggers reload
- Read worldbook.json from disk
- Replace Resource in ECS
- Systems automatically use new data

**Benefits**:
- Rapid iteration on content
- No compilation for data changes
- Designer-friendly workflow

### 3. Hex Grid Pattern
**Coordinate System**: Axial (q, r)

**Movement Mapping**:
- Q: Northwest
- W: North
- E: Northeast  
- A: Southwest
- S: South
- D: Southeast

**Math Utilities** (`hex.rs`):
- Distance calculation
- Neighbor finding
- Line of sight
- Pathfinding ready

### 4. Plugin Architecture
**Bevy Plugin Pattern**:
```rust
pub struct WorldPlugin;
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(...)
           .add_systems(Startup, ...)
           .add_systems(Update, ...);
    }
}
```

**Benefits**:
- Modular feature addition
- Clear dependency management
- Easy enable/disable of features

### 5. Material Shader Pattern
**Custom Hex Rendering**:
```rust
Material2dPlugin::<HexTileMaterial>::default()
```

**Capabilities**:
- Custom shaders for hex tiles
- Biome-specific rendering
- Fog of war effects
- Lighting overlays

## Data Flow Patterns

### 1. Generation Pipeline Pattern
```
canonize: Architecture.md → canon.json
    ↓
plan: canon + themes → world plan
    ↓
expand: plan → detailed regions
    ↓
image-plan: themes → asset designs
    ↓
images: designs → actual tilesets
    ↓
narrative: world → dialogue/quests
```

### 2. JSON Schema Pattern
**Tool**: Pydantic models

**Benefits**:
- Type-safe generation
- Automatic validation
- Clear contracts between systems
- IDE autocomplete

**Example**:
```python
class WorldBook(BaseModel):
    plan: WorldPlan
    regions: list[RegionBible]
    
# Usage
wb = WorldBook.model_validate_json(data)
```

### 3. Resource Loading Pattern
```rust
fn load_worldbook(mut commands: Commands) {
    let text = std::fs::read_to_string("build/world/worldbook.json")
        .expect("worldbook.json");
    let wb: WorldBook = serde_json::from_str(&text)
        .expect("valid worldbook");
    commands.insert_resource(wb);
}
```

**Key Points**:
- Load once at startup
- Deserialize to strongly-typed structs
- Insert as global resource
- Systems query resource as needed

## AI Integration Patterns

### 1. Prompt Engineering Pattern
**Structure**:
```python
SYSTEM_CREATIVE = "You are a horror RPG designer..."
SYSTEM_IMAGE = "You are an art director..."

prompt = f"""{SYSTEM_CREATIVE}
Source: {markdown_content}
Task: {specific_instruction}
Return JSON only.
"""
```

**Best Practices**:
- Clear role definition
- Structured input format
- Explicit output requirements
- JSON schema enforcement

### 2. Response Validation Pattern
```python
response_format={
    "type": "json_schema",
    "json_schema": {
        "name": "ModelName",
        "schema": Model.model_json_schema()
    }
}
```

**Benefits**:
- Guaranteed valid JSON
- Type-safe responses
- No parsing errors
- Predictable structure

### 3. Incremental Generation Pattern
**Approach**: Build world in stages, each informed by previous

**Benefits**:
- Maintains consistency
- Allows refinement
- Reduces token usage
- Easier debugging

## Game System Patterns

### 1. Band Progression Pattern
**Structure**:
- Bands define level ranges (1-20, 21-40, etc.)
- Each band has unique mechanics
- Progressive horror intensity
- Gating mechanisms between bands

**Implementation**:
- Check player distance from origin
- Map distance to band
- Apply band-specific rules
- Trigger band transitions

### 2. Inverted Combat Pattern
**Traditional**: Win → Get stronger
**Our Pattern**: Win → Get weaker

**Mechanics**:
- Health is currency for actions
- Victory costs permanent HP
- Healing is rare and precious
- Death is strategic retreat

### 3. Companion Trauma Pattern
**State Tracking**:
```rust
struct Companion {
    trauma_level: f32,
    breaking_points: Vec<String>,
    memories: Vec<TraumaticEvent>,
}
```

**Progression**:
- Events increase trauma
- Trauma affects behavior
- Breaking points trigger changes
- Some trauma is permanent

### 4. Forge Redemption Pattern
**Concept**: Second chances at great cost

**Mechanics**:
- Identify redemption opportunity
- Pay cost (health/companion/item)
- Reverse specific consequence
- Cannot undo everything

## Performance Patterns

### 1. Lazy Loading Pattern
- Load only visible hex tiles
- Stream in adjacent regions
- Unload distant content
- Maintain small memory footprint

### 2. System Ordering Pattern
```rust
app.add_systems(Update, (
    input_system,
    movement_system,
    encounter_system,
    render_system
).chain())
```

**Benefits**:
- Predictable execution order
- Avoid race conditions
- Optimize cache usage

### 3. Query Optimization Pattern
```rust
Query<&Transform, (With<Player>, Without<Enemy>)>
```

**Benefits**:
- Filter at query time
- Reduce iteration count
- Better cache locality

## Content Patterns

### 1. Biome Consistency Pattern
- Each band has specific biome set
- Biomes have tileset variants
- Smooth transitions between biomes
- Environmental storytelling

### 2. POI Distribution Pattern
- Villages: Safe havens (rare)
- Shrines: Forge access points
- Lairs: Combat encounters
- Ruins: Lore and items
- Dungeons: Major challenges
- Camps: Rest but risky
- Portals: Band transitions

### 3. NPC Dialogue Pattern
**Structure**:
```json
{
  "npc_id": "guardian_ella",
  "dialogue_trees": {
    "first_meeting": [...],
    "quest_active": [...],
    "post_trauma": [...]
  }
}
```

**Progression**:
- Context-aware responses
- Trauma-modified dialogue
- Quest state tracking
- Relationship evolution

## Development Patterns

### 1. Markdown-First Pattern
**Workflow**:
1. Design in markdown
2. Generate via AI
3. Test in game
4. Iterate on markdown
5. Regenerate

**Benefits**:
- Version control friendly
- Designer accessible
- Clear documentation
- Fast iteration

### 2. Fail-Fast Pattern
```rust
.expect("worldbook.json must exist")
.expect("valid JSON structure")
```

**Philosophy**:
- Crash early in development
- Clear error messages
- No silent failures
- Easy debugging

### 3. Single Source of Truth
- Architecture.md defines game rules
- Themes.md defines aesthetics
- Generated JSON is disposable
- Markdown is versioned

## Anti-Patterns to Avoid

### 1. Database Complexity
**Avoid**: Complex ORM, migrations, queries
**Use**: Direct JSON loading

### 2. Over-Abstraction
**Avoid**: Generic systems, deep inheritance
**Use**: Specific, simple functions

### 3. Defensive Programming
**Avoid**: Null checks, fallbacks, recovery
**Use**: Expect valid data, fail fast

### 4. Runtime Generation
**Avoid**: Procedural generation during play
**Use**: Pre-generated content, hot-reload

## Success Patterns

### 1. Continuous Integration
- Every change builds
- All tests pass
- Content generates
- Game runs

### 2. Playtesting Loop
- Implement feature
- Hot-reload test
- Get feedback
- Iterate quickly

### 3. Content Pipeline
- Markdown edit
- AI generation
- JSON validation  
- Game integration
- Player experience

These patterns form the foundation of our technical architecture, ensuring consistent, maintainable, and performant development of Dragon's Labyrinth.
