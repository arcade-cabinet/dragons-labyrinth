//! Third-party crate integrations
//!
//! Configures and integrates external crates into our ECS architecture.

use bevy::prelude::*;

/// Plugin group for all third-party integrations
pub struct ThirdPartyPlugins;

impl Plugin for ThirdPartyPlugins {
    fn build(&self, app: &mut App) {
        app
            // Hex grid management with hexx
            .add_plugins(HexxPlugin)
            
            // Physics with avian
            .add_plugins(AvianPlugin)
            
            // Dialogue with YarnSpinner
            .add_plugins(YarnSpinnerIntegration)
            
            // Narrative graphs with Cobweb
            .add_plugins(CobwebPlugin)
            
            // Level editing with Yoleck
            .add_plugins(YoleckIntegration)
            
            // Procedural generation with mapgen
            .add_plugins(MapgenPlugin)
            
            // Tile rendering with claytiles (if we use it)
            // .add_plugins(ClayTilesPlugin)
            ;
    }
}

/// Hexx hex grid integration
struct HexxPlugin;

impl Plugin for HexxPlugin {
    fn build(&self, app: &mut App) {
        use hexx::*;
        
        // Set up hex layout resource
        let layout = HexLayout {
            hex_size: Vec2::new(32.0, 32.0),
            orientation: HexOrientation::Flat,
            origin: Vec2::ZERO,
        };
        
        app.insert_resource(layout);
        
        // Add hex-related systems
        app.add_systems(Update, (
            update_hex_positions,
            handle_hex_selection,
            calculate_hex_paths,
        ));
    }
}

fn update_hex_positions(
    mut query: Query<(&crate::components::HexPosition, &mut Transform)>,
    layout: Res<hexx::HexLayout>,
) {
    for (hex_pos, mut transform) in query.iter_mut() {
        let world_pos = layout.hex_to_world_pos(hex_pos.hex);
        transform.translation.x = world_pos.x;
        transform.translation.z = world_pos.y; // Note: Z for 3D
        // Keep Y (elevation) as-is
    }
}

fn handle_hex_selection(
    // Implementation for selecting hexes with mouse
) {
    // TODO: Implement hex selection
}

fn calculate_hex_paths(
    // Implementation for pathfinding
) {
    // TODO: Implement A* pathfinding on hex grid
}

/// Avian physics integration
struct AvianPlugin;

impl Plugin for AvianPlugin {
    fn build(&self, app: &mut App) {
        use avian3d::prelude::*;
        
        app
            .add_plugins(PhysicsPlugins::default())
            .insert_resource(Gravity(Vec3::Y * -9.81))
            .add_systems(Update, apply_physics_to_entities);
    }
}

fn apply_physics_to_entities(
    // Sync physics with hex positions
) {
    // TODO: Implement physics integration
}

/// YarnSpinner dialogue integration
struct YarnSpinnerIntegration;

impl Plugin for YarnSpinnerIntegration {
    fn build(&self, app: &mut App) {
        use bevy_yarnspinner::prelude::*;
        
        app
            .add_plugins(YarnSpinnerPlugin::new())
            .add_systems(Update, (
                handle_dialogue_triggers,
                process_dialogue_choices,
                update_dialogue_variables,
            ));
    }
}

fn handle_dialogue_triggers(
    // Check for dialogue trigger conditions
) {
    // TODO: Implement dialogue triggers
}

fn process_dialogue_choices(
    // Handle player choices in dialogue
) {
    // TODO: Process dialogue choices and consequences
}

fn update_dialogue_variables(
    // Sync game state with YarnSpinner variables
) {
    // TODO: Update dialogue variables based on game state
}

/// Cobweb narrative graph integration
struct CobwebPlugin;

impl Plugin for CobwebPlugin {
    fn build(&self, app: &mut App) {
        // TODO: Add cobweb once we have the crate details
        app.add_systems(Update, (
            update_story_graph,
            check_branch_conditions,
        ));
    }
}

fn update_story_graph(
    // Progress through narrative graph
) {
    // TODO: Implement story progression
}

fn check_branch_conditions(
    // Check if branches are available
) {
    // TODO: Check narrative branch conditions
}

/// Yoleck level editor integration
struct YoleckIntegration;

impl Plugin for YoleckIntegration {
    fn build(&self, app: &mut App) {
        use bevy_yoleck::prelude::*;
        
        app
            .add_plugins(YoleckPluginForGame)
            .add_systems(Update, (
                handle_level_loading,
                handle_level_saving,
            ));
    }
}

fn handle_level_loading(
    // Load levels created in editor
) {
    // TODO: Load yoleck levels
}

fn handle_level_saving(
    // Save current level state
) {
    // TODO: Save level data
}

/// Mapgen procedural generation integration
struct MapgenPlugin;

impl Plugin for MapgenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, generate_initial_world)
           .add_systems(Update, expand_world_as_needed);
    }
}

fn generate_initial_world(
    mut commands: Commands,
) {
    // TODO: Generate starting area using mapgen
    // This should create hex tiles with appropriate biomes
}

fn expand_world_as_needed(
    // Generate new areas as player explores
) {
    // TODO: Procedurally expand the world
}
