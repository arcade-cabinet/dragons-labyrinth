//! Combat variety and enemy definitions
//!
//! This module defines all enemy types, their variants,
//! and how weather/dread affects combat.

use serde::{Deserialize, Serialize};

/// Wolf variant types that appear from Level 1
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WolfVariant {
    /// Basic grey wolf - standard enemy
    GreyWolf {
        health: f32,
        speed: f32,
        pack_bonus: bool,
    },
    
    /// Dire wolf - larger and more aggressive
    DireWolf {
        health: f32,
        damage_multiplier: f32,
        intimidation_radius: f32,
    },
    
    /// Shadow wolf - appears in fog/darkness
    ShadowWolf {
        health: f32,
        stealth_rating: f32,
        void_touched: bool,
    },
    
    /// Frost wolf - appears in snow/winter
    FrostWolf {
        health: f32,
        freeze_chance: f32,
        cold_resistance: f32,
    },
    
    /// Rabid wolf - unpredictable and dangerous
    RabidWolf {
        health: f32,
        frenzy_chance: f32,
        disease_damage: f32,
    },
    
    /// Alpha wolf - pack leader
    AlphaWolf {
        health: f32,
        pack_size: u8,
        buff_radius: f32,
    },
    
    /// Void wolf - corrupted by void energy (late game)
    VoidWolf {
        health: f32,
        void_damage: f32,
        phase_chance: f32,
    },
}

impl WolfVariant {
    /// Get base stats for early game wolves
    pub fn early_game_variants() -> Vec<Self> {
        vec![
            WolfVariant::GreyWolf {
                health: 20.0,
                speed: 5.0,
                pack_bonus: true,
            },
            WolfVariant::DireWolf {
                health: 35.0,
                damage_multiplier: 1.5,
                intimidation_radius: 10.0,
            },
            WolfVariant::ShadowWolf {
                health: 15.0,
                stealth_rating: 0.7,
                void_touched: false,
            },
            WolfVariant::FrostWolf {
                health: 25.0,
                freeze_chance: 0.2,
                cold_resistance: 0.8,
            },
            WolfVariant::RabidWolf {
                health: 18.0,
                frenzy_chance: 0.3,
                disease_damage: 2.0,
            },
        ]
    }
    
    /// Get the variant most suited for given weather
    pub fn weather_appropriate(weather: Weather) -> Self {
        match weather {
            Weather::Snow | Weather::Storm => WolfVariant::FrostWolf {
                health: 25.0,
                freeze_chance: 0.3,
                cold_resistance: 0.9,
            },
            Weather::Fog => WolfVariant::ShadowWolf {
                health: 15.0,
                stealth_rating: 0.9,
                void_touched: false,
            },
            Weather::VoidStorm => WolfVariant::VoidWolf {
                health: 40.0,
                void_damage: 10.0,
                phase_chance: 0.2,
            },
            _ => WolfVariant::GreyWolf {
                health: 20.0,
                speed: 5.0,
                pack_bonus: true,
            },
        }
    }
}

/// Weather conditions that affect combat
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Weather {
    Clear,
    Rain,
    Storm,
    Fog,
    Snow,
    AshFall,
    VoidStorm,
}

/// Combat modifiers based on weather
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherCombatEffects {
    pub visibility_modifier: f32,      // 0.0 = no visibility, 1.0 = full
    pub movement_speed_modifier: f32,  // Multiplier for movement
    pub accuracy_modifier: f32,        // Hit chance modifier
    pub elemental_damage: Option<ElementalDamage>,
    pub special_effects: Vec<WeatherEffect>,
}

/// Elemental damage types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ElementalDamage {
    Cold(f32),
    Lightning(f32),
    Void(f32),
    Fire(f32),
}

/// Special weather effects during combat
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WeatherEffect {
    /// Fog reduces visibility
    ReducedVisibility { radius: f32 },
    
    /// Rain makes surfaces slippery
    SlipperyGround { slip_chance: f32 },
    
    /// Lightning strikes randomly
    LightningStrikes { chance: f32, damage: f32 },
    
    /// Snow slows movement
    MovementHindrance { speed_reduction: f32 },
    
    /// Ash clouds cause breathing issues
    BreathingDifficulty { stamina_drain: f32 },
    
    /// Void storms corrupt entities
    VoidCorruption { corruption_per_second: f32 },
}

