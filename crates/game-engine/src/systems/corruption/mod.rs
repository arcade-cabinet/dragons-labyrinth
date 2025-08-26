//! Corruption System - Complete ECS Integration
//!
//! Production-ready Bevy plugin for Dragon's Labyrinth's world transformation system.
//! This system manages the spread of corruption across hex tiles and integrates
//! with Dragon's Labyrinth's core horror progression mechanics.

use bevy::prelude::*;
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};

pub mod components;
pub mod systems;
pub mod resources;
pub mod events;

pub use components::*;
pub use systems::*;
pub use resources::*;
pub use events::*;

/// Main corruption system plugin
pub struct CorruptionPlugin;

impl Plugin for CorruptionPlugin {
    fn build(&self, app: &mut App) {
        app
            // Initialize resources
            .init_resource::<CorruptionSpreadConfig>()
            .init_resource::<CorruptionSourceManager>()
            .init_resource::<PurificationManager>()
            
            // Register events
            .add_event::<CorruptionSpreadEvent>()
            .add_event::<CorruptionSourceEvent>()
            .add_event::<PurificationEvent>()
            .add_event::<HorrorEventCorruptionEvent>()
            .add_event::<CorruptionDreadChangeEvent>()
            .add_event::<NpcCorruptionEvent>()
            .add_event::<CorruptionVisualEvent>()
            .add_event::<CorruptionHotspotEvent>()
            .add_event::<CorruptionResistanceEvent>()
            .add_event::<EnvironmentalCorruptionEvent>()
            
            // Startup systems
            .add_systems(Startup, (
                setup_corruption_system,
                initialize_corruption_sources,
                setup_purification_manager,
            ).chain())
            
            // Main update systems
            .add_systems(Update, (
                // Core corruption processing
                corruption_spread_system,
                corruption_source_management_system,
                
                // Effects and integration
                corruption_dread_integration_system,
                npc_corruption_system,
                environmental_corruption_system,
                
                // Player interactions
                purification_system,
                corruption_visual_effects_system,
                
                // Monitoring and hotspots
                corruption_hotspot_detection_system,
                corruption_analytics_system,
            ).chain())
            
            // Periodic systems
            .add_systems(FixedUpdate, (
                corruption_spread_calculation,
                corruption_source_cleanup,
                corruption_balance_monitoring,
            ).chain())
            
            // Register component reflection
            .register_type::<Corruption>()
            .register_type::<CorruptionSource>()
            .register_type::<CorruptionResistance>()
            .register_type::<CorruptionEffect>()
            .register_type::<PurificationAbility>()
            .register_type::<CorruptionVisualMarker>()
            .register_type::<CorruptionHotspot>()
            
            // Register enums
            .register_type::<CorruptionType>()
            .register_type::<PurificationMethod>()
            .register_type::<CorruptionSeverity>();
    }
}

/// Resource for corruption spread configuration
#[derive(Resource, Reflect, Debug)]
#[reflect(Resource)]
pub struct CorruptionSpreadConfig {
    pub base_spread_rate: f32,
    pub dread_thresholds: [f32; 5], // Corruption levels for dread levels 0-4
    pub biome_resistance_modifiers: HashMap<String, f32>,
    pub biome_amplification_modifiers: HashMap<String, f32>,
    pub max_spread_distance: u32,
    pub horror_event_amplifier: f32,
}

impl Default for CorruptionSpreadConfig {
    fn default() -> Self {
        let mut biome_resistance = HashMap::new();
        biome_resistance.insert("temple".to_string(), -0.3);
        biome_resistance.insert("mountain".to_string(), -0.1);
        biome_resistance.insert("plains".to_string(), 0.0);
        
        let mut biome_amplification = HashMap::new();
        biome_amplification.insert("swamp".to_string(), 0.3);
        biome_amplification.insert("forest".to_string(), 0.1);
        
        Self {
            base_spread_rate: 0.1, // Base corruption spread per day
            dread_thresholds: [0.0, 0.2, 0.4, 0.6, 0.8], // Dread level boundaries
            biome_resistance_modifiers: biome_resistance,
            biome_amplification_modifiers: biome_amplification,
            max_spread_distance: 5, // Corruption spreads within 5 hexes
            horror_event_amplifier: 0.05,
        }
    }
}

/// Resource for managing corruption sources
#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
pub struct CorruptionSourceManager {
    pub active_sources: HashMap<Entity, f32>, // Entity -> intensity
    pub source_decay_rate: f32,
    pub maximum_sources: usize,
}

