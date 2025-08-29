# Asset Curation Task: Dungeons & Boss Rooms

## Context
We need to build 3D raytraced dungeon walls DOOM-style for boss encounters. These should feel claustrophobic, oppressive, and increasingly horrific.

## Your Task
Find wall segments, dungeon pieces, and architectural elements we can use to construct modular boss arenas.

## Assets to Review
Paths:
- `ordered/assets/library/models/dungeon/`
- `ordered/assets/library/models/walls/`
- `ordered/assets/library/models/architecture/`
- `ordered/assets/library/models/medieval/dungeon/`
- `ordered/assets/library/textures/walls/`
- `ordered/assets/library/textures/dungeon/`

## Dungeon Requirements
1. **Modular Wall Pieces**
   - Straight walls
   - Corner pieces
   - T-junctions
   - Door frames
   - Windows/openings

2. **Environmental Elements**
   - Pillars/columns
   - Stairs/ramps
   - Pits/chasms
   - Bridges
   - Gates/barriers

3. **Boss Arena Features**
   - Central arena space
   - Elevated platforms
   - Cover elements
   - Environmental hazards
   - Dramatic lighting points

## Horror Progression
- **Early Dungeons**: Stone walls, torch lighting
- **Mid Dungeons**: Cracks, water damage, shadows
- **Late Dungeons**: Flesh walls, organic growth, reality distortion

## Technical Requirements
- Must support runtime lighting (torches, magic, etc.)
- Collision meshes for physics
- LOD variants for performance
- Modular snapping system

## Output Format
```rust
// Dungeon Piece: [type]
// Asset: [filename]
// Dimensions: [size in units]
// Snapping: [grid size]

pub struct DungeonPiece {
    pub piece_type: DungeonPieceType,
    pub model: Handle<Scene>,
    pub collision: Handle<Mesh>,
    pub snap_points: Vec<Vec3>,
}

#[derive(Clone, Copy)]
pub enum DungeonPieceType {
    WallStraight,
    WallCorner,
    WallTJunction,
    Pillar,
    Floor,
    Ceiling,
    Stairs,
    Door,
    Gate,
}

pub struct DungeonBuilder {
    pub fn create_boss_arena(
        size: f32,
        theme: DungeonTheme,
        dread_level: u8,
    ) -> Vec<DungeonPiece> {
        // Generate arena layout
        // Apply corruption based on dread
    }
}
```

## DOOM-Style Rendering Notes
- We'll use ray marching for atmospheric fog
- Dynamic shadows from point lights
- Texture palette shifts for mood
- Particle effects for atmosphere

## Questions to Answer
1. Do we have enough modular pieces for variety?
2. Can pieces snap together cleanly?
3. Are there door/gate models for transitions?
4. Do we need to model custom pieces?

Move selected assets to: `crates/game-engine/assets/models/dungeon/`
