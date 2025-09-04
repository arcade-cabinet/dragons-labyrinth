use bevy::prelude::*;
use bevy_cobweb_ui::prelude::*;
use bevy_cobweb::prelude::*;
use crate::world::state::GameState;
use crate::game::GameStateEnum;

/// Minimal UI system that only handles logic - UI structure is in .cob files
#[derive(Resource, Debug, Default)]
pub struct UIManager {
    pub current_screen: UIScreen,
    pub splash_progress: f32,
    pub splash_complete: bool,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum UIScreen {
    #[default]
    Splash,
    MainMenu,
    CharacterCreation,
    InGame,
    Settings,
}

/// Load cobweb UI file and handle state transitions only
pub fn setup_cobweb_ui(mut commands: Commands) {
    // Load the main UI file - all UI structure is defined there
    // TODO: Implement proper .cob file loading when available
    commands.spawn((
        Name::new("CobwebUI"),
    ));
}

/// Handle UI state transitions - UI layout handled by .cob files
pub fn handle_ui_transitions(
    mut ui_manager: ResMut<UIManager>,
    mut game_state: ResMut<NextState<GameStateEnum>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    match ui_manager.current_screen {
        UIScreen::Splash => {
            ui_manager.splash_progress += time.delta_secs() * 0.5;
            if ui_manager.splash_progress >= 1.0 {
                ui_manager.splash_complete = true;
            }
            
            if ui_manager.splash_complete && keyboard_input.just_pressed(KeyCode::Space) {
                ui_manager.current_screen = UIScreen::MainMenu;
                game_state.set(GameStateEnum::MainMenu);
            }
        }
        UIScreen::MainMenu => {
            // State transitions handled by .cob file interactions
        }
        UIScreen::CharacterCreation => {
            // Character creation logic handled by .cob file
        }
        UIScreen::InGame => {
            // In-game UI state handled by .cob file
        }
        UIScreen::Settings => {
            // Settings UI handled by .cob file
        }
    }
}

/// Simple character creator input handler - UI defined in .cob file
pub fn handle_character_creator_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<NextState<GameStateEnum>>,
) {
    if keyboard_input.just_pressed(KeyCode::Enter) {
        game_state.set(GameStateEnum::Playing);
    }
    
    if keyboard_input.just_pressed(KeyCode::Escape) {
        game_state.set(GameStateEnum::MainMenu);
    }
}