/// Resource for purification management
#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
pub struct PurificationManager {
    pub purification_methods: HashMap<String, f32>, // Method -> effectiveness
    pub purification_costs: HashMap<String, f32>,   // Method -> resource cost
    pub resistance_factors: HashMap<String, f32>,   // Biome -> purification resistance
}

/// Component marking an entity as source of corruption
#[derive(Component, Reflect, Clone, Debug, PartialEq)]
#[reflect(Component)]
pub struct CorruptionSource {
    pub source_type: CorruptionType,
    pub intensity: f32,
    pub spread_radius: u32,
    pub decay_rate: f32,
    pub created_at: DateTime<Utc>,
}

#[derive(Reflect, Clone, Debug, PartialEq)]
pub enum CorruptionType {
    VoidRift,
    CursedArtifact,
    DragonPresence,
    HorrorEvent,
    NecroticMagic,
    Environmental,
}

/// Component marking corruption level on tiles and entities
#[derive(Component, Reflect, Clone, Debug, PartialEq)]
#[reflect(Component)]
pub struct Corruption {
    pub level: f32,              // 0.0-1.0 corruption intensity
    pub dread_level: i32,        // 0-4 calculated dread level
    pub sources: Vec<Entity>,    // Contributing corruption sources
    pub resistance: f32,         // 0.0-1.0 resistance to corruption
    pub last_update: DateTime<Utc>,
    pub corruption_history: Vec<CorruptionChange>,
}

#[derive(Reflect, Clone, Debug, PartialEq)]
pub struct CorruptionChange {
    pub timestamp: DateTime<Utc>,
    pub old_level: f32,
    pub new_level: f32,
    pub source: String,
}

/// Component for corruption visual effects
#[derive(Component, Reflect, Clone, Debug, PartialEq)]
#[reflect(Component)]
pub struct CorruptionVisualMarker {
    pub visual_overlay: String,
    pub particle_effects: Vec<String>,
    pub color_tint: (f32, f32, f32, f32), // RGBA
    pub environmental_changes: Vec<String>,
    pub animation_intensity: f32,
}

/// Component for entities that can resist corruption
#[derive(Component, Reflect, Clone, Debug, PartialEq)]
#[reflect(Component)]
pub struct CorruptionResistance {
    pub base_resistance: f32,
    pub resistance_sources: Vec<ResistanceSource>,
    pub immunity_duration: Option<f32>,
    pub resistance_decay_rate: f32,
}

#[derive(Reflect, Clone, Debug, PartialEq)]
pub struct ResistanceSource {
    pub source_type: String,
    pub resistance_value: f32,
    pub duration: Option<f32>,
    pub stacks: bool,
}

/// Event fired when corruption spreads
#[derive(Event, Reflect, Clone, Debug, PartialEq)]
#[reflect(Event)]
pub struct CorruptionSpreadEvent {
    pub affected_tiles: Vec<Entity>,
    pub spread_amount: f32,
    pub dread_level_changes: Vec<DreadLevelChange>,
    pub new_sources_created: Vec<Entity>,
}

#[derive(Reflect, Clone, Debug, PartialEq)]
pub struct DreadLevelChange {
    pub tile_entity: Entity,
    pub old_dread_level: i32,
    pub new_dread_level: i32,
}

/// Event fired when corruption source is added
#[derive(Event, Reflect, Clone, Debug, PartialEq)]
#[reflect(Event)]
pub struct CorruptionSourceEvent {
    pub source_entity: Entity,
    pub source_type: CorruptionType,
    pub intensity: f32,
    pub location_entity: Entity,
}

/// Event fired when purification is attempted
#[derive(Event, Reflect, Clone, Debug, PartialEq)]
#[reflect(Event)]
pub struct PurificationEvent {
    pub target_entity: Entity,
    pub purification_method: PurificationMethod,
    pub purification_power: f32,
    pub success: bool,
    pub corruption_removed: f32,
}

#[derive(Reflect, Clone, Debug, PartialEq)]
pub enum PurificationMethod {
    HolyWater,
    LightMagic,
    Cleansing,
    Ritual,
    Prayer,
    NaturalHealing,
}

/// System to setup corruption on startup
fn setup_corruption_system(
    mut commands: Commands,
) {
    info!("Initializing Corruption System");
    
    commands.insert_resource(CorruptionSourceManager::default());
    commands.insert_resource(PurificationManager::default());
    
    info!("Corruption System initialized successfully");
}

