# Dragon's Labyrinth Project Design Bible

## Core Philosophy

Dragon's Labyrinth is a horror RPG disguised as an adventure. The game begins with opening your front door on a beautiful morning and ends in first-person darkness, hunted by an ancient intelligence.

**Fundamental Principle**: "We're not building 'an RPG with horror elements' - we're building a horror experience that happens to have RPG mechanics."

### Core Principles
1. **Zero External Dependencies**: All assets are AI-generated or from Freesound
2. **Idempotent Generation**: Running the same prompt always produces compatible results
3. **Component Independence**: Each component works standalone and in combination
4. **Performance by Design**: Mobile-friendly optimization from the start
5. **Horror-First Narrative**: Every system reinforces the growing dread

## Horror-First Design

### The Journey IS the Game
Like Frodo's walk to Mordor, the growing dread is the experience. The game is about feeling the weight of inevitability, the chill in the air that grows colder with each step.

### Narrative Arc
The game follows a strict emotional progression that never decreases:
**Peace → Unease → Dread → Terror → Horror**

Each phase transforms all game systems:
- **Peace**: Beautiful world, helpful NPCs, standard quests
- **Unease**: Things seem off, birds stop singing, whispers start
- **Dread**: Open fear, NPCs flee, companions question continuing
- **Terror**: Ghost towns, reality questions, preparation for inevitable
- **Horror**: Full first-person labyrinth experience

### Vision-Critical Elements
1. **The Opening**: First-person view of leaving home on a peaceful morning
2. **The Dragon's Labyrinth**: Jarring transition to first-person horror
3. **Proximity Horror**: Dragon actively hunts you with audio cues
4. **Sanity System**: False sounds, hallucinations, reality breakdown
5. **Companion Trauma**: Psychological impact on party members
6. **Multiple Endings**: Based on understanding, not power

## Narrative Orchestration Architecture

### Core Realization
We don't need Python orchestration. We need AI orchestration guided by narrative structure. The game's emotional journey IS the orchestrator.

### Individual Narrative Metaprompts
Instead of one master metaprompt, we use individual narrative-focused metaprompts for each system that:
1. **Control the narrative perspective** - Each system knows its emotional purpose
2. **Trust AI with mechanics** - While conveying all narrative stages it must abide by
3. **Generate diverse content** - Each system generates ALL the varied content it needs
4. **Maintain thematic coherence** - Everything serves the horror journey

### Event-Driven Dependencies
Systems react to narrative events rather than technical dependencies:
- Dread level increases trigger cascading changes
- NPCs dynamically adjust behavior based on emotional stage
- Quests transform based on narrative progression
- Companions react to accumulated trauma

## Project Rules & Principles

### No External Dependencies
**RULE**: The project MUST NOT rely on any purchased or external asset libraries.
- ✅ DO: Generate all models, icons, and UI elements via AI
- ✅ DO: Use Freesound for audio effects (CC0 licensed)
- ❌ DON'T: Reference external asset packs
- ❌ DON'T: Create asset library abstractions

### Idempotent Code Generation
**RULE**: Generated code must be idempotent - running generation multiple times produces functionally identical results.

Implementation Requirements:
1. Use deterministic IDs for all generated entities
2. Sort all collections before output
3. Version all API contracts
4. Generate consistent file names
5. Preserve existing customizations when regenerating

### Component-Based Architecture
**RULE**: Every system must be composed of small, focused components.
- Maximum component size: 100 lines of core logic
- Single responsibility per component
- Clear input/output contracts
- No cross-component dependencies

### Performance-First Design
**RULE**: All generated code must meet performance targets.

Minimum Requirements:
- 60 FPS with 10,000 rendered hex tiles
- < 200MB memory for full game
- < 2 second area load times
- 30 FPS on mid-range mobile devices

## Key Components

### Stage Progression
1. **Peace (0)**: Bright world, mundane quests, friendly NPCs
2. **Unease (1)**: Shadows, whispers, Hollow Caretaker boss
3. **Dread (2)**: Swamps, ruins, economy collapse, Forsaken Knight boss
4. **Terror (3)**: Reality warps, companion betrayal, moral horrors
5. **Horror (4)**: Dragon's labyrinth, stalking mechanics, final choice

### Companions
- **Einar**: Loyal friend who breaks under pressure
- **Mira**: Optimist who abandons party in Dread stage
- **Sorin**: Scholar who becomes traitor boss if not handled properly
- **Tamara**: Innocent baker's apprentice, represents lost innocence

### Boss Encounters
- Each boss offers meaningful choices (empathy vs brutality, forgiveness vs execution)
- Choices influence companion morale, available endings, and dragon proximity
- Final dragon encounter has three endings: Acceptance, Defiance, Understanding

## Systems Integration
- **Narrative Orchestration**: All systems respond to dread level (0-4)
- **Sanity System**: Hallucinations and false audio cues
- **Proximity Horror**: Dragon presence through sound/environment
- **Choice Consequences**: Boss encounters influence endings

## Content Generation Integration
Each metaprompt doesn't just create systems, it generates ALL content needed:
- **Biomes**: 5-10 unique types per emotional stage
- **Weapons**: From normal swords to cursed weapons
- **Audio**: Music and sounds for each dread level
- **Models**: Characters, items, environments

## Asset Generation Rules

### Model Generation
- AI-generated .glb files with vertex colors
- Optimized for mobile performance
- Consistent art style across all stages

### Audio Integration
- Freesound integration directly in prompts
- CC0 licensed content only
- Spatial audio for proximity horror

### Icon Generation
- SVG-based UI elements
- Consistent visual language
- Responsive design principles

## Implementation Standards

### Bevy/Rust Optimization
- Entity Component System architecture
- Memory-safe model management
- WebAssembly compilation for web deployment
- Direct memory control vs garbage collection

### Quality Metrics
- Frame rate consistency
- Memory usage monitoring
- Load time optimization
- Mobile compatibility testing