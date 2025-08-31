use bevy::prelude::*;

pub fn hint_ui(mut commands: Commands) {
    commands.spawn(TextBundle::from_section(
        "QWEASD move | R reload | T shop | Enter dungeon | Esc exit",
        TextStyle { font_size: 16.0, color: Color::WHITE, font: Default::default() }
    ));
}
