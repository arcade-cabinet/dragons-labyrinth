# Architecture Pivot Decision Document

## Decision Date: 2025-08-25

## The Decision
**Eliminate the game-database layer entirely and build the world directly into Bevy ECS components.**

## Context and Rationale

### Team Size Reality
- We are a 2-person team (user + AI)
- No dedicated writers or content creators
- Cannot maintain complex data pipelines
- Need to maximize value from existing content

### The HBF Gift
- 70,801 entities with 2,245 containing rich content
- 617 hex tiles = 180+ levels of gameplay
- Complete dungeon systems with 335+ rooms
- Full faction networks and NPC relationships
- Weather systems, rumor tables, treasure placement
- **This is 5+ years of content creation already done**

### Why Direct ECS?

#### Problems with Database Approach:
1. **Duplication**: ORM models duplicate ECS components
2. **Synchronization**: Constant need to sync DB↔ECS
3. **Performance**: Runtime loading overhead
4. **Complexity**: Extra abstraction layer
5. **Maintenance**: Two places to update for changes

#### Benefits of Direct ECS:
1. **Simplicity**: Components ARE the data
2. **Performance**: Everything compiled, no runtime loading
3. **Type Safety**: Rust compiler validates everything
4. **Direct Integration**: No translation layer needed
5. **Maintenance**: Patch generated code directly

## Technical Approach

### Phase 1: Enhanced Extraction
```rust
// Current extractor misses HTML refs
extract_doc_title(html) // ✓ We have this
extract_html_refs(html) // ✗ MISSING - must add
crawl_ref_graph(refs)   // ✗ MISSING - must add
```

### Phase 2: Code Generation
```rust
// Generate actual Rust files, not data
generate_world_module() -> Vec<RustFile> {
    vec![
        generate_hexes_rs(),      // 617 hex spawn functions
        generate_dungeons_rs(),    // 335+ room entities
        generate_npcs_rs(),        // 500+ NPCs with dialogue
        generate_factions_rs(),    // 50+ faction networks
        generate_weather_rs(),     // Regional weather systems
        generate_relationships_rs() // Wire all connections
    ]
}
```

### Phase 3: Relationship Resolution
```rust
// HashMap for entity lookups during spawn
let mut world_index: HashMap<String, Entity> = HashMap::new();

// Spawn phase
let hex_n2 = spawn_hex_n2(&mut commands);
world_index.insert("hex_n2", hex_n2);

// Relationship phase
if let Some(faction) = world_index.get("red_snakes") {
    commands.entity(npc).insert(FactionMember(*faction));
}
```

## Migration Strategy

### From game-database to game-engine:
```
game-database/src/models/*.rs → game-engine/src/components/*.rs
game-database/src/systems/*.rs → game-engine/src/systems/*.rs
```

### Key Transformations:
- SeaORM Models → Bevy Components
- Database queries → ECS queries
- Foreign keys → Entity references
- JSON fields → Nested components

## AI Strategy

### Batching for Efficiency:
```yaml
Batch 1: Settlement NPCs
  - Group by location
  - 20-30 NPCs per API call
  - Shared context for consistency

Batch 2: Faction Dialogue
  - Group by faction
  - Conspiracy-themed dialogue
  - Consistent voice per faction

Batch 3: Companion Classification
  - Analyze personality traits
  - Classify as companion/mercenary/quest-giver
  - Generate trauma responses
```

### Token Optimization:
- Compress HTML before sending
- Use structured prompts
- Bundle similar requests
- Cache common patterns

## Risk Mitigation

### One-Time Generation Risks:
1. **Risk**: Generation might fail
   - **Mitigation**: Checkpoint commit before run
   - **Recovery**: git reset to checkpoint

2. **Risk**: Quality issues
   - **Mitigation**: Sample run first (5 of each type)
   - **Recovery**: Adjust prompts and regenerate

3. **Risk**: Missing relationships
   - **Mitigation**: Validation pass after generation
   - **Recovery**: Manual patches to generated code

### Long-Term Benefits:
1. **No regeneration needed** - It's just Rust code
2. **Direct debugging** - Step through actual spawning
3. **Version control** - Git tracks all changes
4. **Performance** - Everything compiled
5. **Simplicity** - No runtime complexity

## Success Criteria

### Must Have:
- [ ] All 617 hexes spawnable with weather
- [ ] All 335+ dungeon rooms connected
- [ ] All NPCs with basic dialogue
- [ ] All factions with member relationships
- [ ] Asset manifest generated

### Should Have:
- [ ] Quest chains from rumors
- [ ] Companion classifications
- [ ] Weather gameplay effects
- [ ] Treasure placement

### Nice to Have:
- [ ] Rich AI dialogue for all NPCs
- [ ] Dynamic quest generation
- [ ] Faction conspiracy quests

## The Commitment

Once we run the transformer:
1. **The generated code IS the game world**
2. **We delete hexroll-transformer**
3. **All future work is patches to generated code**
4. **No going back to database approach**

This is a one-way door decision, and we're walking through it.

## Memory for Next Agent
This is a fundamental architecture change. We're abandoning the database layer completely in favor of direct ECS components. The HBF data will be transformed ONCE into Rust code that spawns the entire world. This generated code becomes part of the game forever. Future changes are direct edits to that generated code, not regeneration.
