# Complete HexRoll HBF Data Structure Analysis

## CRITICAL DISCOVERY: We were only looking at 3% of the data!

### Data Distribution (70,801 total entities)
- **68,556 Empty Entities (96.8%)**: Placeholder/index entries with empty values
- **1,013 HTML Entities (1.4%)**: Rich HTML content (shops, dungeons, settlements)
- **47 JSON Entities (0.07%)**: Map data with UUID references
- **~1,185 Other Entities (1.7%)**: Plain text or other formats

### Refs Table (1,570 entries - the ACTUAL content)
- **920 Locations**: NPCs, shops, places with names and descriptions
- **645 Hexes**: Map hex definitions
- **5 Factions**: Group/faction data

## The REAL Data Model

```
┌─────────────────────┐
│   ENTITIES TABLE    │
│   (70,801 rows)     │
├─────────────────────┤
│ uuid (TEXT)         │ ←──── Primary Key
│ value (TEXT)        │       
└─────────────────────┘
         │
         ├── 68,556 Empty → Placeholders for game indices
         ├── 1,013 HTML → Rich content with embedded UUID refs
         ├── 47 JSON → Map data with UUID references
         └── 1,185 Other → Plain text content
                │
                └──── Contains UUID references to ──→
                                                      │
┌─────────────────────┐                              ↓
│    REFS TABLE       │ ←─────────────────────────────
│   (1,570 rows)      │
├─────────────────────┤
│ uuid (TEXT)         │ ←──── Referenced by Entities
│ value (TEXT)        │       Names (NPCs, places)
│ details (TEXT)      │       Descriptions
│ type (TEXT)         │       location|hex|faction
│ icon (TEXT)         │       Visual indicators
│ anchor (TEXT)       │       Link anchors
└─────────────────────┘
```

## What We've Been Missing

1. **Empty entities are intentional** - They're placeholders in the game's indexing system
2. **JSON entities contain map structure** - These define the world layout
3. **HTML entities reference Refs by UUID** - Cross-references create relationships
4. **Refs table is the master data** - All NPCs, locations, and world elements

## Extraction Strategy

### Phase 1: Extract ALL Refs (1,570 entries)
- Locations (920): NPCs, shops, buildings
- Hexes (645): Map tiles with terrain
- Factions (5): Groups and organizations

### Phase 2: Process HTML Entities (1,013 entries) 
- Parse HTML structure
- Extract embedded UUID references
- Map relationships to Refs

### Phase 3: Process JSON Entities (47 entries)
- Parse map structure
- Extract hex relationships
- Build world topology

### Phase 4: Handle Other Content (1,185 entries)
- Identify patterns
- Extract any additional data

## SeaORM Models Needed

```rust
// Primary content table
pub struct Ref {
    pub uuid: String,      // Primary key
    pub value: String,     // Name/title
    pub details: String,   // Description
    pub ref_type: String,  // location|hex|faction
    pub icon: Option<String>,
    pub anchor: Option<String>,
}

// Entity wrapper table
pub struct Entity {
    pub uuid: String,      // Primary key
    pub value: String,     // Content (empty, HTML, JSON, or text)
    pub content_type: ContentType, // Derived field
}

// Relationship mapping
pub struct EntityRefLink {
    pub entity_uuid: String,
    pub ref_uuid: String,
    pub link_type: String, // embedded|reference|map
}

// Map structure
pub struct MapHex {
    pub x: i32,
    pub y: i32,
    pub hex_type: String,
    pub uuid: String,      // Links to Ref
    pub feature: Option<String>,
    pub rivers: Vec<i32>,
}

// Rich content entities
pub struct Settlement {
    pub uuid: String,
    pub name: String,
    pub shops: Vec<String>, // UUIDs to shop Refs
    pub npcs: Vec<String>,  // UUIDs to NPC Refs
}
```

## Why Previous Analysis Failed

We focused on HTML patterns (only 1,013 entities) and missed:
- 68,556 empty entities that define the game structure
- 47 JSON entities containing the world map
- 1,185 other entities with additional content
- The Refs table being the PRIMARY data source

## Next Steps

1. **Extract Refs table completely** - This is the master data
2. **Parse JSON map entities** - Build world topology
3. **Process HTML with UUID mapping** - Create relationships
4. **Generate SeaORM models** - Cover ALL data types
5. **Build import pipeline** - Handle 100% of content

Total content to import:
- 1,570 Refs (actual game content)
- 1,013 HTML entities (rich descriptions)
- 47 JSON entities (world structure)
- 1,185 other entities (additional content)
- = **3,815 entities with actual content** (not 2,198!)
