//! Classless progression system - you start as a villager and become what you do
//!
//! This is the core uniqueness of Dragon's Labyrinth: no character classes.
//! Your identity emerges from your actions, choices, and the tools you use.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Core trait categories that define who you become
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TraitCategory {
    /// Combat style traits from weapon usage
    CombatStyle,
    /// Defense style from armor choices
    DefenseStyle,
    /// Moral alignment from choices
    MoralAlignment,
    /// Social reputation from interactions
    SocialReputation,
    /// Craft specialization from activities
    CraftSpecialization,
    /// Horror response from dread exposure
    HorrorResponse,
    /// Leadership from companion management
    Leadership,
    /// Exploration from world discovery
    Exploration,
}

/// Weapon mastery that develops through use
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WeaponMastery {
    /// No weapon preference yet
    Untrained,
    /// Sword usage creates disciplined fighter
    Swordsman { proficiency: f32, style: SwordStyle },
    /// Axe usage creates brutal warrior
    Axeman { proficiency: f32, style: AxeStyle },
    /// Bow usage creates precise hunter
    Archer { proficiency: f32, style: ArcheryStyle },
    /// Staff usage creates mystic defender
    StaffWielder { proficiency: f32, style: StaffStyle },
    /// Dagger usage creates quick assassin
    Rogue { proficiency: f32, style: DaggerStyle },
    /// Shield focus creates stalwart protector
    Guardian { proficiency: f32, style: ShieldStyle },
    /// Mixed weapons creates adaptable fighter
    Versatile { proficiencies: HashMap<String, f32> },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SwordStyle {
    Duelist,      // Precise, honorable combat
    Knight,       // Heavy strikes, protective
    Bladedancer,  // Fluid, artistic combat
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AxeStyle {
    Berserker,    // Rage-fueled combat
    Lumberjack,   // Methodical, powerful
    Executioner,  // Precise, devastating
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ArcheryStyle {
    Hunter,       // Patient, tracking
    Sniper,       // Long-range precision
    Skirmisher,   // Mobile harassment
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StaffStyle {
    Sage,         // Defensive, wise
    Wanderer,     // Travel-focused
    BattleMage,   // Combat-oriented
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DaggerStyle {
    Assassin,     // Stealth kills
    Surgeon,      // Precise strikes
    Trickster,    // Misdirection
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ShieldStyle {
    Sentinel,     // Immovable defense
    Protector,    // Ally-focused
    Bastion,      // Area control
}

/// Armor affinity that develops from protection choices
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ArmorAffinity {
    /// No armor preference
    Unarmored { agility: f32 },
    /// Light armor creates nimble fighter
    LightArmor { mobility: f32, stealth: f32 },
    /// Medium armor creates balanced warrior
    MediumArmor { flexibility: f32, durability: f32 },
    /// Heavy armor creates walking fortress
    HeavyArmor { protection: f32, intimidation: f32 },
    /// Robes create mystic presence
    Robes { wisdom: f32, presence: f32 },
    /// Mixed armor creates pragmatist
    Pragmatic { adaptability: f32 },
}

/// Emergent trait from accumulated actions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmergentTrait {
    pub name: String,
    pub category: TraitCategory,
    pub strength: f32,              // 0.0-1.0 how developed
    pub manifestations: Vec<String>, // How it shows in gameplay
    pub npc_reactions: Vec<String>, // How NPCs respond
    pub synergies: Vec<String>,     // Traits that enhance this
    pub conflicts: Vec<String>,     // Traits that oppose this
}

/// The complete progression identity of a character
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterIdentity {
    /// You always start as a simple villager
    pub origin: VillagerBackground,
    
    /// Current weapon mastery from usage
    pub weapon_mastery: WeaponMastery,
    
    /// Current armor affinity from choices
    pub armor_affinity: ArmorAffinity,
    
    /// All accumulated traits
    pub traits: HashMap<String, EmergentTrait>,
    
    /// How the world perceives you
    pub reputation: WorldPerception,
    
    /// Achievements that mirror your growth
    pub achievements: Vec<Achievement>,
    
    /// Your philosophical alignment (4-path system)
    pub philosophy: PhilosophicalIdentity,
}

/// Starting backgrounds - all are simple villagers
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VillagerBackground {
    Farmer,        // Worked the fields
    Apprentice,    // Helped a craftsman
    Orphan,        // Raised by village
    Merchant,      // Family traded goods
    Guard,         // Watched the walls
    Scholar,       // Read old books
    Hunter,        // Tracked game
    Healer,        // Tended the sick
}

impl VillagerBackground {
    pub fn starting_inclination(&self) -> &'static str {
        match self {
            Self::Farmer => "Endurance and patience",
            Self::Apprentice => "Attention to detail",
            Self::Orphan => "Self-reliance",
            Self::Merchant => "Social awareness",
            Self::Guard => "Vigilance",
            Self::Scholar => "Curiosity",
            Self::Hunter => "Tracking instinct",
            Self::Healer => "Empathy",
        }
    }
}

/// How NPCs and the world perceive you
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldPerception {
    /// General reputation descriptors
    pub descriptors: Vec<String>, // "The Kind Swordsman", "The Brutal Axeman"
    
    /// Faction standings
    pub faction_standings: HashMap<String, f32>,
    
    /// Notable deeds known by NPCs
    pub known_deeds: Vec<KnownDeed>,
    
    /// Fear/respect level
    pub fear_level: f32,    // 0.0-1.0 how much NPCs fear you
    pub respect_level: f32, // 0.0-1.0 how much NPCs respect you
    pub fame_level: f32,    // 0.0-1.0 how well-known you are
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnownDeed {
    pub deed: String,
    pub moral_weight: f32, // -1.0 evil to 1.0 good
    pub impact: f32,       // 0.0-1.0 how significant
    pub witnesses: Vec<String>, // Who saw/knows
}

/// Achievements are the mirror of traits - external recognition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Achievement {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: AchievementCategory,
    pub rarity: AchievementRarity,
    pub unlock_condition: String,
    pub trait_requirements: Vec<String>, // Traits needed
    pub world_impact: String, // How it changes NPC behavior
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AchievementCategory {
    CombatMastery,      // Weapon/combat achievements
    MoralChoice,        // Significant moral decisions
    CompanionBond,      // Relationship milestones
    WorldExploration,   // Discovery achievements
    HorrorSurvival,     // Surviving horror stages
    CraftMastery,       // Creating/forging items
    Reputation,         // Social standing milestones
    Philosophy,         // Philosophical path progress
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AchievementRarity {
    Common,    // Most players will get
    Uncommon,  // Requires some effort
    Rare,      // Difficult to achieve
    Epic,      // Very challenging
    Legendary, // Exceptional accomplishment
    Mythic,    // Near-impossible
}

/// Integration with the 4-path philosophy system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhilosophicalIdentity {
    pub strength_alignment: f32,  // 0.0-1.0
    pub harmony_alignment: f32,   // 0.0-1.0
    pub light_alignment: f32,      // 0.0-1.0
    pub dark_alignment: f32,       // 0.0-1.0
    pub dominant_path: PhilosophyPath,
    pub identity_coherence: f32,  // How consistent you are
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PhilosophyPath {
    Strength,  // Power through force
    Harmony,   // Balance and cooperation
    Light,     // Purity and selflessness
    Dark,      // Corruption and sacrifice
    Conflicted, // No clear path
}

/// Trait progression rules
impl EmergentTrait {
    /// Create a new trait from repeated actions
    pub fn emerge(name: &str, category: TraitCategory, initial_strength: f32) -> Self {
        Self {
            name: name.to_string(),
            category,
            strength: initial_strength.clamp(0.0, 1.0),
            manifestations: Vec::new(),
            npc_reactions: Vec::new(),
            synergies: Vec::new(),
            conflicts: Vec::new(),
        }
    }
    
    /// Strengthen trait through use
    pub fn reinforce(&mut self, amount: f32) {
        self.strength = (self.strength + amount).clamp(0.0, 1.0);
    }
    
    /// Weaken trait through opposing actions
    pub fn diminish(&mut self, amount: f32) {
        self.strength = (self.strength - amount).clamp(0.0, 1.0);
    }
    
    /// Check if trait is dominant (affects behavior)
    pub fn is_dominant(&self) -> bool {
        self.strength >= 0.7
    }
    
    /// Check if trait is emerging (starting to show)
    pub fn is_emerging(&self) -> bool {
        self.strength >= 0.3 && self.strength < 0.7
    }
}

/// NPC reaction templates based on traits
pub struct NPCReactionTemplates;

impl NPCReactionTemplates {
    pub fn to_swordsman(proficiency: f32) -> Vec<&'static str> {
        match proficiency {
            p if p < 0.3 => vec![
                "You hold that sword like a farmer holds a pitchfork.",
                "Still getting used to the weight, I see.",
            ],
            p if p < 0.7 => vec![
                "I see you've been practicing with that blade.",
                "A competent swordsman. Good to have around.",
            ],
            _ => vec![
                "Master swordsman! It's an honor.",
                "Your blade work is legendary in these parts.",
                "The way you move with that sword... magnificent.",
            ],
        }
    }
    
    pub fn to_brutal_fighter(brutality: f32) -> Vec<&'static str> {
        match brutality {
            b if b < 0.5 => vec![
                "You fight when you must. Respectable.",
                "Not one for unnecessary violence, I hope.",
            ],
            _ => vec![
                "I've heard about what you did. Stay away from me.",
                "Monster! You're no better than what we fight!",
                "Please... I have children...",
            ],
        }
    }
    
    pub fn to_kind_soul(kindness: f32) -> Vec<&'static str> {
        match kindness {
            k if k > 0.7 => vec![
                "You're too good for this dark world.",
                "Thank you for everything you've done for us.",
                "The children sing songs about your kindness.",
            ],
            _ => vec![
                "You seem decent enough.",
                "At least you're not cruel.",
            ],
        }
    }
}

/// Example trait definitions
pub fn define_core_traits() -> Vec<EmergentTrait> {
    vec![
        EmergentTrait {
            name: "Battle-Hardened".to_string(),
            category: TraitCategory::CombatStyle,
            strength: 0.0,
            manifestations: vec![
                "Reduced flinching from hits".to_string(),
                "Steadier aim under pressure".to_string(),
                "Calm dialogue options in combat".to_string(),
            ],
            npc_reactions: vec![
                "You've seen real combat, haven't you?".to_string(),
                "A veteran. Good to have you with us.".to_string(),
            ],
            synergies: vec!["Brave".to_string(), "Steadfast".to_string()],
            conflicts: vec!["Cowardly".to_string(), "Peaceful".to_string()],
        },
        EmergentTrait {
            name: "Merciful".to_string(),
            category: TraitCategory::MoralAlignment,
            strength: 0.0,
            manifestations: vec![
                "Non-lethal takedown options".to_string(),
                "Healing items appear more frequently".to_string(),
                "Enemies may surrender instead of fighting to death".to_string(),
            ],
            npc_reactions: vec![
                "You showed mercy when others wouldn't.".to_string(),
                "The kind-hearted warrior, they call you.".to_string(),
            ],
            synergies: vec!["Compassionate".to_string(), "Healer".to_string()],
            conflicts: vec!["Ruthless".to_string(), "Vengeful".to_string()],
        },
        EmergentTrait {
            name: "Shadow-Touched".to_string(),
            category: TraitCategory::HorrorResponse,
            strength: 0.0,
            manifestations: vec![
                "See better in darkness".to_string(),
                "Horror creatures hesitate before attacking".to_string(),
                "Corruption spreads slower".to_string(),
            ],
            npc_reactions: vec![
                "There's something dark about you...".to_string(),
                "You've stared into the void, haven't you?".to_string(),
                "Stay back! You reek of the corruption!".to_string(),
            ],
            synergies: vec!["Fearless".to_string(), "Void-Walker".to_string()],
            conflicts: vec!["Pure-Hearted".to_string(), "Light-Blessed".to_string()],
        },
    ]
}
