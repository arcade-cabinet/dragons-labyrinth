use bevy::prelude::*;
use crate::components::{Companion, CompanionState, TraumaLevel};
use crate::resources::{DreadLevel, GameState};

pub fn companion_psychology_system(
    time: Res<Time>,
    dread_level: Res<DreadLevel>,
    mut companion_query: Query<(&mut Companion, &Transform)>,
    player_query: Query<&Transform, (With<crate::components::Player>, Without<Companion>)>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (mut companion, companion_transform) in companion_query.iter_mut() {
            // Calculate distance from player (affects anxiety)
            let distance = player_transform.translation.distance(companion_transform.translation);
            let distance_stress = (distance - 5.0).max(0.0) * 0.1;
            
            // Apply dread effects to companion psychology
            let dread_stress = match dread_level.current {
                0..=20 => 0.0,
                21..=40 => 0.5,
                41..=60 => 1.5,
                61..=80 => 3.0,
                81..=100 => 5.0,
                _ => 10.0,
            };
            
            // Update companion stress
            companion.stress += (distance_stress + dread_stress) * time.delta_seconds();
            companion.stress = companion.stress.clamp(0.0, 100.0);
            
            // Update trust based on player actions and time
            if companion.stress < 30.0 {
                companion.trust += 0.1 * time.delta_seconds();
            } else if companion.stress > 70.0 {
                companion.trust -= 0.2 * time.delta_seconds();
            }
            companion.trust = companion.trust.clamp(0.0, 100.0);
            
            // Update companion state based on stress and trust
            update_companion_state(&mut companion);
            
            // Handle trauma accumulation
            if companion.stress > 80.0 && companion.trust < 20.0 {
                companion.trauma_level = TraumaLevel::Severe;
            } else if companion.stress > 60.0 && companion.trust < 40.0 {
                companion.trauma_level = TraumaLevel::Moderate;
            } else if companion.stress > 40.0 {
                companion.trauma_level = TraumaLevel::Mild;
            }
            
            // Log significant changes for dialogue triggers
            if companion.state_changed_this_frame {
                info!("Companion {} state changed to {:?}", 
                      companion.name, companion.state);
                
                // TODO: Trigger appropriate dialogue based on new state
                // This would integrate with Bevy YarnSpinner
            }
        }
    }
}

fn update_companion_state(companion: &mut Companion) {
    let previous_state = companion.state.clone();
    
    companion.state = match (companion.stress as i32, companion.trust as i32) {
        (0..=20, 80..=100) => CompanionState::Loyal,
        (0..=30, 60..=100) => CompanionState::Content,
        (0..=40, 40..=100) => CompanionState::Stable,
        (41..=60, 50..=100) => CompanionState::Nervous,
        (41..=60, 0..=49) => CompanionState::Wary,
        (61..=80, 30..=100) => CompanionState::Distressed,
        (61..=80, 0..=29) => CompanionState::Hostile,
        (81..=100, _) => CompanionState::Broken,
        _ => CompanionState::Stable,
    };
    
    companion.state_changed_this_frame = previous_state != companion.state;
}

pub fn spawn_companion(
    mut commands: Commands,
    name: String,
    companion_type: String,
    spawn_position: Vec3,
) {
    commands.spawn((
        Companion {
            name: name.clone(),
            companion_type,
            stress: 10.0,
            trust: 50.0,
            state: CompanionState::Stable,
            trauma_level: TraumaLevel::None,
            dialogue_flags: std::collections::HashMap::new(),
            state_changed_this_frame: false,
        },
        Transform::from_translation(spawn_position),
        GlobalTransform::default(),
        Visibility::default(),
        Name::new(format!("Companion_{}", name)),
    ));
}

pub fn companion_dialogue_triggers(
    mut companion_query: Query<&Companion, Changed<Companion>>,
    // TODO: Add YarnSpinner dialogue runner integration
) {
    for companion in companion_query.iter_mut() {
        if companion.state_changed_this_frame {
            match companion.state {
                CompanionState::Distressed => {
                    // Trigger distressed dialogue
                    info!("Triggering distressed dialogue for {}", companion.name);
                }
                CompanionState::Hostile => {
                    // Trigger hostile dialogue
                    info!("Triggering hostile dialogue for {}", companion.name);
                }
                CompanionState::Broken => {
                    // Trigger broken/trauma dialogue
                    info!("Triggering trauma dialogue for {}", companion.name);
                }
                _ => {}
            }
        }
    }
}
