# Dragon's Labyrinth - TOML Rule Templates

## Overview

This document provides complete templates for TOML rule generation by Cursor Background Agents (CB-001). These templates ensure consistent formatting, idempotent generation, and proper integration with the AI agent system.

**Critical Requirements**:
- Deterministic IDs using consistent hashing
- Horror progression awareness in all rules
- Asset requirements explicitly specified
- Validation constraints clearly defined

## Universal Template Structure

All TOML files follow this base structure:

```toml
[metadata]
id = "deterministic_id_from_content_hash"
name = "Human Readable Name" 
description = "Brief description of what this rule generates"
version = "1.0.0"
created_by = "background_agent_cb_001"
dread_level = 0  # 0-4 horror progression level
act = 1  # 1-3 narrative act
priority = 1  # 1-10 generation priority

[generation]
agent = "maps|levels|ui|dialogue|audio"
model = "gpt-4o-mini"
temperature = 0.7
max_tokens = 2000

prompt_template = """
Detailed generation prompt with {variables}
Context: {horror_context}
Dread Level: {dread_level}
Companions: {companion_states}
"""

[assets]
# Required assets for this generation
models = []
textures = []  
audio = []
ui_elements = []

[validation]
required_fields = []
constraints = {}
success_criteria = []

[horror_progression]
triggers = []
corruption_effects = []
companion_impacts = {}
```

## Map Generation Rules

### Template: Hex World Generation
```toml
[metadata]
id = "hex_world_gen_{biome}_{dread_level}"
name = "Hex World: {biome} (Dread {dread_level})"
description = "Generate hex-based world for {biome} at dread level {dread_level}"
version = "1.0.0"
created_by = "background_agent_cb_001"
dread_level = {dread_level}
act = {act}
priority = {priority}

[generation]
agent = "maps"
model = "gpt-4o-mini"
temperature = 0.7
max_tokens = 2000

prompt_template = """
Generate a hex-based world map for Dragon's Labyrinth.

CONTEXT:
- Biome: {biome}
- Dread Level: {dread_level}/4
- Act: {act}/3
- Horror Theme: {horror_theme}

REQUIREMENTS:
- Use Hexx coordinate system (axial coordinates)
- Generate {hex_count} total hexes
- Include pathfinding weights based on terrain
- Corruption level increases with dread: {corruption_level}

HEX TYPES REQUIRED:
{hex_types}

HORROR INTEGRATION:
- Dread 0: Beautiful but subtle wrongness
- Dread 1: Obvious signs of decay
- Dread 2: Heavy corruption, reality distorts
- Dread 3: Nightmare landscape
- Dread 4: Impossible geometry, void zones

Generate Rust code for Bevy that spawns this world using HexPosition components.
Include asset references for each hex tile type.
"""

variables = [
    "biome",
    "dread_level", 
    "act",
    "horror_theme",
    "hex_count",
    "corruption_level",
    "hex_types"
]

[assets]
models = [
    "{biome}_hex_base.glb",
    "{biome}_hex_corrupted.glb", 
    "{biome}_props_peaceful.glb",
    "{biome}_props_corrupted.glb"
]
textures = [
    "{biome}_base_albedo.png",
    "{biome}_corruption_overlay.png",
    "{biome}_splatmap.png"
]
audio = [
    "{biome}_ambience_dread_{dread_level}.ogg",
    "{biome}_footsteps.ogg"
]

[validation]
required_fields = [
    "hex_layout",
    "tile_assignments", 
    "spawn_point",
    "pathfinding_weights"
]
constraints = {
    max_hexes = 200,
    min_hexes = 50,
    corruption_range = [0.0, 1.0]
}
success_criteria = [
    "All hexes have valid coordinates",
    "Pathfinding graph is connected",
    "Asset references are valid"
]

[horror_progression]
triggers = [
    "player_enters_corrupted_hex",
    "hex_corruption_spreads",
    "reality_distortion_zones"
]
corruption_effects = [
    "hex_texture_darkens",
    "ambient_audio_distorts", 
    "pathfinding_becomes_maze"
]
companion_impacts = {
    Einar = "protective_instinct_triggered",
    Mira = "forces_optimism_about_beauty",
    Sorin = "takes_notes_obsessively",
    Tamara = "notices_wrongness_first"
}
```

