//! Environmental Decay System - World Responds to Player Corruption
//! 
//! This module implements the sophisticated environmental decay system discovered in vision integration:
//! - World literally responds to player corruption and dread levels
//! - NPCs lock doors when you approach at high dread levels
//! - Economic collapse (gold becomes worthless, survival items precious)
//! - Visual corruption (colors desaturate, shadows lengthen)
//! - Reality distortion and sanity-based false audio/visuals

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

pub mod visual_corruption;
pub mod economic_decay;
pub mod social_decay;
pub mod reality_distortion;

pub use visual_corruption::*;
pub use economic_decay::*;
pub use social_decay::*;
pub use reality_distortion::*;

// ============================================================================
// ENVIRONMENTAL DECAY PLUGIN
// ============================================================================

pub struct EnvironmentalDecayPlugin;

impl Plugin for EnvironmentalDecayPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<GlobalDecayState>()
            .init_resource::<VisualCorruptionState>()
            .init_resource::<EconomicCollapseState>()
            .init_resource::<SocialDecayState>()
            .init_resource::<RealityDistortionState>()
            
            // Events
            .add_event::<CorruptionSpreadEvent>()
            .add_event::<EconomicCollapseEvent>()
            .add_event::<SocialIsolationEvent>()
            .add_event::<RealityDistortionEvent>()
            .add_event::<EnvironmentalDecayEvent>()
            .add_event::<PurificationEvent>()
            
            // Systems
            .add_systems(Startup, setup_decay_system)
            .add_systems(Update, (
                // Core decay systems
                corruption_spread_system,
                dread_level_response_system,
                environmental_decay_progression_system,
                
                // Visual corruption systems
                color_desaturation_system,
                shadow_lengthening_system,
                texture_degradation_system,
                lighting_corruption_system,
                
                // Economic decay systems
                economic_collapse_system,
                currency_devaluation_system,
                survival_item_inflation_system,
                trade_network_breakdown_system,
                
                // Social decay systems
                npc_fear_response_system,
                door_locking_system,
                social_isolation_system,
                evacuation_system,
                
                // Reality distortion systems
                false_audio_system,
                false_visual_system,
                sanity_based_hallucination_system,
                reality_stability_system,
                
                // Recovery and purification systems
                natural_resistance_system,
                purification_opportunity_system,
                corruption_containment_system,
            ).chain());
    }
}

// ============================================================================
// CORE DECAY COMPONENTS
// ============================================================================

/// Component tracking environmental corruption of hex tiles
#[derive(Component, Reflect, Clone, Debug, Serialize, Deserialize)]
#[reflect(Component)]
pub struct EnvironmentalCorruption {
    // Corruption metrics
    pub corruption_level: f32,          // 0.0-1.0 overall corruption level
    pub corruption_type: CorruptionType,
    pub corruption_source: String,      // What caused corruption
    pub corruption_age: f32,            // How long corruption has existed
    
    // Visual corruption effects
    pub color_desaturation: f32,        // 0.0-1.0 how much color is drained
    pub shadow_lengthening: f32,        // Multiplier for shadow length
    pub texture_degradation: f32,       // 0.0-1.0 texture decay level
    pub lighting_dimming: f32,          // 0.0-1.0 light reduction
    
    // Corruption spread mechanics
    pub spreads_to_adjacent: bool,      // Does corruption spread?
    pub spread_rate: f32,               // Rate of spread per second
    pub spread_resistance: f32,         // Resistance to spread
    pub containment_status: ContainmentStatus,
    
    // Environmental effects
    pub atmospheric_effects: Vec<String>, // Atmospheric changes
    pub weather_corruption: Option<WeatherCorruption>,
    pub ecological_damage: f32,         // 0.0-1.0 ecosystem damage
    pub natural_cycle_disruption: f32,  // 0.0-1.0 disruption to natural cycles
    
    // Recovery potential
    pub natural_resistance: f32,        // 0.0-1.0 natural resistance to corruption
    pub recovery_potential: f32,        // 0.0-1.0 potential for recovery
    pub purification_requirements: Vec<String>, // What's needed to purify
    pub corruption_permanence: f32,     // 0.0-1.0 how permanent corruption is
}

/// Component for NPCs that respond to player dread level
#[derive(Component, Reflect, Clone, Debug, Serialize, Deserialize)]
#[reflect(Component)]
pub struct DreadResponsiveNPC {
    // Fear response thresholds
    pub fear_threshold: f32,            // Dread level where NPC becomes afraid
    pub flee_threshold: f32,            // Dread level where NPC flees
    pub hostility_threshold: f32,       // Dread level where NPC becomes hostile
    
    // Current response state
    pub current_fear_level: f32,        // 0.0-1.0 current fear of player
    pub response_behavior: NPCBehaviorState,
    pub last_player_proximity: f32,     // Distance when player was last detected
    pub fear_memory_duration: f32,      // How long fear persists
    
    // Behavioral responses
    pub locks_doors: bool,              // Does NPC lock doors when player approaches?
    pub refuses_trade: bool,            // Does NPC refuse to trade?
    pub flees_on_sight: bool,           // Does NPC flee when seeing player?
    pub warns_others: bool,             // Does NPC warn other NPCs?
    
    // Social influence
    pub social_influence: f32,          // 0.0-1.0 influence on other NPCs
    pub spreads_fear: bool,             // Does this NPC spread fear to others?
    pub evacuation_leader: bool,        // Does this NPC organize evacuations?
}

/// Component tracking economic relationships affected by decay
#[derive(Component, Reflect, Clone, Debug, Serialize, Deserialize)]
#[reflect(Component)]
pub struct EconomicEntity {
    // Economic role
    pub entity_type: EconomicEntityType,
    pub base_wealth: f32,               // Original wealth/value
    pub current_wealth: f32,            // Current wealth after decay
    
    // Trade behavior
    pub accepts_gold: bool,             // Still accepts gold currency?
    pub gold_value_modifier: f32,       // Multiplier for gold value (can be < 1.0)
    pub survival_item_preference: f32,  // 0.0-1.0 preference for survival items
    pub barter_only: bool,              // Only accepts barter, no currency?
    
    // Decay response
    pub corruption_sensitivity: f32,    // 0.0-1.0 sensitivity to environmental corruption
    pub economic_panic_level: f32,      // 0.0-1.0 current panic about economy
    pub trade_network_connectivity: f32, // 0.0-1.0 connection to broader trade network
    
    // Survival economics
    pub hoards_resources: bool,         // Does entity hoard survival resources?
    pub price_inflation_rate: f32,      // Rate of price increases
    pub desperate_trade_threshold: f32, // Desperation level for bad trades
}

