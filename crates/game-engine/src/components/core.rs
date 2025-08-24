//! Core components for Dragon's Labyrinth
//! 
//! These are the fundamental building blocks of our ECS architecture.
//! Every entity in the game is composed of these components.

use bevy::prelude::*;
use hexx::Hex;
use serde::{Deserialize, Serialize};

/// Player component - marks the player entity
#[derive(Component, Debug)]
pub struct Player {
    pub save_slot: u8,
    pub dread_level: u8, // 0-4 as per design bible
}

/// Position on the hex grid
#[derive(Component, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct HexPosition {
    pub hex: Hex,
    pub elevation: f32,
}

/// Health component for all living entities
#[derive(Component, Clone, Debug)]
pub struct Health {
    pub current: f32,
    pub max: f32,
    pub wounds: Vec<Wound>,
    pub is_dead: bool,
}

#[derive(Clone, Debug)]
pub struct Wound {
    pub severity: f32,
    pub description: String,
    pub affects_sanity: bool,
}

/// Sanity/psychology component
#[derive(Component, Clone, Debug)]
pub struct Sanity {
    pub current: f32,
    pub max: f32,
    pub hallucination_threshold: f32,
    pub breakdown_count: u32,
}

/// Inventory component
#[derive(Component, Clone, Debug, Default)]
pub struct Inventory {
    pub items: Vec<Entity>, // References to item entities
    pub capacity: usize,
    pub weight: f32,
}

/// Name component for any named entity
#[derive(Component, Clone, Debug)]
pub struct Name(pub String);

/// Faction/allegiance component
#[derive(Component, Clone, Debug)]
pub enum Faction {
    Player,
    Companion,
    Villager,
    Hostile,
    Neutral,
    Corrupted, // Entities corrupted by dread
}
