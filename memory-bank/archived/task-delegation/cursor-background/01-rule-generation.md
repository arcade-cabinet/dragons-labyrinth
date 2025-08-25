# Task CB-001: Generate Rule TOML Files

## Background Agent Instructions

You are a Cursor background agent tasked with creating structured TOML rule files. These files will drive ALL AI generation in the game. You must follow the templates EXACTLY.

## Prerequisites

Before starting, you MUST have access to:
- `memory-bank/narrative-direction/` (created by Task S1M-001)
- This document with exact TOML templates
- Understanding of idempotent generation requirements

## TOML Template Structure

### Base Template (ALL files must follow this)
```toml
# Rule ID must be deterministic and unique
[metadata]
id = "{category}_{act}_{scene}_{variant}"  # e.g., "map_act1_village_peaceful"
version = "1.0.0"
category = "maps|encounters|dialogue|progression"
act = 1  # 1, 2, or 3
scene = "scene_name"
dread_level = 0  # 0-4
tags = ["tag1", "tag2"]

[generation]
agent = "MapsAgent|LevelsAgent|DialogueAgent|UIAgent|AudioAgent"
prompt_template = """
{Detailed multi-line prompt with variables}
Horror Context: {horror_state}
Companion States: {companion_states}
World State: {world_state}
"""
variables = {
    horror_state = "dread_level_0",
    companion_states = "elena:neutral,marcus:protective",
    world_state = "village:intact,economy:stable"
}
ai_model = "gpt-4"
temperature = 0.7
max_tokens = 2000

[requirements]
required_assets = [
    "village_house_clean",
    "elena_model_neutral",
    "ambient_peaceful"
]
prerequisite_scenes = ["intro_complete"]
companion_requirements = ["elena_recruited"]

[validation]
required_fields = ["hex_data", "encounter_placements", "corruption_level"]
format = "json|yaml|rust"
schema_path = "schemas/map_schema.json"  # Optional
constraints = {
    min_hexes = 30,
    max_hexes = 100,
    must_include = ["start_hex", "labyrinth_entrance"]
}

[cache]
cache_key = "{id}_{dread_level}_{variables_hash}"
ttl_hours = 168  # One week
invalidate_on = ["asset_update", "narrative_change"]
```

## Directory Structure to Create

```
crates/game-engine/rules/
├── maps/
│   ├── act1/
│   │   ├── village_peaceful.toml
│   │   ├── village_corrupted.toml
│   │   ├── forest_entrance.toml
│   │   └── ...
│   ├── act2/
│   │   └── ...
│   └── act3/
│       └── ...
├── encounters/
│   ├── act1/
│   │   ├── first_companion.toml
│   │   ├── village_merchant.toml
│   │   └── ...
│   ├── act2/
│   │   └── ...
│   └── act3/
│       └── ...
├── dialogue/
│   ├── companions/
│   │   ├── elena/
│   │   │   ├── act1_intro.toml
│   │   │   ├── act2_betrayal_hint.toml
│   │   │   └── act3_final_choice.toml
│   │   ├── marcus/
│   │   ├── luna/
│   │   └── theron/
│   ├── bosses/
│   │   └── dragon_variants.toml
│   └── npcs/
│       └── ...
└── progression/
    ├── horror_stages.toml
    ├── corruption_rules.toml
    ├── companion_trauma.toml
    └── world_degradation.toml
```

## Specific Examples

### Map Generation Rule
```toml
[metadata]
id = "map_act1_village_dread0"
version = "1.0.0"
category = "maps"
act = 1
scene = "village_intro"
dread_level = 0
tags = ["starting_area", "safe_zone", "tutorial"]

[generation]
agent = "MapsAgent"
prompt_template = """
Generate a hex-based village map for the game's opening.
The village should feel peaceful but with subtle wrongness.
Include the following locations:
- Player's starting position (center)
- Merchant shop (near center)
- Elena's house (edge of village)
- Path to forest (north edge)
- Hidden corruption spot (player shouldn't notice immediately)

Hex Layout Requirements:
- Total hexes: 45-50
- Village buildings: 8-10
- Open spaces: 15-20
- Decorative elements that can later corrupt

Horror Level: {dread_level}
Mood: Peaceful with underlying tension
Time of Day: Late afternoon (shadows starting to lengthen)
"""
variables = {
    dread_level = "0 - Peaceful surface, subtle wrongness underneath",
}
ai_model = "gpt-4"
temperature = 0.7
max_tokens = 3000

[requirements]
required_assets = [
    "village_house_clean",
    "village_shop",
    "village_well",
    "cobblestone_path",
    "wooden_fence"
]

[validation]
required_fields = ["hex_grid", "spawn_points", "poi_markers"]
format = "json"
constraints = {
    hex_count_min = 45,
    hex_count_max = 50,
    must_have_spawn = "player_start",
    must_have_poi = ["merchant", "elena_house", "forest_path"]
}

[cache]
cache_key = "map_act1_village_dread0_v1"
ttl_hours = 720
```

