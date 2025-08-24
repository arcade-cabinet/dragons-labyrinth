# Narrative Orchestration Architecture

## Core Realization
We don't need Python orchestration. We need AI orchestration guided by narrative structure. The game's emotional journey IS the orchestrator.

## The Journey as Architecture

### Act 1: The Opening (Peace)
**Directory**: `game/opening/`
**Metaprompt Focus**: 
- First-person perspective, morning light through windows
- The last moment of peace they'll ever know
- Door handle mechanics that will haunt them
- Transition to hex world must feel jarring

### Act 2: The Exploration (Unease)
**Directory**: `game/overworld/`
**Metaprompt Focus**:
- Hex-based exploration with elevation as storytelling
- Beauty with shadows at the edges
- NPCs who seem normal but aren't quite right
- Quests that start innocent but twist

### Act 3: The Descent (Dread)
**Directory**: `game/systems/`
**Metaprompt Focus**:
- Combat that makes you feel weaker, not stronger
- Inventory where items lose meaning
- Companions who start breaking down
- Towns where doors lock when you approach

### Act 4: The Labyrinth (Terror)
**Directory**: `game/labyrinth/`
**Metaprompt Focus**:
- Return to first-person, but wrong
- Procedural generation that ensures unique horror each time
- Sound-driven navigation in darkness
- The dragon as hunter, not boss

### Act 5: The After (?)
**Directory**: `game/endgame/`
**Metaprompt Focus**:
- What does "winning" even mean?
- Companions need therapy
- The world has changed because you exist
- Multiple endings based on understanding, not choices

## Directory-Based Generation

Instead of `orchestrator.py`, we have:

```
dragon-labyrinth-game/
├── metaprompt.md          # Master narrative guide
├── opening/
│   ├── metaprompt.md      # "Create the door that changes everything"
│   ├── door_scene.gd      # Generated
│   └── transition.gd      # Generated
├── world/
│   ├── metaprompt.md      # "A world that reacts to growing dread"
│   ├── hex_system/
│   │   ├── metaprompt.md  # "Elevation tells stories"
│   │   └── *.gd           # Generated
│   └── biomes/
│       ├── metaprompt.md  # "Beauty hiding horror"
│       └── */             # Generated biomes
├── entities/
│   ├── metaprompt.md      # "Everyone is affected by the curse"
│   ├── player/
│   ├── companions/
│   └── npcs/
├── systems/
│   ├── metaprompt.md      # "Systems that reinforce helplessness"
│   ├── combat/
│   ├── inventory/
│   └── quests/
├── labyrinth/
│   ├── metaprompt.md      # "This is where we keep the real horror"
│   ├── generator/
│   ├── dragon_ai/
│   └── escape_sequences/
└── audio/
    ├── metaprompt.md      # "Sound is the primary horror vector"
    ├── music21/           # Procedural dread
    └── freesound/         # Environmental terror
```

## Key Principles

### 1. Narrative Coherence Over Technical Architecture
Each metaprompt knows:
- Its emotional purpose
- Its place in the journey
- How player state affects it
- What other systems it must harmonize with

### 2. AI Generates 99%, We Guide 1%
We provide:
- The emotional journey map
- Key story beats
- Aesthetic guidelines
- The core horror concept

AI creates:
- All code implementation
- Asset generation
- System interactions
- Procedural variations

### 3. Style Guide as DNA
Each metaprompt contains:
- Visual style (low-poly, vertex colors, chess-piece characters)
- Audio style (ambient dread, reactive music)
- Narrative tone for its section
- Emotional targets

### 4. The Dragon's Labyrinth Exception
The ONLY place we might want more control:
- First-person horror mechanics
- Dragon AI behavior
- Specific scare sequences
- The truth reveal

But even here, we guide through detailed metaprompts rather than code.

## Implementation Approach - Using Our Existing Infrastructure

### Key Realization
We already have the perfect infrastructure in `template_processor.py` and `prompt_chain.py`. We don't need to rebuild - we need to repurpose!

### Phase 1: Create Narrative-Driven Templates with Content Generation
Instead of just technical templates, create narrative templates that ALSO generate all needed content:

```
prompts/
├── narrative_guide.jinja2          # The emotional journey reference
├── opening/
│   └── door_scene.jinja2          # "Create the door that changes everything"
│                                  # ALSO: Generate door models, sounds, lighting
├── world/
│   ├── hex_exploration.jinja2     # "A world that reacts to dread"
│   │                              # ALSO: Generate biomes for each dread level
│   └── biome_generator.jinja2     # "Generate EVERY biome type for this stage"
│                                  # Uses the brilliant biome metaprompt approach
├── entities/
│   ├── player.jinja2              # "You don't get stronger, you get marked"
│   │                              # ALSO: Generate character models, curse effects
│   ├── companions.jinja2          # "They'll need therapy after this"
│   │                              # ALSO: Generate trauma animations, dialogue
│   └── npcs.jinja2                # "They lock doors when you approach"
│                                  # ALSO: Generate fear reactions, models
├── systems/
│   ├── combat.jinja2              # "Fighting makes you weaker"
│   │                              # ALSO: Generate weapon models, impact effects
│   ├── inventory.jinja2           # "Items lose meaning"
│   │                              # ALSO: Generate degrading item visuals
│   └── quests.jinja2              # "Innocent tasks that twist"
│                                  # ALSO: Generate quest items, locations
├── labyrinth/
│   ├── first_person.jinja2        # "Return to first-person, but wrong"
│   │                              # ALSO: Generate corridor variations
│   ├── procedural.jinja2          # "Never the same twice"
│   │                              # ALSO: Generate tile sets, trap models
│   └── dragon_ai.jinja2           # "It hunts you"
│                                  # ALSO: Generate dragon model, animations
└── audio/
    ├── music21_horror.jinja2       # "Procedural dread"
    │                              # ALSO: Generate music for each dread level
    └── freesound_terror.jinja2    # "Environmental horror"
                                   # ALSO: Download/process sounds for each biome
```

