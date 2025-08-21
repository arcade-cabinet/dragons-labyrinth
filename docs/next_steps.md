# Dragon's Labyrinth - Next Steps

## Priority 1: Core Game Systems Integration

### 1.1 Migrate Existing Game Logic
- [ ] Move database.rs functionality to a dedicated `dragons_save` crate
- [ ] Integrate cutscenes.rs into the UI crate with Cobweb
- [ ] Move generators.rs content to appropriate crates (maps/ai)
- [ ] Refactor systems.rs into proper ECS systems across crates

### 1.2 Database & Save System
```rust
// Create new crate: dragons_save
- Player progress persistence
- World state serialization  
- Settings management
- Save file encryption/compression
```

### 1.3 Game State Management
- [ ] Implement proper state machine with Bevy States
- [ ] Create loading/menu/gameplay/pause states
- [ ] Add state transition effects using VFX crate

## Priority 2: Content Implementation

### 2.1 Hex-Based Gameplay
- [ ] Implement hex grid movement system
- [ ] Add turn-based combat on hex grid
- [ ] Create hex-based pathfinding for AI
- [ ] Design hex tile types and properties

### 2.2 Horror Atmosphere Systems
- [ ] Implement dread system mechanics
- [ ] Create dynamic lighting system
- [ ] Add fog of war with visibility
- [ ] Implement sanity/corruption mechanics
- [ ] Create ambient horror sound system

### 2.3 3D Hexagonal World
- [ ] Toggle between 2D/3D views
- [ ] Implement camera controls for 3D hex view
- [ ] Add elevation/height to hex tiles
- [ ] Create multi-level dungeons

## Priority 3: Advanced Features

### 3.1 Procedural Generation
- [ ] Implement biome-based map generation
- [ ] Create dungeon generation algorithms
- [ ] Add prop/decoration placement
- [ ] Generate narrative elements procedurally

### 3.2 AI Behaviors
- [ ] Implement companion AI system
- [ ] Create enemy behavior patterns
- [ ] Add dialogue system with NPCs
- [ ] Implement emergent AI behaviors

### 3.3 Visual Effects
- [ ] Create spell/ability effects
- [ ] Implement weather system
- [ ] Add post-processing effects
- [ ] Create death/damage effects

## Priority 4: Polish & Optimization

### 4.1 Performance
- [ ] Implement LOD system for 3D hexes
- [ ] Add frustum culling
- [ ] Optimize particle systems
- [ ] Profile and optimize hot paths

### 4.2 User Experience
- [ ] Create main menu with Cobweb UI
- [ ] Implement settings menu
- [ ] Add controller support
- [ ] Create tutorial system

### 4.3 Audio Polish
- [ ] Implement 3D spatial audio
- [ ] Create dynamic music system
- [ ] Add voice acting support
- [ ] Implement audio occlusion

## Technical Debt

### Code Quality
- [ ] Add comprehensive error handling
- [ ] Write unit tests for each crate
- [ ] Add integration tests
- [ ] Document public APIs
- [ ] Set up CI/CD pipeline

### Architecture Improvements
- [ ] Implement proper event bus
- [ ] Add resource pooling for entities
- [ ] Create debug overlay system
- [ ] Add hot-reload for assets

## Experimental Features

### 3D Hexagonal Innovations
- [ ] Experiment with bevy_clay_tiles for complex hex shapes
- [ ] Try hexagonal prisms with varying heights
- [ ] Implement hex-based terrain deformation
- [ ] Create destructible hex environments

### Advanced Rendering
- [ ] Implement ray-marched volumetric fog
- [ ] Add screen-space reflections
- [ ] Create custom shaders for horror effects
- [ ] Implement temporal upsampling

## Immediate Next Actions

1. **Create dragons_save crate**
   ```bash
   cargo new --lib crates/save
   ```

2. **Implement basic game loop**
   - Start with hex movement
   - Add basic combat
   - Create simple AI

3. **Build first playable prototype**
   - One small hex map
   - Player movement
   - One enemy type
   - Basic combat

4. **Set up development workflow**
   - Configure hot-reload
   - Add debug commands
   - Create level editor bindings

## Resource Requirements

### Assets Needed
- [ ] Hex tile textures/sprites
- [ ] Character models/sprites
- [ ] UI elements and icons
- [ ] Sound effects library
- [ ] Music tracks

### Tools Setup
- [ ] Configure asset pipeline
- [ ] Set up level design workflow
- [ ] Create content creation guidelines
- [ ] Establish art style guide

## Success Metrics

### Week 1 Goals
- Basic hex movement working
- One complete game state (menu → game → menu)
- Save/load functionality
- Basic AI enemy

### Month 1 Goals  
- 3 biomes implemented
- 5 enemy types
- Combat system complete
- First dungeon playable

### Quarter 1 Goals
- Full game loop
- 10+ hours of content
- All core systems integrated
- Beta release ready