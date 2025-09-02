use bevy::prelude::*;
use bevy::audio::*;
use std::collections::HashMap;
use serde_json::Value;
use crate::world::resources::DreadLevel;
use crate::world::resources::GameState;

// Note: Audio assets are pre-generated during build time using FREESOUND_API_KEY
// Runtime audio system only plays pre-bundled audio files - no API calls needed

#[derive(Resource, Debug)]
pub struct ProceduralAudioSystem {
    pub current_ambience: Option<Handle<AudioSource>>,
    pub dread_music_handles: HashMap<DreadTheme, Handle<AudioSource>>,
    pub current_theme: DreadTheme,
    pub volume_multiplier: f32,
    pub audio_cache: HashMap<String, Handle<AudioSource>>,
    pub is_loading: bool,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum DreadTheme {
    Peace,      // Levels 1-20: Peaceful village music
    Unease,     // Levels 21-40: Subtle corruption, minor key
    Dread,      // Levels 41-60: Ominous atmosphere
    Terror,     // Levels 61-80: Active horror sounds
    Void,       // Levels 81-120: Reality breakdown
}

#[derive(Debug, Clone)]
pub struct AudioPrompt {
    pub theme: DreadTheme,
    pub description: String,
    pub duration_seconds: u32,
    pub style: AudioStyle,
}

#[derive(Debug, Clone)]
pub enum AudioStyle {
    Ambient,
    Musical,
    EffectLayer,
    Stinger,
}

impl Default for ProceduralAudioSystem {
    fn default() -> Self {
        Self {
            current_ambience: None,
            dread_music_handles: HashMap::new(),
            current_theme: DreadTheme::Peace,
            volume_multiplier: 0.7,
            audio_cache: HashMap::new(),
            is_loading: false,
        }
    }
}

impl ProceduralAudioSystem {
    pub fn get_theme_prompts() -> HashMap<DreadTheme, Vec<AudioPrompt>> {
        let mut prompts = HashMap::new();
        
        prompts.insert(DreadTheme::Peace, vec![
            AudioPrompt {
                theme: DreadTheme::Peace,
                description: "Peaceful medieval village ambience with gentle lute melodies, birds chirping, distant church bells, warm and welcoming atmosphere".to_string(),
                duration_seconds: 120,
                style: AudioStyle::Ambient,
            },
            AudioPrompt {
                theme: DreadTheme::Peace,
                description: "Hopeful fantasy adventure music with flutes and strings, suggesting the beginning of a heroic journey".to_string(),
                duration_seconds: 180,
                style: AudioStyle::Musical,
            }
        ]);
        
        prompts.insert(DreadTheme::Unease, vec![
            AudioPrompt {
                theme: DreadTheme::Unease,
                description: "Subtly unsettling ambience with distant thunder, wind through dead trees, occasional discordant notes mixed with fading village sounds".to_string(),
                duration_seconds: 150,
                style: AudioStyle::Ambient,
            },
            AudioPrompt {
                theme: DreadTheme::Unease,
                description: "Music transitioning from major to minor key, with strings becoming more tense and percussion adding subtle anxiety".to_string(),
                duration_seconds: 200,
                style: AudioStyle::Musical,
            }
        ]);
        
        prompts.insert(DreadTheme::Dread, vec![
            AudioPrompt {
                theme: DreadTheme::Dread,
                description: "Ominous atmospheric soundscape with deep drones, distant screams, unsettling whispers, and the sound of reality beginning to crack".to_string(),
                duration_seconds: 180,
                style: AudioStyle::Ambient,
            },
            AudioPrompt {
                theme: DreadTheme::Dread,
                description: "Dark orchestral horror music with dissonant strings, low brass, and percussion that suggests approaching doom".to_string(),
                duration_seconds: 240,
                style: AudioStyle::Musical,
            }
        ]);
        
        prompts.insert(DreadTheme::Terror, vec![
            AudioPrompt {
                theme: DreadTheme::Terror,
                description: "Active horror soundscape with monster roars, reality tearing sounds, chaotic percussion, and voices of the damned".to_string(),
                duration_seconds: 200,
                style: AudioStyle::Ambient,
            },
            AudioPrompt {
                theme: DreadTheme::Terror,
                description: "Intense horror music with frantic strings, pounding drums, discordant choir voices, and sounds of cosmic terror".to_string(),
                duration_seconds: 180,
                style: AudioStyle::Musical,
            }
        ]);
        
        prompts.insert(DreadTheme::Void, vec![
            AudioPrompt {
                theme: DreadTheme::Void,
                description: "Reality breakdown audio with reverse reverbs, impossible sounds, fragments of all previous themes distorted beyond recognition, and the void consuming everything".to_string(),
                duration_seconds: 240,
                style: AudioStyle::Ambient,
            },
            AudioPrompt {
                theme: DreadTheme::Void,
                description: "Post-apocalyptic nightmare music with heavily processed instruments, temporal audio distortions, and the sound of existence itself unraveling".to_string(),
                duration_seconds: 300,
                style: AudioStyle::Musical,
            }
        ]);
        
        prompts
    }
    
