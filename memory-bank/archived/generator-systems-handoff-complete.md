# Generator Systems Architecture - Complete AI Handoff

## MISSION CRITICAL SUMMARY

The `src/generator/` Python systems are currently **architecturally misaligned** with the Rust/Bevy ECS game architecture. They were designed for Godot + SQLite integration that doesn't exist. **Immediate redesign required** to create proper Python → Rust world crate → ECS pipeline.

## CURRENT STATE ANALYSIS

### What's Working ✅
1. **HBF Data Source** - Rich SQLite database with 70,801 entities from "The Lands of Vo'il" world
2. **Entity Clustering Logic** - `transformer.py` successfully routes entities by category (regions, settlements, factions, dungeons)
3. **Processor Architecture** - Good organization for specialized entity processing
4. **Seeds System Foundation** - Basic structure exists in `src/generator/seeds/`

### What's Broken ❌
1. **Output Format Mismatch** - All systems generate SQLite/Godot data instead of Rust world crate data
2. **Architecture Disconnect** - No integration with Rust/Bevy ECS build process
3. **Seeds System Incomplete** - Missing world crate generation, templates, processors
4. **Models Wrong Format** - SQLite models instead of Rust-compatible Pydantic models

## ARCHITECTURAL REDESIGN REQUIRED

### Current (Broken) Flow
```
HBF Database → Python Processing → SQLite Database → ❌ (No Godot exists)
```

### Required (Fixed) Flow  
```
HBF Database → Python Processing → Rust World Crate → build.rs → ECS Components → Game
```

## KEY SYSTEMS ANALYSIS

### 1. Seeds System (`src/generator/seeds/`)

#### Current State
- ✅ Basic `__init__.py` with run() function pattern
- ✅ SQLModel-based models.py (but targets wrong output)
- ✅ Extractor patterns for literature sources
- ❌ No world crate generation
- ❌ No Rust templates
- ❌ No literature source integration

#### Redesign Requirements
```python
# New architecture needed:
src/generator/seeds/
├── __init__.py              # Simple run() → generate world crate files
├── models.py               # Rust-compatible Pydantic models
├── extractors.py           # Extract from literature (keep current)
├── processors.py           # Process seeds into world data
├── world_generator.py      # NEW: Generate Rust world crate files
└── templates/              # NEW: Jinja2 templates for Rust generation
    ├── seeds.rs.j2         # Template for crates/world/src/seeds.rs
    ├── biomes.rs.j2        # Template for crates/world/src/biomes.rs
    └── regions.rs.j2       # Template for crates/world/src/regions.rs
```

#### Output Target
Generate these files in world crate:
- `crates/world/src/seeds.rs` - Narrative/motif/semantic seeds
- `crates/world/src/biomes.rs` - Biome definitions with corruption stages
- `crates/world/src/regions.rs` - Region data with horror progression

### 2. Entities System (`src/generator/entities/`)

#### Current State
- ✅ Excellent transformer.py with HBF clustering (70,801 entities)
- ✅ Good processor organization (regions, settlements, factions, dungeons)
- ✅ ML utilities and base processor patterns
- ❌ SQLite models instead of Rust-compatible models
- ❌ Database population instead of world crate generation
- ❌ No Rust templates or world generation

#### Redesign Requirements
```python
# Architecture update needed:
src/generator/entities/
├── __init__.py              # Simple run() → generate world crate files  
├── transformer.py           # Keep current HBF clustering logic ✅
├── models.py               # NEW: Rust-compatible Pydantic models
├── world_generator.py      # NEW: Generate Rust world crate files
├── processors/             # Update to generate world data
│   ├── base.py            # NEW: Base for world data generation
│   ├── regions.py         # Update: Process regions → biome/settlement data
│   ├── settlements.py     # Update: Process settlements → scale/services
│   ├── factions.py        # Update: Process factions → political data
│   └── dungeons.py        # Update: Process dungeons → encounter data
└── templates/             # NEW: Jinja2 templates for Rust generation
    ├── entities.rs.j2     # Template for crates/world/src/entities.rs
    ├── settlements.rs.j2  # Template for settlements data
    ├── factions.rs.j2     # Template for faction data
    └── dungeons.rs.j2     # Template for dungeon data
```

