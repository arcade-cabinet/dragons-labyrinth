//! Combat Systems - ECS systems for D&D 5e combat mechanics

use bevy::prelude::*;
use sea_orm::DatabaseConnection;
use rand::Rng;
use super::components::*;
use super::resources::*;
use super::events::*;
use crate::bevy_integration::DatabaseQuery;

/// Initialize combat when encounter begins
pub fn combat_initiation_system(
    mut commands: Commands,
    mut combat_events: EventReader<CombatInitiatedEvent>,
    mut combat_state: ResMut<CombatState>,
    creature_cache: Res<CreatureTemplateCache>,
) {
    for event in combat_events.read() {
        info!("Initiating combat at ({}, {})", event.position.q, event.position.r);
        
        // Spawn player entity if not already present
        if combat_state.player_entity.is_none() {
            let player_entity = commands.spawn((
                Name::new("Player"),
                CombatParticipant {
                    participant_type: ParticipantType::Player,
                    team: CombatTeam::PlayerTeam,
                },
                CreatureStats {
                    current_hp: event.player_hp,
                    max_hp: event.player_max_hp,
                    armor_class: event.player_ac,
                    strength: 14,
                    dexterity: 14,
                    constitution: 14,
                    intelligence: 12,
                    wisdom: 13,
                    charisma: 12,
                },
                CombatPosition {
                    q: 0,
                    r: 0,
                    facing: CombatFacing::North,
                },
                TurnOrder {
                    initiative: roll_initiative(14), // Using dex mod
                    turn_taken: false,
                    is_player_controlled: true,
                },
                CombatActions {
                    actions: create_player_actions(),
                    used_actions: Vec::new(),
                },
                StatusEffects {
                    effects: Vec::new(),
                },
            )).id();
            
            combat_state.player_entity = Some(player_entity);
        }
        
        // Spawn creatures from encounter
        for (i, creature_name) in event.creature_names.iter().enumerate() {
            if let Some(template) = creature_cache.templates.get(creature_name) {
                let creature_entity = commands.spawn((
                    Name::new(format!("{} #{}", creature_name, i + 1)),
                    CombatCreature {
                        creature_id: Uuid::new_v4(),
                        name: creature_name.clone(),
                        creature_type: template.creature_type.clone(),
                        challenge_rating: template.challenge_rating.clone(),
                    },
                    CombatParticipant {
                        participant_type: ParticipantType::Enemy,
                        team: CombatTeam::EnemyTeam,
                    },
                    CreatureStats {
                        current_hp: roll_hp(&template.hit_points_formula),
                        max_hp: roll_hp(&template.hit_points_formula),
                        armor_class: template.armor_class,
                        strength: template.abilities.strength,
                        dexterity: template.abilities.dexterity,
                        constitution: template.abilities.constitution,
                        intelligence: template.abilities.intelligence,
                        wisdom: template.abilities.wisdom,
                        charisma: template.abilities.charisma,
                    },
                    CombatPosition {
                        q: (i as i32 + 2),
                        r: 0,
                        facing: CombatFacing::South,
                    },
                    TurnOrder {
                        initiative: roll_initiative(template.abilities.dexterity),
                        turn_taken: false,
                        is_player_controlled: false,
                    },
                    CombatActions {
                        actions: template.actions.clone(),
                        used_actions: Vec::new(),
                    },
                    StatusEffects {
                        effects: Vec::new(),
                    },
                    CombatAI {
                        behavior_type: determine_ai_behavior(&template.creature_type),
                        target_priority: vec![ParticipantType::Player, ParticipantType::Companion],
                        preferred_range: determine_preferred_range(&template.actions),
                        morale: 50 + template.abilities.charisma,
                    },
                )).id();
                
                combat_state.creature_entities.push(creature_entity);
            }
        }
        
        combat_state.phase = CombatPhase::Initiative;
        combat_state.current_turn = 0;
        
        info!("Combat initiated with {} creatures", event.creature_names.len());
    }
}

