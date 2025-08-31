use bevy::prelude::*;
use world::{plugin::WorldPlugin, resources::WorldBook, material::HexTileMaterial};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Worldbuilder".into(),
                resolution: (1280., 800.).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(Material2dPlugin::<HexTileMaterial>::default())
        .add_plugins(WorldPlugin)
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, load_worldbook)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn load_worldbook(mut commands: Commands) {
    let text = std::fs::read_to_string("build/world/worldbook.json").expect("build/world/worldbook.json");
    let wb: WorldBook = serde_json::from_str(&text).expect("valid worldbook");
    commands.insert_resource(wb);
}
