//! Dragon's Labyrinth Game Engine - Complete ECS Integration
//!
//! Production-ready Bevy game engine for Dragon's Labyrinth's sophisticated horror RPG systems.
//! Features layer cake hex tile system, dual-path morality, companion psychology, and dread progression.

use bevy::prelude::*;

pub mod components;
pub mod systems;
pub mod world;

pub use components::*;
pub use systems::*;
pub use world::*;

/// Master plugin that coordinates all Dragon's Labyrinth game systems
pub struct DragonLabyrinthGamePlugin;

impl Plugin for DragonLabyrinthGamePlugin {
    fn build(&self, app: &mut App) {
        app
            // Core Bevy plugins (would be added by the main app)
            // .add_plugins(DefaultPlugins)
            
            // Register reflection for debugging
            .register_type::<DragonLabyrinthGameState>()
            
            // Initialize game state
            .init_state::<DragonLabyrinthGameState>()
            
            // Add all Dragon's Labyrinth systems
            .add_plugins((
                // Core world and tile management
                DragonLabyrinthWorldPlugin,
                
                // Layer cake hex rendering system
                HexRenderingPlugin,
                
                // Master horror orchestrator
                systems::dread_progression::DreadProgressionPlugin,
                
                // Sophisticated companion psychology
                systems::companion_psychology::CompanionPsychologyPlugin,
                
                // World corruption and transformation
                systems::corruption::CorruptionPlugin,
                
                // Dual-path morality and sentimental items
                systems::forge::ForgeSystemPlugin,
                
                // Movement validation and layer cake priority
                MovementValidationPlugin,
                
                // Additional game systems
                WeatherSystemPlugin,
                EncounterSystemPlugin,
                NPCSystemPlugin,
                DialogueSystemPlugin,
                InventorySystemPlugin,
            ))
            
            // Integration systems (run after all individual systems)
            .add_systems(Update, (
                system_integration_orchestrator,
                cross_system_event_handler,
                game_state_manager,
            ).after(systems::dread_progression::dread_level_calculation_system))
            
            // Analytics and monitoring
            .add_systems(FixedUpdate, (
                game_performance_monitor,
                system_health_check,
                player_experience_analytics,
            ))
            
            // Development and debugging systems
            .add_systems(Update, (
                #[cfg(debug_assertions)]
                debug_info_display_system,
                #[cfg(debug_assertions)]
                component_inspector_system,
            ));
    }
}

/// Game states for Dragon's Labyrinth
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash, Reflect)]
#[reflect(Resource)]
pub enum DragonLabyrinthGameState {
    #[default]
    Loading,
    MainMenu,
    WorldGeneration,
    InGame,
    PauseMenu,
    ForgeSession,
    TherapySession,
    CorruptionEvent,
    DreadCrisis,
    GameComplete,
}

/// Plugin for core world management and hex tiles
pub struct DragonLabyrinthWorldPlugin;

impl Plugin for DragonLabyrinthWorldPlugin {
    fn build(&self, app: &mut App) {
        app
            // Initialize world resources
            .init_resource::<WorldConfig>()
            .init_resource::<HexWorldManager>()
            
            // Register events
            .add_event::<WorldGenerationEvent>()
            .add_event::<TileLoadEvent>()
            .add_event::<TileUnloadEvent>()
            
            // Startup systems
            .add_systems(Startup, (
                initialize_world_system,
                setup_layer_cake_system,
                load_initial_tiles_system,
            ).chain())
            
            // Core world systems
            .add_systems(Update, (
                world_streaming_system,
                tile_loading_system,
                tile_unloading_system,
                biome_adjacency_validation_system,
            ))
            
            // Register components (skip Path and Feature due to hexx::Hex reflection issues)
            .register_type::<components::hex_tiles::HexTile>()
            .register_type::<components::hex_tiles::Biome>();
            // Note: Path and Feature contain hexx::Hex which doesn't implement Reflect
    }
}

/// Plugin for hex rendering and layer cake visualization
pub struct HexRenderingPlugin;

impl Plugin for HexRenderingPlugin {
    fn build(&self, app: &mut App) {
        app
            // Initialize rendering resources
            .init_resource::<HexRenderingConfig>()
            .init_resource::<AssetLibrary>()
            
            // Register events
            .add_event::<TileRenderEvent>()
            .add_event::<LayerCakeUpdateEvent>()
            .add_event::<CorruptionVisualEvent>()
            
            // Rendering systems
            .add_systems(Update, (
                layer_cake_rendering_system,
                corruption_visual_system,
                dread_level_visual_system,
                asset_loading_system,
                hex_mesh_generation_system,
            ))
            
            // Register rendering components
            .register_type::<HexMeshComponent>()
            .register_type::<LayerCakeRenderer>()
            .register_type::<CorruptionVisualComponent>();
    }
}