### Template: Encounter Placement
```toml
[metadata]
id = "encounter_{encounter_type}_{location}_{dread_level}"
name = "Encounter: {encounter_type} at {location}"
description = "Generate {encounter_type} encounter at {location} for dread {dread_level}"
version = "1.0.0"
created_by = "background_agent_cb_001"
dread_level = {dread_level}
act = {act}
priority = {priority}

[generation]
agent = "levels"
model = "gpt-4o-mini"
temperature = 0.7
max_tokens = 2000

prompt_template = """
Generate an encounter for Dragon's Labyrinth using Yoleck integration.

ENCOUNTER DETAILS:
- Type: {encounter_type}
- Location: {location}
- Dread Level: {dread_level}/4
- Moral Complexity: {moral_complexity}

YOLECK REQUIREMENTS:
- Generate .yol file format (JSON entity descriptions)
- Include entity positions, properties, and behaviors
- Support in-engine editing with egui widgets
- Reference existing asset library when possible

HORROR INTEGRATION:
- Every encounter serves horror progression
- Moral choices have lasting consequences
- Companion reactions based on trauma levels
- Reality becomes less reliable at higher dread

COMPANION STATES:
{companion_states}

Generate both the Yoleck entity definitions and the Rust encounter logic.
"""

[assets]
models = [
    "{encounter_type}_set.glb",
    "{location}_environment.glb",
    "moral_choice_indicators.glb"
]
audio = [
    "{encounter_type}_intro.ogg",
    "{encounter_type}_resolution.ogg",
    "moral_choice_stinger.ogg"
]

[validation]
required_fields = [
    "encounter_entities",
    "moral_choices",
    "companion_reactions",
    "consequence_effects"
]
constraints = {
    max_entities = 20,
    moral_complexity_range = [1, 5]
}

[horror_progression]
triggers = [
    "moral_choice_made",
    "companion_trauma_increased",
    "reality_questioned"
]
companion_impacts = {
    Einar = "judges_player_morality",
    Mira = "tries_to_find_bright_side",
    Sorin = "analyzes_philosophical_implications", 
    Tamara = "innocence_affected"
}
```

## Dialogue System Rules

### Template: Companion Dialogue Trees
```toml
[metadata]
id = "dialogue_{companion}_{emotional_state}_{dread_level}"
name = "Dialogue: {companion} ({emotional_state})"
description = "Generate dialogue tree for {companion} in {emotional_state} at dread {dread_level}"
version = "1.0.0"
created_by = "background_agent_cb_001"
dread_level = {dread_level}
act = {act}
priority = {priority}

[generation]
agent = "dialogue"
model = "gpt-4o-mini"
temperature = 0.8  # Higher temperature for dialogue variety
max_tokens = 3000

prompt_template = """
Generate YarnSpinner dialogue tree for Dragon's Labyrinth.

COMPANION PROFILE:
- Name: {companion}
- Emotional State: {emotional_state}
- Trauma Level: {trauma_level}/10
- Trust Level: {trust_level}/10
- Current Arc: {character_arc_stage}

VOICE CHARACTERISTICS:
{voice_characteristics}

DREAD CONTEXT:
- Level: {dread_level}/4
- World State: {world_state}
- Recent Events: {recent_events}

YARN SPINNER REQUIREMENTS:
- Use bevy_yarnspinner compatible format
- Include character state tracking
- Support branching based on player history
- Include emotional effect tracking

HORROR INTEGRATION:
- Dialogue reflects psychological impact
- References growing wrongness in world
- Shows character degradation over time
- Includes false dialogue options at high dread

Generate complete .yarn file with multiple conversation nodes.
"""

[assets]
audio = [
    "{companion}_voice_{emotional_state}_01.ogg",
    "{companion}_voice_{emotional_state}_02.ogg",
    "{companion}_voice_{emotional_state}_03.ogg",
    "{companion}_ambient_breathing.ogg"
]
ui_elements = [
    "{companion}_portrait_{emotional_state}.png",
    "dialogue_box_dread_{dread_level}.png"
]

[validation]
required_fields = [
    "yarn_nodes",
    "character_state_updates",
    "branching_logic",
    "emotional_effects"
]
constraints = {
    min_dialogue_options = 2,
    max_dialogue_options = 4,
    trauma_range = [0.0, 10.0]
}

[horror_progression]
triggers = [
    "companion_mentions_wrongness",
    "trust_threshold_changed",
    "trauma_milestone_reached"
]
companion_impacts = {
    self = "emotional_state_evolution",
    others = "group_dynamic_effects"
}
```

