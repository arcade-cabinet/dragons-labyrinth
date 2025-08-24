# Asset Curation and Integration

## Your Task
You have 1.7GB of CC0 assets in `crates/game-content-static/assets/`. Analyze EVERY asset and determine how it serves the emotional journey.

## Asset Categories to Review

### Models (551MB)
- `models/medieval/` (85MB) - Peace stage baseline
- `models/horror/` (112MB) - Terror/Horror elements
- `models/misc/` (170MB) - Environmental pieces
- `models/characters/` (57MB) - NPCs and companions

**Critical Questions:**
- Which models work for each dread stage?
- How to corrupt medieval models for horror?
- What's missing for boss encounters?
- Can we repurpose fantasy models?

### Textures (354MB)
- `textures/medieval/` (27MB) - Base world textures
- `textures/horror/` (1.1MB) - Corruption overlays
- `textures/architecture/` (5.2MB) - Building materials
- `textures/nature/` (2.6MB) - Environmental

**Texture Evolution Plan:**
- Stage 0: Full color medieval textures
- Stage 1: 20% desaturation
- Stage 2: 50% corruption blend
- Stage 3: 80% horror overlay
- Stage 4: Complete nightmare

### Audio (2MB)
- Limited audio assets available
- Need Freesound integration for:
  - Dread-responsive ambience
  - Companion voice lines
  - Horror proximity effects
  - Philosophy-specific sounds

### Sprites (9.2MB)
- UI elements for different dread levels
- Character portraits that degrade
- Inventory items that corrupt

## ECS Integration Requirements

For EACH asset, generate the Bevy ECS code:

```rust
// Example for a medieval house model
#[derive(Component)]
pub struct MedievalHouse {
    pub dread_variants: [Handle<Scene>; 5],
    pub corruption_level: f32,
    pub philosophy_modifiers: PhilosophyModifiers,
}

impl MedievalHouse {
    pub fn spawn(
        commands: &mut Commands,
        asset_server: &AssetServer,
        position: Vec3,
        dread: DreadLevel,
    ) {
        // Implementation that loads correct variant
    }
}
```

## Missing Assets to Generate

### Priority 1: Core Narrative
- Dragon (all stages)
- Void rifts
- Forge locations
- Sentimental items

### Priority 2: Companion Specifics
- Einar breaking animations
- Mira departure scene
- Sorin betrayal transformation
- Tamara corruption

### Priority 3: Philosophy Visuals
- Strength path effects
- Harmony auras
- Light miracles
- Dark consumption

## Your Deliverables
1. Complete asset inventory with purpose mapping
2. ECS component definitions for ALL assets
3. Bevy resource loading systems
4. Dread-based asset swapping logic
5. List of critical missing assets
6. Freesound queries for audio gaps
