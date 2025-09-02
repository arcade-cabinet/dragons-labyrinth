use bevy::prelude::*;
use rand::Rng;
use crate::components::*;
use crate::resources::{Stats, Abilities, award_xp, check_unlocks, LightDark};

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
                let x = pos.q as f32 * 64.0; let y = (pos.r+1) as f32 * 48.0;
                commands.spawn((SpriteBundle { texture: tex, transform: Transform::from_xyz(x,y,12.0), ..Default::default() }, Creature{ id:"shade".into(), name:"Shade".into(), tags: vec!["encounter".into()] }));
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
