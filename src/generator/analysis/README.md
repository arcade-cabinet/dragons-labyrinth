# Dragons Labyrinth Analysis Package

## Overview

The analysis package extracts and analyzes 70,801+ HBF (HexRoll Battle Format) entities from `raw/game.hbf` and generates comprehensive Pydantic models with AI-powered analysis. This system uses a clean 3-phase architecture with intelligent clustering and spatial coordinate extraction.

## Architecture

### Core Components

```
src/generator/analysis/
├── __main__.py              # Clean orchestration - 5 lines of high-level coordination
├── models.py               # Intelligent model architecture with built-in AI generation
├── constants.py            # Entity categories and thresholds
├── transformer.py          # HBF clustering logic (preserved - works excellently)
├── templates/              # Jinja2 templates for AI model generation
│   ├── regions_analysis.j2      # Individual regions model generation
│   ├── settlements_analysis.j2  # Individual settlements model generation  
│   ├── factions_analysis.j2     # Individual factions model generation
│   ├── dungeons_analysis.j2     # Individual dungeons model generation
│   ├── dungeon_container.j2     # Phase 2: Dungeon integration models
│   ├── region_container.j2      # Phase 3: Complete region integration
│   └── init_module.j2           # Template for __init__.py generation
└── utils.py                # Generic OpenAI utilities (in parent package)
```

### Model Architecture

#### RawEntity
Individual HBF entity with automatic clustering and file writing capability:
- **Spatial Extraction**: Extracts hex coordinates (e.g., "W2S51") from entity content
- **UUID Tracking**: Tracks entity UUID and all referenced UUIDs (settlements, factions, dungeons)
- **Smart Clustering**: Auto-routes to appropriate category based on content matching
- **File Management**: Writes itself to disk in proper directory structure

#### RawEntitiesCluster  
Category-specific cluster with AI generation capability:
- **Sample Management**: Collects HTML/JSON files within threshold limits
- **AI Generation**: Uses OpenAI with Jinja2 templates to generate Pydantic models
- **Connection Parsing**: Extracts UUID fields and import paths from generated models
- **Threshold Routing**: Respects sample size limits from constants

#### RawEntities
Master orchestration container:
- **3-Phase Pipeline**: Orchestrates individual → dungeon containers → region containers
- **Auto-Initialization**: Creates all known entity clusters on startup
- **Unified Coordination**: Single interface for complete analysis workflow

## 3-Phase Processing Pipeline

### Phase 1: Individual Category Models
Generates models for each entity category:
- **regions.py** - Hex tile models with spatial coordinates and entity connections
- **settlements.py** - Establishment models with NPC data and economic systems
- **factions.py** - Political entities with territorial control and relationships  
- **dungeons.py** - Area models with encounters, monsters, and treasure

### Phase 2: Dungeon Container Models
Generates **dungeon_container.py** that integrates individual dungeon areas into complete dungeon complexes:
- **Requires**: dungeons.py from Phase 1
- **Integrates**: Area connections, navigation, progression tracking
- **Output**: Complete dungeon models linking areas together

### Phase 3: Region Container Models  
Generates **region_container.py** that integrates ALL models into complete regions:
- **Requires**: ALL individual models + dungeon containers
- **Integrates**: Spatial relationships, economic systems, political control
- **Output**: Master integration for complete regional gameplay

## Data Extraction Features

### Spatial Coordinate Extraction
All templates extract spatial positioning from HBF entities:
- **Hex Coordinates**: "Hex W2S51" patterns from hidden doc-title elements
- **Map Coordinates**: x, y, zoom, hex ID from map-coords anchors
- **Breadcrumb Paths**: Region/settlement/dungeon navigation hierarchies

### UUID Connection Tracking
Comprehensive entity relationship mapping:
- **Entity UUIDs**: From filename patterns (entity_{UUID}.html)
- **Cross-References**: Settlement, faction, dungeon, region UUIDs from links
- **NPC Anchors**: Character UUIDs from anchor elements
- **Hidden Relationships**: Faction memberships from spoiler tags

