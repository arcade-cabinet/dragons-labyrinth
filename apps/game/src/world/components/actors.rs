//! Actor-related components (NPCs, monsters, characters)

use bevy::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct NPC {
    pub name: String,
    pub npc_type: NPCType,
    pub dialogue_tree: String,
    pub faction_allegiance: Option<String>,
    pub interaction_radius: f32,
    pub is_available: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NPCType {
    Merchant,
    Guard,
    Villager,
    Elder,
    Scholar,
    Healer,
    Guide,
    Stranger,
    Corrupted,
}

impl Default for NPC {
    fn default() -> Self {
        Self {
            name: "Unknown".to_string(),
            npc_type: NPCType::Villager,
            dialogue_tree: "default".to_string(),
            faction_allegiance: None,
            interaction_radius: 2.0,
            is_available: true,
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct Monster {
    pub name: String,
    pub monster_type: MonsterType,
    pub challenge_rating: f32,
    pub health: f32,
    pub max_health: f32,
    pub aggression_level: f32,
    pub corruption_aura: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MonsterType {
    Beast,
    Undead,
    Corrupted,
    Elemental,
    Aberration,
    Construct,
    Dragon,
}

impl Default for Monster {
    fn default() -> Self {
        Self {
            name: "Unknown Monster".to_string(),
            monster_type: MonsterType::Beast,
            challenge_rating: 1.0,
            health: 50.0,
            max_health: 50.0,
            aggression_level: 0.5,
            corruption_aura: 0.1,
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct CharacterModel {
    pub model_path: String,
    pub scale: f32,
    pub animation_state: AnimationState,
    pub material_override: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AnimationState {
    Idle,
    Walking,
    Running,
    Fighting,
    Talking,
    Dying,
    Dead,
}

impl Default for CharacterModel {
    fn default() -> Self {
        Self {
            model_path: "models/default_character.glb".to_string(),
            scale: 1.0,
            animation_state: AnimationState::Idle,
            material_override: None,
        }
    }
}
