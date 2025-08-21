//! UI system for Dragon's Labyrinth

use bevy::prelude::*;
use bevy_cobweb::prelude::*;
use bevy_cobweb_ui::prelude::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            CobwebPlugin,
            CobwebUiPlugin,
        ))
        .init_resource::<UIState>()
        .add_systems(Startup, setup_ui)
        .add_systems(Update, (
            update_ui,
            handle_ui_events,
        ));
    }
}

#[derive(Resource, Default)]
pub struct UIState {
    pub menu_open: bool,
    pub inventory_open: bool,
    pub dialogue_active: bool,
    pub current_dialogue: Option<String>,
}

fn setup_ui(mut commands: Commands) {
    // UI setup will go here
}

fn update_ui(
    ui_state: Res<UIState>,
    mut commands: Commands,
) {
    // UI update logic
}

fn handle_ui_events(
    mut ui_state: ResMut<UIState>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        ui_state.menu_open = !ui_state.menu_open;
    }
    if keyboard.just_pressed(KeyCode::KeyI) {
        ui_state.inventory_open = !ui_state.inventory_open;
    }
}