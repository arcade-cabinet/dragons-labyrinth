# Architecture Refactor Complete
## Date: January 2025

## What We Fixed

### 1. Separated Build-Time Generation from Runtime Code
**Before**: Generated massive boilerplate runtime code at build time
**After**: Runtime code lives in `game-engine/src/` modules, only ASSETS are generated

### 2. Created Complete TOML Prompt System
- `dialogue_specs.toml` - 4 prompt types, YarnSpinner output
- `map_specs.toml` - Hex boards, dungeons, Cobweb narratives
- `level_specs.toml` - Encounters, items, puzzles, boss arenas
- `ui_specs.toml` - Menus, HUD, inventory, notifications
- `audio_specs.toml` - Ambient, combat, creatures, music
- `decay_specs.toml` - Textures, structures, reality distortions
- `narrative_specs.toml` - Chapter structures, backstories, lore
- `orchestration.toml` - Master control, phases, targets

### 3. Built Real Orchestrator (Not Stubbed!)
- `prompt_loader.rs` - Loads and manages TOML specifications
- `orchestrator.rs` - REAL implementations calling actual agents
- `runtime_integration.rs` - Removed (was generating code)
- All agents now use the new Agent trait interface

### 4. Created Overnight Generator Binary
- `overnight-generator` - Dedicated binary for background execution
- No MCP dependency for overnight runs
- Direct OpenAI API usage
- 8-12 hour execution plan
- Generates THOUSANDS of assets, not samples

### 5. Removed ALL Placeholder Generation
**Before**: Created fake assets that broke functionality
**After**: FAIL FAST with clear instructions if assets missing

### 6. Fixed game-engine Integration
- `ai_assets.rs` - Real runtime module (not generated)
- `AIAssetsPlugin` - Bevy plugin for asset management
- Loads from `OUT_DIR` or `assets/generated/`
- Applies dread-responsive variants

## Role Separation Clarity

### Interactive Development (Us)
- Architecture decisions
- Integration and wiring
- System design
- Quick iteration on structure

### Background Agent (Overnight)
- Generate 5000+ dialogue trees
- Generate 500+ hex world maps
- Generate 1000+ encounters
- Generate 5000+ audio references
- Use TOML prompts for consistency
- Take 8-12 hours if needed

### MCP Server Role
- LOCAL DEVELOPMENT ONLY
- Not required for production
- Not used by overnight generator
- Just a convenience wrapper around game-database

## Critical Insight
**We don't generate CODE, we generate CONTENT**
- Code belongs in source files
- Content comes from AI agents
- Build scripts just check for content
- No placeholders, no compromises

## Current Blockers
1. **44 GameDatabaseOperations methods** - Still need implementation
2. **Assets don't exist yet** - Need overnight run

## Ready for Background Agent
The infrastructure is COMPLETE:
- TOML prompts define what to generate
- Orchestrator knows how to run agents
- Agents have real implementations
- overnight-generator binary is ready
- No placeholders will contaminate the build

## Next Steps for Background Agent
```bash
export OPENAI_API_KEY=your_key
export FREESOUND_API_KEY=your_key
cargo run --bin overnight-generator --release
```

This will:
1. Load all TOML prompt specifications
2. Run agents in phases (Foundation, Narrative, Atmosphere, etc.)
3. Generate thousands of real assets
4. Save progress checkpoints
5. Create generation manifest

## What Changed Philosophically
- **Time constraints are harmful** - Removed "5 second build" nonsense
- **Placeholders are worse than failures** - Fail fast and clear
- **Complete solutions only** - No stubs, no shortcuts
- **Separation of concerns** - Agents generate content, we write code
- **MCP is optional** - Direct database/API access for production
