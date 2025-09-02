use bevy::prelude::*;

pub struct WorldPlugin;
impl Plugin for WorldPlugin {
	fn build(&self, _app: &mut App) {
		// Game now centralizes systems in HorrorRpgPlugin; this module remains for parity.
	}
}