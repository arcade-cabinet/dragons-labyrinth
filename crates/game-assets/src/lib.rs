//! Static game content for Dragon's Labyrinth
//! 
//! This crate is the single source of truth for all designed game content,
//! rules, and mechanics. It defines what the game IS, not how it runs.
//!
//! ## Architecture
//! 
//! - **Core Mechanics**: Traits, progression, philosophy paths
//! - **Horror System**: Dread levels and their effects
//! - **Characters**: Companion archetypes and NPCs
//! - **Visual Style**: Colors, typography, animations (dread-responsive)
//! - **Audio Rules**: Sound curves and horror progression
//! - **Forge System**: Sentimental items and endgame mechanics

pub mod traits;
pub mod progression;
pub mod dread;
pub mod characters;
pub mod philosophy;
pub mod forge;
pub mod visual;
pub mod levels;
pub mod combat;
pub mod companions;

// Re-export core types
pub use traits::{
    CharacterIdentity, EmergentTrait, TraitCategory, 
    WeaponMastery, ArmorAffinity, VillagerBackground,
    WorldPerception, Achievement
};
pub use progression::{
    PlayerAction, ProgressionTracker, TraitEmergenceRules,
    TraitEffects, CombatModifiers
};
pub use dread::{
    DreadInfluence, HorrorResistance, DreadTraitSynergies
};
pub use characters::CompanionArchetype;

use serde::{Deserialize, Serialize};

/// Global dread level that affects ALL game systems
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DreadLevel(pub u8);

impl DreadLevel {
    pub const PEACE: Self = Self(0);
    pub const UNEASE: Self = Self(1);
    pub const DREAD: Self = Self(2);
    pub const TERROR: Self = Self(3);
    pub const HORROR: Self = Self(4);
    
    /// Interpolation factor for smooth transitions
    pub fn factor(&self) -> f32 {
        self.0 as f32 / 4.0
    }
    
    /// Inverse factor (1.0 at peace, 0.0 at horror)
    pub fn inverse_factor(&self) -> f32 {
        1.0 - self.factor()
    }
    
    /// Get the narrative stage name
    pub fn stage_name(&self) -> &'static str {
        match self.0 {
            0 => "Peace",
            1 => "Unease", 
            2 => "Dread",
            3 => "Terror",
            4 => "Horror",
            _ => "Unknown",
        }
    }
    
    /// Get description of this dread stage
    pub fn description(&self) -> &'static str {
        match self.0 {
            0 => "The world is beautiful and full of hope",
            1 => "Something feels wrong, but you can't place it",
            2 => "The horror becomes undeniable",
            3 => "Reality itself begins to break down",
            4 => "You understand the truth, and it destroys you",
            _ => "Beyond comprehension",
        }
    }
}

/// Master content guide that provides all static game data
pub struct GameContent {
    dread_level: DreadLevel,
}

impl GameContent {
    pub fn new(dread_level: DreadLevel) -> Self {
        Self { dread_level }
    }
    
    /// Get visual style for current dread
    pub fn visual_style(&self) -> visual::VisualStyle {
        visual::VisualStyle::new(self.dread_level)
    }
    
    /// Get trait rules for current dread
    pub fn trait_rules(&self) -> progression::TraitEmergenceRules {
        progression::TraitEmergenceRules
    }
    
    /// Get dread influence on traits
    pub fn dread_influence(&self) -> dread::DreadInfluence {
        dread::DreadInfluence::for_level(self.dread_level.0)
    }
    
    /// Get horror resistance calculations
    pub fn calculate_horror_resistance(
        &self, 
        traits: &std::collections::HashMap<String, EmergentTrait>
    ) -> dread::HorrorResistance {
        dread::HorrorResistance::calculate(traits)
    }
}

/// The game's unique identity: classless progression
pub mod identity {
    use super::*;
    
    /// Core principle: You start as a villager and become what you do
    pub const CLASSLESS_PROGRESSION: &str = 
        "No character classes. Your identity emerges from your actions.";
    
    /// The journey IS the game
    pub const JOURNEY_FOCUS: &str = 
        "Like Frodo's walk to Mordor, the growing dread and transformation is the experience.";
    
    /// Horror-first design
    pub const HORROR_FIRST: &str = 
        "This is not an RPG with horror elements, but horror that uses RPG mechanics.";
}