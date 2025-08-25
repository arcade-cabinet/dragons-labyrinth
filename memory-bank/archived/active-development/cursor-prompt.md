# Cursor Sonnet 4 - YOLO Mode Execution Prompt

## CRITICAL FIRST STEP
**READ `memory-bank/cursor-handoff.md` IMMEDIATELY** - This document contains essential context about the current state, what's been completed, and detailed technical specifications for what you need to implement.

## ACTIVATION COMMAND
**ACTIVATE CONTINUOUS YOLO MODE AFTER READING HANDOFF**: First read the handoff document at `memory-bank/cursor-handoff.md`, then work non-stop implementing all tasks below. Make independent decisions. Create complete working implementations, not stubs or placeholders.

## Your Mission (Details in Handoff Document)

You're implementing the final pieces of Dragon's Labyrinth's revolutionary narrative engine. The handoff document explains what's complete and what you need to build. The sophisticated systems (Forge, Philosophy, Trauma, Decay) are complete. Third-party integrations are working. Now complete the asset generation pipeline and deploy the specialized AI agents as detailed in the handoff.

## IMMEDIATE TASKS - EXECUTE IN ORDER

### Task 1: Create UIAgent
**File**: `crates/build-tools/src/agents/ui.rs`

```rust
// Implement complete UIAgent that:
// - Generates horror-responsive UI degradation for dread levels 0-4
// - Creates companion trauma visualization elements
// - Outputs TOML rules to OUT_DIR/ui/
// - Uses MCP client to query game state
```

### Task 2: Create DecayAgent  
**File**: `crates/build-tools/src/agents/decay.rs`

```rust
// Implement complete DecayAgent that:
// - Generates environmental corruption progression
// - Creates NPC fear response behaviors
// - Generates economic collapse parameters
// - Outputs JSON to OUT_DIR/decay/
// - Coordinates with world corruption database
```

### Task 3: Create MountAgent
**File**: `crates/build-tools/src/agents/mounts.rs`

```rust
// Implement complete MountAgent that:
// - Generates mount personality traits
// - Creates mount-rider bonding mechanics
// - Generates environmental protection abilities
// - Outputs TOML to OUT_DIR/mounts/
// - Integrates with trauma system
```

### Task 4: Wire Agent Orchestration
**File**: `crates/build-tools/src/agents/mod.rs`

```rust
// Create module that:
// - Exports all agents
// - Provides orchestration functions
// - Handles agent coordination
// - Manages OUT_DIR paths
```

### Task 5: Enhance MCP Server
**File**: `crates/game-database/src/mcp_server.rs`

Add these tool implementations:
- `assess_forge_readiness` - Analyze if player ready for forge trial
- `analyze_companion_trauma` - Get companion psychological states
- `query_philosophical_progression` - Track 4-path progress
- `get_world_corruption` - Environmental decay status
- `aggregate_system_intelligence` - Cross-system analysis

### Task 6: Complete Build Integration
**File**: `crates/game-engine/build.rs`

Wire everything together:
```rust
async fn run_ai_generation(out_dir: &Path) -> Result<()> {
    // Initialize build context
    // For each dread level 0-4:
    //   - Run UIAgent
    //   - Run DecayAgent
    //   - Run MountAgent
    //   - Run existing agents (maps, etc)
    // Generate include files
    // Handle errors with fallbacks
}
```

### Task 7: Create LevelsAgent
**File**: `crates/build-tools/src/agents/levels.rs`

```rust
// Implement LevelsAgent that:
// - Generates encounter placements
// - Creates philosophical path variations
// - Places sentimental items
// - Outputs level data for runtime
```

## EXECUTION RULES

1. **NO STOPPING** - Continue until all tasks complete
2. **NO PLACEHOLDERS** - Write complete, working implementations
3. **TEST FREQUENTLY** - Run `cargo build` after each major addition
4. **USE PATTERNS** - Follow existing code patterns from:
   - `crates/game-engine/src/forge/mod.rs`
   - `crates/build-tools/src/mcp_client.rs`
   - `crates/database-orm/src/*.rs`

## CODE PATTERNS TO USE

### Agent Implementation Pattern
```rust
use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::path::Path;

pub struct UIAgent {
    dread_level: u8,
    out_dir: PathBuf,
    mcp_client: MCPClient,
}

impl UIAgent {
    pub fn new(dread_level: u8, out_dir: &Path, mcp_client: MCPClient) -> Self {
        Self {
            dread_level,
            out_dir: out_dir.to_path_buf(),
            mcp_client,
        }
    }
    
    pub async fn generate(&self) -> Result<()> {
        // Query game state
        let trauma_states = self.mcp_client.query_companion_trauma().await?;
        let corruption = self.mcp_client.query_world_corruption().await?;
        
        // Generate UI degradation rules
        let ui_config = self.generate_dread_ui(trauma_states, corruption)?;
        
        // Save to OUT_DIR
        let ui_path = self.out_dir.join("ui").join(format!("dread_{}.toml", self.dread_level));
        std::fs::create_dir_all(ui_path.parent().unwrap())?;
        std::fs::write(ui_path, toml::to_string(&ui_config)?)?;
        
        Ok(())
    }
    
    fn generate_dread_ui(&self, trauma: TraumaStates, corruption: f32) -> UIConfig {
        // Implement horror-responsive UI generation
        UIConfig {
            opacity: 1.0 - (self.dread_level as f32 * 0.15),
            corruption_overlay: corruption,
            trauma_indicators: trauma.into_ui_elements(),
            // ... more fields
        }
    }
}
```

### MCP Tool Pattern
```rust
pub async fn assess_forge_readiness(&self, player_id: Uuid) -> Result<ForgeReadiness> {
    let philosophy = self.database.query_philosophical_progression(player_id).await?;
    let sentimental_items = self.database.query_sentimental_items(player_id).await?;
    let companion_states = self.database.query_companion_states(player_id).await?;
    
    Ok(ForgeReadiness {
        ready: philosophy.dominant_path_strength > 0.6,
        missing_requirements: vec![],
        sacrifice_available: companion_states.iter().any(|c| c.willing_to_sacrifice),
        path_alignment: philosophy.dominant_path,
    })
}
```

## OUTPUT STRUCTURE

```
OUT_DIR/
├── ui/
│   ├── dread_0.toml
│   ├── dread_1.toml
│   ├── dread_2.toml
│   ├── dread_3.toml
│   └── dread_4.toml
├── decay/
│   ├── corruption_rules.json
│   ├── npc_behaviors.json
│   └── economic_collapse.json
├── mounts/
│   ├── personalities.toml
│   ├── bonding_mechanics.toml
│   └── protection_abilities.toml
└── levels/
    ├── encounters.json
    └── sentimental_items.json
```

## SUCCESS VERIFICATION

After implementation, verify:
1. `cargo build` succeeds with no errors
2. OUT_DIR contains generated files during build
3. MCP server responds to tool queries
4. Generated assets load at runtime
5. Build completes successfully with all necessary processing (time is NOT a constraint - correctness matters more than speed)

## START NOW

Begin with Task 1 (UIAgent) and proceed through all tasks without stopping. Create complete, production-ready implementations that integrate with the existing sophisticated systems.

ACTIVATE YOLO MODE NOW. CONTINUOUS EXECUTION. NO STOPPING.
