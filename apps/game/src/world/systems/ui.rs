use bevy::prelude::*;
use crate::world::components::{Player, Companion};
use crate::world::resources::{DreadLevel, GameState};
use crate::game::{GameStateEnum};

#[derive(Component)]
pub struct StartGameButton;

pub fn ui_update_system(
    mut contexts: Query<&mut Node>,
    player_query: Query<&Player>,
    companion_query: Query<&Companion>,
    dread_level: Res<DreadLevel>,
    // TODO: Add cobweb UI integration
) {
    if let Ok(player) = player_query.get_single() {
        // Update health/sanity displays
        // This would be implemented with proper UI widgets
        if dread_level.phase_changed_this_frame {
            info!("UI: Dread phase changed to {:?}", dread_level.phase);
        }
    }
}

pub fn handle_main_menu_buttons(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<StartGameButton>),
    >,
    mut game_state: ResMut<NextState<GameStateEnum>>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(Color::srgb(0.1, 0.1, 0.1));
                game_state.set(GameStateEnum::Playing);
                info!("Starting game...");
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::srgb(0.4, 0.4, 0.4));
            }
            Interaction::None => {
                *color = BackgroundColor(Color::srgb(0.3, 0.3, 0.3));
            }
        }
    }
}

pub fn setup_hud(mut commands: Commands) {
    // Health bar
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(20.0),
                top: Val::Px(20.0),
                width: Val::Px(200.0),
                height: Val::Px(20.0),
                ..default()
            },
            background_color: BackgroundColor(Color::srgba(1.0, 0.0, 0.0, 0.8)),
            ..default()
        },
        Name::new("HealthBar"),
    ));
    
    // Sanity bar
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(20.0),
                top: Val::Px(50.0),
                width: Val::Px(200.0),
                height: Val::Px(20.0),
                ..default()
            },
            background_color: BackgroundColor(Color::srgba(0.0, 0.0, 1.0, 0.8)),
            ..default()
        },
        Name::new("SanityBar"),
    ));
    
    // Dread level indicator
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                right: Val::Px(20.0),
                top: Val::Px(20.0),
                width: Val::Px(150.0),
                height: Val::Px(50.0),
                ..default()
            },
            background_color: BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
            ..default()
        },
        Name::new("DreadIndicator"),
    ));
}

pub fn update_hud_system(
    player_query: Query<&Player>,
    dread_level: Res<DreadLevel>,
    mut health_bar_query: Query<&mut Style, (With<Node>, Without<Text>)>,
    mut text_query: Query<&mut Text>,
) {
    if let Ok(player) = player_query.get_single() {
        // Update health bar width based on current health
        let health_percentage = player.health / player.max_health;
        
        // Update sanity bar width based on current sanity
        let sanity_percentage = player.sanity / player.max_sanity;
        
        // This is a simplified implementation
        // In practice, you'd want to identify specific UI elements by component markers
    }
}
