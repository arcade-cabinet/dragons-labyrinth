# Entities & Seeds Redesign Plan for Rust/Bevy ECS Integration

## PROBLEM ANALYSIS

### Current Broken State
1. **entities/models.py** - Defines SQLite tables (HexTiles, Entities, Companions, Encounters, Assets) for Godot integration that doesn't exist
2. **entities/processors/** - Complex ML processors trying to populate database tables instead of generating Rust data
3. **seeds/** - Incomplete system with extractors that generate database entries instead of world crate data
4. **Integration mismatch** - All systems assume Godot + SQLite when architecture is Rust + Bevy + ECS

### Required Architecture
```
Python Generators → World Crate Data → build.rs → ECS Components → Game Crate
```

## REDESIGN PLAN

### Phase 1: Seeds System Redesign

**Goal**: Make `src/generator/seeds` a proper standalone system for generating seeds in the world crate

#### New Seeds Architecture
```
src/generator/seeds/
├── __init__.py              # Simple run() function as entry point
├── models.py               # Rust-compatible data models (not SQLite)
├── extractors.py           # Extract from literature sources 
├── processors.py           # Process seeds into world data
├── world_generator.py      # Generate Rust world crate files
└── templates/              # Jinja2 templates for Rust code generation
    ├── seeds.rs.j2         # Template for world/src/seeds.rs
    ├── biomes.rs.j2        # Template for world/src/biomes.rs  
    ├── regions.rs.j2       # Template for world/src/regions.rs
    └── constants.rs.j2     # Template for world/src/constants.rs
```

#### Seeds Output (Generated Rust Files)
```
crates/world/src/
├── lib.rs                  # World crate entry point
├── seeds.rs                # Generated narrative/motif/semantic seeds
├── biomes.rs               # Generated biome definitions
├── regions.rs              # Generated region data
├── constants.rs            # Generated game constants
└── builder.rs              # Builder for ECS integration
```

#### Seeds Models (Rust-Compatible)
```python
from pydantic import BaseModel
from typing import Any

class NarrativeSeed(BaseModel):
    """Narrative pattern for world generation"""
    structure_name: str
    story_beats: list[str]
    themes: list[str]
    horror_stage: int
    corruption_arc: list[str]

class BiomeDefinition(BaseModel):
    """Biome definition for world generation"""
    name: str
    base_terrain: str
    corruption_stages: list[str]
    encounter_types: list[str]
    resource_availability: dict[str, float]

class RegionData(BaseModel):
    """Region data for world generation"""
    name: str
    biome_distribution: dict[str, float]
    dread_level: int
    settlement_density: float
    faction_presence: list[str]
```

### Phase 2: Entities System Redesign  

**Goal**: Transformer → specialized processors with HBF knowledge → world crate data generation

#### New Entities Architecture
```
src/generator/entities/
├── __init__.py              # Simple run() function
├── transformer.py           # HBF entity clustering (keep current logic)
├── models.py               # Rust-compatible output models  
├── world_generator.py      # Generate Rust world crate files
├── processors/             # Specialized processors for world data
│   ├── __init__.py
│   ├── base.py            # Base processor for world data generation
│   ├── regions.py         # Process regions → biome/settlement data
│   ├── settlements.py     # Process settlements → scale/services data  
│   ├── factions.py        # Process factions → political/territorial data
│   └── dungeons.py        # Process dungeons → encounter/treasure data
└── templates/             # Jinja2 templates for Rust code
    ├── entities.rs.j2     # Template for world/src/entities.rs
    ├── settlements.rs.j2  # Template for world/src/settlements.rs
    ├── factions.rs.j2     # Template for world/src/factions.rs
    └── dungeons.rs.j2     # Template for world/src/dungeons.rs
```

#### Entities Output (Generated Rust Files)
```
crates/world/src/
├── entities.rs             # Generated entity definitions
├── settlements.rs          # Generated settlement data
├── factions.rs             # Generated faction data
├── dungeons.rs             # Generated dungeon data
└── spatial.rs              # Generated hex coordinates and spatial data
```

#### Entities Models (Rust-Compatible)
```python
from pydantic import BaseModel
from typing import Any

class SettlementData(BaseModel):
    """Settlement data for world generation"""
    name: str
    scale: str  # village, town, city
    services: list[str]  # blacksmith, tavern, shrine
    economic_activity: list[str]
    corruption_resistance: float
    hex_coordinates: tuple[int, int]

class FactionData(BaseModel):
    """Faction data for world generation"""  
    name: str
    political_alignment: str
    territorial_reach: str  # local, regional, widespread
    hostility_level: float
    operating_locations: list[str]
    member_composition: dict[str, int]

class DungeonData(BaseModel):
    """Dungeon data for world generation"""
    name: str
    entrance_type: str
    horror_themes: list[str]
    encounter_density: int
    treasure_level: int
    hex_coordinates: tuple[int, int]
```

### Phase 3: World Crate Architecture

**Goal**: Rust crate that contains all world data and provides ECS generation

#### World Crate Structure
```
crates/world/
├── Cargo.toml              # World crate dependencies
├── build.rs                # Build script for codegen
├── src/
│   ├── lib.rs              # World crate API
│   ├── builder.rs          # ECS builder for game integration
│   ├── seeds.rs            # Generated seeds (from seeds system)
│   ├── biomes.rs           # Generated biome definitions
│   ├── regions.rs          # Generated region data
│   ├── entities.rs         # Generated entity data (from entities system)
│   ├── settlements.rs      # Generated settlement data
│   ├── factions.rs         # Generated faction data
│   ├── dungeons.rs         # Generated dungeon data
│   ├── spatial.rs          # Generated hex coordinates
│   ├── encounters.rs       # Generated encounter rules
│   └── psychology.rs       # Generated companion psychology rules
└── templates/              # Templates for ECS generation
    ├── components.rs.j2    # Bevy components template
    ├── resources.rs.j2     # Bevy resources template  
    └── systems.rs.j2       # Bevy systems template
```

#### World Crate API
```rust
// crates/world/src/lib.rs
pub mod seeds;
pub mod biomes; 
pub mod regions;
pub mod entities;
pub mod settlements;
pub mod factions;
pub mod dungeons;
pub mod spatial;
pub mod encounters;
pub mod psychology;
pub mod builder;

pub use builder::WorldBuilder;

// Main API for game integration
pub fn create_world_builder() -> WorldBuilder {
    WorldBuilder::new()
        .with_seeds(&seeds::NARRATIVE_SEEDS)
        .with_biomes(&biomes::BIOME_DEFINITIONS)
        .with_regions(&regions::REGION_DATA)
        .with_entities(&entities::ENTITY_DATA)
}
```

### Phase 4: Game Integration

**Goal**: apps/game uses world crate as build dependency

#### Game Integration Architecture
```
apps/game/
├── build.rs                # Uses world crate to generate ECS
├── Cargo.toml             # Depends on world crate
└── src/
    ├── main.rs            # Game entry point
    ├── world/             # Generated ECS modules (from build.rs)
    │   ├── mod.rs         # Generated module
    │   ├── components.rs  # Generated Bevy components  
    │   ├── resources.rs   # Generated Bevy resources
    │   └── systems.rs     # Generated Bevy systems
    └── game.rs            # Game logic using generated ECS
```

#### Game build.rs
```rust
// apps/game/build.rs
use world::WorldBuilder;

fn main() {
    let world_builder = world::create_world_builder();
    
    // Generate ECS components, resources, systems
    world_builder
        .generate_components("src/world/components.rs")
        .generate_resources("src/world/resources.rs") 
        .generate_systems("src/world/systems.rs");
        
    println!("cargo:rerun-if-changed=../../crates/world/src");
}
```

## IMPLEMENTATION STEPS

### Step 1: Fix Seeds System
1. **Redesign models.py** - Remove SQLite models, add Rust-compatible Pydantic models
2. **Update extractors.py** - Extract to Pydantic models instead of database
3. **Create world_generator.py** - Generate Rust files from extracted seeds
4. **Create Rust templates** - Jinja2 templates for world crate generation
5. **Update __init__.py** - Simple run() that generates world crate files

### Step 2: Fix Entities System  
1. **Redesign models.py** - Remove SQLite models, add Rust-compatible output models
2. **Update processors/** - Generate world data instead of database entries
3. **Create world_generator.py** - Generate Rust files from processed entities
4. **Create Rust templates** - Jinja2 templates for entity data generation
5. **Update transformer.py** - Route to new processors

### Step 3: Create World Crate
1. **Setup crate structure** - Cargo.toml, lib.rs, builder.rs
2. **Test generation** - Run seeds + entities to generate world crate files
3. **Create ECS builder** - WorldBuilder that generates Bevy components
4. **Add templates** - Templates for ECS generation

### Step 4: Update Game Integration
1. **Update apps/game/build.rs** - Use world crate for ECS generation  
2. **Update Cargo.toml** - Add world crate dependency
3. **Test build process** - Ensure generated ECS compiles
4. **Update game code** - Use generated components/resources/systems

## SUCCESS CRITERIA

1. **Seeds system generates world crate Rust files** - No database dependencies
2. **Entities system processes HBF into world data** - Specialized processors with HBF knowledge  
3. **World crate contains all world building data** - Rust-native data structures
4. **Game build.rs generates ECS from world crate** - Bevy components, resources, systems
5. **Full pipeline works end-to-end** - Python generation → Rust world crate → ECS → Game

## ARCHITECTURAL BENEFITS

1. **Clean separation** - Python for data generation, Rust for game logic
2. **Deterministic builds** - Generated files are committed and reviewable
3. **Type safety** - Rust ensures data integrity throughout pipeline
4. **Performance** - No runtime database queries, all data compiled into game
5. **Maintainability** - Clear boundaries between generation and runtime systems

## NEXT ACTIONS

1. **Start with seeds redesign** - It's simpler and establishes the pattern
2. **Create world crate skeleton** - Basic structure and builder pattern
3. **Redesign entities system** - Focus on HBF processing to world data
4. **Test integration** - Ensure build.rs can generate ECS properly
5. **Document handoff** - Complete documentation for AI continuation

This redesign transforms the current broken SQLite/Godot-focused architecture into a proper Rust/Bevy ECS generation pipeline that leverages the rich HBF data for actual worldbuilding.
