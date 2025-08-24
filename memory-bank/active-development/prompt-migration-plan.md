# Dragon's Labyrinth - Prompt Migration Plan: Individual Narrative Metaprompts

## Core Insight: Individual Metaprompts with Narrative Awareness

After our architectural breakthrough, we've moved from a Master Metaprompt to **individual narrative-focused metaprompts** that each understand their place in the horror journey. This gives us thematic diversity while maintaining narrative coherence.

## What Makes Dragon's Labyrinth Special (MUST PROTECT)

### 1. The Dragon's Labyrinth Horror Experience
- **The Jarring Transition**: Hex-based tactical → First-person horror
- **The Hunt**: Dragon actively stalking you with proximity-based audio
- **Sanity System**: Not just a mechanic, but false sounds and hallucinations
- **The Ambush System**: SLAM transitions between perspectives
- **The Revelation**: Dragon as guardian, not monster
- **The Trauma**: This experience echoes through the entire game

### 2. Narrative-Driven Design Decisions
- Companions need therapy after the labyrinth
- NPCs are labyrinth survivors who need help
- The endgame "Labyrinth Echoes" event
- Horror Arena mode bringing back the terror
- Multiple endings based on understanding, not just power

### 3. Unique Mechanical Concepts
- Hex grid with elevation and environmental storytelling
- Metaprompt-driven content generation (biomes, classes, items)
- Zero-dependency philosophy (no external assets)
- Staggered generation pipeline
- Audio as a first-class citizen (Music21 + Freesound integration)

## The Journey IS the Orchestrator

Instead of a Master Metaprompt or Python orchestration, **the narrative journey orchestrates everything**:

1. **Peace → Unease → Dread → Terror**: This progression drives all generation
2. **Individual Metaprompts**: Each system has its own narrative-aware template
3. **Content Generation**: Each metaprompt generates 5-10 variations for its emotional stage
4. **Existing Infrastructure**: template_processor.py and prompt_chain.py handle the technical orchestration

## Proposed Architecture

### 1. Narrative-Focused Jinja2 Templates (~100-200 lines each)
Each template embeds emotional journey context:

```jinja2
<!-- prompts/systems/quest.jinja2 -->
<!-- SYSTEM_PROMPT -->
You are creating the quest system for Dragon's Labyrinth at the {{ emotional_stage }} stage.

NARRATIVE CONTEXT:
- Previous: {{ previous_state }}
- Current: {{ current_state }}
- Next: {{ next_state }}
- Dread Level: {{ dread_level }}

CONTENT GENERATION REQUIREMENTS:
- Generate 10-20 quest variations appropriate for {{ current_state }}
- Create quest items that reflect the emotional stage
- Generate NPC quest-givers showing appropriate fear levels
- Include quest completion sounds (Music21 for ambient, Freesound for effects)

The quests should {{ narrative_purpose }}.

TECHNICAL REQUIREMENTS:
- Follow Godot 4 best practices
- Use established hex grid system
- Integrate with companion trauma system
- Output both GDScript code and content manifests
<!-- /SYSTEM_PROMPT -->
```

### 2. Vision-Critical Elements (Protected in Every Template)
These appear in relevant templates to preserve the horror experience:

```yaml
vision_critical:
  dragon_labyrinth:
    - First-person horror transitions
    - Proximity-based dragon AI
    - Sanity system with false reality
    - The revelation about protection
  
  companion_trauma:
    - Psychological impact accumulation
    - Therapy mechanics post-labyrinth
    - Abandonment based on trauma levels
  
  narrative_progression:
    - Never decreasing dread
    - Environmental decay
    - NPC behavioral changes
```

### 3. Content Generation Integration
Each template generates ALL needed assets:

```yaml
biome_generator:
  emotional_stage: "unease"
  generates:
    - 5-10 unique biome variations
    - Tile models (low-poly GLB)
    - Ambient soundscapes (Music21)
    - Environmental effects (Freesound)
    - Biome transition logic
    - Fog/weather shaders
```

## Migration Strategy

### Phase 1: Create Narrative Templates Directory Structure
```
prompts/
├── opening/          # Door scene templates
│   ├── first_person_door.jinja2
│   └── morning_peace.jinja2
├── world/           # Hex exploration templates
│   ├── biome_generator.jinja2
│   ├── hex_grid_system.jinja2
│   └── map_exploration.jinja2
├── systems/         # Game systems with narrative awareness
│   ├── quest_system.jinja2
│   ├── combat_system.jinja2
│   ├── inventory_system.jinja2
│   └── companion_system.jinja2
├── labyrinth/       # Horror-specific templates
│   ├── dragon_proximity.jinja2
│   ├── sanity_system.jinja2
│   └── horror_transitions.jinja2
└── audio/           # Audio generation templates
    ├── adaptive_music.jinja2
    └── environmental_audio.jinja2
```

