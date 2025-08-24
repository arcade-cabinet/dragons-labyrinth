//! Dual Forge System - Ultimate endgame test with sophisticated mechanics
//! 
//! This module implements the revolutionary dual forge system discovered in vision integration:
//! - Forge of High Elves (Light Path) vs Cursed Forge (Dark Path) 
//! - Sentimental items collected throughout game as reagents
//! - Trials that test ALL game systems (hex navigation, mounted combat, first-person, party coordination)
//! - Companion sacrifice mechanics (essence vs blood) affecting mythic gear power
//! - Ultimate moral choice with permanent consequences

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod trials;
pub mod sentimental_items;
pub mod sacrifice;
pub mod forge_paths;

pub use trials::*;
pub use sentimental_items::*;
pub use sacrifice::*;
pub use forge_paths::*;

// ============================================================================
// FORGE SYSTEM PLUGIN
// ============================================================================

pub struct ForgeSystemPlugin;

impl Plugin for ForgeSystemPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<ForgeState>()
            .init_resource::<SentimentalReagents>()
            .init_resource::<ForgeTrialStatus>()
            
            // Events
            .add_event::<ForgePathChosenEvent>()
            .add_event::<TrialCompletedEvent>()
            .add_event::<SacrificeOfferedEvent>()
            .add_event::<ForgeActivatedEvent>()
            .add_event::<MythicGearCreatedEvent>()
            
            // Systems
            .add_systems(Startup, setup_forge_system)
            .add_systems(Update, (
                // Forge progression systems
                check_forge_readiness,
                process_forge_trials,
                handle_sentimental_reagent_collection,
                evaluate_sacrifice_offerings,
                
                // Forge path systems
                light_forge_mechanics,
                dark_forge_mechanics,
                forge_path_commitment_tracking,
                
                // Trial orchestration systems
                hex_navigation_trial_system,
                mounted_combat_trial_system,
                first_person_trial_system,
                party_coordination_trial_system,
                
                // Sacrifice and mythic gear systems
                companion_sacrifice_system,
                mythic_gear_blessing_system,
                mythic_gear_curse_system,
                
                // Memory and emotional systems
                memory_trigger_system,
                emotional_resonance_system,
            ).chain());
    }
}

// ============================================================================
// CORE FORGE COMPONENTS
// ============================================================================

/// Component marking entities capable of forge interaction
#[derive(Component, Reflect, Clone, Debug, Serialize, Deserialize)]
#[reflect(Component)]
pub struct ForgeEntity {
    pub forge_type: ForgeType,
    pub activation_level: f32,    // 0.0-1.0 how active forge is
    pub accumulated_power: f32,   // Power from reagents and sacrifices
    pub trials_required: Vec<String>, // Which trials must be completed
    pub is_accessible: bool,      // Can player access forge right now?
}

/// Component for items that have sentimental value as forge reagents
#[derive(Component, Reflect, Clone, Debug, Serialize, Deserialize)]
#[reflect(Component)]
pub struct SentimentalReagent {
    pub emotional_weight: f32,           // 0.0-1.0 how meaningful to player
    pub memory_description: String,      // Why this matters
    pub sentimental_category: SentimentalCategory,
    pub forge_power: f32,               // Power when used as reagent
    pub light_compatibility: f32,       // 0.0-1.0 compatibility with light forge
    pub dark_compatibility: f32,        // 0.0-1.0 compatibility with dark forge
    pub essence_vs_blood_ratio: f32,    // -1.0 to 1.0 (essence to blood)
    pub triggers_memory: bool,          // Does using this trigger flashback?
    pub sacrifice_resistance: f32,      // How hard it is to give up
}

/// Component for tracking forge trial progression
#[derive(Component, Reflect, Clone, Debug, Serialize, Deserialize)]
#[reflect(Component)]
pub struct ForgeTrialProgress {
    pub trial_type: TrialType,
    pub completion_status: TrialStatus,
    pub attempts_made: u32,
    pub best_score: f32,             // 0.0-1.0 best performance
    pub required_score: f32,         // 0.0-1.0 score needed to pass
    pub failure_reasons: Vec<String>, // Why attempts failed
    pub trial_specific_data: serde_json::Value, // Trial-specific progress
}

