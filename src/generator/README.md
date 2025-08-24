# Dragon's Labyrinth - AI Asset Generator

Revolutionary AI-powered asset generation system for Dragon's Labyrinth.
**Build-time Python AI generation ↔ Runtime Rust game engine**

## Architecture Overview

```
AI Agents → BPY Scripts → Generic Processor → Bevy GLB Assets
    ↓           ↓              ↓               ↓
  Python     Blender       Python          Rust Game
```

### Core Principles
- **Clean Separation**: Python AI generation completely separate from Rust runtime
- **Generic BPY Processor**: ONE processor for ALL AI workflows
- **80/20 Strategy**: 80% CC0 library reuse + 20% targeted AI generation
- **Horror-First**: Every system responds to dread levels 0-4

## Directory Structure

```
src/generator/
├── bpy_processor.py          # Generic BPY → GLB processor (THE core)
├── constants.py              # System constants and configuration
├── organize_cc0_library.py   # CC0 asset organization utility
├── __init__.py              # Package initialization
├── __main__.py              # Main entry point
└── ai/                      # AI Agent Framework
    ├── maps_agent.py        # Hex world generation
    ├── levels_agent.py      # Encounter placement
    ├── ui_agent.py          # Horror UI degradation
    ├── dialogue_agent.py    # YarnSpinner dialogue
    ├── audio_agent.py       # Spatial audio systems
    ├── tiles_agent.py       # Tile asset generation
    ├── requirements.txt     # AI dependencies
    ├── database/
    │   └── schema.sql       # Asset tracking schema
    └── tools/
        └── dragon_search_tool.py  # CC0 library search
```

## AI Agent System

### 5 Specialized Agents
1. **MapsAgent**: Hexx integration, hex world generation, biome corruption
2. **LevelsAgent**: Yoleck integration, encounter placement, interactive objects
3. **UIAgent**: Cobweb integration, horror-responsive interface degradation
4. **DialogueAgent**: YarnSpinner integration, companion arcs, moral choices
5. **AudioAgent**: Freesound integration, proximity horror, spatial audio
6. **TilesAgent**: Hex tile generation with horror progression

### Agent Workflow
```python
from src.generator.ai.tiles_agent import TilesAgent

# Initialize agent
agent = TilesAgent()

# Generate AI descriptions for tiles
descriptions = agent.generate_tile_descriptions(
    tile_types=["grassland", "forest", "mountain"],
    dread_level=0  # Peace stage
)

# Convert to BPY scripts and process
results = agent.generate_bpy_scripts_from_descriptions(
    descriptions, 
    dread_level=0
)

# Results: GLB files ready for Bevy
```

## Generic BPY Processor

The **revolutionary** generic processor that ALL AI workflows use:

```python
from src.generator.bpy_processor import BPYProcessor

processor = BPYProcessor()

# Process single BPY script
result = processor.execute_bpy_script(
    script_content="# BPY script here...",
    export_path="assets/generated/my_asset.glb"
)

# Process batch of scripts
results = processor.process_batch([
    {"script": "# BPY script 1...", "filename": "asset1.glb"},
    {"script": "# BPY script 2...", "filename": "asset2.glb"}
], "output/directory")
```

### Key Features
- Direct BPY execution within Blender context
- Automatic GLB export for Bevy
- Batch processing with manifests
- Clean error handling and logging
- Performance metrics tracking

## Horror Progression System

Every AI agent is **dread-aware** and generates content that responds to the 5-stage horror progression:

```python
DREAD_LEVELS = {
    0: "Peace - Beautiful morning, birds singing, warm sunlight",
    1: "Unease - Something feels off, shadows too long, colors muted", 
    2: "Dread - Visible decay, darkness spreading, hope fading",
    3: "Terror - Active malevolence, reality distorting, companions breaking",
    4: "Horror - Complete nightmare, first-person stalking, reality shattered"
}
```

All AI agents generate content that:
- Reflects the current dread level
- Shows corruption progression
- Enhances environmental storytelling
- Supports the horror narrative

## Database Integration

Asset tracking with SQLite:

