//! Combat Queries - Database queries for combat system integration

use anyhow::Result;
use database_orm::*;
use sea_orm::{DatabaseConnection, EntityTrait, ColumnTrait, QueryFilter};
use std::collections::HashMap;
use tracing::{debug, info};
use uuid::Uuid;
use super::components::*;
use super::resources::*;

/// Load creature templates from HBF data into ECS resources
pub async fn load_creature_templates_from_db(
    db: &DatabaseConnection,
    cache: &mut CreatureTemplateCache,
) -> Result<()> {
    // This would query the parsed HBF creature data and convert to ECS format
    // For now, adding some basic templates based on D&D 5e SRD
    
    cache.templates.insert("Wolf".to_string(), CreatureTemplate {
        name: "Wolf".to_string(),
        creature_type: "beast".to_string(),
        challenge_rating: "1/4".to_string(),
        armor_class: 13,
        hit_points_formula: "2d8+2".to_string(),
        abilities: CreatureAbilityScores {
            strength: 12, dexterity: 15, constitution: 12,
            intelligence: 3, wisdom: 12, charisma: 6,
        },
        actions: vec![
            CombatAction {
                name: "Bite".to_string(),
                action_type: ActionType::Attack,
                attack_bonus: Some(4),
                damage_formula: Some("2d4+2".to_string()),
                range: Some(1),
                save_dc: None,
                save_ability: None,
                description: "Melee Weapon Attack: +4 to hit, reach 5 ft.".to_string(),
                recharge: None,
            }
        ],
        special_abilities: vec!["Keen Hearing and Smell".to_string(), "Pack Tactics".to_string()],
        damage_resistances: Vec::new(),
        damage_immunities: Vec::new(),
        condition_immunities: Vec::new(),
        senses: vec!["darkvision 60 ft.".to_string()],
        languages: Vec::new(),
    });
    
    cache.templates.insert("Goblin".to_string(), CreatureTemplate {
        name: "Goblin".to_string(),
        creature_type: "humanoid".to_string(),
        challenge_rating: "1/4".to_string(),
        armor_class: 15,
        hit_points_formula: "2d6".to_string(),
        abilities: CreatureAbilityScores {
            strength: 8, dexterity: 14, constitution: 10,
            intelligence: 10, wisdom: 8, charisma: 8,
        },
        actions: vec![
            CombatAction {
                name: "Scimitar".to_string(),
                action_type: ActionType::Attack,
                attack_bonus: Some(4),
                damage_formula: Some("1d6+2".to_string()),
                range: Some(1),
                save_dc: None,
                save_ability: None,
                description: "Melee Weapon Attack: +4 to hit, reach 5 ft.".to_string(),
                recharge: None,
            },
            CombatAction {
                name: "Shortbow".to_string(),
                action_type: ActionType::Attack,
                attack_bonus: Some(4),
                damage_formula: Some("1d6+2".to_string()),
                range: Some(24),
                save_dc: None,
                save_ability: None,
                description: "Ranged Weapon Attack: +4 to hit, range 80/320 ft.".to_string(),
                recharge: None,
            }
        ],
        special_abilities: vec!["Nimble Escape".to_string()],
        damage_resistances: Vec::new(),
        damage_immunities: Vec::new(),
        condition_immunities: Vec::new(),
        senses: vec!["darkvision 60 ft.".to_string()],
        languages: vec!["Common".to_string(), "Goblin".to_string()],
    });
    
    cache.templates.insert("Skeleton".to_string(), CreatureTemplate {
        name: "Skeleton".to_string(),
        creature_type: "undead".to_string(),
        challenge_rating: "1/4".to_string(),
        armor_class: 13,
        hit_points_formula: "2d8+4".to_string(),
        abilities: CreatureAbilityScores {
            strength: 10, dexterity: 14, constitution: 15,
            intelligence: 6, wisdom: 8, charisma: 5,
        },
        actions: vec![
            CombatAction {
                name: "Shortsword".to_string(),
                action_type: ActionType::Attack,
                attack_bonus: Some(4),
                damage_formula: Some("1d6+2".to_string()),
                range: Some(1),
                save_dc: None,
                save_ability: None,
                description: "Melee Weapon Attack: +4 to hit, reach 5 ft.".to_string(),
                recharge: None,
            },
            CombatAction {
                name: "Shortbow".to_string(),
                action_type: ActionType::Attack,
                attack_bonus: Some(4),
                damage_formula: Some("1d6+2".to_string()),
                range: Some(24),
                save_dc: None,
                save_ability: None,
                description: "Ranged Weapon Attack: +4 to hit, range 80/320 ft.".to_string(),
                recharge: None,
            }
        ],
        special_abilities: Vec::new(),
        damage_resistances: Vec::new(),
        damage_immunities: vec![DamageType::Poison],
        condition_immunities: vec![StatusEffectType::Poisoned],
        senses: vec!["darkvision 60 ft.".to_string()],
        languages: Vec::new(),
    });
    
    info!("Loaded {} creature templates", cache.templates.len());
    Ok(())
}

