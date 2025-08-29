# Audio Generation Architecture

## Overview

The audio generation system is a Python-based asset generator that creates context-aware audio files (OGG format) for Dragon's Labyrinth. It operates as part of the main generator infrastructure, producing audio assets that evolve with the game's horror narrative.

## Core Concept

Audio is treated exactly like visual assets in the generation pipeline:
- **Music files**: Algorithmically composed using Music21 → exported as OGG files
- **Sound effects**: Downloaded from Freesound → processed and exported as OGG files
- **Integration**: Mechanical prompts specify audio needs inline, generator produces the assets

Just like how we generate GLB models for tiles and props, we generate OGG files for music and sound effects.

## Architecture Components

### 1. Python Generator Infrastructure (`generator/audio/`)

```
generator/audio/
├── context.py         # AudioContext: Game state → audio requirements
├── metaprompt.py      # AudioMetaprompt: Creates AI prompts for audio specs
├── composer.py        # Music21Composer: JSON specs → music → OGG files
├── freesound.py       # FreesoundIntegration: Downloads/processes SFX
└── exporter.py        # GodotAudioExporter: Creates integration code
```

### 2. Audio Request Format

Other mechanical prompts can specify audio needs:

```yaml
audio_requirements:
  music:
    emotional_state: "growing_unease"
    tempo_range: [80, 100]
    key_progression: "minor_to_diminished"
    layers_needed: ["base", "dread"]
  
  soundscape:
    area_type: "abandoned_settlement"
    dread_level: 3
    specific_sounds:
      - "empty_wind"
      - "creaking_doors"
      - "distant_sobbing"
  
  events:
    - trigger: "player_enters_area"
      audio: "music_transition_to_dread"
    - trigger: "companion_breakdown"
      audio: "silence_then_sobbing"
```

### 3. Generation Pipeline

1. **Context Gathering**: Game systems provide AudioContext
2. **Metaprompt Generation**: Create AI prompt for audio specifications
3. **AI Processing**: Generate JSON audio specification
4. **Asset Creation**:
   - Music: Music21 → algorithmic composition → PyOGG → OGG files
   - SFX: Freesound API → download → process for dread → OGG files
5. **Export**: Generate Godot integration code + asset manifest

## Integration with Master Architecture

### Dependency Flow
```
Biome Generation
    ↓ (defines area_type, base dread)
Quest System 
    ↓ (defines emotional progression)
Dread System
    ↓ (provides current dread level)
Audio Generator
    ↓ (creates context-aware audio assets)
Audio Assets (OGG files + integration code)
```

### Event-Driven Updates
When dread level changes:
1. Dread system emits event
2. Audio generator receives new context
3. Generates transition music/sounds
4. Exports updated audio assets

## Key Design Principles

### 1. Context-Aware Generation
Audio isn't generic - it's generated based on:
- Current area (labyrinth vs settlement)
- Dread level (0-5)
- Narrative phase (peace → terror)
- Companion states (trauma affects music stability)
- Dragon proximity (spatial audio effects)

### 2. Horror Evolution
Audio tells the transformation story:
- **Dread 0**: Birds, gentle wind, major keys, 60-80 BPM
- **Dread 1**: Sparse sounds, minor shifts, subtle wrongness
- **Dread 2**: Unnatural gaps, diminished chords, 80-100 BPM
- **Dread 3**: Oppressive silence, chromatic passages, reversed sounds
- **Dread 4**: Reality glitches, atonal music, 100-140 BPM
- **Dread 5**: Complete breakdown or deathly silence, 120-160 BPM

### 3. Asset-Based Approach
Like visual assets:
- Generated once, used many times
- Cached for performance
- Versioned based on context
- Delivered as standard OGG files

## Example Usage

When generating a new area:

```python
# 1. Area provides context
context = AudioContext(
    area_name="Abandoned Mine",
    area_type="labyrinth_horror",
    dread_level=3,
    narrative_phase="growing_dread",
    proximity_to_dragon=0.7
)

# 2. Generate metaprompt
metaprompt = AudioMetaprompt().generate_metaprompt(context)

# 3. AI generates specification
audio_spec = ai_generate(metaprompt)  # Returns JSON

# 4. Create audio assets
music_score = Music21Composer().compose_from_spec(audio_spec)
music_file = composer.export_to_audio(score, Path("audio/music/"))

# 5. Download/process environmental sounds
soundscape_config = FreesoundIntegration().generate_soundscape_config(
    area_type="labyrinth_horror",
    dread_level=3
)

# 6. Export Godot integration
GodotAudioExporter().export_audio_system(
    area_name="Abandoned Mine",
    audio_spec=audio_spec,
    music_files=[music_file],
    sfx_files=downloaded_sounds,
    output_dir=Path("generated/audio/")
)
```

## Benefits of This Architecture

1. **Truly Adaptive**: Audio generated for specific game contexts
2. **Horror-Aware**: Every sound reinforces the narrative journey
3. **Efficient**: Assets generated once, reused as needed
4. **Integrated**: Works seamlessly with other generation systems
5. **Professional Quality**: Music21 for composition, Freesound for real SFX

## Audio Generation Decision Guide

The AI needs clear rules for choosing between generation methods:

### Use Music21 Generation For:
- **Background music**: Adaptive layers, emotional progressions
- **Musical stings**: Combat victory, level up, quest complete
- **Abstract horror**: Atonal compositions, reality breaking sounds
- **Tempo-based effects**: Heartbeats, breathing rhythms
- **Key progressions**: Major → minor → diminished transitions

### Use Freesound Integration For:
- **Creature sounds**: Dragon breathing, monster roars, footsteps
- **Environmental audio**: Wind, water, fire, cave echoes
- **Physical interactions**: Doors, chests, item pickups
- **UI feedback**: Button clicks, menu sounds
- **Realistic effects**: Swords clashing, spell impacts

### Integration Examples:

```yaml
# Biome with mixed audio needs
name: "dragon_lair"
assets:
  audio:
    # Music21 for atmospheric horror
    music:
      type: "generated"
      spec:
        key: "C diminished"
        tempo: 120
        layers: ["heartbeat", "dissonant_strings", "whispers"]
    
    # Freesound for realistic effects  
    effects:
      - type: "freesound"
        search: "dragon breathing deep"
        id: "dragon_breath"
      - type: "freesound"
        search: "cave dripping echo"
        id: "cave_ambience"
```

## Next Steps

1. **Update mechanical prompts** to include audio_requirements sections
2. **Integrate with generator pipeline** to process audio alongside visuals
3. **Create audio asset manifest** for tracking generated files
4. **Test with biome generation** to ensure proper context flow
5. **Document audio request format** for prompt creators
