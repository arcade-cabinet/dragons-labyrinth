# Dragon's Labyrinth - Background Agent Handoff Prompt

## Mission: Parse and Integrate HBF Export into Dragon's Labyrinth

### Context
You are a background agent tasked with processing a massive HBF (Hexroll Backpack Format) export that contains a complete tabletop RPG world. The user has determined that parsing an existing HBF export is more efficient than generating content from scratch. Your mission is to analyze the SQLite database and ~70,000 HTML files to extract game data and map it to our Bevy ECS components.

## Input Data Location
- **Primary Input**: `/hbf-export/` (repository root)
- **SQLite Database**: `nTR8nJOW.hbf` (24MB)
  - `Entities` table: 70,801 records (uuid TEXT PRIMARY KEY, value TEXT)
  - `Refs` table: 1,570 records (value, details, uuid, type, icon, anchor)
- **HTML Files**: `/hbf-export/hbf_export_data/entities/` - 70,800 HTML files
  - Named by UUID (e.g., `1l4YQODC.html`)
  - Contains dungeon rooms, NPCs, locations, treasures
  - Fully rendered HTML with stats, descriptions, rolls
- **JSON Files**:
  - `map.json`: 146KB hex map with tiles, regions, realms, borders
  - `refs.json`: 277KB reference/search metadata
- **Exporter Tool**: `hexroll_exporter/` - Rust crate for processing HBF files
- **Data Types**: Maps, NPCs, settlements, dungeons, encounters, items, quests

## Technical Stack
- **Game Engine**: Bevy 0.16.1 with ECS architecture
- **Database**: SeaORM 1.1.14 for persistence
- **Asset Pipeline**: RON → GLTF generation (no Blender dependency)
- **Language**: Rust with async/await support

## Core Systems Already Implemented

### 1. Asset Pipeline (✅ Complete)
- `blender-bridge`: Converts RON to GLTF without Blender
- `simple_gltf`: Pure Rust GLTF generation
- `game-content-static`: Build script for asset generation
- All Python scripts converted to RON format

### 2. Game Systems (✅ Complete)
- **180-Level Structure**: journey_to/from/seal_void arcs
- **Hex World**: Tactical overworld with elevation/weather/corruption
- **Combat**: 7 wolf variants, weather effects, elemental damage
- **Save System**: player.db (persistent) vs game.db (playthrough)
- **3D Labyrinths**: DOOM-style procedural generation
- **Companions**: Elena/Marcus/Quinn with full dialogue trees
- **Death Scars**: Permanent modifications system
- **Philosophy**: Dual-axis alignment (Strength/Harmony, Light/Dark)

### 3. Crate Architecture
```
crates/
├── blender-bridge/      # Asset conversion (RON → GLTF)
├── game-content-static/ # Static content and build scripts
├── game-database/       # Save system and persistence
├── database-orm/        # SeaORM model definitions
└── game-engine/         # Main game with Bevy

src/
├── systems/
│   ├── hex_world.rs    # Hex world implementation
│   └── labyrinth_3d.rs # 3D dungeon generation
├── components/          # ECS components
├── resources/           # Game resources
└── generators/          # Content generation
```

## Your Task: HBF Parser Implementation

### CRITICAL ARCHITECTURAL CHANGE (User Direction)
The user wants proper integration, not a quick parse:

1. **Move the crate** (PARTIALLY DONE):
   - ✅ Moved `hexroll_exporter` → `crates/hbf-parser`
   - ✅ Created `crates/hbf-parser/import/` directory
   - ✅ Moved `nTR8nJOW.hbf` → `crates/hbf-parser/import/`
   - ⚠️ Still need to move HTML files to `import/` directory

2. **Pattern Recognition Priority**:
   - NOT all 70k HTML files are unique - find the patterns!
   - Create `crates/hbf-parser/patterns/` directory
   - Identify patterns for:
     - Dungeons (rooms, layouts, themes)
     - Inns (services, NPCs, rumors)
     - NPCs (stat blocks, personalities, dialogue)
     - Weather events
     - Random encounters
     - Treasure hoards
     - Magic items
     - Settlements
     - Wilderness areas
   - Store patterns as RON or TOML (determine optimal)

3. **Build.rs Implementation**:
   - Load HBF database
   - Load pattern definitions
   - Generate Bevy ECS systems
   - Generate hex map data structures
   - Output to `src/generated/` within the crate

### NEW APPROACH: Pattern-Based Generation

Instead of parsing all 70k files at runtime, the user wants:

```rust
// crates/hbf-parser/build.rs
fn main() {
    // 1. Analyze HTML files to find patterns
    let patterns = extract_patterns("import/hbf_export_data/entities/");
    
    // 2. Save patterns to RON/TOML
    save_patterns("patterns/", patterns);
    
    // 3. Generate Rust code from patterns
    generate_dungeon_systems(patterns.dungeons);
    generate_npc_systems(patterns.npcs);
    generate_encounter_tables(patterns.encounters);
    generate_hex_features(patterns.terrain);
    
    // 4. Output to src/generated/
    println!("cargo:rerun-if-changed=import/");
    println!("cargo:rerun-if-changed=patterns/");
}
```