### Content Analysis
Rich data extraction from HBF content:
- **Economic Systems**: Shop inventories, pricing, trade routes
- **Political Systems**: Faction relationships, territorial control, conflicts
- **Combat Systems**: Monster stat blocks, encounters, treasure distribution
- **Environmental Systems**: Weather patterns, hazards, biome data

## Template System

### Individual Analysis Templates
Each template extracts category-specific data with absolute imports and UUID tracking:

#### regions_analysis.j2
- **Extracts**: Hex coordinates, biome data, settlement/dungeon connections
- **Patterns**: BeautifulSoup extraction for spatial data and entity relationships
- **Output**: RegionHexTile models with complete coordinate and connection data

#### settlements_analysis.j2
- **Extracts**: Establishment data, NPC information, economic systems
- **Patterns**: Shop inventories, tavern menus, faction memberships (often in spoiler tags)
- **Output**: SettlementEstablishment models with social and economic data

#### factions_analysis.j2
- **Extracts**: Political relationships, territorial control, member information
- **Patterns**: Alliance/enemy relationships, stronghold locations, influence zones
- **Output**: FactionEntity models with political and territorial data

#### dungeons_analysis.j2
- **Extracts**: Area layouts, encounter data, treasure distribution
- **Patterns**: Monster stat blocks, environmental descriptions, area connections
- **Output**: DungeonArea models with encounter and navigation data

### Container Templates
Integration templates using connection information from Phase 1:

#### dungeon_container.j2
- **Context**: Uses dungeons.py connection information
- **Integrates**: All dungeon areas into complete complexes
- **Output**: DungeonContainer models for complete dungeon gameplay

#### region_container.j2
- **Context**: Uses ALL individual model connections
- **Integrates**: Regions, settlements, factions, dungeons into unified regional data
- **Output**: RegionContainer models for complete regional gameplay

### Template Features
- **Absolute Imports**: No wildcards, explicit import paths throughout
- **Connection Context**: Templates receive connection information from Phase 1
- **Spatial Integration**: All templates handle coordinate systems and entity relationships
- **Modern Python**: str | None syntax, list[str] types, proper type annotations

## Configuration

### Constants (constants.py)
```python
# Entity categories (from existing constants)
REGIONS = ["Aurora Bushes", "Vicious Crags", ...]  # 27 total
SETTLEMENTS = ["Village of Harad", "City of Headsmen", ...]  # 10 total  
FACTIONS = ["The Defiled Wolves", "Fists of Justice", ...]  # 5 total
DUNGEONS = ["Bowel of the Raging Pits", ...]  # 18 total

# AI generation thresholds
HTML_ENTITIES_SAMPLE_THRESHOLD = 10
JSON_ENTITIES_SAMPLE_THRESHOLD = 5
DEFAULT_MODEL = "gpt-4o-2024-08-06"

# Directory paths
ANALYSIS_OUTPUT_DIR = "analysis"
PROCESSOR_MODELS_DIR = "src/generator/processors/models"
HBF_RAW_FILE = "raw/game.hbf"
```

## Usage

### Command Line
```bash
# Run complete analysis pipeline
hatch run dl_analysis

# Or directly with Python
python -m generator.analysis
```

### Programmatic Usage
```python
from generator.analysis import main
from generator.analysis.models import RawEntities

# Run complete pipeline
main()

# Or use models directly
entities = RawEntities()
# ... add entities
entities.write_all_entities(analysis_dir, logger)
phase1_results = entities.generate_all_individual_models(models_dir, templates_dir, logger)
container_results = entities.generate_container_models(models_dir, templates_dir, phase1_results, logger)
```

## Data Flow

### Input: HBF Database (raw/game.hbf)
70,801+ entities extracted via SQLite query:
```sql
SELECT uuid, value FROM Entities
```

