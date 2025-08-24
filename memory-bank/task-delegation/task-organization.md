# Master Task Organization

## Current Project State Summary

### Recent Achievements (User-Completed)
- ✅ Moved core assets to `crates/game-engine/assets/` (intro/outro videos)
- ✅ Ported `bpy_processor.py` to Rust in `crates/game-assets/build.rs`
- ✅ Ported `organize_cc0_library.py` to Rust with full attribution
- ✅ Created filesystem-based queue system (`raw/` → `library/`)
- ✅ Created `blender-bridge` crate for OBJ/FBX → GLB conversion
- ✅ Established idempotent asset processing pipeline

### Current Gaps
- ❌ `src/generator/ai` needs cleanup based on build-tools
- ❌ Build-tools AI agents not fully wired
- ❌ Narrative rules not structured for AI generation
- ❌ Database build.rs not connected to game-assets
- ❌ Structured tools in wrong place (should be in game-database runtime)
- ❌ Database created in wrong location (should be OUT_DIR, not XDG)

## Task Assignments by Agent Type

### 🔴 CRITICAL PATH - Advanced Foreground Agent Tasks

#### Task AF-001: Complete Build-Tools Wiring
**Priority**: IMMEDIATE
**Dependencies**: None
**Context Required**: 
- Review `crates/build-tools/src/agents.rs`
- Check `crates/game-database` integration points
- Understand game-assets build pipeline

**Deliverables**:
1. Complete OpenAI API integration in all agents
2. Wire database connection in build.rs
3. Implement actual tool execution
4. Connect to game-assets library catalog
5. Clean up `src/generator/ai` based on new architecture

**Key Technical Points**:
- Use `openai_dive` for structured responses
- Implement idempotent generation checks
- Connect to SQLite in XDG directory
- Ensure horror progression awareness

---

#### Task AF-002: Wire Game-Engine Build Dependencies
**Priority**: HIGH  
**Dependencies**: AF-001
**Context Required**:
- game-engine CAN have build-tools as build-dependency
- Generation happens in game-engine/build.rs
- Rules live in game-engine/rules/

**Deliverables**:
1. Update game-engine/Cargo.toml:
   - Add build-tools as build-dependency
   - Add game-database as build-dependency
2. Implement game-engine/build.rs:
   - Pass OUT_DIR to build-tools for generation target
   - Load rules from game-engine/rules/
   - Call build-tools agents with OUT_DIR path
   - Include generated code from OUT_DIR
3. Create rule structure:
```
crates/game-engine/rules/
├── maps/
├── encounters/
├── dialogue/
└── progression/
```
4. Generated code structure (in OUT_DIR):
```
$OUT_DIR/generated/
├── maps.rs
├── encounters.rs
├── dialogue.rs
└── progression.rs
```

**Architecture Benefits**:
- Generated code in OUT_DIR (compile-time generated)
- game-engine includes from OUT_DIR directly
- No intermediate files in src/
- Clean build-time generation path

---

#### Task AF-003: Fix Game-Database Architecture
**Priority**: HIGH
**Dependencies**: AF-001
**Context Required**:
- Database should be in OUT_DIR for release inclusion
- Structured tools belong in game-database RUNTIME
- Provides bidirectional communication

**Deliverables**:
1. Fix Cargo.toml dependency issues
2. Move structured tools from build-tools to game-database/src/tools.rs
3. Implement build.rs that:
   - Creates SQLite database in OUT_DIR (ships with game!)
   - Populates from game-assets library catalog
   - Generates initial world data
4. Runtime implementation:
   - Start and manage database connection
   - Provide structured tools for build-tools
   - Provide bevy_sqlx ECS world for game-engine
   - Enable bidirectional communication

**Key Architecture Points**:
- Database in OUT_DIR gets compiled into release
- Runtime layer facilitates all communication
- Single source of truth for structured tools

---

### 🟡 Standard Sonnet Agent Tasks

#### Task SS-001: Complete Blender-Bridge Implementation
**Priority**: MEDIUM
**Dependencies**: None
**Context Required**:
- Review Python `bpy_processor.py` for patterns
- Understand OBJ+MTL pairing requirements

**Deliverables**:
1. Detect OBJ files with MTL pairs
2. Implement Blender subprocess spawning
3. Handle batch conversion requests
4. Generate GLB output files
5. Update asset metadata

---

#### Task SS-002: Recursive ZIP Processing
**Priority**: MEDIUM
**Dependencies**: None
**Context Required**:
- Current `game-assets/build.rs` implementation
- ZIP detection and extraction logic

**Deliverables**:
1. Detect nested ZIPs in extracted content
2. Extract to `OUT_DIR` staging area
3. Recursively process contents
4. Move to `library/` when complete
5. Clean up staging areas

---

#### Task SS-003: Asset Inspector PyO3 Bridge
**Priority**: LOW
**Dependencies**: SS-001, SS-002
**Context Required**:
- Python validation UI requirements
- PyO3 bridge patterns

**Deliverables**:
1. Complete PyO3 bindings
2. Expose asset validation API
3. Human-in-the-loop approval system
4. Metadata editing capabilities

---

### 🟢 Sonnet 1M Context Tasks

#### Task S1M-001: Complete Narrative Bible Synthesis
**Priority**: HIGH
**Dependencies**: None
**Context Required**:
- ALL files in `memory-bank/` especially:
  - `design_bible.md`
  - `biomes_reference.md`
  - `companions_reference.md`
