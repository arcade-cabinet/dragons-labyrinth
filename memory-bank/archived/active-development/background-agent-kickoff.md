# Background Agent Kickoff
## Mission: Complete Database Implementation
## Priority: CRITICAL PATH BLOCKER

## START HERE
Read these files IN ORDER:
1. `memory-bank/nightly-handoff.md` - Your specific instructions
2. `memory-bank/activeContext.md` - Current state
3. `crates/game-database/src/traits.rs` - The 44 methods you need to implement
4. `crates/game-database/src/engine.rs` - Where to implement them

## YOUR SINGLE FOCUS
Implement the 44 missing methods in `GameDatabaseOperations` trait.

## Method Categories to Implement

### Player Operations (8 methods)
- create_player
- update_player_progression  
- get_save_slots
- delete_save_slot
- get_horror_state
- record_horror_event
- get_horror_progression_history
- get_aggregated_statistics

### Companion Operations (5 methods)
- update_companion_trauma
- record_companion_betrayal
- get_available_companions
- update_companion_availability
- (get_active_companions already done)

### World Operations (6 methods)
- update_hex_tile_corruption
- get_tiles_for_corruption_spread
- discover_hex_tile
- get_discovered_tiles
- update_dragon_proximity
- record_world_event

### Encounter/Dialogue (4 methods)
- get_active_encounters
- complete_encounter
- get_dialogue_options
- update_dialogue_state

### Inventory Management (5 methods)
- get_player_inventory
- add_item_to_inventory
- remove_item_from_inventory
- update_item_condition
- get_items_by_corruption_level

### AI/Asset Operations (7 methods)
- record_asset_usage
- get_asset_dependencies
- update_asset_approval
- create_ai_workflow
- update_workflow_status
- get_workflows_pending_review
- record_workflow_human_feedback

### Game State (5 methods)
- get_game_state
- update_game_state
- get_player_statistics
- update_player_statistics
- (record_world_event listed above)

### Save System (4 methods)
- (get_save_slots listed above)
- (delete_save_slot listed above)
- (get_player_by_save_slot already done)
- (create_player listed above)

## Implementation Pattern
```rust
#[async_trait::async_trait]
impl GameDatabaseOperations for GameDatabase {
    async fn method_name(&self, params...) -> DatabaseResult<ReturnType> {
        let conn = self.connection.read().await;
        
        // Use database_orm entities
        let result = Entity::find()
            .filter(column::Column.eq(value))
            .one(&*conn)
            .await?;
            
        Ok(result)
    }
}
```

## Critical Rules
1. Use `database_orm::*` models, NEVER create new ones
2. Use SeaORM patterns (ActiveModel for inserts/updates)
3. Return proper `DatabaseResult<T>` types
4. Use `&*conn` for database operations
5. Import what you need from database_orm

## Test Your Work
```bash
# After each batch of methods
cargo build --package game-database

# When all done
cargo run --package game-database --bin mcp-server
```

## Success Criteria
- All 44 methods have implementations (not stubs!)
- game-database compiles without errors
- mcp-server runs without panics

## If You Get Stuck
1. Check existing implemented methods for patterns
2. Look at database_orm models for field names
3. Use SeaORM documentation
4. Leave detailed TODOs if truly blocked

## Time Estimate
~15-20 minutes per method × 44 methods = ~12-15 hours

## Final Note
This is THE critical blocker. Nothing else can proceed until these methods are implemented. Focus exclusively on this task. Do not refactor, optimize, or improve other code. Just implement the 44 methods.

Good luck! You've got this!
