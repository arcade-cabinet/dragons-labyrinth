# Dragon's Labyrinth - Active Context
## Current Sprint: HBF Export Integration
### Date: 2025-01-20 Evening

## IMMEDIATE FOCUS: Parse HBF SQLite Export

### Current State (As of 2025-01-20 Evening)
Major pivot to HBF export approach after comprehensive system implementation:

**✅ COMPLETED TODAY:**
- Converted ALL Python scripts to RON format (eliminated Blender dependency)
- Implemented simple GLTF generation without requiring Blender
- Created comprehensive 180-level structure with journey_to/from/seal_void
- Built hex world system with tactical overworld mechanics
- Implemented 7 wolf variants with weather-appropriate spawning
- Created weather combat effects system with elemental damage
- Fixed blender-bridge to work without subprocess/BPY
- Established RON as primary model definition format
- Researched Hexroll/SCROLL extensively
- Created comprehensive SCROLL modules (stashed for future)

**🔄 NEW FOCUS:**
- Parse HBF SQLite database (incoming to /hbf-export)
- Analyze ~70k HTML files for game content
- Correlate Hexroll data to ECS components
- Build import pipeline for Bevy

## Critical Architecture Decisions

### Model Ownership
- **database-orm**: Owns ALL SeaORM model definitions
- **game-database**: Uses models from database-orm, provides operations
- **NO DUPLICATION**: Single source of truth for models

### Binary Architecture
Only 2 binaries in entire project:
1. `game-engine/dragons-labyrinth` - The actual game executable
2. `game-database/mcp-server` - MCP server for AI agent queries

Libraries only (no binaries):
- `build-tools` - Used by build scripts
- `database-orm` - Model definitions
- `game-assets` - Asset management
- `blender-bridge` - 3D conversion

### Dependency Management Rules
- ALWAYS use `cargo add` for adding dependencies
- Run `cargo upgrade --incompatible` to validate
- Never manually edit dependency versions
- Both build and runtime dependencies must be explicit

## Phase 3 Integration Status

### Completed Components ✅
1. **Blender-Bridge**: Pure Rust wrapper using blr crate
2. **Game-Assets Build**: TOML parsing fixed, CC0 processing works
3. **Database-ORM Models**: All entities implemented with proper UUIDs
4. **MCP Server Config**: Local .cursor/mcp.json configured

### Active Issues 🔧
1. GameDatabaseOperations trait - 44 methods need implementation
2. Bevy reflection for DateTime types - using i64 timestamps
3. Cross-crate compilation dependencies

### Pending Tasks 📋
1. TOML-based prompt queue system
2. Asset routing to agents (Blender, OpenAI, Freesound)
3. Remaining 2.5D/3D asset generation

## Key Principles

### NEVER Create Minimal Solutions
- Build complete, full-featured systems
- Fix errors properly, don't work around them
- No stub implementations in production

### Clean Architecture
- Strict crate separation by responsibility
- Build-time vs runtime separation
- Proper Cargo configuration

## Next Immediate Actions

1. **Implement GameDatabaseOperations methods** (Priority 1)
2. **Fix remaining compilation errors**
3. **Test full build pipeline**
4. **Deploy TOML prompt system**

## Build Commands
```bash
# Database (builds lib + mcp-server)
cargo build --package game-database

# Game (builds lib + game binary)  
cargo build --package game-engine

# Run game
cargo run --package game-engine

# Run MCP server
cargo run --package game-database
```

## Known Working Systems
- AI Agent generation (UIAgent, DecayAgent, etc.)
- Horror progression (dread levels 0-4)
- Asset processing pipeline
- Database architecture (structurally correct)

## Environment
- API Keys: OPENAI_API_KEY, FREESOUND_API_KEY available
- Rust: Latest stable
- Bevy: 0.16.1
- SeaORM: 1.1.14

## Handoff Ready
Database architecture is clean and correct. Focus next session on implementing the 44 trait methods to complete the system.
