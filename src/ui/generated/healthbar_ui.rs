// Auto-generated Cobweb UI component for health_bar
use bevy::prelude::*;
use cobweb_ui::prelude::*;

#[derive(Component)]
pub struct HealthbarUI {
    dread_level: u8,
    degradation_active: bool,
    sanity_level: f32,
}

pub fn spawn_healthbar_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    dread: Res<DreadState>,
) {
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                ..default()
            },
            ..default()
        },
        HealthbarUI {
            dread_level: dread.level,
            degradation_active: dread.level > 0,
            sanity_level: 1.0,
        },
    )).with_children(|parent| {
        // Base UI structure
        parent.spawn(NodeBundle {
            style: Style {
                width: Val::Px(200.0),
                height: Val::Px(50.0),
                ..default()
            },
            background_color: Color::srgba(0.1, 0.1, 0.1, 0.8).into(),
            ..default()
        });
    });
}

pub fn update_healthbar_degradation(
    mut query: Query<(&mut HealthbarUI, &mut BackgroundColor)>,
    dread: Res<DreadState>,
    time: Res<Time>,
) {
    for (mut ui, mut bg_color) in query.iter_mut() {
        if ui.dread_level != dread.level {
            ui.dread_level = dread.level;
            ui.degradation_active = dread.level > 0;
        }
        
        // Apply dread-based degradation
        match dread.level {
            0 => {
                // Clean state
                *bg_color = Color::srgba(0.1, 0.1, 0.1, 0.8).into();
            },
            1 => {
                // Subtle flicker
                let flicker = (time.elapsed_secs() * 10.0).sin() * 0.05;
                *bg_color = Color::srgba(0.1, 0.1 - flicker, 0.1, 0.8).into();
            },
            2 => {
                // Corruption spreading
                let corruption = (time.elapsed_secs() * 2.0).sin() * 0.2;
                *bg_color = Color::srgba(0.2 + corruption, 0.05, 0.1, 0.85).into();
            },
            3 => {
                // Major distortion
                let distortion = (time.elapsed_secs() * 5.0).sin() * 0.3;
                *bg_color = Color::srgba(0.3 + distortion, 0.0, 0.05, 0.9).into();
            },
            4 => {
                // Nightmare mode
                let nightmare = (time.elapsed_secs() * 20.0).sin().abs();
                *bg_color = Color::srgba(nightmare, 0.0, 0.0, 0.95).into();
            },
            _ => {}
        }
    }
}