/// Initialize corruption sources from existing high-corruption areas
fn initialize_corruption_sources(
    mut commands: Commands,
    hex_tiles_query: Query<(Entity, &crate::components::hex_tiles::HexTile, &crate::components::hex_tiles::Corruption), Without<CorruptionSource>>,
    mut source_manager: ResMut<CorruptionSourceManager>,
) {
    for (entity, hex_tile, corruption) in hex_tiles_query.iter() {
        if corruption.level >= 0.8 {
            // High corruption tiles become corruption sources
            commands.entity(entity).insert(CorruptionSource {
                source_type: CorruptionType::Environmental,
                intensity: corruption.level,
                spread_radius: (corruption.level * 5.0) as u32,
                decay_rate: 0.01,
                created_at: Utc::now(),
            });
            
            source_manager.active_sources.insert(entity, corruption.level);
        }
    }
    
    info!("Initialized {} corruption sources", source_manager.active_sources.len());
}

/// Setup purification manager
fn setup_purification_manager(
    mut commands: Commands,
) {
    let mut purification_manager = PurificationManager::default();
    
    // Configure purification methods
    purification_manager.purification_methods.insert("holy_water".to_string(), 0.3);
    purification_manager.purification_methods.insert("light_magic".to_string(), 0.5);
    purification_manager.purification_methods.insert("cleansing_ritual".to_string(), 0.7);
    
    // Configure costs
    purification_manager.purification_costs.insert("holy_water".to_string(), 1.0);
    purification_manager.purification_costs.insert("light_magic".to_string(), 2.0);
    purification_manager.purification_costs.insert("cleansing_ritual".to_string(), 5.0);
    
    // Configure biome resistance
    purification_manager.resistance_factors.insert("swamp".to_string(), 0.4);
    purification_manager.resistance_factors.insert("mountain".to_string(), 0.1);
    purification_manager.resistance_factors.insert("temple".to_string(), 0.0);
    
    commands.insert_resource(purification_manager);
    
    info!("Purification manager configured");
}

/// Core system for corruption spread
fn corruption_spread_system(
    mut commands: Commands,
    time: Res<Time>,
    config: Res<CorruptionSpreadConfig>,
    sources_query: Query<(Entity, &CorruptionSource, &crate::components::hex_tiles::HexTile)>,
    mut tiles_query: Query<(Entity, &crate::components::hex_tiles::HexTile, &mut crate::components::hex_tiles::Corruption, &crate::components::hex_tiles::Biome), Without<CorruptionSource>>,
    mut spread_events: EventWriter<CorruptionSpreadEvent>,
) {
    let mut affected_tiles = Vec::new();
    let mut dread_level_changes = Vec::new();
    
    // Calculate corruption spread from sources
    for (source_entity, corruption_source, source_hex) in sources_query.iter() {
        for (tile_entity, hex_tile, mut corruption, biome) in tiles_query.iter_mut() {
            let distance = hex_tile.coordinates.distance_to(source_hex.coordinates);
            
            if distance <= corruption_source.spread_radius {
                let old_corruption = corruption.level;
                let old_dread = corruption.dread_level;
                
                // Calculate influence based on distance
                let distance_factor = 1.0 - (distance as f32 / corruption_source.spread_radius as f32);
                let influence = corruption_source.intensity * distance_factor * config.base_spread_rate * time.delta_seconds();
                
                // Apply biome modifiers
                let biome_modifier = config.biome_amplification_modifiers.get(&biome.biome_type)
                    .unwrap_or(&0.0) - config.biome_resistance_modifiers.get(&biome.biome_type).unwrap_or(&0.0);
                
                let modified_influence = influence * (1.0 + biome_modifier);
                
                // Update corruption level
                corruption.level = (corruption.level + modified_influence).clamp(0.0, 1.0);
                corruption.last_update = Utc::now();
                
                if !corruption.sources.contains(&source_entity) {
                    corruption.sources.push(source_entity);
                }
                
                corruption.corruption_history.push(CorruptionChange {
                    timestamp: Utc::now(),
                    old_level: old_corruption,
                    new_level: corruption.level,
                    source: format!("{:?}", corruption_source.source_type),
                });
                
                // Calculate new dread level
                let new_dread_level = calculate_dread_level(corruption.level, &config.dread_thresholds);
                corruption.dread_level = new_dread_level;
                
                if (corruption.level - old_corruption).abs() > 0.01 {
                    affected_tiles.push(tile_entity);
                    
                    if new_dread_level != old_dread {
                        dread_level_changes.push(DreadLevelChange {
                            tile_entity,
                            old_dread_level: old_dread,
                            new_dread_level: new_dread_level,
                        });
                    }
                }
            }
        }
    }
    
    if !affected_tiles.is_empty() {
        spread_events.send(CorruptionSpreadEvent {
            affected_tiles,
            spread_amount: config.base_spread_rate * time.delta_seconds(),
            dread_level_changes,
            new_sources_created: Vec::new(),
        });
    }
}