### Processing: Intelligent Clustering
1. **Entity Creation**: RawEntity parses JSON/HTML content and determines category
2. **Cluster Routing**: Entities routed to appropriate RawEntitiesCluster
3. **File Writing**: Entities written to `analysis/{category}/{entity_name}/entity_{uuid}.{ext}`
4. **Sample Collection**: Files collected for AI analysis within threshold limits

### Phase 1: Individual Models
Each category generates comprehensive Pydantic models:
```python
# Generated model structure
class RegionHexTile(BaseModel):
    entity_uuid: str = Field(..., description="UUID from filename")
    hex_coordinate: str | None = Field(None, description="Hex coordinate like 'W2S51'")
    region_uuid: str | None = Field(None, description="Parent region UUID")
    settlement_uuid: str | None = Field(None, description="Settlement in this hex")
    dungeon_uuids: list[str] = Field(default_factory=list, description="Dungeon UUIDs")
    faction_uuids: list[str] = Field(default_factory=list, description="Faction UUIDs")
    
    @classmethod
    def extract_from_html(cls, html_content: str, filename: str) -> "RegionHexTile":
        # BeautifulSoup extraction logic for coordinates and UUIDs
```

### Phase 2: Container Integration
Container models integrate related components:
- **dungeon_container.py**: Integrates individual dungeon areas into complete dungeons
- **region_container.py**: Integrates ALL models into unified regional data

### Output: Generated Models
```
src/generator/processors/models/
├── regions.py              # Individual hex tile models
├── settlements.py          # Individual establishment models  
├── factions.py            # Individual political entity models
├── dungeons.py            # Individual area models
├── dungeon_container.py   # Integrated dungeon complexes
├── region_container.py    # Complete regional integration
└── __init__.py            # Generated exports using template
```

## File Organization

### Analysis Output Structure
```
analysis/
├── regions/
│   ├── aurora_bushes/
│   │   ├── entity_YVyOmKIy.html   # Hex W2S51 with Veterans encounter
│   │   └── entity_*.html
│   └── vicious_crags/
│       └── entity_*.html
├── settlements/  
│   ├── city_of_headsmen/
│   │   ├── entity_6cXq5UWU.html   # Eolandra's Fashion shop
│   │   └── entity_*.html
│   └── village_of_harad/
├── factions/
│   ├── the_defiled_wolves/
│   │   ├── entity_2S5YYS65.html   # Kaelia's Castle (Hex W4S51)
│   │   └── entity_*.html  
│   └── the_fists_of_justice/
└── dungeons/
    ├── caverns_of_the_infernal_lich/
    │   ├── entity_7k14QyHb.html   # Cave area #22
    │   └── entity_*.html
    └── tomb_of_the_grey_ogre/
```

### Generated Model Files
```
src/generator/processors/models/
├── regions.py              # Generated with AI analysis
├── settlements.py          # Generated with AI analysis
├── factions.py            # Generated with AI analysis
├── dungeons.py            # Generated with AI analysis
├── dungeon_container.py   # Generated with connection context
├── region_container.py    # Generated with all model context
└── __init__.py            # Generated with template
```

## API Reference

### RawEntities
Main orchestration interface:

```python
class RawEntities(BaseModel):
    def add_entity(self, uuid: str, value: str):
        """Add entity and auto-route to appropriate cluster."""
    
    def write_all_entities(self, analysis_dir: Path, logger: logging.Logger):
        """Write all clustered entities to disk."""
    
    def generate_all_individual_models(self, models_dir: Path, templates_dir: Path, logger: logging.Logger) -> dict[str, GenerationResults]:
        """Phase 1: Generate individual category models."""
    
    def generate_container_models(self, models_dir: Path, templates_dir: Path, phase1_results: dict[str, GenerationResults], logger: logging.Logger) -> dict[str, GenerationResults]:
        """Phase 2 & 3: Generate container models."""
    
    def get_summary(self) -> dict[str, dict[str, int] | int]:
        """Get entity count summary by category."""
```

