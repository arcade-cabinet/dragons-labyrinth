//! Component organization for Dragon's Labyrinth ECS
//!
//! This module organizes all game components into logical groups.
//! Each submodule contains related components for a specific game system.

pub mod core;
pub mod world;
pub mod companions;
pub mod narrative;
pub mod horror;

// Re-export commonly used components
pub use core::{Player, HexPosition, Health, Sanity, Inventory, Name, Faction};
pub use world::{HexTile, TerrainType, BiomeType, Environment, PointOfInterest};
pub use companions::{Companion, CompanionArchetype, CompanionState, Mount, Relationship};
pub use narrative::{Quest, QuestType, DialogueRunner, MoralChoice, NPC, StoryNode};
pub use horror::{GlobalDread, Corruption, Hallucination, HorrorAudio, HorrorVisual, Trauma};

use bevy::prelude::*;

/// Component bundle for spawning a player entity
#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub position: HexPosition,
    pub health: Health,
    pub sanity: Sanity,
    pub inventory: Inventory,
    pub name: Name,
}

/// Component bundle for spawning a companion
#[derive(Bundle)]
pub struct CompanionBundle {
    pub companion: Companion,
    pub position: HexPosition,
    pub health: Health,
    pub sanity: Sanity,
    pub name: Name,
    pub faction: Faction,
}

/// Component bundle for spawning an NPC
#[derive(Bundle)]
pub struct NPCBundle {
    pub npc: NPC,
    pub position: HexPosition,
    pub health: Health,
    pub name: Name,
    pub faction: Faction,
}

/// Component bundle for hex tiles
#[derive(Bundle)]
pub struct HexTileBundle {
    pub tile: HexTile,
    pub corruption: Corruption,
    pub environment: Environment,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum TileType {
    Grass,
    Forest,
    Stone,
    Water,
    Village,
    Ruins,
    Corrupted,  // Appears with increasing dread
    Void,       // Horror stage tiles
}

// Companion system following trauma mechanics from design bible
#[derive(Component, Clone, Debug)]
pub struct Companion {
    pub name: String,
    pub companion_type: CompanionType,
    pub trauma_level: f32,  // 0.0-1.0, affects behavior
    pub current_state: CompanionState,
    pub loyalty: f32,       // Can decrease, affecting ending
}

#[derive(Clone, Debug, PartialEq)]
pub enum CompanionType {
    Einar,   // Loyal friend who breaks under pressure
    Mira,    // Optimist who abandons party in Dread stage
    Sorin,   // Scholar who becomes traitor boss if mishandled
    Tamara,  // Innocent baker's apprentice
}

#[derive(Clone, Debug, PartialEq)]
pub enum CompanionState {
    Normal,
    Uneasy,
    Traumatized,
    Broken,
    Abandoned,  // Left the party
    Hostile,    // Becomes enemy
}

// Player character component
#[derive(Component)]
pub struct Player {
    pub hex_position: (i32, i32),
    pub sanity: f32,  // 0.0-1.0, affects hallucinations
    pub inventory: Vec<String>,
}

// Quest system following narrative progression
#[derive(Component, Clone, Debug)]
pub struct Quest {
    pub id: String,
    pub title: String,
    pub description: String,
    pub quest_type: QuestType,
    pub dread_stage: u8,  // Which stage this quest appears in
    pub completed: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub enum QuestType {
    Delivery,      // Peace stage: "deliver bread"
    Investigation, // Unease: "find missing person"
    Survival,      // Dread: "gather supplies"
    Moral,         // Terror: "your companion begs you to kill them"
    Final,         // Horror: Dragon encounter
}

// NPC system that reacts to dread progression
#[derive(Component, Clone, Debug)]
pub struct Npc {
    pub name: String,
    pub npc_type: NpcType,
    pub dialogue_state: DialogueState,
    pub fear_level: f32,  // Increases with dread
}

#[derive(Clone, Debug, PartialEq)]
pub enum NpcType {
    Villager,
    Merchant,
    Guard,
    Questgiver,
}

#[derive(Clone, Debug, PartialEq)]
pub enum DialogueState {
    Friendly,      // Peace stage
    Nervous,       // Unease stage
    Fearful,       // Dread stage
    Refuses,       // Terror stage
    Gone,          // Horror stage - fled or disappeared
}

// Audio cue system for proximity horror
#[derive(Component)]
pub struct AudioCue {
    pub cue_type: AudioCueType,
    pub trigger_distance: f32,
    pub is_hallucination: bool,  // False audio based on sanity
}

#[derive(Clone, Debug, PartialEq)]
pub enum AudioCueType {
    DragonBreath,
    Footsteps,
    Whispers,
    Ambient,
    Music,
}