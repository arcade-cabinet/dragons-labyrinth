//! Game Systems Module - Pure Bevy ECS Integration
//!
//! This module contains all game systems converted from SeaORM database queries to pure Bevy ECS.
//! Features sophisticated horror RPG systems with layer cake hex tiles, dread progression, 
//! companion psychology, and dual-path morality systems.

// Core ECS systems (converted from game-database migration)
pub mod dread_progression;
pub mod companion_psychology;
pub mod corruption;
pub mod forge;
pub mod movement_validation;

// Additional ECS systems
pub mod hex_rendering;
pub mod combat;
pub mod settlement;
pub mod weather;
pub mod faction;
pub mod dungeon;
pub mod encounter;

// Re-export system functions and types for convenience
pub use dread_progression::DreadProgressionPlugin;
pub use companion_psychology::CompanionPsychologyPlugin;
pub use corruption::CorruptionPlugin;
pub use forge::ForgeSystemPlugin;
pub use movement_validation::movement_validation_system;

use bevy::prelude::*;
use std::collections::HashMap;

/// Common coordinate system for hex tiles (using hexx crate integration)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct HexCoordinates {
    pub q: i32,
    pub r: i32,
    pub s: i32,
}

impl HexCoordinates {
    pub fn new(q: i32, r: i32) -> Self {
        Self { q, r, s: -q - r }
    }
    
    pub fn from_hbf_coords(x: i32, y: i32) -> Self {
        Self::new(x, y)
    }
    
    pub fn distance_to(&self, other: HexCoordinates) -> u32 {
        ((self.q - other.q).abs() + (self.r - other.r).abs() + (self.s - other.s).abs()) as u32 / 2
    }
    
    pub fn neighbors(&self) -> Vec<HexCoordinates> {
        vec![
            HexCoordinates::new(self.q + 1, self.r - 1), // NE
            HexCoordinates::new(self.q + 1, self.r),     // E
            HexCoordinates::new(self.q, self.r + 1),     // SE
            HexCoordinates::new(self.q - 1, self.r + 1), // SW
            HexCoordinates::new(self.q - 1, self.r),     // W
            HexCoordinates::new(self.q, self.r - 1),     // NW
        ]
    }
    
    pub fn within_radius(&self, radius: u32) -> Vec<HexCoordinates> {
        let mut results = Vec::new();
        let radius = radius as i32;
        
        for q in -radius..=radius {
            let r_min = (-radius - q).max(-radius);
            let r_max = (-radius - q).min(radius);
            
            for r in r_min..=r_max {
                results.push(HexCoordinates::new(self.q + q, self.r + r));
            }
        }
        
        results
    }
}

/// Common viewport system for hex world rendering
#[derive(Debug, Clone, Reflect)]
pub struct Viewport {
    pub center: HexCoordinates,
    pub radius: u32,
}

impl Viewport {
    pub fn new(center: HexCoordinates, radius: u32) -> Self {
        Self { center, radius }
    }
    
    pub fn contains(&self, coordinates: HexCoordinates) -> bool {
        self.center.distance_to(coordinates) <= self.radius
    }
    
    pub fn get_visible_tiles(&self) -> Vec<HexCoordinates> {
        self.center.within_radius(self.radius)
    }
}

/// Layer cake rendering data for hex tiles
#[derive(Debug, Clone, Reflect)]
pub struct LayerCakeRenderData {
    pub position: HexCoordinates,
    pub biome_layer: BiomeLayer,
    pub path_layer: Option<PathLayer>,
    pub feature_layer: Option<FeatureLayer>,
    pub corruption_overlay: Option<CorruptionOverlay>,
    pub dread_effects: Vec<DreadEffect>,
}

#[derive(Debug, Clone, Reflect)]
pub struct BiomeLayer {
    pub biome_type: String,
    pub texture_id: String,
    pub movement_modifier: f32,
    pub adjacency_rules: Vec<String>,
}

#[derive(Debug, Clone, Reflect)]
pub struct PathLayer {
    pub path_type: String,
    pub texture_id: String,
    pub movement_modifier: f32,
    pub connections: Vec<HexCoordinates>,
}

#[derive(Debug, Clone, Reflect)]
pub struct FeatureLayer {
    pub feature_type: String,
    pub texture_id: String,
    pub interactions: Vec<String>,
    pub blocks_movement: bool,
}

#[derive(Debug, Clone, Reflect)]
pub struct CorruptionOverlay {
    pub corruption_level: f32,
    pub overlay_texture: String,
    pub particle_effects: Vec<String>,
    pub color_tint: (f32, f32, f32, f32), // RGBA
}

