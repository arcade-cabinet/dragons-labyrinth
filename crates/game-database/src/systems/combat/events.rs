//! Combat Events - ECS events for combat system communication

use bevy::prelude::*;
use super::components::*;
use crate::systems::HexPosition;

/// Event to initiate combat
#[derive(Event, Debug, Clone)]
pub struct CombatInitiatedEvent {
    pub position: HexPosition,
    pub creature_names: Vec<String>,
    pub environment_type: String,
    pub player_hp: i32,
    pub player_max_hp: i32,
    pub player_ac: i32,
}

/// Event for attack actions
#[derive(Event, Debug, Clone)]
pub struct AttackEvent {
    pub attacker: Entity,
    pub target: Entity,
    pub action_name: String,
    pub attack_bonus: i32,
    pub damage_formula: Option<String>,
    pub damage_type: DamageType,
}

/// Event for damage application
#[derive(Event, Debug, Clone)]
pub struct DamageEvent {
    pub target: Entity,
    pub damage: i32,
    pub damage_type: DamageType,
    pub source: Entity,
    pub is_critical: bool,
}

/// Event when combat ends
#[derive(Event, Debug, Clone)]
pub struct CombatEndedEvent {
    pub result: CombatResult,
    pub rounds_lasted: u32,
}

#[derive(Debug, Clone)]
pub enum CombatResult {
    Victory,
    Defeat,
    Flee,
}

/// Event when creature is defeated
#[derive(Event, Debug, Clone)]
pub struct CreatureDefeatedEvent {
    pub entity: Entity,
    pub participant_type: ParticipantType,
    pub defeated_by: Entity,
}

/// Event when player is defeated
#[derive(Event, Debug, Clone)]
pub struct PlayerDefeatedEvent {
    pub cause: String,
    pub final_damage: i32,
}

/// Event for status effect application
#[derive(Event, Debug, Clone)]
pub struct StatusEffectEvent {
    pub target: Entity,
    pub effect: StatusEffect,
    pub source: Entity,
}

/// Event for healing
#[derive(Event, Debug, Clone)]
pub struct HealingEvent {
    pub target: Entity,
    pub healing: i32,
    pub source: Entity,
    pub healing_type: HealingType,
}

#[derive(Debug, Clone)]
pub enum HealingType {
    Magical,
    Natural,
    Potion,
    Rest,
}

/// Event for movement in combat
#[derive(Event, Debug, Clone)]
pub struct MovementEvent {
    pub entity: Entity,
    pub from: CombatPosition,
    pub to: CombatPosition,
    pub movement_type: MovementType,
}

#[derive(Debug, Clone)]
pub enum MovementType {
    Walk,
    Run,
    Teleport,
    Forced, // Pushed/pulled
}

/// Event for spell casting
#[derive(Event, Debug, Clone)]
pub struct SpellCastEvent {
    pub caster: Entity,
    pub spell_name: String,
    pub targets: Vec<Entity>,
    pub spell_level: u32,
    pub save_dc: Option<i32>,
    pub save_ability: Option<String>,
}

/// Event for environmental effects
#[derive(Event, Debug, Clone)]
pub struct EnvironmentalEffectEvent {
    pub effect_type: EnvironmentalEffectType,
    pub area: Vec<CombatPosition>,
    pub duration: Option<i32>,
    pub source: String,
}

/// Event for turn progression
#[derive(Event, Debug, Clone)]
pub struct TurnProgressEvent {
    pub current_entity: Entity,
    pub turn_number: u32,
    pub phase: CombatPhase,
}

/// Event for initiative rolls
#[derive(Event, Debug, Clone)]
pub struct InitiativeEvent {
    pub entity: Entity,
    pub initiative_roll: i32,
    pub dexterity_modifier: i32,
}

/// Event for opportunity attacks
#[derive(Event, Debug, Clone)]
pub struct OpportunityAttackEvent {
    pub attacker: Entity,
    pub target: Entity,
    pub triggered_by: MovementType,
}

/// Event for death saves (D&D 5e mechanic)
#[derive(Event, Debug, Clone)]
pub struct DeathSaveEvent {
    pub entity: Entity,
    pub roll: i32,
    pub successes: u32,
    pub failures: u32,
    pub is_stable: bool,
}

/// Event for horror effects in combat
#[derive(Event, Debug, Clone)]
pub struct HorrorEffectEvent {
    pub affected_entity: Entity,
    pub horror_type: HorrorEffectType,
    pub intensity: f32,
    pub source: String,
}

#[derive(Debug, Clone)]
pub enum HorrorEffectType {
    CorruptionSickness,
    VoidWhispers,
    BloodLust,
    Despair,
    Madness,
}

/// Event for companion trauma during combat
#[derive(Event, Debug, Clone)]
pub struct CompanionTraumaEvent {
    pub companion: Entity,
    pub trauma_type: TraumaType,
    pub severity: f32,
    pub triggered_by: String,
}

#[derive(Debug, Clone)]
pub enum TraumaType {
    WitnessedDeath,
    SevereInjury,
    BetrayalFear,
    CorruptionExposure,
    HorrorOverload,
}

/// Event for loot drops after combat
#[derive(Event, Debug, Clone)]
pub struct LootDropEvent {
    pub source_creature: Entity,
    pub items: Vec<LootItem>,
    pub position: CombatPosition,
}

#[derive(Debug, Clone)]
pub struct LootItem {
    pub name: String,
    pub item_type: String,
    pub quantity: u32,
    pub value: u32,
    pub rarity: ItemRarity,
}

#[derive(Debug, Clone)]
pub enum ItemRarity {
    Common,
    Uncommon,
    Rare,
    VeryRare,
    Legendary,
}

/// Event for combat dialogue (taunts, commands, etc.)
#[derive(Event, Debug, Clone)]
pub struct CombatDialogueEvent {
    pub speaker: Entity,
    pub dialogue_type: DialogueType,
    pub text: String,
    pub targets: Vec<Entity>,
}

#[derive(Debug, Clone)]
pub enum DialogueType {
    Taunt,
    Command, // For companions
    Intimidation,
    Plea,
    BattleCry,
    DeathWords,
}
