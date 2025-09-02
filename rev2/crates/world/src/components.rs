use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy)]
pub struct AxialPos { pub q: i32, pub r: i32 }

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Pawn;

#[derive(Component, Debug, Clone)]
pub struct Npc { pub id: String, pub name: String }

#[derive(Component, Debug, Clone)]
pub struct Creature { pub id: String, pub name: String, pub tags: Vec<String> }

#[derive(Component, Debug, Clone)]
pub struct HexCell { pub q: i32, pub r: i32, pub kind: String }
