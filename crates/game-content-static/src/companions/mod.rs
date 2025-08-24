//! Companion personality system with Elena, Marcus, and Quinn
//!
//! This module implements the three distinct companion personalities
//! and their dialogue trees, reactions, and relationship mechanics.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The three companion archetypes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CompanionType {
    Elena,   // The optimistic adventurer
    Marcus,  // The grizzled warrior  
    Quinn,   // The mystic scholar
}

/// Complete companion state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Companion {
    pub companion_type: CompanionType,
    pub relationship_level: f32,  // 0.0 to 100.0
    pub trust: f32,               // -100.0 to 100.0
    pub corruption: f32,          // 0.0 to 100.0
    pub mood: CompanionMood,
    pub alive: bool,
    pub betrayed: bool,
    pub romance_available: bool,
    pub personal_quest_stage: u32,
    pub dialogue_history: Vec<String>,
    pub trait_modifiers: CompanionTraits,
}

/// Companion mood states
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum CompanionMood {
    Happy,
    Neutral,
    Worried,
    Angry,
    Frightened,
    Corrupted,
    Hopeless,
}

/// Companion personality traits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanionTraits {
    pub optimism: f32,      // -1.0 to 1.0
    pub bravery: f32,       // 0.0 to 1.0
    pub loyalty: f32,       // 0.0 to 1.0
    pub pragmatism: f32,    // 0.0 to 1.0
    pub curiosity: f32,     // 0.0 to 1.0
    pub empathy: f32,       // 0.0 to 1.0
}

impl CompanionType {
    /// Get base personality traits for each companion
    pub fn base_traits(&self) -> CompanionTraits {
        match self {
            CompanionType::Elena => CompanionTraits {
                optimism: 0.8,
                bravery: 0.6,
                loyalty: 0.9,
                pragmatism: 0.3,
                curiosity: 0.8,
                empathy: 0.9,
            },
            CompanionType::Marcus => CompanionTraits {
                optimism: -0.2,
                bravery: 0.9,
                loyalty: 0.7,
                pragmatism: 0.9,
                curiosity: 0.3,
                empathy: 0.5,
            },
            CompanionType::Quinn => CompanionTraits {
                optimism: 0.2,
                bravery: 0.4,
                loyalty: 0.6,
                pragmatism: 0.5,
                curiosity: 1.0,
                empathy: 0.7,
            },
        }
    }
    
    /// Get companion backstory
    pub fn backstory(&self) -> &'static str {
        match self {
            CompanionType::Elena => {
                "Elena grew up in the village, always dreaming of adventure beyond its walls. \
                Your father taught her swordplay when she was young, and she's been waiting \
                for a chance to use those skills. Her infectious enthusiasm masks a deep fear \
                of being ordinary. She sees this journey as her chance to become the hero \
                she's always imagined herself to be."
            }
            CompanionType::Marcus => {
                "Marcus served in the kingdom's army for twenty years before retiring to your \
                village. Your father saved his life in battle, and Marcus swore a life debt. \
                He's seen too much death and carries those scars both physically and mentally. \
                Despite his gruff exterior, he genuinely cares about protecting those who \
                cannot protect themselves."
            }
            CompanionType::Quinn => {
                "Quinn arrived in the village three years ago, claiming to study ancient texts \
                in the local ruins. They speak in riddles and seem to know more than they let on. \
                Your father consulted them about strange dreams before his disappearance. Quinn's \
                knowledge of the void and its corruption is unsettling, as if they've encountered \
                it before."
            }
        }
    }
    
    /// Get companion's personal quest
    pub fn personal_quest(&self) -> CompanionQuest {
        match self {
            CompanionType::Elena => CompanionQuest {
                name: "The Weight of Heroes".to_string(),
                description: "Elena must confront the reality that heroism comes with terrible costs.".to_string(),
                stages: vec![
                    "Elena expresses doubts about her abilities".to_string(),
                    "First innocent death shakes her confidence".to_string(),
                    "She considers giving up and going home".to_string(),
                    "Choice: Encourage her idealism or teach her pragmatism".to_string(),
                    "Elena either becomes a true hero or a broken idealist".to_string(),
                ],
                rewards: vec!["Elena's Determination (passive buff)".to_string()],
            },
            CompanionType::Marcus => CompanionQuest {
                name: "Old Soldiers Never Die".to_string(),
                description: "Marcus must face the ghosts of his past and decide if redemption is possible.".to_string(),
                stages: vec![
                    "Marcus mentions a village he failed to save".to_string(),
                    "Encounter survivors who blame him".to_string(),
                    "Marcus considers sacrificing himself".to_string(),
                    "Choice: Help him forgive himself or embrace his guilt".to_string(),
                    "Marcus finds peace or becomes death-seeking".to_string(),
                ],
                rewards: vec!["Marcus's Tactical Knowledge (combat bonus)".to_string()],
            },
            CompanionType::Quinn => CompanionQuest {
                name: "The Scholar's Burden".to_string(),
                description: "Quinn's true connection to the void is revealed.".to_string(),
                stages: vec![
                    "Quinn admits to studying void magic".to_string(),
                    "Strange symbols appear on Quinn's skin".to_string(),
                    "Quinn's previous expedition to the labyrinth revealed".to_string(),
                    "Choice: Trust Quinn's knowledge or reject void corruption".to_string(),
                    "Quinn becomes void conduit or seals their power".to_string(),
                ],
                rewards: vec!["Quinn's Void Insight (corruption resistance)".to_string()],
            },
        }
    }
}