/// Manage turn order and initiative
pub fn turn_order_system(
    mut combat_state: ResMut<CombatState>,
    turn_query: Query<(Entity, &TurnOrder)>,
    mut next_phase: ResMut<NextState<CombatPhase>>,
) {
    if combat_state.phase != CombatPhase::Initiative {
        return;
    }
    
    // Collect all entities with turn order
    let mut turn_entities: Vec<(Entity, i32)> = turn_query
        .iter()
        .map(|(entity, turn_order)| (entity, turn_order.initiative))
        .collect();
    
    // Sort by initiative (highest first)
    turn_entities.sort_by(|a, b| b.1.cmp(&a.1));
    
    // Set turn order
    combat_state.turn_order = turn_entities.into_iter().map(|(entity, _)| entity).collect();
    
    if !combat_state.turn_order.is_empty() {
        next_phase.set(CombatPhase::PlayerTurn);
        info!("Turn order established: {} participants", combat_state.turn_order.len());
    }
}

/// Handle attack resolution
pub fn attack_resolution_system(
    mut attack_events: EventReader<AttackEvent>,
    mut damage_events: EventWriter<DamageEvent>,
    query: Query<&CreatureStats>,
) {
    for attack_event in attack_events.read() {
        if let (Ok(attacker_stats), Ok(target_stats)) = (
            query.get(attack_event.attacker),
            query.get(attack_event.target)
        ) {
            // Roll attack
            let attack_roll = rand::thread_rng().gen_range(1..=20);
            let total_attack = attack_roll + attack_event.attack_bonus;
            let is_critical = attack_roll == 20;
            let hits = total_attack >= target_stats.armor_class || is_critical;
            
            if hits {
                let damage = if let Some(formula) = &attack_event.damage_formula {
                    let base_damage = parse_damage_formula(formula);
                    if is_critical { base_damage * 2 } else { base_damage }
                } else {
                    1
                };
                
                damage_events.send(DamageEvent {
                    target: attack_event.target,
                    damage,
                    damage_type: attack_event.damage_type.clone(),
                    source: attack_event.attacker,
                    is_critical,
                });
                
                info!("Attack hits! Damage: {} (critical: {})", damage, is_critical);
            } else {
                info!("Attack misses! ({} vs AC {})", total_attack, target_stats.armor_class);
            }
        }
    }
}

/// Apply damage and check for creature death
pub fn damage_application_system(
    mut damage_events: EventReader<DamageEvent>,
    mut defeat_events: EventWriter<CreatureDefeatedEvent>,
    mut query: Query<&mut CreatureStats>,
    names: Query<&Name>,
    participants: Query<&CombatParticipant>,
) {
    for damage_event in damage_events.read() {
        if let Ok(mut stats) = query.get_mut(damage_event.target) {
            // Apply damage
            stats.current_hp = (stats.current_hp - damage_event.damage).max(0);
            
            if let Ok(name) = names.get(damage_event.target) {
                info!("{} takes {} damage ({}HP remaining)", 
                      name.as_str(), damage_event.damage, stats.current_hp);
            }
            
            // Check for defeat
            if stats.current_hp <= 0 {
                if let Ok(participant) = participants.get(damage_event.target) {
                    defeat_events.send(CreatureDefeatedEvent {
                        entity: damage_event.target,
                        participant_type: participant.participant_type.clone(),
                        defeated_by: damage_event.source,
                    });
                }
            }
        }
    }
}

/// Handle creature AI behavior
pub fn creature_ai_system(
    mut attack_events: EventWriter<AttackEvent>,
    combat_state: Res<CombatState>,
    ai_query: Query<(Entity, &CombatAI, &CombatActions, &CombatPosition), With<CombatParticipant>>,
    target_query: Query<(Entity, &CombatPosition, &CombatParticipant)>,
) {
    if combat_state.phase != CombatPhase::CreatureTurn {
        return;
    }
    
    for (entity, ai, actions, position) in ai_query.iter() {
        // Find targets based on AI behavior
        let targets: Vec<(Entity, &CombatPosition)> = target_query
            .iter()
            .filter(|(_, _, participant)| {
                ai.target_priority.contains(&participant.participant_type) &&
                participant.team != CombatTeam::EnemyTeam
            })
            .map(|(entity, pos, _)| (entity, pos))
            .collect();
        
        if let Some((target_entity, _target_pos)) = targets.first() {
            // Select best action based on AI behavior
            if let Some(action) = select_ai_action(ai, actions) {
                attack_events.send(AttackEvent {
                    attacker: entity,
                    target: *target_entity,
                    action_name: action.name.clone(),
                    attack_bonus: action.attack_bonus.unwrap_or(0),
                    damage_formula: action.damage_formula.clone(),
                    damage_type: DamageType::Slashing, // Default, would be parsed from action
                });
            }
        }
    }
}