impl Weather {
    /// Get combat effects for this weather
    pub fn combat_effects(&self) -> WeatherCombatEffects {
        match self {
            Weather::Clear => WeatherCombatEffects {
                visibility_modifier: 1.0,
                movement_speed_modifier: 1.0,
                accuracy_modifier: 1.0,
                elemental_damage: None,
                special_effects: vec![],
            },
            
            Weather::Rain => WeatherCombatEffects {
                visibility_modifier: 0.8,
                movement_speed_modifier: 0.9,
                accuracy_modifier: 0.9,
                elemental_damage: None,
                special_effects: vec![
                    WeatherEffect::SlipperyGround { slip_chance: 0.1 },
                ],
            },
            
            Weather::Storm => WeatherCombatEffects {
                visibility_modifier: 0.6,
                movement_speed_modifier: 0.7,
                accuracy_modifier: 0.7,
                elemental_damage: Some(ElementalDamage::Lightning(5.0)),
                special_effects: vec![
                    WeatherEffect::SlipperyGround { slip_chance: 0.2 },
                    WeatherEffect::LightningStrikes { chance: 0.05, damage: 20.0 },
                    WeatherEffect::MovementHindrance { speed_reduction: 0.3 },
                ],
            },
            
            Weather::Fog => WeatherCombatEffects {
                visibility_modifier: 0.3,
                movement_speed_modifier: 0.8,
                accuracy_modifier: 0.5,
                elemental_damage: None,
                special_effects: vec![
                    WeatherEffect::ReducedVisibility { radius: 5.0 },
                ],
            },
            
            Weather::Snow => WeatherCombatEffects {
                visibility_modifier: 0.7,
                movement_speed_modifier: 0.6,
                accuracy_modifier: 0.8,
                elemental_damage: Some(ElementalDamage::Cold(2.0)),
                special_effects: vec![
                    WeatherEffect::MovementHindrance { speed_reduction: 0.4 },
                ],
            },
            
            Weather::AshFall => WeatherCombatEffects {
                visibility_modifier: 0.5,
                movement_speed_modifier: 0.7,
                accuracy_modifier: 0.6,
                elemental_damage: Some(ElementalDamage::Fire(1.0)),
                special_effects: vec![
                    WeatherEffect::BreathingDifficulty { stamina_drain: 2.0 },
                    WeatherEffect::ReducedVisibility { radius: 8.0 },
                ],
            },
            
            Weather::VoidStorm => WeatherCombatEffects {
                visibility_modifier: 0.4,
                movement_speed_modifier: 0.5,
                accuracy_modifier: 0.4,
                elemental_damage: Some(ElementalDamage::Void(3.0)),
                special_effects: vec![
                    WeatherEffect::VoidCorruption { corruption_per_second: 0.5 },
                    WeatherEffect::ReducedVisibility { radius: 6.0 },
                    WeatherEffect::MovementHindrance { speed_reduction: 0.5 },
                ],
            },
        }
    }
}

/// Enemy behavior patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CombatBehavior {
    /// Aggressive - always attacks
    Aggressive { attack_range: f32 },
    
    /// Defensive - waits for player action
    Defensive { counter_chance: f32 },
    
    /// Pack hunter - coordinates with allies
    PackHunter { coordination_radius: f32 },
    
    /// Ambush predator - hides and strikes
    Ambusher { stealth_duration: f32 },
    
    /// Hit and run - strikes then retreats
    HitAndRun { retreat_distance: f32 },
    
    /// Berserker - gets stronger when hurt
    Berserker { rage_threshold: f32 },
}

/// Combat encounter definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatEncounter {
    pub enemies: Vec<Enemy>,
    pub weather: Weather,
    pub terrain: TerrainType,
    pub dread_level: u8,
    pub special_conditions: Vec<CombatCondition>,
}

