//! Level editor and management for Dragon's Labyrinth

use bevy::prelude::*;
use bevy_yoleck::prelude::*;
use serde::{Deserialize, Serialize};

pub struct LevelsPlugin;

impl Plugin for LevelsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(YoleckPluginForGame)
            .add_yoleck_entity::<LevelEntity>()
            .init_resource::<CurrentLevel>()
            .add_systems(Startup, setup_level_system)
            .add_systems(Update, update_level_system);
    }
}

#[derive(Resource, Default)]
pub struct CurrentLevel {
    pub name: String,
    pub id: u32,
    pub loaded: bool,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct LevelEntity {
    pub position: Vec3,
    pub entity_type: EntityType,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum EntityType {
    Player,
    Enemy(String),
    Item(String),
    Trigger(String),
    Decoration(String),
}

fn setup_level_system(mut commands: Commands) {
    // Level system setup
}

fn update_level_system(
    current_level: Res<CurrentLevel>,
) {
    // Level update logic
}