### Template: Moral Choice Dialogues
```toml
[metadata]
id = "moral_choice_{scenario}_{dread_level}"
name = "Moral Choice: {scenario}"
description = "Generate moral choice dialogue for {scenario} at dread {dread_level}"
version = "1.0.0"
created_by = "background_agent_cb_001"
dread_level = {dread_level}
act = {act}
priority = {priority}

[generation]
agent = "dialogue"
model = "gpt-4o-mini"
temperature = 0.7
max_tokens = 2500

prompt_template = """
Generate a moral choice dialogue sequence for Dragon's Labyrinth.

SCENARIO: {scenario}
MORAL COMPLEXITY: {complexity_level}/5
DREAD LEVEL: {dread_level}/4

CHOICE FRAMEWORK:
- No clearly "right" answers at higher dread levels
- Each choice has meaningful consequences
- Companions react based on their values/trauma
- Long-term impact on world state

HORROR ELEMENTS:
- Choices become more disturbing as dread increases
- Reality of consequences may be unclear
- Companion reactions show their psychological state
- Player's growing mark/corruption influences options

COMPANION REACTIONS REQUIRED:
{companion_reactions}

Generate YarnSpinner dialogue with:
1. Context setup
2. Choice presentation
3. Immediate reactions
4. Consequence revelation
5. Long-term relationship effects
"""

[validation]
required_fields = [
    "choice_options",
    "immediate_consequences", 
    "long_term_effects",
    "companion_reactions"
]
constraints = {
    min_choices = 2,
    max_choices = 3,
    consequence_severity_range = [1, 5]
}

[horror_progression]
triggers = [
    "moral_boundary_crossed",
    "companion_trust_lost",
    "world_corruption_increased"
]
corruption_effects = [
    "dialogue_options_become_darker",
    "companion_voices_distort",
    "reality_of_choices_questioned"
]
```

## Sentimental Items System Rules

### Template: Sentimental Item Definitions
```toml
[metadata]
id = "sentimental_item_{item_name}_{collection_act}"
name = "Sentimental Item: {item_name}"
description = "Generate sentimental item {item_name} collected in act {collection_act}"
version = "1.0.0"
created_by = "background_agent_cb_001"
dread_level = {collection_dread_level}
act = {collection_act}
priority = {priority}

[generation]
agent = "ui"  # Items are UI/inventory related
model = "gpt-4o-mini"
temperature = 0.6
max_tokens = 1500

prompt_template = """
Generate a sentimental item for Dragon's Labyrinth.

ITEM PROFILE:
- Name: {item_name}
- Collection Context: {collection_context}
- Apparent Purpose: {apparent_purpose}
- True Purpose: {forge_purpose}
- Emotional Weight: {emotional_significance}

SENTIMENTAL MECHANICS:
- Cannot be discarded (inventory shows grayed out)
- Gains "memories" from significant events
- Purpose revealed only at forge
- Creates "aha!" moment of realization

MEMORY SYSTEM:
- Track events where item was present
- Build emotional attachment through use
- Connect to companion relationships
- Reflect journey's moral choices

FORGE INTEGRATION:
- Light Forge: Item willingly transforms
- Dark Forge: Item must be sacrificed
- Enhancement depends on memories accumulated
- Final gear reflects entire journey

Generate Rust item struct with memory tracking and forge integration.
"""

[assets]
models = [
    "{item_name}_clean.glb",
    "{item_name}_corrupted.glb",
    "{item_name}_forge_ready.glb"
]
textures = [
    "{item_name}_base.png",
    "{item_name}_memory_glow.png"
]
ui_elements = [
    "{item_name}_icon.png",
    "{item_name}_tooltip_background.png"
]

[validation]
required_fields = [
    "item_properties",
    "memory_tracking",
    "forge_aspects",
    "emotional_connections"
]
constraints = {
    max_memories = 10,
    emotional_weight_range = [0.0, 1.0],
    forge_power_range = [1, 5]
}

[forge_integration]
light_forge_enhancement = "{light_enhancement}"
dark_forge_enhancement = "{dark_enhancement}"
required_memories = {min_memories}
mythic_tier_requirement = true

[horror_progression]
corruption_effects = [
    "item_appearance_darkens",
    "memories_become_nightmares", 
    "forge_purpose_reveals_horror"
]
companion_impacts = {
    all = "react_to_item_attachment"
}
```

### Template: Memory Event Triggers
```toml
[metadata]
id = "memory_event_{event_type}_{trigger_scene}"
name = "Memory Event: {event_type}"
description = "Generate memory event for sentimental items during {trigger_scene}"
version = "1.0.0"
created_by = "background_agent_cb_001"
dread_level = {dread_level}
act = {act}
priority = 5  # Memory events are important but not critical path

[generation]
agent = "ui"
model = "gpt-4o-mini"
temperature = 0.7
max_tokens = 1200

prompt_template = """
Generate a memory event for sentimental items in Dragon's Labyrinth.

EVENT CONTEXT:
- Type: {event_type}
- Scene: {trigger_scene}
- Emotional Impact: {emotional_impact}
- Items Affected: {affected_items}

MEMORY MECHANICS:
- Items "witness" significant events
- Build emotional weight over time
- Connect player choices to forge power
- Create narrative continuity

HORROR INTEGRATION:
- Early memories are beautiful/innocent
- Later memories show corruption/trauma
- Final memories may be nightmarish
- Forge reveals true meaning

Generate memory data structure and integration logic.
"""

[validation]
required_fields = [
    "event_trigger",
    "affected_items",
    "memory_content",
    "emotional_weight"
]
constraints = {
    emotional_impact_range = [0.1, 1.0],
    max_items_per_event = 3
}

[memory_content]
description_template = "{emotional_descriptor} memory of {event_description}"
visual_representation = "{visual_memory_cue}"
companion_involvement = {companion_reactions}
player_choice_reflection = "{choice_impact}"
```