/// Component for reality distortion effects
#[derive(Component, Reflect, Clone, Debug, Serialize, Deserialize)]
#[reflect(Component)]
pub struct RealityDistortion {
    // Distortion metrics
    pub distortion_level: f32,          // 0.0-1.0 level of reality distortion
    pub distortion_type: DistortionType,
    pub stability: f32,                 // 0.0-1.0 how stable reality is
    pub distortion_frequency: f32,      // How often distortions occur
    
    // Audio distortions
    pub false_audio_frequency: f32,     // Rate of false audio events
    pub audio_reliability: f32,         // 0.0-1.0 reliability of audio cues
    pub phantom_sounds: Vec<String>,    // Types of phantom sounds
    
    // Visual distortions
    pub false_visual_frequency: f32,    // Rate of false visual events
    pub visual_reliability: f32,        // 0.0-1.0 reliability of visual cues
    pub phantom_visuals: Vec<String>,   // Types of phantom visuals
    
    // Sanity integration
    pub sanity_threshold: f32,          // Sanity level where distortions begin
    pub sanity_responsiveness: f32,     // How much sanity affects distortions
    pub distortion_sanity_drain: f32,   // Sanity cost of experiencing distortions
}

// ============================================================================
// DECAY ENUMS AND TYPES
// ============================================================================

#[derive(Reflect, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum CorruptionType {
    PlayerCorruption,   // Corruption emanating from player
    DragonCorruption,   // Corruption from dragon presence
    VoidCorruption,     // Corruption from void/labyrinth
    MoralCorruption,    // Corruption from immoral acts
    TraumaCorruption,   // Corruption from accumulated trauma
    EconomicCorruption, // Corruption from economic collapse
}

#[derive(Reflect, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum ContainmentStatus {
    Spreading,      // Corruption actively spreading
    Contained,      // Corruption contained but stable
    Accelerating,   // Corruption spreading faster
    Diminishing,    // Corruption slowly reducing
    Purified,       // Corruption successfully purified
}

#[derive(Reflect, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum NPCBehaviorState {
    Normal,         // Normal behavior
    Cautious,       // Cautious around player
    Fearful,        // Afraid of player
    Hostile,        // Hostile to player
    Fled,           // Has fled the area
    Evacuated,      // Evacuated by others
}

#[derive(Reflect, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum EconomicEntityType {
    Merchant,       // Buys and sells goods
    Vendor,         // Sells specific items
    Bank,           // Handles currency exchange
    Crafter,        // Creates items from materials
    Farmer,         // Produces food and basic goods
    Noble,          // Wealthy patron with resources
}

#[derive(Reflect, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum DistortionType {
    SanityBased,    // Distortions based on sanity level
    CorruptionBased, // Distortions based on environmental corruption
    TraumaBased,    // Distortions based on companion trauma
    DreadBased,     // Distortions based on dread level
    ProximityBased, // Distortions based on dragon proximity
}

#[derive(Reflect, Clone, Debug, Serialize, Deserialize)]
pub struct WeatherCorruption {
    pub corrupted_weather_type: String, // Type of corrupted weather
    pub intensity: f32,                 // 0.0-1.0 intensity of corruption
    pub affects_visibility: bool,       // Does weather affect visibility?
    pub affects_movement: bool,         // Does weather affect movement?
    pub psychological_impact: f32,      // 0.0-1.0 psychological effect
}

// ============================================================================
// DECAY RESOURCES
// ============================================================================

/// Global environmental decay state
#[derive(Resource, Reflect, Clone, Debug, Serialize, Deserialize)]
#[reflect(Resource)]
pub struct GlobalDecayState {
    // Overall corruption metrics
    pub global_corruption_level: f32,   // 0.0-1.0 average world corruption
    pub corruption_spread_rate: f32,    // Current rate of corruption spread
    pub total_corrupted_tiles: u32,     // Number of corrupted hex tiles
    pub corruption_epicenters: Vec<Entity>, // Sources of major corruption
    
    // Player corruption influence
    pub player_corruption_radius: f32,  // Radius of player's corruption influence
    pub player_corruption_intensity: f32, // Intensity of player's corruption
    pub corruption_follows_player: bool, // Does corruption follow player movement?
    
    // Decay progression
    pub decay_stage: DecayStage,        // Current stage of world decay
    pub decay_acceleration: f32,        // Rate of decay acceleration
    pub irreversible_damage: f32,       // 0.0-1.0 damage that cannot be undone
    
    // Resistance and recovery
    pub natural_world_resistance: f32,  // 0.0-1.0 world's resistance to corruption
    pub active_purification_efforts: u32, // Number of active purification attempts
    pub recovery_zones: Vec<Entity>,    // Areas showing signs of recovery
    
    // System integration
    pub affects_companion_trauma: bool, // Does decay increase companion trauma?
    pub affects_philosophical_choices: bool, // Does decay affect available choices?
    pub affects_forge_readiness: bool,  // Does decay affect forge accessibility?
}

impl Default for GlobalDecayState {
    fn default() -> Self {
        Self {
            global_corruption_level: 0.0,
            corruption_spread_rate: 0.01, // Slow initial spread
            total_corrupted_tiles: 0,
            corruption_epicenters: Vec::new(),
            player_corruption_radius: 2.0, // Affects 2 hex tiles around player
            player_corruption_intensity: 0.0,
            corruption_follows_player: false, // Initially static
            decay_stage: DecayStage::Pristine,
            decay_acceleration: 1.0,
            irreversible_damage: 0.0,
            natural_world_resistance: 0.7, // World has natural resistance
            active_purification_efforts: 0,
            recovery_zones: Vec::new(),
            affects_companion_trauma: true,
            affects_philosophical_choices: true,
            affects_forge_readiness: true,
        }
    }
}

/// Visual corruption state tracking
#[derive(Resource, Reflect, Clone, Debug, Serialize, Deserialize)]
#[reflect(Resource)]
pub struct VisualCorruptionState {
    // Color and lighting corruption
    pub global_color_saturation: f32,   // 0.0-1.0 global color saturation
    pub shadow_length_multiplier: f32,  // Multiplier for shadow lengths
    pub ambient_light_reduction: f32,   // 0.0-1.0 reduction in ambient light
    pub color_palette_shift: ColorPaletteShift, // How colors are shifting
    
    // Texture and material corruption
    pub texture_degradation_level: f32, // 0.0-1.0 global texture degradation
    pub material_corruption_spread: f32, // Rate of material corruption spread
    pub surface_decay_intensity: f32,   // Intensity of surface decay effects
    
    // Atmospheric corruption
    pub fog_corruption_level: f32,      // 0.0-1.0 corrupted fog/atmosphere
    pub particle_corruption_density: f32, // Density of corruption particles
    pub air_quality_degradation: f32,   // 0.0-1.0 air quality reduction
    
