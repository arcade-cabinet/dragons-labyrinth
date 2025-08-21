use bevy::prelude::*;
use dragons_core::{components::*, resources::*};
use hexx::Hex;
use rand::Rng;

/// Plugin for turn-based combat system
pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<CombatState>()
            .init_resource::<CombatManager>()
            .init_resource::<CombatLog>()
            .add_event::<CombatEvent>()
            .add_event::<DamageEvent>()
            .add_systems(OnEnter(CombatState::Idle), cleanup_combat)
            .add_systems(OnEnter(CombatState::Starting), initialize_combat)
            .add_systems(OnEnter(CombatState::PlayerTurn), start_player_turn)
            .add_systems(OnEnter(CombatState::EnemyTurn), start_enemy_turn)
            .add_systems(OnEnter(CombatState::Victory), handle_victory)
            .add_systems(OnEnter(CombatState::Defeat), handle_defeat)
            .add_systems(Update, (
                process_combat_events,
                handle_player_combat_input.run_if(in_state(CombatState::PlayerTurn)),
                process_enemy_turns.run_if(in_state(CombatState::EnemyTurn)),
                update_combat_ui,
                check_combat_end,
            ));
    }
}

/// Combat state machine
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum CombatState {
    #[default]
    Idle,       // Not in combat
    Starting,   // Combat initializing
    PlayerTurn, // Player's turn
    EnemyTurn,  // Enemy turns
    Victory,    // Player won
    Defeat,     // Player lost
}

/// Combat manager resource
#[derive(Resource, Default)]
pub struct CombatManager {
    pub participants: Vec<Entity>,
    pub turn_order: Vec<Entity>,
    pub current_turn_index: usize,
    pub round: u32,
    pub player_actions_remaining: u32,
}

/// Combat log for displaying messages
#[derive(Resource, Default)]
pub struct CombatLog {
    pub messages: Vec<String>,
    pub max_messages: usize,
}

impl CombatLog {
    pub fn add(&mut self, message: String) {
        self.messages.push(message);
        if self.messages.len() > self.max_messages {
            self.messages.remove(0);
        }
        info!("Combat: {}", self.messages.last().unwrap());
    }
}

/// Combat participant component
#[derive(Component)]
pub struct CombatParticipant {
    pub initiative: f32,
    pub attack_range: f32,
    pub damage_dice: (u32, u32), // (number of dice, dice sides)
    pub armor_class: u32,
    pub is_enemy: bool,
}

/// Health component
#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Health {
    pub fn take_damage(&mut self, amount: f32) {
        self.current = (self.current - amount).max(0.0);
    }
    
    pub fn is_dead(&self) -> bool {
        self.current <= 0.0
    }
    
    pub fn percentage(&self) -> f32 {
        self.current / self.max
    }
}

/// Combat events
#[derive(Event)]
pub enum CombatEvent {
    StartCombat { enemies: Vec<Entity> },
    Attack { attacker: Entity, target: Entity },
    EndTurn { entity: Entity },
    Flee { entity: Entity },
}

/// Damage event for visual feedback
#[derive(Event)]
pub struct DamageEvent {
    pub entity: Entity,
    pub amount: f32,
    pub attacker: Entity,
}

/// Initialize combat when triggered
fn initialize_combat(
    mut combat_manager: ResMut<CombatManager>,
    mut combat_log: ResMut<CombatLog>,
    mut next_state: ResMut<NextState<CombatState>>,
    query: Query<(Entity, &CombatParticipant)>,
) {
    combat_log.messages.clear();
    combat_log.add("Combat begins!".to_string());
    
    // Roll initiative for all participants
    let mut participants_with_initiative: Vec<(Entity, f32)> = query
        .iter()
        .map(|(entity, participant)| {
            let roll = rand::thread_rng().gen_range(1..=20) as f32;
            (entity, roll + participant.initiative)
        })
        .collect();
    
    // Sort by initiative (highest first)
    participants_with_initiative.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    
    combat_manager.turn_order = participants_with_initiative
        .iter()
        .map(|(entity, _)| *entity)
        .collect();
    
    combat_manager.participants = combat_manager.turn_order.clone();
    combat_manager.current_turn_index = 0;
    combat_manager.round = 1;
    
    // Start first turn
    if !combat_manager.turn_order.is_empty() {
        let first_entity = combat_manager.turn_order[0];
        if let Ok((_, participant)) = query.get(first_entity) {
            if participant.is_enemy {
                next_state.set(CombatState::EnemyTurn);
            } else {
                next_state.set(CombatState::PlayerTurn);
            }
        }
    }
}