## Horror Progression Rules

### Template: Dread Level Transitions
```toml
[metadata]
id = "dread_transition_{from_level}_to_{to_level}"
name = "Dread Transition: Level {from_level} → {to_level}"
description = "Generate transition from dread level {from_level} to {to_level}"
version = "1.0.0"
created_by = "background_agent_cb_001"
dread_level = {to_level}
act = {act}
priority = 10  # Highest priority - drives all other systems

[generation]
agent = "maps"  # Maps agent handles world state
model = "gpt-4o-mini"
temperature = 0.6
max_tokens = 2500

prompt_template = """
Generate dread level transition for Dragon's Labyrinth.

TRANSITION: {from_level} → {to_level}
NARRATIVE TRIGGER: {trigger_event}
WORLD IMPACT: {world_changes}

SYSTEM EFFECTS:
- Visual corruption increases by {corruption_increase}
- Audio environment shifts to {audio_shift}
- Companion behavior changes: {companion_changes}
- Player power feeling: {power_feeling}

HORROR MANIFESTATION:
{horror_manifestations}

TRANSITION REQUIREMENTS:
- Gradual change over {transition_duration} 
- All systems must respond to new dread level
- Companion trauma accumulates
- World corruption spreads
- Player agency feels reduced

Generate transition logic, asset swaps, and system updates.
"""

[assets]
models = [
    "world_corruption_mask_{to_level}.glb",
    "reality_distortion_effects_{to_level}.glb"
]
textures = [
    "corruption_overlay_{to_level}.png", 
    "atmosphere_shift_{to_level}.png"
]
audio = [
    "dread_transition_{from_level}_to_{to_level}.ogg",
    "ambient_shift_{to_level}.ogg",
    "reality_crack_stinger.ogg"
]

[validation]
required_fields = [
    "trigger_conditions",
    "transition_effects",
    "system_updates",
    "companion_reactions"
]
constraints = {
    transition_duration_seconds = 30.0,
    corruption_increase_range = [0.1, 0.3],
    companion_trauma_increase = 1.0
}

[horror_progression]
world_changes = [
    "sky_color_shift",
    "vegetation_corruption",
    "architecture_decay",
    "npc_behavior_change"
]
audio_changes = [
    "ambient_layer_swap",
    "music_key_change",
    "new_horror_sounds",
    "companion_voice_distortion"
]
visual_changes = [
    "lighting_temperature_shift",
    "shadow_length_increase", 
    "texture_desaturation",
    "geometry_distortion"
]

[companion_impacts]
Einar = {
    trauma_increase = 1.0,
    behavior_change = "more_protective_desperate",
    dialogue_shift = "suggests_retreat_more"
}
Mira = {
    trauma_increase = 1.5,  # Most affected by world corruption
    behavior_change = "forced_optimism_cracks",
    dialogue_shift = "nervous_laughter_increases"
}
Sorin = {
    trauma_increase = 0.8,  # Academic curiosity shields him initially
    behavior_change = "obsessive_note_taking",
    dialogue_shift = "theories_become_frantic"
}
Tamara = {
    trauma_increase = 2.0,  # Innocence is most vulnerable
    behavior_change = "selective_mutism_begins",
    dialogue_shift = "questions_become_whispers"
}
```

### Template: Corruption Manifestations
```toml
[metadata]
id = "corruption_{manifestation_type}_{affected_system}"
name = "Corruption: {manifestation_type} in {affected_system}"
description = "Generate corruption manifestation {manifestation_type} affecting {affected_system}"
version = "1.0.0"
created_by = "background_agent_cb_001"
dread_level = {required_dread_level}
act = {act}
priority = 8

[generation]
agent = "maps"
model = "gpt-4o-mini"
temperature = 0.7
max_tokens = 1800

prompt_template = """
Generate corruption manifestation for Dragon's Labyrinth.

MANIFESTATION: {manifestation_type}
AFFECTED SYSTEM: {affected_system}
INTENSITY: {intensity_level}/5
SPREAD PATTERN: {spread_pattern}

CORRUPTION RULES:
- Player proximity can trigger corruption
- Dragon influence spreads corruption
- Moral choices accelerate corruption
- Some corruption is permanent

VISUAL EFFECTS:
{visual_effects}

AUDIO EFFECTS:
{audio_effects}

GAMEPLAY IMPACT:
{gameplay_impact}

Generate corruption system that integrates with all game systems.
"""

[assets]
models = [
    "{manifestation_type}_stage_1.glb",
    "{manifestation_type}_stage_2.glb", 
    "{manifestation_type}_stage_3.glb"
]
textures = [
    "{manifestation_type}_progression.png",
    "{manifestation_type}_mask.png"
]
audio = [
    "{manifestation_type}_corruption_sound.ogg",
    "{manifestation_type}_ambient_distortion.ogg"
]

[validation]
required_fields = [
    "corruption_stages",
    "spread_logic",
    "visual_progression",
    "audio_progression"
]
constraints = {
    intensity_range = [1, 5],
    spread_rate_range = [0.1, 2.0],
    max_affected_hexes = 50
}

[corruption_mechanics]
trigger_conditions = [
    "player_proximity",
    "dragon_influence", 
    "moral_corruption",
    "time_based_spread"
]
progression_stages = {
    stage_1 = "subtle_wrongness",
    stage_2 = "obvious_decay",
    stage_3 = "reality_distortion"
}
reversal_conditions = [
    "player_purification_action",
    "companion_sacrifice",
    "forge_blessing"
]
```

