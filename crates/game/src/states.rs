use bevy::prelude::*;

/// Main game states following the handoff document
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    MainMenu,
    Loading,
    InGame,
    Paused,
    GameOver,
}

/// Plugin to handle game state transitions with fade effects
pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<GameState>()
            .add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
            .add_systems(OnExit(GameState::MainMenu), cleanup_main_menu)
            .add_systems(OnEnter(GameState::Loading), start_loading)
            .add_systems(Update, update_loading.run_if(in_state(GameState::Loading)))
            .add_systems(OnEnter(GameState::InGame), setup_game)
            .add_systems(OnExit(GameState::InGame), cleanup_game)
            .add_systems(OnEnter(GameState::Paused), setup_pause_menu)
            .add_systems(OnExit(GameState::Paused), cleanup_pause_menu)
            .add_systems(OnEnter(GameState::GameOver), setup_game_over)
            .add_systems(Update, handle_pause_input.run_if(in_state(GameState::InGame)))
            .add_systems(Update, handle_pause_menu_input.run_if(in_state(GameState::Paused)))
            .add_systems(Update, handle_quick_save_load);
    }
}

#[derive(Component)]
struct MainMenuUI;

#[derive(Component)]
struct PauseMenuUI;

#[derive(Component)]
struct GameOverUI;

#[derive(Component)]
struct LoadingScreen;

#[derive(Resource)]
struct LoadingProgress {
    tasks_completed: u32,
    total_tasks: u32,
}

fn setup_main_menu(mut commands: Commands) {
    // Simple main menu with buttons
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: BackgroundColor(Color::srgb(0.1, 0.1, 0.15)),
            ..default()
        },
        MainMenuUI,
    )).with_children(|parent| {
        // Title
        parent.spawn(TextBundle::from_section(
            "DRAGON'S LABYRINTH",
            TextStyle {
                font_size: 72.0,
                color: Color::srgb(0.8, 0.2, 0.2),
                ..default()
            },
        ));
        
        // Start button
        parent.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(200.0),
                    height: Val::Px(65.0),
                    margin: UiRect::all(Val::Px(20.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor(Color::srgb(0.2, 0.2, 0.3)),
                ..default()
            },
            StartButton,
        )).with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "START GAME",
                TextStyle {
                    font_size: 30.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));
        });
        
        // Load button
        parent.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(200.0),
                    height: Val::Px(65.0),
                    margin: UiRect::all(Val::Px(20.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor(Color::srgb(0.2, 0.2, 0.3)),
                ..default()
            },
            LoadButton,
        )).with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "LOAD GAME",
                TextStyle {
                    font_size: 30.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));
        });
        
        // Quit button
        parent.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(200.0),
                    height: Val::Px(65.0),
                    margin: UiRect::all(Val::Px(20.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor(Color::srgb(0.2, 0.2, 0.3)),
                ..default()
            },
            QuitButton,
        )).with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "QUIT",
                TextStyle {
                    font_size: 30.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));
        });
    });
}

fn cleanup_main_menu(
    mut commands: Commands,
    query: Query<Entity, With<MainMenuUI>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn start_loading(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // Create loading screen
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: BackgroundColor(Color::BLACK),
            ..default()
        },
        LoadingScreen,
    )).with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            "Loading...",
            TextStyle {
                font_size: 48.0,
                color: Color::WHITE,
                ..default()
            },
        ));
    });
    
    // Initialize loading progress
    commands.insert_resource(LoadingProgress {
        tasks_completed: 0,
        total_tasks: 3, // Simplified loading
    });
}

fn update_loading(
    mut commands: Commands,
    mut progress: ResMut<LoadingProgress>,
    mut next_state: ResMut<NextState<GameState>>,
    query: Query<Entity, With<LoadingScreen>>,
) {
    // Simulate loading progress
    progress.tasks_completed += 1;
    
    if progress.tasks_completed >= progress.total_tasks {
        // Cleanup loading screen
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
        
        // Transition to game
        next_state.set(GameState::InGame);
        commands.remove_resource::<LoadingProgress>();
    }
}

fn setup_game(
    mut commands: Commands,
) {
    info!("Game started!");
    // Game setup happens in main.rs setup systems
}

fn cleanup_game(
    mut commands: Commands,
) {
    info!("Cleaning up game state");
}

fn setup_pause_menu(
    mut commands: Commands,
) {
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
            ..default()
        },
        PauseMenuUI,
    )).with_children(|parent| {
        parent.spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            background_color: BackgroundColor(Color::srgb(0.15, 0.15, 0.2)),
            ..default()
        }).with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "PAUSED",
                TextStyle {
                    font_size: 48.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));
            
            parent.spawn(TextBundle::from_section(
                "Press ESC to resume\nPress Q to quit to menu",
                TextStyle {
                    font_size: 24.0,
                    color: Color::srgb(0.7, 0.7, 0.7),
                    ..default()
                },
            ));
        });
    });
}

fn cleanup_pause_menu(
    mut commands: Commands,
    query: Query<Entity, With<PauseMenuUI>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn setup_game_over(
    mut commands: Commands,
) {
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: BackgroundColor(Color::srgb(0.1, 0.0, 0.0)),
            ..default()
        },
        GameOverUI,
    )).with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            "YOU DIED",
            TextStyle {
                font_size: 96.0,
                color: Color::srgb(0.8, 0.0, 0.0),
                ..default()
            },
        ));
    });
}

fn handle_pause_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Paused);
    }
}

fn handle_pause_menu_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::InGame);
    } else if keyboard.just_pressed(KeyCode::KeyQ) {
        next_state.set(GameState::MainMenu);
    }
}

fn handle_quick_save_load(
    keyboard: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<GameState>>,
) {
    if current_state.get() != &GameState::InGame {
        return;
    }
    
    if keyboard.just_pressed(KeyCode::F5) {
        info!("Quick save triggered!");
        // TODO: Integrate with save system
    }
    
    if keyboard.just_pressed(KeyCode::F9) {
        info!("Quick load triggered!");
        // TODO: Integrate with save system
    }
}

// Button components for menu interaction
#[derive(Component)]
struct StartButton;

#[derive(Component)]
struct LoadButton;

#[derive(Component)]
struct QuitButton;

// Button interaction system
pub fn menu_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&StartButton>, Option<&LoadButton>, Option<&QuitButton>),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    for (interaction, mut color, start, load, quit) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if start.is_some() {
                    next_state.set(GameState::Loading);
                } else if load.is_some() {
                    // TODO: Load saved game
                    info!("Load game not yet implemented");
                } else if quit.is_some() {
                    app_exit_events.send(AppExit::Success);
                }
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::srgb(0.3, 0.3, 0.4));
            }
            Interaction::None => {
                *color = BackgroundColor(Color::srgb(0.2, 0.2, 0.3));
            }
        }
    }
}