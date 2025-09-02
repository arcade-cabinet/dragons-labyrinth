# Dragon's Labyrinth - World Design Document

## Core World Structure

### The World of Aethermoor
A massive hexagonal continent where the player journeys from the forgotten southern reaches to the dragon's northern labyrinth. The world is designed as a coherent overworld with logical geography, not disconnected regions.

### Geographic Layout (South to North)

```
                    [THE DRAGON'S LABYRINTH]
                           Level 180
                              ^^^
                    [THE MYTHIC FORGES]
                      Levels 140-179
                     /               \
            [Dark Dwarf Depths]  [High Elf Peaks]
                    \                 /
                     [THE KINGDOM PROPER]
                       Levels 61-139
                            |||
                    [THE KINGDOM GATES]
                         Level 60
                            |||
                    [THE BORDERLANDS]
                       Levels 31-59
                            |||
                    [THE WILD REACHES]
                       Levels 11-30
                            |||
                    [THE FORGOTTEN COAST]
                        Levels 1-10
                         (START)
```

## Three-Act Political Progression

### Act 1: Rise to Recognition (Levels 1-60)
**Theme**: From nobody to somebody
**Journey**: Southern coast → Kingdom gates
**Political Arc**: Village conflicts → Regional tensions → National threat
**Culmination**: Earning entry to the kingdom by defeating the dragonbrood threat

### Act 2: The Cost of Power (Levels 61-120)
**Theme**: Power corrupts, even heroes
**Journey**: Throughout the kingdom's territories
**Political Arc**: Court intrigue → Civil unrest → Royal confrontation
**Culmination**: Confrontation with the corrupted king

### Act 3: The Price of Truth (Levels 121-180)
**Theme**: Becoming what you fought against
**Journey**: To the mythic forges and dragon's labyrinth
**Political Arc**: Ancient alliances → Mythic trials → Final revelation
**Culmination**: Discovering the dragon was protecting the world from the void

## Biome Progression & Logic

### Southern Reaches (Levels 1-30)
**The Forgotten Coast (1-10)**
- **Biomes**: Sandy beaches, tidal pools, coastal cliffs, fishing villages
- **Features**: Small harbors, lighthouse ruins, sea caves, driftwood camps
- **Atmosphere**: Salty air, crying gulls, gentle decay, nostalgic abandonment
- **Key Locations**:
  - Driftwood Village (Starting point)
  - The Old Lighthouse (First dungeon)
  - Smuggler's Cove (First moral choice)

**The Wild Reaches (11-30)**
- **Biomes**: Rolling grasslands, scattered forests, gentle rivers
- **Features**: Farming hamlets, old mills, stone bridges, traveling merchants
- **Atmosphere**: Pastoral beauty with hints of wrongness
- **Key Locations**:
  - Millhaven (First real town)
  - The Whispering Woods (Companion introduction)
  - Bandit's Crossing (Leadership test)

### The Borderlands (Levels 31-59)
**Transitional Territories**
- **Biomes**: Dense forests → rocky hills → mountain passes
- **Features**: Frontier towns, military outposts, refugee camps
- **Atmosphere**: Growing tension, visible dragon damage, military presence
- **Key Locations**:
  - Ironhold Fortress (Military hub)
  - The Scarred Valley (Dragon attack site)
  - Companion's Home (Personal quest)
  - The Kingdom Gates (Act 1 climax)

### The Kingdom Proper (Levels 61-120)
**Central Territories (61-90)**
- **Biomes**: Cultivated farmlands, royal forests, river valleys
- **Features**: Major cities, noble estates, grand temples
- **Atmosphere**: Opulent decay, political tension, social collapse
- **Key Locations**:
  - The Capital City (Political hub)
  - Noble Quarter (Intrigue quests)
  - The Common Districts (Rebellion brewing)

**Outlying Provinces (91-120)**
- **Biomes**: War-torn fields, occupied territories, contested borders
- **Features**: Siege camps, burned villages, military tribunals
- **Atmosphere**: Open warfare, desperate civilians, moral bankruptcy
- **Key Locations**:
  - The King's War Camp (Confrontation setup)
  - The Throne of Judgment (Act 2 climax)
  - The Exile's Road (Transition to Act 3)

### The Ancient Territories (Levels 121-180)
**The Forge Approaches (121-140)**
- **Biomes**: Primordial forests OR volcanic wastelands (path dependent)
- **Features**: Ancient ruins, mystical barriers, trial grounds
- **Atmosphere**: Mythic weight, ancient power, otherworldly beauty/terror

**The Mythic Forges (141-160)**
- **High Elf Peaks** (Light Path):
  - Crystal spires, aurora bridges, singing stones
  - Trials of virtue, sacrifice, and wisdom
  
- **Dark Dwarf Depths** (Dark Path):
  - Magma rivers, obsidian halls, soul furnaces
  - Trials of power, dominance, and will

**The Dragon's Domain (161-180)**
- **Biomes**: Reality-broken wastelands, void-touched territories
- **Features**: Impossible geometry, temporal fractures, the labyrinth entrance
- **Atmosphere**: Cosmic horror, reality unraveling, terrible truth dawning
- **The Labyrinth**: First-person horror experience, physics breaking down

