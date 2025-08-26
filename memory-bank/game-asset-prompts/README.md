# Dragon's Labyrinth Asset Prompt Library

## Organization Structure

This directory contains all DALL-E prompts for Dragon's Labyrinth, organized for consistency and maintainability.

### Universal Prompts (Apply to All Acts)
- **biome-prompts.md** - Base terrain textures (consistency-locked)
- **paths-prompts.md** - Road/bridge overlays (consistency-locked)
- **ui-prompts.md** - Interface elements by dread level
- **effects-prompts.md** - Weather, magic, corruption overlays

### Act-Specific Content (Level Ranges)
- **prompts-1-20.md** - Act 1 Early (Peace → Unease) - Bright world, welcoming features
- **prompts-21-40.md** - Act 1 Mid (Unease → Dread) - Dragon blight appears in bands
- **prompts-41-60.md** - Act 1 Late (Dread → Terror) - Approaching hellscape, molten terrain
- **prompts-61-120.md** - Act 2 (Terror → Madness) - Social corruption, political chaos
- **prompts-121-180.md** - Act 3 (Madness → Void) - Eldritch corruption, reality breakdown

### Thematic Progression

#### Levels 1-20: Clean & Welcoming
- Lush biomes, cozy taverns, friendly NPCs
- Basic monsters (bandits, wolves, goblins)
- Chess-piece style character tokens

#### Levels 21-40: Dragon Blight Bands
- Mixed corruption - some areas scorched, some still green
- Damaged features, early corrupted monsters
- Transition from hope to dread

#### Levels 41-60: Hellscape Approach
- Lava fields, dried riverbeds, charred forests
- Ruined fortresses, nightmare features
- Dragon-spawned monsters

#### Levels 61-120: Social Apocalypse
- Reuse tiles from 1-60 but with social corruption features
- War camps, execution sites, cruel overlords
- Human monsters, political horror

#### Levels 121-180: Void Corruption
- Nightmare biomes, impossible architecture
- Eldritch features, reality-warping effects
- Cosmic horror monsters

## Prompt Constraints (Global)

All prompts enforce these standards discovered through ChatGPT refinement work:

### Visual Consistency
- **1024x1024 HD resolution**
- **Flat aerial top-down view** (no perspective, no tilt, no horizon)
- **Medieval dark fantasy art style**
- **No circular framing** (edge-to-edge coverage)

### Layer System Support
- **Biomes**: Seamless tileable textures, opaque backgrounds
- **Paths/Features**: Transparent PNG overlays, soft feathered edges
- **Characters**: Chess-piece style tokens, top-down silhouettes
- **Effects**: Transparent overlays for atmospheric enhancement

### Technical Requirements
- **No borders, no frames, no cast shadows**
- **Consistent scale** across all asset types
- **Edge-to-edge seamless** for hex tile compositing
- **Negative constraints** included to prevent unwanted compositions

## Usage Notes

1. **Universal prompts** (biomes, paths) can be generated once and reused across all acts
2. **Act-specific prompts** contain features, monsters, and characters appropriate to that story phase
3. **Degraded variants** are created by appending corruption descriptors while maintaining base constraints
4. **Chess-piece tokens** ensure characters read clearly on any biome background

## File Status

- ✅ **biome-prompts.md** - Refined and consistency-locked
- ✅ **paths-prompts.md** - Refined and consistency-locked  
- ✅ **split-assets-prompt-design.md** - Analysis and refinement process
- 🔄 **All other files** - Need reorganization from DALL-E_ASSET_GENERATION_LIST.md

## Next Steps

1. Extract universal content (UI, effects) into separate files
2. Break down character/monster/feature content by act progression
3. Apply consistency constraints to all prompts
4. Generate and test asset quality across the progression
