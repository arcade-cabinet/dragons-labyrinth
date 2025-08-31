use bevy::prelude::*;
use rand::Rng;
use crate::components::*;
use crate::systems::movement::axial_to_world;
use crate::abilities::{Stats, award_xp, Abilities, check_unlocks};
use crate::alignment::LightDark;

pub fn encounter_roller(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    player: Query<&AxialPos, With<Player>>,
    asset_server: Res<AssetServer>,
    mut stats: ResMut<Stats>,
    mut abilities: ResMut<Abilities>,
    mut align: ResMut<LightDark>,
) {
    if keys.just_pressed(KeyCode::KeyE) {
        if let Ok(pos) = player.get_single() {
            let mut rng = rand::thread_rng();
            let roll: f32 = rng.gen();
            if roll < 0.6 {
                let tex = asset_server.load("sprites/enemy_pawn.png");
                let (x,y) = axial_to_world(pos.q, pos.r+1, 32.0);
                commands.spawn((SpriteBundle { texture: tex, transform: Transform::from_xyz(x, y, 12.0), ..default() }, Creature{ id:"shade".into(), name:"Shade".into(), tags: vec!["encounter".into()] }));
                award_xp(&mut stats, "melee", 1);
                align.dark += 1;
                check_unlocks(&stats, &mut abilities);
                info!("Encounter spawned; melee+1, dark+1");
            } else {
                award_xp(&mut stats, "stealth", 1);
                align.light += 1;
                check_unlocks(&stats, &mut abilities);
                info!("Avoided encounter; stealth+1, light+1");
            }
        }
    }
}
