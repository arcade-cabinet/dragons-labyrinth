# Cursor Sonnet 4 - YOLO Mode Handoff

## Current State Summary
Dragon's Labyrinth has completed foundational architecture with sophisticated systems (Forge, Philosophy, Trauma, Decay) and third-party library integrations (hexx, bevy_ecs_tilemap, bevy_kira_audio, bevy_hanabi, pathfinding). The revolutionary narrative engine is operational with direct Rust code ownership.

## What Cursor Should Complete (YOLO Continuous Execution)

### Priority 1: Asset Generation Pipeline Completion
**Location**: `crates/build-tools/src/`
**Status**: MCP client exists, needs asset generation workflow

**Tasks**:
1. Complete AI agent workflow orchestration in `agents/` directory
2. Wire up CC0 asset search before generation
3. Implement batch generation with manifests
4. Create OUT_DIR integration for build-time generation
5. Add fallback generation for missing assets

### Priority 2: Deploy Tier 3 Specialized Agents (Selective Prompts)
These agents use selective prompts and are perfect for Cursor's YOLO mode:

#### UIAgent Implementation
**Location**: `crates/build-tools/src/agents/ui.rs`
- Generate horror-responsive UI degradation rules
- Create dread-level specific UI variants (0-4)
- Generate companion trauma visualization elements
- Create forge trial UI components
- Output as TOML rules for runtime consumption

#### DecayAgent Implementation  
**Location**: `crates/build-tools/src/agents/decay.rs`
- Generate environmental corruption progression rules
- Create NPC fear response behaviors
- Generate economic collapse parameters
- Create reality distortion effects
- Output as structured data for runtime

#### MountAgent Implementation
**Location**: `crates/build-tools/src/agents/mounts.rs`
- Generate mount personality traits
- Create mount-rider bonding mechanics
- Generate environmental protection abilities
- Create mount trauma responses
- Output as ECS component configurations

### Priority 3: MCP Server Runtime Intelligence
**Location**: `crates/game-database/src/mcp_server.rs`
**Status**: Framework exists, needs tool implementations

**Tasks**:
1. Implement forge readiness assessment tool
2. Create companion trauma analysis tool
3. Add philosophical progression queries
4. Implement world corruption status tool
5. Create cross-system intelligence aggregation

### Priority 4: Build Integration Wiring
**Location**: `crates/game-engine/build.rs`
**Status**: Build script exists, needs full AI generation workflow

**Tasks**:
1. Call all AI agents in correct order
2. Generate assets for dread levels 0-4
3. Create database at build time in OUT_DIR
4. Generate include files for runtime
5. Add error handling with fallbacks

## File Structure for Cursor

```
crates/
├── build-tools/src/
│   ├── agents/
│   │   ├── mod.rs          # Agent orchestration
│   │   ├── ui.rs           # UIAgent (IMPLEMENT)
│   │   ├── decay.rs        # DecayAgent (IMPLEMENT)
│   │   ├── mounts.rs       # MountAgent (IMPLEMENT)
│   │   ├── maps.rs         # Exists, enhance
│   │   └── levels.rs       # Create
│   ├── mcp_client.rs       # Exists, enhance
│   └── lib.rs              # Wire everything
├── game-database/src/
│   ├── mcp_server.rs       # Enhance with tools
│   └── lib.rs              # Exists
└── game-engine/
    ├── build.rs            # Wire AI generation
    └── src/                # Runtime consumption
```

## Key Patterns to Follow

### Agent Pattern
```rust
pub struct UIAgent {
    dread_level: u8,
    mcp_client: MCPClient,
}

impl UIAgent {
    pub async fn generate(&self) -> Result<UIAssets> {
        // 1. Query game state via MCP
        let state = self.mcp_client.query_game_state().await?;
        
        // 2. Generate appropriate UI variants
        let variants = self.generate_dread_variants(state)?;
        
        // 3. Output as TOML/JSON for runtime
        self.save_to_out_dir(variants)?;
        
        Ok(variants)
    }
}
```

### Build Integration Pattern
```rust
// In game-engine/build.rs
async fn run_ai_generation(out_dir: &Path) -> Result<()> {
    let mut context = BuildContext::new(out_dir)?;
    
    // Generate for each dread level
    for dread_level in 0..=4 {
        let ui_agent = UIAgent::new(dread_level, &context);
        ui_agent.generate().await?;
        
        let decay_agent = DecayAgent::new(dread_level, &context);
        decay_agent.generate().await?;
        
        let mount_agent = MountAgent::new(dread_level, &context);
        mount_agent.generate().await?;
    }
    
    Ok(())
}
```

## Expected Outputs

### UI Generation
- `OUT_DIR/ui/dread_0.toml` through `dread_4.toml`
- UI degradation rules and parameters
- Component visibility/corruption states

### Decay Generation
- `OUT_DIR/decay/corruption_rules.json`
- NPC behavior modification tables
- Economic collapse progression data

### Mount Generation
- `OUT_DIR/mounts/personalities.toml`
- Mount-rider bonding parameters
- Environmental protection abilities

## Success Criteria
1. ✅ All three Tier 3 agents implemented and generating
2. ✅ MCP server providing runtime intelligence
3. ✅ Build script orchestrating all generation
4. ✅ Generated assets loading at runtime
5. ✅ No compilation errors
6. ✅ < 5 second build time with generation

## YOLO Mode Instructions
- Work continuously without stopping
- Make decisions independently
- Implement complete solutions, not stubs
- Test by running `cargo build` frequently
- Use existing patterns from completed code
- Focus on working implementation over perfection

## References
- Sophisticated systems: `crates/game-engine/src/{forge,philosophy,psychology,decay}/`
- MCP patterns: `crates/build-tools/src/mcp_client.rs`
- Database models: `crates/database-orm/src/`
- Build integration: `crates/game-engine/build.rs`
