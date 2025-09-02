use bevy::prelude::*;
use crate::components::character::*;
use crate::resources::GameState;

#[derive(Resource, Debug)]
pub struct CharacterCreator {
    pub active: bool,
    pub current_character: CharacterData,
    pub preview_entity: Option<Entity>,
}

impl Default for CharacterCreator {
    fn default() -> Self {
        Self {
            active: false,
            current_character: CharacterData::default(),
            preview_entity: None,
        }
    }
}

pub fn setup_character_creator(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut creator: ResMut<CharacterCreator>,
) {
    creator.active = true;
    
    // Create character creator UI
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Row,
                ..default()
            },
            background_color: Color::rgba(0.0, 0.0, 0.0, 0.8).into(),
            ..default()
        },
        Name::new("CharacterCreatorUI"),
    )).with_children(|parent| {
        // Left panel - Character preview
        parent.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(60.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        }).with_children(|preview_panel| {
            preview_panel.spawn(TextBundle::from_section(
                "Character Preview",
                TextStyle {
                    font: asset_server.load("fonts/courier_new.ttf"),
                    font_size: 24.0,
                    color: Color::WHITE,
                },
            ));
        });
        
        // Right panel - Customization options
        parent.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(40.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            background_color: Color::rgba(0.1, 0.1, 0.1, 0.9).into(),
            ..default()
        }).with_children(|options_panel| {
            // Title
            options_panel.spawn(TextBundle::from_section(
                "Create Your Character",
                TextStyle {
                    font: asset_server.load("fonts/courier_new.ttf"),
                    font_size: 32.0,
                    color: Color::rgb(0.8, 0.2, 0.2),
                },
            ));
            
            // Name input
            spawn_name_section(options_panel, &asset_server);
            
            // Gender selection
            spawn_gender_section(options_panel, &asset_server);
            
            // Appearance options
            spawn_appearance_section(options_panel, &asset_server);
            
            // Stats allocation
            spawn_stats_section(options_panel, &asset_server);
            
            // Create character button
            options_panel.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(50.0),
                        margin: UiRect::top(Val::Px(20.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::rgb(0.2, 0.6, 0.2).into(),
                    ..default()
                },
                CreateCharacterButton,
            )).with_children(|button| {
                button.spawn(TextBundle::from_section(
                    "Begin Adventure",
                    TextStyle {
                        font: asset_server.load("fonts/courier_new.ttf"),
                        font_size: 20.0,
                        color: Color::WHITE,
                    },
                ));
            });
        });
    });
}

fn spawn_name_section(parent: &mut ChildBuilder, asset_server: &AssetServer) {
    parent.spawn(TextBundle::from_section(
        "Name:",
        TextStyle {
            font: asset_server.load("fonts/courier_new.ttf"),
            font_size: 18.0,
            color: Color::WHITE,
        },
    ));
    
    // TODO: Add text input component when available
    parent.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(30.0),
                margin: UiRect::bottom(Val::Px(10.0)),
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            background_color: Color::rgb(0.2, 0.2, 0.2).into(),
            border_color: Color::WHITE.into(),
            ..default()
        },
        NameInput,
    ));
}

fn spawn_gender_section(parent: &mut ChildBuilder, asset_server: &AssetServer) {
    parent.spawn(TextBundle::from_section(
        "Gender:",
        TextStyle {
            font: asset_server.load("fonts/courier_new.ttf"),
            font_size: 18.0,
            color: Color::WHITE,
        },
    ));
    
    parent.spawn(NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Row,
            margin: UiRect::bottom(Val::Px(10.0)),
            ..default()
        },
        ..default()
    }).with_children(|gender_row| {
        // Male button
        gender_row.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(80.0),
                    height: Val::Px(30.0),
                    margin: UiRect::right(Val::Px(10.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::rgb(0.3, 0.3, 0.6).into(),
                ..default()
            },
            GenderButton(Gender::Male),
        )).with_children(|button| {
            button.spawn(TextBundle::from_section(
                "Male",
                TextStyle {
                    font: asset_server.load("fonts/courier_new.ttf"),
                    font_size: 16.0,
                    color: Color::WHITE,
                },
            ));
        });
        
        // Female button
        gender_row.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(80.0),
                    height: Val::Px(30.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::rgb(0.6, 0.3, 0.6).into(),
                ..default()
            },
            GenderButton(Gender::Female),
        )).with_children(|button| {
            button.spawn(TextBundle::from_section(
                "Female",
                TextStyle {
                    font: asset_server.load("fonts/courier_new.ttf"),
                    font_size: 16.0,
                    color: Color::WHITE,
                },
            ));
        });
    });
}

