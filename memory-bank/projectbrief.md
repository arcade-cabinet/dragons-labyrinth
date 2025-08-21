# Dragon's Labyrinth - Project Brief

## Vision Statement
Dragon's Labyrinth is a **horror-first RPG** disguised as an adventure game. Players begin by opening their front door on a beautiful morning and end in first-person darkness, hunted by an ancient intelligence. This is not an RPG with horror elements - it's a horror experience that uses RPG mechanics to deliver its narrative.

## Core Requirements

### Technical Foundation
- **Platform**: Rust with Bevy 0.16.1 game engine
- **Architecture**: Entity Component System (ECS) with <100 line systems
- **Deployment**: Native + WebAssembly for web/mobile
- **Performance**: 60 FPS with 10,000+ hex tiles, 30 FPS mobile

### Asset Strategy
- **Zero External Dependencies**: No purchased or external asset libraries
- **AI Generation**: All models, textures, and UI elements via AI
- **Audio**: Freesound integration for CC0 licensed content only
- **Idempotent**: Same generation inputs always produce compatible results

### Gameplay Architecture
- **Hex Grid World**: Hexx crate for pathfinding and navigation
- **Board Rendering**: Beauty textures with splatmaps and control maps
- **Dialogue System**: Yarn Spinner for narrative-heavy interactions
- **Level Editing**: Yoleck for in-engine content creation
- **UI Framework**: Cobweb UI for declarative interface design

## Narrative Structure

### Emotional Progression
**Peace → Unease → Dread → Terror → Horror**

Each stage fundamentally transforms all game systems:
- Stage 0 (Peace): Beautiful world, helpful NPCs, standard quests
- Stage 1 (Unease): Shadows lengthen, whispers start, Hollow Caretaker boss
- Stage 2 (Dread): Swamps spread, economy collapses, Forsaken Knight boss
- Stage 3 (Terror): Reality warps, companion betrayal, moral horrors
- Stage 4 (Horror): Dragon's labyrinth, first-person stalking, final choice

### Companion Arcs
- **Einar**: Loyal friend who breaks under pressure (trauma > 0.8)
- **Mira**: Optimist who abandons party in Dread stage (level 2)
- **Sorin**: Scholar who becomes traitor boss if loyalty < 0.3
- **Tamara**: Innocent baker's apprentice representing lost innocence

### Boss Encounters
Each boss offers meaningful moral choices:
- Empathy vs brutality
- Forgiveness vs execution
- Understanding vs destruction

Choices influence:
- Companion morale and trauma
- Available endings (Acceptance, Defiance, Understanding)
- Dragon proximity in final labyrinth

## Success Criteria

### Technical Metrics
- ✅ 60 FPS with 10,000+ rendered hex tiles
- ✅ < 200MB memory usage for full game
- ✅ < 2 second area load times
- ✅ 30 FPS on mid-range mobile devices
- ✅ No memory leaks or garbage collection issues

### Narrative Metrics
- Horror progression feels inevitable and emotional
- Companion arcs evoke genuine attachment and loss
- Boss encounters present meaningful moral dilemmas
- Multiple endings feel earned through player choices
- Dragon presence creates genuine dread through audio/proximity

### Content Metrics
- All assets AI-generated or from Freesound
- Idempotent generation produces consistent results
- Component-based systems under 100 lines each
- Zero external dependencies maintained
- Mobile-friendly from initial design

## Project Scope

### In Scope
- Complete 5-stage horror progression
- 4 companion characters with full arcs
- 3 major boss encounters + dragon finale
- Hex-based world with 5-10 biomes per stage
- Proximity horror audio system
- Sanity/hallucination mechanics
- Multiple endings based on choices

### Out of Scope
- Multiplayer functionality
- User-generated content tools
- Voice acting (text-based dialogue)
- Procedural quest generation
- Save game cloud sync
- Achievement system

## Development Philosophy

### Horror-First Principle
Every feature, system, and asset must serve the horror narrative. If something doesn't enhance the growing dread or emotional journey, it doesn't belong in the game.

### Component Independence
Each system works standalone and in combination. No system should require another to function, only enhance when combined.

### Performance by Design
Mobile optimization isn't an afterthought - every architectural decision considers mobile performance from the start.

### Narrative as Orchestrator
The dread level (0-4) is the master orchestrator. All systems respond to and are transformed by the current narrative stage.