use bevy::prelude::*;
use crate::components::*;

pub fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let tex = asset_server.load("sprites/party_pawn.png");
    commands.spawn((
        SpriteBundle { texture: tex, transform: Transform::from_xyz(0.0, 0.0, 12.0), ..default() },
        Player, Pawn, AxialPos { q:0, r:0 }, Name::new("Player")
    ));
}

pub fn camera_follow(player_q: Query<&Transform, With<Player>>, mut cams: Query<&mut Transform, (With<Camera>, Without<Player>)>) {
    if let Ok(p) = player_q.get_single() {
        for mut c in cams.iter_mut() { c.translation.x = p.translation.x; c.translation.y = p.translation.y; }
    }
}

pub fn handle_input_move(keys: Res<ButtonInput<KeyCode>>, mut q: Query<(&mut Transform, &mut AxialPos), With<Player>>) {
    if let Ok((mut tf, mut ap)) = q.get_single_mut() {
        let mut dir: Option<(i32,i32)> = None;
        if keys.just_pressed(KeyCode::ArrowUp) { dir = Some((0,-1)); }
        if keys.just_pressed(KeyCode::ArrowDown) { dir = Some((0,1)); }
        if keys.just_pressed(KeyCode::ArrowLeft) { dir = Some((-1,0)); }
        if keys.just_pressed(KeyCode::ArrowRight) { dir = Some((1,0)); }
        if let Some((dq, dr)) = dir {
            ap.q += dq; ap.r += dr;
            let (x,y) = axial_to_world(ap.q, ap.r, 32.0);
            tf.translation.x = x; tf.translation.y = y;
        }
    }
}

pub fn axial_to_world(q: i32, r: i32, radius: f32) -> (f32, f32) {
    let x = (1.5 * q as f32) * radius;
    let y = ((r as f32 * 1.732) + (q as f32 * 0.866)) * radius;
    (x, y)
}
