use bevy::prelude::*;
use serde::Deserialize;
use crate::components::*;

#[derive(Debug, Clone, Deserialize)]
struct DialogueLine { speaker: String, line: String, intent: String, stage: String }
#[derive(Debug, Clone, Deserialize)]
struct DialoguePack { npc_id: String, context: String, lines: Vec<DialogueLine> }

#[derive(Component)] pub struct DialogueUI;
#[derive(Resource, Default)] pub struct CurrentDialogue { pub lines: Vec<DialogueLine>, pub idx: usize }

pub fn interact_dialogue(
    keys: Res<ButtonInput<KeyCode>>,
    player: Query<&AxialPos, With<Player>>,
    npcs: Query<(&AxialPos, &Npc)>,
    mut commands: Commands,
    mut cur: ResMut<CurrentDialogue>,
) {
    if keys.just_pressed(KeyCode::Space) {
        if let Ok(pp) = player.get_single() {
            for (ap, npc) in npcs.iter() {
                if ap.q == pp.q && ap.r == pp.r {
                    let path = format!("build/narrative/dialogue_{}.json", npc.id);
                    if let Ok(text) = std::fs::read_to_string(&path) {
                        if let Ok(pack) = serde_json::from_str::<DialoguePack>(&text) {
                            cur.lines = pack.lines;
                            cur.idx = 0;
                            spawn_dialogue_ui(&mut commands, &cur);
                        }
                    }
                }
            }
        }
    }
    if keys.just_pressed(KeyCode::Backspace) {
        for e in commands.iter_entities().filter(|(_,c)| c.contains::<DialogueUI>()) { commands.entity(e.0).despawn_recursive(); }
        cur.lines.clear(); cur.idx = 0;
    }
    if keys.just_pressed(KeyCode::Enter) {
        if !cur.lines.is_empty() { cur.idx = (cur.idx + 1).min(cur.lines.len()-1); spawn_dialogue_ui(&mut commands, &cur); }
    }
}

fn spawn_dialogue_ui(commands: &mut Commands, cur: &CurrentDialogue) {
    let mut root = commands.spawn((NodeBundle{
        style: Style { position_type: PositionType::Absolute, left: Val::Px(40.0), bottom: Val::Px(30.0), width: Val::Percent(60.0), height: Val::Px(120.0), ..default() },
        background_color: BackgroundColor(Color::rgba(0.0,0.0,0.0,0.7)),
        ..default()
    }, DialogueUI));
    if let Some(line) = cur.lines.get(cur.idx) {
        root.with_children(|p| {
            p.spawn(TextBundle::from_section(format!("{}: {}", line.speaker, line.line), TextStyle{ font: Default::default(), font_size: 18.0, color: Color::WHITE }));
            p.spawn(TextBundle::from_section("[Enter] next  [Backspace] close", TextStyle{ font: Default::default(), font_size: 14.0, color: Color::GRAY }));
        });
    }
}
