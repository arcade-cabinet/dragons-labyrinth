//! Item components for sophisticated inventory and equipment system

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Sentimental items and equipment with horror progression integration
#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub struct Item {
    pub id: Uuid,
    pub player_entity: Option<Entity>, // Who owns this item (if owned)
    pub companion_entity: Option<Entity>, // Which companion gave this (if applicable)
    
    // Basic item data
    pub name: String,
    pub description: String,
    pub item_type: ItemType,
    pub rarity: ItemRarity,
    
    // Sentimental value system (core to horror progression)
    pub sentimental_value: f32, // 0.0-1.0 how emotionally significant
    pub memory_description: Option<String>, // Why this item matters
    pub emotional_category: Option<EmotionalCategory>,
    pub acquired_context: Option<String>, // Story of how player got this
    
    // Horror corruption system
    pub corruption_level: f32, // 0.0-1.0 how corrupted by horror
    pub dread_resonance: i32, // Which dread level (0-4) this item resonates with
    
    // Forge system integration
    pub forge_reagent_power: f32, // Power when used as reagent
    pub light_path_compatibility: f32, // 0.0-1.0 for High Elves forge
    pub dark_path_compatibility: f32, // 0.0-1.0 for Cursed forge
    pub essence_vs_blood_ratio: f32, // -1.0 to 1.0 (essence to blood)
    
    // Game mechanics
    pub stack_size: i32, // How many can be stacked
    pub durability: Option<f32>, // Current durability (if applicable)
    pub max_durability: Option<f32>, // Maximum durability
    
    // Asset references
    pub icon_asset_id: Option<String>, // Reference to asset in game-assets
    pub model_asset_id: Option<String>, // 3D model reference
    
    // Timestamps
    pub created_at: DateTime<Utc>,
    pub last_modified: DateTime<Utc>,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub enum ItemType {
    Sentimental,
    Weapon,
    Armor,
    Consumable,
    Key,
    Tool,
    Material,
    Quest,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub enum ItemRarity {
    Common,
    Uncommon,
    Rare,
    Legendary,
    Cursed,
    Blessed,
    Mythic,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub enum EmotionalCategory {
    Love,
    Loss,
    Friendship,
    Fear,
    Hope,
    Courage,
    Despair,
}

/// Component for equipment slots and bonuses
#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub struct Equipment {
    pub item_entity: Entity,
    pub equipment_slot: EquipmentSlot,
    pub equipped_by: Entity,     // Player or companion entity
    pub stat_bonuses: StatBonuses,
    pub special_abilities: Vec<SpecialAbility>,
    pub equipped_at: DateTime<Utc>,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub enum EquipmentSlot {
    Head,
    Chest,
    Legs,
    Feet,
    Hands,
    MainHand,
    OffHand,
    Ring,
    Amulet,
    Cloak,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub struct StatBonuses {
    pub armor_class: i32,
    pub attack_bonus: i32,
    pub damage_bonus: i32,
    pub movement_speed: f32,
    pub dread_resistance: f32,
    pub corruption_resistance: f32,
    pub healing_bonus: f32,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub struct SpecialAbility {
    pub ability_name: String,
    pub description: String,
    pub activation_type: ActivationType,
    pub cooldown_turns: Option<u32>,
    pub resource_cost: Option<String>,
    pub effectiveness: f32,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub enum ActivationType {
    Passive,
    OnUse,
    OnEquip,
    OnUnequip,
    OnDamage,
    OnHeal,
    OnMove,
    OnEncounter,
}

/// Component for inventory management
#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub struct Inventory {
    pub owner_entity: Entity,
    pub items: Vec<InventorySlot>,
    pub max_capacity: u32,
    pub weight_limit: Option<f32>,
    pub current_weight: f32,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub struct InventorySlot {
    pub item_entity: Entity,
    pub quantity: u32,
    pub slot_index: u32,
}

/// Event fired when item is equipped
#[derive(Event, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Event)]
pub struct ItemEquippedEvent {
    pub item_entity: Entity,
    pub equipped_by: Entity,
    pub equipment_slot: EquipmentSlot,
    pub previous_item: Option<Entity>,
}

/// Event fired when item is used
#[derive(Event, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Event)]
pub struct ItemUsedEvent {
    pub item_entity: Entity,
    pub used_by: Entity,
    pub usage_context: ItemUsageContext,
    pub effectiveness: f32,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub enum ItemUsageContext {
    Combat,
    Exploration,
    Dialogue,
    Forge,
    Healing,
    Crafting,
    Puzzle,
}

/// Component for weapon-specific properties
#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub struct Weapon {
    pub weapon_type: WeaponType,
    pub damage_dice: String,     // "1d8", "2d6", etc.
    pub damage_type: DamageType,
    pub range: Option<u32>,      // Range in hexes for ranged weapons
    pub two_handed: bool,
    pub finesse: bool,           // Can use DEX instead of STR
    pub versatile: Option<String>, // Different damage if two-handed
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub enum WeaponType {
    Sword,
    Axe,
    Mace,
    Dagger,
    Bow,
    Crossbow,
    Spear,
    Staff,
    Wand,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub enum DamageType {
    Slashing,
    Piercing,
    Bludgeoning,
    Fire,
    Cold,
    Lightning,
    Acid,
    Poison,
    Necrotic,
    Radiant,
    Psychic,
}

/// Component for armor-specific properties
#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub struct Armor {
    pub armor_type: ArmorType,
    pub base_armor_class: u32,
    pub max_dex_bonus: Option<i32>,
    pub strength_requirement: Option<u32>,
    pub stealth_disadvantage: bool,
    pub magical_enhancement: Option<i32>, // +1, +2, etc.
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub enum ArmorType {
    Light,
    Medium,
    Heavy,
    Shield,
    Natural,
    Magical,
}