/// Component for companions offered for sacrifice
#[derive(Component, Reflect, Clone, Debug, Serialize, Deserialize)]
#[reflect(Component)]
pub struct SacrificeCandidate {
    pub companion_id: Uuid,
    pub sacrifice_method: Option<SacrificeMethod>,
    pub power_potential: f32,        // Power gained from this sacrifice
    pub player_resistance: f32,      // How much player resists sacrificing them
    pub companion_willingness: f32,  // How willing companion is
    pub emotional_cost: f32,         // Trauma to player from sacrifice
    pub mythic_gear_influence: MythicGearType, // What gear would be influenced
}

// ============================================================================
// FORGE ENUMS AND TYPES
// ============================================================================

#[derive(Reflect, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum ForgeType {
    HighElvesForge,  // Light path - tests moral purity and selflessness
    CursedForge,     // Dark path - tests willpower and sacrifice tolerance
}

#[derive(Reflect, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum SentimentalCategory {
    Friendship,  // Items representing bonds with companions
    Love,        // Items representing romantic connections
    Loss,        // Items representing grief and mourning
    Hope,        // Items representing optimism and future
    Fear,        // Items representing trauma and anxiety
    Wisdom,      // Items representing learning and growth
    Innocence,   // Items representing purity and childhood
    Sacrifice,   // Items representing things given up
}

#[derive(Reflect, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum TrialType {
    HexNavigation,      // Tests hex grid movement mastery
    MountedCombat,      // Tests combat while mounted
    FirstPerson,        // Tests first-person perspective skills
    PartyCoordination,  // Tests multi-companion coordination
    SacrificeResolve,   // Tests willingness to sacrifice for power
    MemoryIntegration,  // Tests ability to process memories
    MoralConsistency,   // Tests consistency with chosen philosophy
    UltimateChoice,     // Final test combining all systems
}

#[derive(Reflect, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum TrialStatus {
    NotStarted,
    InProgress,
    Failed,
    Passed,
    Mastered,  // Exceptional performance
}

#[derive(Reflect, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum SacrificeMethod {
    Essence,  // Painless extraction of companion essence (light path)
    Blood,    // Painful blood sacrifice (dark path)
    Memory,   // Sacrifice memories of companion (ultimate corruption)
}

#[derive(Reflect, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum MythicGearType {
    LightBlessedWeapon,   // Weapon blessed by High Elves forge
    DarkCursedWeapon,     // Weapon cursed by dark forge
    LightBlessedArmor,    // Armor blessed with protection
    DarkCursedArmor,      // Armor cursed with power
    LightBlessedAccessory, // Accessory with light magic
    DarkCursedAccessory,  // Accessory with dark magic
}

// ============================================================================
// FORGE RESOURCES
// ============================================================================

/// Global forge system state
#[derive(Resource, Reflect, Clone, Debug, Serialize, Deserialize)]
#[reflect(Resource)]
pub struct ForgeState {
    // Forge accessibility
    pub light_forge_discovered: bool,
    pub dark_forge_discovered: bool,
    pub both_forges_accessible: bool,
    
    // Path progression
    pub chosen_path: Option<ForgeType>,
    pub path_commitment_level: f32,     // 0.0-1.0 how committed to chosen path
    pub can_still_switch_paths: bool,   // Can player change their mind?
    pub path_switch_cost: f32,          // Cost of switching paths
    
    // Trial progression
    pub total_trials_available: u32,
    pub trials_completed: u32,
    pub trials_failed: u32,
    pub current_trial: Option<TrialType>,
    pub trial_unlock_progression: f32,  // 0.0-1.0 how many trials unlocked
    
    // Forge readiness
    pub readiness_score: f32,           // 0.0-1.0 ready for final activation
    pub missing_requirements: Vec<String>, // What's still needed
    pub forge_master_approval: bool,    // Has forge master approved player?
    pub ultimate_choice_available: bool, // Can make final forge choice?
    
    // Power accumulation
    pub total_reagent_power: f32,       // Power from sentimental items
    pub total_sacrifice_power: f32,     // Power from companion sacrifices
    pub light_path_power: f32,          // Power aligned with light
    pub dark_path_power: f32,           // Power aligned with dark
    pub power_balance: f32,             // -1.0 to 1.0 (dark to light)
    