    pub fn determine_theme_from_dread_level(dread_level: f32) -> DreadTheme {
        match dread_level {
            0.0..=0.2 => DreadTheme::Peace,
            0.2..=0.4 => DreadTheme::Unease,
            0.4..=0.6 => DreadTheme::Dread,
            0.6..=0.8 => DreadTheme::Terror,
            _ => DreadTheme::Void,
        }
    }
}

pub fn update_procedural_audio(
    mut audio_system: ResMut<ProceduralAudioSystem>,
    dread_level: Res<DreadLevel>,
    mut commands: Commands,
    audio: Res<Audio>,
    game_state: Res<GameState>,
) {
    // Determine current theme based on dread level
    let new_theme = ProceduralAudioSystem::determine_theme_from_dread_level(dread_level.current);
    
    // Check if we need to transition to a new theme
    if new_theme != audio_system.current_theme {
        info!("Transitioning audio theme from {:?} to {:?}", audio_system.current_theme, new_theme);
        audio_system.current_theme = new_theme.clone();
        
        // Request new audio generation for this theme
        request_theme_audio(&mut audio_system, &new_theme);
    }
    
    // Update volume based on intensity
    let target_volume = calculate_dynamic_volume(&dread_level, &audio_system.current_theme);
    audio_system.volume_multiplier = audio_system.volume_multiplier * 0.95 + target_volume * 0.05; // Smooth transition
}

fn request_theme_audio(audio_system: &mut ProceduralAudioSystem, theme: &DreadTheme) {
    if audio_system.is_loading {
        return;
    }
    
    // Load pre-generated audio files based on theme
    // These files were created during build time using the audio API
    let audio_files = get_theme_audio_files(theme);
    
    for audio_file in audio_files {
        info!("Loading pre-generated audio: {}", audio_file);
        // Audio files are loaded from assets/audio/ directory
        // No runtime API calls - everything is bundled with the game
    }
    
    audio_system.is_loading = false;
}

fn get_theme_audio_files(theme: &DreadTheme) -> Vec<&'static str> {
    match theme {
        DreadTheme::Peace => vec![
            "audio/themes/peace_ambient.ogg",
            "audio/themes/peace_musical.ogg",
        ],
        DreadTheme::Unease => vec![
            "audio/themes/unease_ambient.ogg", 
            "audio/themes/unease_musical.ogg",
        ],
        DreadTheme::Dread => vec![
            "audio/themes/dread_ambient.ogg",
            "audio/themes/dread_musical.ogg", 
        ],
        DreadTheme::Terror => vec![
            "audio/themes/terror_ambient.ogg",
            "audio/themes/terror_musical.ogg",
        ],
        DreadTheme::Void => vec![
            "audio/themes/void_ambient.ogg",
            "audio/themes/void_musical.ogg",
        ],
    }
}

fn calculate_dynamic_volume(dread_level: &DreadLevel, theme: &DreadTheme) -> f32 {
    let base_volume = match theme {
        DreadTheme::Peace => 0.3,
        DreadTheme::Unease => 0.4,
        DreadTheme::Dread => 0.6,
        DreadTheme::Terror => 0.8,
        DreadTheme::Void => 1.0,
    };
    
    // Add some dynamic variation based on dread progression
    let variation = (dread_level.current * 10.0).sin() * 0.1;
    (base_volume + variation).clamp(0.1, 1.0)
}

pub fn play_contextual_audio_stinger(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio_system: Res<ProceduralAudioSystem>,
    event_type: AudioStingerType,
) {
    let audio_file = match event_type {
        AudioStingerType::BossEncounter => "audio/stingers/boss_encounter.ogg",
        AudioStingerType::CompanionFlee => "audio/stingers/companion_flee.ogg",
        AudioStingerType::DreadIncrease => "audio/stingers/dread_increase.ogg",
        AudioStingerType::VoidTear => "audio/stingers/void_tear.ogg",
        AudioStingerType::PlayerDeath => "audio/stingers/player_death.ogg",
    };
    
    let handle = asset_server.load(audio_file);
    commands.spawn(AudioBundle {
        source: handle,
        settings: PlaybackSettings {
            volume: Volume::new(audio_system.volume_multiplier * 0.8),
            mode: PlaybackMode::Despawn,
            ..default()
        },
    });
}

#[derive(Debug, Clone)]
pub enum AudioStingerType {
    BossEncounter,
    CompanionFlee,
    DreadIncrease,
    VoidTear,
    PlayerDeath,
}

// System to handle menu audio
pub fn play_menu_audio(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio_system: Res<ProceduralAudioSystem>,
) {
    // Play main menu ambience
    let menu_audio = asset_server.load("audio/menu/dark_ambience.ogg");
    commands.spawn(AudioBundle {
        source: menu_audio,
        settings: PlaybackSettings {
            volume: Volume::new(0.5),
            mode: PlaybackMode::Loop,
            ..default()
        },
    });
}

// System to handle character creation audio
pub fn play_character_creation_audio(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Slightly hopeful but foreboding character creation music
    let creation_audio = asset_server.load("audio/character_creation/hopeful_dread.ogg");
    commands.spawn(AudioBundle {
        source: creation_audio,
        settings: PlaybackSettings {
            volume: Volume::new(0.4),
            mode: PlaybackMode::Loop,
            ..default()
        },
    });
}

// Audio event system for UI interactions
pub fn play_ui_sound_effects(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
) {
    for interaction in &interaction_query {
        let sound_file = match interaction {
            Interaction::Hovered => "audio/ui/button_hover.ogg",
            Interaction::Pressed => "audio/ui/button_press.ogg",
            _ => continue,
        };
        
        let handle = asset_server.load(sound_file);
        commands.spawn(AudioBundle {
            source: handle,
            settings: PlaybackSettings {
                volume: Volume::new(0.3),
                mode: PlaybackMode::Despawn,
                ..default()
            },
        });
    }
}