/// Load encounter tables from database
pub async fn load_encounter_tables_from_db(
    db: &DatabaseConnection,
    tables: &mut EncounterTables,
) -> Result<()> {
    // Load encounter tables by biome from HBF data
    
    tables.tables.insert("forest".to_string(), BiomeEncounterTable {
        biome: "forest".to_string(),
        base_probability: 0.2,
        encounters: vec![
            EncounterEntry {
                roll_range: (1, 3),
                creature_names: vec!["Wolf".to_string()],
                quantity_formula: "1d4".to_string(),
                special_conditions: Vec::new(),
            },
            EncounterEntry {
                roll_range: (4, 5),
                creature_names: vec!["Goblin".to_string()],
                quantity_formula: "2d4".to_string(),
                special_conditions: vec!["ambush".to_string()],
            },
            EncounterEntry {
                roll_range: (6, 6),
                creature_names: vec!["Wolf".to_string(), "Goblin".to_string()],
                quantity_formula: "1d2".to_string(),
                special_conditions: vec!["mixed_encounter".to_string()],
            },
        ],
    });
    
    tables.tables.insert("swamp".to_string(), BiomeEncounterTable {
        biome: "swamp".to_string(),
        base_probability: 0.4,
        encounters: vec![
            EncounterEntry {
                roll_range: (1, 2),
                creature_names: vec!["Skeleton".to_string()],
                quantity_formula: "1d6".to_string(),
                special_conditions: vec!["undead_presence".to_string()],
            },
            EncounterEntry {
                roll_range: (3, 4),
                creature_names: vec!["Goblin".to_string()],
                quantity_formula: "1d4+1".to_string(),
                special_conditions: vec!["desperate".to_string()],
            },
            EncounterEntry {
                roll_range: (5, 6),
                creature_names: vec!["Wolf".to_string()],
                quantity_formula: "1d2".to_string(),
                special_conditions: vec!["diseased".to_string()],
            },
        ],
    });
    
    tables.tables.insert("mountain".to_string(), BiomeEncounterTable {
        biome: "mountain".to_string(),
        base_probability: 0.25,
        encounters: vec![
            EncounterEntry {
                roll_range: (1, 4),
                creature_names: vec!["Goblin".to_string()],
                quantity_formula: "1d6+2".to_string(),
                special_conditions: vec!["high_ground".to_string()],
            },
            EncounterEntry {
                roll_range: (5, 6),
                creature_names: vec!["Wolf".to_string()],
                quantity_formula: "1d3".to_string(),
                special_conditions: vec!["pack_hunting".to_string()],
            },
        ],
    });
    
    info!("Loaded encounter tables for {} biomes", tables.tables.len());
    Ok(())
}

