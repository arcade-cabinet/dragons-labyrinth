# Asset Curation Task: Horror Characters

## Context
You are analyzing CC0 assets for Dragon's Labyrinth, a horror-driven narrative RPG. The game progresses through 5 dread levels (0=Peace to 4=Madness), with visual and audio elements degrading as horror intensifies.

## Your Task
Analyze the horror character models and textures in the provided assets. For each asset:

1. **Visual Analysis** (for textures/sprites):
   - Describe what you see
   - Rate horror appropriateness (1-10)
   - Suggest dread level (0-4) where it fits
   - Note any modifications needed

2. **3D Model Analysis** (for .fbx/.obj/.glb files):
   - Describe the model structure
   - Check for proper rigging/bones
   - Identify missing materials/textures
   - Suggest use cases in gameplay

3. **Integration Recommendations**:
   - ECS component it should use
   - Bevy bundle configuration
   - Loading code snippet
   - Gameplay mechanics it enables

## Assets to Review
Path: `ordered/assets/library/models/horror/characters/`
- 105M of horror character models
- Related textures in `ordered/assets/library/textures/horror/characters/`

## Selection Criteria
We need:
- **The Dragon** - Main antagonist, should be imposing
- **Corrupted Villagers** - NPCs who've succumbed to dread
- **Shadow Creatures** - Hallucination enemies
- **Companion Corruption States** - Visual variants for companion breakdown
- **Boss Variants** - For dungeon encounters

## Output Format
For each selected asset, provide:

```rust
// Asset: [filename]
// Description: [what it is]
// Dread Level: [0-4]
// Use Case: [specific game usage]

pub struct [ComponentName] {
    pub model_path: &'static str,
    pub texture_path: &'static str,
    pub scale: Vec3,
    pub animations: Vec<String>,
}

impl [ComponentName] {
    pub fn spawn(commands: &mut Commands, asset_server: &AssetServer) {
        commands.spawn((
            SceneBundle {
                scene: asset_server.load("[path]"),
                transform: Transform::from_scale(Vec3::splat([scale])),
                ..default()
            },
            // Additional components
        ));
    }
}
```

## Questions to Answer
1. Do we have a suitable dragon model?
2. Are there enough corruption stages for visual progression?
3. Can we create companion variants from existing models?
4. What's missing that we absolutely need?

Move selected assets to: `crates/game-engine/assets/models/horror/`
