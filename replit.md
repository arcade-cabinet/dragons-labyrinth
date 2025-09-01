# Overview

Dragon's Labyrinth is a narrative-driven horror RPG that disguises itself as a hex-based adventure game. Built as a browser-based HTML5 game with JavaScript and Canvas rendering, the project creates an emotional journey from peaceful exploration to absolute terror. The game follows a "horror first, RPG second" philosophy where the emotional arc drives all gameplay mechanics and systems.

The core experience centers around a hexagonal tile-based world where players progress through distinct emotional stages: Peace → Unease → Dread → Terror. Unlike traditional RPGs that focus on character progression and power growth, this game implements an "inverted combat" system where fighting makes players weaker, companions suffer psychological trauma, and the world itself becomes increasingly hostile.

# User Preferences

Preferred communication style: Simple, everyday language.

# System Architecture

## Frontend Architecture

The game uses a single-page HTML5 application with Canvas-based rendering for the hexagonal world. The main game loop handles tile rendering, player movement, and UI updates through JavaScript. A custom cursor is disabled to enhance immersion, and the interface uses a retro terminal aesthetic with Courier New font and glowing red text effects.

The loading system provides visual feedback during asset initialization, featuring a progress bar and atmospheric text styling that reinforces the horror theme from the very beginning.

## Tile-Based World System

The core architecture revolves around a "Layer Cake Tile System" where each hexagonal tile consists of multiple composable layers:

- **Base Biome Layer**: Defines terrain type (grassland, forest, lava, etc.) with associated movement modifiers and environmental effects
- **Path Overlay Layer**: Roads, trails, and other transportation routes that modify movement speed and provide comfort bonuses
- **Feature Overlay Layer**: Buildings, encounters, and interactive elements that drive narrative progression

This modular approach allows for complex environmental storytelling while maintaining clean, reusable components that can be mixed and matched across the 120-level progression.

## Emotional Progression Engine

The game implements a sophisticated emotional state system that tracks the world's transition through four distinct acts:

1. **Levels 1-20**: Peace to Unease - Bright, welcoming environments with subtle corruption hints
2. **Levels 21-40**: Unease to Dread - Transition zones showing dragon blight and environmental damage  
3. **Levels 41-60**: Dread to Terror - Hellscape approaching the Dragon's Labyrinth
4. **Levels 61-120**: Post-dragon return journey - Social apocalypse and political breakdown

Each emotional stage affects biome generation, NPC behavior, companion psychology, and available interactions. The system ensures progression never reverses, creating an inexorable descent into horror.

## Companion Psychology System

NPCs and companions feature dynamic psychological states that respond to environmental conditions and player actions. Companions can flee, break under stress, or suffer trauma from witnessing violence. This system reinforces the horror narrative by making every action carry emotional weight beyond traditional RPG mechanics.

## Asset Generation Pipeline

The game leverages AI-generated content across all major asset categories:

- **3D Models**: Low-poly GLB files with vertex colors for environmental features
- **Terrain Textures**: Seamless, tileable biome textures generated with specific top-down aerial constraints
- **Audio**: Procedural horror music and atmospheric sound effects
- **Content**: Quests, dialogue, and encounters that reflect the current emotional stage

The asset generation uses carefully crafted prompts that ensure consistency in perspective, tileability, and thematic coherence across the massive 120-level world.

# External Dependencies

## AI Generation Services
- **DALL-E**: Primary asset generation for terrain textures and environmental art with specialized prompts for seamless tiling
- **Music21**: Procedural horror music generation system
- **Freesound**: Audio effects library for environmental and interaction sounds

## Development Tools
- **Godot Engine References**: While the current implementation is HTML5/JavaScript, the architecture documents reference Godot 4.x patterns for potential future porting
- **Template Processor**: Custom content generation system for creating narrative-aware game elements

## Browser Technologies
- **HTML5 Canvas**: Core rendering system for hexagonal world display
- **Web Audio API**: Audio playback and atmospheric sound management
- **Local Storage**: Game state persistence and progress tracking

The system is designed to be entirely self-contained within the browser environment, with no external server dependencies for core gameplay. All assets are pre-generated and bundled with the application to ensure consistent performance and offline capability.