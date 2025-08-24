//! Dread system integration with traits and progression
//!
//! This module defines how the horror progression affects trait development
//! and how traits influence horror resistance/vulnerability.

use crate::traits::*;
use crate::progression::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// How dread affects trait development
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DreadInfluence {
    /// Current dread level (0-4)
    pub level: u8,
    
    /// Traits that strengthen at this dread level
    pub strengthened_traits: Vec<String>,
    
    /// Traits that weaken at this dread level
    pub weakened_traits: Vec<String>,
    
    /// New traits that can emerge at this level
    pub emergent_traits: Vec<String>,
    
    /// Trait development multipliers
    pub development_multipliers: HashMap<String, f32>,
}

impl DreadInfluence {
    pub fn for_level(level: u8) -> Self {
        match level {
            0 => Self::peace(),
            1 => Self::unease(),
            2 => Self::anxiety(),
            3 => Self::terror(),
            4 => Self::horror(),
            _ => Self::peace(),
        }
    }
    
    fn peace() -> Self {
        Self {
            level: 0,
            strengthened_traits: vec![
                "Optimistic".to_string(),
                "Trusting".to_string(),
                "Compassionate".to_string(),
                "Helpful".to_string(),
            ],
            weakened_traits: vec![
                "Paranoid".to_string(),
                "Fearful".to_string(),
                "Cynical".to_string(),
            ],
            emergent_traits: vec![
                "Innocent".to_string(),
                "Naive".to_string(),
                "Peaceful".to_string(),
            ],
            development_multipliers: HashMap::from([
                ("Compassionate".to_string(), 1.5),
                ("Helpful".to_string(), 1.3),
                ("Battle-Hardened".to_string(), 0.5), // Harder to develop in peace
            ]),
        }
    }
    
    fn unease() -> Self {
        Self {
            level: 1,
            strengthened_traits: vec![
                "Vigilant".to_string(),
                "Cautious".to_string(),
                "Observant".to_string(),
            ],
            weakened_traits: vec![
                "Naive".to_string(),
                "Careless".to_string(),
            ],
            emergent_traits: vec![
                "Suspicious".to_string(),
                "Alert".to_string(),
                "Questioning".to_string(),
            ],
            development_multipliers: HashMap::from([
                ("Vigilant".to_string(), 1.4),
                ("Paranoid".to_string(), 1.2),
                ("Trusting".to_string(), 0.8),
            ]),
        }
    }
    
    fn anxiety() -> Self {
        Self {
            level: 2,
            strengthened_traits: vec![
                "Survivor".to_string(),
                "Resourceful".to_string(),
                "Battle-Hardened".to_string(),
            ],
            weakened_traits: vec![
                "Optimistic".to_string(),
                "Peaceful".to_string(),
            ],
            emergent_traits: vec![
                "Stress-Tested".to_string(),
                "Hardened".to_string(),
                "Pragmatic".to_string(),
            ],
            development_multipliers: HashMap::from([
                ("Battle-Hardened".to_string(), 1.5),
                ("Survivor".to_string(), 1.4),
                ("Compassionate".to_string(), 0.7), // Harder to stay compassionate
            ]),
        }
    }
    
    fn terror() -> Self {
        Self {
            level: 3,
            strengthened_traits: vec![
                "Desperate".to_string(),
                "Ruthless".to_string(),
                "Broken".to_string(),
            ],
            weakened_traits: vec![
                "Merciful".to_string(),
                "Hopeful".to_string(),
                "Stable".to_string(),
            ],
            emergent_traits: vec![
                "Traumatized".to_string(),
                "Hollow".to_string(),
                "Cold".to_string(),
            ],
            development_multipliers: HashMap::from([
                ("Ruthless".to_string(), 2.0), // Develops quickly under pressure
                ("Broken".to_string(), 1.8),
                ("Merciful".to_string(), 0.3), // Very hard to maintain
                ("Hopeful".to_string(), 0.2),
            ]),
        }
    }
    
    fn horror() -> Self {
        Self {
            level: 4,
            strengthened_traits: vec![
                "Shadow-Touched".to_string(),
                "Void-Walker".to_string(),
                "Mad".to_string(),
                "Transcendent".to_string(),
            ],
            weakened_traits: vec![
                "Human".to_string(), // Losing humanity
                "Rational".to_string(),
                "Grounded".to_string(),
            ],
            emergent_traits: vec![
                "Dragon-Touched".to_string(),
                "Eldritch-Mind".to_string(),
                "Beyond-Mortal".to_string(),
            ],
            development_multipliers: HashMap::from([
                ("Shadow-Touched".to_string(), 3.0), // Rapid transformation
                ("Mad".to_string(), 2.5),
                ("Human".to_string(), 0.1), // Almost impossible to maintain
            ]),
        }
    }
}

/// How traits affect horror resistance/vulnerability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HorrorResistance {
    /// Base resistance from traits
    pub base_resistance: f32,
    
    /// Resistance modifiers from specific traits
    pub trait_modifiers: HashMap<String, f32>,
    
    /// Vulnerability multipliers
    pub vulnerabilities: HashMap<String, f32>,
    
    /// Special immunities from traits
    pub immunities: Vec<HorrorImmunity>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HorrorImmunity {
    FearEffects,        // Immune to fear
    SanityDrain,        // Immune to sanity loss
    Corruption,         // Immune to corruption
    Hallucinations,     // Can't hallucinate
    CompanionBetrayal,  // Companions won't betray
    DreadProgression,   // Dread doesn't increase
}