### Phase 2: Enhance Templates with Narrative + Content Generation
Each template includes narrative guidance AND content generation:
```jinja2
<!-- SYSTEM_PROMPT -->
You are creating the {{ system_name }} for Dragon's Labyrinth, a horror game disguised as an RPG.

EMOTIONAL CONTEXT: This system appears at {{ emotional_stage }} of the journey.
- Previous state: {{ previous_emotional_state }}
- Current state: {{ current_emotional_state }}  
- Next state: {{ next_emotional_state }}

NARRATIVE ROLE: {{ narrative_purpose }}

CONTENT GENERATION REQUIREMENTS:
Generate ALL necessary content for this system at this emotional stage:
{% if system_name == "hex_exploration" %}
- Generate biomes appropriate for {{ current_emotional_state }}:
  * BIOME: Peaceful Meadow (for early game)
  * BIOME: Corrupted Lands (for growing dread)
  * BIOME: Nightmare Terrain (for high dread)
  * Generate 5-10 unique biomes per emotional stage
{% elif system_name == "combat" %}
- Generate weapon models that reflect {{ current_emotional_state }}:
  * Early: Normal swords, practical armor
  * Mid: Weapons that whisper, armor that weighs you down
  * Late: Cursed weapons that hurt to hold
{% endif %}

INTEGRATION: This system must harmonize with:
{% for dep in narrative_dependencies %}
- {{ dep.name }}: {{ dep.relationship }}
{% endfor %}

STYLE GUIDE:
- Visual: Low-poly, vertex colors, chess-piece style
- Audio: {{ audio_direction }}
- Feel: {{ emotional_target }}

DIVERSITY REQUIREMENT:
Do not draw from the same well - create varied, unique content that all serves the narrative purpose but with rich thematic diversity.
<!-- /SYSTEM_PROMPT -->

<!-- FILE: {{ system_name }}.gd -->
<!-- FILE: {{ system_name }}_controller.gd -->
<!-- MODEL: {{ system_name }}_models.glb -->
<!-- AUDIO: {{ system_name }}_sounds.ogg -->
<!-- Additional files and assets as needed -->
```

### Phase 3: Adapt PromptChainManager
Update dependencies to reflect narrative flow:
```python
NARRATIVE_DEPENDENCIES = {
    "door_scene": [],  # The beginning
    "hex_exploration": ["door_scene"],  # After the door
    "companions": ["hex_exploration"],  # Met during exploration
    "quests": ["companions", "npcs"],  # Need people for quests
    "combat": ["quests"],  # Combat emerges from failed quests
    "labyrinth": ["ALL"],  # Everything leads here
    "endgame": ["labyrinth"]  # What comes after
}
```

### Phase 4: Use Existing Audio System
The audio generation system is already perfect:
- Music21 for procedural music that responds to dread
- Freesound integration for environmental audio
- Just need horror-focused metaprompts

### Phase 5: Minimal Orchestrator
```python
# This just feeds narrative context to our existing system
class NarrativeOrchestrator(GodotGameGenerator):
    def generate_game(self):
        # Load narrative journey
        narrative = self.load_narrative_journey()
        
        # Process each stage with emotional context
        for stage in narrative.stages:
            context = {
                "emotional_stage": stage.name,
                "dread_level": stage.dread_level,
                "narrative_purpose": stage.purpose,
                "previous_emotional_state": stage.previous,
                "current_emotional_state": stage.current,
                "next_emotional_state": stage.next
            }
            
            # Use existing template processor
            self.processor.process_template(
                stage.template_name,
                context
            )
```

## Success Metrics

The game succeeds when:
1. Opening door scene creates lasting dread
2. Players feel the emotional journey, not game mechanics
3. Each system reinforces the narrative
4. The labyrinth feels alive and hunting
5. Companions feel real enough that their trauma matters

## Quote for This Architecture

"We're not building a game engine. We're conducting an emotional symphony where each instrument knows its part in the crescendo from peace to madness."

## Next Steps

1. Create the directory structure
2. Write the master narrative metaprompt
3. Create location-specific metaprompts
4. Build the minimal runner
5. Generate and iterate

The orchestrator isn't Python. It's the story itself.