### Phase 2: Update Each Template with Narrative Context
For each current template:
1. **Add emotional stage parameters**
2. **Include content generation requirements**
3. **Embed vision-critical protections**
4. **Specify asset outputs**

### Phase 3: Leverage Existing Infrastructure
- **template_processor.py**: Already handles Jinja2 processing perfectly
- **prompt_chain.py**: Update dependencies to reflect narrative flow
- **config.py + base.py**: Use new configuration system for emotional stages

### Phase 4: Test with Growing Dread
Use the quest system as proof of concept:
- Generate quests for each emotional stage
- Verify content reflects narrative progression
- Ensure all assets are generated appropriately

## Example: Quest System Migration

### Current Monolithic Template (800+ lines)
```jinja2
# Generate complete quest system with all mechanics...
[Massive template trying to handle everything]
```

### New Narrative-Aware Template (150 lines)
```jinja2
<!-- prompts/systems/quest.jinja2 -->
<!-- SYSTEM_PROMPT -->
You are creating quests for Dragon's Labyrinth's {{ emotional_stage }} stage.

NARRATIVE UNDERSTANDING:
{{ emotional_stage }} means:
{% if emotional_stage == "peace" %}
- Players feel safe and curious
- Quests are helpful and innocent
- NPCs are friendly and grateful
{% elif emotional_stage == "unease" %}
- Something feels wrong but unclear
- Quests have unexpected twists
- NPCs seem nervous about something
{% elif emotional_stage == "dread" %}
- Active fear permeates everything
- Quests reveal horrible truths
- NPCs actively avoid the player
{% elif emotional_stage == "terror" %}
- Reality itself is breaking down
- Quests become survival horror
- NPCs have fled or gone mad
{% endif %}

CONTENT TO GENERATE:
1. **Quest Variations** (10-20 for this stage)
   - Titles that reflect {{ emotional_stage }}
   - Descriptions showing narrative progression
   - Objectives that reinforce the horror journey

2. **Quest Items** (5-10 unique items)
   - Models: Low-poly GLB with vertex colors
   - Names/descriptions matching emotional tone
   - Properties that might change with dread

3. **Audio Assets**
   - Quest acceptance sounds (Music21 generated)
   - Completion sounds (hollow at high dread)
   - Ambient quest music (procedural horror)

4. **NPC Dialogue**
   - Quest giver personalities for {{ emotional_stage }}
   - Dialogue showing appropriate fear/concern
   - Refusal dialogues at high dread levels

Generate complete GDScript implementation with content manifests.
<!-- /SYSTEM_PROMPT -->
```

## Benefits of Individual Narrative Metaprompts

1. **Thematic Diversity**: Each system can interpret the emotional journey uniquely
2. **No Central Bottleneck**: No single Master Metaprompt to maintain
3. **Natural Evolution**: Systems can evolve independently while staying narratively coherent
4. **Content Integration**: Each metaprompt generates all needed assets
5. **Existing Infrastructure**: Uses our strongest systems (template_processor.py)

## Implementation Priority

### Immediate Actions:
1. Create narrative template structure
2. Convert quest system as proof of concept
3. Update prompt_chain.py dependencies for narrative flow
4. Test biome generation with emotional stages
5. Validate audio generation integration

### Vision-Critical Templates First:
1. **opening/first_person_door.jinja2** - The haunting beginning
2. **labyrinth/dragon_proximity.jinja2** - Core horror experience
3. **systems/companion_system.jinja2** - Trauma mechanics
4. **world/biome_generator.jinja2** - Environmental storytelling
5. **labyrinth/sanity_system.jinja2** - Reality breakdown

## Measuring Success

The migration succeeds when:
- Each template knows its place in the horror journey
- Generated content naturally reflects emotional progression
- No need for post-generation narrative checking
- Systems feel diverse yet coherent
- The door scene still haunts players

## Core Philosophy Reminder

"We're not building 'an RPG with horror elements' - we're building a horror experience that happens to have RPG mechanics."

The individual narrative metaprompts ensure every generated line of code, every model, every sound effect serves this horror journey.
