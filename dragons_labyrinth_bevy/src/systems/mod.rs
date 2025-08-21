use bevy::prelude::*;
use crate::components::*;
use crate::resources::*;

// Core dread progression system - heart of horror-first design
pub fn dread_progression_system(
    mut dread_state: ResMut<DreadState>,
    mut hex_world: ResMut<HexWorld>,
    mut narrative_state: ResMut<NarrativeState>,
    mut ambient_light: ResMut<AmbientLight>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    tile_query: Query<(Entity, &HexTile, &Handle<StandardMaterial>)>,
) {
    let previous_level = dread_state.current_level;
    
    // Example progression trigger (would be driven by player actions)
    // dread_state.progress(0.1); // This would be called by various game events
    
    // When dread level changes, update everything
    if dread_state.current_level != previous_level {
        info!("Dread level increased to: {} ({})", 
              dread_state.current_level, 
              dread_state.get_stage_name());
        
        // Corrupt world based on new dread level
        let corruption_radius = dread_state.current_level as f32 * 5.0;
        hex_world.corrupt_area(0, 0, corruption_radius, dread_state.current_level);
        
        // Adjust ambient lighting - darker as horror increases
        let brightness = 1.0 - (dread_state.current_level as f32 * 0.15);
        ambient_light.brightness = brightness.max(0.1);
        
        // Update tile materials based on corruption
        for (entity, tile, material_handle) in tile_query.iter() {
            if let Some(material) = materials.get_mut(material_handle) {
                let color = get_tile_color(&tile.tile_type, tile.dread_level);
                material.base_color = color;
                
                // Add emissive glow for corrupted tiles
                if tile.tile_type == TileType::Corrupted {
                    material.emissive = Color::srgb(0.3, 0.0, 0.8);
                }
            }
        }
        
        // Update companion trauma based on dread progression
        for (companion_type, state) in narrative_state.companion_states.iter_mut() {
            match dread_state.current_level {
                2 => if *companion_type == CompanionType::Mira {
                    *state = CompanionState::Abandoned; // Mira leaves in Dread stage
                },
                3 => *state = CompanionState::Traumatized,
                4 => if *state != CompanionState::Abandoned {
                    *state = CompanionState::Broken;
                },
                _ => {}
            }
        }
    }
}

// Companion trauma system following design bible
pub fn companion_trauma_system(
    dread_state: Res<DreadState>,
    mut companion_query: Query<&mut Companion>,
    mut narrative_state: ResMut<NarrativeState>,
) {
    for mut companion in companion_query.iter_mut() {
        // Increase trauma based on dread level
        let trauma_increase = dread_state.current_level as f32 * 0.01;
        companion.trauma_level = (companion.trauma_level + trauma_increase).min(1.0);
        
        // Update companion state based on trauma and type
        let new_state = match companion.companion_type {
            CompanionType::Einar => {
                if companion.trauma_level > 0.8 {
                    CompanionState::Broken // Einar breaks under pressure
                } else if companion.trauma_level > 0.5 {
                    CompanionState::Traumatized
                } else {
                    CompanionState::Normal
                }
            },
            CompanionType::Mira => {
                if dread_state.current_level >= 2 {
                    CompanionState::Abandoned // Leaves in Dread stage
                } else {
                    CompanionState::Normal
                }
            },
            CompanionType::Sorin => {
                if companion.trauma_level > 0.9 && companion.loyalty < 0.3 {
                    CompanionState::Hostile // Becomes traitor boss
                } else {
                    CompanionState::Normal
                }
            },
            CompanionType::Tamara => {
                if companion.trauma_level > 0.6 {
                    CompanionState::Traumatized // Innocent affected heavily
                } else {
                    CompanionState::Normal
                }
            },
        };
        
        if companion.current_state != new_state {
            companion.current_state = new_state.clone();
            narrative_state.companion_states.insert(companion.companion_type.clone(), new_state);
        }
    }
}