    // Final forge activation
    pub forge_activated: bool,
    pub activation_timestamp: Option<f64>, // Game time when activated
    pub final_choice_made: Option<String>, // What choice was made
    pub mythic_gear_created: Vec<MythicGearType>, // Gear created from forge
}

impl Default for ForgeState {
    fn default() -> Self {
        Self {
            light_forge_discovered: false,
            dark_forge_discovered: false,
            both_forges_accessible: false,
            chosen_path: None,
            path_commitment_level: 0.0,
            can_still_switch_paths: true,
            path_switch_cost: 0.0,
            total_trials_available: 8,
            trials_completed: 0,
            trials_failed: 0,
            current_trial: None,
            trial_unlock_progression: 0.0,
            readiness_score: 0.0,
            missing_requirements: vec![
                "Complete hex navigation trial".to_string(),
                "Complete mounted combat trial".to_string(),
                "Complete first-person trial".to_string(),
                "Complete party coordination trial".to_string(),
                "Collect 5+ sentimental reagents".to_string(),
                "Choose forge path commitment".to_string(),
            ],
            forge_master_approval: false,
            ultimate_choice_available: false,
            total_reagent_power: 0.0,
            total_sacrifice_power: 0.0,
            light_path_power: 0.0,
            dark_path_power: 0.0,
            power_balance: 0.0,
            forge_activated: false,
            activation_timestamp: None,
            final_choice_made: None,
            mythic_gear_created: Vec::new(),
        }
    }
}

/// Resource tracking sentimental reagents collected throughout game
#[derive(Resource, Reflect, Clone, Debug, Serialize, Deserialize)]
#[reflect(Resource)]
pub struct SentimentalReagents {
    pub collected_items: Vec<SentimentalReagentData>,
    pub total_emotional_weight: f32,
    pub light_reagent_power: f32,
    pub dark_reagent_power: f32,
    pub memory_triggers_available: u32,
    pub categories_represented: Vec<SentimentalCategory>,
}

#[derive(Reflect, Clone, Debug, Serialize, Deserialize)]
pub struct SentimentalReagentData {
    pub item_entity: Entity,
    pub reagent_data: SentimentalReagent,
    pub collection_story: String,       // Story of how player acquired this
    pub emotional_journey: Vec<String>,  // How meaning has evolved
    pub usage_potential: f32,           // How much power using this would give
    pub sacrifice_difficulty: f32,      // How hard it would be to give up
}

impl Default for SentimentalReagents {
    fn default() -> Self {
        Self {
            collected_items: Vec::new(),
            total_emotional_weight: 0.0,
            light_reagent_power: 0.0,
            dark_reagent_power: 0.0,
            memory_triggers_available: 0,
            categories_represented: Vec::new(),
        }
    }
}

/// Resource tracking trial progression and requirements
#[derive(Resource, Reflect, Clone, Debug, Serialize, Deserialize)]
#[reflect(Resource)]
pub struct ForgeTrialStatus {
    pub available_trials: Vec<TrialType>,
    pub completed_trials: Vec<TrialType>,
    pub failed_trials: Vec<(TrialType, String)>, // Trial and failure reason
    pub current_trial_progress: Option<TrialProgressData>,
    pub trial_scores: std::collections::HashMap<String, f32>, // Trial name -> score
    pub overall_trial_mastery: f32,     // 0.0-1.0 overall performance
    pub trials_unlock_conditions_met: bool,
}

#[derive(Reflect, Clone, Debug, Serialize, Deserialize)]
pub struct TrialProgressData {
    pub trial_type: TrialType,
    pub current_phase: String,          // Current phase of trial
    pub phase_progress: f32,            // 0.0-1.0 progress in current phase
    pub performance_metrics: serde_json::Value, // Trial-specific metrics
    pub attempts_this_session: u32,
    pub time_spent_seconds: f32,
}

impl Default for ForgeTrialStatus {
    fn default() -> Self {
        Self {
            available_trials: vec![TrialType::HexNavigation], // Start with basic trial
            completed_trials: Vec::new(),
            failed_trials: Vec::new(),
            current_trial_progress: None,
            trial_scores: std::collections::HashMap::new(),
            overall_trial_mastery: 0.0,
            trials_unlock_conditions_met: false,
        }
    }
}