**Pattern Examples to Find**:
- **Dungeon Rooms**: "Chamber", "Corridor", "Hall", "Vault"
- **NPCs**: `<div class="statblock">`, CR ratings, stat arrays
- **Treasures**: "gp worth", "Magic Items:", "artifacts"
- **Encounters**: "will attack", "hostile", "friendly"
- **Settlements**: "Inn", "Tavern", "Shop", "Temple"
- **Features**: "ruins", "cave", "tower", "bridge"

## Your Task: HBF Parser Implementation

### Phase 1: Analysis (✅ COMPLETED BY USER)

**Discovered Structure**:
1. **SQLite Schema**:
   ```sql
   CREATE TABLE Entities (uuid TEXT PRIMARY KEY, value TEXT);
   CREATE TABLE Refs (value TEXT, details TEXT, uuid TEXT, type TEXT, icon TEXT, anchor TEXT);
   ```

2. **Key Entities**:
   - `map` entity: Contains complete hex map as JSON (146KB)
   - Individual entities: Most have empty values in DB, content is in HTML files
   - Cross-references in Refs table for search/navigation

3. **HTML Content Pattern** (from `1l4YQODC.html`):
   ```html
   <a class="map-coords" hex="ysvG50Pq" x="176" y="-25" zoom="2">
   <h4><span id="editable-title">Chamber</span></h4>
   <h5>Foreshadowing</h5>
   <h5>Description</h5>
   <blockquote>...</blockquote>
   <div class="monster-block">...</div>
   <ul>Monster Hoard: coins, artifacts, magic items</ul>
   ```

4. **Map JSON Structure** (from `map.json`):
   ```json
   {
     "map": [{
       "x": 0, "y": 0,
       "type": "JungleHex",
       "uuid": "gDhlZths",
       "feature": "Other",
       "rivers": [2, 1],
       "trails": [2, 5],
       "region": "FstfgXXx",
       "realm": "X7li5Fcx"
     }],
     "realms": {}, "regions": {}, "borders": {}
   }
   ```

### Phase 2: Data Extraction
1. **Core Entities to Extract**:
   - **Hexes**: Position, terrain, features, encounters
   - **Settlements**: Name, population, NPCs, buildings
   - **NPCs**: Names, stats, relationships, dialogue hints
   - **Dungeons**: Layout, rooms, traps, treasure
   - **Encounters**: Combat tables, special events
   - **Items**: Equipment, treasure, special items
   - **Quests**: Objectives, rewards, connections

2. **HTML Parsing Strategy**:
   ```rust
   // Parse HTML content for structured data
   struct HexrollContent {
       title: String,
       sections: Vec<Section>,
       tables: Vec<Table>,
       links: Vec<CrossReference>,
   }
   ```

### Phase 3: ECS Mapping

Map Hexroll data to our existing components:

```rust
// Hex World Mapping
HexrollHex → HexTile {
    position: HexPosition,
    tile_type: TileType,
    elevation: i32,
    corruption: f32,
    weather: Weather,
}

// Settlement Mapping  
HexrollSettlement → Village {
    name: String,
    population: u32,
    corruption_level: f32,
    npcs: Vec<Entity>,
    buildings: Vec<Building>,
}

// NPC Mapping
HexrollNPC → Companion/Villager {
    name: String,
    personality: PersonalityTraits,
    dialogue: DialogueTree,
    trust: f32,
    corruption: f32,
}

// Dungeon Mapping
HexrollDungeon → Labyrinth {
    layout: LabyrinthLayout,
    rooms: Vec<Room>,
    boss: Option<BossEncounter>,
    corruption_theme: CorruptionLevel,
}
```

### Phase 4: Integration Pipeline

1. **Use Existing Hexroll Exporter**:
   The user has provided `hexroll_exporter` crate that already handles HBF parsing:
   ```rust
   // hexroll_exporter/src/loader.rs - Already implemented!
   pub struct HexrollSnapshot {
       pub map: MapData,
       pub entities: HtmlEntities, // HashMap<String, String>
       pub refs: Vec<RefRecord>,
   }
   
   pub struct MapTile {
       pub x: i32, pub y: i32,
       pub biome: String,
       pub uuid: String,
       pub feature: String,
       pub rivers: Vec<u8>,
       pub trails: Vec<u8>,
       pub region: Option<String>,
       pub realm: Option<String>,
   }
   ```
   
   **Usage**:
   ```rust
   use hexroll_exporter::loader::load_snapshot;
   
   let snapshot = load_snapshot(&Path::new("nTR8nJOW.hbf"))?;
   // snapshot.map.tiles - All hex tiles
   // snapshot.entities - All HTML content by UUID
   // snapshot.refs - Search/reference metadata
   ```

2. **Progress Tracking**:
   - Track parsing progress (0-100%)
   - Log extracted entity counts
   - Report mapping conflicts
   - Validate data integrity