    // Dynamic corruption effects
    pub corruption_animation_speed: f32, // Speed of corruption animations
    pub corruption_pulse_frequency: f32, // Frequency of corruption pulses
    pub corruption_growth_visibility: bool, // Can players see corruption growing?
}

#[derive(Reflect, Clone, Debug, Serialize, Deserialize)]
pub struct ColorPaletteShift {
    pub hue_shift: f32,                 // -180.0 to 180.0 hue shift
    pub saturation_reduction: f32,      // 0.0-1.0 saturation reduction
    pub brightness_reduction: f32,      // 0.0-1.0 brightness reduction
    pub color_temperature_shift: f32,   // Shift toward cold colors
}

/// Economic collapse state tracking
#[derive(Resource, Reflect, Clone, Debug, Serialize, Deserialize)]
#[reflect(Resource)]
pub struct EconomicCollapseState {
    // Currency devaluation
    pub gold_value_multiplier: f32,     // Multiplier for gold value (starts at 1.0)
    pub currency_stability: f32,        // 0.0-1.0 stability of currency system
    pub inflation_rate: f32,            // Rate of price inflation
    
    // Survival economy emergence
    pub survival_item_value_multiplier: f32, // Multiplier for survival item values
    pub barter_system_prevalence: f32,  // 0.0-1.0 how much economy runs on barter
    pub resource_scarcity_level: f32,   // 0.0-1.0 scarcity of basic resources
    
    // Trade network collapse
    pub trade_route_functionality: f32, // 0.0-1.0 how well trade routes work
    pub merchant_availability: f32,     // 0.0-1.0 how many merchants still operate
    pub market_confidence: f32,         // 0.0-1.0 confidence in economic system
    
    // Economic panic indicators
    pub economic_panic_level: f32,      // 0.0-1.0 level of economic panic
    pub hoarding_behavior_prevalence: f32, // 0.0-1.0 how much NPCs hoard
    pub desperate_trading_frequency: f32, // Frequency of desperate trades
}

impl Default for EconomicCollapseState {
    fn default() -> Self {
        Self {
            gold_value_multiplier: 1.0, // Start with normal gold value
            currency_stability: 0.9,
            inflation_rate: 0.02, // 2% baseline inflation
            survival_item_value_multiplier: 1.0,
            barter_system_prevalence: 0.1, // Some barter always exists
            resource_scarcity_level: 0.1,
            trade_route_functionality: 0.9,
            merchant_availability: 0.8,
            market_confidence: 0.7,
            economic_panic_level: 0.1,
            hoarding_behavior_prevalence: 0.2,
            desperate_trading_frequency: 0.1,
        }
    }
}

/// Social decay state tracking
#[derive(Resource, Reflect, Clone, Debug, Serialize, Deserialize)]
#[reflect(Resource)]
pub struct SocialDecayState {
    // Social isolation metrics
    pub player_social_isolation_level: f32, // 0.0-1.0 how isolated player is
    pub npc_fear_of_player: f32,        // 0.0-1.0 average NPC fear
    pub social_rejection_level: f32,    // 0.0-1.0 level of social rejection
    
    // Community breakdown
    pub community_cohesion: f32,        // 0.0-1.0 community cohesion level
    pub social_trust_level: f32,        // 0.0-1.0 trust between NPCs
    pub collective_morale: f32,         // 0.0-1.0 collective NPC morale
    pub social_order_stability: f32,    // 0.0-1.0 stability of social order
    
    // Evacuation and abandonment
    pub evacuation_zones: Vec<Entity>,  // Areas being evacuated
    pub abandoned_settlements: Vec<Entity>, // Completely abandoned areas
    pub refugee_populations: u32,       // Number of displaced NPCs
    pub evacuation_coordination: f32,   // 0.0-1.0 coordination of evacuations
    
    // Fear propagation
    pub fear_propagation_rate: f32,     // Rate at which fear spreads
    pub fear_amplification_factor: f32, // How much fear amplifies
    pub collective_fear_level: f32,     // 0.0-1.0 overall fear in population
}

impl Default for SocialDecayState {
    fn default() -> Self {
        Self {
            player_social_isolation_level: 0.0,
            npc_fear_of_player: 0.0,
            social_rejection_level: 0.0,
            community_cohesion: 0.8,
            social_trust_level: 0.7,
            collective_morale: 0.6,
            social_order_stability: 0.8,
            evacuation_zones: Vec::new(),
            abandoned_settlements: Vec::new(),
            refugee_populations: 0,
            evacuation_coordination: 0.3,
            fear_propagation_rate: 0.05,
            fear_amplification_factor: 1.2,
            collective_fear_level: 0.1,
        }
    }
}

/// Reality distortion state tracking
#[derive(Resource, Reflect, Clone, Debug, Serialize, Deserialize)]
#[reflect(Resource)]
pub struct RealityDistortionState {
    // Reality stability
    pub reality_stability: f32,         // 0.0-1.0 stability of reality
    pub distortion_intensity: f32,      // 0.0-1.0 intensity of distortions
    pub distortion_frequency: f32,      // Frequency of distortion events
    
    // Sanity-based distortions
    pub sanity_distortion_threshold: f32, // Sanity level where distortions begin
    pub sanity_distortion_intensity: f32, // How much sanity affects distortions
    pub false_perception_rate: f32,     // Rate of false perceptions
    
    // Audio hallucinations
    pub false_audio_types: Vec<String>, // Types of false audio
    pub audio_distortion_probability: f32, // Probability of audio distortions
    pub phantom_dragon_audio: bool,     // False dragon sounds?
    pub phantom_companion_voices: bool, // False companion voices?
    
    // Visual hallucinations
    pub false_visual_types: Vec<String>, // Types of false visuals
    pub visual_distortion_probability: f32, // Probability of visual distortions
    pub phantom_movement: bool,         // False movement in periphery?
    pub phantom_entities: bool,         // False entities appearing?
    
    // Reality coherence
    pub memory_reliability: f32,        // 0.0-1.0 reliability of memories
    pub temporal_stability: f32,        // 0.0-1.0 stability of time perception
    pub spatial_stability: f32,         // 0.0-1.0 stability of space perception
}

impl Default for RealityDistortionState {
    fn default() -> Self {
        Self {
            reality_stability: 1.0, // Start with stable reality
            distortion_intensity: 0.0,
            distortion_frequency: 0.0,
            sanity_distortion_threshold: 0.5,
            sanity_distortion_intensity: 0.0,
            false_perception_rate: 0.0,
            false_audio_types: vec![
                "Distant roaring".to_string(),
                "Companion calling for help".to_string(),
                "Footsteps behind you".to_string(),
                "Whispers in unknown language".to_string(),
            ],
            audio_distortion_probability: 0.0,
            phantom_dragon_audio: false,
            phantom_companion_voices: false,
            false_visual_types: vec![
                "Movement in peripheral vision".to_string(),
                "Shadows moving independently".to_string(),
                "Familiar faces in darkness".to_string(),
                "Corruption spreading faster than real".to_string(),
            ],
            visual_distortion_probability: 0.0,
            phantom_movement: false,
            phantom_entities: false,
            memory_reliability: 1.0,
            temporal_stability: 1.0,
            spatial_stability: 1.0,
        }
    }
}

