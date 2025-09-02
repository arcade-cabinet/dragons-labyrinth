pub mod tilemap_spawn;
pub mod movement;
pub mod encounters;
pub mod ui;
pub mod shops;
pub mod dungeon;
pub mod dialogue;
pub mod quests;
pub mod save;

use bevy::prelude::*;
pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