/// Handle player turn start
fn start_player_turn(
    mut combat_manager: ResMut<CombatManager>,
    mut combat_log: ResMut<CombatLog>,
) {
    combat_manager.player_actions_remaining = 2; // Move + Action
    combat_log.add("Your turn! Press 1-9 for abilities, Space to end turn".to_string());
}

/// Handle player combat input
fn handle_player_combat_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut combat_events: EventWriter<CombatEvent>,
    mut combat_manager: ResMut<CombatManager>,
    mut next_state: ResMut<NextState<CombatState>>,
    player_query: Query<Entity, With<Player>>,
    enemy_query: Query<(Entity, &Transform, &Health), (With<CombatParticipant>, Without<Player>)>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    // End turn with Space
    if keyboard.just_pressed(KeyCode::Space) {
        if let Ok(player_entity) = player_query.get_single() {
            combat_events.send(CombatEvent::EndTurn { entity: player_entity });
            next_state.set(CombatState::EnemyTurn);
        }
    }
    
    // Attack with left click
    if mouse_button.just_pressed(MouseButton::Left) && combat_manager.player_actions_remaining > 0 {
        // Get clicked target
        let Ok(window) = windows.get_single() else { return };
        let Some(cursor_position) = window.cursor_position() else { return };
        let Ok((camera, camera_transform)) = camera_query.get_single() else { return };
        
        // Cast ray to find target
        if let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) {
            let t = -ray.origin.y / ray.direction.y;
            if t > 0.0 {
                let world_pos = ray.origin + ray.direction * t;
                
                // Find closest enemy to click position
                let mut closest_enemy = None;
                let mut closest_distance = f32::MAX;
                
                for (entity, transform, health) in enemy_query.iter() {
                    if !health.is_dead() {
                        let distance = transform.translation.distance(world_pos);
                        if distance < 2.0 && distance < closest_distance {
                            closest_distance = distance;
                            closest_enemy = Some(entity);
                        }
                    }
                }
                
                if let Some(target) = closest_enemy {
                    if let Ok(player_entity) = player_query.get_single() {
                        combat_events.send(CombatEvent::Attack {
                            attacker: player_entity,
                            target,
                        });
                        combat_manager.player_actions_remaining -= 1;
                    }
                }
            }
        }
    }
    
    // Ability hotkeys 1-9
    for key in 1..=9 {
        let keycode = match key {
            1 => KeyCode::Digit1,
            2 => KeyCode::Digit2,
            3 => KeyCode::Digit3,
            4 => KeyCode::Digit4,
            5 => KeyCode::Digit5,
            6 => KeyCode::Digit6,
            7 => KeyCode::Digit7,
            8 => KeyCode::Digit8,
            9 => KeyCode::Digit9,
            _ => continue,
        };
        
        if keyboard.just_pressed(keycode) {
            info!("Ability {} activated (not yet implemented)", key);
        }
    }
}

/// Handle enemy turn start
fn start_enemy_turn(
    mut combat_log: ResMut<CombatLog>,
    combat_manager: Res<CombatManager>,
    query: Query<&Name, With<CombatParticipant>>,
) {
    if let Some(current_entity) = combat_manager.turn_order.get(combat_manager.current_turn_index) {
        if let Ok(name) = query.get(*current_entity) {
            combat_log.add(format!("{}'s turn", name.as_str()));
        }
    }
}

/// Process enemy AI turns
fn process_enemy_turns(
    mut combat_events: EventWriter<CombatEvent>,
    mut next_state: ResMut<NextState<CombatState>>,
    mut combat_manager: ResMut<CombatManager>,
    enemy_query: Query<(Entity, &Health, &HexPosition), (With<CombatParticipant>, Without<Player>)>,
    player_query: Query<(Entity, &HexPosition), With<Player>>,
    time: Res<Time>,
) {
    // Simple AI: attack player if in range
    if let Some(current_entity) = combat_manager.turn_order.get(combat_manager.current_turn_index) {
        if let Ok((enemy_entity, health, enemy_pos)) = enemy_query.get(*current_entity) {
            if !health.is_dead() {
                if let Ok((player_entity, player_pos)) = player_query.get_single() {
                    let distance = enemy_pos.0.unsigned_distance_to(player_pos.0);
                    
                    // Attack if in range (melee range = 1 hex)
                    if distance <= 1 {
                        combat_events.send(CombatEvent::Attack {
                            attacker: enemy_entity,
                            target: player_entity,
                        });
                    }
                }
            }
            
            // End enemy turn after a short delay
            combat_events.send(CombatEvent::EndTurn { entity: enemy_entity });
            
            // Move to next turn
            combat_manager.current_turn_index += 1;
            if combat_manager.current_turn_index >= combat_manager.turn_order.len() {
                combat_manager.current_turn_index = 0;
                combat_manager.round += 1;
            }
            
            // Check whose turn is next
            if let Some(next_entity) = combat_manager.turn_order.get(combat_manager.current_turn_index) {
                if let Ok((_, _, _)) = enemy_query.get(*next_entity) {
                    // Another enemy's turn
                } else {
                    // Player's turn
                    next_state.set(CombatState::PlayerTurn);
                }
            }
        }
    }
}