## Forge System Rules

### Template: Forge Trial Generation
```toml
[metadata]
id = "forge_trial_{trial_type}_{forge_path}"
name = "Forge Trial: {trial_type} ({forge_path})"
description = "Generate {trial_type} trial for {forge_path} forge path"
version = "1.0.0"
created_by = "background_agent_cb_001"
dread_level = 4  # Always maximum dread at forge
act = 3
priority = 10  # Critical endgame content

[generation]
agent = "levels"  # Trials are complex encounter sequences
model = "gpt-4o"  # Use full model for endgame complexity
temperature = 0.6
max_tokens = 4000

prompt_template = """
Generate forge trial for Dragon's Labyrinth endgame.

TRIAL TYPE: {trial_type}
FORGE PATH: {forge_path}
DIFFICULTY: {difficulty_level}

TRIAL PHILOSOPHY:
- Light Forge: Tests worthiness through love/compassion
- Dark Forge: Tests power through domination/sacrifice
- Must test ALL learned game mechanics
- Companion involvement affects difficulty

MECHANICAL INTEGRATION:
- Hex navigation under pressure
- Combat mastery requirements  
- Moral choice under duress
- System knowledge application

COMPANION INVOLVEMENT:
{companion_states}

SENTIMENTAL ITEMS:
{item_requirements}

SUCCESS CONDITIONS:
{success_criteria}

Generate complete trial sequence with branching outcomes.
"""

[assets]
models = [
    "{forge_path}_forge_environment.glb",
    "{trial_type}_challenge_props.glb",
    "mythic_crafting_animations.glb"
]
textures = [
    "{forge_path}_forge_atmosphere.png",
    "{trial_type}_environmental_effects.png"
]
audio = [
    "{forge_path}_forge_ambience.ogg",
    "{trial_type}_trial_music.ogg",
    "mythic_crafting_sounds.ogg",
    "companion_trial_voices.ogg"
]

[validation]
required_fields = [
    "trial_stages",
    "difficulty_scaling",
    "companion_integration",
    "success_outcomes",
    "failure_consequences"
]
constraints = {
    min_trial_duration_minutes = 10,
    max_trial_duration_minutes = 30,
    companion_involvement_range = [0.0, 1.0],
    skill_requirements = ["hex_navigation", "combat", "moral_judgment"]
}

[forge_mechanics]
light_forge_requirements = {
    companion_loyalty_minimum = 7.0,
    player_compassion_actions = 5,
    sentimental_items_willing = true,
    trial_approach = "cooperative"
}
dark_forge_requirements = {
    player_power_actions = 5,
    sacrifice_willingness = true,
    sentimental_items_consumed = true,
    trial_approach = "domination"
}

[mythic_tier_crafting]
light_path_result = {
    prefix = "Blessed",
    special_abilities = ["healing_aura", "void_immunity", "companion_inspiration"],
    appearance = "radiant_golden_glow"
}
dark_path_result = {
    prefix = "Cursed",
    special_abilities = ["life_drain", "fear_aura", "reality_rend"],
    appearance = "writhing_shadows"
}

[horror_integration]
trial_corruption_effects = [
    "reality_becomes_unstable",
    "companion_voices_distorted",
    "moral_choices_have_immediate_horror"
]
success_horror_impact = {
    light_forge = "world_corruption_slowed",
    dark_forge = "world_corruption_embraced"
}
```

## Economic System Rules