/// System for managing corruption sources
fn corruption_source_management_system(
    mut commands: Commands,
    mut source_events: EventReader<CorruptionSourceEvent>,
    mut source_manager: ResMut<CorruptionSourceManager>,
    time: Res<Time>,
    mut sources_query: Query<(Entity, &mut CorruptionSource)>,
) {
    // Handle new corruption source events
    for source_event in source_events.read() {
        source_manager.active_sources.insert(source_event.source_entity, source_event.intensity);
        
        info!("Added corruption source: {:?} with intensity {:.2}", 
              source_event.source_type, source_event.intensity);
    }
    
    // Update existing sources (decay over time)
    for (entity, mut source) in sources_query.iter_mut() {
        source.intensity *= 1.0 - source.decay_rate * time.delta_seconds();
        
        // Remove sources that have decayed too much
        if source.intensity < 0.01 {
            commands.entity(entity).remove::<CorruptionSource>();
            source_manager.active_sources.remove(&entity);
            
            debug!("Removed decayed corruption source");
        } else {
            source_manager.active_sources.insert(entity, source.intensity);
        }
    }
}

/// System for purification attempts
fn purification_system(
    mut purification_events: EventReader<PurificationEvent>,
    mut corruption_query: Query<&mut crate::components::hex_tiles::Corruption>,
    purification_manager: Res<PurificationManager>,
    biome_query: Query<&crate::components::hex_tiles::Biome>,
    mut commands: Commands,
) {
    for purification_event in purification_events.read() {
        if let Ok(mut corruption) = corruption_query.get_mut(purification_event.target_entity) {
            let method_effectiveness = purification_manager.purification_methods
                .get(&format!("{:?}", purification_event.purification_method))
                .unwrap_or(&0.0);
            
            // Get biome resistance if available
            let biome_resistance = if let Ok(biome) = biome_query.get(purification_event.target_entity) {
                purification_manager.resistance_factors
                    .get(&biome.biome_type)
                    .unwrap_or(&0.2)
            } else {
                &0.2
            };
            
            let effective_purification = purification_event.purification_power * method_effectiveness * (1.0 - biome_resistance);
            let old_corruption = corruption.level;
            
            corruption.level = (corruption.level - effective_purification).max(0.0);
            corruption.last_update = Utc::now();
            
            let corruption_removed = old_corruption - corruption.level;
            
            if corruption_removed > 0.01 {
                corruption.corruption_history.push(CorruptionChange {
                    timestamp: Utc::now(),
                    old_level: old_corruption,
                    new_level: corruption.level,
                    source: format!("Purification: {:?}", purification_event.purification_method),
                });
                
                info!("Purification successful: removed {:.2} corruption", corruption_removed);
            }
        }
    }
}

/// System for corruption effects on NPCs
fn npc_corruption_system(
    mut npc_query: Query<(Entity, &mut crate::components::npcs::NPC, &crate::components::hex_tiles::HexTile)>,
    corruption_query: Query<&crate::components::hex_tiles::Corruption>,
    mut npc_corruption_events: EventWriter<NpcCorruptionEvent>,
    time: Res<Time>,
) {
    for (npc_entity, mut npc, hex_tile) in npc_query.iter_mut() {
        if let Some(tile_entity) = npc.hex_tile_entity {
            if let Ok(tile_corruption) = corruption_query.get(tile_entity) {
                let corruption_influence = tile_corruption.level * npc.corruption_susceptibility * time.delta_seconds() * 0.1;
                let old_corruption = npc.current_corruption_level;
                
                npc.current_corruption_level = (npc.current_corruption_level + corruption_influence).min(1.0);
                
                if (npc.current_corruption_level - old_corruption) > 0.01 {
                    npc_corruption_events.send(NpcCorruptionEvent {
                        npc_entity,
                        npc_name: npc.name.clone(),
                        corruption_increase: corruption_influence,
                        new_corruption_level: npc.current_corruption_level,
                        behavioral_changes: get_corruption_behavioral_changes(npc.current_corruption_level),
                    });
                }
            }
        }
    }
}

