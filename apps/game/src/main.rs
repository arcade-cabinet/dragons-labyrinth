use bevy::prelude::*;
mod world;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Dragon's Labyrinth - Physics Refactor".into(),
            resolution: (1280., 800.).into(),
            ..default()
        }),
        ..default()
    }));
    world::register(&mut app);
    app.run();
}

// No worldbook loading; all ECS is generated at build time into world module
