// Component definitions for Dragon's Labyrinth ECS architecture
use bevy::prelude::*;
use hexx::Hex;
use serde::{Deserialize, Serialize};

/// Player component marker
#[derive(Component)]
pub struct Player;

/// Hex position component for entities on the hex grid
#[derive(Component, Clone, Copy, Debug)]
pub struct HexPosition(pub Hex);

/// Hex tile component
#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct HexTile {
    pub hex: Hex,
    pub tile_type: TileType,
    pub dread_level: u8,
    pub corruption: f32,
    pub elevation: f32,
    pub passable: bool,
}

/// Different tile types in the world
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum TileType {
    Grass,
    Forest,
    Swamp,
    Stone,
    Corrupted,
}

/// Companion character component
#[derive(Component, Clone, Debug)]
pub struct Companion {
    pub name: String,
    pub companion_type: CompanionType,
    pub sanity: f32,
    pub loyalty: f32,
    pub trauma_level: f32,
}

#[derive(Clone, Debug)]
pub enum CompanionType {
    Einar,    // Loyal friend who breaks under pressure
    Mira,     // Optimist who abandons party in Dread stage
    Sorin,    // Scholar who becomes traitor boss if not handled properly
    Tamara,   // Innocent baker's apprentice
}

/// NPC component for non-companion characters
#[derive(Component, Clone, Debug)]
pub struct NPC {
    pub name: String,
    pub npc_type: String,
    pub sanity: f32,
    pub dialogue_tree: String,
    pub flee_threshold: f32,
}

/// Monster/enemy component
#[derive(Component, Clone, Debug)]
pub struct Monster {
    pub name: String,
    pub monster_type: String,
    pub health: f32,
    pub damage: f32,
    pub detection_radius: f32,
    pub move_speed: f32,
    pub behavior: MonsterBehavior,
}

#[derive(Clone, Debug)]
pub enum MonsterBehavior {
    Observe,
    Follow,
    Stalk,
    Hunt,
    Relentless,
}

/// Quest component
#[derive(Component, Clone, Debug)]
pub struct Quest {
    pub id: String,
    pub title: String,
    pub description: String,
    pub quest_type: QuestType,
    pub completion_status: QuestStatus,
    pub required_dread_level: u8,
    pub moral_choice: Option<MoralChoice>,
}

#[derive(Clone, Debug)]
pub enum QuestType {
    Delivery,        // Peace stage - deliver bread
    Investigation,   // Unease stage - investigate disturbances
    Survival,        // Dread stage - survive encounters
    MoralDilemma,    // Terror stage - difficult moral choices
    BossEncounter,   // Any stage - major boss fights
}

#[derive(Clone, Debug)]
pub enum QuestStatus {
    Available,
    Active,
    Completed,
    Failed,
    Abandoned,
}

#[derive(Clone, Debug)]
pub struct MoralChoice {
    pub description: String,
    pub options: Vec<MoralOption>,
}

#[derive(Clone, Debug)]
pub struct MoralOption {
    pub text: String,
    pub consequence: MoralConsequence,
}

#[derive(Clone, Debug)]
pub enum MoralConsequence {
    IncreaseDread(u8),
    DecreaseSanity(f32),
    CompanionLeaves(CompanionType),
    UnlockBossEncounter(String),
    ChangeEnding(String),
}

/// Item component
#[derive(Component, Clone, Debug)]
pub struct Item {
    pub name: String,
    pub description: String,
    pub item_type: ItemType,
    pub corrupted: bool,
    pub durability: Option<f32>,
}

#[derive(Clone, Debug)]
pub enum ItemType {
    Consumable,
    Tool,
    Weapon,
    KeyItem,
    Currency,
}

/// Dialogue component for interactive conversations
#[derive(Component, Clone, Debug)]
pub struct Dialogue {
    pub character_name: String,
    pub current_node: String,
    pub available_responses: Vec<DialogueResponse>,
    pub dread_dependent: bool,
}

#[derive(Clone, Debug)]
pub struct DialogueResponse {
    pub text: String,
    pub next_node: String,
    pub requires_sanity: Option<f32>,
    pub consequence: Option<DialogueConsequence>,
}

#[derive(Clone, Debug)]
pub enum DialogueConsequence {
    ChangeSanity(f32),
    ProgressQuest(String),
    UnlockLocation(String),
    TriggerEvent(String),
}

/// Audio source component for 3D positional audio
#[derive(Component, Clone, Debug)]
pub struct AudioSource3D {
    pub sound_type: SoundType,
    pub volume: f32,
    pub range: f32,
    pub dread_responsive: bool,
}

#[derive(Clone, Debug)]
pub enum SoundType {
    Ambient,
    Voice,
    Effect,
    Music,
    Horror,
}

/// Health component for entities that can take damage
#[derive(Component, Clone, Debug)]
pub struct Health {
    pub current: f32,
    pub maximum: f32,
    pub regeneration_rate: f32,
}

/// Movement component for entities that can move on the hex grid
#[derive(Component, Clone, Debug)]
pub struct Movement {
    pub speed: f32,
    pub target_hex: Option<Hex>,
    pub path: Vec<Hex>,
    pub movement_type: MovementType,
}

#[derive(Clone, Debug)]
pub enum MovementType {
    Walking,
    Flying,
    Teleporting,
    Phasing,
}

/// Visual effects component for particle systems and animations
#[derive(Component, Clone, Debug)]
pub struct VisualEffect {
    pub effect_type: EffectType,
    pub duration: f32,
    pub intensity: f32,
    pub dread_scaling: bool,
}

#[derive(Clone, Debug)]
pub enum EffectType {
    Corruption,
    Healing,
    Magic,
    Horror,
    Death,
    Transformation,
}

/// AI behavior component for NPCs and monsters
#[derive(Component, Clone, Debug)]
pub struct AIBehavior {
    pub behavior_tree: String,
    pub current_state: AIState,
    pub decision_timer: f32,
    pub memory: AIMemory,
}

#[derive(Clone, Debug)]
pub enum AIState {
    Idle,
    Patrol,
    Investigate,
    Flee,
    Attack,
    Dialogue,
    Corrupted,
}

#[derive(Clone, Debug)]
pub struct AIMemory {
    pub last_seen_player: Option<Hex>,
    pub alert_level: f32,
    pub known_threats: Vec<Hex>,
    pub conversation_history: Vec<String>,
}