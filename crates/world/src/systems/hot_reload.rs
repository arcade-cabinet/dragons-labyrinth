use bevy::prelude::*;
use crate::resources::WorldBook;

pub fn hot_reload_keys(
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut wb_opt: Option<ResMut<WorldBook>>,
    q: Query<Entity>,
) {
    if keys.just_pressed(KeyCode::KeyR) {
        if let Ok(text) = std::fs::read_to_string("build/world/worldbook.json") {
            if let Ok(new_wb) = serde_json::from_str::<WorldBook>(&text) {
                for e in q.iter() { commands.entity(e).despawn_recursive(); }
                if let Some(mut wb) = wb_opt { *wb = new_wb; } else { commands.insert_resource(new_wb); }
                info!("reloaded worldbook.json");
            }
        }
    }
}
