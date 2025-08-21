//! AI and pathfinding systems for Dragon's Labyrinth

use bevy::prelude::*;
use big_brain::prelude::*;
use pathfinding::prelude::*;

pub struct AIPlugin;

impl Plugin for AIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(BigBrainPlugin::new(PreUpdate))
            .init_resource::<AIConfig>()
            .add_systems(Update, (
                update_ai_behaviors,
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

#[derive(Component)]
pub struct AIAgent {
    pub behavior_type: BehaviorType,
    pub state: AIState,
    pub target: Option<Entity>,
}

#[derive(Clone, Debug)]
pub enum BehaviorType {
    Aggressive,
    Defensive,
    Neutral,
    Fleeing,
    Patrolling,
}

#[derive(Clone, Debug)]
pub enum AIState {
    Idle,
    Moving,
    Attacking,
    Fleeing,
    Investigating,
    Dead,
}

fn update_ai_behaviors(
    mut agents: Query<&mut AIAgent>,
    time: Res<Time>,
) {
    // AI behavior update logic
}

fn process_pathfinding(
    agents: Query<&AIAgent>,
) {
    // Pathfinding logic
}