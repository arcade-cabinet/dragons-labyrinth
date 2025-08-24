# MASTER ORCHESTRATION - Dragon's Labyrinth Production

## THE VISION
Dragon's Labyrinth is a **horror-first emotional journey** disguised as an RPG. The player opens their door on a beautiful morning and ends in first-person darkness, hunted by an ancient intelligence. This is NOT procedural - it's a precisely crafted tragedy unique to each player's choices.

## CRITICAL PRINCIPLE: LAYERS, NOT FEATURES
We build in COMPLETE LAYERS:
1. **ALL Levels** scripted first (emotional beats)
2. **ALL Maps** created second (spatial representation)
3. **ALL Assets** integrated third (visual support)

Do NOT mix layers. COMPLETE each before moving to the next.

## YOUR MISSION
You have access to:
- 1.7GB of CC0 assets in `crates/game-content-static/assets/`
- Complete game design in `crates/dragons-docs/book/`
- Trait/philosophy system in `crates/game-content-static/`
- Bevy 0.16.1 with full ecosystem

## PHASE 1: Level Scripting (READ: 01-COMPLETE-level-structure.md)
**Goal**: Script EVERY encounter for EVERY path variation
- 12 core transitions across full journey
- 3 versions each (TO/FROM/SEALING)
- 2 philosophy paths per transition (first half: Strength/Harmony, second half: Light/Dark)
- 6+ companion combinations each
- 7 dread levels each (Peace through Void)
= 3,024+ unique encounter variations

**Success Criteria**: 
- Player's emotional journey is PERFECT
- Every choice has weight
- Philosophy paths feel completely different
- Companion presence changes everything

## PHASE 2: Hex Map Translation (READ: 02-hex-map-translation.md)
**Goal**: Translate scripted levels to hex-based world
- Hexx integration for grid logic
- Dread-responsive tile corruption
- Philosophy-specific pathfinding
- Companion navigation effects

**Success Criteria**:
- Spatial representation reinforces emotion
- Navigation itself tells story
- Hex grid degrades with dread
- Philosophy paths are visually distinct

## PHASE 3: Asset Integration (READ: 03-asset-curation-integration.md)  
**Goal**: Map ALL assets to emotional purpose
- Analyze 1.7GB of CC0 content
- Generate ECS components for everything
- Identify critical missing pieces
- Create dread-based swapping systems

**Success Criteria**:
- Every asset serves the horror
- Smooth visual degradation
- No placeholder content
- Performance maintained

## THE UNIQUE MECHANICS

### Classless Progression
- Start as villager
- Become what you do
- Weapons define fighting style
- Armor defines defense approach
- NPCs react to accumulated traits

### 4-Path Philosophy
- NOT classes, but worldviews
- Strength: Power through force
- Harmony: Balance in all
- Light: Purity and sacrifice  
- Dark: Corruption and power

### Horror Progression
- Peace (0): Beautiful world
- Unease (1): Something's wrong
- Dread (2): Horror undeniable
- Terror (3): Reality breaks
- Horror (4): Truth revealed

### The Forge System
- Sentimental items collected throughout
- Light Forge: Essence sacrifice
- Dark Forge: Blood sacrifice
- Mythic tier changes dragon encounter

## TECHNICAL REQUIREMENTS

### Performance Targets
- 60 FPS with 10,000+ hex tiles
- 30 FPS on mobile
- < 200MB memory usage
- < 2 second load times

### Architecture
- Bevy 0.16.1 ECS
- Components < 100 lines each
- Systems < 100 lines each
- Clean separation of concerns

### Content Structure
```
game-content-static/     - Rules and identity
├── traits/             - Classless progression
├── philosophy/         - 4-path system
├── dread/             - Horror mechanics
└── assets/            - 1.7GB CC0 library

game-engine/           - Runtime
├── components/        - ECS components
├── systems/          - Game systems
└── plugins/          - Third-party integration
```

## YOUR APPROACH

1. **Start with 01-level-scripting-analysis.md**
   - Understand emotional beats
   - Script complete encounters
   - Account for all variations

2. **Then 02-hex-map-translation.md**
   - Translate emotions to space
   - Build dread progression
   - Create philosophy paths

3. **Finally 03-asset-curation-integration.md**
   - Analyze what we have
   - Map to purpose
   - Generate ECS code

## REMEMBER
- **Quality over quantity**
- **Emotion over mechanics**
- **Vision over generation**
- **Complete layers, not features**

This is a PRECISE emotional journey, not a procedural playground. Every decision serves the horror. Every system reinforces dread. Every asset has purpose.

BEGIN WITH LEVEL SCRIPTING. The emotional journey is EVERYTHING.
