# Asset Curation Task: Audio & Horror Soundscape

## Context
Audio is critical for horror. We need sounds that build tension, create false positives (hallucinations), and respond to dread progression.

## Your Task
Evaluate audio files for horror appropriateness and gameplay integration. Consider both diegetic (in-world) and non-diegetic (soundtrack) audio.

## Assets to Review
Paths:
- `ordered/assets/library/audio/`
- Any .ogg, .wav, .mp3 files throughout the library

## Audio Categories Needed

### 1. Ambient Soundscapes
- **Peace (Dread 0)**: Birds, wind, village life
- **Unease (Dread 1)**: Quieter birds, distant sounds
- **Anxiety (Dread 2)**: Silence punctuated by sudden sounds
- **Horror (Dread 3)**: Whispers, breathing, scratching
- **Madness (Dread 4)**: Reality distortion, impossible sounds

### 2. Dragon Presence
- Breathing (distant to close)
- Wing beats
- Roars/growls
- Movement sounds
- False positives (sounds like dragon but isn't)

### 3. Companion Reactions
- Gasps
- Crying
- Laughing (inappropriate times)
- Screaming
- Breaking down

### 4. Environmental
- Footsteps (various surfaces)
- Doors creaking
- Water dripping
- Wind through cracks
- Fire crackling
- Chains rattling

### 5. Combat/Action
- Weapon sounds
- Impact sounds
- Magic effects
- Death sounds

## Hallucination Audio
Some sounds should only play when sanity is low:
- Children laughing
- Dead companions speaking
- Your own voice
- Sounds from memories

## Technical Implementation
```rust
// Audio Type: [category]
// Asset: [filename]
// Dread Range: [0-4 or specific]
// Hallucination: [true/false]

pub struct HorrorAudioBank {
    pub ambient: HashMap<DreadLevel, Vec<Handle<AudioSource>>>,
    pub stingers: Vec<AudioStinger>,
    pub hallucinations: Vec<HallucinationAudio>,
}

pub struct AudioStinger {
    pub sound: Handle<AudioSource>,
    pub trigger: StingerTrigger,
    pub cooldown: f32,
}

pub enum StingerTrigger {
    RandomChance(f32),
    ProximityToThreat(f32),
    CompanionBreakdown,
    PlayerLowSanity,
    DreadIncrease,
}

pub struct HallucinationAudio {
    pub sound: Handle<AudioSource>,
    pub sanity_threshold: f32,
    pub fake_source: Option<Vec3>, // Sounds like it's coming from somewhere
}

impl HorrorAudioBank {
    pub fn play_contextual(
        &self,
        dread: u8,
        sanity: f32,
        audio: &Audio,
    ) {
        // Select and play appropriate audio
        // Layer multiple tracks
        // Apply effects based on state
    }
}
```

## Spatial Audio Notes
- Use bevy_kira_audio for advanced mixing
- 3D positional audio for in-world sounds
- Reverb in dungeons
- Muffling through walls
- Doppler effects for moving threats

## Questions to Answer
1. Do we have enough ambient loops?
2. Are there good stinger sounds for scares?
3. Can we create a "dragon approaching" soundscape?
4. Do we need to generate/record custom audio?

Move selected assets to: `crates/game-engine/assets/audio/`