#[derive(Reflect, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum DecayStage {
    Pristine,       // No decay (dread 0)
    EarlyDecay,     // Initial signs of decay (dread 1)
    ModerateDecay,  // Obvious decay (dread 2)
    SevereDecay,    // Severe environmental damage (dread 3)
    CatastrophicDecay, // Near-total collapse (dread 4)
}

// ============================================================================
// DECAY EVENTS
// ============================================================================

#[derive(Event, Reflect, Clone, Debug)]
#[reflect(Default)]
pub struct CorruptionSpreadEvent {
    pub source_entity: Entity,          // Source of corruption
    pub affected_entities: Vec<Entity>, // Entities affected by spread
    pub corruption_type: CorruptionType,
    pub spread_intensity: f32,          // 0.0-1.0 intensity of spread
    pub spread_radius: f32,             // Radius of corruption spread
    pub environmental_factors: Vec<String>, // Factors affecting spread
}

impl Default for CorruptionSpreadEvent {
    fn default() -> Self {
        Self {
            source_entity: Entity::PLACEHOLDER,
            affected_entities: Vec::new(),
            corruption_type: CorruptionType::PlayerCorruption,
            spread_intensity: 0.0,
            spread_radius: 0.0,
            environmental_factors: Vec::new(),
        }
    }
}

#[derive(Event, Reflect, Clone, Debug)]
#[reflect(Default)]
pub struct EconomicCollapseEvent {
    pub trigger_source: String,         // What triggered economic collapse
    pub affected_economic_entities: Vec<Entity>, // Economic entities affected
    pub collapse_severity: f32,         // 0.0-1.0 severity of collapse
    pub currency_devaluation: f32,      // Amount of currency devaluation
    pub survival_item_inflation: f32,   // Amount of survival item inflation
    pub trade_network_damage: f32,      // Damage to trade networks
}

impl Default for EconomicCollapseEvent {
    fn default() -> Self {
        Self {
            trigger_source: String::new(),
            affected_economic_entities: Vec::new(),
            collapse_severity: 0.0,
            currency_devaluation: 0.0,
            survival_item_inflation: 0.0,
            trade_network_damage: 0.0,
        }
    }
}

#[derive(Event, Reflect, Clone, Debug)]
#[reflect(Default)]
pub struct SocialIsolationEvent {
    pub affected_npcs: Vec<Entity>,     // NPCs changing behavior toward player
    pub isolation_trigger: String,      // What triggered social isolation
    pub fear_level_increase: f32,       // Increase in NPC fear
    pub behavioral_changes: Vec<(Entity, NPCBehaviorState)>, // Behavior changes
    pub doors_locked: Vec<Entity>,      // Doors that were locked
    pub trade_refusals: Vec<Entity>,    // NPCs refusing trade
}

impl Default for SocialIsolationEvent {
    fn default() -> Self {
        Self {
            affected_npcs: Vec::new(),
            isolation_trigger: String::new(),
            fear_level_increase: 0.0,
            behavioral_changes: Vec::new(),
            doors_locked: Vec::new(),
            trade_refusals: Vec::new(),
        }
    }
}

#[derive(Event, Reflect, Clone, Debug)]
#[reflect(Default)]
pub struct RealityDistortionEvent {
    pub distortion_type: DistortionType,
    pub distortion_description: String, // Description of distortion
    pub intensity: f32,                 // 0.0-1.0 intensity of distortion
    pub duration: f32,                  // Duration of distortion effect
    pub affects_audio: bool,            // Does distortion affect audio?
    pub affects_visual: bool,           // Does distortion affect visuals?
    pub sanity_cost: f32,               // Sanity cost of experiencing distortion
    pub false_information: Vec<String>, // False information provided by distortion
}

impl Default for RealityDistortionEvent {
    fn default() -> Self {
        Self {
            distortion_type: DistortionType::SanityBased,
            distortion_description: String::new(),
            intensity: 0.0,
            duration: 0.0,
            affects_audio: false,
            affects_visual: false,
            sanity_cost: 0.0,
            false_information: Vec::new(),
        }
    }
}

#[derive(Event, Reflect, Clone, Debug)]
#[reflect(Default)]
pub struct EnvironmentalDecayEvent {
    pub decay_type: EnvironmentalDecayType,
    pub affected_area: Entity,          // Area affected by decay
    pub decay_intensity: f32,           // 0.0-1.0 intensity of decay
    pub decay_speed: f32,               // Speed of decay progression
    pub ecological_impact: f32,         // 0.0-1.0 impact on ecology
    pub recovery_difficulty: f32,       // 0.0-1.0 difficulty of recovery
    pub purification_requirements: Vec<String>, // Requirements for purification
}

#[derive(Reflect, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum EnvironmentalDecayType {
    EcosystemCollapse,  // Ecosystem breakdown
    WeatherCorruption,  // Weather pattern corruption
    WaterContamination, // Water source contamination
    SoilDegradation,    // Soil corruption and degradation
    AirPollution,       // Air quality degradation
    BiodiversityLoss,   // Loss of plant/animal life
}

impl Default for EnvironmentalDecayEvent {
    fn default() -> Self {
        Self {
            decay_type: EnvironmentalDecayType::EcosystemCollapse,
            affected_area: Entity::PLACEHOLDER,
            decay_intensity: 0.0,
            decay_speed: 0.0,
            ecological_impact: 0.0,
            recovery_difficulty: 0.0,
            purification_requirements: Vec::new(),
        }
    }
}

#[derive(Event, Reflect, Clone, Debug)]
#[reflect(Default)]
pub struct PurificationEvent {
    pub purification_source: Entity,    // Source of purification
    pub purified_area: Entity,          // Area being purified
    pub purification_method: String,    // Method of purification
    pub purification_strength: f32,     // 0.0-1.0 strength of purification
    pub corruption_removed: f32,        // Amount of corruption removed
    pub recovery_potential_restored: f32, // Recovery potential restored
    pub environmental_healing: f32,     // Environmental healing achieved
}

impl Default for PurificationEvent {
    fn default() -> Self {
        Self {
            purification_source: Entity::PLACEHOLDER,
            purified_area: Entity::PLACEHOLDER,
            purification_method: String::new(),
            purification_strength: 0.0,
            corruption_removed: 0.0,
            recovery_potential_restored: 0.0,
            environmental_healing: 0.0,
        }
    }
}

