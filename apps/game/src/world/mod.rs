pub mod resources; // contains game_state only; world state is at crate::world::state
pub mod hex;
pub mod material;
pub mod save;
pub mod state;
pub mod abilities;
pub mod alignment;
pub mod atlas;
pub mod actors;
pub mod systems;

// ECS consolidation: alias existing modules under world/
pub mod components;