/// Process status effects each turn
pub fn status_effect_system(
    mut query: Query<&mut StatusEffects>,
    mut damage_events: EventWriter<DamageEvent>,
) {
    for mut status_effects in query.iter_mut() {
        status_effects.effects.retain_mut(|effect| {
            // Process effect
            match effect.effect_type {
                StatusEffectType::Poisoned => {
                    // Poison damage each turn
                    damage_events.send(DamageEvent {
                        target: Entity::PLACEHOLDER, // Would need entity reference
                        damage: 1,
                        damage_type: DamageType::Poison,
                        source: Entity::PLACEHOLDER,
                        is_critical: false,
                    });
                }
                _ => {} // Other effects handled elsewhere
            }
            
            // Reduce duration
            effect.duration -= 1;
            effect.duration > 0
        });
    }
}

/// Check for combat end conditions
pub fn combat_resolution_system(
    mut combat_state: ResMut<CombatState>,
    mut next_state: ResMut<NextState<CombatPhase>>,
    mut combat_end_events: EventWriter<CombatEndedEvent>,
    participants: Query<&CombatParticipant>,
    stats: Query<&CreatureStats>,
) {
    let mut player_team_alive = false;
    let mut enemy_team_alive = false;
    
    for participant in participants.iter() {
        if let Ok(creature_stats) = stats.get_single() { // This would need proper entity lookup
            if creature_stats.is_alive() {
                match participant.team {
                    CombatTeam::PlayerTeam => player_team_alive = true,
                    CombatTeam::EnemyTeam => enemy_team_alive = true,
                    _ => {}
                }
            }
        }
    }
    
    // Check end conditions
    if !player_team_alive {
        combat_end_events.send(CombatEndedEvent {
            result: CombatResult::Defeat,
            rounds_lasted: combat_state.current_turn,
        });
        next_state.set(CombatPhase::Cleanup);
    } else if !enemy_team_alive {
        combat_end_events.send(CombatEndedEvent {
            result: CombatResult::Victory,
            rounds_lasted: combat_state.current_turn,
        });
        next_state.set(CombatPhase::Cleanup);
    }
}

/// Apply weather effects to combat
pub fn weather_combat_effects_system(
    combat_state: Res<CombatState>,
    mut query: Query<&mut CreatureStats>,
) {
    if let Some(weather_effect) = &combat_state.environment.weather_effects {
        for effect in &weather_effect.combat_effects {
            match effect.as_str() {
                "slippery_ground" => {
                    // Reduce movement speed
                    debug!("Slippery ground affects movement");
                }
                "lightning_risk" => {
                    // Random lightning damage
                    if rand::thread_rng().gen::<f32>() < 0.1 {
                        debug!("Lightning strikes during combat!");
                    }
                }
                _ => {}
            }
        }
    }
}

/// Apply terrain effects to combat positioning
pub fn terrain_combat_effects_system(
    combat_state: Res<CombatState>,
    query: Query<&CombatPosition>,
) {
    if let Some(environment) = &combat_state.environment {
        // Apply terrain movement costs and cover bonuses
        for position in query.iter() {
            let movement_cost = calculate_terrain_movement_cost(&environment.terrain);
            debug!("Position ({}, {}) has movement cost: {}", 
                   position.q, position.r, movement_cost);
        }
    }
}

/// Apply corruption effects during combat
pub fn corruption_combat_effects_system(
    combat_state: Res<CombatState>,
    mut query: Query<&mut StatusEffects>,
) {
    if let Some(environment) = &combat_state.environment {
        if environment.corruption_level > 0.5 {
            // High corruption causes fear effects
            for mut status_effects in query.iter_mut() {
                if rand::thread_rng().gen::<f32>() < 0.1 {
                    status_effects.effects.push(StatusEffect {
                        effect_type: StatusEffectType::Frightened,
                        duration: 3,
                        source: "corruption".to_string(),
                        save_ends: Some(("wisdom".to_string(), 12)),
                    });
                }
            }
        }
    }
}

