# Dragon's Labyrinth: Technical Architecture Guide

## Core System Philosophy

**Regions-First Generation**: Abandon fully open procedural worlds in favor of named, authored regions that provide structure for AI generation while maintaining narrative coherence.

**Layer-Cake Tiles**: Base biome + overlay features + paths system enables complex visual composition while maintaining performance and modularity.

**Horror-First Design**: Every technical system must respond to and reinforce the emotional progression from Peace (0) to Void (4).

## Project Structure

### Godot 4 Frontend
```
godot/
├── project.godot                 # Godot project configuration
├── assets/                      # Generated game assets
│   ├── biomes/                  # Base terrain tiles (128x128 PNG)
│   ├── features/                # Overlay elements (bridges, ruins)
│   ├── paths/                   # Road and trail overlays
│   ├── characters/              # NPC and monster sprites
│   ├── audio/                   # Environmental sounds and music
│   └── ui/                      # Interface elements
├── scenes/                      # Godot scene files
│   ├── World.tscn              # Main hex overworld scene
│   └── transitions/             # 3D first-person story scenes
├── scripts/                     # GDScript game logic
│   ├── World.gd                # Main world controller
│   ├── BiomeRules.gd           # Generated biome behavior rules
│   ├── TransitionLoader.gd     # 3D scene management
│   └── TileSetBuilder.gd       # Runtime tileset construction
├── metadata/                    # Asset and system manifests
│   ├── manifest.json           # Master asset inventory
│   ├── regions.json            # Regional definitions
│   └── biome_assets.json       # Biome-asset relationships
└── tilesets/                   # Compiled Godot TileSet resources
```

### Python Orchestrator Backend
```
src/dragons_labyrinth/
├── cli.py                      # Main command-line interface
├── orchestrator.py             # GPT-5 structured generation
├── constants.py                # Path management and project structure
├── code_generator.py           # GDScript generation from specs
└── workflow.py                 # Asset generation workflows

prompts/                        # AI generation specifications
├── regions/                    # Regional meta-prompts and generated specs
│   └── heartlands/             # Band 1-20 regions
│       ├── 01-verdant-crossing/
│       │   ├── meta-prompt.md  # Manual creative direction
│       │   ├── prompt.toml     # Generated asset specifications
│       │   └── spec.yaml       # Generated code specifications
│       └── ...
├── biomes/                     # Universal biome generation
├── characters/                 # NPC and companion generation
└── GLOBAL_STYLE_GUIDE.toml    # Universal visual consistency

source/guides/                  # Foundational documentation
├── WorldFoundation.guide.md   # World identity and design philosophy
└── TechnicalArchitecture.guide.md  # This file
```

## Generation Pipeline

### 1. Manual Creative Direction (Human Authored)
- **Region Meta-Prompts**: Detailed creative direction for each named region
- **Emotional Arcs**: What players should feel and learn in each area
- **Narrative Beats**: Core quests and story progression requirements
- **Thematic Consistency**: Ensuring each region serves the larger horror progression

### 2. Structured AI Generation (GPT-5 Orchestrated)
- **TOML Asset Prompts**: Generated from meta-prompts using structured outputs
- **YAML Code Specifications**: System behavior rules derived from creative direction
- **Biome Adjacency Rules**: Logical terrain transitions and geographic relationships
- **Encounter Tables**: Random events and environmental storytelling elements

### 3. Asset Creation (DALL-E + Processing)
- **Visual Assets**: Generated at 256x256, downsampled to 128x128 for performance
- **Transparency Enforcement**: Alpha channel validation and correction
- **Style Consistency**: Global style guide ensures visual coherence
- **Performance Optimization**: Mobile-friendly formats and compression

### 4. Code Generation (Template-Based)
- **GDScript Systems**: Generated from YAML specifications using Jinja2 templates
- **Biome Behavior**: Rules for movement, hazards, rest requirements
- **Encounter Logic**: Random event triggers and narrative progression
- **UI Integration**: Dynamic systems responding to dread progression

## Layer-Cake Tile System

### Base Biomes (60-70% coverage)
Primary terrain types that define the fundamental character of each hex:
- `lush_plains`, `healthy_forest`, `misty_woods`, `rocky_outcrops`
- `dry_plains`, `charred_forest`, `desert_edge`, `cracked_hills`
- `lava_fields`, `obsidian_desert`, `jagged_rocks`, `void_cracks`

