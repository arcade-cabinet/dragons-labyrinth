"""
Audio Agent - AI-powered proximity horror audio generation for Dragon's Labyrinth
Uses OpenAI to generate spatial audio configurations and horror soundscapes
"""

import os
import json
import sqlite3
from pathlib import Path
from typing import Optional
from dataclasses import dataclass, asdict
from datetime import datetime

from openai import OpenAI


@dataclass
class AudioConfig:
    """Configuration for audio generation"""
    audio_type: str  # ambient, proximity, companion_voice, ui_feedback, dragon_presence
    dread_level: int  # 0-4
    distance: float = 10.0  # Distance from source (for proximity audio)
    intensity: float = 0.5  # 0.0-1.0 intensity level
    false_positive: bool = False  # Generate hallucination sounds
    environment: str = "outdoor"  # outdoor, cave, labyrinth, village


class AudioAgent:
    """AI agent for generating horror audio configurations"""
    
    def __init__(self, db_path: str = "assets/assets.db"):
        self.client = OpenAI(api_key=os.environ.get("OPENAI_API_KEY"))
        self.db_path = db_path
        self._init_database()
    
    def _init_database(self):
        """Initialize audio generation tracking"""
        conn = sqlite3.connect(self.db_path)
        cursor = conn.cursor()
        
        cursor.execute('''
            CREATE TABLE IF NOT EXISTS audio_generations (
                generation_id TEXT PRIMARY KEY,
                audio_type TEXT NOT NULL,
                dread_level INTEGER,
                prompt TEXT,
                response TEXT,
                audio_data TEXT,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )
        ''')
        
        conn.commit()
        conn.close()
    
    def generate_audio_config(self, config: AudioConfig) -> dict[str, any]:
        """
        Generate audio configuration using AI
        
        Returns:
            Dictionary with audio specifications and spatial settings
        """
        # Create horror-aware prompt
        prompt = self._create_audio_prompt(config)
        
        try:
            # Call OpenAI for audio generation
            response = self.client.chat.completions.create(
                model="gpt-4o",
                messages=[
                    {"role": "system", "content": self._get_system_prompt()},
                    {"role": "user", "content": prompt}
                ],
                temperature=0.7,
                response_format={"type": "json_object"}
            )
            
            # Parse response
            audio_data = json.loads(response.choices[0].message.content)
            
            # Record generation
            generation_id = f"audio_{config.audio_type}_{datetime.now().strftime('%Y%m%d_%H%M%S')}"
            self._record_generation(generation_id, config, prompt, response.choices[0].message.content, audio_data)
            
            return {
                "success": True,
                "generation_id": generation_id,
                "audio_data": audio_data,
                "config": asdict(config)
            }
            
        except Exception as e:
            return {
                "success": False,
                "error": str(e),
                "config": asdict(config)
            }
    
    def _get_system_prompt(self) -> str:
        """Get system prompt for audio generation"""
        return """You are an audio designer for a horror RPG called Dragon's Labyrinth.
        
        The game has 5 dread levels (0-4):
        - 0: Peace - Natural sounds, birds, water
        - 1: Unease - Sounds slightly off, too quiet
        - 2: Dread - Ominous ambience, distant threats
        - 3: Terror - Distorted reality, proximity threats
        - 4: Horror - Dragon breathing, overwhelming presence
        
        Audio types:
        - Ambient: Background atmosphere
        - Proximity: Distance-based intensity (dragon, threats)
        - Companion_voice: Character voice modulation
        - UI_feedback: Interface sound effects
        - Dragon_presence: The dragon hunting you
        
        Generate spatial audio configurations for horror progression.
        
        Return JSON with:
        {
            "base_audio": {
                "description": "...",
                "freesound_tags": ["medieval", "forest", "birds"],
                "volume": 0.5,
                "loop": true
            },
            "spatial_config": {
                "falloff_distance": 20.0,
                "min_distance": 1.0,
                "max_distance": 50.0,
                "rolloff_factor": 1.0,
                "doppler_scale": 1.0
            },
            "dread_modifications": [
                {
                    "trigger": "dread_level_2",
                    "effect": "pitch_shift",
                    "amount": -0.2,
                    "description": "Lower pitch creates unease"
                }
            ],
            "proximity_layers": [
                {
                    "distance_range": [0, 10],
                    "intensity": 1.0,
                    "effects": ["heartbeat", "breathing"],
                    "panic_level": "extreme"
                }
            ],
            "false_positives": [
                {
                    "chance": 0.1,
                    "sound": "footsteps_behind",
                    "duration": 2.0,
                    "description": "Player hears following when nothing there"
                }
            ],
            "bevy_audio_config": {
                "source_type": "spatial",
                "attenuation": "inverse_distance",
                "panning": "balanced"
            }
        }
        """
    
    def _create_audio_prompt(self, config: AudioConfig) -> str:
        """Create prompt for audio generation"""
        audio_types = {
            "ambient": "Background environmental soundscape",
            "proximity": "Distance-based threat audio",
            "companion_voice": "Character voice processing",
            "ui_feedback": "Interface interaction sounds",
            "dragon_presence": "The dragon hunting the player"
        }
        
        dread_descriptions = {
            0: "peaceful and natural",
            1: "subtly wrong, uncanny valley",
            2: "openly threatening atmosphere",
            3: "reality breaking down, intense fear",
            4: "pure nightmare, dragon's breath on your neck"
        }
        
        environment_acoustics = {
            "outdoor": "open space with natural reverb",
            "cave": "enclosed with echo and dripping",
            "labyrinth": "stone corridors with confusion",
            "village": "mix of indoor/outdoor spaces"
        }
        
        return f"""Generate audio configuration for Dragon's Labyrinth.
        
        Audio Type: {config.audio_type} - {audio_types.get(config.audio_type, 'Custom audio')}
        Dread Level: {config.dread_level} - {dread_descriptions[config.dread_level]}
        Distance: {config.distance}m from source
        Intensity: {config.intensity:.1%}
        False Positive: {config.false_positive} (hallucination sounds)
        Environment: {config.environment} - {environment_acoustics.get(config.environment, '')}
        
        Create audio design that:
        1. Reflects the current dread level appropriately
        2. Uses spatial audio for proximity effects at {config.distance}m
        3. Includes false positives: {config.false_positive}
        4. Matches {config.environment} acoustic properties
        5. Scales intensity to {config.intensity:.1%}
        
        Include Freesound API tags for searching CC0 audio.
        """
    
    def _record_generation(self, generation_id: str, config: AudioConfig, prompt: str, response: str, audio_data: dict):
        """Record generation in database"""
        conn = sqlite3.connect(self.db_path)
        cursor = conn.cursor()
        
        cursor.execute('''
            INSERT INTO audio_generations 
            (generation_id, audio_type, dread_level, prompt, response, audio_data)
            VALUES (?, ?, ?, ?, ?, ?)
        ''', (generation_id, config.audio_type, config.dread_level, prompt, response, json.dumps(audio_data)))
        
        conn.commit()
        conn.close()
    
    def generate_bevy_audio_system(self, audio_data: dict, output_path: str = "src/audio/generated/") -> str:
        """
        Generate Bevy audio system code
        
        Returns:
            Path to generated Rust audio system file
        """
        Path(output_path).mkdir(parents=True, exist_ok=True)
        
        audio_type = audio_data.get("config", {}).get("audio_type", "unknown")
        safe_name = audio_type.lower().replace("_", "")
        
        rust_code = f'''// Auto-generated Bevy audio system for {audio_type}
use bevy::prelude::*;
use bevy::audio::{{PlaybackSettings, SpatialSettings}};

#[derive(Resource)]
pub struct {safe_name.capitalize()}AudioConfig {{
    pub base_volume: f32,
    pub dread_multiplier: f32,
    pub spatial_scale: f32,
}}

impl Default for {safe_name.capitalize()}AudioConfig {{
    fn default() -> Self {{
        Self {{
            base_volume: 0.5,
            dread_multiplier: 1.0,
            spatial_scale: 1.0,
        }}
    }}
}}

pub fn setup_{safe_name}_audio(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {{
    // Load audio assets
    let audio_handle = asset_server.load("audio/{safe_name}_base.ogg");
    
    commands.insert_resource({safe_name.capitalize()}AudioConfig::default());
}}

pub fn update_{safe_name}_proximity(
    mut commands: Commands,
    audio_config: Res<{safe_name.capitalize()}AudioConfig>,
    player: Query<&Transform, With<Player>>,
    audio_sources: Query<(Entity, &Transform), With<AudioSource>>,
    dread: Res<DreadState>,
    asset_server: Res<AssetServer>,
) {{
    let Ok(player_transform) = player.get_single() else {{ return; }};
    
    for (entity, source_transform) in audio_sources.iter() {{
        let distance = player_transform.translation.distance(source_transform.translation);
        
        // Calculate volume based on distance and dread
        let base_falloff = 1.0 / (1.0 + distance * 0.1);
        let dread_modifier = 1.0 + (dread.level as f32 * 0.3);
        let volume = audio_config.base_volume * base_falloff * dread_modifier;
        
        // Apply spatial audio settings
        let spatial_settings = SpatialSettings {{
            position: source_transform.translation,
            velocity: Vec3::ZERO,
            ..default()
        }};
        
        // Update or spawn audio
        commands.entity(entity).insert((
            AudioBundle {{
                source: asset_server.load(get_dread_audio_path(dread.level)),
                settings: PlaybackSettings {{
                    mode: bevy::audio::PlaybackMode::Loop,
                    volume: bevy::audio::Volume::new(volume),
                    ..default()
                }}.with_spatial(spatial_settings),
            }},
        ));
    }}
}}

fn get_dread_audio_path(dread_level: u8) -> &'static str {{
    match dread_level {{
        0 => "audio/{safe_name}_peace.ogg",
        1 => "audio/{safe_name}_unease.ogg",
        2 => "audio/{safe_name}_dread.ogg",
        3 => "audio/{safe_name}_terror.ogg",
        4 => "audio/{safe_name}_horror.ogg",
        _ => "audio/{safe_name}_base.ogg",
    }}
}}

pub fn spawn_false_positive_audio(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player: Query<&Transform, With<Player>>,
    dread: Res<DreadState>,
    mut timer: Local<Timer>,
    time: Res<Time>,
) {{
    if dread.level < 2 {{ return; }} // No false positives in early game
    
    timer.tick(time.delta());
    if !timer.finished() {{ return; }}
    
    // Reset timer with random interval
    timer.set_duration(std::time::Duration::from_secs_f32(
        10.0 + rand::random::<f32>() * 20.0
    ));
    timer.reset();
    
    // Chance of false positive increases with dread
    let chance = 0.1 * dread.level as f32;
    if rand::random::<f32>() > chance {{ return; }}
    
    let Ok(player_transform) = player.get_single() else {{ return; }};
    
    // Spawn false positive behind player
    let offset = Vec3::new(
        rand::random::<f32>() * 10.0 - 5.0,
        0.0,
        -10.0,
    );
    
    commands.spawn((
        AudioBundle {{
            source: asset_server.load("audio/false_positive_footsteps.ogg"),
            settings: PlaybackSettings {{
                mode: bevy::audio::PlaybackMode::Despawn,
                volume: bevy::audio::Volume::new(0.3),
                ..default()
            }}.with_spatial(SpatialSettings {{
                position: player_transform.translation + offset,
                ..default()
            }}),
        }},
        FalsePositiveAudio,
    ));
}}

#[derive(Component)]
struct FalsePositiveAudio;
'''
        
        # Write to file
        file_path = Path(output_path) / f"{safe_name}_audio.rs"
        file_path.write_text(rust_code)
        
        return str(file_path)
    
    def generate_proximity_horror_suite(self) -> list[dict]:
        """
        Generate complete proximity horror audio suite
        
        Returns:
            List of generation results for all audio types
        """
        audio_configs = [
            # Dragon proximity at different distances
            AudioConfig("dragon_presence", 4, 5.0, 1.0, False, "labyrinth"),
            AudioConfig("dragon_presence", 4, 20.0, 0.5, False, "labyrinth"),
            AudioConfig("dragon_presence", 4, 50.0, 0.2, False, "labyrinth"),
            
            # Ambient for each dread level
            AudioConfig("ambient", 0, 100.0, 0.3, False, "outdoor"),
            AudioConfig("ambient", 1, 100.0, 0.4, False, "outdoor"),
            AudioConfig("ambient", 2, 100.0, 0.5, False, "village"),
            AudioConfig("ambient", 3, 100.0, 0.7, True, "cave"),
            AudioConfig("ambient", 4, 100.0, 1.0, True, "labyrinth"),
            
            # UI feedback with degradation
            AudioConfig("ui_feedback", 0, 0.0, 1.0, False, "outdoor"),
            AudioConfig("ui_feedback", 2, 0.0, 0.8, False, "outdoor"),
            AudioConfig("ui_feedback", 4, 0.0, 0.5, True, "outdoor"),
        ]
        
        results = []
        for config in audio_configs:
            result = self.generate_audio_config(config)
            results.append(result)
            
            if result["success"]:
                # Generate Bevy system
                system_path = self.generate_bevy_audio_system(result)
                result["system_path"] = system_path
        
        return results
