# Architecture Decision: AI Generation vs Direct Implementation

## The Core Insight

After analyzing our needs, we've identified a clear separation:

### Heavy AI Generation Required (Stays in build-tools)
1. **Narrative Content** - Too massive to write manually
   - 12 companions × dialogue trees × 5 dread levels
   - Branching quests with moral complexity
   - Dynamic relationship graphs
   - Context-sensitive responses

2. **Why AI for Narrative?**
   - Exponential branching possibilities
   - Consistency across massive content
   - Psychological depth for each companion
   - Moral nuance in choices

### Direct Implementation (During Asset Curation)
1. **Visual Assets** - Sonnet analyzes and integrates
   - Reviews CC0 assets with vision
   - Generates .bpy scripts for missing pieces
   - Writes ECS integration code

2. **Map Generation** - Algorithms work fine
   - Use `mapgen` crate's algorithms
   - No AI needed for hex layouts
   - Corruption overlays are mathematical

3. **Audio** - Mix existing assets
   - Layer CC0 audio based on dread
   - Spatial audio with bevy_kira_audio
   - No generation needed

## Storage Architecture

### game.db (Ships with Game)
```sql
-- This IS the game's narrative content
dialogues      -- Generated .yarn references
quests         -- Complex quest structures  
story_nodes    -- Cobweb narrative graphs
relationships  -- Companion dynamics

-- This is READ-ONLY in production
```

### player.db (User's XDG Directory)
```sql
-- User-specific data only
save_games     -- Player progress
settings       -- Preferences
achievements   -- Unlocked content
statistics     -- Playtime, choices made
```

## Build Tools Focus

```rust
crates/build-tools/
├── agents/
│   ├── dialogue.rs    // YarnSpinner generation
│   ├── quests.rs      // Quest chain generation
│   ├── relationships.rs // Companion interactions
│   └── story_graph.rs // Cobweb narrative graphs
```

## Asset Curation Process

```bash
# Sonnet with 1M context does:
1. Analyze CC0 assets visually
2. Select appropriate ones
3. Generate integration code
4. Identify gaps
5. Create .bpy scripts for missing pieces
6. Build actual gameplay features
```

## The Key Differentiator

**Narrative** = Metaprompt-heavy, deeply nested, needs AI
**Everything Else** = Can be coded directly during implementation

This is why YarnSpinner integration is critical - it's our bridge between AI-generated narrative content and runtime gameplay.

## Implementation Timeline

### Phase 1: Narrative Generation Pipeline
- Set up YarnSpinner integration
- Build dialogue generation
- Create quest generation
- Store in game.db

### Phase 2: Asset Curation with Sonnet
- Analyze visual assets
- Select and integrate
- Build gameplay systems
- Generate missing pieces

### Phase 3: Direct Implementation
- Wire up mapgen
- Configure audio mixing
- Build corruption shaders
- Complete gameplay loop

## Why This Works

1. **Narrative Complexity** - AI handles the exponential branching
2. **Asset Quality** - Human+AI curation ensures quality
3. **Performance** - Direct implementation is faster than generation
4. **Maintainability** - Clear separation of concerns
5. **Scalability** - Can add more narrative content easily

## The Bottom Line

- **Build-tools** = Narrative content factory (YarnSpinner, Cobweb)
- **Asset curation** = Direct integration with Sonnet's help
- **game.db** = The actual narrative content of the game
- **player.db** = User's personal game state

This gives us the best of both worlds: AI for massive narrative generation, direct implementation for everything else.
