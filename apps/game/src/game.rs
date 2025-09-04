use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_cobweb::prelude::*;
use bevy_cobweb_ui::prelude::*;
use bevy_yarnspinner::prelude::*;

use crate::world::systems::*;
use crate::world::components::*;
use crate::world::state::*;
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
            CobwebPlugin::default(),
            CobwebUiPlugin,
            YarnSpinnerPlugin::default(),
        ));

        // World organization consolidated into this plugin

        // Game resources - only include what actually exists
        app.init_resource::<WorldState>()
            .init_resource::<GameState>()
            .init_resource::<DreadLevel>()
            .init_resource::<AssetHandles>();

        // Game systems - only include what actually exists
        app.add_systems(Startup, (
            setup_camera,
            setup_world,
        ))
        .add_systems(Update, (
            cross_platform_input_system,
            layer_cake_hex_world_system,
            companion_psychology_system,
            dread_progression_system,
            asset_loading_system,
            ui_update_system,
            check_forced_rest,
        ).run_if(in_state(GameStateEnum::Playing)));

        // Game states
        app.init_state::<GameStateEnum>();
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
        Camera3d::default(),
        Transform::from_xyz(0.0, 50.0, 0.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        OrthographicProjection {
            scale: 0.5,
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

pub fn run_app() {
    App::new()
        .add_plugins(GamePlugin)
        .run();
}