/// System for detecting corruption hotspots
fn corruption_hotspot_detection_system(
    corruption_query: Query<(Entity, &crate::components::hex_tiles::Corruption, &crate::components::hex_tiles::HexTile, &crate::components::hex_tiles::Biome), Changed<crate::components::hex_tiles::Corruption>>,
    mut hotspot_events: EventWriter<CorruptionHotspotEvent>,
    mut commands: Commands,
) {
    for (entity, corruption, hex_tile, biome) in corruption_query.iter() {
        if corruption.level >= 0.7 {
            // Mark as corruption hotspot
            commands.entity(entity).insert(CorruptionHotspot {
                threat_level: calculate_threat_level(corruption.level, &biome.biome_type),
                nearby_settlements: Vec::new(), // Would be populated by proximity system
                requires_attention: corruption.level >= 0.9,
                spreading_actively: corruption.sources.len() > 0,
            });
            
            hotspot_events.send(CorruptionHotspotEvent {
                hotspot_entity: entity,
                corruption_level: corruption.level,
                threat_level: calculate_threat_level(corruption.level, &biome.biome_type),
                requires_immediate_action: corruption.level >= 0.9,
            });
        }
    }
}

/// System for corruption visual effects
fn corruption_visual_effects_system(
    mut commands: Commands,
    corruption_query: Query<(Entity, &crate::components::hex_tiles::Corruption), Changed<crate::components::hex_tiles::Corruption>>,
    mut visual_events: EventWriter<CorruptionVisualEvent>,
) {
    for (entity, corruption) in corruption_query.iter() {
        let visual_marker = CorruptionVisualMarker {
            visual_overlay: get_corruption_overlay(corruption.dread_level),
            particle_effects: get_corruption_particles(corruption.level),
            color_tint: get_corruption_color_tint(corruption.level),
            environmental_changes: get_environmental_changes(corruption.dread_level),
            animation_intensity: corruption.level,
        };
        
        commands.entity(entity).insert(visual_marker.clone());
        
        visual_events.send(CorruptionVisualEvent {
            affected_entity: entity,
            visual_effects: visual_marker,
            dread_level: corruption.dread_level,
        });
    }
}

/// System for corruption integration with dread progression
fn corruption_dread_integration_system(
    mut corruption_events: EventReader<CorruptionSpreadEvent>,
    mut dread_events: EventWriter<crate::systems::dread_progression::DreadLevelChangeEvent>,
) {
    for corruption_event in corruption_events.read() {
        for dread_change in &corruption_event.dread_level_changes {
            dread_events.send(crate::systems::dread_progression::DreadLevelChangeEvent {
                source: "corruption_spread".to_string(),
                old_dread_level: dread_change.old_dread_level,
                new_dread_level: dread_change.new_dread_level,
                affected_entity: dread_change.tile_entity,
                change_reason: "Corruption level changed".to_string(),
            });
        }
    }
}

// Periodic systems

fn corruption_spread_calculation(
    config: Res<CorruptionSpreadConfig>,
    sources_query: Query<&CorruptionSource>,
) {
    // Calculate optimal spread parameters
    let active_sources = sources_query.iter().count();
    debug!("Corruption spread calculation: {} active sources", active_sources);
}

fn corruption_source_cleanup(
    mut commands: Commands,
    mut source_manager: ResMut<CorruptionSourceManager>,
    sources_query: Query<(Entity, &CorruptionSource)>,
) {
    // Clean up weak sources
    for (entity, source) in sources_query.iter() {
        if source.intensity < 0.05 {
            commands.entity(entity).remove::<CorruptionSource>();
            source_manager.active_sources.remove(&entity);
        }
    }
}