### Phase 5: Data Enrichment

Enhance Hexroll data with our game-specific features:

1. **Add Horror Progression**:
   - Map areas to dread levels (0-4)
   - Assign corruption values based on distance from labyrinth
   - Add void rifts at appropriate locations

2. **Integrate Narrative**:
   - Place father's journal pages
   - Position philosophy crystals
   - Add sentimental items
   - Create companion interaction points

3. **Apply Combat Variants**:
   - Replace generic wolves with our 7 variants
   - Add weather-specific spawning
   - Integrate elemental damage types

## Expected Challenges

### 1. Data Volume
- 70k HTML files require efficient batch processing
- Use async/parallel parsing where possible
- Implement caching for repeated patterns

### 2. HTML Structure Variations
- Hexroll may use different HTML patterns
- Build robust parsers with fallbacks
- Log unparseable content for manual review

### 3. Coordinate Systems
- Hexroll uses hex coordinates
- Map to our Bevy hex grid system
- Ensure consistent positioning

### 4. Content Gaps
- Hexroll won't have our specific mechanics
- Need to augment with Dragon's Labyrinth features
- Maintain narrative consistency

## Success Criteria

### Must Have
- [ ] Parse 90%+ of HBF content successfully
- [ ] Map all major entity types to ECS
- [ ] Maintain 180-level progression structure
- [ ] Preserve companion personalities
- [ ] Integrate horror progression

### Nice to Have
- [ ] Visual preview of imported world
- [ ] Diff tool for comparing imports
- [ ] Selective import options
- [ ] Content validation reports

## Code Patterns to Follow

### Error Handling
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HbfError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("HTML parsing failed: {0}")]
    HtmlParse(String),
    
    #[error("ECS mapping failed: {entity}")]
    EcsMapping { entity: String },
}
```

### Async Processing
```rust
use tokio::task::JoinSet;

async fn parse_html_batch(files: Vec<PathBuf>) -> Result<Vec<ParsedContent>> {
    let mut tasks = JoinSet::new();
    
    for file in files {
        tasks.spawn(async move {
            parse_single_html(file).await
        });
    }
    
    // Collect results
    let mut results = Vec::new();
    while let Some(res) = tasks.join_next().await {
        results.push(res??);
    }
    
    Ok(results)
}
```

## Memory Bank Context

### Key Documents
- `memory-bank/active-development/progress.md` - Current state
- `memory-bank/active-development/activeContext.md` - Focus areas
- `memory-bank/active-development/techContext.md` - Technical details
- `memory-bank/active-development/systemPatterns.md` - Code patterns

### Design Documents
- `crates/dragons-docs/book/design/projectbrief.md` - Core vision
- `crates/dragons-docs/book/design/` - Full design specs
- `crates/dragons-docs/book/tech/` - Architecture details

## Workflow (UPDATED)

1. **Complete Migration**: 
   - Move `hbf-export/hbf_export_data/` → `crates/hbf-parser/import/`
   - Update `Cargo.toml` to include in workspace
   - Add to git and commit

2. **Pattern Discovery Phase**:
   - Sample 100-200 HTML files from each category
   - Identify common HTML structures and patterns
   - Document pattern templates in `patterns/README.md`
   - Create RON/TOML definitions for each pattern type

3. **Build.rs Implementation**:
   - Create pattern extractor functions
   - Generate Rust code from patterns
   - Create ECS component definitions
   - Generate encounter tables and loot tables
   - Output to `src/generated/`

4. **Integration**:
   - Connect generated systems to existing game
   - Map to 180-level progression
   - Add horror progression overlays
   - Test with sample data

5. **Validation**:
   - Ensure all major content types are captured
   - Verify pattern coverage (aim for 90%+ of files)
   - Test generated code compiles and runs

6. **Documentation**:
   - Update memory banks with discovered patterns
   - Document pattern format for future content
   - Create usage examples

## Environment Variables
- `OPENAI_API_KEY` - Available for content enhancement
- `FREESOUND_API_KEY` - Available for audio generation

## Performance Targets
- Parse rate: >1000 HTML files/second
- Memory usage: <2GB during import
- Import time: <5 minutes for full dataset
- ECS mapping: 100% of core entities

## Final Notes

This is a critical pivot that will accelerate development significantly. The HBF export contains months of procedurally generated content that would take extensive AI processing to recreate. Your role is to efficiently extract, transform, and integrate this content while maintaining the horror progression and narrative structure we've carefully designed.

Remember: We're not just importing data - we're weaving Hexroll's world generation into Dragon's Labyrinth's horror journey. Every hex, every NPC, every dungeon must serve the greater narrative of transformation and corruption.

The user has high confidence in this approach based on their research. Make it work efficiently and comprehensively.

## Status Tracking

Update these files as you work:
- `memory-bank/active-development/progress.md` - Log milestones
- `memory-bank/active-development/hbf-import-status.md` - Create this for detailed tracking
- Make commits with prefix `hbf:` for easy tracking

Good luck, agent. The fate of 180 levels rests in your parsing skills.