```sql
-- Asset generations tracked
CREATE TABLE tile_ai_generations (
    id INTEGER PRIMARY KEY,
    tile_types TEXT,
    dread_level INTEGER, 
    descriptions TEXT,
    created_at TIMESTAMP
);

-- BPY batch tracking  
CREATE TABLE bpy_batch_generations (
    batch_id TEXT PRIMARY KEY,
    scripts_count INTEGER,
    created_at TIMESTAMP
);
```

## Usage Examples

### Generate Complete Tileset
```python
from src.generator.ai.tiles_agent import TilesAgent

agent = TilesAgent()
results = agent.generate_complete_tileset()  # All dread levels 0-4
```

### Custom Tile Generation
```python
# Generate specific tiles for dread level 2
descriptions = agent.generate_tile_descriptions(
    tile_types=["corrupted_swamp", "dying_forest"],
    dread_level=2
)

batch_results = agent.generate_bpy_scripts_from_descriptions(
    descriptions, 
    dread_level=2,
    batch_name="corruption_tiles"
)
```

### Process in Blender
```bash
# Execute generated BPY scripts in Blender
blender --python assets/generated/bpy_scripts/batch_name/execute.py
```

## CC0 Library Integration

The system intelligently searches CC0 assets before generating new content:

```python
from src.generator.organize_cc0_library import organize_assets

# Organize CC0 library for intelligent reuse
stats = organize_assets(
    source_dir=Path("/path/to/cc0/assets"),
    target_dir=PROJECT_ROOT,
    filter_relevant=True  # Filter for horror medieval RPG
)
```

### Smart Asset Selection
- **80% Reuse**: Professional CC0 assets used intelligently
- **20% Generation**: Horror-specific variants and game-unique content
- **Semantic Search**: AI searches existing assets before generating
- **Performance First**: Mobile-optimized asset selection

## Integration with Bevy

Generated assets integrate seamlessly with the Rust Bevy game engine:

```rust
// Auto-generated Rust loaders
use bevy::prelude::*;

fn load_generated_tiles(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Load AI-generated tiles
    commands.spawn(SceneBundle {
        scene: asset_server.load("tiles/grassland_dread0.glb"),
        ..default()
    });
}
```

## Performance Targets

All generated assets meet strict performance requirements:
- **< 100,000 vertices** per asset
- **≤ 2048px textures** maximum
- **60 FPS** target with full asset load
- **30 FPS** minimum on mobile devices

## Development Workflow

1. **Design Phase**: AI agents generate descriptions and specifications
2. **BPY Generation**: Convert specifications to Blender Python scripts
3. **Asset Processing**: Generic processor executes BPY → exports GLB
4. **Rust Integration**: Bevy loads GLB assets at runtime
5. **Horror Progression**: All assets respond to dread level changes

## Revolutionary Benefits

### For Development
- **Massive Cost Savings**: Intelligent asset reuse vs pure generation
- **Clean Architecture**: Complete separation of concerns
- **Scalable Pipeline**: Add new agents without changing core processor
- **Deterministic Results**: Same inputs produce identical outputs

### For Horror Experience
- **Narrative-Driven**: All generation serves the horror progression
- **Emotional Consistency**: AI understands and enhances dread levels
- **Environmental Storytelling**: Assets tell the story through visual progression
- **Player Journey**: Every generated element supports the emotional arc

## Requirements

- **Python 3.11+** with uv/hatch
- **Blender 4.5+** for BPY script execution  
- **OpenAI API key** for AI generation
- **Rust 1.88+** with Bevy 0.16.1 for game runtime

## Getting Started

```bash
# 1. Set up environment
export OPENAI_API_KEY="your-key-here"

# 2. Install dependencies (handled by uv/hatch)
# Dependencies auto-installed by project setup

# 3. Generate tile assets
python -m src.generator.ai.tiles_agent

# 4. Process in Blender
blender --python assets/generated/bpy_scripts/latest_batch/execute.py

# 5. Assets ready for Bevy!
ls assets/generated/tiles/*.glb
```

## Architecture Revolution

This system represents a **fundamental paradigm shift** in game development:

- **Traditional**: Embedded generators, runtime AI, complex architectures
- **Revolutionary**: Build-time AI, clean separation, intelligent reuse

The result is a **maintainable, scalable, cost-effective** system that produces better assets while serving the horror narrative perfectly.

**The journey IS the orchestrator** - and now the AI understands that journey. 🐉
