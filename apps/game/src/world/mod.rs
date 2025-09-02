use bevy::prelude::*;

pub mod components;
pub mod resources;  
pub mod queries;
pub mod systems;

// Generated ECS modules from HBF entity processing
pub mod regions;
pub mod settlements;
pub mod dungeons;

pub fn register(app: &mut App) {
    resources::register_resources(app);
    systems::register_systems(app);
    
    // Register generated ECS systems
app.add_systems(Startup, regions::spawn_regions);
app.add_systems(Update, regions::update_regions_corruption);
app.add_systems(Startup, settlements::spawn_settlements);
app.add_systems(Update, settlements::update_settlements_corruption);
app.add_systems(Startup, dungeons::spawn_dungeons);
app.add_systems(Update, dungeons::update_dungeons_corruption);
}