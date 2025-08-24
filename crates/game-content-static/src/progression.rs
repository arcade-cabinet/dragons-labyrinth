//! Progression mechanics - how a villager becomes a hero (or monster)
//!
//! This module defines HOW traits develop, not just what they are.
//! Every action feeds into the progression system.

use crate::traits::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Action categories that influence trait development
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PlayerAction {
    /// Combat actions
    Combat {
        weapon_used: String,
        style: CombatStyle,
        target: String,
        outcome: CombatOutcome,
    },
    /// Dialogue choices
    Dialogue {
        choice_type: DialogueChoice,
        moral_weight: f32, // -1.0 evil to 1.0 good
        companion_present: Option<String>,
    },
    /// Equipment changes
    Equipment {
        item_type: EquipmentType,
        item_name: String,
        duration_worn: f32, // How long used
    },
    /// Exploration activities
    Exploration {
        discovery_type: DiscoveryType,
        danger_level: f32,
    },
    /// Crafting/forge activities
    Crafting {
        item_created: String,
        materials_used: Vec<String>,
        quality: ItemQuality,
    },
    /// Companion interactions
    CompanionInteraction {
        companion: String,
        interaction_type: InteractionType,
        bond_change: f32,
    },
    /// Horror responses
    HorrorResponse {
        dread_level: u8,
        response_type: HorrorResponseType,
        sanity_impact: f32,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CombatStyle {
    Aggressive,   // Direct assault
    Defensive,    // Cautious approach
    Tactical,     // Strategic positioning
    Brutal,       // Excessive force
    Merciful,     // Non-lethal
    Desperate,    // Panicked fighting
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CombatOutcome {
    Victory,
    Defeat,
    Fled,
    Negotiated,
    Spared,
    Executed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DialogueChoice {
    Aggressive,
    Diplomatic,
    Deceptive,
    Compassionate,
    Intimidating,
    Humorous,
    Silent,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EquipmentType {
    Weapon,
    Armor,
    Accessory,
    Consumable,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DiscoveryType {
    Location,
    Secret,
    Lore,
    Danger,
    Treasure,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ItemQuality {
    Poor,
    Standard,
    Fine,
    Exceptional,
    Masterwork,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum InteractionType {
    Support,
    Challenge,
    Gift,
    Request,
    Comfort,
    Abandon,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HorrorResponseType {
    Brave,
    Terrified,
    Curious,
    Protective,
    Breakdown,
    Acceptance,
}

/// Tracks progression over time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressionTracker {
    /// Total actions of each type
    pub action_counts: HashMap<String, u32>,
    
    /// Recent actions (last 50) for momentum
    pub recent_actions: Vec<PlayerAction>,
    
    /// Trait development momentum
    pub trait_momentum: HashMap<String, f32>,
    
    /// Weapon usage statistics
    pub weapon_usage: HashMap<String, WeaponUsageStats>,
    
    /// Armor worn duration
    pub armor_worn: HashMap<String, f32>,
    
    /// Moral choice history
    pub moral_history: Vec<MoralChoice>,
    
    /// NPC interaction history
    pub npc_interactions: HashMap<String, NPCRelationship>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeaponUsageStats {
    pub total_swings: u32,
    pub hits_landed: u32,
    pub critical_hits: u32,
    pub enemies_defeated: u32,
    pub combat_duration: f32,
    pub preferred_style: CombatStyle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoralChoice {
    pub description: String,
    pub weight: f32, // -1.0 to 1.0
    pub witnesses: Vec<String>,
    pub consequences: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NPCRelationship {
    pub interactions: u32,
    pub relationship_level: f32, // -1.0 hostile to 1.0 beloved
    pub known_traits: Vec<String>, // Traits this NPC has observed
    pub reputation_contribution: f32,
}

/// Rules for how actions create traits
pub struct TraitEmergenceRules;

impl TraitEmergenceRules {
    /// Calculate trait emergence from actions
    pub fn calculate_trait_emergence(
        action: &PlayerAction,
        current_traits: &HashMap<String, EmergentTrait>,
    ) -> Vec<TraitUpdate> {
        let mut updates = Vec::new();
        
        match action {
            PlayerAction::Combat { weapon_used, style, outcome, .. } => {
                // Weapon mastery progression
                updates.push(TraitUpdate {
                    trait_name: format!("{}_mastery", weapon_used),
                    category: TraitCategory::CombatStyle,
                    change: 0.02, // Slow, steady progression
                });
                
                // Style-based traits
                match style {
                    CombatStyle::Aggressive => {
                        updates.push(TraitUpdate {
                            trait_name: "Aggressive Fighter".to_string(),
                            category: TraitCategory::CombatStyle,
                            change: 0.03,
                        });
                    }
                    CombatStyle::Merciful => {
                        updates.push(TraitUpdate {
                            trait_name: "Merciful".to_string(),
                            category: TraitCategory::MoralAlignment,
                            change: 0.05,
                        });
                        updates.push(TraitUpdate {
                            trait_name: "Ruthless".to_string(),
                            category: TraitCategory::MoralAlignment,
                            change: -0.05, // Opposing trait weakens
                        });
                    }
                    CombatStyle::Brutal => {
                        updates.push(TraitUpdate {
                            trait_name: "Brutal".to_string(),
                            category: TraitCategory::CombatStyle,
                            change: 0.04,
                        });
                        updates.push(TraitUpdate {
                            trait_name: "Merciful".to_string(),
                            category: TraitCategory::MoralAlignment,
                            change: -0.04,
                        });
                    }
                    _ => {}
                }
                
                // Outcome-based traits
                match outcome {
                    CombatOutcome::Victory => {
                        updates.push(TraitUpdate {
                            trait_name: "Battle-Hardened".to_string(),
                            category: TraitCategory::CombatStyle,
                            change: 0.01,
                        });
                    }
                    CombatOutcome::Fled => {
                        updates.push(TraitUpdate {
                            trait_name: "Survivor".to_string(),
                            category: TraitCategory::CombatStyle,
                            change: 0.02,
                        });
                        updates.push(TraitUpdate {
                            trait_name: "Cowardly".to_string(),
                            category: TraitCategory::SocialReputation,
                            change: 0.01,
                        });
                    }
                    _ => {}
                }
            }
            
            PlayerAction::Dialogue { choice_type, moral_weight, .. } => {
                match choice_type {
                    DialogueChoice::Compassionate => {
                        updates.push(TraitUpdate {
                            trait_name: "Compassionate".to_string(),
                            category: TraitCategory::MoralAlignment,
                            change: 0.04,
                        });
                    }
                    DialogueChoice::Intimidating => {
                        updates.push(TraitUpdate {
                            trait_name: "Intimidating".to_string(),
                            category: TraitCategory::SocialReputation,
                            change: 0.03,
                        });
                    }
                    DialogueChoice::Deceptive => {
                        updates.push(TraitUpdate {
                            trait_name: "Deceptive".to_string(),
                            category: TraitCategory::SocialReputation,
                            change: 0.03,
                        });
                        updates.push(TraitUpdate {
                            trait_name: "Trustworthy".to_string(),
                            category: TraitCategory::SocialReputation,
                            change: -0.03,
                        });
                    }
                    _ => {}
                }
                
                // Moral weight influences alignment
                if *moral_weight > 0.5 {
                    updates.push(TraitUpdate {
                        trait_name: "Good-Hearted".to_string(),
                        category: TraitCategory::MoralAlignment,
                        change: moral_weight * 0.02,
                    });
                } else if *moral_weight < -0.5 {
                    updates.push(TraitUpdate {
                        trait_name: "Dark-Hearted".to_string(),
                        category: TraitCategory::MoralAlignment,
                        change: moral_weight.abs() * 0.02,
                    });
                }
            }
            
            PlayerAction::HorrorResponse { dread_level, response_type, .. } => {
                match response_type {
                    HorrorResponseType::Brave => {
                        updates.push(TraitUpdate {
                            trait_name: "Fearless".to_string(),
                            category: TraitCategory::HorrorResponse,
                            change: 0.03 * (*dread_level as f32 / 4.0), // Higher dread = more trait gain
                        });
                    }
                    HorrorResponseType::Breakdown => {
                        updates.push(TraitUpdate {
                            trait_name: "Broken".to_string(),
                            category: TraitCategory::HorrorResponse,
                            change: 0.05,
                        });
                        updates.push(TraitUpdate {
                            trait_name: "Resilient".to_string(),
                            category: TraitCategory::HorrorResponse,
                            change: -0.05,
                        });
                    }
                    HorrorResponseType::Acceptance => {
                        updates.push(TraitUpdate {
                            trait_name: "Shadow-Touched".to_string(),
                            category: TraitCategory::HorrorResponse,
                            change: 0.04,
                        });
                    }
                    _ => {}
                }
            }
            
            _ => {} // Other action types
        }
        
        updates
    }
    
    /// Determine if a trait should trigger an achievement
    pub fn check_achievement_unlock(
        trait_name: &str,
        trait_strength: f32,
        action_count: u32,
    ) -> Option<Achievement> {
        match (trait_name, trait_strength) {
            ("sword_mastery", s) if s >= 0.9 => Some(Achievement {
                id: "master_swordsman".to_string(),
                name: "Master of the Blade".to_string(),
                description: "Achieved true mastery with the sword".to_string(),
                category: AchievementCategory::CombatMastery,
                rarity: AchievementRarity::Epic,
                unlock_condition: "90% sword mastery".to_string(),
                trait_requirements: vec!["sword_mastery".to_string()],
                world_impact: "Guards salute you, enemies hesitate".to_string(),
            }),
            
            ("Merciful", s) if s >= 0.8 => Some(Achievement {
                id: "angel_of_mercy".to_string(),
                name: "Angel of Mercy".to_string(),
                description: "Showed consistent compassion in a dark world".to_string(),
                category: AchievementCategory::MoralChoice,
                rarity: AchievementRarity::Rare,
                unlock_condition: "80% merciful trait".to_string(),
                trait_requirements: vec!["Merciful".to_string()],
                world_impact: "Enemies may surrender, NPCs trust you more".to_string(),
            }),
            
            ("Shadow-Touched", s) if s >= 0.7 => Some(Achievement {
                id: "void_walker".to_string(),
                name: "Walker Between Worlds".to_string(),
                description: "Embraced the darkness without losing yourself".to_string(),
                category: AchievementCategory::HorrorSurvival,
                rarity: AchievementRarity::Epic,
                unlock_condition: "70% shadow-touched trait".to_string(),
                trait_requirements: vec!["Shadow-Touched".to_string()],
                world_impact: "Horror creatures recognize you as kin".to_string(),
            }),
            
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TraitUpdate {
    pub trait_name: String,
    pub category: TraitCategory,
    pub change: f32, // Positive reinforces, negative diminishes
}

/// How traits influence gameplay
pub struct TraitEffects;

impl TraitEffects {
    /// Get combat modifiers from traits
    pub fn get_combat_modifiers(traits: &HashMap<String, EmergentTrait>) -> CombatModifiers {
        let mut modifiers = CombatModifiers::default();
        
        for (name, trait_data) in traits {
            if !trait_data.is_emerging() {
                continue; // Only emerging or dominant traits have effects
            }
            
            let strength = trait_data.strength;
            
            match name.as_str() {
                "Battle-Hardened" => {
                    modifiers.damage_reduction += strength * 0.1;
                    modifiers.stamina_efficiency += strength * 0.15;
                }
                "Aggressive Fighter" => {
                    modifiers.attack_speed += strength * 0.2;
                    modifiers.damage_bonus += strength * 0.15;
                    modifiers.defense_penalty += strength * 0.1;
                }
                "sword_mastery" => {
                    modifiers.accuracy += strength * 0.2;
                    modifiers.critical_chance += strength * 0.1;
                }
                "Fearless" => {
                    modifiers.horror_resistance += strength * 0.3;
                    modifiers.morale_bonus += strength * 0.2;
                }
                _ => {}
            }
        }
        
        modifiers
    }
    
    /// Get dialogue options from traits
    pub fn get_dialogue_options(
        traits: &HashMap<String, EmergentTrait>,
        base_options: Vec<String>,
    ) -> Vec<DialogueOption> {
        let mut options = Vec::new();
        
        // Add base options
        for text in base_options {
            options.push(DialogueOption {
                text,
                trait_requirement: None,
                trait_modifier: None,
            });
        }
        
        // Add trait-specific options
        for (name, trait_data) in traits {
            if trait_data.is_dominant() {
                match name.as_str() {
                    "Intimidating" => {
                        options.push(DialogueOption {
                            text: "[Intimidate] You will tell me, or you will suffer.".to_string(),
                            trait_requirement: Some("Intimidating".to_string()),
                            trait_modifier: Some(trait_data.strength),
                        });
                    }
                    "Compassionate" => {
                        options.push(DialogueOption {
                            text: "[Compassion] I understand your pain. Let me help.".to_string(),
                            trait_requirement: Some("Compassionate".to_string()),
                            trait_modifier: Some(trait_data.strength),
                        });
                    }
                    "Deceptive" => {
                        options.push(DialogueOption {
                            text: "[Lie] Of course, I'm on your side. Trust me.".to_string(),
                            trait_requirement: Some("Deceptive".to_string()),
                            trait_modifier: Some(trait_data.strength),
                        });
                    }
                    _ => {}
                }
            }
        }
        
        options
    }
}

#[derive(Debug, Clone, Default)]
pub struct CombatModifiers {
    pub damage_bonus: f32,
    pub damage_reduction: f32,
    pub attack_speed: f32,
    pub accuracy: f32,
    pub critical_chance: f32,
    pub defense_penalty: f32,
    pub stamina_efficiency: f32,
    pub horror_resistance: f32,
    pub morale_bonus: f32,
}

#[derive(Debug, Clone)]
pub struct DialogueOption {
    pub text: String,
    pub trait_requirement: Option<String>,
    pub trait_modifier: Option<f32>, // Success chance modifier
}