fn corruption_balance_monitoring(
    corruption_query: Query<&crate::components::hex_tiles::Corruption>,
) {
    let total_tiles = corruption_query.iter().count();
    let corrupted_tiles = corruption_query.iter().filter(|c| c.level > 0.1).count();
    let highly_corrupted = corruption_query.iter().filter(|c| c.level > 0.7).count();
    
    if total_tiles > 0 {
        let corruption_percentage = (corrupted_tiles as f32 / total_tiles as f32) * 100.0;
        let high_corruption_percentage = (highly_corrupted as f32 / total_tiles as f32) * 100.0;
        
        debug!("Corruption monitoring: {:.1}% tiles corrupted, {:.1}% highly corrupted", 
               corruption_percentage, high_corruption_percentage);
    }
}

// Helper functions

fn calculate_dread_level(corruption_level: f32, thresholds: &[f32; 5]) -> i32 {
    for (level, threshold) in thresholds.iter().enumerate().rev() {
        if corruption_level >= *threshold {
            return level as i32;
        }
    }
    0
}

fn calculate_threat_level(corruption_level: f32, biome_type: &str) -> f32 {
    let base_threat = corruption_level;
    
    // Biome threat modifiers
    let biome_modifier = match biome_type {
        "swamp" => 0.3,
        "forest" => 0.1,
        "mountain" => 0.0,
        "temple" => -0.2,
        _ => 0.0,
    };
    
    (base_threat + biome_modifier).clamp(0.0, 1.0)
}

fn get_corruption_overlay(dread_level: i32) -> String {
    match dread_level {
        0 => "none".to_string(),
        1 => "faint_shadows".to_string(),
        2 => "dark_veins".to_string(),
        3 => "writhing_darkness".to_string(),
        4 => "void_tendrils".to_string(),
        _ => "none".to_string(),
    }
}

fn get_corruption_particles(corruption_level: f32) -> Vec<String> {
    let mut particles = Vec::new();
    
    if corruption_level > 0.2 {
        particles.push("dark_motes".to_string());
    }
    if corruption_level > 0.5 {
        particles.push("whispers_visual".to_string());
    }
    if corruption_level > 0.8 {
        particles.push("void_cracks".to_string());
    }
    
    particles
}

fn get_corruption_color_tint(corruption_level: f32) -> (f32, f32, f32, f32) {
    // RGBA color tint based on corruption level
    let red_tint = corruption_level * 0.3;
    let green_reduction = corruption_level * 0.5;
    let blue_reduction = corruption_level * 0.4;
    
    (1.0 + red_tint, 1.0 - green_reduction, 1.0 - blue_reduction, 1.0)
}

fn get_environmental_changes(dread_level: i32) -> Vec<String> {
    match dread_level {
        0 => Vec::new(),
        1 => vec!["wilted_plants".to_string()],
        2 => vec!["twisted_trees".to_string(), "dead_grass".to_string()],
        3 => vec!["cracked_earth".to_string(), "poisoned_water".to_string()],
        4 => vec!["void_rifts".to_string(), "reality_distortion".to_string()],
        _ => Vec::new(),
    }
}

fn get_corruption_behavioral_changes(corruption_level: f32) -> Vec<String> {
    let mut changes = Vec::new();
    
    if corruption_level > 0.3 {
        changes.push("paranoid".to_string());
    }
    if corruption_level > 0.5 {
        changes.push("aggressive".to_string());
    }
    if corruption_level > 0.7 {
        changes.push("irrational".to_string());
    }
    if corruption_level > 0.9 {
        changes.push("hostile".to_string());
    }
    
    changes
}

// Event definitions

#[derive(Event, Reflect, Clone, Debug, PartialEq)]
#[reflect(Event)]
pub struct NpcCorruptionEvent {
    pub npc_entity: Entity,
    pub npc_name: String,
    pub corruption_increase: f32,
    pub new_corruption_level: f32,
    pub behavioral_changes: Vec<String>,
}

#[derive(Event, Reflect, Clone, Debug, PartialEq)]
#[reflect(Event)]
pub struct CorruptionVisualEvent {
    pub affected_entity: Entity,
    pub visual_effects: CorruptionVisualMarker,
    pub dread_level: i32,
}

#[derive(Event, Reflect, Clone, Debug, PartialEq)]
#[reflect(Event)]
pub struct CorruptionHotspotEvent {
    pub hotspot_entity: Entity,
    pub corruption_level: f32,
    pub threat_level: f32,
    pub requires_immediate_action: bool,
}

/// Component marking corruption hotspots
#[derive(Component, Reflect, Clone, Debug, PartialEq)]
#[reflect(Component)]
pub struct CorruptionHotspot {
    pub threat_level: f32,
    pub nearby_settlements