### Template: Economic Collapse Events
```toml
[metadata]
id = "economic_event_{event_type}_{severity_level}"
name = "Economic Event: {event_type}"
description = "Generate economic collapse event {event_type} at severity {severity_level}"
version = "1.0.0"
created_by = "background_agent_cb_001"
dread_level = {trigger_dread_level}
act = 2  # Economic collapse happens in Act 2
priority = 7

[generation]
agent = "levels"  # Economic events affect world encounters
model = "gpt-4o-mini"
temperature = 0.7
max_tokens = 2000

prompt_template = """
Generate economic collapse event for Dragon's Labyrinth.

EVENT: {event_type}
SEVERITY: {severity_level}/5  
TRIGGER: {trigger_condition}
WORLD IMPACT: {world_impact}

ECONOMIC MECHANICS:
- Dragon influence disrupts trade
- Fear causes hoarding/fleeing
- Currency becomes worthless
- Barter system emerges
- Essential goods become precious

HORROR INTEGRATION:
- Economy reflects growing dread
- NPCs prioritize survival over profit
- Moral choices around scarce resources
- Companion reactions to desperation

WORLD CHANGES:
{world_changes}

Generate economic event system with NPC behavior updates.
"""

[assets]
models = [
    "abandoned_shop_{severity_level}.glb",
    "refugee_camp_props.glb",
    "barricaded_buildings.glb"
]
textures = [
    "economic_decay_{severity_level}.png",
    "boarded_windows.png"
]
audio = [
    "empty_marketplace.ogg",
    "desperate_voices.ogg",
    "economic_collapse_ambience.ogg"
]

[validation]
required_fields = [
    "economic_impact",
    "npc_behavior_changes",
    "world_visual_changes",
    "player_interaction_changes"
]
constraints = {
    severity_range = [1, 5],
    affected_npcs_percentage = [0.2, 0.8],
    economic_recovery_impossible = true
}

[economic_mechanics]
currency_value_multiplier = 0.1  # Gold becomes nearly worthless
essential_goods = [
    "food",
    "medicine", 
    "weapons",
    "warm_clothing"
]
barter_ratios = {
    "1_sword" = "10_days_food",
    "1_healing_potion" = "1_week_shelter",
    "1_magic_item" = "safe_passage"
}

[world_changes]
shop_availability = {
    luxury_goods = 0.0,
    basic_goods = 0.3,
    essential_goods = 0.1
}
npc_behavior_changes = [
    "merchants_flee_or_hide",
    "guards_abandon_posts", 
    "citizens_board_windows",
    "refugees_flood_roads"
]
infrastructure_decay = [
    "bridges_unrepaired",
    "roads_become_dangerous",
    "communication_breaks_down"
]

[companion_impacts]
Einar = "guilt_about_not_protecting_economy"
Mira = "forces_optimism_about_recovery"
Sorin = "analyzes_economic_theory_frantically"
Tamara = "doesnt_understand_why_people_are_mean"
```

## Audio System Rules

### Template: Proximity Horror Audio
```toml
[metadata]
id = "proximity_audio_dragon_{distance_category}"
name = "Dragon Proximity Audio: {distance_category}"
description = "Generate dragon proximity audio for {distance_category} distance"
version = "1.0.0"
created_by = "background_agent_cb_001"
dread_level = 4  # Only active at maximum horror
act = 3
priority = 9  # Critical for horror experience

[generation]
agent = "audio"
model = "gpt-4o-mini"
temperature = 0.8  # High creativity for horror audio
max_tokens = 1500

prompt_template = """
Generate proximity horror audio for Dragon's Labyrinth.

DISTANCE: {distance_category}
DRAGON STATE: {dragon_state}
PLAYER FEAR: {fear_level}/10

PROXIMITY CATEGORIES:
- Very Far: Barely audible hints
- Far: Subtle presence indicators  
- Medium: Clear awareness of stalking
- Close: Immediate danger sensation
- Touching: Reality breakdown terror

HORROR AUDIO DESIGN:
- Breathing that doesn't match wind
- Footsteps that echo wrong
- Reality distortion sounds
- False audio cues (companion voices)
- Silence more terrifying than sound

SPATIAL AUDIO:
- 3D positioned relative to player
- Volume/filtering based on distance  
- Environmental occlusion effects
- Psychological impact timing

Generate audio specification for Bevy spatial audio system.
"""

[assets]
audio = [
    "dragon_breath_{distance_category}.ogg",
    "dragon_footsteps_{distance_category}.ogg",
    "reality_distortion_{distance_category}.ogg",
    "false_companion_voices.ogg"
]

[validation]
required_fields = [
    "distance_ranges",
    "audio_layers",
    "spatial_positioning",
    "psychological_triggers"
]
constraints = {
    max_distance = 100.0,
    min_distance = 0.1,
    volume_falloff_curve = "exponential",
    fear_escalation_rate = [0.1, 2.0]
}

[spatial_audio]
positioning = {
    coordinate_system = "hex_based",
    occlusion_enabled = true,
    reverb_zones = [
        "labyrinth_corridors",
        "void_spaces", 
        "reality_tears"
    ]
}
distance_categories = {
    very_far = [50.0, 100.0],
    far = [20.0, 50.0], 
    medium = [10.0, 20.0],
    close = [3.0, 10.0],
    touching = [0.0, 3.0]
}

[horror_progression]
audio_evolution = [
    "breathing_becomes_unnatural",
    "footsteps_multiply_impossibly",
    "companion_voices_distorted",
    "reality_audio_breaks_down"
]
psychological_impact = {
    sanity_drain = 0.1,
    false_hope_audio = true,
    directional_confusion = true
}
```

