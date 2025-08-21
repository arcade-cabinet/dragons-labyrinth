// System implementations for Dragon's Labyrinth ECS architecture
use bevy::prelude::*;
use hexx::*;
use crate::components::*;
use crate::resources::*;

/// Handle hex-based movement input with modern Bevy input system
pub fn handle_hex_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut HexPosition), With<Player>>,
    hex_world: Res<HexWorld>,
    mut dread_state: ResMut<DreadState>,
) {
    if let Ok((mut transform, mut hex_pos)) = player_query.get_single_mut() {
        let mut new_hex = hex_pos.0;
        
        // Hexagonal movement using Q, W, E, A, S, D keys (6 directions)
        if keyboard.just_pressed(KeyCode::KeyQ) {
            new_hex = new_hex + Hex::new(-1, 0);  // Northwest
        }
        if keyboard.just_pressed(KeyCode::KeyW) {
            new_hex = new_hex + Hex::new(0, -1);  // North
        }
        if keyboard.just_pressed(KeyCode::KeyE) {
            new_hex = new_hex + Hex::new(1, -1);  // Northeast
        }
        if keyboard.just_pressed(KeyCode::KeyA) {
            new_hex = new_hex + Hex::new(-1, 1);  // Southwest
        }
        if keyboard.just_pressed(KeyCode::KeyS) {
            new_hex = new_hex + Hex::new(0, 1);   // South
        }
        if keyboard.just_pressed(KeyCode::KeyD) {
            new_hex = new_hex + Hex::new(1, 0);   // Southeast
        }
        
        // Check if movement is valid
        if new_hex != hex_pos.0 {
            if let Some(tile) = hex_world.get_tile(new_hex) {
                if tile.passable {
                    hex_pos.0 = new_hex;
                    
                    // Convert hex to world position
                    let layout = HexLayout {
                        orientation: HexOrientation::Flat,
                        origin: Vec2::ZERO,
                        hex_size: Vec2::splat(1.0),
                        invert_x: false,
                        invert_y: false,
                    };
                    let world_pos = layout.hex_to_world_pos(new_hex);
                    transform.translation.x = world_pos.x;
                    transform.translation.z = world_pos.y;
                    
                    // Movement has chance to trigger dread progression
                    if tile.corruption > 0.5 {
                        dread_state.increase_dread(0.05);
                    }
                    
                    info!("Player moved to hex {:?}", new_hex);
                }
            }
        }
    }
}

/// Update dread progression over time and based on events
pub fn update_dread_progression(
    mut dread_state: ResMut<DreadState>,
    time: Res<Time>,
    companion_state: Res<CompanionState>,
    narrative_state: Res<NarrativeState>,
) {
    // Slow automatic dread progression
    let base_progression = 0.001 * time.delta_seconds();
    
    // Accelerate based on low companion morale
    let morale_factor = if companion_state.get_group_sanity() < 30.0 { 2.0 } else { 1.0 };
    
    // Accelerate based on moral choices
    let moral_factor = 1.0 + (narrative_state.player_reputation.brutality_score * 0.5);
    
    let total_progression = base_progression * morale_factor * moral_factor;
    dread_state.increase_dread(total_progression);
    
    // Log dread level changes
    let previous_level = dread_state.level;
    if previous_level != dread_state.level {
        info!("Dread level increased to: {} ({})", dread_state.level, dread_state.get_stage_name());
    }
}