// ============================================================================
// DECAY SYSTEM SETUP
// ============================================================================

fn setup_decay_system(mut commands: Commands) {
    info!("Initializing Environmental Decay System - World responds to player corruption");
    
    // Initialize visual corruption parameters
    commands.insert_resource(VisualCorruptionState {
        global_color_saturation: 1.0, // Start with full color
        shadow_length_multiplier: 1.0,
        ambient_light_reduction: 0.0,
        color_palette_shift: ColorPaletteShift {
            hue_shift: 0.0,
            saturation_reduction: 0.0,
            brightness_reduction: 0.0,
            color_temperature_shift: 0.0,
        },
        texture_degradation_level: 0.0,
        material_corruption_spread: 0.0,
        surface_decay_intensity: 0.0,
        fog_corruption_level: 0.0,
        particle_corruption_density: 0.0,
        air_quality_degradation: 0.0,
        corruption_animation_speed: 1.0,
        corruption_pulse_frequency: 0.0,
        corruption_growth_visibility: false,
    });
    
    info!("Environmental Decay System initialized - world awaits corruption");
}

// ============================================================================
// CORE DECAY SYSTEMS
// ============================================================================

/// Corruption spread system based on dread level and player actions
fn corruption_spread_system(
    mut corruption_events: EventWriter<CorruptionSpreadEvent>,
    mut global_decay: ResMut<GlobalDecayState>,
    corrupted_tiles: Query<(Entity, &mut EnvironmentalCorruption)>,
    // TODO: Add player position and dread level queries
) {
    // Update global corruption metrics
    let total_corruption: f32 = corrupted_tiles.iter()
        .map(|(_, corruption)| corruption.corruption_level)
        .sum();
    let tile_count = corrupted_tiles.iter().count() as f32;
    
    if tile_count > 0.0 {
        global_decay.global_corruption_level = total_corruption / tile_count;
        global_decay.total_corrupted_tiles = corrupted_tiles.iter()
            .filter(|(_, corruption)| corruption.corruption_level > 0.1)
            .count() as u32;
    }
    
    // Process corruption spread
    for (entity, mut corruption) in corrupted_tiles.iter_mut() {
        if corruption.spreads_to_adjacent && corruption.containment_status == ContainmentStatus::Spreading {
            // TODO: Find adjacent hex tiles and spread corruption
            let spread_amount = corruption.spread_rate * 0.016; // Assume ~60 FPS
            
            if spread_amount > 0.01 {
                corruption_events.send(CorruptionSpreadEvent {
                    source_entity: entity,
                    affected_entities: vec![], // TODO: Get adjacent entities
                    corruption_type: corruption.corruption_type.clone(),
                    spread_intensity: spread_amount,
                    spread_radius: 1.0, // Adjacent tiles
                    environmental_factors: corruption.atmospheric_effects.clone(),
                });
                
                info!("Corruption spreading from tile {} (intensity: {:.3})", 
                      entity.index(), spread_amount);
            }
        }
    }
}

/// Dread level response system - world responds to player dread
fn dread_level_response_system(
    mut global_decay: ResMut<GlobalDecayState>,
    mut visual_corruption: ResMut<VisualCorruptionState>,
    mut economic_collapse: ResMut<EconomicCollapseState>,
    mut social_decay: ResMut<SocialDecayState>,
    mut reality_distortion: ResMut<RealityDistortionState>,
    // TODO: Add dread level query
) {
    let current_dread_level = 0; // TODO: Get actual dread level from player
    
    // Update decay stage based on dread level
    global_decay.decay_stage = match current_dread_level {
        0 => DecayStage::Pristine,
        1 => DecayStage::EarlyDecay,
        2 => DecayStage::ModerateDecay,
        3 => DecayStage::SevereDecay,
        4.. => DecayStage::CatastrophicDecay,
    };
    
    // Adjust corruption intensity based on dread
    global_decay.player_corruption_intensity = (current_dread_level as f32) / 4.0;
    global_decay.corruption_follows_player = current_dread_level >= 2;
    
    // Update visual corruption based on dread
    visual_corruption.global_color_saturation = 1.0 - (current_dread_level as f32 * 0.2);
    visual_corruption.shadow_length_multiplier = 1.0 + (current_dread_level as f32 * 0.5);
    visual_corruption.ambient_light_reduction = (current_dread_level as f32) * 0.15;
    
    // Update economic collapse based on dread
    economic_collapse.gold_value_multiplier = 1.0 - (current_dread_level as f32 * 0.2);
    economic_collapse.survival_item_value_multiplier = 1.0 + (current_dread_level as f32 * 0.3);
    economic_collapse.economic_panic_level = (current_dread_level as f32) * 0.25;
    
    // Update social decay based on dread
    social_decay.npc_fear_of_player = (current_dread_level as f32) * 0.25;
    social_decay.player_social_isolation_level = (current_dread_level as f32) * 0.2;
    social_decay.community_cohesion = 0.8 - (current_dread_level as f32 * 0.15);
    
    // Update reality distortion based on dread
    reality_distortion.distortion_intensity = (current_dread_level as f32) * 0.2;
    reality_distortion.distortion_frequency = (current_dread_level as f32) * 0.1;
    reality_distortion.false_perception_rate = (current_dread_level as f32) * 0.15;
    
    if current_dread_level >= 3 {
        reality_distortion.phantom_dragon_audio = true;
        reality_distortion.phantom_companion_voices = true;
        reality_distortion.phantom_movement = true;
    }
}

/// Environmental decay progression system
fn environmental_decay_progression_system(
    mut environmental_decay_events: EventWriter<EnvironmentalDecayEvent>,
    mut corrupted_areas: Query<(Entity, &mut EnvironmentalCorruption)>,
    global_decay: Res<GlobalDecayState>,
    time: Res<Time>,
) {
    for (entity, mut corruption) in corrupted_areas.iter_mut() {
        // Age the corruption
        corruption.corruption_age += time.delta_seconds();
        
        // Increase corruption level over time if spreading
        if corruption.containment_status == ContainmentStatus::Spreading {
            let decay_rate = corruption.spread_rate * global_decay.decay_acceleration;
            corruption.corruption_level = (corruption.corruption_level + decay_rate * time.delta_seconds()).min(1.0);
            
            // Increase environmental effects as corruption grows
            corruption.ecological_damage = corruption.corruption_level * 0.8;
            corruption.natural_cycle_disruption = corruption.corruption_level * 0.6;
            
            // Check for major decay milestones
            if corruption.corruption_level > 0.5 && corruption.ecological_damage > 0.4 {
                environmental_decay_events.send(EnvironmentalDecayEvent {
                    decay_type: EnvironmentalDecayType::EcosystemCollapse,
                    affected_area: entity,
                    decay_intensity: corruption.corruption_level,
                    decay_speed: decay_rate,
                    ecological_impact: corruption.ecological_damage,
                    recovery_difficulty: 1.0 - corruption.recovery_potential,
                    purification_requirements: corruption.purification_requirements.clone(),
                });
            }
        }
    }
}