### Template: Companion Voice Evolution
```toml
[metadata]
id = "companion_voice_{companion}_{trauma_stage}"
name = "Voice Evolution: {companion} ({trauma_stage})"
description = "Generate voice evolution for {companion} at trauma stage {trauma_stage}"
version = "1.0.0"
created_by = "background_agent_cb_001"
dread_level = {dread_level}
act = {act}
priority = 8

[generation]
agent = "audio"
model = "gpt-4o-mini"
temperature = 0.7
max_tokens = 1800

prompt_template = """
Generate companion voice evolution for Dragon's Labyrinth.

COMPANION: {companion}
TRAUMA STAGE: {trauma_stage}
VOICE BASELINE: {original_voice_characteristics}

TRAUMA EFFECTS ON VOICE:
- {trauma_level}/10 trauma accumulated
- Specific trauma triggers: {trauma_triggers}
- Emotional state: {current_emotional_state}
- Physical condition: {physical_condition}

VOICE EVOLUTION STAGES:
- Confident: Clear, strong, natural speaking
- Doubting: Slight tremor, longer pauses
- Breaking: Shaky, whispered, fragmented
- Broken: Monotone, silent, or incoherent

HORROR INTEGRATION:
- Voice reflects psychological damage
- Breathing patterns change with fear
- False voices appear at high dread
- Companion voices distort near dragon

Generate audio processing specifications for voice evolution.
"""

[assets]
audio = [
    "{companion}_voice_baseline.ogg",
    "{companion}_voice_{trauma_stage}_var_01.ogg",
    "{companion}_voice_{trauma_stage}_var_02.ogg",
    "{companion}_voice_breathing_{trauma_stage}.ogg",
    "{companion}_voice_distorted.ogg"
]

[validation]
required_fields = [
    "voice_characteristics",
    "trauma_effects",
    "breathing_patterns",
    "distortion_parameters"
]
constraints = {
    trauma_range = [0.0, 10.0],
    voice_clarity_range = [0.1, 1.0],
    breathing_rate_multiplier = [0.5, 3.0]
}

[voice_processing]
effects_chain = [
    "pitch_variation",
    "tremor_addition",
    "breath_integration",
    "clarity_reduction",
    "horror_distortion"
]
trauma_mappings = {
    confident = { pitch_stability = 1.0, volume_consistency = 1.0 },
    doubting = { pitch_stability = 0.8, volume_consistency = 0.9 },
    breaking = { pitch_stability = 0.5, volume_consistency = 0.6 },
    broken = { pitch_stability = 0.2, volume_consistency = 0.3 }
}
```

## UI System Rules

### Template: Horror-Responsive Interface
```toml
[metadata]
id = "ui_horror_responsive_{ui_element}_{dread_level}"
name = "Horror UI: {ui_element} (Dread {dread_level})"
description = "Generate horror-responsive UI for {ui_element} at dread level {dread_level}"
version = "1.0.0"
created_by = "background_agent_cb_001"
dread_level = {dread_level}
act = {act}
priority = 6

[generation]
agent = "ui"
model = "gpt-4o-mini"
temperature = 0.7
max_tokens = 2000

prompt_template = """
Generate horror-responsive UI for Dragon's Labyrinth using Cobweb UI.

UI ELEMENT: {ui_element}
DREAD LEVEL: {dread_level}/4
HORROR THEME: {horror_theme}

COBWEB UI REQUIREMENTS:
- Declarative .cob scene format
- Reactive primitives for game state
- Horror-responsive visual changes
- Performance-optimized for mobile

HORROR PROGRESSION:
- Dread 0: Clean, medieval parchment style
- Dread 1: Slight tears, ink blots
- Dread 2: Water damage, hard to read
- Dread 3: Blood stains, text shifts
- Dread 4: Reality glitches, barely functional

UI DEGRADATION EFFECTS:
{degradation_effects}

Generate Cobweb UI scene with horror responsiveness built in.
"""

[assets]
ui_elements = [
    "{ui_element}_clean.png",
    "{ui_element}_dread_{dread_level}.png",
    "{ui_element}_corruption_mask.png",
    "{ui_element}_glitch_frames.png"
]
textures = [
    "parchment_base.png",
    "ink_blot_overlay.png",
    "blood_stain_overlay.png",
    "reality_glitch_shader.png"
]

[validation]
required_fields = [
    "cobweb_scene_definition",
    "horror_responsiveness",
    "performance_optimization",
    "state_reactivity"
]
constraints = {
    max_texture_size = 512,
    ui_response_time_ms = 16,
    corruption_opacity_range = [0.0, 0.8]
}

[horror_responsiveness]
visual_degradation = [
    "texture_corruption_increase",
    "color_desaturation",
    "geometry_distortion",
    "readability_reduction"
]
interaction_changes = [
    "button_response_delay",
    "false_ui_elements",
    "input_interpretation_errors",
    "navigation_confusion"
]

[cobweb_integration]
reactive_properties = [
    "dread_level_binding",
    "companion_state_binding",
    "corruption_level_binding"
]
scene_format = ".cob"
localization_support = true
```

