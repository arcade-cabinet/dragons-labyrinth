use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use avian3d::prelude::*;
use bevy_cobweb::prelude::*;
use bevy_cobweb_ui::prelude::*;
use bevy_yarnspinner::prelude::*;
use bevy_yoleck::prelude::*;

use crate::systems::*;
use crate::components::*;
use crate::resources::*;

pub struct HorrorRpgPlugin;

impl Plugin for HorrorRpgPlugin {
    fn build(&self, app: &mut App) {
        // Core Bevy plugins
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Horror RPG".into(),
                canvas: Some("#game-canvas".into()),
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }));

        // Third-party plugins
        app.add_plugins((
            TilemapPlugin,
            PhysicsPlugins::default(),
            CobwebPlugin::default(),
            CobwebUiPlugin,
            YarnSpinnerPlugin,
            YoleckPlugin,
        ));

        // Game resources
        app.init_resource::<WorldState>()
            .init_resource::<GameState>()
            .init_resource::<DreadLevel>()
            .init_resource::<AssetHandles>();

        // Game components registration
        app.register_component_hooks::<Tile>()
            .register_component_hooks::<Player>()
            .register_component_hooks::<Companion>()
            .register_component_hooks::<DreadSource>();

        // Game systems
        app.add_systems(Startup, (
            setup_camera,
            setup_world,
            setup_ui,
            load_assets,
        ))
        .add_systems(Update, (
            player_movement_system,
            hex_world_generation_system,
            companion_psychology_system,
            dread_progression_system,
            asset_loading_system,
            ui_update_system,
        ).run_if(in_state(GameStateEnum::Playing)))
        .add_systems(OnEnter(GameStateEnum::MainMenu), setup_main_menu)
        .add_systems(OnExit(GameStateEnum::MainMenu), cleanup_main_menu);

        // Game states
        app.init_state::<GameStateEnum>();

        // Development tools
        #[cfg(debug_assertions)]
        {
            app.add_plugins(bevy::dev_tools::ui_debug_overlay::UiDebugPlugin);
        }
    }
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum GameStateEnum {
    #[default]
    Loading,
    MainMenu,
    Playing,
    Dialogue,
    Boss,
    Labyrinth,
}

fn setup_camera(mut commands: Commands) {
    // Overhead camera for hex world
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 50.0, 0.0)
                .looking_at(Vec3::ZERO, Vec3::Y),
            projection: Projection::Orthographic(OrthographicProjection {
                scale: 0.5,
                ..default()
            }),
            ..default()
        },
        Name::new("OverheadCamera"),
    ));
}

fn setup_world(
    mut commands: Commands,
    mut world_state: ResMut<WorldState>,
) {
    // Initialize world generation seed
    world_state.seed = rand::random();
    
    // Create initial tile map
    let tilemap_entity = commands.spawn((
        Name::new("HexTilemap"),
        TilemapBundle {
            grid_size: TilemapGridSize { x: 64.0, y: 64.0 },
            map_type: TilemapType::Hexagon(HexCoordSystem::Row),
            size: TilemapSize { x: 128, y: 128 },
            storage: TilemapStorage::new(16, TileEntity::default()),
            texture: TilemapTexture::Single(Handle::default()), // Will be set when assets load
            tile_size: TilemapTileSize { x: 64.0, y: 64.0 },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..default()
        },
    )).id();

    world_state.tilemap_entity = Some(tilemap_entity);
}

fn setup_ui(mut commands: Commands) {
    // Root UI container
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        },
        Name::new("UIRoot"),
    ));
}

fn setup_main_menu(mut commands: Commands) {
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            background_color: BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.8)),
            ..default()
        },
        MainMenuMarker,
    )).with_children(|parent| {
        parent.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(200.0),
                    height: Val::Px(50.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
                ..default()
            },
            StartGameButton,
        )).with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Start Journey",
                TextStyle {
                    font_size: 24.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));
        });
    });
}

fn cleanup_main_menu(
    mut commands: Commands,
    menu_query: Query<Entity, With<MainMenuMarker>>,
) {
    for entity in menu_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

#[derive(Component)]
struct MainMenuMarker;

#[derive(Component)]
struct StartGameButton;