### RawEntitiesCluster
Category-specific cluster with AI capabilities:

```python
class RawEntitiesCluster(BaseModel):
    def add_entity(self, entity: RawEntity) -> bool:
        """Add entity if it belongs to this cluster."""
    
    def write_entities_to_disk(self, analysis_dir: Path):
        """Write cluster entities to disk and collect file paths."""
    
    def can_generate_models(self) -> bool:
        """Check if cluster has enough samples for AI generation."""
    
    def generate_models(self, models_dir: Path, templates_dir: Path, logger: logging.Logger) -> GenerationResults:
        """Generate AI models using OpenAI and template system."""
```

### GenerationResults
Results from AI model generation:

```python
class GenerationResults(BaseModel):
    models_generated: list[str]          # Generated file paths
    analysis_notes: list[str]            # Generation details
    connections: ModelConnections | None # UUID connection information
    success: bool                        # Success status
```

### ModelConnections
Connection information for container integration:

```python
class ModelConnections(BaseModel):
    uuid_fields: list[str]        # UUID field names
    connection_fields: list[str]  # Fields connecting to other entities
    import_path: str             # Absolute import path
    exported_classes: list[str]  # Class names exported by model
```

## Spatial Coordinate System

### Hex Coordinate Extraction
Entities contain spatial positioning extracted from HBF content:
- **Format**: "Hex W2S51" patterns from `<div hidden id="doc-title">`
- **Map Data**: x, y, zoom coordinates from `<a class="map-coords">` elements
- **Breadcrumbs**: Navigation hierarchy from breadcrumb spans

### Entity Relationship Mapping
UUID connections extracted from links:
- **Regions**: `/sandbox/nTR8nJOW/region/{uuid}` patterns
- **Settlements**: `/sandbox/nTR8nJOW/location/{uuid}` patterns
- **Factions**: `/sandbox/nTR8nJOW/faction/{uuid}` patterns  
- **Dungeons**: `/sandbox/nTR8nJOW/location/{uuid}` patterns
- **NPCs**: `<a class="npc-anchor" name="{uuid}">` patterns

## AI Integration

### Template-Based Generation
Uses Jinja2 templates with OpenAI file upload API:
- **System Prompts**: Generated from templates with category-specific instructions
- **File Uploads**: Direct upload of written HTML/JSON files (no temporary files)
- **Structured Output**: AI generates complete Pydantic models with BeautifulSoup extraction logic

### Connection Context  
Container templates receive connection information:
```jinja2
# dungeon_container.j2
**Available Individual Models**:
- Import path: `{{ dungeons_connections.import_path }}`
- **Exported Classes**: {{ dungeons_connections.exported_classes | join(', ') }}
- **Connection Fields**: {{ dungeons_connections.connection_fields | join(', ') }}

# Generated imports
{% for class_name in dungeons_connections.exported_classes %}
from {{ dungeons_connections.import_path }} import {{ class_name }}
{% endfor %}
```

### Absolute Import Enforcement
All generated models use absolute imports:
- **No Wildcards**: No `from x import *` patterns
- **Explicit Imports**: Full paths like `from generator.processors.models.settlements import SettlementModel`
- **Connection-Driven**: Import statements generated from Phase 1 connection data

## Sample Entity Processing

### Threshold Management
Processing respects sample size limits from constants:
- **HTML Threshold**: 10 files per category (configurable)
- **JSON Threshold**: 5 files per category (configurable)
- **Intelligent Sampling**: Best representative samples selected automatically

### Entity Categories
Automatically routed based on content matching:

#### Regions (27 total)
- Aurora Bushes, Vicious Crags, Javelin Plains, etc.
- **Sample**: "Hex W2S51 in Aurora Bushes" with Veterans encounter