/// Individual enemy in combat
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enemy {
    pub enemy_type: EnemyType,
    pub behavior: CombatBehavior,
    pub position: (f32, f32),
    pub health: f32,
    pub damage: f32,
    pub armor: f32,
    pub special_abilities: Vec<SpecialAbility>,
}

/// Types of enemies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EnemyType {
    Wolf(WolfVariant),
    Bandit(BanditType),
    Undead(UndeadType),
    Construct(ConstructType),
    VoidSpawn(VoidType),
    Dragon(DragonFragment),
}

/// Bandit enemy types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BanditType {
    Scout,
    Archer,
    Warrior,
    Leader,
}

/// Undead enemy types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UndeadType {
    Skeleton,
    Zombie,
    Wraith,
    Lich,
}

/// Construct enemy types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConstructType {
    Golem,
    Sentinel,
    Guardian,
}

/// Void spawn types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VoidType {
    Wisp,
    Horror,
    Aberration,
}

/// Dragon fragment types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DragonFragment {
    Scale,
    Claw,
    Wing,
    Heart,
}

/// Special abilities enemies can have
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpecialAbility {
    Charge { range: f32, damage: f32 },
    Howl { fear_radius: f32, duration: f32 },
    Regeneration { health_per_second: f32 },
    Teleport { range: f32, cooldown: f32 },
    Summon { summon_type: String, count: u8 },
    VoidBlast { damage: f32, corruption: f32 },
}

/// Terrain types that affect combat
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TerrainType {
    Open,
    Forest,
    Mountain,
    Swamp,
    Ruins,
    Labyrinth,
    VoidRift,
}

/// Special combat conditions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CombatCondition {
    /// Time limit for the encounter
    TimeLimit(f32),
    
    /// Protect an NPC
    ProtectObjective { target: String, health: f32 },
    
    /// Survive waves of enemies
    SurvivalWaves { wave_count: u8 },
    
    /// Cannot use certain abilities
    RestrictedAbilities { blocked: Vec<String> },
    
    /// Environmental hazard
    EnvironmentalHazard { hazard_type: String, damage: f32 },
}

/// Generate a combat encounter for a given level
pub fn generate_combat_encounter(
    level: u32,
    weather: Weather,
    dread_level: u8,
) -> CombatEncounter {
    // Early game (L1-20): Focus on wolf variants
    if level <= 20 {
        let wolf_count = match level {
            1..=5 => 1 + (level / 3) as usize,
            6..=10 => 2 + (level / 5) as usize,
            11..=15 => 3 + (level / 7) as usize,
            16..=20 => 4,
            _ => 2,
        };
        
        let mut enemies = Vec::new();
        for i in 0..wolf_count {
            let variant = if i == 0 && level >= 10 {
                // Pack leader
                WolfVariant::AlphaWolf {
                    health: 40.0 + level as f32,
                    pack_size: wolf_count as u8,
                    buff_radius: 15.0,
                }
            } else {
                // Weather-appropriate wolves
                WolfVariant::weather_appropriate(weather)
            };
            
            enemies.push(Enemy {
                enemy_type: EnemyType::Wolf(variant),
                behavior: if i == 0 {
                    CombatBehavior::PackHunter { coordination_radius: 20.0 }
                } else {
                    CombatBehavior::Aggressive { attack_range: 10.0 }
                },
                position: (i as f32 * 5.0, 0.0),
                health: 20.0 + (level as f32 * 2.0),
                damage: 5.0 + (level as f32 * 0.5),
                armor: level as f32 * 0.2,
                special_abilities: if level >= 5 {
                    vec![SpecialAbility::Howl { fear_radius: 10.0, duration: 2.0 }]
                } else {
                    vec![]
                },
            });
        }
        
        CombatEncounter {
            enemies,
            weather,
            terrain: TerrainType::Forest,
            dread_level,
            special_conditions: if weather == Weather::Storm {
                vec![CombatCondition::EnvironmentalHazard {
                    hazard_type: "Lightning".to_string(),
                    damage: 10.0,
                }]
            } else {
                vec![]
            },
        }
    } else {
        // Later game encounters would be defined here
        CombatEncounter {
            enemies: vec![],
            weather,
            terrain: TerrainType::Open,
            dread_level,
            special_conditions: vec![],
        }
    }
}
