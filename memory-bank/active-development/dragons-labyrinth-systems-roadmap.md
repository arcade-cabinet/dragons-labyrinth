# Dragon's Labyrinth Systems Roadmap
## Building Our Unique Horror Experience on Top of ECS Foundation

### Status: D&D Foundation Complete → Build Dragon's Labyrinth Systems

We've successfully built a complete D&D ECS foundation with database-driven architecture. Now we need to build Dragon's Labyrinth's unique horror systems on top.

## Current Foundation (Completed ✅)
- **Combat System**: Full D&D 5e mechanics with tactical positioning
- **Hex Rendering**: Database-driven visualization with corruption overlays
- **Settlement/NPC Systems**: Rich interaction and trade mechanics  
- **Weather/Faction Systems**: Environmental and political mechanics
- **Dungeon Navigation**: Room-to-room movement with doorway systems
- **Database Integration**: 70k+ HBF entities powering all mechanics

## Dragon's Labyrinth Unique Systems (To Build)

### 1. Companion Psychology & Therapy System 🎯 **PRIORITY 1**
**What makes this unique**: Not D&D companions - deep psychological trauma system
- **Components**: CompanionPsychology, TraumaState, TherapyProgress, TrustLevel
- **Systems**: trauma_progression, therapy_sessions, trust_building, breakdown_detection
- **Resources**: CompanionBonds, TherapyOptions, TraumaHistory
- **Integration**: Existing companion database entities extended with psychology

### 2. Dread Progression Controller 🎯 **PRIORITY 2**  
**What makes this unique**: Master orchestrator that transforms all systems based on emotional stage
- **Components**: DreadLevel, CorruptionInfluence, HorrorProgression
- **Systems**: dread_escalation, system_corruption, reality_distortion
- **Resources**: GlobalDreadState, EmotionalProgression, HorrorThresholds
- **Integration**: Modifies all existing systems based on dread level (0-4)

### 3. Sentimental Item & Forge System 🎯 **PRIORITY 3**
**What makes this unique**: Items collected throughout journey become forge reagents
- **Components**: SentimentalItem, ForgeReagent, MythicGear, ForgeProgress
- **Systems**: item_collection, forge_trials, mythic_creation, sacrifice_mechanics
- **Resources**: ForgeState, ReagentCollection, MythicRecipes
- **Integration**: Uses existing item database but with special sentimental tracking

### 4. 3D First-Person Dungeon System 🎯 **PRIORITY 4**
**What makes this unique**: Not 2.5D tiles - full 3D raycasting with Avian physics
- **Components**: DungeonGeometry, RaycastCollider, FirstPersonView, SoundNavigation
- **Systems**: dungeon_generation, raycasting_navigation, sound_positioning, geometry_corruption
- **Resources**: DungeonLayouts, GeometryState, PhysicsWorld
- **Integration**: Uses existing dungeon database but generates 3D geometry

### 5. Dragon Presence & Stalking System
**What makes this unique**: Dragon as intelligent stalking predator, not traditional boss
- **Components**: DragonPresence, ProximityEffects, StalkingBehavior, DragonAwareness
- **Systems**: proximity_detection, stalking_ai, presence_effects, chase_mechanics
- **Resources**: DragonState, ProximityThresholds, StalkingPatterns
- **Integration**: Overlays onto existing encounter and corruption systems

### 6. Philosophy & Light/Dark Path System
**What makes this unique**: Moral choices affect physics and reality, not just story
- **Components**: PhilosophyAlignment, MoralChoices, PathProgression, RealityInfluence
- **Systems**: choice_recording, path_determination, reality_modification, trait_progression
- **Resources**: PhilosophyState, ChoiceHistory, PathAbilities
- **Integration**: Affects all existing systems based on player's philosophical alignment

