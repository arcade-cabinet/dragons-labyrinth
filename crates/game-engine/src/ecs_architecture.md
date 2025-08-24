# Dragon's Labyrinth ECS Architecture

## Core Principles
1. **Dread-Driven**: All systems respond to global dread level (0-4)
2. **Database Separation**: game.db (static) vs player.db (user state)
3. **Direct Asset Integration**: Assets in game-engine/assets with accompanying ECS code
4. **Third-Party Integration**: Leverage existing crates for complex systems

## Component Hierarchy

### Core Components
```rust
// Player & Movement
Player { save_slot, dread_level }
Position { hex: Hex } // Using hexx crate
Velocity { linear, angular } // For avian physics

// World & Environment  
HexTile { terrain, elevation, corruption }
Biome { type, decay_level }
Environmental { weather, time_of_day }

// Companions & NPCs
Companion { psychology, trauma, loyalty }
NPC { dialogue_tree, fear_level }
Mount { bond_strength, abilities }

// Game Systems
Quest { narrative_stage, moral_weight }
Inventory { items, capacity }
Health { current, max, wounds }
```

### Third-Party Integration Points

#### Hexx (Hex Grid)
- Components: HexPosition, HexTile, HexLayout
- Systems: HexMovementSystem, HexPathfindingSystem, HexRenderSystem

#### Avian (Physics)
- Components: RigidBody, Collider, PhysicsBundle
- Systems: PhysicsSystem, CollisionSystem

#### YarnSpinner (Dialogue)
- Components: DialogueRunner, DialogueView, VariableStorage
- Systems: DialogueSystem, ChoiceSystem, NarrativeEventSystem

#### Cobweb (Story Graph)
- Components: StoryNode, StoryBranch, NarrativeState
- Systems: StoryProgressionSystem, BranchingSystem

#### Yoleck (Level Editor)
- Components: YoleckEntity, LevelData, EditableProperties
- Systems: LevelLoadingSystem, EditorSystem

#### Mapgen (Procedural Generation)
- Components: MapSeed, GenerationRules, BiomeMap
- Systems: TerrainGenerationSystem, BiomeDistributionSystem

#### ClayTiles (Voxel/Tile Rendering)
- Components: TileMesh, TileTexture, TileAnimation
- Systems: TileRenderingSystem, TileUpdateSystem

## System Organization

### Core Game Loop
1. **Input Systems** → Player input, UI interaction
2. **Movement Systems** → Hex movement, physics
3. **AI Systems** → Companion behavior, NPC reactions
4. **Narrative Systems** → Quest progression, dialogue
5. **Horror Systems** → Dread progression, corruption
6. **Render Systems** → Visual updates, UI

### Database Architecture
```
game.db (Built-in, Read-only in production)
├── static_content/
│   ├── dialogue_trees
│   ├── quest_definitions  
│   ├── level_layouts
│   └── asset_metadata

player.db (User's XDG directory)
├── save_slots/
│   ├── player_state
│   ├── companion_states
│   ├── quest_progress
│   └── world_changes
├── settings/
└── achievements/
```

## Asset Integration Pattern

For each asset category:
1. Analyze with Sonnet vision (textures) or 3D analysis (models)
2. Move selected assets to `game-engine/assets/[category]/`
3. Create corresponding ECS components
4. Build systems that load and use these assets
5. Wire into gameplay mechanics

## Migration from Current State

### Phase 1: Core ECS Setup
- [ ] Consolidate duplicate component definitions
- [ ] Create proper plugin architecture
- [ ] Set up resource management

### Phase 2: Database Separation  
- [ ] Move player-specific data out of game.db
- [ ] Create player.db initialization
- [ ] Build migration system for existing saves

### Phase 3: Third-Party Integration
- [ ] Wire up hexx for hex grid
- [ ] Integrate avian for physics
- [ ] Connect yarnspinner for dialogue
- [ ] Set up cobweb for narrative graph
- [ ] Configure yoleck for level editing

### Phase 4: Asset Curation & Integration
- [ ] Batch assets for Sonnet analysis
- [ ] Move curated assets to game-engine
- [ ] Build loading systems for each asset type
- [ ] Create gameplay features using assets

## Playable Content Goals

By completing this architecture:
- **Boss Rooms**: 3D raytraced walls, DOOM-style dungeons
- **Hex Maps**: Fully explorable world with biomes
- **Companion System**: Psychology, trauma, bonding
- **Horror Progression**: Visual & audio corruption
- **Narrative**: Complete dialogue trees and quests
