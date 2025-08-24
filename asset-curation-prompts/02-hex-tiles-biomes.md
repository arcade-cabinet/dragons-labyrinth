# Asset Curation Task: Hex Tiles & Biomes

## Context
Dragon's Labyrinth uses a hex-based world that corrupts over time. We need tiles that can represent different biomes and show visual degradation across 5 dread levels.

## Your Task
Find and evaluate textures and models suitable for hex tiles. We use the `hexx` crate with flat-top orientation, 32x32 unit tiles.

## Assets to Review
Paths:
- `ordered/assets/library/textures/nature/`
- `ordered/assets/library/textures/floors/`
- `ordered/assets/library/models/nature/`
- `ordered/assets/library/textures/medieval/village/`
- `ordered/assets/library/textures/corrupted/` (if exists)

## Biomes Needed (from design bible)
1. **Meadow** - Starting area, peaceful
2. **Dark Forest** - Dense trees, limited visibility
3. **Wetlands** - Swamps, water hazards
4. **Highlands** - Rocky, elevation changes
5. **Wasteland** - Corrupted areas
6. **Village** - Populated areas
7. **Ruins** - Abandoned structures
8. **Boss Arena** - Special encounter areas

## Corruption Stages
Each biome needs 5 visual variants:
- **Stage 0**: Pristine, colorful
- **Stage 1**: Slightly faded colors
- **Stage 2**: Visible decay, cracks
- **Stage 3**: Heavy corruption, organic growth
- **Stage 4**: Nightmare fuel, reality breaking

## Selection Criteria
- Must tile seamlessly for hex grids
- Clear visual distinction between biomes
- Can be darkened/corrupted programmatically
- Performance-friendly (mobile target)

## Output Format
```rust
// Biome: [name]
// Base Asset: [filename]
// Tiling: [seamless/needs-work]

pub struct [BiomeName]Tiles {
    pub base_texture: Handle<Image>,
    pub normal_map: Option<Handle<Image>>,
    pub corruption_overlay: Option<Handle<Image>>,
}

impl [BiomeName]Tiles {
    pub fn apply_to_hex(
        hex: Hex,
        mesh: &mut Mesh,
        dread_level: u8,
    ) {
        // Texture coordinates for hex
        // Apply corruption based on dread
    }
}
```

## Integration Notes
- Hexes are rendered using bevy_ecs_tilemap
- Each hex is 32x32 world units
- Y-axis is elevation (for 3D terrain)
- Corruption shaders will be applied in real-time

Move selected assets to: `crates/game-engine/assets/textures/biomes/`