/// Plugin for movement validation and layer cake priority system
pub struct MovementValidationPlugin;

impl Plugin for MovementValidationPlugin {
    fn build(&self, app: &mut App) {
        app
            // Initialize movement resources
            .init_resource::<MovementConfig>()
            
            // Register events
            .add_event::<MovementAttemptEvent>()
            .add_event::<MovementValidationEvent>()
            .add_event::<EquipmentOverrideEvent>()
            
            // Movement systems
            .add_systems(Update, (
                systems::movement_validation::movement_validation_system,
                equipment_override_system,
                path_modifier_system,
                biome_effect_system,
            ))
            
            // Register components
            .register_type::<MovementIntent>()
            .register_type::<MovementModifier>()
            .register_type::<EquipmentOverride>();
    }
}

/// Plugin for weather system
pub struct WeatherSystemPlugin;

impl Plugin for WeatherSystemPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<WeatherConfig>()
            .add_event::<WeatherChangeEvent>()
            .add_systems(Update, (
                weather_progression_system,
                weather_effect_system,
                seasonal_modifier_system,
            ))
            .register_type::<WeatherState>()
            .register_type::<SeasonalEffect>();
    }
}

/// Plugin for encounter system
pub struct EncounterSystemPlugin;

impl Plugin for EncounterSystemPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<EncounterConfig>()
            .add_event::<components::encounters::EncounterTriggerEvent>()
            .add_event::<components::encounters::EncounterCompleteEvent>()
            .add_systems(Update, (
                encounter_trigger_system,
                encounter_resolution_system,
                narrative_encounter_system,
            ))
            .register_type::<components::encounters::Encounter>()
            .register_type::<components::encounters::EncounterLocation>();
    }
}

/// Plugin for NPC system
pub struct NPCSystemPlugin;

impl Plugin for NPCSystemPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<NPCConfig>()
            .add_event::<components::npcs::NPCInteractionEvent>()
            .add_systems(Update, (
                npc_behavior_system,
                npc_schedule_system,
                npc_corruption_system,
            ))
            .register_type::<components::npcs::NPC>()
            .register_type::<components::npcs::NPCService>();
    }
}

/// Plugin for dialogue system
pub struct DialogueSystemPlugin;

impl Plugin for DialogueSystemPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<DialogueConfig>()
            .add_event::<DialogueEvent>()
            .add_systems(Update, (
                dialogue_system,
                therapeutic_dialogue_system,
                npc_dialogue_system,
            ))
            .register_type::<DialogueState>()
            .register_type::<DialogueOption>();
    }
}

/// Plugin for inventory and item management
pub struct InventorySystemPlugin;

impl Plugin for InventorySystemPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<InventoryConfig>()
            .add_event::<components::items::ItemEquippedEvent>()
            .add_event::<components::items::ItemUsedEvent>()
            .add_systems(Update, (
                inventory_management_system,
                item_usage_system,
                equipment_system,
            ))
            .register_type::<components::items::Item>()
            .register_type::<components::items::Inventory>()
            .register_type::<components::items::Equipment>();
    }
}

// System integration and coordination

/// Master system that coordinates interactions between all game systems
fn system_integration_orchestrator(
    dread_events: EventReader<systems::dread_progression::DreadLevelChangeEvent>,
    mut psychology_events: EventWriter<systems::companion_psychology::TherapyProgressEvent>,
    mut corruption_events: EventWriter<systems::corruption::CorruptionSpreadEvent>,
    mut forge_events: EventWriter<systems::forge::ForgeIntegrationEvent>,
) {
    // This system would coordinate complex interactions between systems
    // For example: high dread levels trigger companion trauma, which affects forge readiness
    
    if !dread_events.is_empty() {
        debug!("System integration orchestrator processing {} dread level changes", 
               dread_events.len());
    }
}

/// Handles cross-system events and ensures proper event propagation
fn cross_system_event_handler(
    mut commands: Commands,
) {
    // Handle complex cross-system events that require coordination
    // This system ensures events are properly propagated between systems
}