#### Output Target
Generate these files in world crate:
- `crates/world/src/entities.rs` - Entity definitions
- `crates/world/src/settlements.rs` - Settlement data with services/scale
- `crates/world/src/factions.rs` - Faction data with territorial control  
- `crates/world/src/dungeons.rs` - Dungeon data with encounter rules
- `crates/world/src/spatial.rs` - Hex coordinates and spatial relationships

### 3. HBF Integration Strategy

#### Rich Data Available (Keep These Patterns)
```python
# From transformer.py analysis:
REGIONS = [
    "Aurora Bushes", "Vicious Crags", "Javelin Plains", 
    "Shimmering Shores", "Writhing Waters", # ... 27 total
]

SETTLEMENTS = [
    "Village of Harad", "City of Headsmen", "Town of Tinder",
    "Village of Thale", "Village of Tethered Crypt", # ... 10 total  
]

FACTIONS = [
    "The Defiled Wolves", "Fists of Justice", "Militia of Tethered Crypt",
    "The Restless", "Elite Guards of Headsmen" # ... 5 total
]

DUNGEONS = [
    "Bowel of the Raging Pits", "Den of the Raging Pits", 
    "Crypt of the Mindless", # ... 18 total
]
```

#### Entity Processing Results (From Testing)
- **Aurora Bushes region**: 1 entity → complete hex map with Village of Harad, rivers [2,1], trails [2,5]
- **The Defiled Wolves faction**: 1 entity → political alignment assessment, hostility level
- **Bowel of the Raging Pits dungeon**: 7 entities → horror themes, encounter density, treasure assessment

## WORLD CRATE ARCHITECTURE

### Target Structure
```
crates/world/
├── Cargo.toml              # World crate dependencies
├── build.rs                # Build script for ECS generation  
├── src/
│   ├── lib.rs              # World crate API
│   ├── builder.rs          # ECS builder for game integration
│   ├── seeds.rs            # Generated from seeds system
│   ├── biomes.rs           # Generated biome definitions
│   ├── regions.rs          # Generated region data
│   ├── entities.rs         # Generated from entities system
│   ├── settlements.rs      # Generated settlement data
│   ├── factions.rs         # Generated faction data  
│   ├── dungeons.rs         # Generated dungeon data
│   ├── spatial.rs          # Generated hex coordinates
│   ├── encounters.rs       # Generated encounter rules
│   └── psychology.rs       # Generated companion psychology
└── templates/              # Templates for ECS generation
    ├── components.rs.j2    # Bevy components template
    ├── resources.rs.j2     # Bevy resources template
    └── systems.rs.j2       # Bevy systems template
```

### Integration with Game
```rust
// apps/game/build.rs uses world crate:
use world::WorldBuilder;

fn main() {
    let world_builder = world::create_world_builder();
    
    world_builder
        .generate_components("src/world/components.rs")
        .generate_resources("src/world/resources.rs")
        .generate_systems("src/world/systems.rs");
}
```

## IMPLEMENTATION PRIORITY

