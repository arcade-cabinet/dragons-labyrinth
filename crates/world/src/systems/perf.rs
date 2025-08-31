use bevy::prelude::*;
use crate::systems::spawn::HexCell;

#[derive(Resource)] pub struct PerfSettings { pub cull_distance: f32, pub edges_enabled: bool }
impl Default for PerfSettings { fn default() -> Self { Self { cull_distance: 900.0, edges_enabled: true } } }

pub fn cull_far_tiles(cameras: Query<&Transform, With<Camera>>, mut q: Query<(&Transform, &mut Visibility), With<HexCell>>, settings: Res<PerfSettings>) {
    if let Ok(cam) = cameras.get_single() {
        let cx = cam.translation.x; let cy = cam.translation.y;
        for (tf, mut vis) in q.iter_mut() {
            let dx = tf.translation.x - cx; let dy = tf.translation.y - cy;
            let dist2 = dx*dx + dy*dy;
            vis.set_if_neq(if dist2 > settings.cull_distance*settings.cull_distance { Visibility::Hidden } else { Visibility::Visible });
        }
    }
}

pub fn toggle_batching(keys: Res<ButtonInput<KeyCode>>, mut settings: ResMut<PerfSettings>) {
    if keys.just_pressed(KeyCode::KeyB) { settings.edges_enabled = !settings.edges_enabled; info!("Batching mode {}", if settings.edges_enabled {"OFF"} else {"ON"}); }
}