fn spawn_appearance_section(parent: &mut ChildBuilder, asset_server: &AssetServer) {
    parent.spawn(TextBundle::from_section(
        "Appearance:",
        TextStyle {
            font: asset_server.load("fonts/courier_new.ttf"),
            font_size: 18.0,
            color: Color::WHITE,
        },
    ));
    
    // Hair style
    spawn_slider_option(parent, asset_server, "Hair Style", SliderType::HairStyle);
    
    // Skin tone
    spawn_slider_option(parent, asset_server, "Skin Tone", SliderType::SkinTone);
    
    // Height
    spawn_slider_option(parent, asset_server, "Height", SliderType::Height);
    
    // Weight
    spawn_slider_option(parent, asset_server, "Build", SliderType::Weight);
}

fn spawn_stats_section(parent: &mut ChildBuilder, asset_server: &AssetServer) {
    parent.spawn(TextBundle::from_section(
        "Attributes (20 points to distribute):",
        TextStyle {
            font: asset_server.load("fonts/courier_new.ttf"),
            font_size: 18.0,
            color: Color::WHITE,
        },
    ));
    
    let stats = ["Strength", "Dexterity", "Constitution", "Intelligence", "Wisdom", "Charisma"];
    for (i, stat) in stats.iter().enumerate() {
        spawn_stat_adjuster(parent, asset_server, stat, i);
    }
}

fn spawn_slider_option(parent: &mut ChildBuilder, asset_server: &AssetServer, label: &str, slider_type: SliderType) {
    parent.spawn(NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Row,
            margin: UiRect::bottom(Val::Px(5.0)),
            align_items: AlignItems::Center,
            ..default()
        },
        ..default()
    }).with_children(|row| {
        row.spawn(TextBundle::from_section(
            format!("{}:", label),
            TextStyle {
                font: asset_server.load("fonts/courier_new.ttf"),
                font_size: 14.0,
                color: Color::WHITE,
            },
        ));
        
        // TODO: Implement actual slider component
        row.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Px(100.0),
                    height: Val::Px(20.0),
                    margin: UiRect::left(Val::Px(10.0)),
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                background_color: Color::rgb(0.2, 0.2, 0.2).into(),
                border_color: Color::WHITE.into(),
                ..default()
            },
            Slider(slider_type),
        ));
    });
}

fn spawn_stat_adjuster(parent: &mut ChildBuilder, asset_server: &AssetServer, stat_name: &str, index: usize) {
    parent.spawn(NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Row,
            margin: UiRect::bottom(Val::Px(5.0)),
            align_items: AlignItems::Center,
            ..default()
        },
        ..default()
    }).with_children(|row| {
        // Stat name
        row.spawn(TextBundle::from_section(
            format!("{}:", stat_name),
            TextStyle {
                font: asset_server.load("fonts/courier_new.ttf"),
                font_size: 14.0,
                color: Color::WHITE,
            },
        ));
        
        // Decrease button
        row.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(25.0),
                    height: Val::Px(25.0),
                    margin: UiRect::left(Val::Px(10.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::rgb(0.6, 0.2, 0.2).into(),
                ..default()
            },
            StatButton { index, increase: false },
        )).with_children(|button| {
            button.spawn(TextBundle::from_section(
                "-",
                TextStyle {
                    font: asset_server.load("fonts/courier_new.ttf"),
                    font_size: 16.0,
                    color: Color::WHITE,
                },
            ));
        });
        
        // Current value
        row.spawn((
            TextBundle::from_section(
                "10",
                TextStyle {
                    font: asset_server.load("fonts/courier_new.ttf"),
                    font_size: 14.0,
                    color: Color::WHITE,
                },
            ),
            StatDisplay(index),
        ));
        
        // Increase button
        row.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(25.0),
                    height: Val::Px(25.0),
                    margin: UiRect::left(Val::Px(5.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::rgb(0.2, 0.6, 0.2).into(),
                ..default()
            },
            StatButton { index, increase: true },
        )).with_children(|button| {
            button.spawn(TextBundle::from_section(
                "+",
                TextStyle {
                    font: asset_server.load("fonts/courier_new.ttf"),
                    font_size: 16.0,
                    color: Color::WHITE,
                },
            ));
        });
    });
}

