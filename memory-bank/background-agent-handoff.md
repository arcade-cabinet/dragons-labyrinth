# Dragon's Labyrinth - Background Agent Handoff Prompt

## Mission: Parse and Integrate HBF Export into Dragon's Labyrinth

### Context
You are a background agent tasked with processing a massive HBF (Hexroll Backpack Format) export that contains a complete tabletop RPG world. The user has determined that parsing an existing HBF export is more efficient than generating content from scratch. Your mission is to analyze the SQLite database and ~70,000 HTML files to extract game data and map it to our Bevy ECS components.

## Input Data Location
- **Primary Input**: `/hbf-export/` (repository root)
- **Expected Format**: SQLite database with embedded HTML content
- **File Count**: ~70,000 HTML files containing world data
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

### Phase 1: Analysis
1. **Examine SQLite Schema**
   ```rust
   // Create crate: hbf-parser
   // Dependencies: sqlx, serde, html_parser, regex
   ```

2. **Document Structure**
   - Map table relationships
   - Identify primary content tables
   - Document HTML content patterns
   - Note any JSON/XML embedded data

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

1. **Create Import Module**:
   ```rust
   // src/import/hbf.rs
   pub struct HbfImporter {
       db_path: PathBuf,
       progress: Arc<Mutex<ImportProgress>>,
   }
   
   impl HbfImporter {
       pub async fn import(&self) -> Result<GameWorld, ImportError> {
           // 1. Connect to SQLite
           // 2. Parse tables
           // 3. Extract HTML content
           // 4. Map to ECS
           // 5. Validate consistency
       }
   }
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

## Workflow

1. **Start**: Wait for `/hbf-export/` directory to appear
2. **Analyze**: Examine SQLite schema and sample HTML files
3. **Design**: Create parsing strategy document
4. **Implement**: Build `hbf-parser` crate
5. **Test**: Validate with subset of data
6. **Scale**: Process full dataset
7. **Integrate**: Connect to game systems
8. **Validate**: Ensure narrative consistency
9. **Document**: Update memory banks with progress
10. **Commit**: Make checkpoint commits frequently

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
