# Dragon's Labyrinth Architecture Simplification Plan

## Current Problem Analysis

We've been **over-engineering game logic in Python** instead of leveraging Godot's procedural generation strengths. We should be creating "lego sets + instruction manuals" for Godot, not scripting every detail in Python.

## Proposed Architecture: Two-Command System

### 1. world_building
**Purpose**: Generate world rules and metadata for Godot
**Input**: Raw data (books, linguistic sources, hexroll D&D data)
**Output**: Godot metadata files with rules, not hardcoded content

```
data → seeds → world_rules → godot/metadata/
```

### 2. asset_building  
**Purpose**: Generate only missing visual/audio assets
**Input**: TOML variant specifications
**Output**: Assets that don't already exist in godot/assets/

```
prompts/*.toml → rglob check → generate missing → godot/assets/
```

## Key Architecture Changes

### XDG-Based Idempotency
- Move idempotency.db to XDG user data location
- Separate from game logic completely
- Persistent across projects but not part of game files

### TOML-First Asset Generation
- Consolidate all universal-*.toml files to organized prompts/
- Pre-calculate all possible file paths from variant combinations
- Use rglob to get real-time view of existing vs needed assets
- Only generate what's actually missing

### Godot Rule-Based Systems
Instead of hardcoded encounters/sprites/tiles, provide **rules**:

**Biome Adjacency Rules**:
```json
{
  "plains": {"adjacent_to": ["forest", "river"], "weather": ["clear", "rain"]},
  "forest": {"adjacent_to": ["plains", "mountain"], "weather": ["mist", "rain"]}
}
```

**Path Intelligence**:
```json
{
  "stone_road": {"seeks": ["other_roads"], "avoids": ["water"], "connects_regions": true},
  "wooden_bridge": {"seeks": ["water"], "connects": ["road", "path"]}
}
```

**NPC Dialogue Generation**:
- Give Godot personality patterns + emotional states from seeds
- Let Godot generate contextual dialogue using local rules
- Much more dynamic than pre-scripted content

## Hexroll Data Integration Plan

The hexroll features.json is **incredible treasure**:

### Rich D&D Content Available:
- **Complete Cities**: Population, districts, shops with owners/staff
- **Detailed NPCs**: Stats, emotions, possessions, faction memberships  
- **Living Taverns**: Keepers, staff, menus, lodging, patrons, rumors
- **Monster Encounters**: Full D&D stat blocks with tactics
- **Faction Networks**: "The Swords of Justice", "The Defiled Wolves", etc.
- **Economic Systems**: Pricing, services, trade relationships
- **Social Dynamics**: Rumors, relationships, emotional states

### Integration Strategy:
1. **Extract structured data** from hexroll HTML
2. **Convert to Godot-compatible rules** and templates
3. **Use as seed data** for our horror-themed adaptations
4. **Leverage emotional states** for companion psychology
5. **Adapt faction systems** for philosophy paths

## Simplified Pipeline Architecture

### world_building Subpackage
```
src/dragons_labyrinth/world_building/
├── data/          # Books, NLTK, OMW (unchanged)
├── seeds/         # Semantic extraction (unchanged)  
├── rules/         # Generate Godot rule files
├── hexroll/       # Process hexroll D&D data
└── orchestrator/  # Simple sequential execution
```

### Asset Generation System
```
prompts/
├── biomes/        # All biome variants
├── characters/    # All character variants  
├── effects/       # All effect variants
├── ui/            # All UI variants
└── items/         # All item variants
```

**Asset Building Process**:
1. Parse all TOML files in prompts/
2. Calculate all possible combinations (with exclusions)
3. Generate expected file paths
4. rglob existing files in godot/assets/
5. Generate only what's missing
6. Real-time cost estimation and budgeting

## Godot Advantages We Should Leverage

### Procedural Map Generation
- **Intelligent pathfinding** between start/end biomes per act
- **Rule-based biome placement** with adjacency constraints  
- **Dynamic weather systems** responding to corruption
- **Auto-connecting path elements** that seek each other

### Dynamic Dialogue Systems
- **Personality-driven responses** using emotional patterns
- **Context-aware dialogue trees** based on current state
- **Philosophy-influenced conversation** options
- **Procedural rumor generation** from current world state

### Adaptive Horror Progression  
- **Dread-responsive systems** that scale automatically
- **Corruption effects** on existing content
- **Philosophy-driven narrative** branches
- **Player agency** in meaningful choices

## Implementation Priority

### Phase 1: Simplify Current System
1. Move all universal-*.toml to organized prompts/ structure
2. Create simple world_building command (data → seeds → rules)
3. Create asset_building command with rglob checking
4. Set up XDG idempotency separation

### Phase 2: Harvest Hexroll Data
1. Parse hexroll features.json into structured data
2. Extract NPC personality patterns and emotional systems
3. Convert faction dynamics to philosophy path systems
4. Adapt economic and social systems for horror themes

### Phase 3: Godot Rule Systems
1. Implement intelligent map maker with pathfinding
2. Create dynamic dialogue system using psychology patterns
3. Build adaptive horror progression systems
4. Enable procedural content generation from rules

This approach:
- **Leverages Godot's strengths** in procedural generation
- **Reduces Python complexity** to data processing and rules
- **Maximizes hexroll data value** for rich, dynamic content
- **Enables true procedural horror** that adapts to player choices
- **Scales to 180 levels** without hardcoding every detail
