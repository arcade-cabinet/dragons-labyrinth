//! Component organization for Dragon's Labyrinth ECS Architecture
//!
//! This module organizes all game components after the successful SeaORM → Bevy ECS migration.
//! Each module contains related components for specific game systems with pure ECS patterns.

// Core ECS components (converted from game-database migration)
pub mod hex_tiles;
pub mod players;
pub mod companions;
pub mod npcs;
pub mod encounters;
pub mod items;
pub mod forge;
pub mod psychology;

// Additional system components
pub mod world;
pub mod corruption;
pub mod dread_progression;
pub mod weather;
pub mod dialogue;
pub mod narrative;

// Re-export commonly used components for convenience
pub use hex_tiles::{HexTile, Biome, Path, Feature, Corruption};
pub use players::{Player, PlayerStats, DayNightCycle};
pub use companions::{Companion, CompanionState, Mount, Relationship};
pub use npcs::{NPC, NPCService, NPCRelationship};
pub use encounters::{Encounter, EncounterLocation};
pub use items::{Item, Equipment, Inventory, Weapon, Armor};
pub use forge::{SentimentalItem, ForgeProgress, ForgePath};
pub use psychology::{CompanionTherapy, MemoryPalace, TherapyStage};

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Component bundle for spawning a player entity with full ECS integration
#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub stats: PlayerStats,
    pub day_night: DayNightCycle,
    pub inventory: Inventory,
    pub name: Name,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}

/// Component bundle for spawning a companion with psychology system
#[derive(Bundle)]
pub struct CompanionBundle {
    pub companion: Companion,
    pub companion_state: CompanionState,
    pub therapy: psychology::CompanionTherapy,
    pub name: Name,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}

/// Component bundle for spawning an NPC with full interaction system
#[derive(Bundle)]
pub struct NPCBundle {
    pub npc: NPC,
    pub name: Name,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}

/// Component bundle for hex tiles with layer cake system
#[derive(Bundle)]
pub struct HexTileBundle {
    pub hex_tile: HexTile,
    pub biome: Biome,
    pub corruption: hex_tiles::Corruption,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}

/// Component bundle for sentimental items in forge system
#[derive(Bundle)]
pub struct SentimentalItemBundle {
    pub sentimental_item: SentimentalItem,
    pub item: Item,
    pub name: Name,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}

/// Component bundle for encounters
#[derive(Bundle)]
pub struct EncounterBundle {
    pub encounter: Encounter,
    pub encounter_location: EncounterLocation,
    pub name: Name,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

// Common marker components for ECS queries

/// Marker component for entities that are corrupted
#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct CorruptedMarker;

/// Marker component for entities affected by dread
#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct DreadAffectedMarker {
    pub current_dread_level: i32,
    pub dread_resistance: f32,
}

/// Marker component for entities in therapy
#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct InTherapyMarker {
    pub therapy_type: String,
    pub session_start: chrono::DateTime<chrono::Utc>,
}

/// Marker component for forge-related entities
#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct ForgeRelatedMarker {
    pub forge_type: forge::ForgePath,
    pub involvement_level: f32,
}

/// Common component for entity names
#[derive(Component, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct Name(pub String);

impl Name {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }
}

/// Common component for entity descriptions
#[derive(Component, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct Description(pub String);

/// Component for tracking entity relationships
#[derive(Component, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct EntityRelationships {
    pub relationships: std::collections::HashMap<Entity, RelationshipType>,
    pub relationship_history: Vec<RelationshipEvent>,
}

#[derive(Reflect, Debug, Clone, PartialEq)]
pub enum RelationshipType {
    Friend,
    Enemy,
    Neutral,
    Family,
    Romantic,
    Mentor,
    Student,
    Business,
}

#[derive(Reflect, Debug, Clone)]
pub struct RelationshipEvent {
    pub event_type: String,
    pub other_entity: Entity,
    pub relationship_change: f32,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Component for tracking entity health/status
#[derive(Component, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct Health {
    pub current: f32,
    pub maximum: f32,
    pub regeneration_rate: f32,
    pub last_damage_time: Option<chrono::DateTime<chrono::Utc>>,
}

