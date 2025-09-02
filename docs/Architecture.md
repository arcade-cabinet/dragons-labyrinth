# Dragon's Labyrinth - Technical Architecture

## Overview
Dragon's Labyrinth is a horror RPG with an inverted power curve, built on a coherent hexagonal overworld. The game follows a south-to-north journey through the world of Aethermoor, from forgotten coastal villages to the dragon's reality-breaking labyrinth.

## Core Design Philosophy
"A horror experience that happens to have RPG mechanics" - The journey from hope to cosmic horror IS the game.

## World Implementation
See `WorldDesign.md` for complete world structure. Key technical points:
- **One Coherent Overworld**: Not disconnected regions but a logical continent
- **Hexagonal Tile System**: Each hex ~1 mile, with biome + feature overlays
- **Level = Distance**: Progression tied to geographic journey northward
- **Three-Act Structure**: Political progression through defined territories

## Technical Stack

### Game Engine
- **Framework**: Bevy 0.16.1 (Rust)
- **Architecture**: ECS (Entity Component System)
- **Physics**: Avian2d for hex movement, Avian3d for FPS sections
- **Rendering**: bevy_ecs_tilemap for overworld, raycasting for dungeons

### Content Generation
- **Builder**: Rust crate with minijinja templates
- **AI Integration**: openai-dive + tiktoken-rs for gpt-5
- **Idempotent Generation**: Same prompt = same compatible code
- **Scene-Based Prompts**: Location-specific content generation

## Key Systems

### 1. Hex Movement System
```rust
pub struct HexTile {
    coord: HexCoord,        // Axial coordinates (q, r)
    biome: BiomeType,       // Base terrain type
    features: Vec<Feature>, // Overlays (roads, buildings, etc)
    corruption: f32,        // Void corruption level
}
```
- Movement: Q/W/E/A/S/D for six directions
- Variable movement costs by terrain
- Exploration reveals adjacent tiles
- Chunk-based loading for performance

### 2. Inverted Power System
```rust
pub struct PlayerStats {
    max_health: u32,        // Decreases with "progress"
    current_health: u32,    // Currency for actions
    corruption: f32,        // Increases toward north
    sanity: f32,           // Depletes with horror exposure
}
```
- Combat costs permanent HP
- Healing extremely rare
- Death is strategic retreat
- Power makes you weaker

### 3. Companion Psychology
```rust
pub struct Companion {
    trust: f32,             // Built through choices
    trauma: f32,            // Accumulates from events
    breaking_points: Vec<Event>, // Triggers for changes
    loyalty: LoyaltyState,  // Can betray or transcend
}
```
- Trauma affects dialogue and combat
- Breaking points trigger permanent changes
- Relationships define available options
- Forge trials may require sacrifice

### 4. Dread Progression Controller
```rust
pub enum DreadLevel {
    Peace,      // Levels 1-20: Subtle wrongness
    Unease,     // Levels 21-40: Visible decay
    Dread,      // Levels 41-60: Environmental horror
    Terror,     // Levels 61-120: Social collapse
    Madness,    // Levels 121-150: Reality breaks
    Void,       // Levels 151-180: Cosmic horror
}
```
- Affects all game systems
- Modifies asset selection
- Changes NPC behavior
- Corrupts world geometry

### 5. Perspective Transition
```rust
pub enum ViewMode {
    Overworld,   // Hex-based exploration (2.5D)
    Dungeon,     // First-person horror (3D)
    Transition,  // Smooth camera shift
}
```
- Overworld: Tactical hex movement
- Dungeons: DOOM-style raycasting
- Boss fights: Full 3D with physics
- Labyrinth: Reality-breaking FPS

## Content Pipeline

### 1. World Generation Flow
```
WorldDesign.md → Scene Templates → AI Prompts → Generated Code → Game
```

