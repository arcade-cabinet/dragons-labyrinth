# Hexroll Transformer Analysis

## Executive Summary
The new `crates/hexroll-transformer` is a complete ground-up rewrite that takes a fundamentally different approach from all previous attempts. It's simpler, more direct, and leverages AI intelligently.

## Architecture Overview

### Core Philosophy
- **Single-pass transformation**: HBF SQLite → Normalized game.db
- **AI-enhanced classification**: OpenAI for ambiguous content
- **SeaORM-based**: Type-safe database operations
- **Yarn-integrated**: Automatic dialogue generation

### Key Components

#### 1. Data Models (`models.rs`)
- `PageType` enum: 20+ variants (Hex, Settlement, NPC, Dungeon, etc.)
- Rich data structures for each entity type
- `PageResult`: Aggregates all extracted data with optional typed payloads
- **Issue Found**: Duplicate PageType enum definition

#### 2. HTML/JSON Extraction (`extractor.rs`)
- Sophisticated regex and DOM parsing with `scraper` crate
- Specialized extractors for:
  - Hex pages with coordinates and biomes
  - Settlements with population data
  - NPCs with full D&D 5e stat blocks
  - Shops with type classification via `<em>` tags
  - Weather tables (dry/wet columns)
  - Rumor tables (d6 rolls)
  - Dungeons/Caves/Temples/Tombs
  - Factions and dwellings
- Helper functions for clean text extraction
- Pattern detection for page classification

#### 3. AI Analysis (`analyzer.rs`)
- `OpenAiDiveAnalyzer` using openai_dive crate
- Batch classification of HTML fragments
- Token limiting with tiktoken-rs (3500 token max)
- NPC dialogue generation in Yarn Spinner format
- Efficient clustering to minimize API calls

#### 4. Database Schema (`orm.rs`)
- 15 SeaORM entities:
  - `realm` - Top-level container
  - `region` - Contains hexes
  - `biome` - Region subdivisions
  - `hex` - Individual map tiles
  - `settlement` - Cities/Towns/Villages
  - `inn` - Isolated healing places
  - `dwelling` - Farms/Cabins or Strongholds
  - `dungeon` - Caves/Temples/Tombs
  - `monster` - Creature stat blocks
  - `npc` - Character data
  - `faction` - Groups/organizations
  - `npc_faction` - Many-to-many relationships
  - `shop` - Settlement businesses
  - `rumor` - Quest hooks
  - `weather` - Regional weather tables
  - `dialogue` - Yarn scripts per NPC
- Proper foreign key relationships
- UUID preservation for traceability

#### 5. Transformation Pipeline (`pipeline.rs`)
- `HexrollTransformer` orchestrator
- Direct SQLite reading with rusqlite
- Filters 68,556 empty entities
- Processes 2,245 content entities
- Creates normalized database with SeaORM
- **Issue**: Only partially implements export (hexes, weather, rumors)

#### 6. Yarn Integration (`yarn_integration.rs`)
- Writes Yarn dialogue files to `assets/dialogue/`
- Sanitizes filenames for compatibility
- Includes Bevy integration example

### Data Flow

```
70,801 HBF Entities
    ↓ (filter empty)
2,245 Content Entities
    ├── 47 JSON map blobs
    ├── 1,013+ HTML pages  
    └── Others
    ↓ (classify)
Page Types (20+ variants)
    ↓ (extract)
Structured Data
    ↓ (normalize)
14 Database Tables
    ↓ (generate)
Yarn Dialogues
```

## Key Insights

### Why 70k Entities with 68k Empty?
The HBF export creates placeholder UUIDs for every possible entity in the world, but only populates ~3% with actual content. This is likely for:
- Future expansion slots
- Maintaining consistent UUID space
- Simplifying reference management

### Content Distribution
- **617 Hex pages**: Main world tiles with encounters
- **335 Dungeon areas**: Room descriptions
- **254 Cave areas**: Natural cavern systems  
- **~60 Shop types**: From Blacksmith to Veterinarian
- **Multiple NPCs**: With full D&D 5e stat blocks
- **Weather/Rumor tables**: Per hex/settlement

### HTML Pattern Recognition
- Document titles: `<div id="doc-title">...</div>`
- Shop types: `<em>Type</em>` within parentheses
- Settlement detection: "Village of", "Town of", "City of"
- Hex coordinates: "Hex N2", "Hex E1", etc.
- NPC stats: Tables with STR/DEX/CON/INT/WIS/CHA

## Integration Strategy

### For Dragon's Labyrinth

1. **Run transformation**: 
   ```bash
   cargo run --bin hexroll_transformer -- game.hbf --export-db game.db
   ```

2. **Use with bevy_sqlx**:
   - Load entities from game.db at runtime
   - Hydrate ECS components from SeaORM models
   - Maintain UUID → Entity mapping

3. **Dialogue system**:
   - Generated Yarn scripts in dialogue table
   - Use bevy_yarnspinner for playback
   - AI generates contextual conversations

### Entity Relationship Mapping (ERM)
Since all entities preserve original UUIDs:
- Keep Entities/Refs tables for traceability
- New normalized tables reference original UUIDs
- Build Entity → Entity relationships in Bevy
- Cross-reference via WorldIndex resource

## Current Issues to Address

1. **Fix duplicate PageType enum** in models.rs
2. **Create missing CLI binary** 
3. **Complete pipeline.export_to_sea_db()** for ALL entity types
4. **Process Refs relationships** properly
5. **Improve error handling** (too many unwrap())

## Advantages Over Previous Attempts

- **Simpler**: Single-pass vs complex multi-stage
- **AI-smart**: Only calls AI for unknowns/clusters
- **Direct**: No intermediate formats
- **Integrated**: Yarn from the start
- **Type-safe**: SeaORM ensures correctness

## Next Steps

1. Fix the identified issues
2. Create the CLI binary
3. Complete the export implementation
4. Run actual transformation on game.hbf
5. Integrate with game-database
6. Wire up bevy_yarnspinner

## Conclusion

This new hexroll-transformer is architecturally sound and much cleaner than previous attempts. The combination of heuristic parsing for known patterns and AI for edge cases is optimal. The direct HBF → game.db approach with SeaORM is the right path forward.
