# Active Development Context

## Current Status: Major Architecture Pivot - Direct ECS World Generation
**Date**: 2025-08-25
**Phase**: Eliminating Database Layer for Direct ECS Integration

## Architectural Revolution

### The Big Decision
We're **eliminating the game-database layer entirely**. Instead of maintaining a separate database that mirrors ECS, we're building the world directly into Bevy ECS components in `game-engine/src`.

### Why This Change?
1. **Small team reality** - We don't have writers to create content, need the rich HBF world
2. **No need for persistence** - Player saves are separate, the world itself is static
3. **Direct integration** - HBF data should BE our world, not sit alongside it
4. **One-time generation** - Build it once, build it right, then work with Rust code

## Critical HBF Discoveries

### We Were Missing 90% of the Value!
The HBF export contains FAR more than we were extracting:

#### HTML Reference Web
- Every NPC links to their faction
- Faction pages list ALL members and collaborators  
- Dungeon entrances link to specific room areas
- Settlements link to their shops and NPCs
- **We need to crawl these HTML refs to build the relationship graph**

#### Dungeon Complexity
```
- Multi-level dungeons with stairs between floors
- Room-by-room wandering monster tables
- Specific treasure placement (e.g., "Theodora's amulet in area 4")
- Locked doors, barricades, traps per room
- Complete dungeon crawl maps
```

#### Weather as Gameplay
```
- Seasonal weather tables (2d6 rolls)
- Gameplay effects: avalanches, floods, sandstorms
- Visibility and movement modifiers
- Regional variations (dry/wet seasons)
```

#### Faction Networks
```
Example: The Red Snakes
- Leader: Lanthar (Level 6 Rogue with full stats)
- Gathering place: "The Lost Stone Tavern" in Palemoon
- Members: 10 named NPCs with locations
- Collaborators: 5 additional NPCs
- Conspiracy goal: "collect protection money"
```

#### Rich Location Details
```
Cottage example:
- Owner with personality ("seeking solitude", "enraged")
- Faction membership
- Current fears/concerns
- Daily encounter chances
- Full monster stat blocks
```

## The New Plan

### Phase 1: Enhanced Hexroll Transformer
1. **Crawl HTML references** to build complete relationship graph
2. **Extract everything**:
   - Dungeon maps with room connections
   - Monster stat blocks → combat components
   - Faction relationship networks
   - Weather tables → environmental systems
   - Treasure tables → loot systems
   - Rumor connections → quest chains

### Phase 2: One-Time World Generation
1. **Move** `game-database/src/models/` → `game-engine/src/components/`
2. **Move** `game-database/src/systems/` → `game-engine/src/systems/`
3. **Generate** actual `.rs` files with entity spawning code
4. **Create** initialization systems that wire up all relationships
5. **Produce** asset requirement manifests

### Phase 3: AI Enhancement (Bundled)
1. **Batch process** NPCs by location/faction for dialogue
2. **Generate** quest chains from rumor→location→reward
3. **Create** companion-specific trauma responses  
4. **Build** TOML asset request files
5. **Bundle** multiple requests per API call for efficiency

### Phase 4: Commit and Delete
1. **Checkpoint commit** before running
2. **Run transformer once** to generate everything
3. **Verify** the generated world
4. **Final commit** with complete ECS world
5. **Delete** hexroll-transformer crate (rm -rf)

## Technical Approach

### Output Format
Generate actual Rust code files in `game-engine/src/world/`:
```rust
// game-engine/src/world/hexes.rs
pub fn spawn_hexes(commands: &mut Commands, index: &mut HashMap<String, Entity>) {
    let hex_n2 = commands.spawn((
        HexTile { q: 0, r: 2, s: -2 },
        Biome::Ice,
        Weather { table: weather_table_ice() },
        Description("The ice here is blindingly bright..."),
    )).id();
    index.insert("hex_n2".to_string(), hex_n2);
}
```

### Relationship Resolution
Use `HashMap<String, Entity>` during spawn to resolve cross-references:
```rust
// After all entities spawned, wire relationships
if let Some(faction_entity) = index.get("red_snakes") {
    commands.entity(npc_entity).insert(FactionMember(faction_entity));
}
```

### Asset Manifest Generation
```toml
# generated_assets_required.toml
[sprites]
villager_male = 20
villager_female = 18
guard = 15

[models]
cottage = 5
dungeon_entrance = 12
tomb_door = 8

[textures]
ice_hex = 50
desert_hex = 45
```

## Key Insights

### HBF Is Our World
- 617 hex tiles = enough for 180 levels
- 335+ dungeon areas = all dungeons done
- Complete faction networks = social gameplay
- Weather systems = environmental challenges
- Rumor tables = quest generation

### One-Time Is Right-Time
- No idempotency needed
- No runtime loading
- Just generate Rust code once
- All future work is patches to generated code

### AI Optimization Strategy
1. Trial run with samples first
2. Bundle requests by type/location
3. Token-optimize prompts
4. Generate asset requests as part of process

## Next Immediate Steps

1. **Fix hexroll-transformer**:
   - Add HTML ref crawling
   - Extract dungeon connections
   - Parse faction networks
   - Build complete relationship graph

2. **Prepare migration**:
   - Plan component structure
   - Design spawn functions
   - Create relationship resolution

3. **Set up AI pipeline**:
   - Design prompt templates
   - Plan batching strategy
   - Create quality checks

## Memory for Next Agent
**CRITICAL**: We're doing a one-time transformation from HBF → ECS world. The hexroll-transformer will be enhanced to extract ALL the rich data (HTML refs, dungeons, factions, weather), generate Rust code directly into game-engine, then be deleted. This is NOT an iterative process - we build it once, correctly, then work with the generated code forever after.
