use bevy::prelude::*;
use bevy_cobweb_ui::prelude::*;
use crate::world::components::character::*;
use crate::world::resources::GameState;
use crate::game::GameStateEnum;

#[derive(Resource, Debug)]
pub struct UIManager {
    pub current_screen: UIScreen,
    pub splash_progress: f32,
    pub splash_complete: bool,
    pub character_creator: CharacterCreator,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UIScreen {
    Splash,
    MainMenu,
    CharacterCreation,
    InGame,
    Settings,
}

impl Default for UIManager {
    fn default() -> Self {
        Self {
            current_screen: UIScreen::Splash,
            splash_progress: 0.0,
            splash_complete: false,
            character_creator: CharacterCreator::default(),
        }
    }
}

// Component markers for UI elements
#[derive(Component)]
pub struct SplashScreen;

#[derive(Component)]
pub struct MainMenuScreen;

#[derive(Component)]
pub struct CharacterCreationScreen;

#[derive(Component)]
pub struct ProgressBar;

#[derive(Component)]
pub struct LoadingText;

#[derive(Component)]
pub struct PressKeyText;

#[derive(Component)]
pub struct NewGameButton;

#[derive(Component)]
pub struct ContinueButton;

#[derive(Component)]
pub struct SettingsButton;

#[derive(Component)]
pub struct ExitButton;

// Character Creator UI Components
#[derive(Component)]
pub struct CharacterPreview;

#[derive(Component)]
pub struct GenderSelector(pub Gender);

#[derive(Component)]
pub struct AppearanceSlider {
    pub slider_type: AppearanceSliderType,
    pub current_value: usize,
}

#[derive(Debug, Clone)]
pub enum AppearanceSliderType {
    HairStyle,
    SkinTone,
    Height,
    Build,
}

#[derive(Component)]
pub struct BeginAdventureButton;

#[derive(Component)]
pub struct BackToMenuButton;

pub fn setup_splash_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Load the cobweb UI file
    commands.spawn((
        CobwebUIBundle::from_file("ui/splash_screen.cob"),
        SplashScreen,
        Name::new("SplashScreenUI"),
    ));
}

pub fn setup_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        CobwebUIBundle::from_file("ui/main_menu.cob"),
        MainMenuScreen,
        Name::new("MainMenuUI"),
    ));
}

pub fn setup_character_creator_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        CobwebUIBundle::from_file("ui/character_creator.cob"),
        CharacterCreationScreen,
        Name::new("CharacterCreatorUI"),
    ));
}

pub fn update_splash_screen(
    time: Res<Time>,
    mut ui_manager: ResMut<UIManager>,
    mut query: Query<&mut Text, With<LoadingText>>,
    mut progress_query: Query<&mut Style, (With<ProgressBar>, Without<LoadingText>)>,
    mut press_key_query: Query<&mut Text, (With<PressKeyText>, Without<LoadingText>)>,
    mut game_state: ResMut<NextState<GameStateEnum>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if ui_manager.current_screen != UIScreen::Splash {
        return;
    }

    // Update splash progress
    ui_manager.splash_progress += time.delta_seconds() * 0.5; // 2 second loading
    ui_manager.splash_progress = ui_manager.splash_progress.min(1.0);

    // Update progress bar
    if let Ok(mut style) = progress_query.get_single_mut() {
        style.width = Val::Percent(ui_manager.splash_progress * 100.0);
    }

    // Update loading text
    if let Ok(mut text) = query.get_single_mut() {
        let loading_messages = [
            "Awakening ancient horrors...",
            "Corrupting the world...",
            "Summoning dragons...",
            "Preparing your doom...",
        ];
        let index = ((ui_manager.splash_progress * 4.0) as usize).min(3);
        text.sections[0].value = loading_messages[index].to_string();
    }

    // Show "press any key" when loading complete
    if ui_manager.splash_progress >= 1.0 && !ui_manager.splash_complete {
        ui_manager.splash_complete = true;
        if let Ok(mut text) = press_key_query.get_single_mut() {
            text.sections[0].style.color = Color::srgba(0.9, 0.9, 0.9, 1.0);
        }
    }

    // Handle input to proceed
    if ui_manager.splash_complete && keyboard_input.pressed(KeyCode::Space) {
        ui_manager.current_screen = UIScreen::MainMenu;
        game_state.set(GameStateEnum::MainMenu);
    }
}

