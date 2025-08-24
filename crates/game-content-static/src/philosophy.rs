//! The 4-path philosophical framework
//!
//! Players don't choose a class - they develop a philosophy through choices.
//! This is the heart of the emergent identity system.

use crate::traits::EmergentTrait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The four philosophical paths
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum PhilosophyPath {
    /// Path of physical power and dominance
    Strength,
    /// Path of balance and cooperation
    Harmony,
    /// Path of purity and selflessness
    Light,
    /// Path of corruption and sacrifice
    Dark,
}

impl PhilosophyPath {
    pub fn description(&self) -> &'static str {
        match self {
            Self::Strength => "Power through force, command through fear",
            Self::Harmony => "Balance in all things, strength through unity",
            Self::Light => "Purity of purpose, salvation through sacrifice",
            Self::Dark => "Embrace corruption, transcend mortality",
        }
    }
    
    pub fn mechanical_benefits(&self) -> PhilosophyBenefits {
        match self {
            Self::Strength => PhilosophyBenefits {
                combat_modifier: 1.3,
                companion_limit: 6, // Can command more
                fear_generation: 0.5,
                respect_generation: 0.3,
                special_ability: "Intimidation works on bosses".to_string(),
            },
            Self::Harmony => PhilosophyBenefits {
                combat_modifier: 1.0,
                companion_limit: 4,
                fear_generation: -0.2,
                respect_generation: 0.8,
                special_ability: "Companions never betray".to_string(),
            },
            Self::Light => PhilosophyBenefits {
                combat_modifier: 0.9,
                companion_limit: 3,
                fear_generation: -0.5,
                respect_generation: 1.0,
                special_ability: "Healing miracles available".to_string(),
            },
            Self::Dark => PhilosophyBenefits {
                combat_modifier: 1.5,
                companion_limit: 2, // Corruption isolates
                fear_generation: 1.0,
                respect_generation: -0.3,
                special_ability: "Can consume companions for power".to_string(),
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhilosophyBenefits {
    pub combat_modifier: f32,
    pub companion_limit: u32,
    pub fear_generation: f32,
    pub respect_generation: f32,
    pub special_ability: String,
}

/// Player's philosophical development
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhilosophicalIdentity {
    /// Score on each path (0.0-1.0)
    pub strength_score: f32,
    pub harmony_score: f32,
    pub light_score: f32,
    pub dark_score: f32,
    
    /// Which path is dominant
    pub dominant_path: Option<PhilosophyPath>,
    
    /// Secondary influence
    pub secondary_path: Option<PhilosophyPath>,
    
    /// How conflicted the player is
    pub internal_conflict: f32,
    
    /// Stability of identity
    pub identity_stability: f32,
    
    /// Key moments that defined philosophy
    pub defining_moments: Vec<DefiningMoment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefiningMoment {
    pub description: String,
    pub path_influenced: PhilosophyPath,
    pub magnitude: f32,
    pub act: u8, // Which act (1-3)
}

impl PhilosophicalIdentity {
    pub fn new() -> Self {
        Self {
            strength_score: 0.0,
            harmony_score: 0.0,
            light_score: 0.0,
            dark_score: 0.0,
            dominant_path: None,
            secondary_path: None,
            internal_conflict: 0.0,
            identity_stability: 1.0,
            defining_moments: Vec::new(),
        }
    }
    
    /// Update philosophy based on action
    pub fn apply_action(&mut self, action: PhilosophicalAction) {
        match action.path {
            PhilosophyPath::Strength => self.strength_score += action.magnitude,
            PhilosophyPath::Harmony => self.harmony_score += action.magnitude,
            PhilosophyPath::Light => self.light_score += action.magnitude,
            PhilosophyPath::Dark => self.dark_score += action.magnitude,
        }
        
        // Opposing philosophies weaken
        match action.path {
            PhilosophyPath::Strength => self.harmony_score -= action.magnitude * 0.5,
            PhilosophyPath::Harmony => self.strength_score -= action.magnitude * 0.5,
            PhilosophyPath::Light => self.dark_score -= action.magnitude * 0.5,
            PhilosophyPath::Dark => self.light_score -= action.magnitude * 0.5,
        }
        
        // Clamp values
        self.strength_score = self.strength_score.clamp(0.0, 1.0);
        self.harmony_score = self.harmony_score.clamp(0.0, 1.0);
        self.light_score = self.light_score.clamp(0.0, 1.0);
        self.dark_score = self.dark_score.clamp(0.0, 1.0);
        
        // Recalculate dominant path
        self.recalculate_dominant_path();
        
        // Track defining moments
        if action.magnitude > 0.3 {
            self.defining_moments.push(DefiningMoment {
                description: action.description,
                path_influenced: action.path,
                magnitude: action.magnitude,
                act: action.act,
            });
        }
    }
    
    fn recalculate_dominant_path(&mut self) {
        let scores = [
            (PhilosophyPath::Strength, self.strength_score),
            (PhilosophyPath::Harmony, self.harmony_score),
            (PhilosophyPath::Light, self.light_score),
            (PhilosophyPath::Dark, self.dark_score),
        ];
        
        let mut sorted = scores.to_vec();
        sorted.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        
        // Set dominant path if clear leader
        if sorted[0].1 > 0.3 {
            self.dominant_path = Some(sorted[0].0);
        }
        
        // Set secondary if significant
        if sorted[1].1 > 0.2 {
            self.secondary_path = Some(sorted[1].0);
        }
        
        // Calculate internal conflict
        let spread = sorted[0].1 - sorted[3].1;
        self.internal_conflict = 1.0 - spread; // High when scores are close
        
        // Calculate stability
        self.identity_stability = sorted[0].1 / (sorted[0].1 + sorted[1].1 + 0.01);
    }
    
    /// Get unique ending based on philosophy
    pub fn get_ending_variant(&self) -> EndingVariant {
        match (self.dominant_path, self.secondary_path) {
            (Some(PhilosophyPath::Strength), _) => EndingVariant::Conqueror,
            (Some(PhilosophyPath::Harmony), Some(PhilosophyPath::Light)) => EndingVariant::Savior,
            (Some(PhilosophyPath::Harmony), _) => EndingVariant::Peacemaker,
            (Some(PhilosophyPath::Light), _) => EndingVariant::Martyr,
            (Some(PhilosophyPath::Dark), Some(PhilosophyPath::Strength)) => EndingVariant::Tyrant,
            (Some(PhilosophyPath::Dark), _) => EndingVariant::Corrupted,
            (None, _) => EndingVariant::Lost,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhilosophicalAction {
    pub path: PhilosophyPath,
    pub magnitude: f32,
    pub description: String,
    pub act: u8,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum EndingVariant {
    Conqueror,  // Strength dominant
    Peacemaker, // Harmony dominant
    Savior,     // Harmony + Light
    Martyr,     // Light dominant
    Tyrant,     // Dark + Strength
    Corrupted,  // Dark dominant
    Lost,       // No clear path
}

/// Act structure and transitions
pub mod acts {
    use super::*;
    
    /// Act 1: Journey TO Labyrinth (establishing identity)
    pub const ACT1_TRANSITIONS: &[&str] = &[
        "The Abandoned Cart",      // First moral choice
        "The Burning Village",     // Save or flee
        "The Starving Refugees",   // Share or hoard
        "The Bridge Troll",        // Fight or negotiate
        "The Wounded Soldier",     // Help enemy or leave
        "The Fork in the Road",    // Choose companion's fate
    ];
    
    /// Act 2: Fighting the Dragon (testing philosophy)
    pub const ACT2_TRANSITIONS: &[&str] = &[
        "The Dragon's Offer",      // Power for service
        "The Companion's Plea",    // Sacrifice request
        "The Innocent Hostage",    // Save at cost
        "The Final Betrayal",      // Trust broken
    ];
    
    /// Act 3: Sealing the Void (consequences)
    pub const ACT3_TRANSITIONS: &[&str] = &[
        "The Price of Victory",    // What you've become
        "The World's Judgment",    // How others see you
    ];
}
