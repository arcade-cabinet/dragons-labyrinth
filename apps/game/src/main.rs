use bevy::prelude::*;
use horror_rpg::run_app;

fn main() {
    #[cfg(target_arch = "wasm32")]
    {
        console_error_panic_hook::set_once();
    }
    run_app();
}
