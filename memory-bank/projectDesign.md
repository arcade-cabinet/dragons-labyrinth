# Dragon's Labyrinth Project Design Bible

## Table of Contents

1. [Core Philosophy](#core-philosophy)
2. [Current Architecture: Godot-First Generation](#current-architecture-godot-first-generation)
3. [Horror-First Design](#horror-first-design)
4. [Implementation: Simple Direct Generation](#implementation-simple-direct-generation)
5. [Data Integration](#data-integration)
6. [Generation Standards](#generation-standards)
7. [Future Expansion](#future-expansion)

---

## Core Philosophy

Dragon's Labyrinth is a **horror RPG disguised as an adventure**. The game begins with opening your front door on a beautiful morning and ends in first-person darkness, hunted by an ancient intelligence.

**Fundamental Principle**: "We're not building 'an RPG with horror elements' - we're building a horror experience that happens to have RPG mechanics."

### MAJOR ARCHITECTURAL PARADIGM SHIFT (2025-08-28)

**THE FINAL SIMPLIFICATION: From Complex Pipelines to Direct Generation**

After comprehensive analysis, we've achieved a revolutionary simplification:

- **NOT** Complex database architectures
- **NOT** AI-driven pipeline systems  
- **NOT** Rust/Bevy ECS layers

**JUST** Simple Python generating Godot resources!

```python
# The ENTIRE architecture in 4 files
src/dragons_labyrinth/
├── data_sources.py    # Load HBF/literature/psychology data
├── generation.py      # Generate Godot formats (.tres/.tscn/.gd)
├── tracking.py        # Minimal idempotency tracking
└── generate_world.py  # Main orchestrator
```

### Godot-First Decision

**CRITICAL PIVOT**: Generate Godot-native formats directly, no runtime databases or complex abstractions.

**Why Direct Generation Approach:**
1. **Simplicity**: 4 files instead of 20+ modules
2. **Performance**: Sub-second generation, no runtime overhead  
3. **Transparency**: Can inspect generated .tres/.tscn files directly
4. **Direct Integration**: Godot loads resources natively
5. **One-Time Generation**: HBF → Python → Godot resources → permanent game world

Every decision follows these core principles:

1. **Minimal Dependencies**: Just SQLModel for tracking, everything else built-in
2. **Idempotent Generation**: Running the same generation produces compatible results
3. **Direct Output**: Generate exactly what Godot needs
4. **Performance by Design**: Pre-generated resources, no runtime queries
5. **Horror-First Narrative**: Every system reinforces the growing dread
6. **Mathematical Horror**: Distance = progression, corruption spreads mathematically

---

## Current Architecture: Godot-First Generation

### The Journey from Complex to Simple

**Before (Over-engineered):**
- 20+ Python modules with complex inheritance
- Database mixins and abstract patterns
- AI pipelines and LangChain orchestration
- Complex dependency chains

**After (Simple & Direct):**
```python
# Load data
hbf_data = load_hbf_data()  # 65 hex tiles, 245 entities

# Generate Godot resources
for hex_tile in hbf_data["hex_tiles"]:
    generate_hex_tile_resource(hex_tile)  # Creates .tres file

# Track what's been generated
tracker.mark_generated(hex_id, "hex_tile", hex_data, file_path)
```

### Working Implementation

**Successfully Generated:**
- 10 hex tiles as .tres resources
- HexTileData.gd script for Godot
- Generation manifest for content loading
- SQLite tracking for idempotency

**Ready for Generation:**
- 55 more hex tiles from HBF
- 245 entities for creatures/NPCs
- Psychology patterns for companions
- Item and treasure systems

---

## Horror-First Design

### The Journey IS the Game

Like Frodo's walk to Mordor, the growing dread is the experience. The game is about feeling the weight of inevitability, the chill in the air that grows colder with each step.

### Mathematical Horror Progression

The narrative arc is now implemented mathematically:

```python
def calculate_dread_level(coordinate: str) -> int:
    distance = calculate_hex_distance(coordinate)
    return min(4, distance // 20)  # 0-4 dread stages

# Dread stages by distance
# 0-20 hexes: Peace (stage 0)
# 20-40 hexes: Unease (stage 1)  
# 40-60 hexes: Dread (stage 2)
# 60-120 hexes: Terror (stage 3)
# 120+ hexes: Horror (stage 4)
```

### Vision-Critical Elements (Preserved)

These elements define Dragon's Labyrinth and are built into generation:

1. **The Opening**: First-person view of leaving home (future implementation)
2. **Distance Horror**: Mathematical dread increase with distance
3. **Companion Psychology**: Trauma patterns integrated in NPC generation
4. **Horror Names**: Creatures get corrupted names based on CR
5. **World Darkening**: Biome corruption stages per dread level
6. **Multiple Endings**: Based on understanding, not power (future)

### Narrative Arc Implementation

Each dread stage transforms generation:

```python
# Peace (0): Normal generation
if dread_level == 0:
    creature_name = base_name
    
# Unease (1): Slight corruption
elif dread_level == 1:
    creature_name = f"Tainted {base_name}"
    
# Dread (2): Obvious corruption  
elif dread_level == 2:
    creature_name = f"Corrupted {base_name}"
    
# Terror (3): Nightmare variants
elif dread_level == 3:
    creature_name = f"Nightmare {base_name}"
    
# Horror (4): Unspeakable
else:
    creature_name = f"Unspeakable {base_name}"
```

---

## Implementation: Simple Direct Generation

### Data Sources (data_sources.py)

Load all analyzed data without complex abstractions:

```python
def load_hbf_data() -> dict[str, Any]:
    # Direct CSV loading with field size fix
    csv.field_size_limit(1000000)
    
    # Load hex tiles
    with open("hbf_analysis/hex_tiles_full.csv") as f:
        hex_tiles = list(csv.DictReader(f))
    
    # Load entities
    with open("hbf_analysis/entities.csv") as f:
        entities = list(csv.DictReader(f))
    
    return {"hex_tiles": hex_tiles, "entities": entities}
```

### Generation (generation.py)

Generate Godot-native formats directly:

```python
def generate_hex_tile_resource(hex_data: dict) -> str:
    """Generate a .tres resource file for Godot"""
    tres_content = f'''[gd_resource type="Resource" script_class="HexTileData"]
[resource]
coordinate = "{hex_data['coordinate']}"
biome = "{hex_data['biome']}"
dread_level = {calculate_dread_level(hex_data['coordinate'])}
features = {format_features(hex_data['features'])}
'''
    
    output_path = GODOT_RESOURCES_DIR / f"hex_{hex_data['coordinate']}.tres"
    output_path.write_text(tres_content)
    return str(output_path)
```

### Tracking (tracking.py)

Minimal idempotency to avoid regenerating:

```python
class GenerationRecord(SQLModel, table=True):
    id: str = Field(primary_key=True)
    generation_type: str
    timestamp: datetime
    checksum: str  # SHA256 for change detection
    file_path: str
    extra_data: str = "{}"  # Not "metadata" (reserved word)

# Simple check before generation
if not tracker.has_been_generated(item_id, content):
    generate_resource(content)
    tracker.mark_generated(item_id, "type", content, file_path)
```

---

## Data Integration

### HBF Analysis (Complete)

Successfully integrated from CSV/JSON:
- **65 hex tiles** with coordinates, biomes, features
- **245 entities** ready for categorization
- **Pattern analysis** with horror themes
- **World structure** from Lands of Vo'il

### Psychology Patterns (Simplified)

Mathematical formulas replace complex systems:
- **5-stage progression** based on distance
- **Companion trauma** levels 0-4
- **Creature corruption** by challenge rating
- **Biome darkening** per dread stage

### World Context (Ready)

Four biomes with corruption mechanics:
1. **Fearless Wilds** (jungle) - vines blacken → trees weep blood
2. **Vicious Crags** (mountains) - rocks whisper → gravity fluctuates
3. **Ragthorn Woods** (forest) - leaves watch → woods are alive
4. **Heartseeker Forest** (deep forest) - hearts exposed → forest knows all

---

## Generation Standards

### File Structure

Generated Godot files follow strict organization:

```
godot/
├── resources/
│   ├── hex_tiles/      # .tres hex tile data
│   ├── npcs/           # .tres NPC resources
│   └── items/          # .tres item data
├── scenes/
│   ├── creatures/      # .tscn creature scenes
│   ├── world/          # .tscn world composition
│   └── ui/             # .tscn UI elements
├── scripts/
│   ├── HexTileData.gd  # Resource classes
│   ├── Creature.gd     # Entity scripts
│   └── systems/        # Game systems
└── metadata/
    ├── generation_tracking.db  # SQLite tracking
    └── generation_manifest.json # Content registry
```

### Idempotency Requirements

All generation is idempotent through:
1. **Content hashing** - SHA256 of input data
2. **Change detection** - Only regenerate if content changed
3. **Stable IDs** - Deterministic ID generation
4. **Version tracking** - Track generation timestamps

### Performance Targets

Current achievement and targets:
- **Generation speed**: < 1 second for 10 tiles ✓
- **Memory usage**: Minimal (no database overhead) ✓
- **File size**: < 1KB per hex tile resource ✓
- **Godot loading**: Native resource loading ✓

---

## Future Expansion

### Immediate Next Steps

1. **Complete HBF Generation**
   - Generate remaining 55 hex tiles
   - Process 245 entities into creatures/NPCs
   - Add treasure and item generation

2. **Procedural Extension**
   - Generate hexes beyond HBF's 65 tiles
   - Algorithmic biome placement
   - Distance-based feature generation

3. **Psychology Integration**
   - Companion generation with trauma levels
   - NPC behavior by dread stage
   - Dynamic dialogue based on horror level

### Medium-Term Goals

1. **Godot Scene Generation**
   - Complete world scene with hex placement
   - Creature spawning by biome
   - Interactive elements (shops, portals)

2. **Hex System Integration**
   - Complete hexagon_tilemaplayer addon integration
   - Tilemap rendering optimization
   - Fog of war implementation

3. **Corruption Mechanics**
   - Visual corruption overlays
   - Spreading darkness system
   - Reality distortion effects

### Long-Term Vision

1. **First-Person Transitions**
   - Opening door sequence
   - Dragon's labyrinth entry
   - Horror perspective shifts

2. **Audio Integration**
   - Freesound integration for ambience
   - Dynamic audio by dread level
   - Proximity-based dragon sounds

3. **Complete Horror Journey**
   - Full 180-level progression
   - Multiple ending branches
   - Companion story arcs

---

## Legacy Systems (Archived)

The following complex systems have been replaced but are preserved for reference:

<details>
<summary>Click to expand legacy architecture</summary>

### Previous Complex Systems
- **Rust/Bevy ECS** - Replaced with Godot-native
- **Database Architecture** - Replaced with direct generation
- **AI Pipelines** - Replaced with simple functions
- **Metaprompt Systems** - Replaced with direct templates
- **Complex Inheritance** - Replaced with flat structure

These systems were over-engineered for the actual need. The game needs Godot resources, not complex abstractions.

</details>

---

## Summary

Dragon's Labyrinth has successfully pivoted from infrastructure engineering to game development:

**Architecture Evolution:**
- From 20+ modules → 4 simple files
- From complex databases → direct generation
- From AI pipelines → mathematical formulas
- From abstractions → concrete implementations

**Core Achievement:**
The horror-first vision remains intact while the implementation has been dramatically simplified. We can now focus on making the game rather than building infrastructure.

**Guiding Principle:**
"The game is what matters. Everything else is just tooling."

This design ensures a maintainable, performant, and focused development path that prioritizes the horror experience over technical complexity.
