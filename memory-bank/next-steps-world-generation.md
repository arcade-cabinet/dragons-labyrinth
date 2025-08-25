# Next Steps - World Generation Implementation

## Immediate Actions Required

### Step 1: Fix hexroll-transformer Issues
1. **Fix duplicate PageType enum** in models.rs
2. **Create missing CLI binary** at `src/bin/hexroll_transformer.rs`
3. **Add HTML reference crawler**:
   ```rust
   // Need to extract and follow <a href="/faction/xyz">
   fn crawl_html_refs(html: &str) -> Vec<(String, String)> {
       // Extract all href attributes
       // Return (source_uuid, target_uuid) pairs
   }
   ```

### Step 2: Complete Extraction Pipeline
1. **Extract dungeon connections**:
   - Room-to-room connections
   - Stairs between levels
   - Locked doors and keys
   - Treasure placement

2. **Parse faction networks**:
   - Leader → Members
   - Members → Locations
   - Gathering places
   - Conspiracy goals

3. **Build weather systems**:
   - Seasonal tables
   - Regional variations
   - Gameplay effects

### Step 3: Migrate Models to ECS

#### Move files:
```bash
# Models become components
mv crates/game-database/src/models/hex_tiles.rs \
   crates/game-engine/src/components/world/hex_tiles.rs

mv crates/game-database/src/models/npcs.rs \
   crates/game-engine/src/components/world/npcs.rs

# Systems stay as systems
mv crates/game-database/src/systems/weather/ \
   crates/game-engine/src/systems/weather/
```

#### Transform models:
```rust
// FROM: SeaORM model
#[derive(DeriveEntityModel)]
#[sea_orm(table_name = "hex_tiles")]
pub struct Model {
    pub id: Uuid,
    pub q: i32,
}

// TO: Bevy component
#[derive(Component)]
pub struct HexTile {
    pub q: i32,
    pub r: i32,
    pub s: i32,
}
```

### Step 4: Design Code Generation

#### File Structure:
```
game-engine/src/world/
├── mod.rs           // pub mod and spawn_world()
├── hexes.rs         // spawn_all_hexes()
├── dungeons.rs      // spawn_all_dungeons()
├── npcs.rs          // spawn_all_npcs()
├── factions.rs      // spawn_all_factions()
├── weather.rs       // spawn_all_weather()
├── relationships.rs // wire_all_relationships()
└── assets.rs        // required_assets()
```

#### Generation Template:
```rust
// hexes.rs template
use bevy::prelude::*;
use crate::components::world::*;

pub fn spawn_all_hexes(
    commands: &mut Commands,
    index: &mut HashMap<String, Entity>
) {
    // Generated for each hex
    let hex_n2 = commands.spawn((
        HexTile { q: 0, r: 2, s: -2 },
        Biome::Ice,
        Description("The ice here is blindingly bright..."),
        Weather {
            dry: vec!["Clear", "Cold"],
            wet: vec!["Snow", "Blizzard"],
            special: Some("1-in-6 daily avalanche"),
        },
    )).id();
    index.insert("hex_n2".to_string(), hex_n2);
    
    // ... 616 more hexes
}
```

### Step 5: AI Enhancement Pipeline

#### Prompt Templates:
```yaml
npc_dialogue_batch:
  context: "Horror RPG, growing dread"
  location: "{settlement_name}"
  npcs:
    - name: "{npc_name}"
      personality: "{traits}"
      faction: "{faction_name}"
      fears: "{current_fears}"
  request: "Generate 3-5 lines of dialogue for each NPC"

companion_classification:
  npc:
    name: "{name}"
    description: "{description}"
    stats: "{abilities}"
  classify_as:
    - potential_companion (trauma responses)
    - mercenary (cost and loyalty)
    - quest_giver (rumor/quest)
    - background (ambient dialogue)
```

#### Batching Strategy:
```rust
// Group NPCs by location
let location_groups = group_npcs_by_location(all_npcs);

// Process in batches of 20-30
for (location, npcs) in location_groups {
    if npcs.len() > 30 {
        // Split into smaller batches
    }
    
    let prompt = build_batch_prompt(location, npcs);
    let response = ai_client.generate(prompt).await?;
    
    // Parse and assign dialogue
}
```

### Step 6: One-Time Execution

#### Pre-flight Checklist:
```bash
# 1. Checkpoint commit
git add -A
git commit -m "checkpoint: before world generation"

# 2. Sample run (5 of each type)
cargo run --bin hexroll_transformer -- \
    --sample --limit 5 \
    game.hbf

# 3. Review generated samples
# Adjust prompts if needed

# 4. Full generation
cargo run --bin hexroll_transformer -- \
    --full \
    --output crates/game-engine/src/world/ \
    game.hbf

# 5. Verify compilation
cd crates/game-engine
cargo check

# 6. Final commit
git add -A  
git commit -m "feat: complete world generation from HBF"

# 7. Delete transformer
rm -rf crates/hexroll-transformer
git add -A
git commit -m "cleanup: remove one-time transformer"
```

## Validation Checklist

### Must Validate:
- [ ] All hexes have coordinates
- [ ] All dungeons have at least one entrance
- [ ] All NPCs have names
- [ ] All factions have at least one member
- [ ] No orphaned relationships

### Should Validate:
- [ ] Weather tables complete for all regions
- [ ] Treasure placed in correct rooms
- [ ] Quest items match rumors
- [ ] Faction goals make sense

## Asset Generation

The transformer should produce:
```toml
# crates/game-assets/required_world_assets.toml
[hexes]
ice = 50
desert = 45
jungle = 67
mountain = 38
swamp = 23
plains = 44

[dungeons]
tomb_entrance = 12
cave_mouth = 8
temple_door = 5
trapdoor = 15

[npcs]
rogue = 15
cleric = 8
fighter = 12
merchant = 22
peasant = 45

[monsters]
shadow = { count = 10, cr = "1/2" }
ghoul = { count = 15, cr = "1" }
zombie = { count = 20, cr = "1/4" }
```

## Expected Outcome

After successful generation:
1. **game-engine/src/world/** contains 20-30 .rs files
2. **10,000+ entities** spawnable
3. **All relationships** wired correctly
4. **Assets manifest** lists all needed models/sprites
5. **Game is playable** with generated world

## Common Issues & Solutions

### Issue: Too many API calls
**Solution**: Increase batch size, reduce dialogue length

### Issue: Compilation errors in generated code
**Solution**: Template validation, escape strings properly

### Issue: Missing relationships
**Solution**: Two-pass generation, collect then resolve

### Issue: Performance problems
**Solution**: Lazy spawning, chunk loading

## Memory for Next Agent
The next step is to implement the enhanced hexroll-transformer with HTML ref crawling, then run the one-time world generation. This will create all the Rust code needed for the complete game world. After verification, the transformer gets deleted and we work with the generated code forever.