// ============================================================================
// FORGE EVENTS
// ============================================================================

#[derive(Event, Reflect, Clone, Debug)]
#[reflect(Default)]
pub struct ForgePathChosenEvent {
    pub player_entity: Entity,
    pub chosen_path: ForgeType,
    pub commitment_level: f32,
    pub reasoning: String,              // Player's stated reasoning
    pub companion_reactions: Vec<(Entity, String)>, // How companions react
}

impl Default for ForgePathChosenEvent {
    fn default() -> Self {
        Self {
            player_entity: Entity::PLACEHOLDER,
            chosen_path: ForgeType::HighElvesForge,
            commitment_level: 0.0,
            reasoning: String::new(),
            companion_reactions: Vec::new(),
        }
    }
}

#[derive(Event, Reflect, Clone, Debug)]
#[reflect(Default)]
pub struct TrialCompletedEvent {
    pub player_entity: Entity,
    pub trial_type: TrialType,
    pub completion_status: TrialStatus,
    pub performance_score: f32,         // 0.0-1.0 performance on trial
    pub attempts_taken: u32,
    pub insights_gained: Vec<String>,   // What player learned
    pub unlocked_trials: Vec<TrialType>, // Trials unlocked by completion
}

impl Default for TrialCompletedEvent {
    fn default() -> Self {
        Self {
            player_entity: Entity::PLACEHOLDER,
            trial_type: TrialType::HexNavigation,
            completion_status: TrialStatus::NotStarted,
            performance_score: 0.0,
            attempts_taken: 0,
            insights_gained: Vec::new(),
            unlocked_trials: Vec::new(),
        }
    }
}

#[derive(Event, Reflect, Clone, Debug)]
#[reflect(Default)]
pub struct SacrificeOfferedEvent {
    pub player_entity: Entity,
    pub sacrifice_entity: Entity,       // Companion or item being sacrificed
    pub sacrifice_type: SacrificeType,
    pub method: SacrificeMethod,
    pub power_gained: f32,
    pub emotional_cost: f32,            // Trauma to player
    pub resistance_overcome: f32,       // How much resistance was overcome
}

#[derive(Reflect, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum SacrificeType {
    CompanionEssence,   // Extracting companion's essence
    CompanionBlood,     // Blood sacrifice of companion
    SentimentalItem,    // Sacrificing meaningful item
    Memory,             // Sacrificing memories
    PlayerSacrifice,    // Player sacrificing part of themselves
}

impl Default for SacrificeOfferedEvent {
    fn default() -> Self {
        Self {
            player_entity: Entity::PLACEHOLDER,
            sacrifice_entity: Entity::PLACEHOLDER,
            sacrifice_type: SacrificeType::SentimentalItem,
            method: SacrificeMethod::Essence,
            power_gained: 0.0,
            emotional_cost: 0.0,
            resistance_overcome: 0.0,
        }
    }
}

#[derive(Event, Reflect, Clone, Debug)]
#[reflect(Default)]
pub struct ForgeActivatedEvent {
    pub player_entity: Entity,
    pub forge_type: ForgeType,
    pub final_decision: String,         // Ultimate choice made
    pub reagents_consumed: Vec<Entity>,  // Sentimental items used
    pub companions_sacrificed: Vec<Entity>, // Companions sacrificed
    pub total_power_channeled: f32,
    pub mythic_gear_created: Vec<(Entity, MythicGearType)>,
    pub world_impact: serde_json::Value, // How this affects the world
}

impl Default for ForgeActivatedEvent {
    fn default() -> Self {
        Self {
            player_entity: Entity::PLACEHOLDER,
            forge_type: ForgeType::HighElvesForge,
            final_decision: String::new(),
            reagents_consumed: Vec::new(),
            companions_sacrificed: Vec::new(),
            total_power_channeled: 0.0,
            mythic_gear_created: Vec::new(),
            world_impact: serde_json::Value::Null,
        }
    }
}

