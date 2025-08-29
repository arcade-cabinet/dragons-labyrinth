# Dragon's Labyrinth: World Foundation Guide

## Core World Identity

**World Name**: Aethermoor  
**Subtitle**: "The Moorlands of the First Light"

A realm caught between dying light and hungry darkness, where the great dragon Pyraxis once stood guardian against the primordial void that lurks beneath all reality.

## Geographic Philosophy: Concentric Rings of Corruption

Aethermoor is structured as expanding rings of corruption radiating from the Dragon's Labyrinth at the world's heart. Distance from the center determines both threat level and reality's stability.

### The Five Great Rings

1. **The Heartlands (Bands 1-20)**: Peace → Unease
   - Rolling meadows, ancient forests, mountain valleys
   - Life still flourishes, children play safely
   - Subtle signs: withered patches in healthy fields, ash-tasting dreams
   - Biomes: Lush plains, healthy forests, clear streams, gentle hills

2. **The Contested Lands (Bands 21-40)**: Unease → Dread  
   - First scars of dragon-flight across living landscapes
   - Charred forests adjacent to green groves
   - Villages showing raid damage, nervous lords
   - Biomes: Partially burned forests, dry plains, desert edges, cracked hills

3. **The Burning Marches (Bands 41-60)**: Dread → Terror
   - Direct approach to the Labyrinth proper
   - Lava fields, obsidian deserts, blackened stone
   - Ancient fortresses from humanity's last stand
   - Biomes: Molten terrain, dried riverbeds, jagged rock, void-cracks

4. **The Broken Realm (Bands 61-120)**: Terror → Despair → Madness
   - Post-dragon social collapse and political chaos
   - Environmental healing but human cruelty unleashed
   - Warlords, refugees, failing alliances
   - Biomes: Ruined versions of familiar terrain, militarized zones

5. **The Nightmare Expanse (Bands 121-180)**: Madness → Void
   - Reality itself unraveling as void breaks free
   - Impossible geometries, bleeding skies, cosmic horror
   - Familiar landscapes twisted into nightmares
   - Biomes: Corrupted versions of all previous terrain types

## Core Mechanical Philosophy

### Movement & Time Progression
- **Day Cycle**: 8 hex maximum walking, 12 hex running (with encounter risks)
- **Night Requirements**: Shelter mandatory after Band 20, fire/camping mechanics
- **Weather Impact**: Affects movement, rest requirements, encounter tables
- **Seasonal Changes**: Biome generation and hazard modification

### Rest & Survival Mechanics
- **Civilized Zones**: Inns, guarded settlements, safe social spaces
- **Wilderness**: Manual camp setup, fire building, watch posting
- **Hostile Biomes**: Life-or-death shelter requirements (cold, void exposure)
- **Companion Dependency**: Different companions excel in different environments

### Horror-First Emotional Progression
Every system responds to **Dread Level (0-4)**:
- Biome generation adapts to emotional stage
- NPC behavior shifts with growing terror
- Companion psychology evolves with trauma
- Player choices carry permanent moral weight

## Regional Design Principles

Each region must define:
1. **Emotional Arc**: What the player feels and learns
2. **Biome Ratios**: Primary/secondary/tertiary terrain types with percentages
3. **Travel Scope**: Days to traverse, movement restrictions, hazard levels
4. **Adjacency Rules**: Geographic and thematic connections (max 2-3 neighbors)
5. **Narrative Beats**: 3-5 core quests that advance the larger story
6. **Random Events**: Side encounters reinforcing regional theme
7. **Key Characters**: Named NPCs with specific roles and motivations
8. **Transition Hooks**: 3D story scenes loaded via TransitionLoader system
9. **Asset Requirements**: Visual/audio elements that must exist for immersion

## The Dragon's Truth (Narrative Core)

**Pyraxis was never the villain.** He was the last guardian, using eternal flame to seal void-cracks and maintain reality's integrity. His labyrinth served as both prison and fortress, containing horrors beyond mortal comprehension.

By slaying the dragon, the hero becomes complicit in reality's unraveling. The true horror isn't environmental destruction—it's the revelation that the "tyrant" was actually holding back something infinitely worse.

This truth drives every narrative beat:
- Early game: Build attachment to the world's beauty
- Mid game: Growing unease as corruption spreads
- Late game: Cosmic horror as reality comes apart
- Endgame: Player must become the new guardian or watch everything die

## Asset Generation Constraints

### Visual Requirements
- **Tile Format**: 128x128 PNG with transparent backgrounds
- **Layer Cake System**: Base biome + overlay features + paths
- **Deterministic Generation**: Same inputs produce same outputs
- **Mobile Performance**: Optimized for broader device compatibility
- **Quality Upscaling**: Generate at 256x256, downsample to target resolution

### Audio Requirements
- **Dynamic Adaptation**: All audio responds to dread progression
- **Environmental Layers**: Base ambience + weather + supernatural elements
- **Emotional Reinforcement**: Musical themes that enhance narrative beats
- **Zero External Dependencies**: AI-generated or copyright-free sources only

### Technical Architecture
- **Godot 4 Integration**: All assets flow into proper Godot project structure
- **Manifest System**: Metadata tracks relationships and dependencies  
- **Orchestrator Coordination**: Python system generates specifications
- **Regional Modularity**: Each region can be developed independently

This foundation ensures every generated element serves both mechanical gameplay and narrative progression, creating a coherent world that supports the horror-first emotional journey from peace to cosmic dread.
