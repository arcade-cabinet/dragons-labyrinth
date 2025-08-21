# Dragon's Labyrinth - Biomes Reference

## Overview
Biomes in Dragon's Labyrinth are not just environmental zones - they are emotional landscapes that respond to and reinforce the horror progression. Each biome transforms as the dread level increases, creating a world that literally darkens with the player's journey.

## Biome Categories by Dread Level

### Peace Stage (Dread Level 0)
**Grasslands**
- Rolling green hills with wildflowers
- Clear blue skies with puffy white clouds
- Gentle breeze audio cues
- Small villages with friendly NPCs
- Peaceful ambient sounds (birds, wind)

**Forest Clearings**
- Dappled sunlight through leaves
- Friendly woodland creatures
- Well-maintained paths
- Wooden signposts and benches
- Nature sounds (rustling leaves, distant streams)

**Village Centers**
- Cobblestone streets
- Warm light from windows
- Bustling market squares
- Children playing sounds
- Domestic life audio (hammering, cooking, laughter)

### Unease Stage (Dread Level 1)
**Shadowed Woods**
- Longer shadows despite same sun position
- Fewer bird sounds, more silence
- Overgrown paths
- Dead branches start appearing
- Subtle whisper audio cues

**Abandoned Farms**
- Empty fields with dead crops
- Broken fences and gates
- Houses with dark windows
- No smoke from chimneys
- Wind through empty buildings

**Quiet Villages**
- NPCs speak in hushed tones
- Fewer people on streets
- Some shops closed
- Watchful glances from windows
- Reduced ambient human sounds

### Dread Stage (Dread Level 2)
**Swamplands**
- Murky water and dead trees
- Thick fog obscuring vision
- Strange lights in distance
- Bubbling and squelching sounds
- Ominous silence between sounds

**Ruined Settlements**
- Collapsed buildings
- Overgrown with dark vines
- Broken statues and monuments
- No living inhabitants
- Echo effects in empty spaces

**Corrupted Forests**
- Trees twisted into unnatural shapes
- No leaves, only bare branches
- Ground covered in ash or dead matter
- No wildlife sounds
- Occasional creaking of stressed wood

### Terror Stage (Dread Level 3)
**Nightmare Landscapes**
- Reality seems to shift and warp
- Impossible geography (uphill in all directions)
- Colors drain from environment
- Sounds don't match sources
- Companion voices distorted

**Ghost Towns**
- Perfectly preserved but empty
- Doors and windows that slam shut
- Signs of rapid abandonment
- Personal items scattered
- False human sounds from empty buildings

**Labyrinth Approaches**
- Stone corridors appear in open areas
- Walls that weren't there before
- Paths that lead in circles
- Dragon breath sounds in distance
- Architecture defying physics

### Horror Stage (Dread Level 4)
**The Dragon's Labyrinth**
- First-person perspective shift
- Stone corridors lit by flickering torches
- Stalking footsteps behind player
- Dead ends that weren't there before
- Dragon's breath getting closer

**Void Zones**
- Complete darkness beyond torch light
- Sounds with no sources
- Companion voices calling from wrong directions
- Reality completely broken
- Only survival instincts remain

## Biome Transformation Mechanics

### Progressive Corruption
Each biome has multiple states that progress with dread level:
1. **Base State**: Initial peaceful appearance
2. **Subtle Changes**: Small details that feel "off"
3. **Obvious Decay**: Clear signs of abandonment/corruption
4. **Nightmare State**: Reality-defying horror elements
5. **Void State**: Complete breakdown of normal world

### Audio Integration
Each biome has carefully crafted audio landscapes:
- **Peace**: Natural, comforting sounds
- **Unease**: Reduction in positive sounds, subtle wrong notes
- **Dread**: Ominous ambient sounds, false audio cues
- **Terror**: Distorted reality, sounds from wrong sources
- **Horror**: Proximity stalking audio, dragon presence

### Environmental Storytelling
Biomes tell the story without exposition:
- **Personal Items**: Scattered belongings showing rapid flight
- **Architecture**: Buildings that reflect inhabitant's mental state
- **Nature Response**: Plants and animals flee or corrupt
- **Weather**: Darkening skies, unnatural phenomena
- **Lighting**: Progressive dimming of all light sources

## Implementation Notes

### Procedural Generation
- Each biome template can generate multiple variations
- Dread level parameter drives all transformation decisions
- Consistent art style maintained across all states
- Performance optimized for mobile devices

### Asset Requirements
- Base terrain meshes for each biome type
- Corruption overlay textures
- Progressive lighting setups
- Ambient audio tracks per dread level
- Interactive object variations

### Narrative Integration
- Biomes respond to player choices and companion states
- Key story events can trigger biome transformations
- Multiple paths through same biome show different aspects
- Companion reactions vary based on biome corruption level

This biome system ensures that the world itself becomes a character in the horror progression, making the journey from peace to horror feel inevitable and deeply unsettling.