/// Tactical positioning for creatures
pub fn tactical_positioning_system(
    mut query: Query<(&mut CombatPosition, &CombatAI, &CombatActions)>,
    target_query: Query<&CombatPosition, (With<CombatParticipant>, Without<CombatAI>)>,
) {
    for (mut position, ai, actions) in query.iter_mut() {
        match ai.preferred_range {
            CombatRange::Melee => {
                // Move towards nearest target
                if let Some(target_pos) = find_nearest_target(&position, &target_query) {
                    move_towards_target(&mut position, &target_pos);
                }
            }
            CombatRange::Ranged => {
                // Maintain distance from targets
                if let Some(target_pos) = find_nearest_target(&position, &target_query) {
                    move_away_from_target(&mut position, &target_pos);
                }
            }
            _ => {} // Other positioning strategies
        }
    }
}

/// Update companion behavior during combat
pub fn companion_combat_system(
    mut query: Query<(&mut CombatAI, &CombatParticipant)>,
    combat_state: Res<CombatState>,
) {
    // Companions adjust behavior based on player status and combat situation
    for (mut ai, participant) in query.iter_mut() {
        if participant.participant_type == ParticipantType::Companion {
            // Adjust companion behavior based on combat stress
            if combat_state.current_turn > 5 {
                ai.morale = (ai.morale - 5).max(0);
            }
            
            // Companions become more protective if player is injured
            if let Some(player_hp_percent) = combat_state.player_hp_percentage {
                if player_hp_percent < 0.5 {
                    ai.behavior_type = AIBehaviorType::Protective;
                }
            }
        }
    }
}

/// Apply horror progression effects during combat
pub fn horror_progression_combat_system(
    mut query: Query<&mut StatusEffects>,
    combat_state: Res<CombatState>,
) {
    // Long combats increase horror/corruption effects
    if combat_state.current_turn > 10 {
        for mut status_effects in query.iter_mut() {
            if rand::thread_rng().gen::<f32>() < 0.05 {
                status_effects.effects.push(StatusEffect {
                    effect_type: StatusEffectType::Despairing,
                    duration: 5,
                    source: "prolonged_combat".to_string(),
                    save_ends: Some(("charisma".to_string(), 15)),
                });
            }
        }
    }
}

/// Clean up combat entities when combat ends
pub fn combat_cleanup_system(
    mut commands: Commands,
    mut combat_state: ResMut<CombatState>,
    mut combat_end_events: EventReader<CombatEndedEvent>,
    query: Query<Entity, With<CombatParticipant>>,
) {
    for _event in combat_end_events.read() {
        // Despawn all combat entities
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
        
        // Reset combat state
        combat_state.phase = CombatPhase::None;
        combat_state.player_entity = None;
        combat_state.creature_entities.clear();
        combat_state.turn_order.clear();
        combat_state.current_turn = 0;
        
        info!("Combat cleanup completed");
    }
}

/// Helper functions

fn roll_initiative(dexterity: i32) -> i32 {
    let dex_mod = CreatureStats::ability_modifier(dexterity);
    rand::thread_rng().gen_range(1..=20) + dex_mod
}

fn roll_hp(formula: &str) -> i32 {
    parse_damage_formula(formula)
}

fn parse_damage_formula(formula: &str) -> i32 {
    // Parse dice formulas like "2d8+2"
    if let Some(d_pos) = formula.find('d') {
        let num_dice: i32 = formula[..d_pos].parse().unwrap_or(1);
        let rest = &formula[d_pos + 1..];
        
        let (die_size, modifier) = if let Some(plus_pos) = rest.find('+') {
            let die_size: i32 = rest[..plus_pos].parse().unwrap_or(6);
            let modifier: i32 = rest[plus_pos + 1..].parse().unwrap_or(0);
            (die_size, modifier)
        } else if let Some(minus_pos) = rest.find('-') {
            let die_size: i32 = rest[..minus_pos].parse().unwrap_or(6);
            let modifier: i32 = -(rest[minus_pos + 1..].parse().unwrap_or(0));
            (die_size, modifier)
        } else {
            (rest.parse().unwrap_or(6), 0)
        };
        
        let mut total = 0;
        for _ in 0..num_dice {
            total += rand::thread_rng().gen_range(1..=die_size);
        }
        total + modifier
    } else {
        formula.parse().unwrap_or(1)
    }
}

