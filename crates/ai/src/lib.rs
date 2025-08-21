//! AI and pathfinding systems for Dragon's Labyrinth

use bevy::prelude::*;
use big_brain::prelude::*;
use pathfinding::prelude::*;

pub mod behaviors;
pub use behaviors::*;

pub struct AIPlugin;

impl Plugin for AIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(BigBrainPlugin::new(PreUpdate))
            .init_resource::<AIConfig>()
            .add_systems(Update, (
                behaviors::update_ai_behaviors,
                behaviors::patrol_behavior_system,
                behaviors::chase_behavior_system,
                behaviors::flee_behavior_system,
                process_pathfinding,
            ));
    }
}

#[derive(Resource)]
pub struct AIConfig {
    pub max_path_length: usize,
    pub recalculate_interval: f32,
    pub vision_range: f32,
    pub hearing_range: f32,
}

impl Default for AIConfig {
    fn default() -> Self {
        Self {
            max_path_length: 100,
            recalculate_interval: 0.5,
            vision_range: 15.0,
            hearing_range: 25.0,
        }
    }
}



fn process_pathfinding(
    agents: Query<&AIAgent>,
) {
    // Pathfinding logic
}