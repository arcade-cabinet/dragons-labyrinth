use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct HexTile { pub q: i32, pub r: i32, pub kind: String }

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Npc { pub id: String, pub name: String }

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Creature { pub id: String, pub name: String, pub tags: Vec<String> }

#[derive(Component, Debug, Clone)]
pub struct Player;

#[derive(Component, Debug, Clone)]
pub struct Pawn;

#[derive(Component, Debug, Clone)]
pub struct AxialPos { pub q: i32, pub r: i32 }
