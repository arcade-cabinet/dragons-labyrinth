# Dragon's Labyrinth: Universal Style Guide

## Visual Philosophy

**Horror Through Beauty**: Start with genuinely beautiful, inviting environments that gradually reveal underlying corruption. The contrast between initial beauty and growing darkness amplifies the horror experience.

**Adaptive Asset Generation**: Meta-prompts specify exactly what assets they need - size, style, format - rather than conforming to rigid technical constraints. Let creative vision drive technical requirements.

**Emotional Progression**: Visual style evolves dramatically across the five dread bands (0-4), with each ring having distinct aesthetic characteristics that reinforce the narrative journey.

## Band-Specific Visual Themes

### Band 0-1: Peace → Unease (Heartlands)
**Color Palette**: 
- Primary: Soft greens (#7CB342, #8BC34A), warm earth tones (#8D6E63, #A1887F)
- Accents: Golden sunlight (#FFD54F), clear blue skies (#42A5F5)
- Corruption hints: Subtle brown patches (#5D4037), faded yellows (#FFF176)

**Visual Characteristics**:
- Lush, vibrant landscapes with abundant life
- Comfortable pastoral scenes: thatched cottages, flower gardens, stone wells
- Subtle wrongness: occasional withered plants, unexplained shadows
- Lighting: Warm, golden hour lighting that feels safe and inviting

**Asset Types**:
- Rolling meadows with wildflowers (any size needed for composition)
- Ancient oak and birch forests with dappled sunlight
- Stone bridges over crystal streams
- Village buildings: mills, smithies, taverns, shrines
- NPCs: Farmers, merchants, children at play, village guards

### Band 1-2: Unease → Dread (Contested Lands)
**Color Palette**:
- Primary: Muted greens (#689F38), dusty browns (#6D4C41)
- Accents: Rust reds (#D84315), faded golds (#FF8F00)
- Corruption: Ash grays (#424242), sickly yellows (#F57F17)

**Visual Characteristics**:
- Living landscapes marred by obvious damage
- Smoke on horizons, burned patches in forests
- Refugees, damaged buildings, nervous livestock
- Lighting: Hazy, filtered sunlight suggesting distant fires

**Asset Types**:
- Partially burned forests with standing dead trees
- Damaged village buildings with makeshift repairs
- Refugee camps with patched tents and cooking fires
- Military elements: watchtowers, patrol camps, supply wagons
- NPCs: Refugees, nervous guards, traveling merchants, wounded veterans

### Band 2-3: Dread → Terror (Burning Marches)
**Color Palette**:
- Primary: Charcoal blacks (#212121), deep reds (#C62828)
- Accents: Molten oranges (#E65100), sulfur yellows (#F57C00)
- Corruption: Void purples (#4A148C), sickly greens (#2E7D32)

**Visual Characteristics**:
- Hostile, inhospitable terrain
- Active lava flows, obsidian formations, cracked earth
- Monumental ruins suggesting ancient conflicts
- Lighting: Red, flickering firelight with harsh shadows

**Asset Types**:
- Lava fields with bubbling magma pools
- Obsidian spires and jagged rock formations
- Ancient fortress ruins with massive walls
- Corrupted creatures: drake-touched beasts, flame elementals
- NPCs: Desperate survivors, cult fanatics, dragon-marked warriors

### Band 3-4: Terror → Despair → Madness (Broken Realm)
**Color Palette**:
- Primary: Cold grays (#455A64), blood reds (#B71C1C)
- Accents: Rust browns (#3E2723), poison greens (#1B5E20)
- Corruption: Void blacks (#000000), unnatural purples (#6A1B9A)

**Visual Characteristics**:
- Familiar places twisted by human cruelty
- War-torn landscapes with mass graves and gibbets
- Reality beginning to fray at the edges
- Lighting: Harsh, cold light that creates stark contrasts

**Asset Types**:
- Ruined versions of earlier pleasant locations
- Military encampments with siege equipment
- Execution grounds and mass burial sites
- Early void manifestations: reality cracks, floating debris
- NPCs: Warlords, broken survivors, cultists, void-touched humans

### Band 4-5: Madness → Void (Nightmare Expanse)
**Color Palette**:
- Primary: Void blacks (#000000), reality whites (#FFFFFF)
- Accents: Impossible colors that hurt to look at
- Corruption: Everything is corruption - reality itself is breaking

**Visual Characteristics**:
- Landscapes that defy physics and logic
- Familiar elements arranged in impossible ways
- Colors and shapes that shouldn't exist
- Lighting: Sourceless, shifting illumination from nowhere

**Asset Types**:
- Nightmare versions of all previous biomes
- Impossible architecture: stairs that climb down, doors in sky
- Void creatures: geometric horrors, living shadows, reality parasites
- Eldritch phenomena: floating cities, inverted forests, bleeding stars
- NPCs: Void-touched humans, cosmic entities, fragments of former companions

## Technical Specifications

### Flexible Asset Generation
**Size Requirements**: Meta-prompts specify exact dimensions needed
- Hex tiles: Whatever size best serves the visual composition
- UI elements: Scaled for intended use case
- Characters: Sized appropriately for context (portraits vs. full-body)
- Environmental details: Optimized for viewing distance and importance

**Format Standards**:
- **2D Assets**: PNG with transparency where needed, JPEG for solid backgrounds
- **Audio**: MP3 or OGG based on content type and platform requirements
- **3D Elements**: Reference existing inventory rather than generate new models

### Layer Composition System
**Base Layer**: Primary terrain or background
**Feature Layer**: Interactive elements, buildings, landmarks
**Overlay Layer**: Paths, UI elements, temporary effects
**Atmospheric Layer**: Weather, lighting effects, particles

Each layer generated at appropriate resolution for its visual importance and technical requirements.

## 3D Asset Inventory System

### Available 3D Modules
Meta-prompts receive a JSON inventory of pre-built 3D assets for transition scenes:

```json
{
  "environments": {
    "interiors": {
      "tavern": ["cozy_common_room", "private_dining", "cellar_storage"],
      "cottage": ["single_room", "two_room", "loft_bedroom"],
      "cave": ["natural_cavern", "worked_stone", "crystal_chamber"],
      "keep": ["great_hall", "throne_room", "dungeon", "parapet"]
    },
    "exteriors": {
      "forest": ["clearing", "dense_woods", "ancient_grove"],
      "plains": ["open_field", "hilltop", "river_crossing"],
      "mountain": ["cliff_face", "summit", "valley_view"],
      "ruins": ["temple_remains", "castle_walls", "monument_circle"]
    }
  },
  "props": {
    "furniture": ["throne", "table_long", "chair_ornate", "bed_simple"],
    "containers": ["chest_iron", "barrel_wood", "sack_grain"],
    "lighting": ["torch_wall", "candle_cluster", "brazier_iron"],
    "weapons": ["sword_display", "armor_rack", "shield_mounted"]
  },
  "characters": {
    "archetypes": ["noble", "merchant", "warrior", "peasant", "cultist"],
    "poses": ["standing", "sitting", "kneeling", "wounded", "threatening"],
    "emotions": ["confident", "fearful", "angry", "desperate", "mad"]
  }
}
```

### Transition Scene Composition
Meta-prompts specify:
1. **Environment**: Which 3D location to use
2. **Props**: What objects should be present
3. **Characters**: Who appears and in what state
4. **Atmosphere**: Lighting, weather, supernatural effects
5. **Interaction Points**: What the player can examine or interact with

## Quality Standards

### Visual Coherence
- **Consistent Art Direction**: All assets in a band share visual characteristics
- **Thematic Appropriateness**: Every asset reinforces the emotional tone
- **Technical Quality**: Clean lines, appropriate detail level, optimized file sizes
- **Narrative Support**: Visual elements support rather than distract from story

### Performance Optimization
- **Mobile Compatibility**: Assets work well on lower-end devices
- **Loading Efficiency**: Reasonable file sizes for streaming and caching
- **Runtime Performance**: 60fps target on mid-range hardware
- **Memory Management**: Efficient use of VRAM and system RAM

### Accessibility Considerations
- **Color Blindness**: Important information not conveyed through color alone
- **Visual Clarity**: High contrast for important interactive elements
- **Text Readability**: Legible fonts at various sizes
- **Audio Alternatives**: Visual cues accompany important audio information

This adaptive approach ensures that each region gets exactly the assets it needs to create its intended emotional experience, while maintaining visual coherence across the broader game world.
