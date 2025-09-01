mod game;
mod systems;
mod components;
mod resources;
mod utils;

use bevy::prelude::*;
use game::*;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn run_app() {
    App::new()
        .add_plugins(HorrorRpgPlugin)
        .run();
}
