# Dragon's Labyrinth Asset Generation Prompt Library

This directory contains the complete TOML specification library for generating all 180 levels of Dragon's Labyrinth assets using DALL-E 3 via the LangGraph workflow system.

## Directory Structure

```
crates/game-engine/prompts/
├── biomes/                 # Terrain base layers
│   ├── 01-20-clean.toml         # Act 1 Early: Peace → Unease
│   ├── 21-40-blight.toml        # Act 1 Mid: Dragon blight corruption  
│   ├── 41-60-hellscape.toml     # Act 1 Late: Hellscape approach
│   ├── 61-120-social.toml       # Act 2: Social corruption (reuse tiles with war camps)
│   └── 121-180-void.toml        # Act 3: Void/eldritch corruption
├── characters/             # Chess-piece character tokens
│   ├── 01-20-heroes.toml        # Clean heroes and companions
│   ├── 61-120-corrupted.toml    # Socially corrupted characters
│   └── 121-180-nightmare.toml   # Nightmare/void-touched characters
├── monsters/              # Enemy creature tokens
│   ├── 01-20-basic.toml         # Bandits, wolves, goblins
│   ├── 21-60-dragon-spawn.toml  # Dragon-corrupted creatures
│   ├── 61-120-human.toml        # Human monsters, political enemies
│   └── 121-180-eldritch.toml    # Cosmic horror creatures
├── features/              # Interactive overlay elements
│   ├── 01-20-settlements.toml   # Villages, taverns, shops
│   ├── 21-60-damaged.toml       # Ruined/corrupted structures
│   ├── 61-120-war.toml          # War camps, execution sites
│   └── 121-180-impossible.toml  # Eldritch architecture
├── paths/                 # Universal path/bridge overlays
│   ├── universal-paths.toml     # Roads, trails (all corruption levels)
│   └── universal-bridges.toml   # Bridges (all terrain types)
├── ui/                    # Interface elements by dread level
│   ├── dread-0-1.toml          # Clean UI elements
│   ├── dread-2-3.toml          # Corrupted UI elements
│   └── dread-4.toml            # Nightmare UI elements
└── README.md              # This file
```

## Architecture Overview

### Layer Cake System
All assets follow the **Layer Cake Priority System** from Dragon's Labyrinth:

1. **Base Layer (Biomes)**: Opaque background terrain
2. **Path Layer**: Semi-transparent road/bridge overlays  
3. **Feature Layer**: Buildings, dungeons, interactive elements
4. **Character Layer**: Chess-piece tokens (players, NPCs, monsters)
5. **Effect Layer**: Weather, magic, corruption overlays

### Horror Progression (Dread Levels 0-4)
- **Dread 0**: Clean, welcoming (Levels 1-20)
- **Dread 1**: Subtle corruption (Levels 21-40) 
- **Dread 2**: Dragon blight visible (Levels 41-60)
- **Dread 3**: Hellscape/social horror (Levels 61-120)
- **Dread 4**: Void corruption/cosmic horror (Levels 121-180)

### Consistency Standards
All prompts enforce these consistency constraints discovered through ChatGPT refinement:

#### Visual Standards
- **1024×1024 HD resolution** for all assets
- **Flat aerial top-down view** (no perspective, no horizon, no tilt)
- **Medieval dark fantasy art style** progressing to cosmic horror
- **No circular framing** (edge-to-edge coverage)
- **No borders, frames, symbols, or portals**

#### Technical Standards
- **Base biomes**: Seamless tileable textures, opaque backgrounds
- **Overlays**: Transparent PNG with soft feathered edges
- **Characters**: Chess-piece style silhouettes, readable on any background
- **Consistency constraints**: Negative prompts prevent unwanted elements

## Usage Examples

### List Available Specifications
```bash
hatch run dl_cli list-asset-specs
```

### Generate Assets from TOML
```bash
# Generate clean biomes for levels 1-20
hatch run dl_cli generate-assets crates/game-engine/prompts/biomes/01-20-clean.toml

# Generate with custom settings
hatch run dl_cli generate-assets \
  crates/game-engine/prompts/characters/01-20-heroes.toml \
  --output assets/characters \
  --batch-size 3 \
  --autonomous
```

