# Hex Map Translation for Scripted Levels

## Your Task
After ALL levels are scripted, translate them to hex-based spatial representations using the Hexx crate.

## Critical Context
The hex maps are NOT procedurally generated - they're precise spatial translations of the scripted emotional beats.

## Dread-Based Map Evolution

### Stage 0: Peace (Beautiful World)
- Bright, colorful hex tiles
- Clear paths between locations
- Helpful landmarks
- Safe zones abundant

### Stage 1: Unease (Something Wrong)
- Subtle color desaturation
- Paths become ambiguous
- Landmarks shift slightly
- Safe zones reduce

### Stage 2: Dread (Horror Undeniable)
- Swamp corruption spreading
- Paths loop impossibly
- Landmarks become threats
- No safe zones

### Stage 3: Terror (Reality Breaking)
- Hex grid itself distorts
- Non-Euclidean connections
- Landmarks are lies
- Movement becomes nightmare

### Stage 4: Horror (Truth Revealed)
- First-person transition
- Hex grid collapses
- Dragon IS the world
- No escape possible

## Key Hex Mechanics

### Philosophy-Based Navigation
- **Strength**: Direct paths through obstacles
- **Harmony**: Hidden peaceful routes
- **Light**: Blessed paths appear
- **Dark**: Void shortcuts open

### Companion Effects on Maps
- Einar: Reveals tactical positions
- Mira: Shows resource locations
- Sorin: Academic shortcuts
- Tamara: Safe havens
- Alone: Full danger visible

## Technical Requirements
1. Use Hexx for hex grid logic
2. Each hex has 5 dread states
3. Transitions must be seamless
4. Philosophy paths actually different routes
5. Companion presence changes available hexes

## Asset Mapping
From the 1.7GB available:
- `textures/medieval/` - Peace stage base
- `textures/horror/` - Corruption overlays
- `models/medieval/village/` - Starting areas
- `models/horror/characters/` - Enemy positions
- `sprites/` - UI elements for hex selection

## Your Deliverables
1. Hex coordinates for EVERY scripted scene
2. Dread-based tile transformation rules
3. Philosophy-specific pathfinding
4. Companion navigation modifiers
5. Precise hex-to-level trigger mapping