pub fn handle_main_menu_input(
    mut ui_manager: ResMut<UIManager>,
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>,
    new_game_query: Query<&NewGameButton>,
    continue_query: Query<&ContinueButton>,
    settings_query: Query<&SettingsButton>,
    exit_query: Query<&ExitButton>,
    mut game_state: ResMut<NextState<GameStateEnum>>,
) {
    if ui_manager.current_screen != UIScreen::MainMenu {
        return;
    }

    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                // Handle button-specific logic
                if new_game_query.iter().count() > 0 {
                    ui_manager.current_screen = UIScreen::CharacterCreation;
                    game_state.set(GameStateEnum::CharacterCreation);
                }
                
                *color = Color::srgba(0.35, 0.75, 0.35, 1.0).into();
            }
            Interaction::Hovered => {
                *color = Color::srgba(0.4, 0.15, 0.15, 1.0).into();
            }
            Interaction::None => {
                *color = Color::srgba(0.2, 0.05, 0.05, 0.9).into();
            }
        }
    }
}

pub fn handle_character_creator_input(
    mut ui_manager: ResMut<UIManager>,
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>,
    gender_query: Query<&GenderSelector>,
    appearance_query: Query<&AppearanceSlider>,
    begin_query: Query<&BeginAdventureButton>,
    back_query: Query<&BackToMenuButton>,
    mut game_state: ResMut<NextState<GameStateEnum>>,
) {
    if ui_manager.current_screen != UIScreen::CharacterCreation {
        return;
    }

    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                // Handle different button types
                if begin_query.iter().count() > 0 {
                    // Create player character and start game
                    ui_manager.current_screen = UIScreen::InGame;
                    game_state.set(GameStateEnum::Playing);
                } else if back_query.iter().count() > 0 {
                    ui_manager.current_screen = UIScreen::MainMenu;
                    game_state.set(GameStateEnum::MainMenu);
                }
                
                *color = Color::srgba(0.35, 0.75, 0.35, 1.0).into();
            }
            Interaction::Hovered => {
                *color = Color::srgba(0.5, 0.2, 0.2, 1.0).into();
            }
            Interaction::None => {
                *color = Color::srgba(0.2, 0.05, 0.05, 0.9).into();
            }
        }
    }
}

pub fn cleanup_ui_screen<T: Component>(
    mut commands: Commands,
    query: Query<Entity, With<T>>,
) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn spawn_player_character(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    ui_manager: Res<UIManager>,
) {
    let character_data = &ui_manager.character_creator.current_character;
    
    // Determine model path based on gender
    let model_path = match character_data.gender {
        Gender::Male => "models/characters/player/male_base.glb",
        Gender::Female => "models/characters/player/female_base.glb",
    };
    
    let scene_handle = asset_server.load(model_path);
    
    // Apply appearance customization to transform
    let mut transform = Transform::from_xyz(0.0, 1.0, 0.0);
    character_data.appearance.apply_customization(&mut transform);
    
    commands.spawn((
        SceneBundle {
            scene: scene_handle.clone(),
            transform,
            ..default()
        },
        Player {
            character_data: character_data.clone(),
            position: Vec3::new(0.0, 1.0, 0.0),
            facing_direction: Vec3::Z,
            movement_speed: 5.0,
            health: 100.0,
            max_health: 100.0,
            level: 1,
            experience: 0,
        },
        CharacterModel {
            base_model: scene_handle,
            current_animation: None,
            scale_modifier: Vec3::new(
                character_data.appearance.weight,
                character_data.appearance.height,
                character_data.appearance.weight,
            ),
        },
        Name::new("Player"),
    ));
}

// Audio system for horror atmosphere
pub fn update_horror_audio(
    ui_manager: Res<UIManager>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Play appropriate background music based on current screen
    match ui_manager.current_screen {
        UIScreen::Splash => {
            // Ominous intro music
        }
        UIScreen::MainMenu => {
            // Dark ambient menu music
        }
        UIScreen::CharacterCreation => {
            // Slightly hopeful but foreboding music
        }
        UIScreen::InGame => {
            // Dynamic horror music based on dread level
        }
        UIScreen::Settings => {
            // Muted version of menu music
        }
    }
}

pub fn update_ui_animations(
    time: Res<Time>,
    mut text_query: Query<&mut Text>,
    mut style_query: Query<&mut Style>,
) {
    // Add subtle UI animations for horror effect
    let pulse = (time.elapsed_seconds() * 2.0).sin() * 0.1 + 0.9;
    
    // Apply pulsing effect to certain UI elements
    for mut text in &mut text_query {
        if text.sections[0].style.color == Color::srgba(0.8, 0.1, 0.1, 1.0) {
            text.sections[0].style.color = Color::srgba(0.8 * pulse, 0.1, 0.1, 1.0);
        }
    }
}