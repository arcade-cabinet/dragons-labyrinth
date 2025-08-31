use bevy::prelude::*;
use crate::resources::{WorldBook, PlayerState, Lighting, GameMode};
use crate::systems::{spawn::spawn_world, hot_reload::hot_reload_keys, movement::{handle_input_move, click_to_move_setup, handle_click_set_destination, follow_click_path},
    encounters::random_encounters, shops::{load_shops_from_disk, open_shop_ui},
    dungeon::{enter_dungeon, exit_dungeon, render_dungeon}, lighting::cycle_ambient, npc_ui::hint_ui};

pub struct WorldPlugin;
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerState::default())
           .insert_resource(Lighting { ambient: 0.8 })
           .insert_resource(GameMode::default())
           .add_systems(Startup, (spawn_world, load_shops_from_disk, hint_ui, click_to_move_setup))
           .add_systems(Update, (hot_reload_keys, handle_input_move, handle_click_set_destination, follow_click_path, random_encounters, open_shop_ui, enter_dungeon, exit_dungeon, render_dungeon, cycle_ambient));
    }
}