#[derive(Debug, Clone, Reflect)]
pub struct DreadEffect {
    pub effect_type: String,
    pub intensity: f32,
    pub visual_changes: Vec<String>,
    pub audio_changes: Vec<String>,
}

/// Combat encounter data for ECS system
#[derive(Debug, Clone, Reflect)]
pub struct CombatEncounter {
    pub encounter_id: uuid::Uuid,
    pub participants: Vec<Entity>,
    pub environment: EncounterEnvironment,
    pub turn_order: Vec<Entity>,
    pub current_turn: usize,
    pub encounter_phase: CombatPhase,
}

#[derive(Debug, Clone, Reflect)]
pub enum CombatPhase {
    Initiative,
    PlayerTurn,
    EnemyTurn,
    Resolution,
    Completed,
}

#[derive(Debug, Clone, Reflect)]
pub struct EncounterEnvironment {
    pub terrain: String,
    pub weather: WeatherCondition,
    pub lighting: LightingCondition,
    pub hazards: Vec<EnvironmentalHazard>,
    pub cover_points: Vec<HexCoordinates>,
}

#[derive(Debug, Clone, Reflect)]
pub struct WeatherCondition {
    pub condition_type: String,
    pub visibility_modifier: f32,
    pub movement_modifier: f32,
    pub combat_effects: Vec<String>,
    pub dread_amplifier: f32, // Weather can increase dread
}

#[derive(Debug, Clone, Reflect)]
pub struct LightingCondition {
    pub light_level: f32, // 0.0-1.0
    pub shadows: bool,
    pub color_temperature: f32,
    pub horror_effects: Vec<String>, // Lighting affects horror
}

#[derive(Debug, Clone, Reflect)]
pub struct EnvironmentalHazard {
    pub hazard_type: String,
    pub affected_area: Vec<HexCoordinates>,
    pub damage_per_turn: Option<i32>,
    pub status_effects: Vec<String>,
}

/// Settlement interaction data
#[derive(Debug, Clone, Reflect)]
pub struct SettlementData {
    pub settlement_id: uuid::Uuid,
    pub name: String,
    pub settlement_type: String,
    pub population: u32,
    pub corruption_level: f32,
    pub available_services: Vec<SettlementService>,
    pub npcs: Vec<Entity>, // NPC entities
    pub dread_effects: Vec<DreadEffect>,
}

#[derive(Debug, Clone, Reflect)]
pub struct SettlementService {
    pub service_type: String,
    pub provider_entity: Entity, // NPC providing the service
    pub cost: i32,
    pub availability: ServiceAvailability,
    pub corruption_affected: bool,
}

#[derive(Debug, Clone, Reflect)]
pub enum ServiceAvailability {
    Always,
    DayOnly,
    NightOnly,
    DreadLevelDependent(i32),
    CorruptionLevelDependent(f32),
}

/// Faction relationship system
#[derive(Debug, Clone, Reflect)]
pub struct FactionData {
    pub faction_id: uuid::Uuid,
    pub name: String,
    pub faction_type: FactionType,
    pub influence_areas: Vec<HexCoordinates>,
    pub relationships: HashMap<Entity, f32>, // Entity -> relationship strength
    pub goals: Vec<FactionGoal>,
    pub resources: FactionResources,
}

#[derive(Debug, Clone, Reflect)]
pub enum FactionType {
    Cult,
    MerchantGuild,
    Militia,
    ReligiousOrder,
    NobleHouse,
    Rebels,
    Corrupted, // Influenced by dread progression
}

#[derive(Debug, Clone, Reflect)]
pub struct FactionGoal {
    pub goal_type: String,
    pub target_entity: Option<Entity>,
    pub target_location: Option<HexCoordinates>,
    pub completion_criteria: Vec<String>,
    pub priority: f32,
}

#[derive(Debug, Clone, Reflect)]
pub struct FactionResources {
    pub gold: i32,
    pub influence: f32,
    pub military_strength: f32,
    pub corruption_resistance: f32,
}

/// Dungeon exploration system
#[derive(Debug, Clone, Reflect)]
pub struct DungeonLayout {
    pub dungeon_id: uuid::Uuid,
    pub name: String,
    pub dungeon_type: String,
    pub total_rooms: u32,
    pub rooms: HashMap<u32, DungeonRoom>,
    pub discovered_rooms: Vec<u32>,
    pub current_room: Option<u32>,
    pub corruption_level: f32,
}

