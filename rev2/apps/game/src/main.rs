use bevy::prelude::*;
use bevy::asset::AssetPlugin;
use world::{plugin::WorldPlugin, resources::WorldBook};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    file_path: "apps/game/assets".into(),
                    ..Default::default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bevy Worldbuilder (Tilemap)".into(),
                        resolution: (1280., 800.).into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
        )
        .add_plugins(WorldPlugin)
        .add_systems(Startup, load_worldbook)
        .run();
}

fn load_worldbook(mut commands: Commands) {
    std::fs::create_dir_all("build/world").ok();
    let path = "build/world/worldbook.json";
    let text = std::fs::read_to_string(path).unwrap_or_else(|_| {
        // Seed a tiny default book if missing
        let default = r#"
        {"title":"Seed Book","regions":[
            {"name":"Green Vale","band":[1,10],
             "hex_points":[{"axial":"0,0","kind":"village"},{"axial":"0,-1","kind":"shrine"}],
             "npcs":[{"id":"n_vicar","name":"Under-Vicar Marn","axial":"0,0"}]
            }]} "#;
        std::fs::write(path, default).ok();
        default.into()
    });
    let wb: WorldBook = serde_json::from_str(&text).expect("valid worldbook");
    commands.insert_resource(wb);
}