#[derive(Event, Reflect, Clone, Debug)]
#[reflect(Default)]
pub struct MythicGearCreatedEvent {
    pub player_entity: Entity,
    pub gear_entity: Entity,
    pub gear_type: MythicGearType,
    pub blessing_power: f32,            // Positive power (light path)
    pub curse_power: f32,               // Negative power (dark path)
    pub reagents_used: Vec<String>,     // Which reagents influenced this gear
    pub sacrifices_used: Vec<String>,   // Which sacrifices influenced this gear
    pub gear_personality: String,       // Personality of the mythic gear
    pub special_abilities: Vec<String>, // Unique abilities granted
}

impl Default for MythicGearCreatedEvent {
    fn default() -> Self {
        Self {
            player_entity: Entity::PLACEHOLDER,
            gear_entity: Entity::PLACEHOLDER,
            gear_type: MythicGearType::LightBlessedWeapon,
            blessing_power: 0.0,
            curse_power: 0.0,
            reagents_used: Vec::new(),
            sacrifices_used: Vec::new(),
            gear_personality: String::new(),
            special_abilities: Vec::new(),
        }
    }
}

// ============================================================================
// FORGE SYSTEM SETUP
// ============================================================================

fn setup_forge_system(mut commands: Commands) {
    info!("Initializing Dual Forge System - Light vs Dark paths");
    
    // Create forge master entities (NPCs who run the forges)
    commands.spawn((
        Name::new("High Elf Forge Master"),
        ForgeEntity {
            forge_type: ForgeType::HighElvesForge,
            activation_level: 0.0,
            accumulated_power: 0.0,
            trials_required: vec![
                "hex_navigation".to_string(),
                "mounted_combat".to_string(),
                "first_person".to_string(),
                "party_coordination".to_string(),
                "sacrifice_resolve".to_string(),
                "memory_integration".to_string(),
                "moral_consistency".to_string(),
            ],
            is_accessible: false, // Unlocked through story progression
        },
        // Additional components for forge master
        Transform::default(),
    ));
    
    commands.spawn((
        Name::new("Cursed Forge Master"),
        ForgeEntity {
            forge_type: ForgeType::CursedForge,
            activation_level: 0.0,
            accumulated_power: 0.0,
            trials_required: vec![
                "hex_navigation".to_string(),
                "mounted_combat".to_string(),
                "first_person".to_string(),
                "party_coordination".to_string(),
                "sacrifice_resolve".to_string(),
                "memory_integration".to_string(),
                "moral_consistency".to_string(),
            ],
            is_accessible: false, // Unlocked through different story path
        },
        Transform::default(),
    ));
    
    info!("Dual Forge System initialized - trials await");
}

// ============================================================================
// CORE FORGE SYSTEMS
// ============================================================================

/// Check if player is ready to access forge trials
fn check_forge_readiness(
    mut forge_state: ResMut<ForgeState>,
    reagents: Res<SentimentalReagents>,
    trial_status: Res<ForgeTrialStatus>,
    // TODO: Add query for player dread level, companion status, etc.
) {
    // Calculate readiness based on multiple factors
    let reagent_readiness = (reagents.collected_items.len() as f32 / 5.0).min(1.0);
    let trial_readiness = trial_status.overall_trial_mastery;
    let path_readiness = if forge_state.chosen_path.is_some() { 1.0 } else { 0.0 };
    
    forge_state.readiness_score = (reagent_readiness + trial_readiness + path_readiness) / 3.0;
    
    // Update missing requirements
    forge_state.missing_requirements.clear();
    
    if reagents.collected_items.len() < 5 {
        forge_state.missing_requirements.push(
            format!("Collect {} more sentimental reagents", 5 - reagents.collected_items.len())
        );
    }
    
    if trial_status.completed_trials.len() < 4 {
        forge_state.missing_requirements.push(
            "Complete core trials (navigation, combat, first-person, coordination)".to_string()
        );
    }
    
    if forge_state.chosen_path.is_none() {
        forge_state.missing_requirements.push(
            "Choose forge path (Light: High Elves or Dark: Cursed)".to_string()
        );
    }
    
    // Ultimate choice becomes available when readiness is high enough
    forge_state.ultimate_choice_available = forge_state.readiness_score >= 0.8 
        && forge_state.forge_master_approval;
}