### Asset Generation Workflow
The system uses a sophisticated **LangGraph StateGraph** with:
- **Human review checkpoints** for quality control
- **Batch processing** with configurable sizes
- **Error recovery** and retry logic
- **Bevy integration** auto-generates Rust asset loading code
- **Generation metadata** tracking for all assets

## Level Progression Design

### Act 1: Journey TO the Dragon (Levels 1-60)
**Levels 1-20: Peace → Unease**
- Lush biomes, welcoming settlements, basic monsters
- Clean heroes and friendly NPCs
- Stone roads, wooden bridges, cozy taverns

**Levels 21-40: Unease → Dread** 
- Dragon blight appears in bands and patches
- Mixed corruption (some scorched, some still green)
- Damaged features, early corrupted monsters

**Levels 41-60: Dread → Terror**
- Complete hellscape approach to dragon's lair
- Lava fields, bone deserts, molten terrain
- Nightmare features, dragon-spawned creatures

### Act 2: Journey FROM the Dragon (Levels 61-120)
**Terror → Madness: Social Corruption**
- **Reuse tiles** from Act 1 but with social corruption overlays
- War camps, execution sites, cruel political structures
- Human monsters, factional conflicts, social horror
- Same biomes but with authoritarian/war features

### Act 3: Sealing the Void (Levels 121-180)
**Madness → Void: Cosmic Horror**
- Nightmare biomes that break reality
- Impossible architecture, non-Euclidean geometry
- Eldritch creatures, void corruption, temporal anomalies
- UI elements that hurt to look at

## Content Creation Patterns

### Biome Generation Pattern
```toml
[batch]
name = "category_act_phase"
description = "Clear description of content and level range"
level_range = "X-Y"
category = "biome|character|monster|feature|path|bridge|ui"

[style_constraints]
camera_angle = "flat aerial top-down view"
resolution = "1024x1024"
# ... consistency rules

[assets.asset_name]
type = "specific_asset_type"
prompt = "Detailed consistency-locked prompt with all constraints"
layer_type = "base|overlay|token|effect"
dread_level = 0-4  # Horror progression level
priority = 1-10    # Asset importance
```

### Character Token Pattern
All character tokens use **chess-piece silhouette style**:
- Clear readable shape from top-down view
- Distinctive visual markers (armor, weapons, clothing)
- Works on any biome background
- Consistent scale and proportions

### Corruption Progression Pattern
Assets include variants for different corruption levels:
- **Clean**: Original uncorrupted version
- **Degraded**: Weathered, damaged, subtle corruption  
- **Cursed**: Dark twisted version with unnatural elements
- **Nightmare**: Completely corrupted horror version

## Quality Assurance

### Prompt Consistency
- All prompts include full consistency constraints
- Negative constraints prevent unwanted elements
- Style progression maintained across dread levels
- Layer system properly specified

### Testing Integration
- CLI commands validate TOML structure
- Workflow system handles generation errors gracefully
- Human review checkpoints ensure quality
- Bevy integration automatically tested

### Asset Coverage
- **180 levels** fully covered across 3 acts
- **5 dread levels** with appropriate horror progression
- **Layer system** supports all game mechanics
- **Universal elements** (paths, bridges) work across all levels

## Development Commands

```bash
# List all available TOML specifications
hatch run dl_cli list-asset-specs

# Generate assets with interactive review
hatch run dl_cli generate-assets path/to/spec.toml

# Generate assets autonomously (no human review)
hatch run dl_cli generate-assets path/to/spec.toml --autonomous

# Custom output directory and batch size
hatch run dl_cli generate-assets path/to/spec.toml \
  --output custom/path --batch-size 10
```

This prompt library represents the complete asset generation foundation for Dragon's Labyrinth, enabling the creation of a cohesive 180-level horror RPG experience with consistent visual quality and proper layer cake architecture integration.
