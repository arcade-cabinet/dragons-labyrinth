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
// Consolidated: no separate WorldPlugin needed

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        // Core Bevy plugins
        app.add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Horror RPG".into(),
                    canvas: Some("#game-canvas".into()),
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: false,
                    ..default()
                }),
                ..default()
            })
            .set(bevy::asset::AssetPlugin {
                // assets/ is copied to dist by Trunk; load with paths like "textures/..."
                file_path: "assets".into(),
                ..default()
            })
        );

        // Third-party plugins
        app.add_plugins((
            TilemapPlugin,
            PhysicsPlugins::default(),
            CobwebPlugin::default(),
            CobwebUiPlugin,
            YarnSpinnerPlugin,
            YoleckPlugin,
        ));

        // World organization consolidated into this plugin

        // Game resources
        app.init_resource::<WorldState>()
            .init_resource::<GameState>()
            .init_resource::<DreadLevel>()
            .init_resource::<AssetHandles>()
            .init_resource::<RegionalProgression>()
            .init_resource::<MovementPreview>()
            .init_resource::<DayNightCycle>()
            .init_resource::<WeatherSystem>()
            .init_resource::<CharacterCreator>()
            .init_resource::<UIManager>()
            .init_resource::<ProceduralAudioSystem>();

        // Game components registration
        app.register_component_hooks::<Tile>()
            .register_component_hooks::<Player>()
            .register_component_hooks::<Companion>()
            .register_component_hooks::<DreadSource>()
            .register_component_hooks::<NPC>()
            .register_component_hooks::<Monster>()
            .register_component_hooks::<CharacterModel>();

        // Game systems
        app.add_systems(Startup, (
            setup_camera,
            setup_world,
            setup_splash_screen,
            load_assets,
        ))
        .add_systems(Update, (
            player_movement_system,
            hex_world_generation_system,
            companion_psychology_system,
            dread_progression_system,
            asset_loading_system,
            ui_update_system,
            update_day_night_cycle,
            update_movement_preview,
            check_forced_rest,
            update_regional_progression,
            update_horror_audio,
            update_ui_animations,
            update_procedural_audio,
            play_ui_sound_effects,
        ).run_if(in_state(GameStateEnum::Playing)))
        
        // Splash screen
        .add_systems(Update, update_splash_screen.run_if(in_state(GameStateEnum::Loading)))
        .add_systems(OnExit(GameStateEnum::Loading), cleanup_ui_screen::<SplashScreen>)
        
        // Main menu
        .add_systems(OnEnter(GameStateEnum::MainMenu), (setup_main_menu, play_menu_audio))
        .add_systems(Update, handle_main_menu_input.run_if(in_state(GameStateEnum::MainMenu)))
        .add_systems(OnExit(GameStateEnum::MainMenu), cleanup_ui_screen::<MainMenuScreen>)
        
        // Character creation
        .add_systems(OnEnter(GameStateEnum::CharacterCreation), (setup_character_creator_ui, play_character_creation_audio))
        .add_systems(Update, handle_character_creator_input.run_if(in_state(GameStateEnum::CharacterCreation)))
        .add_systems(OnExit(GameStateEnum::CharacterCreation), (
            cleanup_ui_screen::<CharacterCreationScreen>,
            spawn_player_character,
        ));

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
    CharacterCreation,
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
    asset_server: Res<AssetServer>,
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
            texture: TilemapTexture::Vector(vec![
                asset_server.load("textures/tilemap.png"),
                asset_server.load("textures/tilemap_extended.png")
            ]),
            tile_size: TilemapTileSize { x: 64.0, y: 64.0 },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..default()
        },
    )).id();

    world_state.tilemap_entity = Some(tilemap_entity);
}

// Removed - using Cobweb UI system instead

// Removed - using Cobweb UI system instead

#[derive(Component)]
struct MainMenuMarker;

#[derive(Component)]
pub struct StartGameButton;