// ============================================================================
// VISUAL CORRUPTION SYSTEMS
// ============================================================================

/// Color desaturation system based on corruption level
fn color_desaturation_system(
    mut visual_corruption: ResMut<VisualCorruptionState>,
    global_decay: Res<GlobalDecayState>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    material_handles: Query<&Handle<StandardMaterial>>,
) {
    // Update global color saturation based on corruption
    let target_saturation = 1.0 - global_decay.global_corruption_level;
    visual_corruption.global_color_saturation = visual_corruption.global_color_saturation
        .lerp(target_saturation, 0.02); // Gradual change
    
    // Apply color desaturation to materials
    for material_handle in material_handles.iter() {
        if let Some(material) = materials.get_mut(material_handle) {
            // TODO: Apply color desaturation to material
            // This would require shader modifications or post-processing
            info!("Applying color desaturation: {:.2}", visual_corruption.global_color_saturation);
        }
    }
}

/// Shadow lengthening system
fn shadow_lengthening_system(
    mut visual_corruption: ResMut<VisualCorruptionState>,
    global_decay: Res<GlobalDecayState>,
    // TODO: Add lighting and shadow queries
) {
    // Shadows grow longer as corruption increases
    let target_shadow_multiplier = 1.0 + (global_decay.global_corruption_level * 2.0);
    visual_corruption.shadow_length_multiplier = visual_corruption.shadow_length_multiplier
        .lerp(target_shadow_multiplier, 0.01);
    
    // TODO: Apply shadow length changes to lighting system
    info!("Shadow length multiplier: {:.2}", visual_corruption.shadow_length_multiplier);
}

/// Texture degradation system
fn texture_degradation_system(
    mut visual_corruption: ResMut<VisualCorruptionState>,
    corrupted_areas: Query<&EnvironmentalCorruption>,
    // TODO: Add texture and material queries
) {
    // Calculate average texture degradation
    let average_degradation = corrupted_areas.iter()
        .map(|corruption| corruption.texture_degradation)
        .sum::<f32>() / corrupted_areas.iter().count().max(1) as f32;
    
    visual_corruption.texture_degradation_level = average_degradation;
    
    // TODO: Apply texture degradation effects to materials
    if visual_corruption.texture_degradation_level > 0.1 {
        info!("Texture degradation level: {:.2}", visual_corruption.texture_degradation_level);
    }
}

/// Lighting corruption system
fn lighting_corruption_system(
    mut visual_corruption: ResMut<VisualCorruptionState>,
    global_decay: Res<GlobalDecayState>,
    mut lights: Query<&mut PointLight>,
) {
    // Reduce ambient lighting as corruption increases
    let light_reduction = global_decay.global_corruption_level * 0.5;
    visual_corruption.ambient_light_reduction = light_reduction;
    
    // Apply lighting reduction to point lights
    for mut light in lights.iter_mut() {
        light.intensity *= 1.0 - light_reduction;
        
        // Shift light color toward cold/dead colors as corruption increases
        if global_decay.global_corruption_level > 0.3 {
            // TODO: Shift light color toward blue/purple spectrum
        }
    }
}

// ============================================================================
// ECONOMIC DECAY SYSTEMS
// ============================================================================

/// Economic collapse system
fn economic_collapse_system(
    mut economic_events: EventWriter<EconomicCollapseEvent>,
    mut economic_state: ResMut<EconomicCollapseState>,
    global_decay: Res<GlobalDecayState>,
    economic_entities: Query<(Entity, &EconomicEntity)>,
) {
    // Economic collapse accelerates with global corruption
    let collapse_pressure = global_decay.global_corruption_level * global_decay.decay_acceleration;
    
    if collapse_pressure > economic_state.economic_panic_level {
        economic_state.economic_panic_level = collapse_pressure;
        
        // Trigger economic collapse events when thresholds crossed
        if collapse_pressure > 0.5 && economic_state.market_confidence > 0.3 {
            economic_events.send(EconomicCollapseEvent {
                trigger_source: "Environmental corruption reaching critical mass".to_string(),
                affected_economic_entities: economic_entities.iter().map(|(e, _)| e).collect(),
                collapse_severity: collapse_pressure,
                currency_devaluation: 0.3,
                survival_item_inflation: 0.5,
                trade_network_damage: 0.4,
            });
            
            warn!("Economic collapse triggered by environmental corruption");
        }
    }
}

/// Currency devaluation system
fn currency_devaluation_system(
    mut economic_state: ResMut<EconomicCollapseState>,
    mut economic_entities: Query<&mut EconomicEntity>,
    global_decay: Res<GlobalDecayState>,
) {
    // Gold loses value as world becomes more dangerous and survival-focused
    let devaluation_rate = global_decay.global_corruption_level * 0.02;
    economic_state.gold_value_multiplier = (economic_state.gold_value_multiplier - devaluation_rate).max(0.1);
    
    // Update economic entities
    for mut entity in economic_entities.iter_mut() {
        entity.gold_value_modifier = economic_state.gold_value_multiplier;
        
        // Entities stop accepting gold when it becomes too worthless
        entity.accepts_gold = economic_state.gold_value_multiplier > 0.3;
        entity.barter_only = economic_state.gold_value_multiplier < 0.2;
    }
    
    if economic_state.gold_value_multiplier < 0.8 {
        info!("Gold devaluation: {:.2}x original value", economic_state.gold_value_multiplier);
    }
}

/// Survival item inflation system
fn survival_item_inflation_system(
    mut economic_state: ResMut<EconomicCollapseState>,
    global_decay: Res<GlobalDecayState>,
    // TODO: Add item value queries
) {
    // Survival items become more valuable as world becomes dangerous
    let scarcity_multiplier = 1.0 + (global_decay.global_corruption_level * 2.0);
    economic_state.survival_item_value_multiplier = scarcity_multiplier;
    economic_state.resource_scarcity_level = global_decay.global_corruption_level * 0.8;
    
    // TODO: Apply survival item inflation to actual item values
    info!("Survival item value multiplier: {:.2}x", scarcity_multiplier);
}

/// Trade network breakdown system
fn trade_network_breakdown_system(
    mut economic_state: ResMut<EconomicCollapseState>,
    social_decay: Res<SocialDecayState>,
    // TODO: Add trade route and merchant queries
) {
    // Trade networks fail as social order collapses
    economic_state.trade_route_functionality = social_decay.social_order_stability * 0.8;
    economic_state.merchant_availability = social_decay.community_cohesion * 0.9;
    
    // Market confidence drops as fear increases
    economic_state.market_confidence = 1.0 - social_decay.collective_fear_level;
    
    if economic_state.trade_route_functionality < 0.5 {
        warn!("Trade network breakdown: {:.2} functionality remaining", 
              economic_state.trade_route_functionality);
    }
}