/// Companion personal quest structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanionQuest {
    pub name: String,
    pub description: String,
    pub stages: Vec<String>,
    pub rewards: Vec<String>,
}

/// Dialogue node for companion conversations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueNode {
    pub id: String,
    pub speaker: CompanionType,
    pub text: String,
    pub mood_required: Option<CompanionMood>,
    pub trust_required: f32,
    pub responses: Vec<DialogueResponse>,
    pub effects: Vec<DialogueEffect>,
}

/// Player response option
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueResponse {
    pub text: String,
    pub next_node: Option<String>,
    pub trust_change: f32,
    pub relationship_change: f32,
    pub philosophy_alignment: Option<PhilosophyChoice>,
}

/// Philosophy choices in dialogue
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum PhilosophyChoice {
    Strength,
    Harmony,
    Light,
    Dark,
}

/// Effects of dialogue choices
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DialogueEffect {
    ChangeMood(CompanionMood),
    ChangeTrust(f32),
    ChangeRelationship(f32),
    AddCorruption(f32),
    UnlockDialogue(String),
    TriggerQuest(String),
    SetFlag(String, bool),
}

/// Generate contextual dialogue based on game state
pub fn generate_contextual_dialogue(
    companion: &Companion,
    level: u32,
    dread_level: u8,
    recent_event: Option<GameEvent>,
) -> DialogueNode {
    let base_dialogue = match companion.companion_type {
        CompanionType::Elena => generate_elena_dialogue(companion, level, dread_level, recent_event),
        CompanionType::Marcus => generate_marcus_dialogue(companion, level, dread_level, recent_event),
        CompanionType::Quinn => generate_quinn_dialogue(companion, level, dread_level, recent_event),
    };
    
    // Modify based on corruption
    if companion.corruption > 50.0 {
        add_corruption_modifier(base_dialogue, companion.corruption)
    } else {
        base_dialogue
    }
}