#### Settlements (10 total)  
- Village of Harad, City of Headsmen, etc.
- **Sample**: "Eolandra's Fashion" clothing shop with faction connections

#### Factions (5 total)
- The Defiled Wolves, Fists of Justice, etc.
- **Sample**: "Kaelia's Castle" at "Hex W4S51" with faction membership

#### Dungeons (18 total)
- Bowel of the Raging Pits, Caverns of the Infernal Lich, etc.
- **Sample**: "Cave area #22 in Caverns of the Infernal Lich" with treasure

## Generated Model Structure

### Individual Models
AI-generated models with comprehensive extraction methods:

```python
# Example from regions.py (AI-generated)
class RegionHexTile(BaseModel):
    # Entity identification  
    entity_uuid: str = Field(..., description="UUID from filename")
    
    # Spatial coordinates
    hex_coordinate: str | None = Field(None, description="Hex coordinate like 'W2S51'")
    map_x: float | None = Field(None, description="Map X coordinate")
    map_y: float | None = Field(None, description="Map Y coordinate")
    
    # Entity connections
    region_uuid: str | None = Field(None, description="Parent region UUID")
    settlement_uuid: str | None = Field(None, description="Settlement UUID in this hex")
    dungeon_uuids: list[str] = Field(default_factory=list, description="Dungeon UUIDs")
    faction_uuids: list[str] = Field(default_factory=list, description="Faction UUIDs")
    
    @classmethod  
    def extract_from_html(cls, html_content: str, filename: str) -> "RegionHexTile":
        # Complete BeautifulSoup extraction implementation
```

### Container Models
Integration models combining individual components:

```python
# Example from dungeon_container.py (AI-generated)
from generator.processors.models.dungeons import DungeonArea

class DungeonContainer(BaseModel):
    dungeon_uuid: str
    areas: list[DungeonArea]
    area_connections: dict[str, list[str]]
    entrance_hex: str
    progression_tracking: dict[str, Any]
    # Complete integration logic
```

## Error Handling

### Validation Errors
- **Pydantic Validation**: Comprehensive field validation with descriptive error messages
- **OpenAI Integration**: Proper error handling for API failures with file cleanup
- **Template Rendering**: Error handling for missing templates or invalid context

### Logging
Uses Rich logging with structured output:
```python
[11:10:30] INFO     Extracting entities from /path/to/game.hbf
           INFO     Writing clustered entities to disk...
           INFO     PHASE 1: Generating individual category models...
           INFO     ✓ Generated models for regions
```

## Development Workflow

### 1. Edit Templates
Modify Jinja2 templates in `templates/` for different extraction patterns

### 2. Run Analysis
```bash
hatch run dl_analysis
```

### 3. Review Generated Models
Check `src/generator/processors/models/` for AI-generated output

### 4. Integration Testing
Use generated models in processor phase for Rust ECS component generation

## Performance Considerations

### Memory Efficiency
- **Streaming Processing**: Entities processed individually, not loaded all at once
- **Threshold Limits**: Sample sizes limited to prevent memory issues
- **File-Based**: Uses disk storage for entity files instead of keeping in memory

### API Usage Optimization
- **Sample Limits**: Respects thresholds to minimize OpenAI API usage
- **File Uploads**: Direct file uploads instead of creating temporary concatenated files
- **Connection Reuse**: Connection information reused across container generation

### Generation Speed
- **Parallel Processing**: Category models generated independently
- **Template Caching**: Jinja2 templates cached by environment
- **Incremental Updates**: Only regenerates when source data changes

## Integration Points

### Transformer Integration
Uses existing excellent clustering logic from `transformer.py`:
- Preserves entity matching patterns
- Maintains category routing efficiency
- Keeps fallback handling for edge cases

### Processor Pipeline
Feeds into processor phase for Rust ECS generation:
- Generated models provide extraction patterns
- Connection information enables cross-entity relationships
- Spatial coordinates enable proper game world positioning