/// Query settlements for combat location context
pub async fn query_settlement_combat_context(
    db: &DatabaseConnection,
    position: crate::systems::HexPosition,
) -> Result<Option<SettlementCombatContext>> {
    // Find nearby settlements that might affect combat
    let settlements = settlements::Entity::find()
        .all(db)
        .await?;
    
    for settlement in settlements {
        if let (Some(hbf_x), Some(hbf_y)) = (settlement.hbf_x, settlement.hbf_y) {
            let settlement_pos = crate::systems::HexPosition::from_hbf_coords(hbf_x, hbf_y);
            let distance = position.distance_to(&settlement_pos);
            
            if distance <= 3 { // Within 3 hexes of settlement
                return Ok(Some(SettlementCombatContext {
                    settlement_name: settlement.name,
                    settlement_type: settlement.settlement_type,
                    safety_rating: settlement.safety_rating,
                    guards_available: distance == 0, // Guards only in settlement itself
                    reputation_effects: settlement.reputation > 5,
                }));
            }
        }
    }
    
    Ok(None)
}

/// Query weather effects for combat
pub async fn query_weather_combat_effects(
    db: &DatabaseConnection,
    position: crate::systems::HexPosition,
) -> Result<Option<WeatherCombatEffects>> {
    // Find nearest settlement with weather data
    let settlements = settlements::Entity::find()
        .all(db)
        .await?;
    
    let mut nearest_settlement = None;
    let mut min_distance = f32::INFINITY;
    
    for settlement in settlements {
        if let (Some(hbf_x), Some(hbf_y)) = (settlement.hbf_x, settlement.hbf_y) {
            let settlement_pos = crate::systems::HexPosition::from_hbf_coords(hbf_x, hbf_y);
            let distance = position.distance_to(&settlement_pos) as f32;
            
            if distance < min_distance {
                min_distance = distance;
                nearest_settlement = Some(settlement.id);
            }
        }
    }
    
    if let Some(settlement_id) = nearest_settlement {
        let weather_entries = settlements::weather::Entity::find()
            .filter(settlements::weather::Column::SettlementId.eq(settlement_id))
            .all(db)
            .await?;
        
        if !weather_entries.is_empty() {
            // Use first weather entry for now (would roll based on season/time)
            let weather = &weather_entries[0];
            
            return Ok(Some(WeatherCombatEffects {
                condition: weather.weather_condition.clone(),
                visibility_penalty: get_visibility_penalty(&weather.weather_condition),
                movement_penalty: get_movement_penalty(&weather.weather_condition),
                special_effects: get_weather_special_effects(&weather.weather_condition),
            }));
        }
    }
    
    Ok(None)
}

/// Query corruption effects for combat
pub async fn query_corruption_combat_effects(
    db: &DatabaseConnection,
    position: crate::systems::HexPosition,
) -> Result<Option<CorruptionCombatEffects>> {
    let tile = hex_tiles::Entity::find()
        .filter(hex_tiles::Column::Q.eq(position.q))
        .filter(hex_tiles::Column::R.eq(position.r))
        .filter(hex_tiles::Column::S.eq(position.s))
        .one(db)
        .await?;
    
    if let Some(tile) = tile {
        if tile.corruption_level > 0.1 {
            return Ok(Some(CorruptionCombatEffects {
                corruption_level: tile.corruption_level,
                dread_intensity: tile.dread_intensity,
                horror_effects: get_corruption_horror_effects(tile.corruption_level),
                creature_buffs: get_corruption_creature_buffs(tile.corruption_level),
                environmental_changes: get_corruption_environmental_changes(tile.dread_intensity),
            }));
        }
    }
    
    Ok(None)
}