- Three-act structure
- Horror progression mechanics

**Deliverables**:
```
memory-bank/narrative-direction/
├── act1-journey-to-labyrinth/
│   ├── scene-breakdowns.md
│   ├── encounter-scripts.md
│   ├── companion-interactions.md
│   └── horror-progression.md
├── act2-journey-home/
│   ├── scene-breakdowns.md
│   ├── betrayal-mechanics.md
│   ├── world-degradation.md
│   └── companion-trauma.md
└── act3-journey-to-void/
    ├── final-confrontation.md
    ├── void-mechanics.md
    ├── ending-variations.md
    └── narrative-closure.md
```

**Requirements**:
- EXTREME detail for each scene
- Specific dialogue examples
- Exact horror progression triggers
- Asset requirements per scene
- Companion state transitions

---

#### Task S1M-002: Original Vision Expansion Review
**Priority**: HIGH
**Dependencies**: None
**Context Required**:
- ALL files in `memory-bank/larger-vision/`
- Comparison with current scaled implementation
- Focus on achievable expansion opportunities

**Deliverables**:
1. Feature feasibility analysis for cut systems
2. Expanded vision summary document
3. Integration recommendations for:
   - Dual Forge System
   - Sentimental Items
   - Companion Personal Quests
   - Dynamic Economy
   - Environmental Storytelling
4. Updated task documents for approved features

**Status**: ✅ COMPLETE - See `memory-bank/expanded-vision/vision-expansion-summary.md`

---

#### Task S1M-003: Asset Library Full Catalog
**Priority**: MEDIUM
**Dependencies**: SS-002
**Context Required**:
- All processed assets in `library/`
- Metadata from attribution
- Category/theme analysis

**Deliverables**:
1. Complete asset inventory with:
   - File paths and formats
   - Attribution metadata
   - Semantic tags
   - Horror-appropriateness ratings
   - Suggested use cases
2. Gap analysis for missing assets
3. Reuse opportunity matrix

---

### 🔵 Cursor Background Agent Tasks

#### Task CB-001: Generate Rule TOML Files
**Priority**: HIGH
**Dependencies**: S1M-001
**Context Required**:
- Narrative synthesis from S1M-001
- TOML structure requirements
- Idempotency requirements

**Template Structure**:
```toml
[metadata]
id = "unique-deterministic-id"
dread_level = 0
act = 1
scene = "intro"

[generation]
prompt_template = """
Detailed prompt with {variables}
"""
required_assets = ["asset_id_1", "asset_id_2"]
ai_model = "gpt-4"

[validation]
required_fields = ["field1", "field2"]
constraints = { max_tokens = 1000 }
```

**Deliverables**:
- 50+ map generation rules
- 100+ encounter rules
- 200+ dialogue variations
- 30+ companion state rules
- ALL following exact TOML format

---

#### Task CB-002: Asset Manifest Generation
**Priority**: MEDIUM
**Dependencies**: S1M-002
**Context Required**:
- Asset library catalog
- Generation requirements

**Deliverables**:
```toml
# For each asset type
[[assets]]
id = "deterministic-id"
path = "library/category/file.glb"
category = "environment"
theme = "horror"
dread_levels = [2, 3, 4]
tags = ["decay", "organic", "disturbing"]
attribution = "Original Author Name"
```

---

#### Task CB-003: Batch Processing Scripts
**Priority**: LOW
**Dependencies**: CB-001, CB-002
**Context Required**:
- Rule structures
- Batch processing patterns

**Deliverables**:
1. Shell scripts for batch operations
2. Manifest files for processing order
3. Validation scripts for output
4. Progress tracking mechanisms

---

## Execution Order

### Phase 1: Foundation (Immediate)
1. **AF-001**: Complete build-tools wiring [Advanced Agent]
2. **S1M-001**: Narrative bible synthesis [1M Context Agent]
3. **AF-002**: Create game-code crate [Advanced Agent]

### Phase 2: Integration (Next 24 hours)
1. **CB-001**: Generate TOML rules [Background Agents]
2. **AF-003**: Fix database integration [Advanced Agent]
3. **SS-001**: Complete blender-bridge [Standard Agent]

### Phase 3: Completion (48 hours)
1. **SS-002**: Recursive ZIP processing [Standard Agent]
2. **S1M-002**: Asset library catalog [1M Context Agent]
3. **CB-002**: Asset manifests [Background Agents]

### Phase 4: Polish (72 hours)
1. **SS-003**: Asset inspector bridge [Standard Agent]
2. **CB-003**: Batch processing [Background Agents]

## Success Metrics

- ✅ All crates compile without errors
- ✅ Database populates from game-assets
- ✅ AI agents generate test content
- ✅ Complete narrative rules in place
- ✅ Asset pipeline fully automated
- ✅ No circular dependencies
- ✅ Clean module separation

## Critical Notes for Agents

### For Advanced Agents
- You have FULL authority to make architectural decisions
- Delete legacy code without asking
- Implement complete solutions, not stubs
- Consider the entire system interconnection

### For Standard Agents
- Focus on your specific module
- Ask for clarification if interfaces unclear
- Write comprehensive tests
- Document your implementation

### For 1M Context Agents
- Read EVERYTHING before starting
- Synthesize across all documents
- Be exhaustively detailed
- Cross-reference extensively

### For Background Agents
- Follow templates EXACTLY
- Use deterministic IDs
- Validate your output
- Work in batches for efficiency
