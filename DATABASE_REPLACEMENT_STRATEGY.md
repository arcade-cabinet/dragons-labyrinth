# OpenRPG → Database Replacement Strategy

## Load Points Analysis ✅

### Keep As-Is (UI/Scene Loading)
These are UI components and structural elements that should remain as preloaded scenes:

**UI Components (Keep)**:
- `preload("res://ui/inventory/ui_inventory_item.tscn")` → UI template
- `preload("res://combat/ui/cursors/ui_menu_cursor.tscn")` → UI cursor
- `@export var player_controller: PackedScene` → Player controller template  
- `@export var animation_scene: PackedScene` → Animation template
- `@export var action_menu_scene: PackedScene` → Menu UI template

**Reasoning**: These are reusable UI templates, not data. They provide structure/behavior.

### Replace With Database (Data Loading)
These load character data, stats, actions, and content that should come from our 50+ table database:

**Character/Combat Data (Replace)**:
```gdscript
# Current OpenRPG pattern (static .tres loading)
@export var stats: BattlerStats = null  # ← Replace with database query
@export var actions: Array[BattlerAction]  # ← Replace with database query 
@export var ai_scene: PackedScene  # ← Replace with AI behavior from database

# Our database replacement (godot-sqlite)
func load_character_from_database(character_id: String) -> void:
    var db = SQLite.new()
    db.path = "res://game.db"
    db.open_db()
    
    # Load from our sprites/characters tables
    var character_data = db.select_rows("character_records", "character_id = '%s'" % character_id, ["*"])
    var stats_data = db.select_rows("companion_profiles", "character_id = '%s'" % character_id, ["*"])
    var actions_data = db.select_rows("encounter_records", "character_id = '%s'" % character_id, ["*"])
    
    # Create stats from database data instead of .tres file
    stats = create_stats_from_data(stats_data[0])
    actions = create_actions_from_data(actions_data)
```

## Database Mapping Strategy

### Our 50+ Tables → OpenRPG Systems

**Character/Combat Systems**:
- `character_records` (sprites subpackage) → `Battler.stats` 
- `companion_profiles` (psychology subpackage) → Companion behavior
- `monster_records` (sprites subpackage) → Enemy stats
- `encounter_records` (encounters subpackage) → `BattlerAction` arrays
- `combat_scenarios` (encounters subpackage) → Combat setups

**World/Exploration Systems**:
- `hex_tiles` (maps subpackage) → World tile data
- `entity_records` (entities subpackage) → NPCs, locations, items
- `narrative_seeds` (seeds subpackage) → Quest/story content
- `emotional_seeds` (seeds subpackage) → Atmosphere/mood data

**Asset Systems**:
- `asset_records` (assets subpackage) → Generated sprites/textures
- `asset_blob_storage` (assets subpackage) → Actual image data
- `sprite_sheets` (assets subpackage) → Animation frames

## Implementation Strategy

### Phase 1: Identify All Static Data Load Points
```bash
# Find all .tres resource references
find . -name "*.gd" -exec grep -l "\.tres\|BattlerStats\|BattlerAction" {} \;

# Find export vars that load data (not UI)
find . -name "*.gd" -exec grep -l "@export var.*\(Stats\|Action\|Data\)" {} \;
```

### Phase 2: Create Database Loaders
For each data type, create database loader functions:

```gdscript
# systems/database/DataLoader.gd
class_name DatabaseLoader
extends Node

static var db: SQLite = null

static func initialize_database():
    if not db:
        db = SQLite.new()
        db.path = "res://dragon_labyrinth.db"
        db.open_db()

static func load_character_stats(character_id: String) -> BattlerStats:
    initialize_database()
    var rows = db.select_rows("character_records", "character_id = '%s'" % character_id, ["*"])
    if rows.size() > 0:
        return create_battler_stats_from_data(rows[0])
    return null

static func load_character_actions(character_id: String) -> Array[BattlerAction]:
    initialize_database()
    var rows = db.select_rows("encounter_records", "character_id = '%s'" % character_id, ["*"])
    var actions: Array[BattlerAction] = []
    for row in rows:
        actions.append(create_battler_action_from_data(row))
    return actions

static func load_hex_tile_data(hex_coords: Vector3i) -> HexTileData:
    initialize_database()
    var coord_string = "%d,%d,%d" % [hex_coords.x, hex_coords.y, hex_coords.z]
    var rows = db.select_rows("hex_tiles", "coordinate = '%s'" % coord_string, ["*"])
    if rows.size() > 0:
        return create_hex_tile_from_data(rows[0])
    return null

static func load_companion_psychology(companion_id: String) -> Dictionary:
    initialize_database()
    var rows = db.select_rows("companion_profiles", "character_id = '%s'" % companion_id, ["*"])
    if rows.size() > 0:
        return rows[0]
    return {}
```

### Phase 3: Replace Load Points
Replace static loading with database queries:

```gdscript
# OLD OpenRPG pattern:
@export var stats: BattlerStats = load("res://characters/hero_stats.tres")
@export var actions: Array[BattlerAction] = [
    load("res://actions/sword_attack.tres"),
    load("res://actions/heal_spell.tres")
]

# NEW Database pattern:
var stats: BattlerStats
var actions: Array[BattlerAction]

func _ready():
    # Load from database instead of static files
    stats = DatabaseLoader.load_character_stats(character_id)
    actions = DatabaseLoader.load_character_actions(character_id)
```

### Phase 4: Remove Static Content
After database integration is working:
```bash
# Remove obsolete .tres data files (keep .tscn UI templates)
find . -name "*.tres" -type f -delete

# Remove old resource directories
rm -rf resources/characters/
rm -rf resources/actions/ 
rm -rf resources/items/
# Keep resources/ui/ - those are UI templates
```

## Horror RPG Integration Benefits

### Dynamic Content Generation
Instead of static .tres files, we get:
- **AI-generated companions** with unique psychology from our psychology subpackage
- **Procedural encounters** from encounters subpackage with cross-system context
- **Dynamic hex tiles** from maps subpackage with corruption progression
- **Evolving character stats** based on horror progression and companion psychology

### Cross-System Enhancement
Our 50+ table database provides rich context:
- **Characters** get visual design from assets subpackage + psychology from psychology subpackage
- **Encounters** use world context + character relationships + environmental factors
- **Items** have sentimental value based on story progression and companion attachment
- **Locations** have atmospheric description enhanced by seeds subpackage emotional data

### Performance Benefits
- **Memory efficient**: Load only what's needed vs keeping all .tres in memory
- **Dynamic updates**: Change character stats/behavior without reloading scenes
- **Infinite content**: Generate new content on-demand from database patterns
- **Save integration**: Player choices modify database directly

## Implementation Priority

### High Priority (Core Game Loop)
1. **Character/Companion loading** - Replace BattlerStats with database
2. **Hex tile loading** - Replace static world with database-driven tiles
3. **Combat actions** - Replace BattlerAction arrays with database queries
4. **Horror progression** - Integrate with our existing DreadProgression system

### Medium Priority (Content Enhancement)  
1. **Quest/dialogue content** - Load from narrative_seeds and emotional_seeds
2. **Asset rendering** - Load AI-generated sprites from asset_blob_storage
3. **Audio integration** - Load procedural audio cues based on horror progression

### Low Priority (Polish)
1. **UI customization** - Horror-themed UI elements from database
2. **Performance optimization** - Caching strategies for frequent queries
3. **Debug tools** - Database content browser for development

This strategy preserves OpenRPG's excellent UI/scene architecture while replacing their static content system with our sophisticated 50+ table database for infinite, AI-enhanced horror content generation.
