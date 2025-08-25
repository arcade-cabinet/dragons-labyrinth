# Nightly Background Agent Handoff
## Date: January 2025
## Priority: Complete Database Implementation

## 🚨 IMPORTANT: READ THIS FIRST
For extended overnight execution WITHOUT MCP tools:
See: `memory-bank/background-yolo-prompt.md`

## CRITICAL CONTEXT
The project has reached a critical juncture where the architecture is correct but implementation is incomplete. The database layer needs to be fully implemented to unblock the rest of the system.

## YOUR PRIMARY MISSION
Complete the implementation of the `GameDatabaseOperations` trait in `crates/game-database/src/engine.rs`. There are 44 methods that need proper implementation.

## Current State Summary

### ✅ What's Working:
1. **Architecture**: Clean separation between database-orm (models) and game-database (operations)
2. **Models**: All SeaORM models properly defined in database-orm
3. **Build System**: Cargo.toml files properly configured with [lib] and [[bin]] sections
4. **Dependencies**: Correct at both build and runtime levels

### 🔴 What's Broken:
1. **44 Unimplemented Methods** in GameDatabaseOperations trait:
   - Player operations (create_player, update_player_progression, etc.)
   - Save system (get_save_slots, delete_save_slot)
   - Horror state (get_horror_state, record_horror_event)
   - Companion operations (update_companion_trauma, record_companion_betrayal)
   - Hex tile operations (update_hex_tile_corruption, discover_hex_tile)
   - Inventory management (get_player_inventory, add_item_to_inventory)
   - AI workflow operations (create_ai_workflow, update_workflow_status)
   - Game state (get_game_state, update_game_state)
   - Statistics (get_player_statistics, update_player_statistics)

2. **Missing Entities** (TODOs in code):
   - AssetUsageLogs (referenced but not implemented in database-orm)
   - WorkflowSteps (referenced but not implemented in database-orm)

3. **Compilation Errors**:
   - Type mismatches in function arguments
   - Missing trait implementations
   - DateTime<Utc> reflection issues (partially addressed with i64)

## Implementation Guidelines

### For the 44 Methods:
Each method in the trait needs to:
1. Get a connection from the pool
2. Use SeaORM entities from database-orm
3. Perform the database operation
4. Return proper DatabaseResult types
5. Handle errors appropriately

Example pattern:
```rust
async fn create_player(&self, name: String, save_slot: i32) -> DatabaseResult<players::Model> {
    let conn = self.connection.read().await;
    let player = players::ActiveModel {
        id: Set(Uuid::new_v4()),
        name: Set(name),
        save_slot_id: Set(save_slot),
        // ... other fields
    };
    let result = player.insert(&*conn).await?;
    Ok(result)
}
```

### For Missing Entities:
Either:
1. Add them to database-orm as new modules
2. Comment out their usage with TODOs
3. Find alternative existing entities

### Critical Rules:
1. **NEVER create minimal solutions** - implement fully
2. **Use database-orm models** - don't duplicate
3. **Follow SeaORM patterns** - use ActiveModel for inserts/updates
4. **Handle errors properly** - use ? operator and DatabaseResult

## File Locations

### Primary Work Area:
- `crates/game-database/src/engine.rs` - Implement trait methods here
- `crates/game-database/src/traits.rs` - Trait definitions (reference only)
- `crates/database-orm/src/*.rs` - Model definitions (use these)

### Build Commands:
```bash
# Test your changes
cargo build --package game-database

# If successful, test the binary
cargo run --package game-database --bin mcp-server
```

## Success Criteria
1. ✅ All 44 methods implemented
2. ✅ game-database compiles without errors
3. ✅ mcp-server binary runs
4. ✅ Database operations tested

## Additional Context

### Available Dependencies:
- sea-orm with sqlx-sqlite backend
- tokio for async runtime
- uuid for IDs
- chrono for timestamps (use i64 for Bevy compatibility)
- serde/serde_json for serialization

### Database Connection Pattern:
```rust
let conn = self.connection.read().await;
// Use &*conn for operations
```

### Transaction Pattern:
```rust
let txn = self.begin_transaction().await?;
// Operations on &txn
txn.commit().await?;
```

## Next Steps After Completion
Once the database layer is working:
1. Test full integration with game-engine
2. Deploy TOML prompt system
3. Complete asset generation pipeline
4. Full system integration test

## Resources
- SeaORM docs: https://www.sea-ql.org/SeaORM/
- Project memory bank: `/memory-bank/`
- Previous implementation patterns: Check git history

## Final Notes
This is a critical blocking issue. Once these 44 methods are implemented, the entire system can move forward. Focus on correctness over optimization - we can refactor later.

Good luck! The architecture is solid, you just need to fill in the implementation.
