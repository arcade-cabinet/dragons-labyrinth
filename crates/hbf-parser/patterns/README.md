# HBF Parser Pattern Definitions

This directory contains pattern definitions extracted from analyzing the HBF export HTML files.

## Pattern Categories Identified

### 1. Dungeon Rooms
- **Corridors**: Simple passages with doorways and descriptions
- **Crypts**: Complex rooms with multiple doorways, special features, traps, teleportals
- **Chambers**: Varied room types with environmental descriptions

### 2. Settlements & Buildings  
- **Taverns/Inns**: Named establishments with regional weather tables
- **Shops**: Commercial buildings with services
- **Temples**: Religious locations

### 3. Wilderness Hexes
- **Terrain Types**: Jungle, forest, plains, mountains
- **Weather Systems**: Regional weather tables with seasonal variations
- **Special Features**: Waterfalls, ruins, bridges

### 4. NPCs & Characters
- **Stat Blocks**: Character statistics and abilities
- **Personalities**: Behavioral traits and dialogue hints
- **Relationships**: Connections between NPCs

### 5. Items & Treasure
- **Weapons & Armor**: Combat equipment with stats
- **Magic Items**: Enchanted equipment with special properties
- **Treasure Hoards**: Collections of valuables

## File Format

Patterns are stored as RON (Rusty Object Notation) files for easy Rust integration:
- `dungeon_rooms.ron` - Room types and layouts
- `settlements.ron` - Buildings and establishments
- `npcs.ron` - Character templates
- `items.ron` - Equipment and treasure
- `weather.ron` - Environmental systems
- `encounters.ron` - Combat and event tables

## HTML Structure Analysis

### Common Elements
- `<h4 id="title">` - Entity title and breadcrumbs
- `<h5>` - Section headers (Doorways, Description, etc.)
- `<blockquote>` - Main descriptive text
- `<ul><li>` - Feature lists and special properties
- `<table>` - Weather tables and stat blocks
- `<a href="/sandbox/nTR8nJOW/location/">` - Cross-references

### Coordinate System
- Hex coordinates: `hex="uuid"` with `x="num" y="num"`
- Area references: `Area # N` for dungeon rooms
- Location links: `/location/uuid` format

### Interactive Elements
- Dice rolls: `<a class="btn-spawn-dice" data-dice='2d6'>`
- Reroll buttons: `<a class="btn-icon" onclick="javascript:window.app.reroll('uuid')"`
- Map coordinates: `<a class="map-coords" hex="uuid" x="num" y="num">`
