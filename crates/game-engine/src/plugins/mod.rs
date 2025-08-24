//! Plugin architecture for Dragon's Labyrinth
//!
//! Each major game system is organized as a Bevy plugin for modularity.

use bevy::prelude::*;

pub mod core;
pub mod world;
pub mod companions;
pub mod narrative;
pub mod horror;
pub mod third_party;

/// Master plugin that includes all game systems
pub struct DragonsLabyrinthPlugin;

impl Plugin for DragonsLabyrinthPlugin {
    fn build(&self, app: &mut App) {
        app
            // Core systems
            .add_plugins(core::CorePlugin)
            .add_plugins(world::WorldPlugin)
            .add_plugins(companions::CompanionPlugin)
            .add_plugins(narrative::NarrativePlugin)
            .add_plugins(horror::HorrorPlugin)
            // Third-party integrations
            .add_plugins(third_party::ThirdPartyPlugins);
    }
}