impl HorrorResistance {
    pub fn calculate(traits: &HashMap<String, EmergentTrait>) -> Self {
        let mut resistance = Self {
            base_resistance: 0.0,
            trait_modifiers: HashMap::new(),
            vulnerabilities: HashMap::new(),
            immunities: Vec::new(),
        };
        
        for (name, trait_data) in traits {
            if !trait_data.is_emerging() {
                continue;
            }
            
            let strength = trait_data.strength;
            
            match name.as_str() {
                // Resistance traits
                "Fearless" => {
                    resistance.trait_modifiers.insert(name.clone(), strength * 0.3);
                    if strength >= 0.9 {
                        resistance.immunities.push(HorrorImmunity::FearEffects);
                    }
                }
                "Shadow-Touched" => {
                    resistance.trait_modifiers.insert(name.clone(), strength * 0.4);
                    if strength >= 0.8 {
                        resistance.immunities.push(HorrorImmunity::Corruption);
                    }
                }
                "Mad" => {
                    // Madness protects from further madness
                    resistance.trait_modifiers.insert(name.clone(), strength * 0.5);
                    if strength >= 0.7 {
                        resistance.immunities.push(HorrorImmunity::SanityDrain);
                    }
                }
                "Battle-Hardened" => {
                    resistance.trait_modifiers.insert(name.clone(), strength * 0.2);
                }
                
                // Vulnerability traits
                "Innocent" => {
                    resistance.vulnerabilities.insert(name.clone(), strength * 1.5);
                }
                "Broken" => {
                    resistance.vulnerabilities.insert(name.clone(), strength * 1.3);
                }
                "Compassionate" => {
                    // Compassion makes horror harder to bear
                    resistance.vulnerabilities.insert(name.clone(), strength * 1.2);
                }
                _ => {}
            }
        }
        
        // Calculate base resistance
        let total_resistance: f32 = resistance.trait_modifiers.values().sum();
        let total_vulnerability: f32 = resistance.vulnerabilities.values().sum();
        resistance.base_resistance = (total_resistance - total_vulnerability).max(0.0);
        
        resistance
    }
}

/// Dread-specific trait interactions
pub struct DreadTraitSynergies;

impl DreadTraitSynergies {
    /// Get synergistic trait combinations for current dread
    pub fn get_synergies(dread_level: u8) -> Vec<TraitSynergy> {
        match dread_level {
            0 => vec![
                TraitSynergy {
                    traits: vec!["Compassionate".to_string(), "Helpful".to_string()],
                    name: "Village Hero".to_string(),
                    bonus: 0.2,
                    description: "Beloved by peaceful villagers".to_string(),
                },
            ],
            1 => vec![
                TraitSynergy {
                    traits: vec!["Vigilant".to_string(), "Observant".to_string()],
                    name: "Watchful Guardian".to_string(),
                    bonus: 0.25,
                    description: "Nothing escapes your notice".to_string(),
                },
            ],
            2 => vec![
                TraitSynergy {
                    traits: vec!["Battle-Hardened".to_string(), "Survivor".to_string()],
                    name: "Veteran Survivor".to_string(),
                    bonus: 0.3,
                    description: "You've seen it all and lived".to_string(),
                },
            ],
            3 => vec![
                TraitSynergy {
                    traits: vec!["Ruthless".to_string(), "Cold".to_string()],
                    name: "Ice in Your Veins".to_string(),
                    bonus: 0.35,
                    description: "Nothing phases you anymore".to_string(),
                },
                TraitSynergy {
                    traits: vec!["Broken".to_string(), "Hollow".to_string()],
                    name: "Empty Shell".to_string(),
                    bonus: 0.4,
                    description: "There's nothing left to lose".to_string(),
                },
            ],
            4 => vec![
                TraitSynergy {
                    traits: vec!["Shadow-Touched".to_string(), "Void-Walker".to_string()],
                    name: "One with Darkness".to_string(),
                    bonus: 0.5,
                    description: "The void recognizes its own".to_string(),
                },
                TraitSynergy {
                    traits: vec!["Mad".to_string(), "Transcendent".to_string()],
                    name: "Beyond Comprehension".to_string(),
                    bonus: 0.6,
                    description: "Your mind operates on different rules".to_string(),
                },
            ],
            _ => vec![],
        }
    }
    
    /// Get trait conflicts for current dread
    pub fn get_conflicts(dread_level: u8) -> Vec<TraitConflict> {
        match dread_level {
            0..=1 => vec![
                TraitConflict {
                    traits: vec!["Trusting".to_string(), "Paranoid".to_string()],
                    name: "Conflicted Mind".to_string(),
                    penalty: 0.2,
                    description: "You can't decide who to trust".to_string(),
                },
            ],
            2..=3 => vec![
                TraitConflict {
                    traits: vec!["Compassionate".to_string(), "Ruthless".to_string()],
                    name: "Moral Turmoil".to_string(),
                    penalty: 0.3,
                    description: "Your conscience wars with necessity".to_string(),
                },
                TraitConflict {
                    traits: vec!["Hopeful".to_string(), "Broken".to_string()],
                    name: "Shattered Hope".to_string(),
                    penalty: 0.35,
                    description: "Hope hurts more than despair".to_string(),
                },
            ],
            4 => vec![
                TraitConflict {
                    traits: vec!["Human".to_string(), "Shadow-Touched".to_string()],
                    name: "Fading Humanity".to_string(),
                    penalty: 0.5,
                    description: "Your humanity slips away".to_string(),
                },
            ],
            _ => vec![],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraitSynergy {
    pub traits: Vec<String>,
    pub name: String,
    pub bonus: f32,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraitConflict {
    pub traits: Vec<String>,
    pub name: String,
    pub penalty: f32,
    pub description: String,
}