/// Generate Elena's dialogue
fn generate_elena_dialogue(
    companion: &Companion,
    level: u32,
    dread_level: u8,
    recent_event: Option<GameEvent>,
) -> DialogueNode {
    // Early game optimism
    if level <= 20 && dread_level == 0 {
        DialogueNode {
            id: format!("elena_early_{}", level),
            speaker: CompanionType::Elena,
            text: "This is amazing! We're really doing it - we're on an adventure! \
                   I know we'll find your father. I just know it!".to_string(),
            mood_required: None,
            trust_required: 0.0,
            responses: vec![
                DialogueResponse {
                    text: "Your optimism is infectious.".to_string(),
                    next_node: None,
                    trust_change: 5.0,
                    relationship_change: 5.0,
                    philosophy_alignment: Some(PhilosophyChoice::Light),
                },
                DialogueResponse {
                    text: "Don't be naive. This is dangerous.".to_string(),
                    next_node: None,
                    trust_change: -2.0,
                    relationship_change: 0.0,
                    philosophy_alignment: Some(PhilosophyChoice::Dark),
                },
                DialogueResponse {
                    text: "Stay focused. We have work to do.".to_string(),
                    next_node: None,
                    trust_change: 0.0,
                    relationship_change: 2.0,
                    philosophy_alignment: Some(PhilosophyChoice::Strength),
                },
            ],
            effects: vec![],
        }
    }
    // Growing fear as dread increases
    else if dread_level >= 2 && companion.mood == CompanionMood::Worried {
        DialogueNode {
            id: format!("elena_worried_{}", level),
            speaker: CompanionType::Elena,
            text: "Something's wrong. Really wrong. The air feels... thick. Heavy. \
                   Like the world itself is sick. Are you feeling it too?".to_string(),
            mood_required: Some(CompanionMood::Worried),
            trust_required: 20.0,
            responses: vec![
                DialogueResponse {
                    text: "I feel it. We'll face it together.".to_string(),
                    next_node: None,
                    trust_change: 10.0,
                    relationship_change: 10.0,
                    philosophy_alignment: Some(PhilosophyChoice::Harmony),
                },
                DialogueResponse {
                    text: "Fear will only make it worse. Steel yourself.".to_string(),
                    next_node: None,
                    trust_change: 0.0,
                    relationship_change: -5.0,
                    philosophy_alignment: Some(PhilosophyChoice::Strength),
                },
                DialogueResponse {
                    text: "The void is calling. Can't you hear it?".to_string(),
                    next_node: None,
                    trust_change: -10.0,
                    relationship_change: -10.0,
                    philosophy_alignment: Some(PhilosophyChoice::Dark),
                },
            ],
            effects: vec![
                DialogueEffect::ChangeMood(CompanionMood::Frightened),
            ],
        }
    }
    // Corruption taking hold
    else if companion.corruption > 30.0 {
        DialogueNode {
            id: format!("elena_corrupting_{}", level),
            speaker: CompanionType::Elena,
            text: "I had the dream again. The one where I'm standing over your body, \
                   holding a blade dripping with... No. No, I won't let it happen. \
                   I won't become that thing!".to_string(),
            mood_required: None,
            trust_required: 40.0,
            responses: vec![
                DialogueResponse {
                    text: "Fight it, Elena. You're stronger than the void.".to_string(),
                    next_node: None,
                    trust_change: 5.0,
                    relationship_change: 15.0,
                    philosophy_alignment: Some(PhilosophyChoice::Light),
                },
                DialogueResponse {
                    text: "Perhaps the dream is showing you your true nature.".to_string(),
                    next_node: None,
                    trust_change: -20.0,
                    relationship_change: -20.0,
                    philosophy_alignment: Some(PhilosophyChoice::Dark),
                },
            ],
            effects: vec![
                DialogueEffect::AddCorruption(5.0),
            ],
        }
    }
    // Default dialogue
    else {
        DialogueNode {
            id: format!("elena_default_{}", level),
            speaker: CompanionType::Elena,
            text: "I'm with you, no matter what happens.".to_string(),
            mood_required: None,
            trust_required: 0.0,
            responses: vec![
                DialogueResponse {
                    text: "Thank you, Elena.".to_string(),
                    next_node: None,
                    trust_change: 2.0,
                    relationship_change: 2.0,
                    philosophy_alignment: None,
                },
            ],
            effects: vec![],
        }
    }
}

