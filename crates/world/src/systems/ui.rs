use bevy::prelude::*;
use crate::abilities::{Stats, Abilities};
use crate::alignment::LightDark;
use crate::components::{Player, AxialPos};

#[derive(Component)] pub struct HudText;

pub fn setup_ui(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section("", TextStyle{ font: Default::default(), font_size: 16.0, color: Color::WHITE })
            .with_text_alignment(TextAlignment::Left)
            .with_style(Style{ position_type: PositionType::Absolute, left: Val::Px(10.0), top: Val::Px(10.0), ..default() }),
        HudText,
    ));
}

pub fn ui_update(mut q: Query<&mut Text, With<HudText>>, stats: Res<Stats>, abilities: Res<Abilities>, align: Res<LightDark>, player: Query<&AxialPos, With<Player>>) {
    if let Ok(mut text) = q.get_single_mut() {
        let pos = player.get_single().map(|p| format!("q={}, r={}", p.q, p.r)).unwrap_or("?".into());
        let mut abil: Vec<_> = abilities.unlocked.iter().cloned().collect();
        abil.sort();
        text.sections[0].value = format!("[Arrows] move  [E] encounter  [T] shop  [Q] quest log  [R] reload  [S] save  [Space] talk\nPos {} | XP m:{} st:{} lo:{} cr:{} | Align L:{} D:{} | Abilities: {}", pos, stats.melee, stats.stealth, stats.lore, stats.craft, align.light, align.dark, abil.join(", "));
    }
}
