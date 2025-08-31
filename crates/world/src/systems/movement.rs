use bevy::prelude::*;
use std::collections::{HashSet, VecDeque};
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

#[derive(Resource, Default)]
pub struct ClickPath { pub waypoints: VecDeque<(i32,i32)>, pub tile_radius: f32 }

pub fn click_to_move_setup(mut commands: Commands) {
    commands.insert_resource(ClickPath { waypoints: VecDeque::new(), tile_radius: 32.0 });
}

fn world_to_axial(x: f32, y: f32, radius: f32) -> (i32, i32) {
    // Approximate inverse of axial_to_world for point selection
    let qf = (x / (1.5 * radius)).round();
    let rf = ((y - (qf * 0.866 * radius)) / (1.732 * radius)).round();
    (qf as i32, rf as i32)
}

fn neighbors(q: i32, r: i32) -> [(i32,i32);6] {
    [(0,-1),(1,-1),(1,0),(0,1),(-1,1),(-1,0)].map(|(dq,dr)| (q+dq, r+dr))
}

pub fn handle_click_set_destination(
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut path: ResMut<ClickPath>,
    player_q: Query<&AxialPos, With<Player>>,
) {
    if !buttons.just_pressed(MouseButton::Left) { return; }
    let window = if let Ok(w) = windows.get_single() { w } else { return; };
    let (camera, cam_tf) = if let Ok(c) = camera_q.get_single() { c } else { return; };
    if let Some(cursor) = window.cursor_position() {
        if let Some(ray) = camera.viewport_to_world_2d(cam_tf, cursor) {
            let (dest_q, dest_r) = world_to_axial(ray.x, ray.y, path.tile_radius);
            if let Ok(player_pos) = player_q.get_single() {
                // BFS for shortest path on uniform-cost hex grid
                let start = (player_pos.q, player_pos.r);
                let target = (dest_q, dest_r);
                if start == target { return; }
                let mut frontier: VecDeque<(i32,i32)> = VecDeque::new();
                let mut came_from: std::collections::HashMap<(i32,i32),(i32,i32)> = std::collections::HashMap::new();
                let mut visited: HashSet<(i32,i32)> = HashSet::new();
                frontier.push_back(start);
                visited.insert(start);
                let mut found = false;
                while let Some((cq, cr)) = frontier.pop_front() {
                    for (nq, nr) in neighbors(cq, cr) {
                        if visited.contains(&(nq,nr)) { continue; }
                        visited.insert((nq,nr));
                        came_from.insert((nq,nr), (cq,cr));
                        if (nq, nr) == target { found = true; break; }
                        frontier.push_back((nq,nr));
                    }
                    if found { break; }
                }
                if found {
                    let mut rev: Vec<(i32,i32)> = Vec::new();
                    let mut cur = target;
                    while cur != start {
                        rev.push(cur);
                        cur = *came_from.get(&cur).unwrap();
                    }
                    rev.reverse();
                    path.waypoints = VecDeque::from(rev);
                }
            }
        }
    }
}

pub fn follow_click_path(mut path: ResMut<ClickPath>, mut q: Query<(&mut Transform, &mut AxialPos), With<Player>>) {
    if path.waypoints.is_empty() { return; }
    if let Ok((mut tf, mut ap)) = q.get_single_mut() {
        if let Some((nq, nr)) = path.waypoints.pop_front() {
            ap.q = nq; ap.r = nr;
            let (x,y) = axial_to_world(ap.q, ap.r, path.tile_radius);
            tf.translation.x = x; tf.translation.y = y;
        }
    }
}