### 7. 180-Level Narrative Orchestration
**What makes this unique**: Each level designed for specific emotional progression
- **Components**: LevelProgression, EmotionalState, NarrativeTriggers, SystemEvolution
- **Systems**: level_orchestration, emotional_progression, system_evolution, narrative_triggers
- **Resources**: LevelDatabase, ProgressionRules, EmotionalCurve
- **Integration**: Controls how all other systems evolve throughout the game

### 8. Player Growth & Achievement System  
**What makes this unique**: Inner/outer growth separate from D&D mechanics
- **Components**: InnerGrowth, OuterGrowth, PlayerTraits, AchievementProgress
- **Systems**: growth_tracking, trait_evolution, achievement_unlock, progression_validation
- **Resources**: GrowthMetrics, TraitDatabase, AchievementDefinitions
- **Integration**: Tracks player development beyond combat stats

### 9. Reality Distortion System (High Dread Levels)
**What makes this unique**: Non-Euclidean geometry, impossible architecture
- **Components**: RealityStability, GeometryDistortion, SpatialAnomalies, PerceptionFilter
- **Systems**: reality_breakdown, geometry_corruption, spatial_anomalies, perception_alteration
- **Resources**: RealityState, DistortionRules, GeometryTemplates
- **Integration**: Affects hex rendering and 3D dungeons at high dread levels

### 10. Memory Palace & Trauma Visualization
**What makes this unique**: Therapy through navigating psychological spaces
- **Components**: MemoryPalace, TraumaVisualization, PsychicNavigation, HealingProgress
- **Systems**: memory_construction, trauma_navigation, healing_visualization, therapy_completion
- **Resources**: MemoryTemplates, TraumaDatabase, HealingMethods
- **Integration**: Special 3D spaces for companion therapy sessions

## Implementation Priority

### Phase 1: Core Horror Systems (Weeks 1-2)
1. **Companion Psychology System**: The emotional heart of the game
2. **Dread Progression Controller**: Master orchestrator for all systems
3. **Sentimental Item System**: Collection mechanics for forge preparation

### Phase 2: 3D Integration (Weeks 3-4)  
4. **3D Dungeon System**: First-person horror spaces with Avian
5. **Dragon Stalking System**: Intelligent predator AI
6. **Philosophy System**: Light/dark path mechanics

### Phase 3: Advanced Horror (Weeks 5-6)
7. **180-Level Orchestration**: Narrative progression controller
8. **Reality Distortion**: High dread level effects
9. **Memory Palace Therapy**: Psychological healing spaces
10. **Player Growth Tracking**: Achievement and trait systems

## Key Architectural Decisions

### Building on D&D Foundation
- **Keep existing systems**: Combat, weather, settlements provide rich world
- **Layer Dragon's Labyrinth systems**: Psychology, dread, forge override D&D when needed
- **Integration points**: Dread level affects all existing systems
- **Horror transformation**: D&D mechanics become horror mechanics at high dread

### ECS Architecture Consistency
- Each Dragon's Labyrinth system follows same pattern as combat/hex_rendering
- Full components/systems/resources/events structure
- Bevy plugin architecture for clean integration
- Database-driven where appropriate, computed where needed

### Database vs Computed Systems
- **Database-driven**: Companion bonds, sentimental items, philosophy choices, growth tracking
- **Computed**: Dread progression, reality distortion, dragon stalking, trauma visualization
- **Hybrid**: 3D dungeons (layouts from DB, geometry computed), forge trials (reagents from DB, results computed)

## Next Immediate Actions

1. **Review existing database-orm models** for psychology, forge, philosophy entities
2. **Create companion psychology ECS system** with trauma/therapy mechanics
3. **Implement dread progression controller** that modifies all existing systems
4. **Build sentimental item collection** and forge preparation mechanics
5. **Plan 3D dungeon integration** with Avian physics and raycasting

This roadmap transforms our excellent D&D foundation into the unique horror masterpiece that Dragon's Labyrinth is designed to be.
