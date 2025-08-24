# System Patterns - Dragon's Labyrinth

## THE CORE ARCHITECTURE - READ THIS FIRST!

### 1. METAPROMPTS ARE JINJA2 FILES
**NOT** markdown, **NOT** YAML - they are `.jinja2` template files like in `prompts/` directory!

### 2. DISTRIBUTED TEMPLATE ARCHITECTURE
Templates live **WHERE CODE GETS GENERATED**:
```
opening/door_scene.jinja2          → generates files IN opening/
world/hexgrid/hex_system.jinja2    → generates files IN world/hexgrid/
systems/quests/quest_system.jinja2 → generates files IN systems/quests/
```

The directory structure IS the orchestration - no central templates/ directory!

### 3. TEMPLATES GENERATE EVERYTHING
Each Jinja2 template generates:
- **GDScript code** (.gd files)
- **3D models** (.glb files via AI)
- **Audio files** (Music21 → .ogg, Freesound → .ogg)
- **JSON manifests** (for external tools)

### 4. NARRATIVE DRIVES DEPENDENCIES
Not technical dependencies but emotional flow:
```
peace → unease → dread → terror → horror
```

## Jinja2 Template Format (THIS IS THE FORMAT!)

```jinja2
<!-- SYSTEM_PROMPT -->
You are creating the {{ system_name }} for Dragon's Labyrinth at the {{ emotional_stage }} stage.

NARRATIVE CONTEXT:
- Previous state: {{ previous_state }}
- Current state: {{ current_state }}
- Next state: {{ next_state }}
- Dread level: {{ dread_level }}

CONTENT GENERATION REQUIREMENTS:
{% if system_name == "hex_exploration" %}
- Generate 5-10 unique biome types for {{ current_state }}:
  * Early game: Peaceful meadows, sunny forests
  * Mid game: Fog-shrouded woods, silent villages  
  * Late game: Corrupted lands, nightmare terrain
- Create biome models (low-poly GLB, 200-500 triangles)
- Generate ambient sounds (Music21 for music, Freesound for effects)
{% elif system_name == "quest_system" %}
- Generate 10-20 quest variations for {{ current_state }}:
  * Peace: "Deliver bread to the baker"
  * Dread: "The bread... why does it scream?"
  * Horror: "Your companion asks you to kill them"
- Create quest item models reflecting emotional state
- Generate quest completion sounds (triumphant → hollow)
{% endif %}

INTEGRATION: This system must work with:
{% for dep in narrative_dependencies %}
- {{ dep.name }}: {{ dep.relationship }}
{% endfor %}

TECHNICAL REQUIREMENTS:
- Follow Godot 4 best practices
- Use MultiMeshInstance3D for performance
- Integrate with existing hex grid system
- Output idempotent code (same prompt = compatible results)

Generate complete implementation with all assets.
<!-- /SYSTEM_PROMPT -->

<!-- FILE: {{ system_name }}.gd -->
# Main system implementation

<!-- FILE: {{ system_name }}_controller.gd -->
# Controller logic

<!-- FILE: {{ system_name }}_data.gd -->
# Data structures

<!-- MODEL: {{ system_name }}_models.glb -->
# 3D models for this system

<!-- AUDIO: music/{{ system_name }}_theme.ogg -->
# Background music (Music21 generated)

<!-- AUDIO: sfx/{{ system_name }}_effects.ogg -->
# Sound effects (Freesound sourced)

<!-- JSON: manifests/{{ system_name }}_content.json -->
# Content manifest for external tools
```

## How Generation Works

### 1. Entry Point
```bash
python -m generator --stage peace
```

### 2. Generator Flow
```python
# generator/__main__.py reads emotional stage
# → template_processor.py finds distributed templates
# → Processes each template with narrative context
# → AI generates code + models + audio
# → Files created in their directories
```

### 3. Template Discovery
Templates are found by traversing game directories looking for `.jinja2` files:
```python
# NOT this:
templates = Path("templates/").glob("*.jinja2")

# But THIS:
templates = Path(".").rglob("*/*.jinja2")  # Find templates everywhere
```

