//! Combat Components - ECS components for D&D 5e combat system

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Core combat creature component
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct CombatCreature {
    pub creature_id: Uuid,
    pub name: String,
    pub creature_type: String,
    pub challenge_rating: String,
}

/// Creature stats component for D&D 5e mechanics
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct CreatureStats {
    pub current_hp: i32,
    pub max_hp: i32,
    pub armor_class: i32,
    pub strength: i32,
    pub dexterity: i32,
    pub constitution: i32,
    pub intelligence: i32,
    pub wisdom: i32,
    pub charisma: i32,
}

impl CreatureStats {
    pub fn ability_modifier(score: i32) -> i32 {
        (score - 10) / 2
    }
    
    pub fn strength_modifier(&self) -> i32 {
        Self::ability_modifier(self.strength)
    }
    
    pub fn dexterity_modifier(&self) -> i32 {
        Self::ability_modifier(self.dexterity)
    }
    
    pub fn constitution_modifier(&self) -> i32 {
        Self::ability_modifier(self.constitution)
    }
    
    pub fn is_alive(&self) -> bool {
        self.current_hp > 0
    }
    
    pub fn is_bloodied(&self) -> bool {
        self.current_hp <= self.max_hp / 2
    }
}

/// Combat position on tactical grid
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct CombatPosition {
    pub q: i32,
    pub r: i32,
    pub facing: CombatFacing,
}

/// Facing direction for tactical combat
#[derive(Debug, Clone, Reflect, Serialize, Deserialize)]
pub enum CombatFacing {
    North,
    Northeast,
    Southeast,
    South,
    Southwest,
    Northwest,
}

/// Available actions in combat
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct CombatActions {
    pub actions: Vec<CombatAction>,
    pub used_actions: Vec<String>, // Track what's been used this turn
}

#[derive(Debug, Clone, Reflect, Serialize, Deserialize)]
pub struct CombatAction {
    pub name: String,
    pub action_type: ActionType,
    pub attack_bonus: Option<i32>,
    pub damage_formula: Option<String>,
    pub range: Option<u32>,
    pub save_dc: Option<i32>,
    pub save_ability: Option<String>,
    pub description: String,
    pub recharge: Option<String>, // "5-6" for recharge on 5-6
}

#[derive(Debug, Clone, Reflect, Serialize, Deserialize)]
pub enum ActionType {
    Attack,
    Spell,
    Ability,
    Movement,
    Utility,
}

/// Turn order and initiative tracking
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct TurnOrder {
    pub initiative: i32,
    pub turn_taken: bool,
    pub is_player_controlled: bool,
}

/// Active participant in combat
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct CombatParticipant {
    pub participant_type: ParticipantType,
    pub team: CombatTeam,
}

#[derive(Debug, Clone, Reflect, Serialize, Deserialize)]
pub enum ParticipantType {
    Player,
    Companion,
    Enemy,
    Neutral,
}

#[derive(Debug, Clone, Reflect, Serialize, Deserialize)]
pub enum CombatTeam {
    PlayerTeam,
    EnemyTeam,
    Neutral,
}

/// Status effects and conditions
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct StatusEffects {
    pub effects: Vec<StatusEffect>,
}

#[derive(Debug, Clone, Reflect, Serialize, Deserialize)]
pub struct StatusEffect {
    pub effect_type: StatusEffectType,
    pub duration: i32, // rounds remaining
    pub source: String, // what caused this effect
    pub save_ends: Option<(String, i32)>, // (ability, DC) for save to end
}

#[derive(Debug, Clone, Reflect, Serialize, Deserialize)]
pub enum StatusEffectType {
    Poisoned,
    Paralyzed,
    Charmed,
    Frightened,
    Stunned,
    Prone,
    Grappled,
    Restrained,
    Blinded,
    Deafened,
    // Horror-specific effects
    Corrupted,
    Terrified,
    Despairing,
    Bloodthirsty,
}

/// Damage tracking and types
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct DamageResistance {
    pub resistances: Vec<DamageType>,
    pub immunities: Vec<DamageType>,
    pub vulnerabilities: Vec<DamageType>,
}

#[derive(Debug, Clone, Reflect, Serialize, Deserialize)]
pub enum DamageType {
    Slashing,
    Piercing,
    Bludgeoning,
    Fire,
    Cold,
    Lightning,
    Thunder,
    Acid,
    Poison,
    Necrotic,
    Radiant,
    Psychic,
    Force,
    // Horror-specific damage
    Corruption,
    Void,
    Despair,
}

/// AI behavior for creatures
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct CombatAI {
    pub behavior_type: AIBehaviorType,
    pub target_priority: Vec<ParticipantType>,
    pub preferred_range: CombatRange,
    pub morale: i32, // 0-100, affects when AI flees
}

#[derive(Debug, Clone, Reflect, Serialize, Deserialize)]
pub enum AIBehaviorType {
    Aggressive,    // Always attacks
    Defensive,     // Prioritizes defense
    Tactical,      // Uses positioning and abilities
    Berserker,     // Reckless attacks
    Coward,        // Tries to flee
    Protective,    // Defends allies
    Hunting,       // Stalks weakest targets
}

#[derive(Debug, Clone, Reflect, Serialize, Deserialize)]
pub enum CombatRange {
    Melee,      // Prefers close combat
    Ranged,     // Prefers distance
    Mixed,      // Uses both as appropriate
    Support,    // Stays back, uses abilities
}

/// Environmental effects during combat
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct EnvironmentalEffect {
    pub effect_type: EnvironmentalEffectType,
    pub area: Vec<CombatPosition>,
    pub duration: Option<i32>, // rounds, None = permanent
    pub damage_per_turn: Option<String>, // damage formula
}

#[derive(Debug, Clone, Reflect, Serialize, Deserialize)]
pub enum EnvironmentalEffectType {
    Fire,
    Ice,
    Poison,
    Acid,
    Darkness,
    DifficultTerrain,
    // Horror effects
    CorruptionZone,
    VoidRift,
    HallucinationField,
}