### 2. Scene-Based Generation
Each location requires:
- **Visual Description**: Environmental details
- **Audio Landscape**: Ambient sounds
- **Emotional Tone**: Horror level
- **Mechanical Elements**: Gameplay features
- **Narrative Threads**: Quest connections

### 3. Asset Integration
- **Base Assets**: CC0 models/textures
- **Dread Variants**: Progressive corruption
- **Dynamic Loading**: Viewport-based streaming
- **Horror Overlays**: Runtime corruption effects

## Memory & Performance

### Chunk Loading
```rust
pub struct ChunkManager {
    loaded: HashMap<ChunkCoord, WorldChunk>,
    view_radius: u32,      // Tiles to keep loaded
    max_chunks: usize,     // Memory limit
}
```
- Load chunks around player
- Unload distant chunks
- Preload based on movement
- Cache frequently visited areas

### Asset Streaming
- **Overworld**: Low-detail hex tiles
- **Dungeons**: High-detail room assets
- **Transitions**: Preload during travel
- **Variants**: Cache dread-level assets

## Boss Encounters (Scripted Content)

### Major Bosses (9 total)
1. **The Bandit Chief** (Level 20): Leadership consequences
2. **The Corrupt Knight** (Level 40): Fallen hero
3. **The Dragonbrood** (Level 60): Kingdom entry
4. **The Mad Noble** (Level 80): Political decay
5. **The King** (Level 120): Power corrupts
6. **The Forge Guardian** (Level 140): Mythic trial
7. **The Truth Speaker** (Level 160): Reality breaks
8. **Your Shadow Self** (Level 170): Internal conflict
9. **The Dragon** (Level 180): Cosmic revelation

### Mini-Bosses (~20 total)
Positioned at narrative moments throughout the journey.

## Narrative Integration

### Quest Architecture
- **Main Thread**: 11 major quests following geographic progression
- **Companion Quests**: Personal stories tied to locations
- **Moral Choices**: Permanent consequences
- **Environmental Stories**: World tells its own tale

### Dialogue System
- **Context-Aware**: Based on location, dread, relationships
- **Trauma-Modified**: Companion state affects options
- **Choice Memory**: Past decisions referenced
- **Horror Integration**: Sanity affects perception

## Implementation Phases

### Phase 1: Core World (Current)
- Hex tile system
- Basic movement
- Biome generation
- Chunk loading

### Phase 2: Horror Systems
- Dread progression
- Companion psychology
- Inverted combat
- Asset corruption

### Phase 3: Content Generation
- Scene templates
- AI integration
- Quest threading
- Dialogue trees

### Phase 4: Polish
- FPS transitions
- Boss encounters
- Audio landscapes
- Performance optimization

## Success Metrics

### Technical
- 60 FPS on modest hardware
- <3 second load times
- <500MB RAM usage
- Smooth perspective transitions

### Experiential
- Geographic coherence
- Narrative flow
- Emotional progression
- Horror effectiveness

## Key Innovations

### 1. Inverted Power Curve
Instead of getting stronger, every "advancement" makes you weaker, creating genuine horror through mechanical degradation.

### 2. Coherent Overworld
Not random regions but a logical continent with natural biome transitions and geographic sense.

### 3. Scene-Based AI Generation
Location-specific prompts create contextual content that fits the world's emotional journey.

### 4. Companion Trauma System
Deep psychological mechanics where relationships matter more than stats.

### 5. Perspective Horror
Transition from comfortable hex exploration to claustrophobic first-person terror.

## File Structure
```
dragons-labyrinth/
├── docs/
│   ├── Architecture.md    # This file
│   ├── WorldDesign.md     # Complete world structure
│   └── Themes.md          # Art direction
├── crates/
│   ├── builder/           # Content generation
│   └── game/             # Runtime engine
└── apps/
    └── game/             # Main executable
```

## Next Steps
1. Implement hex coordinate system based on WorldDesign.md
2. Create biome transition logic
3. Build scene generation templates
4. Integrate companion psychology with geography
5. Test chunk loading performance