### Utilities Integration
Uses generic utilities from `generator.utils`:
- OpenAI integration with file uploads
- Template rendering to files
- Git repository detection

## Troubleshooting

### Common Issues
1. **Missing Entities**: Check constants.py for entity names matching HBF content
2. **Template Errors**: Verify Jinja2 syntax and context variables
3. **OpenAI Failures**: Check API key and model availability
4. **Import Errors**: Ensure absolute import paths match generated structure

### Debug Information
- **Rich Logging**: Detailed progress and error information
- **Sample Counts**: Threshold compliance and file collection status
- **Connection Data**: UUID field extraction and import path generation
- **Generation Results**: Success/failure status for each phase

## Example Output

After successful analysis run:
```
=== ANALYSIS SUMMARY ===

REGIONS:
  Aurora Bushes: 45 entities
  Vicious Crags: 23 entities
  ...
  TOTAL regions: 958 entities
  ✓ AI models generated: 1 files

SETTLEMENTS:
  Village of Harad: 12 entities
  City of Headsmen: 34 entities
  ...
  TOTAL settlements: 316 entities
  ✓ AI models generated: 1 files

Total entities processed: 70,801
Analysis directories created in analysis/
AI-generated processor models available for review
Ready for processor phase!
```

This analysis package provides the foundation for converting rich HBF world data into structured Pydantic models with complete spatial and relationship information for downstream Rust ECS generation.

# Dragons Labyrinth Analysis Package

## Overview

The analysis package extracts and analyzes 70,801+ HBF (HexRoll Battle Format) entities from `raw/game.hbf` and generates **clean Pydantic models** through a **two‑stage AI pipeline**. The system clusters raw HTML/JSON entities, infers a strict JSON inventory via Structured Outputs, and renders deterministic Python models from Jinja.

## Current Architecture (post‑refactor)

```
src/generator/analysis/
├── __main__.py              # Orchestration entrypoint
├── README.md                # (this file)
├── constants.py             # Entity categories, thresholds, paths
├── transformer.py           # HBF clustering logic (unchanged)
├── utils.py                 # OpenAI + templating helpers
└── models.py                # Transitional: cluster orchestration + AI gen (to be migrated)
```

### Key Runtime Models

- **`RawEntity`**
  - Parses JSON vs HTML, determines category + entity name, writes canonical files to `analysis/<category>/<slug>/entity_<uuid>.(html|json)`.
- **`BaseEntitiesCluster` (abstract)** and specialized clusters:
  - `RawRegionEntities`, `RawSettlementEntities`, `RawFactionEntities`, `RawDungeonEntities`
  - **Stage A (Analysis)**: Uses **Responses Structured Outputs** with a strict JSON schema to produce a **field inventory** and **UUID connection map**. *No code emitted by the model.*
  - **Stage B (Codegen)**: Renders **deterministic Pydantic** classes from the Stage A JSON using an inline Jinja template. *No BeautifulSoup/regex logic in models.*
- **`RawEntities`**
  - Orchestrates Phase 1 (individual category models), Phase 2 (dungeon container), Phase 3 (region container).

## What’s NEW and why it matters

1) **Strict Structured Outputs**: We now force the model to return JSON that conforms to a schema (no prose/code). This removes scraping/bs4 hallucinations.
2) **Deterministic Codegen**: Jinja templates produce clean Pydantic 2 models that comply with `.clinerules`.
3) **Specialized Clusters**: Category‑specific prompts/schemas/templates ensure better field discovery and consistent UUID connection mapping.

## What’s STILL MISSING (gaps to close)

> These are prioritized fixes the next iteration will address.

### A. Spatial Relationship Links (explicit, machine‑readable)
- **Hex Tile ↔ Entity relationships**:
  - We currently store hex coordinates as *fields on models*, but we do **not** persist a **normalized edge** from *any entity* to its **canonical `RegionHexTile` node** (e.g., `hex_id` + `region_uuid` composite key).
  - **Add**: `hex_uuid` (or canonical `hex_key`) on child entities and a **reverse index** in containers so queries like “all entities in hex W2S51” are O(1).
