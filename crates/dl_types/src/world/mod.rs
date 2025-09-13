//! World-related types moved from apps/game/src/world/components

pub mod character;
pub mod companions;
pub mod dread;
pub mod hex;
pub mod player;
pub mod tiles;

// Re-export all world types (specific to avoid ambiguity)
pub use character::{CharacterData, CharacterAppearance, CharacterStats, Gender, HairStyle, SkinTone, ClothingSet, NPC, NPCType, Monster, MonsterType, AIState, CharacterModel};
pub use companions::*;
pub use dread::*;
pub use hex::*;
pub use player::{Player, Mount, MountType, Item, ItemType, Inventory};
pub use tiles::*;
