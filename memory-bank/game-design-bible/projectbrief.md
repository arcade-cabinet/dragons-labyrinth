# Project Brief: Dragon's Labyrinth

## Overview
Dragon's Labyrinth is a horror RPG disguised as a hex-based adventure. Built in Godot with AI-generated assets, the game takes players on an emotional journey from a peaceful morning to absolute terror, where every system reinforces growing dread.

## Core Vision
Create a horror experience that happens to have RPG mechanics. The journey IS the game - like Frodo's walk to Mordor, players feel the weight of inevitability, the chill in the air that grows colder with each step.

## Target Platform
- Primary: Desktop (Windows, macOS, Linux)
- Secondary: Mobile (optimized for performance)
- Engine: Godot 4.x
- Language: GDScript (AI-generated)

## Key Features
1. **The Opening**: First-person door scene - the last peaceful moment
2. **Hexagonal Exploration**: Tactical movement that reveals environmental decay
3. **Growing Dread System**: NPCs flee, companions break, world darkens
4. **Inverted Combat**: Fighting makes you weaker, not stronger
5. **Companion Trauma**: Psychological impact on party members
6. **The Dragon's Labyrinth**: First-person horror where you're hunted
7. **Multiple Endings**: Based on understanding, not power

## AI-Generated Foundation
All assets created through AI generation:
- **Models**: Low-poly GLB files with vertex colors
- **Biomes**: 5-10 unique types per emotional stage
- **Audio**: Music21 for procedural horror, Freesound for effects
- **Systems**: Narrative-aware code generation
- **Content**: Everything from weapons to quests reflects emotional stage

## Development Approach
1. **Narrative-First Design**: Emotional journey drives all systems
2. **Individual Metaprompts**: Each system has narrative-focused template
3. **Content Integration**: Metaprompts generate code AND assets
4. **Existing Infrastructure**: Leverage template_processor.py strength

## Success Criteria
- The door scene haunts players days later
- Companions feel real enough their trauma matters
- Every system reinforces the horror journey
- The dragon encounter causes genuine fear
- Players understand it's horror first, RPG second

## Core Philosophy
"We're not building 'an RPG with horror elements' - we're building a horror experience that happens to have RPG mechanics."

## Emotional Arc
**Peace → Unease → Dread → Terror**

This progression never reverses and affects:
- Quest design (innocent → horrifying)
- NPC behavior (helpful → terrified)
- World state (beautiful → corrupted)
- Audio (peaceful → oppressive)
- Combat (empowering → desperate)

## Technical Innovation
- **Narrative Orchestration**: The journey IS the orchestrator
- **AI Generates 99%**: We guide with 1% narrative context
- **Zero Dependencies**: All assets AI-generated or from Freesound
- **Idempotent Generation**: Same prompt = compatible results
