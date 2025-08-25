# Complete HexRoll Data Extraction Plan

## THREE DISTINCT GAMEPLAY SYSTEMS DISCOVERED

### 1. OVERWORLD HEX MAP (Tile-by-tile placement)
- **47 JSON entities** with hex grid coordinates
- Each hex has UUID linking to Refs table
- Terrain types: JungleHex, DesertHex, etc.
- Rivers, features, and connections

### 2. SETTLEMENTS (Isolated with specific scripting)
- **351 Total Settlements**:
  - 163 Cities (major population centers)
  - 151 Villages (rural communities)  
  - 37 Towns (mid-size settlements)
  - 4 Generic Settlements
- Each has shops, NPCs, inns, taverns
- HTML entities with rich descriptions
- Embedded UUID references to inhabitants

### 3. DUNGEON CRAWL MODE (Room-by-room exploration)
- **1,569 Total Dungeons/Caves**:
  - 990 Dungeons (multi-room complexes)
  - 302 Caves (natural formations)
  - 127 Lairs (monster homes)
  - 18 Tombs (ancient burial sites)
  - 15 Crypts (undead domains)
  - 9 Temples (religious sites)
- **Coordinate System**: x,y,zoom for room navigation
- **Area Numbers**: "Cave area #1", "Dungeon area #5", etc.
- **Named Dungeons**: "Hideout of the Corrupted Order", "Tomb of the Cursed Pits"

## Data Extraction Strategy

### Phase 1: Extract Overworld Map
```sql
-- Get all map JSON entities
SELECT * FROM Entities WHERE value LIKE '{"map":%'
```
- Parse hex grid structure
- Extract terrain types
- Map UUID connections to Refs

### Phase 2: Extract Settlements
```sql
-- Get all settlement HTML entities
SELECT * FROM Entities 
WHERE value LIKE '%Village%' 
   OR value LIKE '%City%'
   OR value LIKE '%Town%'
```
- Parse HTML structure
- Extract shop listings
- Map NPC references
- Capture scripted events

### Phase 3: Extract Dungeon Crawl Maps
```sql
-- Get all dungeon/cave entities
SELECT * FROM Entities 
WHERE value LIKE '%Dungeon area%'
   OR value LIKE '%Cave area%'
   OR value LIKE '%Cave Room%'
   OR value LIKE '%Tomb%'
   OR value LIKE '%Crypt%'
```
- Parse room coordinates (x,y,zoom)
- Extract area numbers and names
- Map dungeon topology
- Capture room descriptions

### Phase 4: Extract Master Content (Refs)
```sql
-- Get all referenced content
SELECT * FROM Refs
```
- 920 Locations (NPCs, shops, places)
- 645 Hexes (terrain definitions)
- 5 Factions (groups/organizations)

## SeaORM Models Required

```rust
// Overworld hex map
pub struct HexTile {
    pub x: i32,
    pub y: i32,
    pub tile_type: String,      // JungleHex, DesertHex, etc.
    pub uuid: String,            // Links to Refs
    pub feature: Option<String>,
    pub rivers: Vec<i32>,
}

// Settlement model
pub struct Settlement {
    pub uuid: String,
    pub name: String,
    pub settlement_type: SettlementType, // City, Village, Town
    pub population: Option<i32>,
    pub shops: Vec<Shop>,
    pub npcs: Vec<String>,       // UUID refs to NPCs
    pub inns: Vec<Inn>,
    pub description_html: String,
}

// Dungeon crawl model
pub struct DungeonComplex {
    pub uuid: String,
    pub name: String,            // "Hideout of the Corrupted Order"
    pub dungeon_type: DungeonType, // Dungeon, Cave, Lair, Tomb, etc.
    pub rooms: Vec<DungeonRoom>,
}

pub struct DungeonRoom {
    pub area_number: i32,        // Cave area #1, #2, etc.
    pub x: i32,
    pub y: i32,
    pub zoom: i32,
    pub room_type: String,       // Cave Hall, Cave Room, Grotto, etc.
    pub description: String,
    pub connections: Vec<String>, // UUIDs to other rooms
}

// Master content
pub struct WorldRef {
    pub uuid: String,
    pub value: String,           // Name
    pub details: String,         // Description
    pub ref_type: RefType,       // location, hex, faction
    pub icon: Option<String>,
    pub anchor: Option<String>,
}

enum SettlementType {
    City,
    Village,
    Town,
}

enum DungeonType {
    Dungeon,
    Cave,
    Lair,
    Tomb,
    Crypt,
    Temple,
}

enum RefType {
    Location,
    Hex,
    Faction,
}
```

## Complete Content Summary

### Total Extractable Content:
- **Overworld**: 47 map entities + 645 hex definitions
- **Settlements**: 351 total (Cities, Villages, Towns)
- **Dungeons**: 1,569 total dungeon complexes
- **NPCs/Locations**: 920 characters and places
- **Factions**: 5 major groups

### Three Gameplay Modes:
1. **Exploration Mode**: Overworld hex map navigation
2. **Settlement Mode**: City/village interaction with shops/NPCs
3. **Crawl Mode**: Room-by-room dungeon exploration

## This gives Dragon's Labyrinth:
- A complete overworld to explore
- Hundreds of settlements to visit
- Over 1,500 dungeons to crawl
- Nearly 1,000 NPCs to interact with
- Rich, interconnected world content