/// Process combat events
fn process_combat_events(
    mut combat_events: EventReader<CombatEvent>,
    mut damage_events: EventWriter<DamageEvent>,
    mut combat_log: ResMut<CombatLog>,
    mut health_query: Query<&mut Health>,
    participant_query: Query<&CombatParticipant>,
    name_query: Query<&Name>,
) {
    for event in combat_events.read() {
        match event {
            CombatEvent::Attack { attacker, target } => {
                if let (Ok(attacker_participant), Ok(mut target_health)) = 
                    (participant_query.get(*attacker), health_query.get_mut(*target)) {
                    
                    // Roll to hit (d20 + modifiers vs AC)
                    let hit_roll = rand::thread_rng().gen_range(1..=20);
                    let total_hit = hit_roll + 5; // +5 modifier for now
                    
                    let target_ac = participant_query.get(*target)
                        .map(|p| p.armor_class)
                        .unwrap_or(10);
                    
                    let attacker_name = name_query.get(*attacker)
                        .map(|n| n.as_str())
                        .unwrap_or("Attacker");
                    let target_name = name_query.get(*target)
                        .map(|n| n.as_str())
                        .unwrap_or("Target");
                    
                    if total_hit >= target_ac {
                        // Roll damage
                        let mut damage = 0.0;
                        for _ in 0..attacker_participant.damage_dice.0 {
                            damage += rand::thread_rng().gen_range(1..=attacker_participant.damage_dice.1) as f32;
                        }
                        
                        target_health.take_damage(damage);
                        combat_log.add(format!("{} hits {} for {} damage!", attacker_name, target_name, damage));
                        
                        damage_events.send(DamageEvent {
                            entity: *target,
                            amount: damage,
                            attacker: *attacker,
                        });
                    } else {
                        combat_log.add(format!("{} misses {}!", attacker_name, target_name));
                    }
                }
            }
            _ => {}
        }
    }
}

/// Check if combat should end
fn check_combat_end(
    mut next_state: ResMut<NextState<CombatState>>,
    mut combat_log: ResMut<CombatLog>,
    player_query: Query<&Health, With<Player>>,
    enemy_query: Query<&Health, (With<CombatParticipant>, Without<Player>)>,
    current_state: Res<State<CombatState>>,
) {
    if *current_state.get() == CombatState::Idle {
        return;
    }
    
    // Check player death
    if let Ok(player_health) = player_query.get_single() {
        if player_health.is_dead() {
            combat_log.add("You have been defeated!".to_string());
            next_state.set(CombatState::Defeat);
            return;
        }
    }
    
    // Check all enemies defeated
    let any_enemy_alive = enemy_query.iter().any(|health| !health.is_dead());
    if !any_enemy_alive {
        combat_log.add("Victory! All enemies defeated!".to_string());
        next_state.set(CombatState::Victory);
    }
}

/// Handle victory
fn handle_victory(
    mut next_state: ResMut<NextState<CombatState>>,
    mut combat_log: ResMut<CombatLog>,
) {
    combat_log.add("You gained experience!".to_string());
    // TODO: Award loot, experience, etc.
    next_state.set(CombatState::Idle);
}

/// Handle defeat
fn handle_defeat(
    mut next_state: ResMut<NextState<CombatState>>,
    mut game_state: ResMut<NextState<crate::states::GameState>>,
) {
    // Transition to game over
    game_state.set(crate::states::GameState::GameOver);
    next_state.set(CombatState::Idle);
}

/// Cleanup after combat
fn cleanup_combat(
    mut combat_manager: ResMut<CombatManager>,
) {
    combat_manager.participants.clear();
    combat_manager.turn_order.clear();
    combat_manager.current_turn_index = 0;
    combat_manager.round = 0;
}

/// Update combat UI
fn update_combat_ui(
    combat_log: Res<CombatLog>,
) {
    // TODO: Update actual UI elements
    // For now just ensure messages are logged
}