// World corruption system - visual transformation based on dread
pub fn world_corruption_system(
    dread_state: Res<DreadState>,
    mut tile_query: Query<(&mut HexTile, &Handle<StandardMaterial>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (mut tile, material_handle) in tile_query.iter_mut() {
        // Apply corruption based on dread level
        if tile.dread_level < dread_state.current_level {
            tile.dread_level = dread_state.current_level;
            
            // Transform tile types in higher dread levels
            if dread_state.current_level >= 3 {
                tile.tile_type = match tile.tile_type {
                    TileType::Grass => TileType::Corrupted,
                    TileType::Forest => TileType::Corrupted,
                    TileType::Village => TileType::Ruins,
                    _ => tile.tile_type.clone(),
                };
            }
            
            // Update visual appearance
            if let Some(material) = materials.get_mut(material_handle) {
                let color = get_tile_color(&tile.tile_type, tile.dread_level);
                material.base_color = color;
            }
        }
    }
}

// Hex interaction system for tap-to-move
pub fn hex_interaction_system(
    mouse_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut hex_world: ResMut<HexWorld>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        if let Ok(window) = windows.get_single() {
            if let Some(cursor_pos) = window.cursor_position() {
                if let Ok((camera, camera_transform)) = camera_query.get_single() {
                    // Convert cursor position to hex coordinates
                    // This would involve ray casting from camera through cursor position
                    // For now, simplified implementation
                    
                    let target_hex = screen_to_hex(cursor_pos, camera, camera_transform);
                    if let Some((q, r)) = target_hex {
                        hex_world.player_position = (q, r);
                        
                        // Move player entity to new position
                        if let Ok(mut player_transform) = player_query.get_single_mut() {
                            let world_pos = hex_to_world(q, r, 0.0);
                            player_transform.translation = world_pos;
                        }
                    }
                }
            }
        }
    }
}

// Camera follow system
pub fn camera_follow_system(
    hex_world: Res<HexWorld>,
    mut camera_query: Query<&mut Transform, (With<Camera3d>, Without<Player>)>,
) {
    if let Ok(mut camera_transform) = camera_query.get_single_mut() {
        let player_world_pos = hex_to_world(
            hex_world.player_position.0,
            hex_world.player_position.1,
            0.0
        );
        
        // Smooth camera follow with isometric offset
        let target_pos = player_world_pos + Vec3::new(10.0, 15.0, 10.0);
        camera_transform.translation = camera_transform.translation.lerp(target_pos, 0.05);
        camera_transform.look_at(player_world_pos, Vec3::Y);
    }
}

// Utility functions
fn get_tile_color(tile_type: &TileType, dread_level: u8) -> Color {
    let darkness_factor = 1.0 - (dread_level as f32 * 0.15);
    
    let base_color = match tile_type {
        TileType::Grass => Color::srgb(0.3, 0.8, 0.3),
        TileType::Forest => Color::srgb(0.1, 0.6, 0.1),
        TileType::Stone => Color::srgb(0.5, 0.5, 0.5),
        TileType::Water => Color::srgb(0.1, 0.3, 0.8),
        TileType::Village => Color::srgb(0.8, 0.6, 0.4),
        TileType::Ruins => Color::srgb(0.4, 0.3, 0.2),
        TileType::Corrupted => Color::srgb(0.5, 0.0, 0.5),
        TileType::Void => Color::srgb(0.1, 0.1, 0.1),
    };
    
    Color::srgb(
        base_color.r() * darkness_factor,
        base_color.g() * darkness_factor,
        base_color.b() * darkness_factor,
    )
}

fn hex_to_world(q: i32, r: i32, elevation: f32) -> Vec3 {
    let size = 1.5;
    let x = size * (3.0_f32.sqrt() * q as f32 + 3.0_f32.sqrt() / 2.0 * r as f32);
    let z = size * (3.0 / 2.0 * r as f32);
    Vec3::new(x, elevation, z)
}

fn screen_to_hex(cursor_pos: Vec2, camera: &Camera, camera_transform: &GlobalTransform) -> Option<(i32, i32)> {
    // Simplified hex coordinate conversion - would need proper ray casting
    // For now, return center hex
    Some((0, 0))
}