/// Apply world corruption effects based on dread state
pub fn apply_world_corruption(
    dread_state: Res<DreadState>,
    mut hex_world: ResMut<HexWorld>,
    mut tile_query: Query<(&HexTile, &mut Handle<StandardMaterial>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if !dread_state.is_changed() {
        return;
    }
    
    let corruption_intensity = dread_state.get_corruption_intensity();
    
    // Spread corruption from epicenter if it exists
    if let Some(epicenter) = hex_world.corruption_epicenter {
        let radius = 5.0 + (dread_state.level as f32 * 3.0);
        hex_world.spread_corruption(epicenter, radius, corruption_intensity * 0.1);
    }
    
    // Update tile visuals based on corruption
    for (tile, material_handle) in tile_query.iter_mut() {
        if let Some(material) = materials.get_mut(&*material_handle) {
            material.base_color = get_corrupted_tile_color(tile, corruption_intensity);
        }
    }
}

/// Update companion trauma and behavior based on dread level
pub fn update_companion_trauma(
    mut companion_query: Query<&mut Companion>,
    mut companion_state: ResMut<CompanionState>,
    dread_state: Res<DreadState>,
    time: Res<Time>,
) {
    let trauma_rate = match dread_state.level {
        0 => 0.0,
        1 => 0.5,
        2 => 2.0,
        3 => 5.0,
        4 => 10.0,
        _ => 0.0,
    };
    
    for mut companion in companion_query.iter_mut() {
        // Apply trauma based on dread level
        companion.trauma_level += trauma_rate * time.delta_seconds();
        
        // Reduce sanity based on trauma
        companion.sanity -= companion.trauma_level * 0.1 * time.delta_seconds();
        companion.sanity = companion.sanity.max(0.0);
        
        // Update companion state resource
        companion_state.companion_morale.insert(
            companion.name.clone(),
            companion.sanity
        );
        
        // Handle companion-specific reactions
        match companion.companion_type {
            CompanionType::Mira => {
                // Mira flees in Dread stage
                if dread_state.level >= 2 && companion.sanity < 30.0 {
                    info!("{} has fled due to overwhelming dread!", companion.name);
                    // Mark for removal or fleeing
                }
            },
            CompanionType::Sorin => {
                // Sorin becomes corrupted at high trauma
                if companion.trauma_level > 80.0 {
                    info!("{} has been corrupted by forbidden knowledge!", companion.name);
                    // Transform into boss encounter
                }
            },
            CompanionType::Einar => {
                // Einar's loyalty decreases under extreme stress
                if dread_state.level >= 3 {
                    companion.loyalty -= 5.0 * time.delta_seconds();
                    companion.loyalty = companion.loyalty.max(0.0);
                }
            },
            CompanionType::Tamara => {
                // Tamara disappears in Terror stage
                if dread_state.level >= 3 && companion.sanity < 10.0 {
                    info!("{} has vanished without a trace...", companion.name);
                    // Mark for removal
                }
            },
        }
    }
    
    // Update group cohesion
    companion_state.group_cohesion = companion_state.get_group_sanity() / 100.0;
}

/// Process narrative events and choices
pub fn process_narrative_events(
    mut narrative_state: ResMut<NarrativeState>,
    dread_state: Res<DreadState>,
    companion_state: Res<CompanionState>,
    mut quest_query: Query<&mut Quest>,
) {
    // Update quest availability based on dread level
    for mut quest in quest_query.iter_mut() {
        if quest.completion_status == QuestStatus::Available {
            if dread_state.level >= quest.required_dread_level {
                quest.completion_status = QuestStatus::Active;
                narrative_state.active_quests.push(quest.id.clone());
                info!("Quest '{}' is now available", quest.title);
            }
        }
    }
    
    // Check for automatic story progression
    match dread_state.level {
        1 => {
            // Unease stage - introduce Hollow Caretaker
            if !narrative_state.completed_quests.contains(&"hollow_caretaker_intro".to_string()) {
                info!("The Hollow Caretaker stirs in the shadows...");
            }
        },
        2 => {
            // Dread stage - economic collapse, Forsaken Knight
            if companion_state.group_cohesion < 0.5 {
                info!("The group's unity begins to fracture under the weight of dread...");
            }
        },
        3 => {
            // Terror stage - reality warps, moral horrors
            info!("Reality itself begins to bend and break...");
        },
        4 => {
            // Horror stage - Dragon's labyrinth
            if narrative_state.unlocked_endings.is_empty() {
                narrative_state.unlocked_endings.extend([
                    "acceptance".to_string(),
                    "defiance".to_string(),
                    "understanding".to_string(),
                ]);
                info!("The Dragon's presence is overwhelming. Three paths lie before you...");
            }
        },
        _ => {}
    }
}

/// Update lighting and atmosphere based on dread level
pub fn update_lighting_based_on_dread(
    dread_state: Res<DreadState>,
    mut ambient_light: ResMut<AmbientLight>,
    mut directional_light_query: Query<&mut DirectionalLight>,
    mut audio_state: ResMut<AudioState>,
) {
    if !dread_state.is_changed() {
        return;
    }
    
    let dread_factor = dread_state.level as f32 / 4.0;
    
    // Dim ambient light as dread increases
    ambient_light.brightness = 500.0 * (1.0 - dread_factor * 0.7);
    
    // Shift ambient light color toward red as dread increases
    let red_shift = dread_factor * 0.3;
    ambient_light.color = Color::srgb(
        0.8 + red_shift,
        0.8 - red_shift,
        0.8 - red_shift,
    );
    
    // Adjust directional light
    for mut light in directional_light_query.iter_mut() {
        light.illuminance = 10000.0 * (1.0 - dread_factor * 0.5);
        
        // Add flickering at high dread levels
        if dread_state.level >= 3 {
            let flicker = (dread_state.progress * 10.0).sin() * 0.1;
            light.illuminance *= 1.0 + flicker;
        }
    }
    
    // Update audio state
    audio_state.update_for_dread_level(dread_state.level);
    
    info!("Atmosphere updated for dread level: {} ({})", 
          dread_state.level, dread_state.get_stage_name());
}

/// Helper function to get corrupted tile colors
fn get_corrupted_tile_color(tile: &HexTile, corruption_intensity: f32) -> Color {
    let base_color = match tile.tile_type {
        TileType::Grass => Color::srgb(0.3, 0.8, 0.3),
        TileType::Forest => Color::srgb(0.1, 0.5, 0.1),
        TileType::Swamp => Color::srgb(0.2, 0.4, 0.1),
        TileType::Stone => Color::srgb(0.5, 0.5, 0.5),
        TileType::Corrupted => Color::srgb(0.5, 0.1, 0.1),
    };
    
    // Apply corruption overlay
    let corruption_factor = tile.corruption * corruption_intensity;
    if corruption_factor > 0.0 {
        let corruption_color = Color::srgb(0.4, 0.1, 0.1);
        base_color.mix(&corruption_color, corruption_factor)
    } else {
        base_color
    }
}