use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use hexx::Hex;
use std::collections::HashMap;

/// Proximity horror audio system using bevy_kira_audio
/// This implements the AudioAgent's spatial audio requirements
pub struct ProximityAudioPlugin;

impl Plugin for ProximityAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AudioPlugin)
            .init_resource::<ProximityAudioAssets>()
            .init_resource::<AudioConfiguration>()
            .init_resource::<HallucinationAudioState>()
            .add_event::<ProximityAudioEvent>()
            .add_systems(Startup, setup_audio_channels)
            .add_systems(Update, (
                update_dragon_proximity_audio,
                process_companion_trauma_audio,
                handle_hallucination_audio,
                update_ambient_soundscape,
                process_proximity_events,
            ).chain());
    }
}

/// Audio assets organized by dread level and type
#[derive(Resource, Default)]
pub struct ProximityAudioAssets {
    /// Dragon proximity sounds for each dread level
    pub dragon_sounds: HashMap<u8, Vec<Handle<AudioSource>>>,
    /// Companion trauma voice lines
    pub companion_voices: HashMap<(String, u8), Handle<AudioSource>>,
    /// Ambient soundscapes per dread level
    pub ambient_tracks: HashMap<u8, Handle<AudioSource>>,
    /// Hallucination sounds (false positives)
    pub hallucination_sounds: Vec<Handle<AudioSource>>,
    /// Heartbeat sounds of increasing intensity
    pub heartbeat_tracks: HashMap<u8, Handle<AudioSource>>,
}

/// Configuration for spatial audio
#[derive(Resource)]
pub struct AudioConfiguration {
    /// Maximum distance for audio to be heard
    pub max_audio_distance: f32,
    /// Dragon proximity threshold for triggering audio
    pub dragon_proximity_threshold: f32,
    /// Volume falloff curve exponent
    pub falloff_exponent: f32,
    /// Current sanity level (affects hallucinations)
    pub sanity_level: f32,
}

impl Default for AudioConfiguration {
    fn default() -> Self {
        Self {
            max_audio_distance: 50.0,
            dragon_proximity_threshold: 30.0,
            falloff_exponent: 2.0,
            sanity_level: 1.0,
        }
    }
}

/// State for managing audio hallucinations
#[derive(Resource, Default)]
pub struct HallucinationAudioState {
    /// Time since last hallucination
    pub time_since_last: f32,
    /// Frequency of hallucinations based on dread
    pub hallucination_frequency: f32,
    /// Currently playing hallucination handles
    pub active_hallucinations: Vec<Handle<AudioInstance>>,
}

/// Events for proximity-triggered audio
#[derive(Event)]
pub struct ProximityAudioEvent {
    pub audio_type: ProximityAudioType,
    pub source_position: Vec3,
    pub intensity: f32,
    pub should_loop: bool,
}

#[derive(Debug, Clone)]
pub enum ProximityAudioType {
    DragonBreathing,
    DragonFootsteps,
    DragonRoar,
    CompanionWhimper(String),
    CompanionScream(String),
    EnvironmentalCreak,
    Hallucination,
}

/// Audio channel assignments for layered playback
fn setup_audio_channels(
    mut commands: Commands,
    audio: Res<Audio>,
) {
    // Create separate audio channels for different sound types
    let ambient_channel = audio.create_channel();
    let proximity_channel = audio.create_channel();
    let voice_channel = audio.create_channel();
    let hallucination_channel = audio.create_channel();
    
    commands.insert_resource(AudioChannels {
        ambient: ambient_channel,
        proximity: proximity_channel,
        voice: voice_channel,
        hallucination: hallucination_channel,
    });
}

#[derive(Resource)]
pub struct AudioChannels {
    pub ambient: AudioChannel,
    pub proximity: AudioChannel,
    pub voice: AudioChannel,
    pub hallucination: AudioChannel,
}

/// Update dragon proximity audio based on distance
fn update_dragon_proximity_audio(
    player_query: Query<&Transform, With<crate::components::Player>>,
    dragon_query: Query<&Transform, With<crate::components::Dragon>>,
    audio_assets: Res<ProximityAudioAssets>,
    audio_config: Res<AudioConfiguration>,
    audio: Res<AudioChannels>,
    dread_state: Res<crate::resources::DreadState>,
    mut last_distance: Local<f32>,
) {
    let Ok(player_transform) = player_query.get_single() else { return };
    let Ok(dragon_transform) = dragon_query.get_single() else { return };
    
    let distance = player_transform.translation.distance(dragon_transform.translation);
    
    // Only play audio when dragon is within threshold
    if distance <= audio_config.dragon_proximity_threshold {
        let intensity = 1.0 - (distance / audio_config.dragon_proximity_threshold);
        let volume = intensity.powf(audio_config.falloff_exponent);
        
        // Select appropriate dragon sound based on dread level and distance
        if let Some(dragon_sounds) = audio_assets.dragon_sounds.get(&dread_state.current_level) {
            let sound_index = match distance {
                d if d < 5.0 => 2,  // Very close - roar
                d if d < 15.0 => 1, // Medium - footsteps
                _ => 0,             // Far - breathing
            };
            
            if let Some(sound) = dragon_sounds.get(sound_index) {
                // Only play if distance changed significantly to avoid spam
                if (distance - *last_distance).abs() > 1.0 {
                    audio.proximity
                        .play(sound.clone())
                        .with_volume(Volume::new(volume as f64))
                        .with_panning(calculate_panning(player_transform, dragon_transform))
                        .handle();
                    
                    *last_distance = distance;
                }
            }
        }
        
        // Add heartbeat intensity based on proximity
        if distance < 10.0 {
            if let Some(heartbeat) = audio_assets.heartbeat_tracks.get(&dread_state.current_level) {
                let heartbeat_volume = ((10.0 - distance) / 10.0) as f64;
                audio.proximity
                    .play(heartbeat.clone())
                    .with_volume(Volume::new(heartbeat_volume))
                    .looped()
                    .handle();
            }
        }
    }
}