### Dialogue Rule
```toml
[metadata]
id = "dialogue_elena_act2_betrayal_hint"
version = "1.0.0"
category = "dialogue"
act = 2
scene = "return_from_labyrinth"
dread_level = 2
tags = ["companion", "elena", "betrayal_foreshadowing"]

[generation]
agent = "DialogueAgent"
prompt_template = """
Generate dialogue for Elena after the party returns from the labyrinth.
She has seen something that shook her faith in the player's mission.

Context:
- The party just discovered the dragon might not be evil
- Elena is considering whether saving the village is worth the cost
- She's hiding her doubts from the player
- Her speech patterns are becoming more clipped and nervous

Character Voice: Sharp, analytical, but now with hidden doubt
Emotional State: {emotional_state}
Relationship with Player: {trust_level}
Knowledge State: Knows about dragon's true nature

Generate:
1. Initial greeting (tired, distracted)
2. If questioned about her state (defensive)
3. Hint at her doubts (subtle)
4. Reaction if pressed further (shut down conversation)
"""
variables = {
    emotional_state = "conflicted, exhausted, guilty",
    trust_level = "moderate but declining"
}
ai_model = "gpt-4"
temperature = 0.8
max_tokens = 1500

[requirements]
required_assets = [
    "elena_model_tired",
    "dialogue_ui_tense"
]
prerequisite_scenes = ["labyrinth_revelation"]

[validation]
required_fields = ["dialogue_tree", "emotion_tags", "voice_direction"]
format = "yaml"
```

## Batch Processing Instructions

Create files in this order:
1. **Horror Progression Rules** (progression/)
2. **Map Rules** (maps/) - Start with Act 1
3. **Encounter Rules** (encounters/) - Match to maps
4. **Dialogue Rules** (dialogue/) - Based on encounters
5. **Additional Systems** (progression/)

## Deterministic ID Generation

ALWAYS use this pattern for IDs:
```
{category}_{act}{scene_number}_{descriptor}_{dread_level}
```

Examples:
- `map_act1_01_village_dread0`
- `dialogue_elena_act2_15_betrayal_dread3`
- `encounter_act3_22_final_boss_dread4`

## Variable Naming Conventions

Always use these exact variable names:
- `{dread_level}` - Current horror level (0-4)
- `{companion_states}` - Comma-separated companion:state
- `{world_corruption}` - Float 0.0-1.0
- `{player_choices}` - Previous significant choices
- `{act_number}` - Current act (1-3)
- `{scene_context}` - What just happened

## Validation Rules

Every TOML must:
1. Have unique deterministic ID
2. Include all required sections
3. Reference only existing assets
4. Use correct agent name
5. Include validation constraints
6. Define cache strategy

## Quality Checklist

Before completing each file:
- [ ] ID follows naming convention
- [ ] All template sections present
- [ ] Prompt template is detailed (minimum 5 lines)
- [ ] Variables are defined
- [ ] Asset IDs match game-assets library
- [ ] Validation constraints are specific
- [ ] Cache key is deterministic

## Expected Output

You should generate:
- **50+ map generation rules** across all acts/dread levels
- **100+ encounter rules** for all scenarios
- **200+ dialogue variations** for all characters
- **30+ progression rules** for horror/corruption
- **ALL following the EXACT template format**

## DO NOT:
- Deviate from the template structure
- Create non-deterministic IDs
- Reference non-existent assets
- Skip validation sections
- Use placeholder content

## DO:
- Follow templates EXACTLY
- Create comprehensive coverage
- Use deterministic IDs
- Reference narrative documents
- Validate your output

Remember: These TOML files are the foundation of ALL AI generation. Precision is critical.