/// Generate Marcus's dialogue
fn generate_marcus_dialogue(
    companion: &Companion,
    level: u32,
    dread_level: u8,
    recent_event: Option<GameEvent>,
) -> DialogueNode {
    // Tactical assessment
    if matches!(recent_event, Some(GameEvent::CombatVictory)) {
        DialogueNode {
            id: format!("marcus_combat_{}", level),
            speaker: CompanionType::Marcus,
            text: "Good form in that fight, but you're dropping your guard on the left. \
                   Your father made the same mistake. I trained it out of him, \
                   I'll train it out of you.".to_string(),
            mood_required: None,
            trust_required: 10.0,
            responses: vec![
                DialogueResponse {
                    text: "I appreciate the guidance.".to_string(),
                    next_node: None,
                    trust_change: 5.0,
                    relationship_change: 5.0,
                    philosophy_alignment: Some(PhilosophyChoice::Harmony),
                },
                DialogueResponse {
                    text: "I don't need your lessons, old man.".to_string(),
                    next_node: None,
                    trust_change: -5.0,
                    relationship_change: -10.0,
                    philosophy_alignment: Some(PhilosophyChoice::Strength),
                },
            ],
            effects: vec![],
        }
    }
    // Warning about corruption
    else if dread_level >= 3 {
        DialogueNode {
            id: format!("marcus_warning_{}", level),
            speaker: CompanionType::Marcus,
            text: "I've seen this before. In the war. Men changed by dark magic. \
                   They started as heroes, ended as monsters. Look at your hands, kid. \
                   See the black veins? You're changing. We both are.".to_string(),
            mood_required: None,
            trust_required: 30.0,
            responses: vec![
                DialogueResponse {
                    text: "Then we stop before it's too late.".to_string(),
                    next_node: None,
                    trust_change: 10.0,
                    relationship_change: 5.0,
                    philosophy_alignment: Some(PhilosophyChoice::Light),
                },
                DialogueResponse {
                    text: "Power always comes with a price.".to_string(),
                    next_node: None,
                    trust_change: 0.0,
                    relationship_change: 0.0,
                    philosophy_alignment: Some(PhilosophyChoice::Dark),
                },
            ],
            effects: vec![
                DialogueEffect::ChangeMood(CompanionMood::Worried),
            ],
        }
    }
    // Default gruff support
    else {
        DialogueNode {
            id: format!("marcus_default_{}", level),
            speaker: CompanionType::Marcus,
            text: "Keep moving. Dwelling on things won't change them.".to_string(),
            mood_required: None,
            trust_required: 0.0,
            responses: vec![
                DialogueResponse {
                    text: "You're right.".to_string(),
                    next_node: None,
                    trust_change: 1.0,
                    relationship_change: 1.0,
                    philosophy_alignment: None,
                },
            ],
            effects: vec![],
        }
    }
}

/// Generate Quinn's dialogue
fn generate_quinn_dialogue(
    companion: &Companion,
    level: u32,
    dread_level: u8,
    recent_event: Option<GameEvent>,
) -> DialogueNode {
    // Cryptic void knowledge
    if level >= 50 && dread_level >= 2 {
        DialogueNode {
            id: format!("quinn_void_{}", level),
            speaker: CompanionType::Quinn,
            text: "The patterns are aligning, just as the texts described. \
                   'When the seeker becomes the sought, when the hunter becomes the beast.' \
                   Your father read these same words. He understood what he would become.".to_string(),
            mood_required: None,
            trust_required: 25.0,
            responses: vec![
                DialogueResponse {
                    text: "What do you mean? What did he become?".to_string(),
                    next_node: Some("quinn_revelation".to_string()),
                    trust_change: 5.0,
                    relationship_change: 10.0,
                    philosophy_alignment: None,
                },
                DialogueResponse {
                    text: "Stop speaking in riddles!".to_string(),
                    next_node: None,
                    trust_change: -5.0,
                    relationship_change: -5.0,
                    philosophy_alignment: Some(PhilosophyChoice::Strength),
                },
                DialogueResponse {
                    text: "I already know. I can feel it happening to me.".to_string(),
                    next_node: None,
                    trust_change: 10.0,
                    relationship_change: 5.0,
                    philosophy_alignment: Some(PhilosophyChoice::Dark),
                },
            ],
            effects: vec![
                DialogueEffect::UnlockDialogue("quinn_void_truth".to_string()),
            ],
        }
    }
    // Philosophical musings
    else if companion.mood == CompanionMood::Neutral {
        DialogueNode {
            id: format!("quinn_philosophy_{}", level),
            speaker: CompanionType::Quinn,
            text: "Every choice creates a new path, yet all paths lead to the same destination. \
                   The question isn't whether we'll arrive, but who we'll be when we get there.".to_string(),
            mood_required: Some(CompanionMood::Neutral),
            trust_required: 15.0,
            responses: vec![
                DialogueResponse {
                    text: "Our choices define us.".to_string(),
                    next_node: None,
                    trust_change: 5.0,
                    relationship_change: 5.0,
                    philosophy_alignment: Some(PhilosophyChoice::Light),
                },
                DialogueResponse {
                    text: "Fate is fate. Choice is an illusion.".to_string(),
                    next_node: None,
                    trust_change: 5.0,
                    relationship_change: 5.0,
                    philosophy_alignment: Some(PhilosophyChoice::Dark),
                },
            ],
            effects: vec![],
        }
    }
    // Default mysterious
    else {
        DialogueNode {
            id: format!("quinn_default_{}", level),
            speaker: CompanionType::Quinn,
            text: "The void watches. It always has.".to_string(),
            mood_required: None,
            trust_required: 0.0,
            responses: vec![
                DialogueResponse {
                    text: "...Right.".to_string(),
                    next_node: None,
                    trust_change: 0.0,
                    relationship_change: 1.0,
                    philosophy_alignment: None,
                },
            ],
            effects: vec![],
        }
    }
}