## Rule Generation Guidelines

### Deterministic ID Generation

All TOML rules must use deterministic IDs:

```toml
# CORRECT: Content-based hash
id = "hex_world_gen_grassland_dread_2_act_1"

# WRONG: Random or timestamp based
id = "random_uuid_12345"
id = "map_generated_2025_01_22"
```

### Variable Substitution Rules

Templates support these variable patterns:

```toml
# Single variables
{variable_name}

# Conditional variables  
{?variable_exists}content{/variable_exists}

# Array iteration
{#array_variable}item: {.}{/array_variable}

# Nested objects
{object.property}
```

### Asset Naming Conventions

```toml
# Models: {category}_{type}_{variant}.glb
models = ["grass_hex_peaceful.glb", "grass_hex_corrupted.glb"]

# Textures: {category}_{purpose}_{variant}.png  
textures = ["grass_albedo_clean.png", "grass_overlay_corrupted.png"]

# Audio: {category}_{purpose}_{variant}.ogg
audio = ["grass_ambience_peaceful.ogg", "grass_footsteps_soft.ogg"]

# UI: {element}_{state}_{variant}.png
ui_elements = ["inventory_background_clean.png", "inventory_background_corrupted.png"]
```

### Validation Requirements

Every TOML rule must include:

1. **Required Fields**: What the generated content MUST contain
2. **Constraints**: Numeric ranges and limits  
3. **Success Criteria**: How to validate generation succeeded
4. **Horror Integration**: How it serves the horror progression

### Priority System

```toml
# Priority levels (1-10)
priority = 10  # Critical path - dread transitions, core systems
priority = 9   # Essential - proximity horror, companion trauma
priority = 8   # Important - corruption effects, major encounters
priority = 7   # Significant - economic systems, world events
priority = 6   # Useful - UI improvements, quality of life
priority = 5   # Optional - extra content, polish features
priority = 1-4 # Nice to have - experimental features
```

## Background Agent Instructions

### CB-001 Generation Process

1. **Read Narrative Bible**: Use S1M-001 output as primary source
2. **Apply Templates**: Use appropriate template for each rule type
3. **Generate Variables**: Create consistent variable sets
4. **Validate Format**: Ensure TOML syntax correctness
5. **Check Determinism**: Verify IDs are content-based
6. **Horror Integration**: Ensure all rules serve horror progression

### Rule Categories to Generate

**High Priority (Generate First)**:
- 20+ Hex world generation rules (all biomes, all dread levels)
- 30+ Companion dialogue rules (all states, all companions)
- 15+ Dread transition rules (all level changes)
- 25+ Encounter generation rules (major story beats)

**Medium Priority**:
- 20+ Sentimental item rules (if expanded vision approved)
- 15+ Forge trial rules (if expanded vision approved)
- 10+ Economic collapse rules (if expanded vision approved)
- 20+ Corruption manifestation rules

**Lower Priority**:
- 15+ UI horror responsiveness rules
- 25+ Audio progression rules  
- 10+ Memory event rules
- 15+ Environmental storytelling rules

### Quality Assurance

Each generated TOML must pass:
- [ ] Syntax validation (valid TOML)
- [ ] Template adherence (all required sections)
- [ ] Horror integration (serves narrative progression)
- [ ] Asset specification (clear requirements)
- [ ] Deterministic ID (content-based hash)
- [ ] Validation constraints (meaningful limits)

## Integration with AI Agents

These TOML rules integrate with the build-tools system:

```rust
// In crates/build-tools/src/rules.rs
pub fn load_toml_rules(rules_dir: &Path) -> Result<Vec<GenerationRule>> {
    let mut rules = Vec::new();
    
    for entry in fs::read_dir(rules_dir)? {
        let path = entry?.path();
        if path.extension() == Some("toml") {
            let content = fs::read_to_string(&path)?;
            let rule: GenerationRule = toml::from_str(&content)?;
            rules.push(rule);
        }
    }
    
    // Sort by priority (highest first)
    rules.sort_by_key(|r| std::cmp::Reverse(r.metadata.priority));
    
    Ok(rules)
}
```

This TOML template system ensures consistent, horror-focused, idempotent content generation across all AI agents while maintaining the clean architectural separation between build-time generation and runtime consumption.
