# Task AF-001: Complete Build-Tools Wiring

## Critical Context

You are completing the AI agent infrastructure that was partially implemented. The user has created a sophisticated Rust-based build-tools system using `openai_dive` but the actual OpenAI API calls and database connections are not yet wired.

## Current State

### What EXISTS:
```rust
// crates/build-tools/src/agents.rs
pub trait Agent: Send + Sync {
    fn name(&self) -> &str;
    fn domain(&self) -> &str;
    async fn generate(&mut self, context: &BuildContext, request: GenerationRequest) 
        -> Result<GenerationResult>;
    fn get_tools(&self) -> Vec<ChatCompletionTool>;
}
```

### What's MISSING:
1. Actual OpenAI API calls in agent implementations
2. Database connection in BuildContext
3. Tool execution implementation
4. Connection to game-assets library catalog

## Required Actions

### 1. Review and Clean Legacy Code
```bash
# DELETE these Python files - they're replaced by Rust
rm -rf src/generator/ai/
```

### 2. Complete Agent Implementations

In `crates/build-tools/src/agents/maps.rs`:
```rust
impl Agent for MapsAgent {
    async fn generate(&mut self, context: &BuildContext, request: GenerationRequest) 
        -> Result<GenerationResult> {
        // IMPLEMENT:
        // 1. Check cache first
        // 2. Build messages with horror context
        // 3. Call OpenAI with tools
        // 4. Process structured response
        // 5. Generate hex map data
        // 6. Store in database
        // 7. Return result with metadata
    }
}
```

### 3. Wire Database Connection

In `crates/build-tools/src/context.rs`:
```rust
impl BuildContext {
    pub async fn connect_database(&mut self, database_url: &str) -> Result<()> {
        // IMPLEMENT:
        // 1. Connect to SQLite in XDG directory
        // 2. Run migrations if needed
        // 3. Verify game-assets table exists
        // 4. Cache connection for reuse
    }
}
```

### 4. Implement Tool Execution

In `crates/build-tools/src/tools.rs`:
```rust
pub async fn execute_tool(context: &BuildContext, name: &str, arguments: &str) -> Result<Value> {
    match name {
        "search_assets" => {
            // Query game-assets library catalog
            // Return matching assets with metadata
        },
        "query_database" => {
            // Execute database queries
            // Return hex tiles, companions, etc.
        },
        "horror_progression" => {
            // Calculate horror state
            // Return dread-appropriate parameters
        },
        "generate_asset" => {
            // Trigger asset generation
            // Store result in database
        },
        _ => Err(Error::UnknownTool(name.to_string()))
    }
}
```

### 5. Connect to Game-Assets Library

The user has created a new asset processing system:
- `crates/game-assets/build.rs` processes CC0 assets
- Assets move from `raw/` to `library/`
- Full attribution metadata is preserved

You need to:
1. Read the processed library catalog
2. Make it searchable via the `search_assets` tool
3. Enable 80/20 reuse strategy (80% existing, 20% generated)

## Integration Points

### With game-database:
- Use database tools for queries
- Store generated content
- Track generation history

### With game-assets:
- Read library catalog at build time
- Search for reusable assets
- Track asset usage

### With blender-bridge:
- Trigger 3D conversions when needed
- Process generated Blender scripts
- Output GLB files for Bevy

## Horror Progression Requirements

EVERY generation must consider:
```rust
pub struct HorrorState {
    pub dread_level: u8,        // 0-4
    pub act: u8,                 // 1-3
    pub dragon_proximity: f32,   // 0.0-1.0
    pub world_corruption: f32,  // 0.0-1.0
    pub companion_states: HashMap<String, CompanionState>,
}
```

## Testing Strategy

1. Create test generation request
2. Mock OpenAI responses initially
3. Verify database writes
4. Check cache functionality
5. Validate horror progression

## Success Criteria

- [ ] All 5 agents make real OpenAI calls
- [ ] Database connection established
- [ ] Tools execute with real data
- [ ] Cache system prevents duplicates
- [ ] Horror progression affects all generation
- [ ] Legacy Python code deleted
- [ ] Integration tests pass

## Code Locations

- Main implementation: `crates/build-tools/src/`
- Agent modules: `crates/build-tools/src/agents/`
- Database integration: `crates/game-database/`
- Asset catalog: `crates/game-assets/library/`

## Critical Notes

1. **Idempotency**: Same inputs MUST produce same outputs
2. **Token Management**: Use tiktoken-rs to stay under limits
3. **Error Recovery**: Graceful fallbacks for API failures
4. **Performance**: Cache aggressively, batch when possible
5. **Horror First**: Every decision filtered through horror lens

## DO NOT:
- Create placeholder implementations
- Skip error handling
- Ignore horror progression
- Generate without caching
- Break idempotency

## DO:
- Implement complete solutions
- Wire everything end-to-end
- Test with real API calls
- Document your decisions
- Consider the full system
