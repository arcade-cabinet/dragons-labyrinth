//! ECS Components for Dragon's Labyrinth
//! 
//! This module imports unified type definitions from dl_types and adds game-specific components.

// Import world types from dl_types (unified type definitions)
pub use dl_types::world::*;

// Game-specific components not in dl_types
pub mod actors;

// Re-export game-specific components
pub use actors::*;
