//! The Forge System - Ultimate endgame test
//!
//! Players collect sentimental items throughout their journey,
//! not knowing these will be reagents for the ultimate forge.
//! This system tests EVERYTHING the player has learned.

use crate::philosophy::PhilosophyPath;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Sentimental items collected throughout the game
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentimentalItem {
    pub id: String,
    pub name: String,
    pub description: String,
    pub memory: String, // Why it matters
    pub category: ItemCategory,
    pub essence_type: EssenceType,
    pub forge_power: f32,
    pub acquired_act: u8,
    pub emotional_weight: f32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum ItemCategory {
    Natural,    // Eagle feather, golden scale
    Mystical,   // Dragon whisper, void ore
    Emotional,  // Crystallized tears, heart of hero
    Corrupted,  // Blood of willing, shadow essence
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum EssenceType {
    Light,  // For High Elves forge
    Dark,   // For Cursed forge
    Neutral, // Works for either
}

/// The two forge paths
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum ForgePath {
    /// High Elves forge - requires essence sacrifice
    LightForge,
    /// Cursed forge - requires blood sacrifice
    DarkForge,
}

impl ForgePath {
    pub fn description(&self) -> &'static str {
        match self {
            Self::LightForge => "Ancient elven forge powered by willing essence",
            Self::DarkForge => "Corrupted forge that demands blood sacrifice",
        }
    }
    
    pub fn sacrifice_type(&self) -> SacrificeType {
        match self {
            Self::LightForge => SacrificeType::Essence,
            Self::DarkForge => SacrificeType::Blood,
        }
    }
    
    pub fn mythic_gear_type(&self) -> MythicGearType {
        match self {
            Self::LightForge => MythicGearType::Blessed,
            Self::DarkForge => MythicGearType::Cursed,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum SacrificeType {
    Essence, // Companion willingly gives essence (they live)
    Blood,   // Companion dies for power
    None,    // Refuse sacrifice (only legendary tier)
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum MythicGearType {
    Blessed, // Light-infused gear
    Cursed,  // Dark-powered gear
}

/// Forge trial categories that test all game systems
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForgeTrial {
    pub name: String,
    pub category: TrialCategory,
    pub description: String,
    pub systems_tested: Vec<String>,
    pub difficulty_solo: f32,     // 0.0-1.0
    pub difficulty_party: f32,    // 0.0-1.0
    pub philosophy_requirement: Option<PhilosophyPath>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum TrialCategory {
    Navigation,    // Hex grid mastery
    Combat,        // All combat systems
    Mounted,       // Mount control
    Puzzle,        // Environmental puzzles
    Moral,         // Philosophical choices
    Survival,      // Resource management
    Integration,   // All systems together
}

/// Gear progression tiers
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum GearTier {
    Common,    // White - starting gear
    Uncommon,  // Green - first upgrades
    Rare,      // Blue - mid-game
    Epic,      // Purple - late-game
    Legendary, // Orange - pre-forge max
    Mythic,    // Red/Gold - forge-only
}

impl GearTier {
    pub fn color(&self) -> &'static str {
        match self {
            Self::Common => "#FFFFFF",
            Self::Uncommon => "#1EFF00",
            Self::Rare => "#0080FF",
            Self::Epic => "#B048F8",
            Self::Legendary => "#FF8000",
            Self::Mythic => "#FF0000",
        }
    }
    
    pub fn power_multiplier(&self) -> f32 {
        match self {
            Self::Common => 1.0,
            Self::Uncommon => 1.5,
            Self::Rare => 2.0,
            Self::Epic => 3.0,
            Self::Legendary => 5.0,
            Self::Mythic => 10.0,
        }
    }
}

/// Define standard sentimental items
pub fn define_sentimental_items() -> Vec<SentimentalItem> {
    vec![
        SentimentalItem {
            id: "eagle_feather".to_string(),
            name: "Eagle Feather".to_string(),
            description: "A perfect golden feather that fell at your feet".to_string(),
            memory: "It landed when you chose to spare your first enemy".to_string(),
            category: ItemCategory::Natural,
            essence_type: EssenceType::Light,
            forge_power: 0.3,
            acquired_act: 1,
            emotional_weight: 0.5,
        },
        SentimentalItem {
            id: "void_ore".to_string(),
            name: "Void Ore".to_string(),
            description: "A chunk of reality-warping metal from the depths".to_string(),
            memory: "Pulled from the first void rift you encountered".to_string(),
            category: ItemCategory::Mystical,
            essence_type: EssenceType::Dark,
            forge_power: 0.6,
            acquired_act: 2,
            emotional_weight: 0.7,
        },
        SentimentalItem {
            id: "crystallized_tears".to_string(),
            name: "Crystallized Tears".to_string(),
            description: "Your companion's tears turned solid from grief".to_string(),
            memory: "They wept when you had to leave someone behind".to_string(),
            category: ItemCategory::Emotional,
            essence_type: EssenceType::Neutral,
            forge_power: 0.8,
            acquired_act: 2,
            emotional_weight: 1.0,
        },
        SentimentalItem {
            id: "dragon_whisper".to_string(),
            name: "Dragon Whisper".to_string(),
            description: "A scale that holds the dragon's dying words".to_string(),
            memory: "The dragon recognized what you'd become".to_string(),
            category: ItemCategory::Mystical,
            essence_type: EssenceType::Neutral,
            forge_power: 1.0,
            acquired_act: 3,
            emotional_weight: 0.9,
        },
    ]
}

/// Forge enhancement matrix - what each item grants
pub fn get_enhancement_matrix() -> HashMap<String, Vec<String>> {
    HashMap::from([
        ("eagle_feather".to_string(), vec![
            "+Movement speed".to_string(),
            "Limited flight".to_string(),
            "Fall damage immunity".to_string(),
        ]),
        ("void_ore".to_string(), vec![
            "+Void resistance".to_string(),
            "Damage vs ancient beings".to_string(),
            "Reality manipulation".to_string(),
        ]),
        ("crystallized_tears".to_string(), vec![
            "Empathy abilities".to_string(),
            "Companion bond strength".to_string(),
            "Emotional manipulation".to_string(),
        ]),
        ("dragon_whisper".to_string(), vec![
            "Dragon language".to_string(),
            "Ancient knowledge".to_string(),
            "Fear immunity".to_string(),
        ]),
    ])
}

/// The ultimate choice at the forge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForgeChoice {
    pub path_chosen: ForgePath,
    pub items_used: Vec<SentimentalItem>,
    pub sacrifice_made: SacrificeType,
    pub companion_sacrificed: Option<String>,
    pub resulting_gear: MythicGear,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MythicGear {
    pub name: String,
    pub tier: GearTier,
    pub gear_type: MythicGearType,
    pub base_power: f32,
    pub enhancements: Vec<String>,
    pub philosophy_bonus: Option<PhilosophyPath>,
    pub world_reaction: String, // How the world responds
}

/// Dragon's reaction to your gear
pub fn dragon_reaction(gear_tier: GearTier) -> &'static str {
    match gear_tier {
        GearTier::Mythic => "The dragon recognizes you as an equal, and speaks.",
        GearTier::Legendary => "The dragon acknowledges your power, but knows you're not ready.",
        _ => "The dragon dismisses you as another foolish mortal.",
    }
}