### Overlay Features (15-25% coverage)
Secondary elements that add visual interest and gameplay function:
- `stone_circles`, `ancient_bridges`, `ruined_towers`, `shrine_clearings`
- `bandit_camps`, `refugee_tents`, `watchtowers`, `supply_caches`
- `void_rifts`, `nightmare_spires`, `reality_tears`, `eldritch_monuments`

### Path Overlays (5-15% coverage)
Transportation infrastructure connecting regions and providing movement bonuses:
- `dirt_paths`, `stone_roads`, `wooden_bridges`, `mountain_passes`
- `hidden_trails`, `smuggler_routes`, `military_highways`, `pilgrim_ways`
- `corruption_scars`, `void_highways`, `nightmare_passages`, `reality_wounds`

## Movement and Time Systems

### Hex Movement Mechanics
- **Walking Speed**: 8 hexes per day maximum (safe, standard encounters)
- **Running Speed**: 12 hexes per day (risky, increased encounter chance)
- **Terrain Modifiers**: Different biomes affect movement speed and safety
- **Weather Impact**: Rain, snow, void storms modify movement and visibility

### Day/Night Cycle Implementation
- **Day Duration**: 16 game turns (8 movement + 8 activity)
- **Night Duration**: 8 game turns (rest period, shelter required after Band 20)
- **Transition Effects**: Dawn/dusk provide atmospheric moments and narrative beats
- **Seasonal Variation**: Longer nights in later bands increase survival pressure

### Rest and Shelter Requirements
- **Civilized Rest**: Inns and settlements provide full recovery and safety
- **Wilderness Camping**: Requires fire, watch rotation, limited recovery
- **Hostile Environment**: Extreme cold, void exposure, supernatural threats
- **Companion Skills**: Different companions provide different survival bonuses

## Regional Integration System

### Meta-Prompt Processing
Each region's `meta-prompt.md` drives generation of:
1. **Asset TOML**: Specific visual requirements and style guidelines
2. **Behavior YAML**: Movement rules, encounter tables, environmental hazards
3. **Narrative Hooks**: 3D transition scenes and story progression requirements
4. **Adjacency Rules**: Which regions connect and how transitions work

### Generated Content Structure
```
prompts/regions/{region-name}/
├── meta-prompt.md       # Human-authored creative direction
├── prompt.toml          # Generated asset specifications
└── spec.yaml            # Generated system behavior rules

specs/regions/{region-name}/
└── spec.yaml            # Canonical behavior specification

godot/assets/{category}/{region-name}/
└── *.png                # Generated visual assets
```

### Orchestrator Commands
- `bootstrap_regions`: Generate all regional TOML/YAML from meta-prompts
- `generate_assets`: Create visual assets from TOML specifications
- `codegen_biomes_gd`: Generate GDScript from YAML specifications
- `bootstrap_project`: Initialize complete Godot project structure

## Transition System (3D Story Beats)

### Scene Loading Architecture
```gdscript
# TransitionLoader.gd
extends Node

func load_transition(scene_name: String, context: Dictionary):
    var scene_path = "res://scenes/transitions/" + scene_name + ".tscn"
    var scene = load(scene_path)
    get_tree().change_scene_to_packed(scene)
```

### Transition Types by Band
- **Bands 1-20**: Tutorial scenes, moral choice moments, character introductions
- **Bands 21-40**: Escalating threats, first supernatural encounters
- **Bands 41-60**: Environmental horror, dragon encounters, reality distortion
- **Bands 61-120**: Social collapse, moral complexity, companion trauma
- **Bands 121-180**: Cosmic horror, reality breakdown, final confrontations

### Technical Requirements
- **First-Person Perspective**: Immersive story moments
- **Environmental Storytelling**: Visual narrative without extensive dialogue
- **Multiple Outcomes**: Scenes adapt to player's moral choices and companion status
- **Performance Optimization**: Efficient loading and unloading for seamless transitions

## Data Flow and Dependencies

### Generation Order
1. **Foundation Guides** → Provide consistent creative direction
2. **Regional Meta-Prompts** → Define specific area requirements
3. **Structured Generation** → Create TOML/YAML specifications
4. **Asset Creation** → Generate visual and audio content
5. **Code Generation** → Create GDScript systems from specifications
6. **Integration Testing** → Verify complete pipeline functionality

### Quality Assurance Pipeline
- **Style Consistency**: Global style guide enforcement across all generations
- **Technical Validation**: PNG transparency, file size, format compliance
- **Narrative Coherence**: Regional connections and story progression verification
- **Performance Testing**: Frame rate and memory usage optimization

This architecture ensures that human creativity drives the vision while AI handles the detailed implementation, creating a scalable system for generating vast amounts of coherent, thematically appropriate game content.
