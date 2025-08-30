# OpenRPG → Dragon's Labyrinth Adaptation Plan

## Systems Worth Adapting ✅

### 1. Combat System (src/combat/) → Horror Combat System
**Value**: Sophisticated turn-based combat with dialogic integration, screen transitions
**Adaptation Strategy**:
- Keep: Turn queue, battler system, UI framework, dialogic integration
- Modify: Invert power progression (getting weaker), add psychological damage
- Add: Companion trauma from combat, dread-based combat modifiers, moral choice consequences

### 2. Gameboard System (src/field/gameboard/) → Hex Grid System  
**Value**: Excellent grid movement, pathfinding, dynamic collision layers
**Adaptation Strategy**:
- Keep: Pathfinding algorithms, layer management, coordinate conversion
- Replace: Square grid → Hex grid using hexagon_tilemaplayer addon
- Add: Cube coordinate system, distance-based corruption, infinite chunk loading

### 3. Field System (src/field/) → World Exploration System
**Value**: Player controller, camera system, cutscene integration
**Adaptation Strategy**: 
- Keep: Player state management, camera following, interaction system
- Modify: Movement for hex directions, perspective switching capability
- Add: Horror atmosphere effects, companion following, dread progression triggers

### 4. Common Systems (src/common/) → Core Horror Systems
**Value**: Player autoload, music system, screen transitions, inventory
**Adaptation Strategy**:
- Keep: Player state pattern, music framework, transition system
- Modify: Music for horror atmosphere, transitions for first-person shift
- Add: Curse progression, companion management, horror inventory (sentimental items)

### 5. Gamepiece System → Companion System
**Value**: Character movement, interaction, registry management
**Adaptation Strategy**:
- Keep: Movement mechanics, interaction patterns, character registry
- Modify: AI for companion following, trauma response behaviors
- Add: Psychology integration, therapy mechanics, abandonment behaviors

## Adaptation Implementation Plan

### Phase 1: Core System Migration
1. **Copy valuable OpenRPG systems** to new organized structure
2. **Rename classes/files** to reflect horror RPG purpose
3. **Preserve working mechanics** while updating for horror themes
4. **Integrate with our existing horror scripts** (DreadProgression, CompanionPsychology)

### Phase 2: Horror Integration
1. **Add dread progression hooks** to all adapted systems
2. **Replace square grid** with hex grid using hexagon_tilemaplayer
3. **Integrate companion psychology** with gamepiece/combat systems
4. **Add database integration** with godot-sqlite for our 50+ table system

### Phase 3: Horror-Specific Features
1. **Perspective switching** system for first-person labyrinth
2. **Dragon stalking AI** using adapted AI systems
3. **Audio horror integration** with proximity detection
4. **Corruption visual effects** throughout all systems

## Specific Adaptations

### Combat → Horror Combat
```gdscript
# Adapt OpenRPG combat for horror
extends OpenRPGCombatBase
class_name HorrorCombatSystem

# Keep excellent turn queue and UI
# Add horror modifications:
func calculate_horror_damage_modifiers():
    var dread_multiplier = DreadProgression.get_dread_multiplier("combat")
    var companion_stress = CompanionPsychology.get_combat_effectiveness()
    return dread_multiplier * companion_stress
```

### Gameboard → HexGrid  
```gdscript
# Adapt OpenRPG grid for hex system
extends OpenRPGGameboard  
class_name HexWorldGameboard

# Keep excellent pathfinding
# Replace coordinate system:
func cell_to_pixel(hex_coords: Vector3i) -> Vector2:
    # Convert cube coordinates to pixel using hex math
    return HexagonTileMapLayer.hex_to_pixel(hex_coords)
```

### Field → WorldExploration
```gdscript
# Adapt OpenRPG field for horror exploration
extends OpenRPGField
class_name HorrorWorldExploration

# Keep player controller patterns
# Add horror atmosphere:
func _ready():
    super._ready()
    DreadProgression.dread_level_changed.connect(_on_dread_changed)
```

## Directory Structure Post-Adaptation

```
systems/
├── horror/                 # DreadProgression + adapted horror mechanics
├── companions/             # CompanionPsychology + adapted gamepiece/companion_system  
├── world/                  # HexTileData + adapted gameboard → hex grid
├── database/               # AssetCatalog + SQLite integration
├── ui/                     # Adapted combat UI + inventory + quest systems
└── audio/                  # Adapted music system for horror atmosphere

scenes/
├── overworld/              # Main hex exploration scene
├── labyrinth/              # First-person horror sequences  
└── ui/                     # Combat/dialogue/inventory scenes

src/ (adapted foundation)/
├── core/                   # Adapted common systems (player, inventory, etc.)
├── controllers/            # Adapted field controllers for hex movement
└── framework/              # Adapted gameboard framework for hex support
```

## Value Preservation Strategy

### Keep and Adapt:
- ✅ Turn-based combat framework → Horror combat with psychological damage
- ✅ Grid movement system → Hex movement with cube coordinates  
- ✅ Player controller pattern → Horror exploration controller
- ✅ Dialogic integration → Companion psychology dialogue
- ✅ Music/transition systems → Horror atmosphere control
- ✅ Inventory system → Sentimental item collection/forge system

### Remove:
- ❌ Cute JRPG art assets and themes
- ❌ Power progression mechanics (leveling up, getting stronger)
- ❌ Traditional party management
- ❌ Generic quest completion rewards

## Integration Points

### With Our Existing Horror Scripts:
- **DreadProgression** becomes master orchestrator of adapted systems
- **CompanionPsychology** integrates with adapted companion/gamepiece systems
- **HexTileData** works with adapted gameboard layer system
- **Database systems** load content into adapted inventory/quest systems

### With Critical Addons:
- **hexagon_tilemaplayer** replaces square grid in adapted gameboard
- **godot-sqlite** provides data for adapted systems
- **dialogic** enhanced with companion psychology
- **limboai** for dragon stalking using adapted AI patterns

This approach preserves the excellent engineering work in OpenRPG while transforming it for our horror RPG vision.