### Phase 1: Fix Seeds System (Simpler, Establishes Pattern)
1. **Redesign models.py** - Rust-compatible Pydantic models
2. **Create world_generator.py** - Generate Rust files from seeds
3. **Create templates/** - Jinja2 templates for Rust code
4. **Update __init__.py** - run() function generates world crate files
5. **Test pipeline** - Ensure world crate compiles with generated files

### Phase 2: Fix Entities System (Complex, High Value)
1. **Redesign models.py** - Rust-compatible output models  
2. **Create world_generator.py** - Generate Rust files from HBF processing
3. **Update processors/** - Generate world data instead of database entries
4. **Create templates/** - Jinja2 templates for entity data
5. **Test HBF integration** - Process all 70,801 entities → world crate

### Phase 3: World Crate & Game Integration
1. **Create world crate structure** - Cargo.toml, lib.rs, builder.rs
2. **Implement WorldBuilder** - API for ECS generation
3. **Update game build.rs** - Use world crate for component generation
4. **Test full pipeline** - Python → world crate → ECS → game

## SAMPLE DATA MODELS

### Seeds Models (Rust-Compatible)
```python
from pydantic import BaseModel

class NarrativeSeed(BaseModel):
    structure_name: str
    story_beats: list[str] 
    themes: list[str]
    horror_stage: int  # 0-4 dread progression
    corruption_arc: list[str]

class BiomeDefinition(BaseModel):
    name: str
    base_terrain: str
    corruption_stages: list[str]  # per act progression
    encounter_types: list[str]
    resource_availability: dict[str, float]
```

### Entities Models (Rust-Compatible)
```python
class SettlementData(BaseModel):
    name: str
    scale: str  # village, town, city
    services: list[str]  # blacksmith, tavern, shrine  
    corruption_resistance: float
    hex_coordinates: tuple[int, int]
    population_estimate: int

class FactionData(BaseModel):
    name: str
    political_alignment: str  # lawful, chaotic, corrupt
    territorial_reach: str  # local, regional, widespread
    hostility_level: float
    operating_locations: list[str]
    member_composition: dict[str, int]

class DungeonData(BaseModel):
    name: str
    entrance_type: str  # cave-mouth, crypt-portal
    horror_themes: list[str] 
    encounter_density: int  # 1-5 scale
    treasure_level: int  # 1-3 scale
    hex_coordinates: tuple[int, int]
```

## RUST GENERATION TEMPLATES

### Example Template (settlements.rs.j2)
```rust
// Generated from HBF settlement processing
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settlement {
    pub name: &'static str,
    pub scale: SettlementScale,
    pub services: &'static [Service],
    pub coordinates: (i32, i32),
    pub corruption_resistance: f32,
}

pub const SETTLEMENTS: &[Settlement] = &[
    {% for settlement in settlements %}
    Settlement {
        name: "{{ settlement.name }}",
        scale: SettlementScale::{{ settlement.scale|title }},
        services: &[{% for service in settlement.services %}Service::{{ service|title }}, {% endfor %}],
        coordinates: ({{ settlement.hex_coordinates.0 }}, {{ settlement.hex_coordinates.1 }}),
        corruption_resistance: {{ settlement.corruption_resistance }},
    },
    {% endfor %}
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SettlementScale {
    Village,
    Town, 
    City,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Service {
    Blacksmith,
    Tavern,
    Shrine,
    Market,
    Stables,
}
```

## TESTING STRATEGY

### Unit Testing
```python
# Test each processor with sample data
def test_settlement_processor():
    sample_hbf_entities = [
        {"name": "Village of Harad", "services": "tavern, blacksmith"}
    ]
    
    processor = SettlementsProcessor()
    result = processor.process_entities(sample_hbf_entities)
    
    assert result.settlements[0].name == "Village of Harad"
    assert Service.Tavern in result.settlements[0].services
```

### Integration Testing
```bash
# Test full pipeline
python -c "from generator.seeds import run; run(world_crate_path='crates/world/src/')"
python -c "from generator.entities import run; run(hbf_path='memory-bank/world-output/nTR8nJOW.hbf', world_crate_path='crates/world/src/')"

# Test world crate compilation
cd crates/world && cargo check

# Test game build with generated world
cd apps/game && cargo build
```

## ERROR HANDLING PATTERNS

### HBF Processing Errors
```python
try:
    entities = extract_hbf_entities(hbf_path)
    clusters = transformer.cluster_entities(entities)
    world_data = process_clusters(clusters)
except HBFError as e:
    logger.error(f"HBF processing failed: {e}")
    # Fallback to default world data
    world_data = load_default_world_data()
```

### Rust Generation Errors
```python
try:
    rust_code = template.render(world_data=data)
    validate_rust_syntax(rust_code)  # Basic syntax check
    write_rust_file(output_path, rust_code)
except TemplateError as e:
    logger.error(f"Template rendering failed: {e}")
    raise WorldGenerationError(f"Failed to generate {output_path}")
```

## PERFORMANCE OPTIMIZATIONS

### Processing Efficiency
- **Batch Process Similar Entities** - Group by processor type
- **Cache Clustering Results** - Don't re-cluster unchanged HBF data
- **Incremental Generation** - Only regenerate changed world data
- **Parallel Processing** - Process entity categories in parallel

### Memory Management  
- **Stream Large HBF Datasets** - Don't load all 70,801 entities at once
- **Generator-Based Templates** - Use generators for large Rust files
- **Cleanup Intermediate Data** - Free processing data after generation

## AI HANDOFF CHECKLIST

### Prerequisites Understanding
- [ ] **Architecture Mismatch** - Current systems target Godot+SQLite, need Rust+Bevy+ECS
- [ ] **HBF Data Richness** - 70,801 entities with regions, settlements, factions, dungeons
- [ ] **Working Components** - transformer.py clustering logic is good, keep it
- [ ] **Broken Components** - models.py, processors, integration all need redesign

### Implementation Steps
1. [ ] **Start with Seeds** - Simpler system, establishes pattern
   - [ ] Redesign models.py with Pydantic models
   - [ ] Create world_generator.py and templates/
   - [ ] Update __init__.py run() function
   - [ ] Test world crate generation

2. [ ] **Fix Entities System** - Complex but high-value
   - [ ] Keep transformer.py clustering (it works)
   - [ ] Redesign models.py for Rust output
   - [ ] Update all processors to generate world data
   - [ ] Create templates for entity data
   - [ ] Test full HBF processing pipeline

3. [ ] **Create World Crate** - Integration point
   - [ ] Setup crate structure (Cargo.toml, lib.rs)
   - [ ] Implement WorldBuilder API
   - [ ] Create ECS generation templates
   - [ ] Test compilation with generated data

4. [ ] **Game Integration** - Final connection
   - [ ] Update apps/game/build.rs to use world crate
   - [ ] Test ECS component generation
   - [ ] Verify game builds and runs with generated world

### Success Criteria
- [ ] **Seeds system generates valid Rust files** in world crate
- [ ] **Entities system processes all HBF data** into world data
- [ ] **World crate compiles** with generated files
- [ ] **Game build.rs generates ECS components** from world crate
- [ ] **Full pipeline works end-to-end**: Python → Rust → ECS → Game
- [ ] **No SQLite or database dependencies remain**

### Key Resources
- **HBF Database**: `memory-bank/world-output/nTR8nJOW.hbf` (70,801 entities)
- **Redesign Plan**: `memory-bank/entities-seeds-redesign-plan.md`
- **Entities README**: `src/generator/entities/README.md` (comprehensive guide)
- **Current Code**: `src/generator/entities/transformer.py` (keep clustering logic)
- **Constants**: `src/generator/constants.py` (region/settlement/faction/dungeon lists)

## FINAL NOTES

This architectural redesign is **mission-critical** for the project. The current Python systems have excellent HBF data processing logic but output to the wrong format. The redesign preserves the good components (clustering, routing, entity recognition) while fixing the fundamental architecture mismatch.

The end result will be a clean Python → Rust world crate → ECS pipeline that leverages the rich HBF worldbuilding data (70,801 entities with detailed region, settlement, faction, and dungeon information) to generate a proper Rust-native world system for the horror RPG.

**Start with seeds system redesign** - it's simpler and establishes the pattern for entities system. **Focus on Rust code generation** - that's the missing piece that connects everything.