#[derive(Debug, Clone, Reflect)]
pub struct DungeonRoom {
    pub room_id: u32,
    pub title: String,
    pub description: String,
    pub room_type: String,
    pub connections: Vec<RoomConnection>,
    pub features: Vec<RoomFeature>,
    pub encounters: Vec<Entity>, // Encounter entities
    pub loot: Vec<Entity>, // Item entities
    pub discovered: bool,
    pub cleared: bool,
}

#[derive(Debug, Clone, Reflect)]
pub struct RoomConnection {
    pub direction: String,
    pub connects_to: u32, // Room ID
    pub door_type: String,
    pub is_locked: bool,
    pub key_required: Option<Entity>, // Key item entity
    pub hidden: bool,
}

#[derive(Debug, Clone, Reflect)]
pub struct RoomFeature {
    pub feature_type: String,
    pub description: String,
    pub interactive: bool,
    pub interaction_results: Vec<String>,
}

/// System integration helpers for cross-system communication
#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct SystemIntegrationState {
    pub active_integrations: HashMap<String, IntegrationStatus>,
    pub event_queue: Vec<CrossSystemEvent>,
    pub system_health: HashMap<String, SystemHealth>,
}

#[derive(Debug, Clone, Reflect)]
pub struct IntegrationStatus {
    pub integration_name: String,
    pub is_active: bool,
    pub last_update: chrono::DateTime<chrono::Utc>,
    pub integration_strength: f32,
}

#[derive(Debug, Clone, Reflect)]
pub struct CrossSystemEvent {
    pub event_type: String,
    pub source_system: String,
    pub target_system: String,
    pub event_data: HashMap<String, f32>,
    pub priority: EventPriority,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Reflect, PartialEq)]
pub enum EventPriority {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Reflect)]
pub struct SystemHealth {
    pub system_name: String,
    pub is_operational: bool,
    pub performance_score: f32, // 0.0-1.0
    pub error_count: u32,
    pub last_health_check: chrono::DateTime<chrono::Utc>,
}

/// Helper function to create system integration events
pub fn create_cross_system_event(
    event_type: String,
    source: String,
    target: String,
    data: HashMap<String, f32>,
    priority: EventPriority,
) -> CrossSystemEvent {
    CrossSystemEvent {
        event_type,
        source_system: source,
        target_system: target,
        event_data: data,
        priority,
        timestamp: chrono::Utc::now(),
    }
}

/// System for processing cross-system events
pub fn cross_system_event_processor(
    mut integration_state: ResMut<SystemIntegrationState>,
    mut dread_events: EventWriter<dread_progression::DreadLevelChangeEvent>,
    mut psychology_events: EventWriter<companion_psychology::TherapyProgressEvent>,
    mut corruption_events: EventWriter<corruption::CorruptionSpreadEvent>,
    mut forge_events: EventWriter<forge::ForgeIntegrationEvent>,
) {
    // Process queued cross-system events
    let events_to_process: Vec<CrossSystemEvent> = integration_state.event_queue.drain(..).collect();
    
    for event in events_to_process {
        match (event.source_system.as_str(), event.target_system.as_str()) {
            ("dread_progression", "companion_psychology") => {
                // Dread progression affects companion psychology
                if let Some(dread_level) = event.event_data.get("dread_level") {
                    psychology_events.send(companion_psychology::TherapyProgressEvent {
                        companion_entity: Entity::PLACEHOLDER, // Would be populated from event data
                        progress_type: companion_psychology::TherapyProgressType::SetbackOccurred,
                        progress_amount: -*dread_level * 0.1, // Negative progress due to dread
                        related_mission: Some("Dread exposure trauma".to_string()),
                    });
                }
            }
            ("companion_psychology", "dread_progression") => {
                // Companion trauma affects dread levels
                if let Some(trauma_level) = event.event_data.get("trauma_level") {
                    dread_events.send(dread_progression::DreadLevelChangeEvent {
                        source: "companion_trauma".to_string(),
                        old_dread_level: 0, // Would be retrieved from actual state
                        new_dread_level: (*trauma_level * 4.0) as i32, // Scale trauma to dread
                        affected_entity: Entity::PLACEHOLDER,
                        change_reason: "Companion psychological breakdown".to_string(),
                    });
                }
            }
            ("corruption", "dread_progression") => {
                // Corruption spreads affect dread levels
                if let Some(corruption_spread) = event.event_data.get("corruption_amount") {
                    dread_events.send(dread_progression::DreadLevelChangeEvent {
                        source: "corruption_spread".to_string(),
                        old_dread_level: 0,
                        new_dread_level: (*corruption_spread * 2.0) as i32,
                        affected_entity: Entity::PLACEHOLDER,
                        change_reason: "Environmental corruption".to_string(),
                    });
                }
            }
            ("forge", "companion_psychology") => {
                // Forge sacrifices cause psychological trauma
                if let Some(sacrifice_trauma) = event.event_data.get("sacrifice_impact") {
                    psychology_events.send(companion_psychology::TherapyProgressEvent {
                        companion_entity: Entity::PLACEHOLDER,
                        progress_type: companion_psychology::TherapyProgressType::SetbackOccurred,
                        progress_amount: -*sacrifice_trauma,
                        related_mission: Some("Forge sacrifice trauma".to_string()),
                    });
                }
            }
            _ => {
                debug!("Unhandled cross-system event: {} -> {}", event.source_system, event.target_system);
            }
        }
    }
}

