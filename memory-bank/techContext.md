# Technical Context - Dragon's Labyrinth

## CRITICAL: READ THIS BEFORE DOING ANYTHING

### THE ARCHITECTURE (STOP FORGETTING THIS!)

1. **Metaprompts = Jinja2 Templates** (`.jinja2` files, NOT markdown!)
2. **Distributed Throughout Game Directories** (NOT centralized!)
3. **Generate EVERYTHING** (code + models + audio + data)
4. **Narrative Drives Dependencies** (peace → unease → dread → terror → horror)

### WHERE WE ARE NOW

```
✅ COMPLETE:
generator/
├── __main__.py              # Entry point: python -m generator --stage peace
├── template_processor.py    # Jinja2 + LangChain (works perfectly!)
├── prompt_chain.py         # Narrative dependencies (ready to use!)
├── constants.py            # All strings externalized
└── audio/                  # Complete Music21 + Freesound system
    ├── composer.py         # Algorithmic music generation
    ├── freesound.py       # API integration for SFX
    ├── exporter.py        # OGG file generation
    └── metaprompt.py      # Audio prompt generation

❌ MISSING:
- Game directory structure with Jinja2 templates
- Narrative-aware templates (current ones in prompts/ are old)
- First actual generation test
```

## Technology Stack (CURRENT)
- **Engine**: Godot 4.x (MultiMesh optimization)
- **Language**: GDScript (AI-generated)
- **Generator**: Python 3.13+ with Poetry
- **AI Integration**: LangChain (OpenAI, Anthropic, Ollama)
- **Template Engine**: Jinja2
- **Audio Generation**: 
  - Music21 for procedural music
  - Freesound API for sound effects
  - PyOGG for format conversion

## The ACTUAL Architecture (From Project Bible)

### Game Directory Structure (CREATE THIS!)
```
dragons-labyrinth/         # Repository root
├── generator/            # ✅ DONE - Python generator
├── opening/              # ❌ NEEDS Jinja2 templates
│   └── door_scene.jinja2
├── world/                # ❌ NEEDS Jinja2 templates
│   ├── hexgrid/
│   │   └── hex_system.jinja2
│   └── biomes/
│       └── biome_generator.jinja2
├── entities/             # ❌ NEEDS Jinja2 templates
│   ├── player/
│   ├── companions/
│   └── npcs/
├── systems/              # ❌ NEEDS Jinja2 templates
│   ├── combat/
│   ├── inventory/
│   └── quests/
├── labyrinth/           # ❌ NEEDS Jinja2 templates
│   ├── first_person_horror.jinja2
│   ├── dragon_proximity.jinja2
│   └── sanity_system.jinja2
└── audio/               # ❌ NEEDS Jinja2 templates
    └── adaptive_music.jinja2
```

### Jinja2 Template Format (USE THIS!)
```jinja2
<!-- SYSTEM_PROMPT -->
You are creating {{ system_name }} for Dragon's Labyrinth at {{ emotional_stage }}.

NARRATIVE CONTEXT:
- Current dread: {{ dread_level }}
- Journey stage: {{ current_state }}

CONTENT TO GENERATE:
- 5-10 variations for this emotional stage
- All GDScript code files
- 3D models (low-poly GLB, 200-500 triangles)
- Audio (Music21 for music, Freesound for effects)
- JSON manifests for external tools
<!-- /SYSTEM_PROMPT -->

<!-- FILE: scripts/{{ system_name }}.gd -->
<!-- FILE: scripts/{{ system_name }}_controller.gd -->
<!-- MODEL: models/{{ system_name }}_models.glb -->
<!-- AUDIO: audio/music/{{ system_name }}_theme.ogg -->
<!-- AUDIO: audio/sfx/{{ system_name }}_effects.ogg -->
<!-- JSON: data/{{ system_name }}_manifest.json -->
```

## How to Run Generation (WHEN TEMPLATES EXIST)

```bash
# Install dependencies (one time)
cd generator
poetry install

# Generate for emotional stage
poetry run python -m generator --stage peace
poetry run python -m generator --stage unease
poetry run python -m generator --stage dread
poetry run python -m generator --stage terror
poetry run python -m generator --stage horror

# Or just:
python -m generator  # Defaults to 'peace'
```

## Dependencies (via Poetry)
```toml
[tool.poetry.dependencies]
python = "^3.13"
langchain = "^0.3.13"
langchain-openai = "^0.2.14"
langchain-anthropic = "^0.3.0"
langchain-community = "^0.3.13"
jinja2 = "^3.1.4"
music21 = "^9.1.0"      # Algorithmic music
pyogg = "^0.7"          # Audio conversion
requests = "^2.32.3"    # Freesound API
pydantic = "^2.10.5"    # Data validation
typer = "^0.15.1"       # CLI (for future)
rich = "^13.9.4"        # Pretty output
```

## Template Discovery (How It Works)

```python
# The generator finds templates throughout the game structure:
def find_templates():
    # NOT looking in templates/ directory!
    # Looking for .jinja2 files in game directories
    return Path(".").rglob("*/*.jinja2")
    
# Each template knows where it lives
# Files generate relative to template location
```

## Content Generation Pipeline

1. **Narrative Context** → Emotional stage determines everything
2. **Template Processing** → Jinja2 template + context → AI prompt
3. **AI Generation** → Creates code, model specs, audio specs
4. **Asset Creation**:
   - Code → Write .gd files
   - Models → AI generates GLB
   - Music → Music21 generates OGG
   - SFX → Freesound download + process
5. **Integration** → All files written to game directories

## Audio Architecture (COMPLETE)

### When to Use What:
- **Music21**: Background music, emotional progressions, abstract sounds
- **Freesound**: Realistic effects, creature sounds, environmental audio

### Integration Example:
```yaml
audio_requirements:
  music:
    type: "generated"  # Music21
    emotional_state: "growing_dread"
    tempo: [100, 120]
    key: "D minor to diminished"
    
  effects:
    - type: "freesound"
      search: "wooden door creak old"
      emotional_processing: "reverse_at_high_dread"
    - type: "freesound"  
      search: "dragon breathing cave"
      spatial: true
```

## Current Problems (BE HONEST)

1. **Wrong File Formats**: We keep making markdown files instead of Jinja2
2. **Centralized Thinking**: Putting everything in templates/ or prompts/
3. **Forgetting the Vision**: Not following projectDesign.md
4. **No Game Structure**: Haven't created the actual game directories
5. **Old Templates**: prompts/ has 800-line monoliths, not mechanical prompts

## What To Do Right Now

1. **DELETE** `opening/metaprompt.md` - wrong format!
2. **CREATE** game directory structure
3. **WRITE** first Jinja2 template: `opening/door_scene.jinja2`
4. **TEST** with: `python -m generator --stage peace`
5. **ITERATE** based on results

## Key Files to Reference

- **Project Bible**: `memory-bank/projectDesign.md` (THE source of truth)
- **Narrative Architecture**: `memory-bank/narrative-orchestration-architecture.md`  
- **Examples**: `memory-bank/design-examples/` (shows what to generate)
- **Generator Code**: `generator/` (ready to use!)

## Remember: The Journey IS the Orchestrator

Stop thinking about technical orchestration. The emotional journey drives everything:
- Directory structure = narrative structure
- Dependencies = emotional flow
- Content = narrative appropriate

**"We're building a horror experience that happens to have RPG mechanics."**
