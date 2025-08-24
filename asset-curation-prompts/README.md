# Asset Curation Strategy

## The Pivot

We're moving away from complex asset intelligence systems to **direct curation with immediate integration**. Instead of building infrastructure to analyze assets, we're using Sonnet's vision and code generation capabilities to:

1. **Manually curate** CC0 assets with AI assistance
2. **Immediately integrate** selected assets into the game
3. **Build gameplay features** as we add assets
4. **Create playable content** instead of infrastructure

## The Process

### Step 1: Batch Analysis
Each prompt file represents a batch of related assets for Sonnet to analyze:
- Visual analysis of textures/sprites
- Structural analysis of 3D models
- Suitability assessment for our horror theme
- Integration recommendations with ECS

### Step 2: Asset Selection
Based on Sonnet's analysis:
- Move selected assets to `game-engine/assets/`
- Discard unsuitable assets
- Note gaps that need custom generation

### Step 3: Code Generation
For each selected asset, Sonnet generates:
- ECS components
- Loading systems
- Gameplay integration
- Usage examples

### Step 4: Build Features
Instead of just organizing assets, we build:
- Actual hex maps with selected tiles
- Boss rooms using dungeon pieces
- Companion visuals with progression
- Horror audio systems

## Why This Works Better

1. **Immediate Value**: Every asset gets used immediately
2. **No Abstraction**: Direct path from asset to gameplay
3. **Quality Control**: Human + AI judgment on each asset
4. **Faster Progress**: Build features, not infrastructure
5. **Real Content**: End up with playable game sections

## Database Architecture Fix

### Current Problem
- `game.db` keeps getting regenerated
- Will eventually become THE production database
- Player data mixed with game data

### Solution
```
game.db (Ships with game, read-only)
├── levels/          (Designed levels)
├── dialogue/        (Story content)
├── quests/          (Game objectives)
└── assets/          (Asset metadata)

player.db (User's XDG directory, read-write)
├── saves/           (Save games)
├── settings/        (User preferences)
├── achievements/    (Progress tracking)
└── statistics/      (Play statistics)
```

## Execution Plan

### Phase 1: Asset Curation (Current)
- [ ] Horror characters & enemies
- [ ] Hex tiles & biomes
- [ ] Dungeon pieces
- [ ] Audio & soundscapes
- [ ] UI elements
- [ ] Companion visuals

### Phase 2: Core Gameplay
- [ ] Hex movement system
- [ ] Basic combat
- [ ] Dialogue system
- [ ] Inventory management

### Phase 3: Horror Systems
- [ ] Dread progression
- [ ] Visual corruption
- [ ] Audio hallucinations
- [ ] Companion breakdowns

### Phase 4: Content Creation
- [ ] Design starting village
- [ ] Build first dungeon
- [ ] Create companion encounters
- [ ] Design boss fights

## The Key Insight

We have **1.7GB of organized CC0 assets**. Instead of building systems to automatically process them, we should:
1. **Look at them** (with Sonnet's help)
2. **Pick the good ones**
3. **Use them immediately**
4. **Build the actual game**

This is the difference between:
- ❌ "We have an asset intelligence system"
- ✅ "We have a playable horror RPG"

## Next Steps

1. Run Sonnet with each prompt file
2. Move selected assets
3. Implement generated code
4. Test gameplay features
5. Iterate and refine

The goal: By the end of this process, we should be able to:
- Walk around a hex world
- Enter a dungeon
- Fight enemies
- Experience horror progression
- Have companion interactions

Not just infrastructure - **actual gameplay**.