// ============================================================================
// SOCIAL DECAY SYSTEMS
// ============================================================================

/// NPC fear response system - NPCs respond to player dread level
fn npc_fear_response_system(
    mut npcs: Query<(Entity, &mut DreadResponsiveNPC, &Transform)>,
    mut social_isolation_events: EventWriter<SocialIsolationEvent>,
    mut social_decay: ResMut<SocialDecayState>,
    // TODO: Add player position and dread level queries
) {
    let player_dread_level = 0.0; // TODO: Get actual player dread level
    let player_position = Vec3::ZERO; // TODO: Get actual player position
    
    let mut doors_locked = Vec::new();
    let mut behavioral_changes = Vec::new();
    
    for (entity, mut npc, transform) in npcs.iter_mut() {
        let distance_to_player = transform.translation.distance(player_position);
        
        // Calculate fear level based on dread and proximity
        let proximity_fear = if distance_to_player < 10.0 {
            (10.0 - distance_to_player) / 10.0 * player_dread_level
        } else {
            0.0
        };
        
        npc.current_fear_level = (npc.current_fear_level + proximity_fear).min(1.0);
        
        // Update NPC behavior based on fear level
        let new_behavior = if npc.current_fear_level >= npc.hostility_threshold {
            NPCBehaviorState::Hostile
        } else if npc.current_fear_level >= npc.flee_threshold {
            NPCBehaviorState::Fled
        } else if npc.current_fear_level >= npc.fear_threshold {
            NPCBehaviorState::Fearful
        } else if npc.current_fear_level > 0.2 {
            NPCBehaviorState::Cautious
        } else {
            NPCBehaviorState::Normal
        };
        
        if new_behavior != npc.response_behavior {
            npc.response_behavior = new_behavior.clone();
            behavioral_changes.push((entity, new_behavior));
            
            // Lock doors if fearful and NPC locks doors
            if npc.locks_doors && npc.current_fear_level >= npc.fear_threshold {
                doors_locked.push(entity);
                info!("NPC {} locked doors due to fear of player", entity.index());
            }
        }
        
        // Spread fear to other NPCs if this NPC spreads fear
        if npc.spreads_fear && npc.current_fear_level > 0.5 {
            social_decay.collective_fear_level += npc.social_influence * 0.01;
        }
    }
    
    // Send social isolation event if significant changes occurred
    if !doors_locked.is_empty() || !behavioral_changes.is_empty() {
        social_isolation_events.send(SocialIsolationEvent {
            affected_npcs: behavioral_changes.iter().map(|(entity, _)| *entity).collect(),
            isolation_trigger: "Player dread level causing NPC fear".to_string(),
            fear_level_increase: player_dread_level * 0.1,
            behavioral_changes,
            doors_locked,
            trade_refusals: vec![], // TODO: Track trade refusals
        });
    }
}

/// Door locking system - NPCs lock doors when player approaches
fn door_locking_system(
    mut social_isolation_events: EventReader<SocialIsolationEvent>,
    // TODO: Add door entity queries and locking mechanics
) {
    for event in social_isolation_events.read() {
        for door_entity in &event.doors_locked {
            // TODO: Actually lock doors in game world
            // This could involve:
            // - Changing door interaction state
            // - Playing door locking sound effects
            // - Updating UI to show locked doors
            // - Preventing player from entering
            
            info!("Door locked by NPC due to fear: entity {}", door_entity.index());
        }
    }
}

/// Social isolation system
fn social_isolation_system(
    mut social_decay: ResMut<SocialDecayState>,
    npcs: Query<&DreadResponsiveNPC>,
    // TODO: Add player reputation and relationship queries
) {
    // Calculate player social isolation based on NPC fear
    let total_fear: f32 = npcs.iter().map(|npc| npc.current_fear_level).sum();
    let npc_count = npcs.iter().count() as f32;
    
    if npc_count > 0.0 {
        social_decay.npc_fear_of_player = total_fear / npc_count;
        social_decay.player_social_isolation_level = social_decay.npc_fear_of_player * 0.8;
        social_decay.social_rejection_level = social_decay.npc_fear_of_player * 0.6;
    }
    
    // Update community cohesion based on collective fear
    social_decay.community_cohesion = (1.0 - social_decay.collective_fear_level).max(0.1);
    social_decay.social_trust_level = social_decay.community_cohesion * 0.9;
    
    if social_decay.player_social_isolation_level > 0.5 {
        warn!("High social isolation: {:.2} - NPCs avoiding player", 
              social_decay.player_social_isolation_level);
    }
}

/// Evacuation system - NPCs organize evacuations when fear is too high
fn evacuation_system(
    mut social_decay: ResMut<SocialDecayState>,
    npcs: Query<(Entity, &DreadResponsiveNPC)>,
    // TODO: Add settlement and population queries
) {
    // Check if evacuation should be triggered
    let evacuation_trigger_threshold = 0.7;
    
    if social_decay.collective_fear_level > evacuation_trigger_threshold {
        // Find evacuation leaders
        let evacuation_leaders: Vec<Entity> = npcs.iter()
            .filter(|(_, npc)| npc.evacuation_leader && npc.response_behavior != NPCBehaviorState::Fled)
            .map(|(entity, _)| entity)
            .collect();
        
        if !evacuation_leaders.is_empty() && social_decay.evacuation_coordination < 0.8 {
            social_decay.evacuation_coordination += 0.1;
            social_decay.refugee_populations += 5; // Some NPCs evacuate
            
            info!("Evacuation organized due to high collective fear: {:.2}", 
                  social_decay.collective_fear_level);
        }
    }
}

// ============================================================================
// REALITY DISTORTION SYSTEMS
// ============================================================================

/// False audio system - sanity-based audio hallucinations
fn false_audio_system(
    mut reality_distortion: ResMut<RealityDistortionState>,
    mut distortion_events: EventWriter<RealityDistortionEvent>,
    // TODO: Add player sanity and position queries
) {
    let player_sanity = 1.0; // TODO: Get actual player sanity
    
    // False audio becomes more frequent as sanity decreases
    if player_sanity < reality_distortion.sanity_distortion_threshold {
        let distortion_probability = reality_distortion.audio_distortion_probability + 
                                   (1.0 - player_sanity) * 0.1;
        
        // TODO: Random chance to trigger false audio
        let should_trigger_false_audio = false; // Placeholder
        
        if should_trigger_false_audio {
            let false_audio_type = reality_distortion.false_audio_types
                .get(0) // TODO: Random selection
                .unwrap_or(&"Unknown sound".to_string())
                .clone();
            
            distortion_events.send(RealityDistortionEvent {
                distortion_type: DistortionType::SanityBased,
                distortion_description: format!("False audio: {}", false_audio_type),
                intensity: 1.0 - player_sanity,
                duration: 3.0,
                affects_audio: true,
                affects_visual: false,
                sanity_cost: 0.05,
                false_information: vec![false_audio_type],
            });
            
            info!("False audio triggered: sanity-based hallucination");
        }
    }
}