/// Handle collection of sentimental items as forge reagents
fn handle_sentimental_reagent_collection(
    mut reagents: ResMut<SentimentalReagents>,
    mut events: EventReader<SacrificeOfferedEvent>,
    query: Query<(Entity, &SentimentalReagent)>,
) {
    // Process new sentimental reagents being offered
    for event in events.read() {
        if event.sacrifice_type == SacrificeType::SentimentalItem {
            if let Ok((entity, reagent)) = query.get(event.sacrifice_entity) {
                let reagent_data = SentimentalReagentData {
                    item_entity: entity,
                    reagent_data: reagent.clone(),
                    collection_story: format!("Offered for forge with emotional weight {:.2}", reagent.emotional_weight),
                    emotional_journey: vec![
                        format!("Acquired: {}", reagent.memory_description),
                        "Offered to forge as reagent".to_string(),
                    ],
                    usage_potential: reagent.forge_power,
                    sacrifice_difficulty: reagent.sacrifice_resistance,
                };
                
                reagents.collected_items.push(reagent_data);
                reagents.total_emotional_weight += reagent.emotional_weight;
                reagents.light_reagent_power += reagent.light_compatibility * reagent.forge_power;
                reagents.dark_reagent_power += reagent.dark_compatibility * reagent.forge_power;
                
                if reagent.triggers_memory {
                    reagents.memory_triggers_available += 1;
                }
                
                if !reagents.categories_represented.contains(&reagent.sentimental_category) {
                    reagents.categories_represented.push(reagent.sentimental_category);
                }
                
                info!("Sentimental reagent collected: {} (power: {:.2})", 
                      reagent.memory_description, reagent.forge_power);
            }
        }
    }
}

/// Evaluate and process sacrifice offerings 
fn evaluate_sacrifice_offerings(
    mut sacrifice_events: EventReader<SacrificeOfferedEvent>,
    mut forge_state: ResMut<ForgeState>,
    mut commands: Commands,
) {
    for event in sacrifice_events.read() {
        match event.sacrifice_type {
            SacrificeType::CompanionEssence => {
                // Light path sacrifice - painless essence extraction
                forge_state.light_path_power += event.power_gained;
                forge_state.total_sacrifice_power += event.power_gained;
                
                info!("Companion essence sacrificed to light forge (power: {:.2})", event.power_gained);
                
                // Spawn mythic gear blessed by sacrifice
                spawn_mythic_gear(&mut commands, MythicGearType::LightBlessedWeapon, event.power_gained);
            },
            
            SacrificeType::CompanionBlood => {
                // Dark path sacrifice - painful blood ritual
                forge_state.dark_path_power += event.power_gained * 1.5; // Dark path more powerful
                forge_state.total_sacrifice_power += event.power_gained;
                
                info!("Companion blood sacrificed to dark forge (power: {:.2})", event.power_gained * 1.5);
                
                // Spawn mythic gear cursed by sacrifice
                spawn_mythic_gear(&mut commands, MythicGearType::DarkCursedWeapon, event.power_gained * 1.5);
            },
            
            SacrificeType::Memory => {
                // Ultimate corruption - sacrificing memories of companion
                forge_state.dark_path_power += event.power_gained * 2.0; // Extremely powerful
                forge_state.power_balance -= 0.5; // Push toward dark
                
                warn!("Memory sacrifice detected - extreme corruption path chosen");
                
                // Memory sacrifice creates the most powerful but most cursed gear
                spawn_mythic_gear(&mut commands, MythicGearType::DarkCursedAccessory, event.power_gained * 2.0);
            },
            
            _ => {
                // Handle other sacrifice types
                info!("Other sacrifice type processed: {:?}", event.sacrifice_type);
            }
        }
        
        // Update power balance between light and dark
        let total_power = forge_state.light_path_power + forge_state.dark_path_power;
        if total_power > 0.0 {
            forge_state.power_balance = (forge_state.light_path_power - forge_state.dark_path_power) / total_power;
        }
    }
}