/// Add corruption effects to dialogue
fn add_corruption_modifier(
    mut dialogue: DialogueNode,
    corruption_level: f32,
) -> DialogueNode {
    // Add unsettling elements to corrupted companion dialogue
    if corruption_level > 75.0 {
        dialogue.text = format!(
            "{} *Their eyes flicker with purple light as they speak, and you notice \
            their shadow moving independently*",
            dialogue.text
        );
        dialogue.effects.push(DialogueEffect::AddCorruption(2.0));
    } else if corruption_level > 50.0 {
        dialogue.text = format!(
            "{} *Black veins pulse visibly under their skin*",
            dialogue.text
        );
        dialogue.effects.push(DialogueEffect::AddCorruption(1.0));
    }
    
    dialogue
}

/// Game events that trigger dialogue
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum GameEvent {
    CombatVictory,
    CombatDefeat,
    BossDefeated,
    CompanionInjured,
    VillagerDeath,
    VoidEncounter,
    TreasureFound,
    PuzzleSolved,
    CorruptionIncrease,
    LevelTransition,
}

/// Companion relationship responses to player actions
pub fn get_relationship_response(
    companion_type: CompanionType,
    action: PlayerAction,
) -> (f32, f32) {  // (trust_change, relationship_change)
    match (companion_type, action) {
        // Elena responses
        (CompanionType::Elena, PlayerAction::SaveInnocent) => (10.0, 15.0),
        (CompanionType::Elena, PlayerAction::KillInnocent) => (-20.0, -25.0),
        (CompanionType::Elena, PlayerAction::ShowMercy) => (5.0, 10.0),
        (CompanionType::Elena, PlayerAction::ActRuthless) => (-5.0, -10.0),
        
        // Marcus responses  
        (CompanionType::Marcus, PlayerAction::TacticalRetreat) => (5.0, 5.0),
        (CompanionType::Marcus, PlayerAction::RecklessCharge) => (-5.0, -5.0),
        (CompanionType::Marcus, PlayerAction::ProtectCompanion) => (15.0, 10.0),
        (CompanionType::Marcus, PlayerAction::SacrificeOthers) => (0.0, -5.0),
        
        // Quinn responses
        (CompanionType::Quinn, PlayerAction::SeekKnowledge) => (10.0, 10.0),
        (CompanionType::Quinn, PlayerAction::DestroyArtifact) => (-5.0, -5.0),
        (CompanionType::Quinn, PlayerAction::EmbraceVoid) => (5.0, 0.0),
        (CompanionType::Quinn, PlayerAction::RejectVoid) => (-5.0, 5.0),
        
        _ => (0.0, 0.0),
    }
}

/// Player actions that affect relationships
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum PlayerAction {
    SaveInnocent,
    KillInnocent,
    ShowMercy,
    ActRuthless,
    TacticalRetreat,
    RecklessCharge,
    ProtectCompanion,
    SacrificeOthers,
    SeekKnowledge,
    DestroyArtifact,
    EmbraceVoid,
    RejectVoid,
}
