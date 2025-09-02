use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use crate::index::WorldIndex;
use crate::resources::{WorldBook, GameSave, LightDark, Stats, Abilities};
use crate::systems;

pub struct WorldPlugin;
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(TilemapPlugin)
            .init_resource::<WorldIndex>()
            .init_resource::<LightDark>()
            .init_resource::<Stats>()
            .init_resource::<Abilities>()
            .insert_resource(GameSave::default())
            .add_systems(Startup, (systems::setup_camera, systems::ui::setup_ui).chain())
            .add_systems(Startup, (systems::tilemap_spawn::spawn_from_worldbook, systems::movement::setup_player).chain())
            .add_systems(Update, (
                systems::movement::camera_follow,
                systems::movement::handle_input_move,
                systems::encounters::encounter_roller,
                systems::ui::ui_update,
                systems::shops::shop_toggle_ui,
                systems::dungeon::dungeon_enter_exit,
                systems::dialogue::interact_dialogue,
                systems::quests::quest_ui_toggle,
                systems::save::save_on_key,
            ));
    }
}
