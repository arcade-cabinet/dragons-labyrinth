mod game;
mod world;
mod utils;
pub mod spatial;

// Include generated world resources at build time
include!(concat!(env!("OUT_DIR"), "/generated_world.rs"));

use bevy::prelude::*;
use game::*;
use spatial::SpatialPlugin;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn run_app() {
	App::new()
		.add_plugins(GamePlugin)
		.run();
}