impl Health {
    pub fn new(max_health: f32) -> Self {
        Self {
            current: max_health,
            maximum: max_health,
            regeneration_rate: 0.0,
            last_damage_time: None,
        }
    }
    
    pub fn is_alive(&self) -> bool {
        self.current > 0.0
    }
    
    pub fn health_percentage(&self) -> f32 {
        self.current / self.maximum
    }
}

/// Component for entity movement speeds and modifiers
#[derive(Component, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct MovementStats {
    pub base_speed: f32,
    pub current_speed: f32,
    pub speed_modifiers: Vec<SpeedModifier>,
    pub movement_type: MovementType,
}

#[derive(Reflect, Debug, Clone)]
pub struct SpeedModifier {
    pub modifier_name: String,
    pub multiplier: f32,
    pub duration: Option<f32>,
    pub source: String,
}

#[derive(Reflect, Debug, Clone, PartialEq)]
pub enum MovementType {
    Walking,
    Running,
    Mounted,
    Flying,
    Swimming,
    Teleporting,
}

/// Component for tracking entity experience and levels
#[derive(Component, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct Experience {
    pub current_level: u32,
    pub current_experience: u64,
    pub experience_to_next_level: u64,
    pub skill_points: u32,
    pub skills: std::collections::HashMap<String, u32>,
}

/// Component for entity factions and allegiances
#[derive(Component, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct Faction {
    pub primary_faction: String,
    pub faction_standings: std::collections::HashMap<String, f32>,
    pub reputation_modifiers: Vec<ReputationModifier>,
}

#[derive(Reflect, Debug, Clone)]
pub struct ReputationModifier {
    pub faction_name: String,
    pub modifier: f32,
    pub reason: String,
    pub duration: Option<f32>,
}

/// Component for entities that can be interacted with
#[derive(Component, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct Interactable {
    pub interaction_types: Vec<InteractionType>,
    pub interaction_range: f32,
    pub requires_items: Vec<String>,
    pub interaction_cooldown: Option<f32>,
    pub last_interaction: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Reflect, Debug, Clone, PartialEq)]
pub enum InteractionType {
    Talk,
    Trade,
    Examine,
    Use,
    Attack,
    Heal,
    PickUp,
    Craft,
    Forge,
    Therapy,
}

/// Component for tracking entity visibility and stealth
#[derive(Component, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct Stealth {
    pub stealth_level: f32,
    pub detection_radius: f32,
    pub is_hidden: bool,
    pub stealth_modifiers: Vec<StealthModifier>,
}

#[derive(Reflect, Debug, Clone)]
pub struct StealthModifier {
    pub modifier_name: String,
    pub stealth_bonus: f32,
    pub detection_bonus: f32,
    pub duration: Option<f32>,
}

// System-specific query helpers

/// Query for all player entities
pub type PlayerQuery<'w, 's> = Query<'w, 's, (Entity, &'static Player, &'static PlayerStats)>;

/// Query for all companion entities
pub type CompanionQuery<'w, 's> = Query<'w, 's, (Entity, &'static Companion, &'static CompanionState)>;

/// Query for all hex tiles
pub type HexTileQuery<'w, 's> = Query<'w, 's, (Entity, &'static HexTile, &'static Biome)>;

/// Query for all corrupted entities
pub type CorruptedQuery<'w, 's> = Query<'w, 's, (Entity, &'static CorruptedMarker, &'static hex_tiles::Corruption)>;

/// Query for all entities in therapy
pub type TherapyQuery<'w, 's> = Query<'w, 's, (Entity, &'static InTherapyMarker, &'static psychology::CompanionTherapy)>;

/// Query for all forge-related entities
pub type ForgeQuery<'w, 's> = Query<'w, 's, (Entity, &'static ForgeRelatedMarker, &'static SentimentalItem)>;
