use bevy::prelude::*;
use crate::world::components::{DreadSource, Player};
use crate::world::resources::{DreadLevel, WorldState};

pub fn dread_progression_system(
    time: Res<Time>,
    mut dread_level: ResMut<DreadLevel>,
    player_query: Query<&Transform, With<Player>>,
    dread_sources: Query<(&Transform, &DreadSource)>,
    world_state: Res<WorldState>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        let mut dread_accumulation = 0.0;
        
        // Base dread progression over time
        dread_accumulation += 0.1 * time.delta_seconds();
        
        // Calculate dread from nearby sources
        for (source_transform, dread_source) in dread_sources.iter() {
            let distance = player_transform.translation.distance(source_transform.translation);
            let max_range = dread_source.range;
            
            if distance <= max_range {
                let distance_factor = 1.0 - (distance / max_range);
                let source_dread = dread_source.intensity * distance_factor;
                dread_accumulation += source_dread * time.delta_seconds();
            }
        }
        
        // Apply corruption-based dread
        if let Some(player_hex) = world_state.player_hex {
            if let Some(&corruption_level) = world_state.corruption_map.get(&player_hex) {
                dread_accumulation += corruption_level * 2.0 * time.delta_seconds();
            }
        }
        
        // Update dread level with decay
        let decay_rate = calculate_decay_rate(dread_level.current);
        dread_level.current += dread_accumulation - (decay_rate * time.delta_seconds());
        dread_level.current = dread_level.current.clamp(0.0, 120.0); // Allow over 100 for void states
        
        // Update dread phase
        update_dread_phase(&mut dread_level);
        
        // Handle phase transitions
        if dread_level.phase_changed_this_frame {
            handle_dread_phase_change(&dread_level);
        }
    }
}

fn calculate_decay_rate(current_dread: f32) -> f32 {
    match current_dread as i32 {
        0..=20 => 2.0,   // Peace: Fast recovery
        21..=40 => 1.0,  // Unease: Moderate recovery
        41..=60 => 0.5,  // Dread: Slow recovery
        61..=80 => 0.2,  // Terror: Very slow recovery
        81..=100 => 0.0, // Void: No natural recovery
        _ => -0.5,       // Beyond void: Actively worsens
    }
}

fn update_dread_phase(dread_level: &mut ResMut<DreadLevel>) {
    let previous_phase = dread_level.phase.clone();
    
    dread_level.phase = match dread_level.current as i32 {
        0..=20 => crate::world::components::DreadPhase::Peace,
        21..=40 => crate::world::components::DreadPhase::Unease,
        41..=60 => crate::world::components::DreadPhase::Dread,
        61..=80 => crate::world::components::DreadPhase::Terror,
        81..=100 => crate::world::components::DreadPhase::Void,
        _ => crate::world::components::DreadPhase::BeyondVoid,
    };
    
    dread_level.phase_changed_this_frame = previous_phase != dread_level.phase;
}

fn handle_dread_phase_change(dread_level: &DreadLevel) {
    match dread_level.phase {
        crate::world::components::DreadPhase::Peace => {
            info!("Dread Phase: Peace - The world feels safe and welcoming");
        }
        crate::world::components::DreadPhase::Unease => {
            info!("Dread Phase: Unease - Something feels... wrong");
            // TODO: Start subtle audio/visual changes
        }
        crate::world::components::DreadPhase::Dread => {
            info!("Dread Phase: Dread - The horror is manifesting");
            // TODO: More obvious corruption effects
        }
        crate::world::components::DreadPhase::Terror => {
            info!("Dread Phase: Terror - Reality is breaking down");
            // TODO: Significant world corruption, companion panic
        }
        crate::world::components::DreadPhase::Void => {
            info!("Dread Phase: Void - The void consumes everything");
            // TODO: World becomes increasingly alien
        }
        crate::world::components::DreadPhase::BeyondVoid => {
            info!("Dread Phase: Beyond Void - You have seen too much");
            // TODO: Complete reality breakdown, final encounters
        }
    }
}

pub fn spawn_dread_source(
    mut commands: Commands,
    position: Vec3,
    intensity: f32,
    range: f32,
    source_type: String,
) {
    commands.spawn((
        DreadSource {
            intensity,
            range,
            source_type: source_type.clone(),
            is_permanent: false,
        },
        Transform::from_translation(position),
        GlobalTransform::default(),
        Name::new(format!("DreadSource_{}", source_type)),
    ));
}

pub fn dread_visual_effects_system(
    dread_level: Res<DreadLevel>,
    mut clear_color: ResMut<ClearColor>,
    // TODO: Add fog, particle systems, shader effects
) {
    if dread_level.phase_changed_this_frame {
        match dread_level.phase {
            crate::world::components::DreadPhase::Peace => {
                clear_color.0 = Color::srgb(0.7, 0.9, 1.0); // Light blue sky
            }
            crate::world::components::DreadPhase::Unease => {
                clear_color.0 = Color::srgb(0.8, 0.8, 0.7); // Slightly yellow/grey
            }
            crate::world::components::DreadPhase::Dread => {
                clear_color.0 = Color::srgb(0.6, 0.5, 0.5); // Reddish grey
            }
            crate::world::components::DreadPhase::Terror => {
                clear_color.0 = Color::srgb(0.4, 0.2, 0.2); // Dark red
            }
            crate::world::components::DreadPhase::Void => {
                clear_color.0 = Color::srgb(0.1, 0.05, 0.1); // Near black with purple tint
            }
            crate::world::components::DreadPhase::BeyondVoid => {
                clear_color.0 = Color::srgb(0.0, 0.0, 0.0); // Pure black
            }
        }
    }
}
