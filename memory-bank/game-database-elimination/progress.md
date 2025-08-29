# Dragon's Labyrinth - Progress Tracker

## Project Status: Phase 9 - Direct ECS World Generation 🚀

### Latest Achievement (2025-08-25 - Major Architecture Pivot)
**ELIMINATING DATABASE LAYER FOR DIRECT ECS**
- ✅ Decision made to eliminate game-database entirely
- ✅ Move all models/systems directly to game-engine as ECS components
- ✅ Discovered we were missing 90% of HBF value (HTML refs, dungeons, factions)
- ✅ Planned one-time world generation approach
- ✅ Designed code generation strategy (generate .rs files, not data)

**CRITICAL HBF DISCOVERIES**
🎯 **HTML Reference Web**: NPCs→Factions, Dungeons→Rooms, all interconnected
🎯 **Complete Dungeons**: Multi-level with stairs, room-by-room monsters, treasure placement
🎯 **Weather Systems**: Seasonal tables with gameplay effects (avalanches, floods)
🎯 **Faction Networks**: Leaders, members, collaborators, conspiracy goals
🎯 **Rich Details**: 617 hexes = 180 levels worth of content ready to use

### The New Architecture

#### What's Changing
- **OUT**: Separate game-database with ORM models
- **OUT**: Runtime data loading from JSON/RON
- **OUT**: Iterative generation approaches
- **IN**: Direct ECS components in game-engine
- **IN**: Generated Rust code files with entity spawning
- **IN**: One-time perfect world generation

#### Why This Works
1. **Small team reality** - We need the rich HBF content, can't write it ourselves
2. **Static world** - The game world doesn't change, only player state
3. **Direct integration** - HBF data becomes our components, not separate data
4. **Maintenance** - Future changes are patches to generated Rust code

### Implementation Plan

#### Phase 1: Enhanced Extraction 🔧
- [ ] Add HTML reference crawling to hexroll-transformer
- [ ] Extract dungeon room connections and stairs
- [ ] Parse complete faction relationship networks
- [ ] Build weather→gameplay effect mappings
- [ ] Extract treasure placement tables

#### Phase 2: Migration & Generation 🔧
- [ ] Move game-database models → game-engine components
- [ ] Move game-database systems → game-engine systems
- [ ] Generate spawn_hexes(), spawn_npcs(), spawn_dungeons() functions
- [ ] Create relationship resolution with HashMap<String, Entity>
- [ ] Generate asset requirement manifests

#### Phase 3: AI Enhancement 🔧
- [ ] Batch NPCs by location for dialogue generation
- [ ] Generate quest chains from rumor connections
- [ ] Create companion trauma responses
- [ ] Build TOML asset requests
- [ ] Bundle API calls for efficiency

#### Phase 4: Execute & Commit 🔧
- [ ] Checkpoint commit before generation
- [ ] Run hexroll-transformer once
- [ ] Verify generated world
- [ ] Final commit with complete world
- [ ] Delete hexroll-transformer (rm -rf)

---

## Previous Achievements (Still Valid)

### Hexroll Transformer Analysis (2025-08-25)
- ✅ New single-pass architecture designed
- ✅ SeaORM schema with 15 tables
- ✅ OpenAI integration for classification
- ✅ Yarn dialogue generation ready
- ⚠️ Needs enhancement for HTML ref crawling

### Third-Party Integrations (Complete)
- ✅ Hexx for hex grid operations
- ✅ bevy_ecs_tilemap for rendering
- ✅ bevy_kira_audio for spatial sound
- ✅ bevy_hanabi for particles
- ✅ pathfinding for AI movement

### Core Systems (Ready for Integration)
- ✅ 180-level horror progression design
- ✅ Companion psychology with trauma
- ✅ 4-Path philosophy framework
- ✅ Environmental decay mechanics
- ✅ Dual Forge system

## Key Metrics

### HBF Content Analysis
- **Total Entities**: 70,801 (68,556 empty placeholders)
- **Content Entities**: 2,245 with actual data
- **Hex Pages**: 617 (enough for entire game)
- **Dungeon Areas**: 335+ rooms
- **Cave Areas**: 254 rooms
- **NPCs**: Hundreds with full D&D stats
- **Factions**: Complete networks with leaders/members
- **Weather Tables**: Regional and seasonal
- **Rumor Tables**: Quest hooks and connections

### Generation Scope
- **Rust Files**: ~20-30 world/*.rs files
- **Components**: ~50 component types
- **Systems**: ~30 game systems
- **Entities**: ~10,000 spawned entities
- **Relationships**: ~50,000 cross-references
- **AI Calls**: ~500 batched requests

## Technical Stack

### Core Engine
- **Bevy**: 0.16.1
- **Hexx**: Hex mathematics
- **bevy_ecs_tilemap**: Tile rendering
- **bevy_kira_audio**: Spatial audio
- **bevy_hanabi**: Particle effects
- **bevy_yarnspinner**: Dialogue

### Transformation Pipeline
- **rusqlite**: HBF reading
- **scraper**: HTML parsing
- **regex**: Pattern matching
- **openai_dive**: AI classification
- **tiktoken-rs**: Token optimization
- **sea-orm**: Temporary for transformation

## Risk Assessment

### Active Risks
- HTML ref crawling complexity
- One-time generation must be perfect
- AI costs for dialogue generation

### Mitigated ✅
- Database complexity eliminated
- No runtime loading issues
- Direct ECS integration

## The Big Picture

We're transforming from a traditional database-backed architecture to a pure ECS world where the HBF data IS our game content. This is a one-time, all-or-nothing transformation that will give us:

1. **Complete World**: 617 hexes, hundreds of dungeons, thousands of NPCs
2. **Rich Interactions**: Faction networks, weather systems, rumor chains
3. **No Maintenance**: Generated code is the game, patch as needed
4. **Performance**: No runtime loading, everything compiled

## Success Criteria

- [ ] All HBF content extracted and transformed
- [ ] Complete ECS world spawnable from generated code
- [ ] All relationships correctly wired
- [ ] Asset manifest covers all needed sprites/models
- [ ] AI-generated dialogue for key NPCs
- [ ] Game is playable with generated world

## Conclusion

This pivot represents a fundamental shift in how we think about the game world. Instead of treating HBF as external data to import, we're making it THE world, generated once into Rust code that becomes part of the game forever.

**Status: READY TO ENHANCE TRANSFORMER AND EXECUTE GENERATION** 🚀