## Key Design Principles

### 1. Logical Geography
- Biomes transition naturally (coast → grassland → forest → mountains)
- Rivers flow from mountains to sea
- Climate makes sense (cold in north/mountains, temperate in middle, warm in south)
- Trade routes follow geographic logic

### 2. Hexagonal Tile System
- Each hex represents roughly 1 mile
- Tiles have base biome + overlay features
- Movement costs vary by terrain and path
- Exploration reveals adjacent tiles

### 3. Level-Based Progression
- Player level ≈ distance traveled from start
- No arbitrary level walls, but natural barriers
- Enemies scale with distance from civilization
- Resources become scarcer as you travel north

### 4. Scene-Based Generation
Each area needs:
- **Visual Description**: What the player sees
- **Audio Landscape**: What the player hears
- **Emotional Tone**: What the player feels
- **Mechanical Purpose**: What the player does
- **Narrative Thread**: How it connects to the larger story

## Companion Integration

### Starting Companions (Levels 1-20)
- **Elena**: Met in Whispering Woods, idealistic healer
- **Marcus**: Found at Bandit's Crossing, pragmatic soldier
- **Quinn**: Discovered in sea caves, mysterious scholar

### Companion Trauma Progression
- **Levels 1-30**: Building bonds, learning backstories
- **Levels 31-60**: First trauma events, stress accumulation
- **Levels 61-90**: Breaking points, loyalty tests
- **Levels 91-120**: Potential betrayals or deeper bonds
- **Levels 121-150**: Forge sacrifices, ultimate tests
- **Levels 151-180**: Final transformations

## Quest Thread Architecture

### Main Quest Line
1. **The Lost Child** (1-10): Establish heroic identity
2. **The Bandit Threat** (11-20): Learn leadership costs
3. **The Refugee Crisis** (21-30): Face impossible choices
4. **The Border War** (31-40): Choose sides
5. **The Dragonbrood** (41-60): Prove worth to kingdom
6. **The Court's Games** (61-80): Navigate politics
7. **The King's Madness** (81-100): Uncover truth
8. **The Civil War** (101-120): Lead or destroy
9. **The Ancient Pact** (121-140): Seek mythic power
10. **The Forge Trials** (141-160): Pay ultimate price
11. **The Dragon's Truth** (161-180): Face cosmic horror

### Recurring Themes
- **Inverted Power**: Getting weaker as you "progress"
- **Moral Ambiguity**: No clearly right choices
- **Companion Psychology**: Relationships define gameplay
- **Environmental Storytelling**: World tells its own story
- **Horror Escalation**: From unease to cosmic dread

## Scene Descriptions for AI Generation

### Example: Driftwood Village (Level 1-3)
**Visual**: Weathered shacks on stilts, nets drying in salt wind, children playing in tide pools, old fishers mending boats
**Audio**: Gulls crying, waves lapping, creaking wood, distant sea shanty
**Emotional**: Nostalgic, peaceful, but something indefinably wrong
**Mechanical**: Tutorial area, basic combat, first choices
**Narrative**: Establish player as outsider, introduce "lost child" quest

### Example: The Kingdom Gates (Level 60)
**Visual**: Massive iron gates, dragon damage visible, guards suspicious, refugees camped outside, smoke on horizon
**Audio**: Armor clanking, desperate pleas, martial drums, distant roaring
**Emotional**: Anticipation mixed with dread, threshold moment
**Mechanical**: Major progression gate, equipment check, companion loyalty test
**Narrative**: Transition from regional to national stakes

### Example: The Dragon's Threshold (Level 180)
**Visual**: Reality fractures, impossible angles, breathing walls, void leaking through cracks, the dragon waiting
**Audio**: Cosmic humming, heartbeat of universe, whispers in dead languages, silence between sounds
**Emotional**: Cosmic horror, terrible understanding, inevitable doom
**Mechanical**: Final boss, all systems climax, choice of endings
**Narrative**: Reveal dragon protected world, player becomes new guardian or destroyer

## Void Corruption (Endgame)

After defeating the dragon, the void begins corrupting the world:
- **Void Tiles**: Overlay existing biomes with corruption
- **Scaled Encounters**: All enemies become void-touched
- **New Mechanics**: Reality distortion, sanity management
- **Infinite Content**: Procedural void events
- **Final Choice**: Become the seal, rule as tyrant, or let world end

## Implementation Priority

### Phase 1: Core World Structure
- Define all biome types and transitions
- Create hex coordinate system
- Implement basic movement and exploration

### Phase 2: Political Progression
- Design three acts with clear transitions
- Create faction system
- Implement reputation/consequence tracking

### Phase 3: Scene Generation
- Build template system for location descriptions
- Create AI prompts for scene-specific content
- Implement dynamic narrative threading

### Phase 4: Horror Integration
- Add dread progression overlays
- Implement companion trauma system
- Create reality distortion effects

## Success Metrics

- **Geographic Coherence**: World feels real and logical
- **Narrative Flow**: Story progresses naturally with geography
- **Emotional Journey**: Player feels progression from hope to horror
- **Mechanical Integration**: All systems support world design
- **Replayability**: Different paths create different experiences