/// Helper function to spawn mythic gear with appropriate properties
fn spawn_mythic_gear(
    commands: &mut Commands,
    gear_type: MythicGearType,
    power_level: f32,
) {
    let gear_entity = commands.spawn((
        Name::new(format!("Mythic {:?}", gear_type)),
        // TODO: Add item components, visual components, etc.
        Transform::default(),
    )).id();
    
    // TODO: Send MythicGearCreatedEvent with details
    info!("Mythic gear created: {:?} with power {:.2}", gear_type, power_level);
}

// ============================================================================
// FORGE PATH MECHANICS
// ============================================================================

/// Light forge mechanics - emphasis on selflessness and moral purity
fn light_forge_mechanics(
    mut forge_state: ResMut<ForgeState>,
    reagents: Res<SentimentalReagents>,
    // TODO: Add queries for companion loyalty, moral choices, etc.
) {
    if let Some(ForgeType::HighElvesForge) = forge_state.chosen_path {
        // Light forge requires selfless reagents and willing companion sacrifices
        let selfless_power = reagents.collected_items.iter()
            .filter(|item| matches!(item.reagent_data.sentimental_category, 
                SentimentalCategory::Friendship | SentimentalCategory::Hope | SentimentalCategory::Wisdom))
            .map(|item| item.reagent_data.forge_power * item.reagent_data.light_compatibility)
            .sum::<f32>();
        
        forge_state.light_path_power = selfless_power;
        
        // Light path becomes stronger with moral consistency
        // TODO: Query player's moral choices and calculate bonus
    }
}

/// Dark forge mechanics - emphasis on willpower and sacrifice tolerance
fn dark_forge_mechanics(
    mut forge_state: ResMut<ForgeState>,
    reagents: Res<SentimentalReagents>,
    // TODO: Add queries for player corruption, sacrifice history, etc.
) {
    if let Some(ForgeType::CursedForge) = forge_state.chosen_path {
        // Dark forge requires painful sacrifices and corrupted reagents
        let dark_power = reagents.collected_items.iter()
            .filter(|item| matches!(item.reagent_data.sentimental_category,
                SentimentalCategory::Loss | SentimentalCategory::Fear | SentimentalCategory::Sacrifice))
            .map(|item| item.reagent_data.forge_power * item.reagent_data.dark_compatibility)
            .sum::<f32>();
        
        forge_state.dark_path_power = dark_power;
        
        // Dark path becomes stronger with willingness to sacrifice
        // TODO: Query player's sacrifice history and calculate bonus
    }
}

/// Track commitment to chosen forge path
fn forge_path_commitment_tracking(
    mut forge_state: ResMut<ForgeState>,
    // TODO: Add queries for player actions that show commitment
) {
    if let Some(chosen_path) = &forge_state.chosen_path {
        // Increase commitment based on consistent actions
        match chosen_path {
            ForgeType::HighElvesForge => {
                // Light path commitment increases with selfless acts
                // TODO: Track selfless choices and increase commitment
            },
            ForgeType::CursedForge => {
                // Dark path commitment increases with sacrificial acts
                // TODO: Track sacrifice choices and increase commitment
            }
        }
        
        // Path switching becomes harder as commitment increases
        forge_state.path_switch_cost = forge_state.path_commitment_level * 10.0;
        forge_state.can_still_switch_paths = forge_state.path_commitment_level < 0.8;
    }
}

// ============================================================================
// TRIAL ORCHESTRATION SYSTEMS (Tests ALL game systems)
// ============================================================================

/// Hex navigation trial system - tests mastery of hex grid movement
fn hex_navigation_trial_system(
    mut trial_status: ResMut<ForgeTrialStatus>,
    // TODO: Add hex position queries, movement tracking, etc.
) {
    if let Some(progress) = &mut trial_status.current_trial_progress {
        if progress.trial_type == TrialType::HexNavigation {
            // Test hex grid movement efficiency, pathfinding, tactical positioning
            // TODO: Implement hex navigation challenges
            info!("Hex navigation trial in progress...");
        }
    }
}

/// Mounted combat trial system - tests combat while mounted
fn mounted_combat_trial_system(
    mut trial_status: ResMut<ForgeTrialStatus>,
    // TODO: Add mount queries, combat queries, etc.
) {
    if let Some(progress) = &mut trial_status.current_trial_progress {
        if progress.trial_type == TrialType::MountedCombat {
            // Test mounted combat coordination, mount-rider synergy, tactical advantage usage
            // TODO: Implement mounted combat challenges
            info!("Mounted combat trial in progress...");
        }
    }
}