/// Query faction relations for combat context
pub async fn query_faction_combat_relations(
    db: &DatabaseConnection,
    position: crate::systems::HexPosition,
    creature_factions: &[String],
) -> Result<FactionCombatContext> {
    let mut faction_relations = HashMap::new();
    
    // Check if any creatures belong to factions the player has relationships with
    let settlements = settlements::Entity::find()
        .all(db)
        .await?;
    
    for settlement in settlements {
        if let Some(faction_name) = &settlement.faction {
            if creature_factions.contains(faction_name) {
                faction_relations.insert(faction_name.clone(), FactionCombatRelation {
                    faction_name: faction_name.clone(),
                    reputation: settlement.reputation,
                    affects_encounter: true,
                    can_negotiate: settlement.reputation > 0,
                    reinforcements_possible: settlement.safety_rating > 6,
                });
            }
        }
    }
    
    Ok(FactionCombatContext {
        relations: faction_relations,
        diplomatic_options: !faction_relations.is_empty(),
    })
}

/// Supporting data structures

#[derive(Debug, Clone)]
pub struct SettlementCombatContext {
    pub settlement_name: String,
    pub settlement_type: String,
    pub safety_rating: i32,
    pub guards_available: bool,
    pub reputation_effects: bool,
}

#[derive(Debug, Clone)]
pub struct WeatherCombatEffects {
    pub condition: String,
    pub visibility_penalty: i32,
    pub movement_penalty: i32,
    pub special_effects: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct CorruptionCombatEffects {
    pub corruption_level: f32,
    pub dread_intensity: i32,
    pub horror_effects: Vec<String>,
    pub creature_buffs: Vec<String>,
    pub environmental_changes: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct FactionCombatContext {
    pub relations: HashMap<String, FactionCombatRelation>,
    pub diplomatic_options: bool,
}

#[derive(Debug, Clone)]
pub struct FactionCombatRelation {
    pub faction_name: String,
    pub reputation: i32,
    pub affects_encounter: bool,
    pub can_negotiate: bool,
    pub reinforcements_possible: bool,
}

/// Helper functions

fn get_visibility_penalty(weather: &str) -> i32 {
    match weather {
        "Foggy" => -5,
        "Stormy" => -3,
        "Rainy" => -2,
        "Cloudy" => -1,
        _ => 0,
    }
}

fn get_movement_penalty(weather: &str) -> i32 {
    match weather {
        "Stormy" => -10, // feet
        "Rainy" => -5,
        "Foggy" => -5,
        _ => 0,
    }
}

fn get_weather_special_effects(weather: &str) -> Vec<String> {
    match weather {
        "Stormy" => vec!["lightning_strikes".to_string(), "high_winds".to_string()],
        "Rainy" => vec!["slippery_terrain".to_string()],
        "Foggy" => vec!["concealment".to_string()],
        "Snowy" => vec!["difficult_terrain".to_string(), "cold_damage".to_string()],
        _ => Vec::new(),
    }
}

fn get_corruption_horror_effects(corruption_level: f32) -> Vec<String> {
    let mut effects = Vec::new();
    
    if corruption_level > 0.3 {
        effects.push("fear_aura".to_string());
    }
    if corruption_level > 0.5 {
        effects.push("whispers".to_string());
    }
    if corruption_level > 0.7 {
        effects.push("hallucinations".to_string());
    }
    if corruption_level > 0.9 {
        effects.push("reality_distortion".to_string());
    }
    
    effects
}

fn get_corruption_creature_buffs(corruption_level: f32) -> Vec<String> {
    let mut buffs = Vec::new();
    
    if corruption_level > 0.4 {
        buffs.push("undead_strength".to_string());
    }
    if corruption_level > 0.6 {
        buffs.push("void_resistance".to_string());
    }
    if corruption_level > 0.8 {
        buffs.push("corruption_regeneration".to_string());
    }
    
    buffs
}

fn get_corruption_environmental_changes(dread_intensity: i32) -> Vec<String> {
    match dread_intensity {
        0 => Vec::new(),
        1 => vec!["withered_plants".to_string()],
        2 => vec!["dark_shadows".to_string(), "unnatural_silence".to_string()],
        3 => vec!["twisted_terrain".to_string(), "poisoned_air".to_string()],
        4 => vec!["void_rifts".to_string(), "reality_breaks".to_string()],
        _ => Vec::new(),
    }
}