## Content Generation Examples

### Biome Generation (from design examples)
```yaml
BIOME: Corrupted Grassland
INTENSITY: Standard
EMOTIONAL_STAGE: dread

Generates:
- corrupted_grass_hex.glb (3D model with withered grass)
- corrupted_grass_ambience.ogg (Music21: dissonant nature sounds)
- grass_dying.ogg (Freesound: actual grass wilting sounds)
- biome_data.json (spawn rates, movement penalties, etc.)
```

### Quest Evolution (from design examples)
```gdscript
# Generated based on emotional_stage
match emotional_stage:
    "peace":
        quest.title = "Fresh Bread Delivery"
        quest.description = "The baker needs help with deliveries"
    "dread":
        quest.title = "The Last Bread"
        quest.description = "No one will say why this is the last batch"
    "horror":
        quest.title = "The Bread Remembers"
        quest.description = "It screams when you touch it"
```

## Audio Pattern (DUAL GENERATION)

### Music21 for Algorithmic Music
```python
# Emotional progression in music
def generate_area_music(emotional_stage: str, area_type: str):
    if emotional_stage == "peace":
        return generate_major_key_melody(tempo=80)
    elif emotional_stage == "dread":
        return generate_dissonant_progression(tempo=120)
    elif emotional_stage == "horror":
        return generate_atonal_nightmare(tempo=160)
```

### Freesound for Realistic Effects
```yaml
audio:
  effects:
    - search: "wooden door creak"
      emotional_filter: "normal"
    - search: "door slam echo"  
      emotional_filter: "dread"
    - search: "door breathing"
      emotional_filter: "horror"  # Yes, the door breathes
```

## The Narrative Journey Architecture

### Directory = Emotional Journey
```
opening/          # Peace - The last normal moment
world/            # Unease - Something's wrong
systems/          # Dread - Systems fail you
labyrinth/        # Terror - First-person horror
endgame/          # Horror - What have you done?
```

### Templates Know Their Place
Each template receives:
- `emotional_stage`: Where in the journey
- `dread_level`: 0-5 numeric progression
- `previous_state`: What came before
- `current_state`: Current emotional state
- `next_state`: Where we're heading

## Mechanical Prompts Philosophy

### Small, Focused Components
- **NOT** 800-line monolithic templates
- **BUT** 50-100 line focused components
- Each does ONE thing perfectly
- Generates ALL assets for that thing

### Example Structure
```
systems/combat/
├── combat_system.jinja2      # Main combat logic
├── damage_calc.jinja2        # Damage calculations
├── combat_ui.jinja2          # Combat interface
└── combat_effects.jinja2     # Visual/audio effects
```

## Key Implementation Files

### Generator Infrastructure (COMPLETE)
- `generator/__main__.py` - Entry point with narrative stages
- `generator/template_processor.py` - Jinja2 + LangChain integration
- `generator/prompt_chain.py` - Narrative dependency management
- `generator/constants.py` - All string constants

### Audio System (COMPLETE)
- `generator/audio/composer.py` - Music21 integration
- `generator/audio/freesound.py` - Freesound API integration
- `generator/audio/exporter.py` - Audio file generation
- `generator/audio/metaprompt.py` - Audio-specific prompting

### What's Missing
- **Game directory structure** with embedded Jinja2 templates
- **Narrative-aware templates** to replace old monolithic ones
- **First test run** of the complete system

## CRITICAL REMINDERS

1. **Check projectDesign.md** - It has EVERYTHING already designed
2. **Templates are Jinja2 files** - Not markdown, not YAML
3. **Distributed architecture** - Templates live where code generates
4. **Content generation** - Every template makes code + models + audio
5. **Narrative drives all** - Technical follows emotional journey

## The Vision (Never Forget)

"We're not building an RPG with horror elements - we're building a horror experience that happens to have RPG mechanics."

Every line of code, every model, every sound effect serves the journey from a peaceful morning to incomprehensible horror. The templates don't just generate a game - they generate an emotional experience.
