# Dragon's Labyrinth - Complete Architecture & Narrative System Task Plan

## Current Understanding
- Core assets moved to `crates/game-engine/assets` 
- New `crates/game-assets` with Rust CC0 processing system
- New `crates/blender-bridge` for model conversion
- Python scripts successfully ported to Rust
- Need to create `game-code` crate for generated content
- Need comprehensive narrative rules system

## Task Breakdown

### Phase 1: Clean Architecture & Complete Wiring
- [ ] Clean out `src/generator/ai` (deprecated Python code)
- [ ] Create `crates/game-code` for all AI-generated code
- [ ] Move build logic from old build.rs to `game-database/build.rs`
- [ ] Fix game-assets to properly move from `raw/` to `library/`
- [ ] Implement recursive zip detection in game-assets
- [ ] Verify OBJ/MTL attachment in blender-bridge
- [ ] Add game-assets as build dependency to game-database

### Phase 2: Complete Build-Tools Wiring
- [ ] Add config-rs for structured rule loading
- [ ] Create idempotent request system for all game phases
- [ ] Wire up actual OpenAI API calls in agents
- [ ] Implement caching and deduplication
- [ ] Add horror progression state management
- [ ] Create asset selection logic (CC0 vs AI generation)

### Phase 3: Narrative System Setup
- [ ] Review all memory-bank bible documents
- [ ] Create `memory-bank/narrative-directives/` directory
- [ ] Create `memory-bank/narrative-completed/` directory
- [ ] Document player's three journeys:
  - Journey to the Labyrinth (Peace → Dread)
  - Journey back home (Terror → False Peace)
  - Final journey to void (Horror → Resolution)

### Phase 4: Rules System
- [ ] Create `crates/game-code/rules/` directory structure
- [ ] Define idempotent rule format (TOML)
- [ ] Create rule categories:
  - Scenes and phases
  - Maps and locations
  - Characters and companions
  - Encounters and bosses
  - Items and equipment
  - Dialogue trees
  - UI elements
  - Audio cues

### Phase 5: Agent Instructions
- [ ] Create `.cursor/rules/01-narrative-generation.mdc`
- [ ] Document database query patterns
- [ ] Define CC0 vs AI decision tree
- [ ] Create structured prompt templates
- [ ] Document integration points (Cobweb UI, YarnSpinner, etc.)
- [ ] Provide error handling guidelines

### Phase 6: Build System Integration
- [ ] Write `crates/game-engine/build.rs`
- [ ] Connect all crates in dependency chain
- [ ] Implement XDG directory structure
- [ ] Create SQLite database initialization
- [ ] Wire up asset pipeline

## Immediate Actions (This Session)

1. **Clean deprecated code**
2. **Create game-code crate structure**
3. **Review memory bank documents**
4. **Begin narrative directive documentation**
5. **Create agent instruction framework**
