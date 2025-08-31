use bevy::prelude::*;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)] struct QuestlineLevel { level: String, beats: Vec<String> }
#[derive(Debug, Clone, Deserialize)] struct QuestlinePack { quest_id: String, levels: Vec<QuestlineLevel> }

#[derive(Component)] pub struct QuestUIRoot;

pub fn quest_ui_toggle(keys: Res<ButtonInput<KeyCode>>, mut commands: Commands, q_ui: Query<Entity, With<QuestUIRoot>>) {
    if keys.just_pressed(KeyCode::KeyQ) {
        let open = q_ui.get_single().is_ok();
        if open {
            for e in q_ui.iter() { commands.entity(e).despawn_recursive(); }
        } else {
            spawn_quest_ui(&mut commands);
        }
    }
}

fn spawn_quest_ui(commands: &mut Commands) {
    let mut root = commands.spawn((NodeBundle{
        style: Style { position_type: PositionType::Absolute, right: Val::Px(20.0), top: Val::Px(20.0), width: Val::Percent(35.0), height: Val::Percent(60.0), flex_direction: FlexDirection::Column, ..default() },
        background_color: BackgroundColor(Color::rgba(0.05,0.05,0.06,0.9)),
        ..default()
    }, QuestUIRoot));

    root.with_children(|p| {
        p.spawn(TextBundle::from_section("Quest Log", TextStyle{ font: Default::default(), font_size: 20.0, color: Color::ORANGE_RED }));
        if let Ok(rd) = std::fs::read_dir("build/narrative") {
            for ent in rd.flatten() {
                if let Some(name) = ent.file_name().to_str() {
                    if name.starts_with("questline_") && name.ends_with(".json") {
                        if let Ok(text) = std::fs::read_to_string(ent.path()) {
                            if let Ok(pack) = serde_json::from_str::<QuestlinePack>(&text) {
                                p.spawn(TextBundle::from_section(format!("- {}", pack.quest_id), TextStyle{ font: Default::default(), font_size: 16.0, color: Color::WHITE }));
                                for lvl in pack.levels {
                                    p.spawn(TextBundle::from_section(format!("  [{}]", lvl.level), TextStyle{ font: Default::default(), font_size: 14.0, color: Color::GRAY }));
                                    for beat in lvl.beats {
                                        p.spawn(TextBundle::from_section(format!("    • {}", beat), TextStyle{ font: Default::default(), font_size: 14.0, color: Color::WHITE }));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        p.spawn(TextBundle::from_section("[Q] close", TextStyle{ font: Default::default(), font_size: 14.0, color: Color::GRAY }));
    });
}
