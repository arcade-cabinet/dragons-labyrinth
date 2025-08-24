//! Opening Sequence (Levels 1-20) for Dragon's Labyrinth
//! The foundation of the entire emotional journey

use bevy::prelude::*;
use bevy_yoleck::prelude::*;
use crate::components::{
    companions::{Companion, CompanionArchetype},
    narrative::{DialogueRunner, Quest, MoralChoice},
    core::{Player, HexPosition},
    world::{HexTile, TerrainType},
};

/// Level 1: The Door That Changes Everything
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, YoleckComponent)]
pub struct Level1_TheDoor {
    pub mother_dialogue: String,
    pub companion_choice: CompanionChoice,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompanionChoice {
    Elena,   // "I need a friend"
    Marcus,  // "We'll split the glory"
    Quinn,   // "Follow my lead"
}

impl Level1_TheDoor {
    pub fn setup(mut commands: Commands) {
        // Mother's house - starting position
        commands.spawn((
            Name::new("Mother's House"),
            HexPosition::new(0, 0),
            DialogueRunner::new("mother_plea.yarn"),
        ));
        
        // The Door - the most important choice
        commands.spawn((
            Name::new("The Door"),
            HexPosition::new(0, 1),
            MoralChoice {
                id: "companion_choice".to_string(),
                title: "Your childhood friend waits outside".to_string(),
                options: vec![
                    ("Come with me, I need a friend".to_string(), CompanionArchetype::Elena),
                    ("Come with me, we'll split the glory".to_string(), CompanionArchetype::Marcus),
                    ("Come with me, but follow my lead".to_string(), CompanionArchetype::Quinn),
                ],
                permanent: true,
                affects_philosophy: true,
            },
        ));
        
        // Father's Legacy - motivation seed
        commands.spawn((
            Name::new("Father's Journal Page"),
            HexPosition::new(1, 0),
            Quest {
                id: "fathers_legacy".to_string(),
                title: "Father's Final Words".to_string(),
                description: "The birds stopped singing. Just like before. Someone must investigate.".to_string(),
                stages: vec![
                    "Read father's journal".to_string(),
                    "Leave the village".to_string(),
                    "Investigate the silence".to_string(),
                ],
                current_stage: 0,
                moral_weight: 1.0,
                affects_ending: true,
            },
        ));
    }
}

/// Levels 1-3: Natural Combat Learning
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, YoleckComponent)]
pub struct Level1to3_PathToVillage {
    pub combat_variants: Vec<WolfVariant>,
    pub weather_effects: Vec<WeatherType>,
    pub hill_choice: HillChoice,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WolfVariant {
    Starving {   // Desperate, aggressive, low HP
        health: f32,
        aggression: f32,
        loot: Vec<String>,
    },
    Mother {     // Protecting cubs, defensive, moral choice
        cubs_present: bool,
        defensive_bonus: f32,
        moral_impact: f32,
    },
    Rabid {      // Void-touched, unpredictable patterns
        corruption_level: f32,
        pattern_chaos: f32,
        void_whispers: bool,
    },
    Pack {       // Coordinated attacks, flanking lessons
        pack_size: u8,
        coordination: f32,
        alpha_present: bool,
    },
    Dire {       // Mini mini-boss at village gate
        health: f32,
        size_multiplier: f32,
        reputation_gain: f32,
    },
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WeatherType {
    Rain {       // Slows movement BUT enables stealth
        movement_penalty: f32,
        stealth_bonus: f32,
    },
    Wind {       // Affects projectiles BUT helps tracking
        projectile_penalty: f32,
        tracking_bonus: f32,
    },
    Fog {        // Reduces vision BUT works both ways
        vision_penalty: f32,
        enemy_confusion: f32,
    },
    Sun {        // Clear vision BUT creates shadows
        vision_bonus: f32,
        shadow_hiding_spots: Vec<HexPosition>,
    },
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HillChoice {
    pub high_path: HighPath,
    pub low_path: LowPath,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HighPath {
    pub tactical_advantage: f32,
    pub stamina_cost: f32,
    pub vision_range: f32,
    pub ambush_protection: bool,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LowPath {
    pub easier_travel: bool,
    pub ambush_risk: f32,
    pub hidden_cache: Option<String>,
}

impl Level1to3_PathToVillage {
    pub fn setup(mut commands: Commands) {
        // Spawn wolf encounters with variety
        let wolf_positions = vec![
            (2, 0),  // Starving wolf
            (3, 1),  // Wolf mother with cubs
            (4, 0),  // Rabid wolf (first void hint)
            (5, 1),  // Wolf pack
            (6, 0),  // Dire wolf at gate
        ];
        
        for (i, (q, r)) in wolf_positions.iter().enumerate() {
            let variant = match i {
                0 => WolfVariant::Starving { 
                    health: 0.6, 
                    aggression: 1.5,
                    loot: vec!["Scrap of Cloth".to_string()],
                },
                1 => WolfVariant::Mother {
                    cubs_present: true,
                    defensive_bonus: 1.5,
                    moral_impact: 0.3,
                },
                2 => WolfVariant::Rabid {
                    corruption_level: 0.2,
                    pattern_chaos: 0.7,
                    void_whispers: true,
                },
                3 => WolfVariant::Pack {
                    pack_size: 4,
                    coordination: 0.8,
                    alpha_present: true,
                },
                4 => WolfVariant::Dire {
                    health: 3.0,
                    size_multiplier: 1.5,
                    reputation_gain: 0.2,
                },
                _ => unreachable!(),
            };
            
            commands.spawn((
                Name::new(format!("Wolf Encounter {}", i + 1)),
                HexPosition::new(*q, *r),
                variant,
            ));
        }
        
        // The Hill Choice - teaching elevation matters
        commands.spawn((
            Name::new("The Diverging Path"),
            HexPosition::new(4, 2),
            HillChoice {
                high_path: HighPath {
                    tactical_advantage: 1.3,
                    stamina_cost: 1.2,
                    vision_range: 2.0,
                    ambush_protection: true,
                },
                low_path: LowPath {
                    easier_travel: true,
                    ambush_risk: 0.4,
                    hidden_cache: Some("Father's Old Sword (70% durability)".to_string()),
                },
            },
        ));
    }
}

/// Levels 3-5: First Village (Staged Introduction)
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, YoleckComponent)]
pub struct Level3to5_FirstVillage {
    pub evening_shops: Vec<Shop>,
    pub morning_shops: Vec<Shop>,
    pub key_npcs: Vec<KeyNPC>,
    pub missing_child_quest: MissingChildQuest,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Shop {
    pub name: String,
    pub owner: String,
    pub evening_open: bool,
    pub morning_open: bool,
    pub trust_required: f32,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyNPC {
    pub name: String,
    pub role: String,
    pub knows_father: bool,
    pub quest_giver: bool,
    pub companion_reaction: String,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MissingChildQuest {
    pub child_name: String,
    pub has_player_toy: bool,  // Critical emotional hook
    pub void_touched_eyes: bool,  // Foreshadowing
    pub appears_later: Vec<u32>,  // Levels where child reappears
}

impl Level3to5_FirstVillage {
    pub fn setup(mut commands: Commands) {
        // Evening arrival - limited access
        let evening_shops = vec![
            Shop {
                name: "The Weary Traveler Inn".to_string(),
                owner: "Gareth".to_string(),
                evening_open: true,
                morning_open: true,
                trust_required: 0.0,
            },
            Shop {
                name: "General Store".to_string(),
                owner: "Martha".to_string(),
                evening_open: true,
                morning_open: true,
                trust_required: 0.0,
            },
            Shop {
                name: "Blacksmith".to_string(),
                owner: "Bjorn".to_string(),
                evening_open: true,  // Working late
                morning_open: true,
                trust_required: 0.0,
            },
        ];
        
        // Morning additions
        let morning_shops = vec![
            Shop {
                name: "Temple of the Dawn".to_string(),
                owner: "Sister Catherine".to_string(),
                evening_open: false,  // Evening mass
                morning_open: true,
                trust_required: 0.1,
            },
            Shop {
                name: "Apothecary".to_string(),
                owner: "Old Willem".to_string(),
                evening_open: false,  // Gathering herbs
                morning_open: true,
                trust_required: 0.2,
            },
        ];
        
        // Key NPCs with connections
        let key_npcs = vec![
            KeyNPC {
                name: "Village Elder Thomas".to_string(),
                role: "Leader".to_string(),
                knows_father: true,
                quest_giver: true,
                companion_reaction: "Elena trusts him. Marcus is suspicious. Quinn observes.".to_string(),
            },
            KeyNPC {
                name: "Sarah the Mother".to_string(),
                role: "Worried Parent".to_string(),
                knows_father: false,
                quest_giver: true,
                companion_reaction: "Elena wants to help. Marcus thinks it's a waste. Quinn defers to you.".to_string(),
            },
            KeyNPC {
                name: "Guard Captain Morris".to_string(),
                role: "Military".to_string(),
                knows_father: true,
                quest_giver: false,
                companion_reaction: "Marcus respects him. Elena finds him harsh. Quinn analyzes.".to_string(),
            },
        ];
        
        // The Missing Child - layers of meaning
        let missing_child = MissingChildQuest {
            child_name: "Little Emma".to_string(),
            has_player_toy: true,  // YOUR old wooden horse
            void_touched_eyes: true,  // Players won't understand yet
            appears_later: vec![60, 120, 180],  // Guide, warning, successor
        };
        
        // Spawn the village
        commands.spawn((
            Name::new("First Village - Haven's Rest"),
            HexPosition::new(7, 0),
        ));
        
        for shop in evening_shops.iter().chain(morning_shops.iter()) {
            commands.spawn((
                Name::new(shop.name.clone()),
                shop.clone(),
            ));
        }
        
        for npc in key_npcs {
            commands.spawn((
                Name::new(npc.name.clone()),
                npc,
            ));
        }
        
        commands.spawn((
            Name::new("Missing Child Quest"),
            missing_child,
            Quest {
                id: "missing_child".to_string(),
                title: "The Lost Child".to_string(),
                description: "Emma went to play with your old wooden horse. She never returned.".to_string(),
                stages: vec![
                    "Talk to Sarah".to_string(),
                    "Search the woods".to_string(),
                    "Find Emma".to_string(),
                    "Notice her eyes".to_string(),
                ],
                current_stage: 0,
                moral_weight: 0.5,
                affects_ending: true,  // She becomes your successor
            },
        ));
    }
}

/// Level 10: First Mini-Boss
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, YoleckComponent)]
pub struct Level10_BanditLieutenant {
    pub mounted: bool,
    pub horse_id: String,  // Same horse we get at L13
    pub combat_solutions: Vec<CombatSolution>,
    pub defeat_consequences: DefeatConsequence,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CombatSolution {
    Environmental { fire_nearby: bool },
    Companion { grab_reins: bool },
    Item { caltrops_available: bool },
    Honorable { dismount_for_duel: bool },
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DefeatConsequence {
    pub death_scar: bool,
    pub gold_loss: f32,
    pub companion_worry: String,
    pub void_attention: f32,
}

impl Level10_BanditLieutenant {
    pub fn setup(mut commands: Commands) {
        commands.spawn((
            Name::new("Bandit Lieutenant Korven"),
            HexPosition::new(15, 5),
            Level10_BanditLieutenant {
                mounted: true,
                horse_id: "merchant_horse_brownie".to_string(),
                combat_solutions: vec![
                    CombatSolution::Environmental { fire_nearby: true },
                    CombatSolution::Companion { grab_reins: true },
                    CombatSolution::Item { caltrops_available: true },
                    CombatSolution::Honorable { dismount_for_duel: false },  // He won't
                ],
                defeat_consequences: DefeatConsequence {
                    death_scar: true,
                    gold_loss: 0.5,
                    companion_worry: "You're scaring me...".to_string(),
                    void_attention: 0.1,
                },
            },
        ));
    }
}

/// Level 13: Mount Introduction
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, YoleckComponent)]
pub struct Level13_MountAcquisition {
    pub merchant_encounter: MerchantEncounter,
    pub horse_recognition: bool,  // Same from boss
    pub acquisition_choice: MountChoice,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MerchantEncounter {
    pub merchant_name: String,
    pub horse_name: String,
    pub horse_injury: String,
    pub previous_sighting: u32,  // Level 7
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MountChoice {
    ReturnHorse { reward: String },
    KeepHorse { merchant_reaction: String },
    HealFirst { bond_increase: f32 },
}

impl Level13_MountAcquisition {
    pub fn setup(mut commands: Commands) {
        commands.spawn((
            Name::new("Mount Acquisition Point"),
            HexPosition::new(20, 8),
            Level13_MountAcquisition {
                merchant_encounter: MerchantEncounter {
                    merchant_name: "Tobias the Trader".to_string(),
                    horse_name: "Brownie".to_string(),
                    horse_injury: "Twisted leg from fleeing".to_string(),
                    previous_sighting: 7,
                },
                horse_recognition: true,
                acquisition_choice: MountChoice::HealFirst { 
                    bond_increase: 0.3 
                },
            },
        ));
    }
}

/// Level 20: Bandit Cave - First 3D Dungeon
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, YoleckComponent)]
pub struct Level20_BanditCave {
    pub dimension_shift: DimensionShift,
    pub boss_identity: BossIdentity,
    pub children_present: bool,
    pub moral_choices: Vec<FinalChoice>,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DimensionShift {
    pub from_2d_hex: bool,
    pub to_3d_doom: bool,
    pub narrative_reason: String,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BossIdentity {
    pub name: String,
    pub backstory: String,
    pub desperation_level: f32,
    pub children_count: u8,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FinalChoice {
    Mercy { reputation_change: f32 },
    Justice { philosophy_shift: String },
    Execution { darkness_increase: f32 },
    Cruelty { void_attention: f32 },
}

impl Level20_BanditCave {
    pub fn setup(mut commands: Commands) {
        commands.spawn((
            Name::new("The Bandit Cave"),
            HexPosition::new(30, 10),
            Level20_BanditCave {
                dimension_shift: DimensionShift {
                    from_2d_hex: true,
                    to_3d_doom: true,
                    narrative_reason: "Ancient dragon-touched architecture".to_string(),
                },
                boss_identity: BossIdentity {
                    name: "Marcus the Desperate".to_string(),
                    backstory: "Lost his farm to void corruption, turned to crime to feed family".to_string(),
                    desperation_level: 0.9,
                    children_count: 3,
                },
                children_present: true,
                moral_choices: vec![
                    FinalChoice::Mercy { reputation_change: 0.3 },
                    FinalChoice::Justice { philosophy_shift: "Harmony".to_string() },
                    FinalChoice::Execution { darkness_increase: 0.2 },
                    FinalChoice::Cruelty { void_attention: 0.5 },
                ],
            },
        ));
    }
}

pub fn register_opening_levels(app: &mut App) {
    app
        .add_systems(Startup, (
            Level1_TheDoor::setup,
            Level1to3_PathToVillage::setup,
            Level3to5_FirstVillage::setup,
            Level10_BanditLieutenant::setup,
            Level13_MountAcquisition::setup,
            Level20_BanditCave::setup,
        ))
        .register_type::<Level1_TheDoor>()
        .register_type::<Level1to3_PathToVillage>()
        .register_type::<Level3to5_FirstVillage>()
        .register_type::<Level10_BanditLieutenant>()
        .register_type::<Level13_MountAcquisition>()
        .register_type::<Level20_BanditCave>();
}
