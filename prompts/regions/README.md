# Regional Meta-Prompts System

This directory contains the manual meta-prompts for each named region of Aethermoor. These are the creative direction documents that drive all automated generation.

## Structure

```
regions/
├── heartlands/
│   ├── 01-verdant-crossing/
│   │   ├── meta-prompt.md          # Manual creative direction
│   │   ├── prompt.toml             # Generated asset prompts
│   │   └── spec.yaml               # Generated code specifications
│   ├── 02-whispering-vale/
│   └── 03-thornhollow-reach/
├── contested-lands/
│   ├── 04-ashfall-borders/
│   ├── 05-broken-crown-valley/
│   └── 06-ember-crossroads/
├── burning-marches/
│   ├── 07-obsidian-wastes/
│   └── ... (continuing through band 60)
└── ... (other major geographic areas)
```

## Regional Naming Convention
- **Band 1-20 (Heartlands)**: Peaceful, pastoral names suggesting growth and safety
- **Band 21-40 (Contested Lands)**: Names suggesting conflict and transition
- **Band 41-60 (Burning Marches)**: Names suggesting desolation and approach to danger
- **Band 61-120 (Broken Realm)**: Names suggesting social collapse and decay
- **Band 121-180 (Nightmare Expanse)**: Names suggesting unreality and cosmic horror

## Meta-Prompt Template
Each region's meta-prompt.md should contain:

### Basic Information
- **Region Name**: Full name with poetic subtitle
- **Band**: Which 20-level band this serves
- **Emotional Goal**: What the player should feel/learn
- **Geographic Position**: Where it sits relative to other regions

### Game Mechanics
- **Biome Composition**: Primary and secondary biomes with ratios
- **Travel Scope**: Days to cross, movement restrictions
- **Adjacency**: Which regions connect (max 2-3 neighbors)
- **Rest Requirements**: Shelter mechanics, safe zones

### Narrative Elements
- **Core Quests**: 3-5 major story beats with emotional arcs
- **Side Encounters**: Random events that reinforce theme
- **Key NPCs**: Named characters with specific roles and motivations
- **Companion Interactions**: How this region affects party dynamics

### Technical Requirements
- **Transition Hooks**: 3D scenes with specific names for TransitionLoader
- **Asset Categories**: Visual elements that must be generated
- **Audio Cues**: Environmental sounds and musical themes
- **Special Mechanics**: Region-specific gameplay features

## Generation Pipeline
1. **Manual**: Write meta-prompt.md (human creativity)
2. **Automated**: Generate prompt.toml from meta-prompt + guides
3. **Automated**: Generate spec.yaml from meta-prompt + architecture
4. **Automated**: Create assets from prompt.toml
5. **Automated**: Generate code from spec.yaml

This ensures human creativity drives the vision while AI handles the detailed implementation.
