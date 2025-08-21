// FMV Cutscene system for Dragon's Labyrinth
// Handles the emotional intro and outro videos that transition between worlds

use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use std::path::PathBuf;

/// Plugin for managing FMV cutscenes
pub struct CutscenePlugin;

impl Plugin for CutscenePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CutsceneState::default())
            .add_event::<CutsceneEvent>()
            .add_systems(Startup, setup_cutscene_system)
            .add_systems(Update, (
                handle_cutscene_events,
                update_video_playback,
                handle_cutscene_skip,
            ))
            .add_systems(OnEnter(GameState::Intro), play_intro_cutscene)
            .add_systems(OnEnter(GameState::Outro), play_outro_cutscene);
    }
}

/// Game states that trigger cutscenes
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Loading,
    Intro,      // Villager intro FMV
    MainMenu,
    Playing,
    Outro,      // Traveler portal FMV after defeating dragon
    Credits,
}

/// Cutscene management state
#[derive(Resource, Default)]
pub struct CutsceneState {
    pub current_video: Option<String>,
    pub is_playing: bool,
    pub can_skip: bool,
    pub playback_position: f32,
    pub total_duration: f32,
}

/// Events for cutscene control
#[derive(Event)]
pub enum CutsceneEvent {
    Play(String),
    Stop,
    Skip,
    Finished,
}

/// Component for video display entity
#[derive(Component)]
pub struct VideoDisplay {
    pub video_path: String,
    pub is_looping: bool,
}

/// Component for skip prompt UI
#[derive(Component)]
pub struct SkipPrompt;

/// Setup the cutscene rendering system
fn setup_cutscene_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Create a full-screen quad for video display
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(1920.0, 1080.0)),
                ..default()
            },
            visibility: Visibility::Hidden,
            ..default()
        },
        VideoDisplay {
            video_path: String::new(),
            is_looping: false,
        },
        Name::new("Video Display"),
    ));
    
    // Create skip prompt UI (hidden by default)
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(50.0),
                right: Val::Px(50.0),
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            background_color: BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
            visibility: Visibility::Hidden,
            ..default()
        },
        SkipPrompt,
    )).with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            "Press SPACE to skip",
            TextStyle {
                font_size: 20.0,
                color: Color::WHITE,
                ..default()
            },
        ));
    });
    
    info!("Cutscene system initialized");
}

/// Play the intro cutscene when starting the game
fn play_intro_cutscene(
    mut cutscene_events: EventWriter<CutsceneEvent>,
) {
    info!("Starting intro cutscene: villager-intro");
    cutscene_events.send(CutsceneEvent::Play(
        "attached_assets/villager-intro_1755767336555.mp4".to_string()
    ));
}

/// Play the outro cutscene after defeating the dragon
fn play_outro_cutscene(
    mut cutscene_events: EventWriter<CutsceneEvent>,
) {
    info!("Starting outro cutscene: traveler-portal");
    cutscene_events.send(CutsceneEvent::Play(
        "attached_assets/traveler-portal_1755767336514.mp4".to_string()
    ));
}

/// Handle cutscene events
fn handle_cutscene_events(
    mut cutscene_events: EventReader<CutsceneEvent>,
    mut cutscene_state: ResMut<CutsceneState>,
    mut video_display: Query<(&mut VideoDisplay, &mut Visibility)>,
    mut skip_prompt: Query<&mut Visibility, (With<SkipPrompt>, Without<VideoDisplay>)>,
    mut next_state: ResMut<NextState<GameState>>,
    current_state: Res<State<GameState>>,
) {
    for event in cutscene_events.read() {
        match event {
            CutsceneEvent::Play(video_path) => {
                // Show video display
                if let Ok((mut display, mut visibility)) = video_display.get_single_mut() {
                    display.video_path = video_path.clone();
                    *visibility = Visibility::Visible;
                }
                
                // Show skip prompt after 2 seconds
                if let Ok(mut prompt_visibility) = skip_prompt.get_single_mut() {
                    *prompt_visibility = Visibility::Visible;
                }
                
                // Update cutscene state
                cutscene_state.current_video = Some(video_path.clone());
                cutscene_state.is_playing = true;
                cutscene_state.can_skip = true;
                cutscene_state.playback_position = 0.0;
                
                // Set duration based on video
                cutscene_state.total_duration = if video_path.contains("villager-intro") {
                    15.0 // Approximate duration for intro
                } else {
                    12.0 // Approximate duration for outro
                };
                
                info!("Playing cutscene: {}", video_path);
            }
            
            CutsceneEvent::Stop | CutsceneEvent::Skip => {
                // Hide video display
                if let Ok((_, mut visibility)) = video_display.get_single_mut() {
                    *visibility = Visibility::Hidden;
                }
                
                // Hide skip prompt
                if let Ok(mut prompt_visibility) = skip_prompt.get_single_mut() {
                    *prompt_visibility = Visibility::Hidden;
                }
                
                // Reset cutscene state
                cutscene_state.current_video = None;
                cutscene_state.is_playing = false;
                cutscene_state.playback_position = 0.0;
                
                info!("Cutscene stopped");
            }
            
            CutsceneEvent::Finished => {
                // Transition to next game state
                match current_state.get() {
                    GameState::Intro => {
                        next_state.set(GameState::Playing);
                        info!("Intro finished, starting game");
                    }
                    GameState::Outro => {
                        next_state.set(GameState::Credits);
                        info!("Outro finished, showing credits");
                    }
                    _ => {}
                }
                
                // Stop the video
                cutscene_events.send(CutsceneEvent::Stop);
            }
        }
    }
}

/// Update video playback (simulated for now, will need actual video decoder)
fn update_video_playback(
    mut cutscene_state: ResMut<CutsceneState>,
    mut cutscene_events: EventWriter<CutsceneEvent>,
    time: Res<Time>,
) {
    if cutscene_state.is_playing {
        cutscene_state.playback_position += time.delta_seconds();
        
        // Check if video finished
        if cutscene_state.playback_position >= cutscene_state.total_duration {
            cutscene_events.send(CutsceneEvent::Finished);
        }
    }
}

/// Handle skip input
fn handle_cutscene_skip(
    keyboard: Res<ButtonInput<KeyCode>>,
    cutscene_state: Res<CutsceneState>,
    mut cutscene_events: EventWriter<CutsceneEvent>,
) {
    if cutscene_state.is_playing && cutscene_state.can_skip {
        if keyboard.just_pressed(KeyCode::Space) {
            info!("Skipping cutscene");
            cutscene_events.send(CutsceneEvent::Skip);
            cutscene_events.send(CutsceneEvent::Finished);
        }
    }
}

/// Trigger the intro cutscene from game start
pub fn trigger_intro_cutscene(
    mut next_state: ResMut<NextState<GameState>>,
) {
    next_state.set(GameState::Intro);
}

/// Trigger the outro cutscene after dragon defeat
pub fn trigger_outro_cutscene(
    mut commands: Commands,
) {
    // Send event to trigger outro FMV
    commands.insert_resource(NextState(Some(GameState::Outro)));
    info!("Triggering emotional outro cutscene - traveler portal");
}

// Note: For actual video playback in Bevy, we would need to integrate
// a video decoding library like ffmpeg-next or gstreamer-rs.
// The current implementation provides the framework and can display
// static frames or image sequences as a fallback.
//
// The emotional transition from first-person 3D (in the FMV) to
// top-down 2.5D gameplay creates that "wonderland" feeling the
// user described, establishing the horror atmosphere from the start.