/// Process companion trauma audio responses
fn process_companion_trauma_audio(
    companion_query: Query<(&Transform, &crate::components::Companion), Changed<crate::components::Companion>>,
    player_query: Query<&Transform, With<crate::components::Player>>,
    audio_assets: Res<ProximityAudioAssets>,
    audio_config: Res<AudioConfiguration>,
    audio: Res<AudioChannels>,
) {
    let Ok(player_transform) = player_query.get_single() else { return };
    
    for (companion_transform, companion) in companion_query.iter() {
        let distance = player_transform.translation.distance(companion_transform.translation);
        
        if distance <= audio_config.max_audio_distance {
            // Get trauma-appropriate voice line
            let trauma_level = (companion.trauma * 5.0) as u8;
            let voice_key = (companion.name.clone(), trauma_level);
            
            if let Some(voice_line) = audio_assets.companion_voices.get(&voice_key) {
                let volume = calculate_volume_from_distance(distance, audio_config.max_audio_distance);
                
                audio.voice
                    .play(voice_line.clone())
                    .with_volume(Volume::new(volume))
                    .with_panning(calculate_panning(player_transform, companion_transform))
                    .handle();
            }
        }
    }
}

/// Handle sanity-based audio hallucinations
fn handle_hallucination_audio(
    mut hallucination_state: ResMut<HallucinationAudioState>,
    audio_config: Res<AudioConfiguration>,
    audio_assets: Res<ProximityAudioAssets>,
    audio: Res<AudioChannels>,
    dread_state: Res<crate::resources::DreadState>,
    time: Res<Time>,
) {
    hallucination_state.time_since_last += time.delta_secs();
    
    // Calculate hallucination frequency based on sanity and dread
    let sanity_multiplier = 2.0 - audio_config.sanity_level;
    let dread_multiplier = 1.0 + (dread_state.current_level as f32 * 0.5);
    hallucination_state.hallucination_frequency = sanity_multiplier * dread_multiplier;
    
    // Trigger hallucinations based on frequency
    let hallucination_interval = 10.0 / hallucination_state.hallucination_frequency.max(0.1);
    
    if hallucination_state.time_since_last >= hallucination_interval {
        if !audio_assets.hallucination_sounds.is_empty() {
            // Play random hallucination sound
            let sound_index = (time.elapsed_secs() as usize) % audio_assets.hallucination_sounds.len();
            let sound = &audio_assets.hallucination_sounds[sound_index];
            
            // Random panning to make it seem like it's coming from different directions
            let random_pan = (time.elapsed_secs().sin() * 0.7) as f64;
            
            let handle = audio.hallucination
                .play(sound.clone())
                .with_volume(Volume::new(0.3 + (audio_config.sanity_level * 0.4) as f64))
                .with_panning(random_pan)
                .handle();
            
            hallucination_state.active_hallucinations.push(handle);
            hallucination_state.time_since_last = 0.0;
        }
    }
    
    // Clean up finished hallucination sounds
    hallucination_state.active_hallucinations.retain(|handle| {
        audio.hallucination.state(handle).is_playing()
    });
}

/// Update ambient soundscape based on dread level
fn update_ambient_soundscape(
    audio_assets: Res<ProximityAudioAssets>,
    audio: Res<AudioChannels>,
    dread_state: Res<crate::resources::DreadState>,
    mut current_ambient: Local<Option<(u8, Handle<AudioInstance>)>>,
) {
    // Check if we need to change ambient track
    if let Some((current_dread, _)) = &current_ambient {
        if *current_dread == dread_state.current_level {
            return; // Already playing correct ambient
        }
    }
    
    // Stop current ambient if playing
    if let Some((_, handle)) = current_ambient.take() {
        audio.ambient.stop(handle);
    }
    
    // Start new ambient track
    if let Some(ambient_track) = audio_assets.ambient_tracks.get(&dread_state.current_level) {
        let handle = audio.ambient
            .play(ambient_track.clone())
            .with_volume(Volume::new(0.5))
            .looped()
            .handle();
        
        *current_ambient = Some((dread_state.current_level, handle));
    }
}