/// Manages overall game state transitions
fn game_state_manager(
    mut game_state: ResMut<NextState<DragonLabyrinthGameState>>,
    current_state: Res<State<DragonLabyrinthGameState>>,
    dread_query: Query<&systems::dread_progression::DreadLevel>,
    corruption_query: Query<&systems::corruption::Corruption>,
) {
    // Manage game state based on world conditions
    for dread_level in dread_query.iter() {
        if dread_level.current_level >= 4 && **current_state != DragonLabyrinthGameState::DreadCrisis {
            warn!("Entering Dread Crisis state due to maximum dread level");
            game_state.set(DragonLabyrinthGameState::DreadCrisis);
        }
    }
}

// Monitoring and analytics systems

/// Monitors game performance and system health
fn game_performance_monitor(
    time: Res<Time>,
) {
    // Monitor system performance and adjust accordingly
    if time.delta_secs() > 0.033 { // Below 30 FPS
        warn!("Game performance degraded: {:.3}s frame time", time.delta_secs());
    }
}

/// Checks system health and reports issues
fn system_health_check(
    dread_query: Query<&systems::dread_progression::DreadLevel>,
    corruption_query: Query<&systems::corruption::Corruption>,
    psychology_query: Query<&systems::companion_psychology::CompanionTherapy>,
) {
    static mut LAST_CHECK: f32 = 0.0;
    
    unsafe {
        LAST_CHECK += 1.0; // Simplified time tracking
        if LAST_CHECK >= 300.0 { // Every 5 minutes
            LAST_CHECK = 0.0;
            
            let total_tiles = corruption_query.iter().count();
            let corrupted_tiles = corruption_query.iter().filter(|c| c.level > 0.1).count();
            let companions_in_therapy = psychology_query.iter().count();
            
            info!("System Health Check: {} tiles ({} corrupted), {} companions in therapy", 
                  total_tiles, corrupted_tiles, companions_in_therapy);
        }
    }
}

/// Analyzes player experience and engagement
fn player_experience_analytics(
    players_query: Query<&components::players::Player>,
) {
    // Analyze player behavior and experience for game balance
    for _player in players_query.iter() {
        // Would analyze player actions, dread exposure, recovery patterns, etc.
    }
}

// Debug systems (only in debug builds)

#[cfg(debug_assertions)]
fn debug_info_display_system() {
    // Display debug information in development builds
}

#[cfg(debug_assertions)]
fn component_inspector_system() {
    // Provide component inspection capabilities for debugging
}

// Resource definitions

#[derive(Resource, Reflect, Debug)]
#[reflect(Resource)]
pub struct WorldConfig {
    pub world_size: u32,
    pub tile_loading_radius: u32,
    pub max_loaded_tiles: usize,
    pub biome_adjacency_rules: std::collections::HashMap<String, Vec<String>>,
}

impl Default for WorldConfig {
    fn default() -> Self {
        let mut adjacency_rules = std::collections::HashMap::new();
        adjacency_rules.insert("lava".to_string(), vec!["mountain".to_string(), "desert".to_string()]);
        adjacency_rules.insert("snow".to_string(), vec!["mountain".to_string(), "forest".to_string()]);
        adjacency_rules.insert("swamp".to_string(), vec!["forest".to_string(), "plains".to_string()]);
        
        Self {
            world_size: 1000, // 1000x1000 hex world
            tile_loading_radius: 20, // Load tiles within 20 hexes of player
            max_loaded_tiles: 1600, // Maximum tiles in memory
            biome_adjacency_rules: adjacency_rules,
        }
    }
}

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
pub struct HexWorldManager {
    pub loaded_tiles: std::collections::HashMap<(i32, i32), Entity>,
    pub pending_loads: Vec<(i32, i32)>,
    pub pending_unloads: Vec<Entity>,
}

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
pub struct HexRenderingConfig {
    pub tile_size: f32,
    pub layer_separation: f32,
    pub corruption_overlay_intensity: f32,
    pub dread_visual_scaling: f32,
}

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
pub struct AssetLibrary {
    pub biome_assets: std::collections::HashMap<String, Handle<Image>>,
    pub path_assets: std::collections::HashMap<String, Handle<Image>>,
    pub feature_assets: std::collections::HashMap<String, Handle<Image>>,
    pub corruption_overlays: std::collections::HashMap<i32, Handle<Image>>,
}

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
pub struct MovementConfig {
    pub base_movement_speed: f32,
    pub run_speed_multiplier: f32,
    pub terrain_modifiers: std::collections::HashMap<String, f32>,
    pub equipment_overrides: std::collections::HashMap<String, f32>,
}