/// False visual system - sanity-based visual hallucinations
fn false_visual_system(
    mut reality_distortion: ResMut<RealityDistortionState>,
    mut distortion_events: EventWriter<RealityDistortionEvent>,
    // TODO: Add player sanity and visual field queries
) {
    let player_sanity = 1.0; // TODO: Get actual player sanity
    
    // False visuals become more frequent as sanity decreases
    if player_sanity < reality_distortion.sanity_distortion_threshold {
        reality_distortion.phantom_movement = player_sanity < 0.6;
        reality_distortion.phantom_entities = player_sanity < 0.4;
        
        // TODO: Implement actual false visual generation
        info!("False visual system active - phantom movement: {}", 
              reality_distortion.phantom_movement);
    }
}

/// Sanity-based hallucination system
fn sanity_based_hallucination_system(
    mut distortion_events: EventReader<RealityDistortionEvent>,
    mut reality_distortion: ResMut<RealityDistortionState>,
    // TODO: Add player sanity modification queries
) {
    for event in distortion_events.read() {
        if event.distortion_type == DistortionType::SanityBased {
            // Apply sanity cost from experiencing distortions
            // TODO: Reduce player sanity by event.sanity_cost
            
            // Distortions make reality less stable
            reality_distortion.reality_stability = (reality_distortion.reality_stability - 0.01).max(0.0);
            
            info!("Sanity-based hallucination experienced: {}", event.distortion_description);
        }
    }
}

/// Reality stability system
fn reality_stability_system(
    mut reality_distortion: ResMut<RealityDistortionState>,
    global_decay: Res<GlobalDecayState>,
    // TODO: Add player sanity queries
) {
    let player_sanity = 1.0; // TODO: Get actual player sanity
    
    // Reality becomes less stable as corruption and sanity loss increase
    let stability_pressure = global_decay.global_corruption_level + (1.0 - player_sanity);
    let target_stability = (1.0 - stability_pressure * 0.3).max(0.1);
    
    reality_distortion.reality_stability = reality_distortion.reality_stability
        .lerp(target_stability, 0.01);
    
    // Update distortion parameters based on reality stability
    reality_distortion.distortion_intensity = 1.0 - reality_distortion.reality_stability;
    reality_distortion.distortion_frequency = reality_distortion.distortion_intensity * 0.5;
    
    if reality_distortion.reality_stability < 0.7 {
        warn!("Reality stability compromised: {:.2}", reality_distortion.reality_stability);
    }
}

// ============================================================================
// RECOVERY AND PURIFICATION SYSTEMS
// ============================================================================

/// Natural resistance system - world's innate resistance to corruption
fn natural_resistance_system(
    mut corrupted_areas: Query<&mut EnvironmentalCorruption>,
    global_decay: Res<GlobalDecayState>,
    time: Res<Time>,
) {
    for mut corruption in corrupted_areas.iter_mut() {
        if corruption.natural_resistance > 0.0 {
            // Natural resistance slowly reduces corruption
            let resistance_effect = corruption.natural_resistance * 
                                   global_decay.natural_world_resistance * 
                                   time.delta_seconds() * 0.01;
            
            corruption.corruption_level = (corruption.corruption_level - resistance_effect).max(0.0);
            
            // Areas with high natural resistance may start recovering
            if corruption.corruption_level < 0.2 && corruption.natural_resistance > 0.8 {
                corruption.containment_status = ContainmentStatus::Diminishing;
                info!("Natural resistance taking effect - corruption diminishing");
            }
        }
    }
}

/// Purification opportunity system
fn purification_opportunity_system(
    mut purification_events: EventWriter<PurificationEvent>,
    corrupted_areas: Query<(Entity, &EnvironmentalCorruption)>,
    // TODO: Add purification source queries (companions, items, abilities)
) {
    for (entity, corruption) in corrupted_areas.iter() {
        if corruption.recovery_potential > 0.5 && !corruption.purification_requirements.is_empty() {
            // TODO: Check if purification requirements are met
            let requirements_met = false; // Placeholder
            
            if requirements_met {
                purification_events.send(PurificationEvent {
                    purification_source: Entity::PLACEHOLDER, // TODO: Actual source
                    purified_area: entity,
                    purification_method: "Natural recovery".to_string(),
                    purification_strength: corruption.recovery_potential,
                    corruption_removed: corruption.recovery_potential * 0.5,
                    recovery_potential_restored: 0.2,
                    environmental_healing: corruption.recovery_potential * 0.3,
                });
            }
        }
    }
}

/// Corruption containment system
fn corruption_containment_system(
    mut corrupted_areas: Query<&mut EnvironmentalCorruption>,
    mut global_decay: ResMut<GlobalDecayState>,
) {
    let mut contained_areas = 0;
    let mut spreading_areas = 0;
    
    for mut corruption in corrupted_areas.iter_mut() {
        // Check if corruption should be contained
        if corruption.spread_resistance > 0.7 && corruption.corruption_level < 0.8 {
            corruption.containment_status = ContainmentStatus::Contained;
            contained_areas += 1;
        } else if corruption.corruption_level > 0.9 {
            corruption.containment_status = ContainmentStatus::Accelerating;
            spreading_areas += 1;
        } else {
            corruption.containment_status = ContainmentStatus::Spreading;
        }
        
        // Adjust spread rate based on containment status
        match corruption.containment_status {
            ContainmentStatus::Contained => {
                corruption.spread_rate *= 0.5; // Slower spread when contained
            },
            ContainmentStatus::Accelerating => {
                corruption.spread_rate *= 1.5; // Faster spread when accelerating
            },
            ContainmentStatus::Diminishing => {
                corruption.spread_rate = 0.0; // No spread when diminishing
            },
            _ => {} // Normal spread rate
        }
    }
    
    // Update global containment metrics
    if contained_areas > spreading_areas {
        global_decay.corruption_spread_rate *= 0.9; // Global spread slows down
        info!("Corruption containment improving: {} contained, {} spreading", 
              contained_areas, spreading_areas);
    } else if spreading_areas > contained_areas * 2 {
        global_decay.corruption_spread_rate *= 1.1; // Global spread accelerates
        warn!("Corruption containment failing: {} contained, {} spreading", 
              contained_areas, spreading_areas);
    }
}