/// First-person trial system - tests first-person perspective skills
fn first_person_trial_system(
    mut trial_status: ResMut<ForgeTrialStatus>,
    // TODO: Add first-person perspective queries, navigation in tight spaces, etc.
) {
    if let Some(progress) = &mut trial_status.current_trial_progress {
        if progress.trial_type == TrialType::FirstPerson {
            // Test first-person navigation, spatial awareness, labyrinth skills
            // TODO: Implement first-person challenges (preparation for dragon encounter)
            info!("First-person trial in progress...");
        }
    }
}

/// Party coordination trial system - tests multi-companion coordination
fn party_coordination_trial_system(
    mut trial_status: ResMut<ForgeTrialStatus>,
    // TODO: Add companion queries, coordination tracking, etc.
) {
    if let Some(progress) = &mut trial_status.current_trial_progress {
        if progress.trial_type == TrialType::PartyCoordination {
            // Test multi-companion tactics, leadership, group decision making
            // TODO: Implement party coordination challenges
            info!("Party coordination trial in progress...");
        }
    }
}

/// Process active forge trials
fn process_forge_trials(
    mut trial_events: EventWriter<TrialCompletedEvent>,
    mut trial_status: ResMut<ForgeTrialStatus>,
    // TODO: Add trial-specific queries
) {
    // Handle trial progression and completion logic
    if let Some(progress) = &trial_status.current_trial_progress {
        // TODO: Check trial completion conditions and emit events
        info!("Processing trial: {:?}", progress.trial_type);
    }
}

/// Companion sacrifice system with essence vs blood mechanics
fn companion_sacrifice_system(
    mut sacrifice_events: EventWriter<SacrificeOfferedEvent>,
    sacrifice_candidates: Query<(Entity, &SacrificeCandidate)>,
    // TODO: Add companion state queries, player input queries, etc.
) {
    // Handle companion sacrifice mechanics
    for (entity, candidate) in sacrifice_candidates.iter() {
        // TODO: Implement sacrifice decision logic
        info!("Sacrifice candidate available: {:?}", candidate.sacrifice_method);
    }
}

/// Mythic gear blessing system for light path
fn mythic_gear_blessing_system(
    mut gear_events: EventWriter<MythicGearCreatedEvent>,
    forge_state: Res<ForgeState>,
    // TODO: Add gear queries, blessing application, etc.
) {
    if forge_state.light_path_power > 0.0 {
        // TODO: Apply light blessings to gear based on reagents and sacrifices
        info!("Light path gear blessings available");
    }
}

/// Mythic gear curse system for dark path
fn mythic_gear_curse_system(
    mut gear_events: EventWriter<MythicGearCreatedEvent>,
    forge_state: Res<ForgeState>,
    // TODO: Add gear queries, curse application, etc.
) {
    if forge_state.dark_path_power > 0.0 {
        // TODO: Apply dark curses to gear based on reagents and sacrifices
        info!("Dark path gear curses available");
    }
}

/// Memory trigger system for sentimental reagents
fn memory_trigger_system(
    reagents: Res<SentimentalReagents>,
    // TODO: Add memory system queries, flashback triggers, etc.
) {
    // Handle memory flashbacks triggered by sentimental reagents
    for reagent_data in &reagents.collected_items {
        if reagent_data.reagent_data.triggers_memory {
            // TODO: Trigger memory flashback system
            info!("Memory trigger available: {}", reagent_data.reagent_data.memory_description);
        }
    }
}

/// Emotional resonance system for forge reagents
fn emotional_resonance_system(
    reagents: Res<SentimentalReagents>,
    // TODO: Add emotional state queries, resonance calculations, etc.
) {
    // Calculate emotional resonance between reagents and current situation
    let total_emotional_resonance = reagents.total_emotional_weight;
    
    if total_emotional_resonance > 0.0 {
        // TODO: Apply emotional effects based on reagent resonance
        info!("Emotional resonance level: {:.2}", total_emotional_resonance);
    }
}