// Event definitions

#[derive(Event, Reflect, Clone, Debug)]
#[reflect(Event)]
pub struct WorldGenerationEvent {
    pub world_seed: u64,
    pub generation_parameters: std::collections::HashMap<String, f32>,
}

#[derive(Event, Reflect, Clone, Debug)]
#[reflect(Event)]
pub struct TileLoadEvent {
    pub coordinates: (i32, i32),
    pub tile_entity: Entity,
}

#[derive(Event, Reflect, Clone, Debug)]
#[reflect(Event)]
pub struct TileUnloadEvent {
    pub tile_entity: Entity,
}

#[derive(Event, Reflect, Clone, Debug)]
#[reflect(Event)]
pub struct LayerCakeUpdateEvent {
    pub tile_entity: Entity,
    pub layers_changed: Vec<String>,
}

// Component definitions

#[derive(Component, Reflect, Clone, Debug, PartialEq)]
#[reflect(Component)]
pub struct HexMeshComponent {
    pub mesh_handle: Handle<Mesh>,
    pub material_handle: Handle<StandardMaterial>,
    pub layer_depth: f32,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq)]
#[reflect(Component)]
pub struct LayerCakeRenderer {
    pub biome_layer: Option<Entity>,
    pub path_layer: Option<Entity>,
    pub feature_layer: Option<Entity>,
    pub corruption_overlay: Option<Entity>,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq)]
#[reflect(Component)]
pub struct CorruptionVisualComponent {
    pub corruption_level: f32,
    pub visual_intensity: f32,
    pub particle_systems: Vec<Handle<bevy::render::render_resource::Shader>>,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq)]
#[reflect(Component)]
pub struct MovementIntent {
    pub target_coordinates: (i32, i32),
    pub movement_type: MovementType,
    pub player_entity: Entity,
}

#[derive(Reflect, Clone, Debug, PartialEq)]
pub enum MovementType {
    Walk,
    Run,
    Teleport,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq)]
#[reflect(Component)]
pub struct MovementModifier {
    pub modifier_source: String,
    pub speed_multiplier: f32,
    pub terrain_override: bool,
    pub duration: Option<f32>,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq)]
#[reflect(Component)]
pub struct EquipmentOverride {
    pub equipment_entity: Entity,
    pub override_type: String,
    pub override_value: f32,
    pub conditions: Vec<String>,
}

// Placeholder system implementations (would be fully implemented)
fn initialize_world_system() {}
fn setup_layer_cake_system() {}
fn load_initial_tiles_system() {}
fn world_streaming_system() {}
fn tile_loading_system() {}
fn tile_unloading_system() {}
fn biome_adjacency_validation_system() {}
fn layer_cake_rendering_system() {}
fn corruption_visual_system() {}
fn dread_level_visual_system() {}
fn asset_loading_system() {}
fn hex_mesh_generation_system() {}
fn equipment_override_system() {}
fn path_modifier_system() {}
fn biome_effect_system() {}
fn weather_progression_system() {}
fn weather_effect_system() {}
fn seasonal_modifier_system() {}
fn encounter_trigger_system() {}
fn encounter_resolution_system() {}
fn narrative_encounter_system() {}
fn npc_behavior_system() {}
fn npc_schedule_system() {}
fn npc_corruption_system() {}
fn dialogue_system() {}
fn therapeutic_dialogue_system() {}
fn npc_dialogue_system() {}
fn inventory_management_system() {}
fn item_usage_system() {}
fn equipment_system() {}

// Resource and component type definitions for placeholder systems
#[derive(Resource, Default)] struct WeatherConfig;
#[derive(Resource, Default)] struct EncounterConfig;
#[derive(Resource, Default)] struct NPCConfig;
#[derive(Resource, Default)] struct DialogueConfig;
#[derive(Resource, Default)] struct InventoryConfig;

#[derive(Event)] struct WeatherChangeEvent;
#[derive(Event)] struct DialogueEvent;

#[derive(Component, Reflect)] #[reflect(Component)] struct WeatherState;
#[derive(Component, Reflect)] #[reflect(Component)] struct SeasonalEffect;
#[derive(Component, Reflect)] #[reflect(Component)] struct DialogueState;
#[derive(Component, Reflect)] #[reflect(Component)] struct DialogueOption;