fn create_player_actions() -> Vec<CombatAction> {
    vec![
        CombatAction {
            name: "Sword Attack".to_string(),
            action_type: ActionType::Attack,
            attack_bonus: Some(5),
            damage_formula: Some("1d8+3".to_string()),
            range: Some(1),
            save_dc: None,
            save_ability: None,
            description: "A melee weapon attack with your sword".to_string(),
            recharge: None,
        },
        CombatAction {
            name: "Crossbow Shot".to_string(),
            action_type: ActionType::Attack,
            attack_bonus: Some(4),
            damage_formula: Some("1d8+2".to_string()),
            range: Some(30),
            save_dc: None,
            save_ability: None,
            description: "A ranged weapon attack with your crossbow".to_string(),
            recharge: None,
        },
    ]
}

fn determine_ai_behavior(creature_type: &str) -> AIBehaviorType {
    match creature_type {
        "undead" => AIBehaviorType::Aggressive,
        "beast" => AIBehaviorType::Hunting,
        "humanoid" => AIBehaviorType::Tactical,
        "fiend" => AIBehaviorType::Berserker,
        _ => AIBehaviorType::Aggressive,
    }
}

fn determine_preferred_range(actions: &[CombatAction]) -> CombatRange {
    let ranged_actions = actions.iter().filter(|a| a.range.unwrap_or(1) > 1).count();
    let melee_actions = actions.iter().filter(|a| a.range.unwrap_or(1) <= 1).count();
    
    if ranged_actions > melee_actions {
        CombatRange::Ranged
    } else if melee_actions > 0 {
        CombatRange::Melee
    } else {
        CombatRange::Mixed
    }
}

fn select_ai_action(ai: &CombatAI, actions: &CombatActions) -> Option<&CombatAction> {
    match ai.behavior_type {
        AIBehaviorType::Aggressive => {
            // Pick highest damage action
            actions.actions.iter()
                .filter(|a| !actions.used_actions.contains(&a.name))
                .max_by_key(|a| parse_damage_formula(&a.damage_formula.clone().unwrap_or("1".to_string())))
        }
        AIBehaviorType::Tactical => {
            // Pick most appropriate action for situation
            actions.actions.first()
        }
        _ => actions.actions.first()
    }
}

fn find_nearest_target(
    position: &CombatPosition,
    target_query: &Query<&CombatPosition, (With<CombatParticipant>, Without<CombatAI>)>
) -> Option<CombatPosition> {
    target_query.iter()
        .min_by_key(|target_pos| {
            let dq = position.q - target_pos.q;
            let dr = position.r - target_pos.r;
            (dq * dq + dr * dr) // Hex distance approximation
        })
        .cloned()
}

fn move_towards_target(position: &mut CombatPosition, target: &CombatPosition) {
    // Simple movement towards target (1 hex per turn)
    let dq = target.q - position.q;
    let dr = target.r - position.r;
    
    if dq.abs() > dr.abs() {
        position.q += dq.signum();
    } else {
        position.r += dr.signum();
    }
}

fn move_away_from_target(position: &mut CombatPosition, target: &CombatPosition) {
    // Simple movement away from target
    let dq = position.q - target.q;
    let dr = position.r - target.r;
    
    if dq.abs() > dr.abs() {
        position.q += dq.signum();
    } else {
        position.r += dr.signum();
    }
}

fn calculate_terrain_movement_cost(terrain: &str) -> f32 {
    match terrain {
        "swamp" => 2.0,
        "mountain" => 1.5,
        "jungle" => 1.3,
        "forest" => 1.1,
        "plains" => 1.0,
        "desert" => 1.2,
        _ => 1.0,
    }
}