- **Dungeon Area ↔ Coordinates relationships**:
  - We extract area numbers and textual coordinates but do **not** emit a **stable `area_key`** usable globally (e.g., `dungeon_uuid:area_number`).
  - **Add**: `area_key` and **bidirectional edges**: area → hex, hex → areas; dungeon → areas; areas ↔ connected_areas.

### B. UUID Edge Typing & Integrity
- We record UUIDs but do **not** enforce **edge types** (e.g., `faction_controls_region`, `settlement_in_hex`, `area_connects_to_area`).
- **Add**: a small **edge taxonomy** + `extract_uuid_connections()` to return `{field_name: entity_kind, edge_type}`.

### C. Coordinate Systems Normalization
- **Hex coordinates** (`W2S51`) and **map coords** (`x,y,hex_id`) appear as free text/optional floats.
- **Add**: `HexKey` value object (`str`), `MapCoord` model (`x: float`, `y: float`, `hex_id: str | None`), unify parsing at transform time.

### D. Container‑Level Indexes
- Container models should materialize **indexes**:
  - `by_hex[hex_key] -> set[EntityRef]`
  - `by_area[area_key] -> DungeonArea`
  - `neighbors[area_key] -> list[area_key]`

### E. Provenance
- Persist minimal provenance per field set (source files, snippet hashes) to let downstream processors trace/verify.

## Roadmap (immediate next steps)

1) **New Subpackage: `analysis/models/`**
   - Break out the models into a true package and keep `models.py` as a thin shim or mark deprecated.
2) **Base + Specialized + Containers**
   - `base.py` → shared value objects (`HexKey`, `MapCoord`), `BaseEntitiesCluster`, common mixins.
   - `regions.py`, `settlements.py`, `factions.py`, `dungeons.py` → Pydantic entities per category + `extract_uuid_connections()`.
   - `containers.py` → `DungeonContainer`, `RegionContainer` with indexes (`by_hex`, `by_area`, `neighbors`).
   - `README.md` inside the package documenting module responsibilities and the edge taxonomy.
3) **Edge Typing**
   - Introduce `EdgeType` enum and return structured edges from `extract_uuid_connections()`.
4) **Coordinate Normalization**
   - Centralize parsing of `W2S51` and `(x,y,hex)` into `base.py` helpers; no per‑model parsing.

## Phase Pipeline (unchanged conceptually)

- **Phase 1**: Category models (regions/settlements/factions/dungeons)
- **Phase 2**: `DungeonContainer` (areas → complexes; neighbors; entrances)
- **Phase 3**: `RegionContainer` (all category integration + spatial indexes)

## Package Layout (target state)

```
src/generator/analysis/models/
├── __init__.py
├── README.md
├── base.py          # BaseEntitiesCluster, HexKey, MapCoord, EdgeType, helpers
├── regions.py       # Region entities
├── settlements.py   # Settlement entities
├── factions.py      # Faction entities
├── dungeons.py      # Dungeon entities
└── containers.py    # DungeonContainer, RegionContainer + indexes
```

### Migration Notes
- The existing `models.py` will be deprecated after the package is adopted.
- Short‑term: keep `models.py` importing from `analysis.models.*` to avoid breaking callers.

## Usage

```bash
# Run complete analysis pipeline
hatch run dl_analysis

# Or directly
python -m generator.analysis
```

Programmatic:

```python
from generator.analysis import main

# Or new package usage (target state)
from generator.analysis.models import base, regions, settlements, factions, dungeons, containers
```

## Troubleshooting & Debugging
- Check **thresholds** in `constants.py` if sampling is off.
- Enable verbose logging to see **Stage A** token budgeting and structured output diagnostics.
- Validate **edge typing** by calling `extract_uuid_connections()` on generated classes.