// UI Components
#[derive(Component)]
pub struct NameInput;

#[derive(Component)]
pub struct GenderButton(pub Gender);

#[derive(Component)]
pub struct Slider(pub SliderType);

#[derive(Debug, Clone)]
pub enum SliderType {
    HairStyle,
    SkinTone,
    Height,
    Weight,
}

#[derive(Component)]
pub struct StatButton {
    pub index: usize,
    pub increase: bool,
}

#[derive(Component)]
pub struct StatDisplay(pub usize);

#[derive(Component)]
pub struct CreateCharacterButton;

// Systems
pub fn handle_character_creator_input(
    mut creator: ResMut<CharacterCreator>,
    mut button_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>,
    gender_button_query: Query<&GenderButton>,
    stat_button_query: Query<&StatButton>,
    create_button_query: Query<&CreateCharacterButton>,
    mut stat_display_query: Query<(&mut Text, &StatDisplay)>,
    mut game_state: ResMut<GameState>,
) {
    for (interaction, mut color) in &mut button_query {
        match *interaction {
            Interaction::Pressed => {
                *color = Color::rgb(0.35, 0.75, 0.35).into();
                
                // Handle different button types
                // TODO: Implement button handling logic
            }
            Interaction::Hovered => {
                *color = Color::rgb(0.25, 0.25, 0.25).into();
            }
            Interaction::None => {
                *color = Color::rgb(0.15, 0.15, 0.15).into();
            }
        }
    }
}

pub fn update_character_preview(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    creator: Res<CharacterCreator>,
    preview_query: Query<Entity, With<CharacterModel>>,
) {
    if creator.is_changed() {
        // Remove existing preview
        if let Some(entity) = creator.preview_entity {
            if let Ok(preview_entity) = preview_query.get(entity) {
                commands.entity(preview_entity).despawn_recursive();
            }
        }
        
        // Spawn new preview with current settings
        let model_path = creator.current_character.appearance.get_player_model_path(&creator.current_character.gender);
        let scene_handle = asset_server.load(&model_path);
        
        let preview_entity = commands.spawn((
            SceneBundle {
                scene: scene_handle.clone(),
                transform: Transform::from_xyz(0.0, 0.0, 5.0)
                    .with_scale(Vec3::splat(2.0)),
                ..default()
            },
            CharacterModel {
                base_model: scene_handle,
                current_animation: None,
                scale_modifier: Vec3::new(
                    creator.current_character.appearance.weight,
                    creator.current_character.appearance.height,
                    creator.current_character.appearance.weight,
                ),
            },
        )).id();
        
        // Update the preview entity reference
        // Note: This requires mutable access to creator, which might need restructuring
    }
}

pub fn finish_character_creation(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    creator: Res<CharacterCreator>,
    ui_query: Query<Entity, With<Node>>,
) {
    if !creator.active {
        return;
    }
    
    // Remove character creator UI
    for entity in &ui_query {
        commands.entity(entity).despawn_recursive();
    }
    
    // Spawn the actual player character
    let model_path = creator.current_character.appearance.get_player_model_path(&creator.current_character.gender);
    let scene_handle = asset_server.load(&model_path);
    
    commands.spawn((
        SceneBundle {
            scene: scene_handle.clone(),
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            ..default()
        },
        Player {
            character_data: creator.current_character.clone(),
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
                creator.current_character.appearance.weight,
                creator.current_character.appearance.height,
                creator.current_character.appearance.weight,
            ),
        },
        Name::new("Player"),
    ));
}