/// Process proximity audio events from other systems
fn process_proximity_events(
    mut events: EventReader<ProximityAudioEvent>,
    player_query: Query<&Transform, With<crate::components::Player>>,
    audio_assets: Res<ProximityAudioAssets>,
    audio_config: Res<AudioConfiguration>,
    audio: Res<AudioChannels>,
    asset_server: Res<AssetServer>,
) {
    let Ok(player_transform) = player_query.get_single() else { return };
    
    for event in events.read() {
        let distance = player_transform.translation.distance(event.source_position);
        
        if distance <= audio_config.max_audio_distance {
            let volume = calculate_volume_from_distance(distance, audio_config.max_audio_distance) * event.intensity;
            let panning = calculate_panning_from_positions(
                player_transform.translation,
                event.source_position,
            );
            
            // Play the appropriate sound based on event type
            let sound_handle = match &event.audio_type {
                ProximityAudioType::DragonBreathing => {
                    asset_server.load("audio/dragon_breathing.ogg")
                }
                ProximityAudioType::DragonFootsteps => {
                    asset_server.load("audio/dragon_footsteps.ogg")
                }
                ProximityAudioType::DragonRoar => {
                    asset_server.load("audio/dragon_roar.ogg")
                }
                ProximityAudioType::CompanionWhimper(name) => {
                    asset_server.load(format!("audio/companion_{}_whimper.ogg", name))
                }
                ProximityAudioType::CompanionScream(name) => {
                    asset_server.load(format!("audio/companion_{}_scream.ogg", name))
                }
                ProximityAudioType::EnvironmentalCreak => {
                    asset_server.load("audio/environmental_creak.ogg")
                }
                ProximityAudioType::Hallucination => {
                    asset_server.load("audio/hallucination_whisper.ogg")
                }
            };
            
            let mut playback = audio.proximity
                .play(sound_handle)
                .with_volume(Volume::new(volume as f64))
                .with_panning(panning);
            
            if event.should_loop {
                playback = playback.looped();
            }
            
            playback.handle();
        }
    }
}

// Helper functions

fn calculate_volume_from_distance(distance: f32, max_distance: f32) -> f32 {
    if distance >= max_distance {
        0.0
    } else {
        1.0 - (distance / max_distance).powf(2.0)
    }
}

fn calculate_panning(listener: &Transform, source: &Transform) -> f64 {
    let direction = source.translation - listener.translation;
    let forward = listener.forward();
    let right = listener.right();
    
    let dot_right = direction.normalize().dot(*right);
    dot_right.clamp(-1.0, 1.0) as f64
}

fn calculate_panning_from_positions(listener_pos: Vec3, source_pos: Vec3) -> f64 {
    let direction = source_pos - listener_pos;
    // Simplified panning based on X difference
    (direction.x / 10.0).clamp(-1.0, 1.0) as f64
}

/// Load AI-generated audio assets
pub fn load_ai_generated_audio(
    mut audio_assets: ResMut<ProximityAudioAssets>,
    asset_server: Res<AssetServer>,
    generated_assets: Res<crate::resources::GeneratedAssets>,
) {
    // Load dragon sounds for each dread level
    for dread_level in 0..=4 {
        let mut sounds = Vec::new();
        sounds.push(asset_server.load(format!("audio/dragon_breathing_dread_{}.ogg", dread_level)));
        sounds.push(asset_server.load(format!("audio/dragon_footsteps_dread_{}.ogg", dread_level)));
        sounds.push(asset_server.load(format!("audio/dragon_roar_dread_{}.ogg", dread_level)));
        audio_assets.dragon_sounds.insert(dread_level, sounds);
        
        // Load ambient tracks
        let ambient = asset_server.load(format!("audio/ambient_dread_{}.ogg", dread_level));
        audio_assets.ambient_tracks.insert(dread_level, ambient);
        
        // Load heartbeat tracks
        let heartbeat = asset_server.load(format!("audio/heartbeat_dread_{}.ogg", dread_level));
        audio_assets.heartbeat_tracks.insert(dread_level, heartbeat);
    }
    
    // Load companion trauma voice lines
    for companion_name in ["einar", "mira", "sorin", "tamara"].iter() {
        for trauma_level in 0..=5 {
            let voice_line = asset_server.load(
                format!("audio/companion_{}_{}.ogg", companion_name, trauma_level)
            );
            audio_assets.companion_voices.insert(
                (companion_name.to_string(), trauma_level),
                voice_line,
            );
        }
    }
    
    // Load hallucination sounds
    for i in 0..10 {
        let hallucination = asset_server.load(format!("audio/hallucination_{}.ogg", i));
        audio_assets.hallucination_sounds.push(hallucination);
    }
}