/// System health monitoring for all game systems
pub fn system_health_monitor(
    mut integration_state: ResMut<SystemIntegrationState>,
    time: Res<Time>,
) {
    static mut LAST_CHECK: f32 = 0.0;
    
    unsafe {
        LAST_CHECK += time.delta_seconds();
        if LAST_CHECK >= 10.0 { // Check every 10 seconds
            LAST_CHECK = 0.0;
            
            let system_names = vec![
                "dread_progression",
                "companion_psychology", 
                "corruption",
                "forge",
                "movement_validation",
                "hex_rendering",
            ];
            
            for system_name in system_names {
                let health = SystemHealth {
                    system_name: system_name.to_string(),
                    is_operational: true, // Would check actual system status
                    performance_score: 1.0, // Would measure actual performance
                    error_count: 0, // Would track actual errors
                    last_health_check: chrono::Utc::now(),
                };
                
                integration_state.system_health.insert(system_name.to_string(), health);
            }
            
            debug!("System health check completed for {} systems", integration_state.system_health.len());
        }
    }
}

// Type aliases for common ECS queries across systems
pub type HexTileQuery<'w, 's> = Query<'w, 's, (Entity, &'static crate::components::hex_tiles::HexTile, &'static crate::components::hex_tiles::Biome)>;
pub type PlayerQuery<'w, 's> = Query<'w, 's, (Entity, &'static crate::components::players::Player)>;
pub type CompanionQuery<'w, 's> = Query<'w, 's, (Entity, &'static crate::components::companions::Companion)>;
pub type NPCQuery<'w, 's> = Query<'w, 's, (Entity, &'static crate::components::npcs::NPC)>;
pub type CorruptionQuery<'w, 's> = Query<'w, 's, (Entity, &'static crate::components::hex_tiles::Corruption)>;

/// Utility functions for common system operations
pub mod utils {
    use super::*;
    
    /// Calculate distance between two hex coordinates
    pub fn hex_distance(a: HexCoordinates, b: HexCoordinates) -> u32 {
        a.distance_to(b)
    }
    
    /// Get all hex tiles within a radius
    pub fn get_tiles_in_radius(center: HexCoordinates, radius: u32) -> Vec<HexCoordinates> {
        center.within_radius(radius)
    }
    
    /// Check if two biomes are adjacent-compatible
    pub fn biomes_can_be_adjacent(biome_a: &str, biome_b: &str) -> bool {
        match (biome_a, biome_b) {
            ("lava", "snow") | ("snow", "lava") => false,
            ("desert", "swamp") | ("swamp", "desert") => false,
            _ => true, // Most biomes can be adjacent
        }
    }
    
    /// Calculate layer cake priority (Player > Path > Biome)
    pub fn calculate_movement_priority(
        has_player: bool,
        has_path: bool,
        biome_modifier: f32,
        equipment_overrides: &[f32],
    ) -> f32 {
        if has_player {
            // Equipment can override terrain effects
            equipment_overrides.iter().sum::<f32>().max(0.0)
        } else if has_path {
            // Path modifiers override biome
            1.0 // Standard path movement
        } else {
            // Base biome movement modifier
            biome_modifier
        }
    }
    
    /// Calculate dread amplification based on multiple factors
    pub fn calculate_dread_amplification(
        base_dread: f32,
        corruption_level: f32,
        companion_trauma: f32,
        environmental_factors: f32,
    ) -> f32 {
        base_dread * (1.0 + corruption_level + companion_trauma + environmental_factors).min(5.0)
    }
}
