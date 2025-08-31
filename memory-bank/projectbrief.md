# Dragon's Labyrinth Project Brief

## Core Mission

Dragon's Labyrinth is a horror RPG that inverts the traditional power fantasy. Instead of growing stronger, players grow more cursed as they journey toward an inevitable confrontation with an ancient dragon. The game uses familiar RPG mechanics to deliver genuine psychological horror through companion trauma, mathematical progression of dread, and the weight of moral choices.

## The Innovation

### Inverted Power Curve
Traditional RPGs: Weak → Strong → Victory
Dragon's Labyrinth: Hopeful → Cursed → Hunted

### Core Game Mechanics
- **Infinite hex map**: Procedurally generated world that extends forever
- **Mathematical corruption**: Dread increases through 5 progression bands (1-20, 21-40, 41-60, 61-120, 121-180)
- **Companion psychology**: NPCs accumulate trauma and psychological scars
- **Forge System**: Redemption mechanic for second chances

## Technical Stack

### Rust/Bevy Game Engine
- **Framework**: Bevy 0.16.1 ECS architecture
- **Rendering**: 2D hex-based world with Material2dPlugin for shaders
- **Architecture**: Component-based with plugins for world, movement, encounters, shops, dungeons
- **Assets**: Tilesets and icons stored in `apps/game/assets/`

### Python AI Generation Pipeline
- **Purpose**: Convert markdown content into game data via OpenAI
- **Models**: Configurable via env vars (default: gpt-5.1 for text, gpt-image-1 for images)
- **Workflow**:
  1. `canonize`: Convert Architecture.md → canon.json
  2. `plan`: Generate world plan from canon + themes
  3. `expand`: Create detailed region bibles  
  4. `image-plan`: Design visual assets
  5. `images`: Generate actual tileset images
  6. `narrative`: Expand NPC dialogue and questlines

### Content-Driven Design
- **Source**: `content/Architecture.md` and `content/Themes.md`
- **Build Output**: JSON files in `build/` directory
- **World Data**: `build/world/worldbook.json` loaded at runtime

## Game Architecture

### Five Progression Bands
1. **Band 1-20**: "Peace" - Pastoral decay, hidden dread, fog barriers
2. **Band 21-40**: "Unease" - World darkens, trust fractures, corrupted bridges
3. **Band 41-60**: "Dread" - Trauma defines play, companions may turn, cursed gates
4. **Band 61-120**: "Terror" - Warped ecosystems, resource collapse, sealed cathedrals
5. **Band 121-180**: "Horror" - Total collapse, dragon's fall unravels reality

### Biome Progression
- Early: wet meadows, ashen forests, flooded villages
- Middle: black swamps, fungal cathedrals, rust plains
- Late: famine fields, bone forests, dragon scars, abyssal chasms

### Art Direction
- Muted, textured, painterly aesthetic
- Seamless hex tiles for biomes
- Symbolic icons for POIs
- Color palettes grow colder with progression
- Fog and silhouettes dominate

## Current Implementation

### Working Features
- **Hex Movement**: Q/W/E/A/S/D navigation on hex grid
- **World Loading**: Reads worldbook.json at startup
- **Hot Reload**: Press R to reload world data
- **Shop System**: Press T to open/close shop UI
- **Dungeon Entry**: Enter key for dungeon, Esc to exit
- **Ambient Lighting**: Dynamic light/dark cycles

### Codebase Structure
```
dragons-labyrinth/
├── ai/                    # Python generation scripts
│   ├── ai.py             # Main orchestrator
│   ├── schemas.py        # Pydantic models
│   ├── prompts.py        # AI system prompts
│   ├── images.py         # Image generation
│   └── dialogue.py       # Narrative expansion
├── apps/game/            # Rust/Bevy game
│   └── src/main.rs       # Entry point
├── crates/world/         # World logic crate
│   └── src/
│       ├── plugin.rs     # Main world plugin
│       ├── resources.rs  # Game state
│       └── systems/      # Game systems
├── content/              # Source markdown
│   ├── Architecture.md   # Game design
│   └── Themes.md        # Art bible
└── build/               # Generated content
    ├── master/          # Canon + themes
    └── world/           # World data
```

## Development Workflow

1. **Edit Content**: Modify Architecture.md or Themes.md
2. **Run Generation**: `python -m ai canonize/plan/expand/image-plan/images/narrative`
3. **Build Game**: `cargo build` 
4. **Run Game**: `cargo run -p game`
5. **Test Changes**: Use R key to hot-reload world data

## Next Steps

### Priority 1: Complete AI Pipeline
- Implement missing schemas in ai/schemas.py
- Add image generation via DALL-E
- Expand narrative generation for all NPCs

### Priority 2: Game Features
- Implement combat system with inverted economy
- Add companion trauma tracking
- Create forge redemption mechanic
- Polish encounter system

### Priority 3: Content Creation
- Generate all biome tilesets
- Create POI icons for all types
- Write full questlines for each band
- Design signature encounters

## Success Criteria

- **Horror Experience**: Players feel increasing dread through progression
- **Mechanical Innovation**: Inverted power curve creates unique gameplay
- **Technical Excellence**: Smooth hex navigation, hot-reload workflow
- **Content Richness**: Procedural world feels hand-crafted
- **Artistic Cohesion**: Consistent painterly aesthetic throughout
