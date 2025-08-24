# 3D Labyrinth Generation - DOOM-Style Horror

## CORE CONCEPT
The transition from hex-based overworld to 3D DOOM-style labyrinths is THE key differentiator. These aren't just dungeons - they're increasingly horrifying architectural nightmares built from CC0 3D models.

## AVAILABLE 3D ASSETS (551MB in models/)

### Medieval Architecture (Use for Early Labyrinths)
From `models/medieval/` (85MB):
- Castle walls and corridors
- Dungeon cells with bars
- Throne rooms for boss arenas
- Towers with spiral staircases
- Bridges over chasms
- Gates and portcullises
- Torch holders and braziers

### Horror Elements (Progressive Corruption)
From `models/horror/` (112MB):
- Character models for enemies
- Twisted architectural elements
- Organic growth overlays
- Bone and flesh structures
- Void corruption effects

### Environmental Pieces
From `models/misc/` (170MB):
- Rocks and cave formations
- Trees growing through walls
- Water features (corrupted later)
- Atmospheric elements

## LABYRINTH PROGRESSION BY DREAD LEVEL

### Peace (Levels 1-12): Tutorial Dungeons
**Visual Style**: Clean medieval architecture
**Layout**: Simple linear paths with clear landmarks
**Mechanics**: 
- Basic key/door progression
- Well-lit with torches
- Minimap always accurate
- Safe rooms with save points

**CC0 Model Usage**:
- Clean castle corridors
- Functional dungeon cells
- Working mechanisms (doors, levers)
- Clear architectural logic

### Unease (Levels 13-24): First Real Dungeons
**Visual Style**: Medieval with subtle wrongness
**Layout**: Branching paths, some dead ends
**Mechanics**:
- Multiple keys, color-coded doors
- Some areas require torch
- Minimap occasionally glitches
- Safe rooms less frequent

**CC0 Model Usage**:
- Walls slightly off-angle
- Shadows don't match light sources
- Doors that lead nowhere
- Stairs that skip floors

### Dread (Levels 25-36): Complex Labyrinths
**Visual Style**: Medieval/horror blend
**Layout**: Multi-level with vertical progression
**Mechanics**:
- Sound-based navigation sections
- Darkness encroaches on light
- Minimap shows impossible geometry
- Safe rooms may be traps

**CC0 Model Usage**:
- Horror models blend with medieval
- Organic growth on walls
- Architecture defies physics
- Rooms bigger inside than outside

### Terror (Levels 37-48): Nightmare Dungeons
**Visual Style**: Horror overwhelms medieval
**Layout**: Non-Euclidean connections
**Mechanics**:
- First-person sections (no map)
- Sound is primary navigation
- Light hurts as much as dark
- No safe rooms

**CC0 Model Usage**:
- Flesh and bone architecture
- Breathing walls
- Gravity-defying structures
- Rooms that weren't there before

### Despair-Madness (Levels 49-60): The Dragon's Labyrinth
**Visual Style**: Reality optional
**Layout**: Procedurally shifting
**Mechanics**:
- DOOM-style but geometry shifts
- Dragon hunts you throughout
- Sanity affects what you see
- Death may not reset properly

**CC0 Model Usage**:
- All models simultaneously
- Past/present/future overlap
- Your childhood home appears
- Dragon IS the architecture

### Void (Levels 121-180): Anti-Labyrinths
**Visual Style**: Absence and presence
**Layout**: You create the path
**Mechanics**:
- Walking creates floor
- Looking creates walls
- Thinking creates enemies
- Forgetting erases progress

**CC0 Model Usage**:
- Models decay as you watch
- Textures slide off geometry
- Polygons rebel against form
- Architecture refuses existence

## MECHANICAL VARIETY IN 3D SECTIONS

### Movement Evolution
1. **Walk** (early): Standard DOOM movement
2. **Run** (mid): Stamina management
3. **Climb** (late): Vertical traversal
4. **Float** (void): Gravity optional
5. **Phase** (final): Through walls

### Combat in Labyrinths
- **Melee**: Close quarters, positioning crucial
- **Ranged**: Projectiles with physics
- **Magic**: Affects architecture itself
- **Stealth**: Darkness as ally/enemy
- **Psychological**: Fighting what isn't there

### Environmental Interactions
- **Doors**: Keys → Puzzles → Faith
- **Levers**: Mechanical → Organic → Mental
- **Pressure Plates**: Weight → Sound → Thought
- **Torches**: Light → Corruption → Memory
- **Mirrors**: Reflection → Truth → Multiplication

## BOSS ARENAS IN 3D

### Arena Types (Not Overworld)
1. **Throne Room**: Circular, pillars for cover
2. **Pit**: Vertical fight, platforming required
3. **Bridge**: Linear with knockback danger
4. **Chamber**: Phases change room geometry
5. **Void**: No floor, create as you fight

### Boss Mechanics Using 3D
- **Phase 1**: Use room geometry for advantage
- **Phase 2**: Boss destroys helpful geometry
- **Phase 3**: Room itself becomes hostile
- **Phase 4**: Geometry rebels against physics
- **Phase 5**: You ARE the room

## TECHNICAL IMPLEMENTATION WITH CC0 MODELS

### Model Pipeline
```
Raw CC0 Model → Bevy Asset → Dread Variants → Runtime Selection
```

### Corruption Stages Per Model
1. **Clean**: Original CC0 model
2. **Tainted**: Subtle texture changes
3. **Corrupted**: Geometry distortion
4. **Consumed**: Horror overlay
5. **Void-touched**: Barely recognizable

### Performance Optimization
- **LOD System**: Distance-based detail
- **Occlusion Culling**: Don't render unseen
- **Texture Atlasing**: Combine materials
- **Instance Rendering**: Repeat elements
- **Dynamic Loading**: Stream sections

## PLAYER EXPERIENCE GOALS

### Early Game (Levels 1-36)
"This is a solid dungeon crawler with great atmosphere"

### Mid Game (Levels 37-96)
"Wait, did that corridor just move?"

### Late Game (Levels 97-156)
"I don't trust anything anymore"

### End Game (Levels 157-180)
"Reality is a suggestion"

## KEY DIFFERENTIATORS

### Why This Works
1. **Variety**: Hex overworld → 3D dungeons prevents fatigue
2. **Horror**: Architecture as enemy is uniquely terrifying
3. **Progression**: Each labyrinth teaches and subverts
4. **Resources**: 551MB of models to build with
5. **Memorable**: Set pieces players won't forget

### What Makes It Special
- **Not procedural**: Crafted horror experiences
- **Not static**: Architecture responds to dread
- **Not safe**: Nowhere is truly safe
- **Not fair**: Horror doesn't play fair
- **Not over**: Even death continues story

## SONNET GENERATION REQUIREMENTS

When generating these labyrinths:
1. Use specific CC0 model names from assets
2. Describe exact room connections
3. Define lighting for each section
4. Specify sound design elements
5. Detail corruption progression
6. Map to emotional journey
7. Ensure mechanical variety
8. Create memorable set pieces
9. Support all philosophy paths
10. Account for companion presence

Each labyrinth should feel like a complete DOOM mod that happens to be inside an RPG, getting progressively more horrifying until reality itself becomes unreliable.
