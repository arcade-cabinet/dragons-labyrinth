//! Narrative and dialogue components
//!
//! Story progression, dialogue trees, and quest management.
//! Integrates with YarnSpinner for dialogue and Cobweb for narrative graphs.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Quest component - tracks narrative objectives
#[derive(Component, Clone, Debug)]
pub struct Quest {
    pub id: String,
    pub title: String,
    pub description: String,
    pub quest_type: QuestType,
    pub moral_weight: MoralWeight,
    pub stages: Vec<QuestStage>,
    pub current_stage: usize,
    pub dread_requirement: u8, // Min dread level to appear
    pub consequences: Vec<Consequence>,
}

#[derive(Clone, Debug)]
pub enum QuestType {
    // Peace (Dread 0)
    SimpleDelivery,     // "Take bread to miller"
    SocialInteraction,  // "Convince merchant to lower prices"
    
    // Unease (Dread 1)
    Investigation,      // "Find missing villager"
    MinorChoice,        // "Report theft or keep quiet"
    
    // Anxiety (Dread 2)
    ResourceGathering,  // "Find food for starving family"
    TrustDecision,      // "Who's telling the truth?"
    
    // Horror (Dread 3)
    SurvivalChoice,     // "Save companion or villagers"
    CompanionRequest,   // "Your friend begs for death"
    
    // Madness (Dread 4)
    ImpossibleChoice,   // "Kill loved one or damn village"
    FinalConfrontation, // Dragon encounter
}

#[derive(Clone, Debug)]
pub enum MoralWeight {
    Trivial,     // No real consequences
    Minor,       // Small impact on world/companions
    Significant, // Major impact on trust/relationships
    Critical,    // Determines companion fate
    Defining,    // Shapes ending
}

#[derive(Clone, Debug)]
pub struct QuestStage {
    pub description: String,
    pub objectives: Vec<Objective>,
    pub dialogue_trigger: Option<String>, // YarnSpinner node
    pub completion_event: Option<String>,
}

#[derive(Clone, Debug)]
pub struct Objective {
    pub description: String,
    pub completed: bool,
    pub optional: bool,
}

#[derive(Clone, Debug)]
pub struct Consequence {
    pub description: String,
    pub impact_type: ConsequenceType,
    pub magnitude: f32,
}

#[derive(Clone, Debug)]
pub enum ConsequenceType {
    CompanionTrust(String),    // Affects specific companion
    GlobalDread,                // Increases world dread
    ResourceLoss,               // Lose items/money
    ReputationChange,           // Village opinion
    StoryBranch,                // Locks/unlocks paths
    CharacterDeath(String),     // Someone dies
}

/// Dialogue component for YarnSpinner integration
#[derive(Component, Clone, Debug)]
pub struct DialogueRunner {
    pub current_node: Option<String>,
    pub yarn_file: String,
    pub variables: DialogueVariables,
    pub active: bool,
}

#[derive(Clone, Debug, Default)]
pub struct DialogueVariables {
    pub player_name: String,
    pub dread_level: u8,
    pub companion_states: Vec<(String, String)>,
    pub quest_flags: Vec<String>,
    pub custom: std::collections::HashMap<String, String>,
}

/// Choice tracking for moral system
#[derive(Component, Clone, Debug)]
pub struct MoralChoice {
    pub choice_id: String,
    pub description: String,
    pub options: Vec<ChoiceOption>,
    pub time_limit: Option<f32>,
    pub default_option: usize,
}

#[derive(Clone, Debug)]
pub struct ChoiceOption {
    pub text: String,
    pub requirements: Vec<ChoiceRequirement>,
    pub consequences: Vec<Consequence>,
    pub companion_reactions: Vec<(String, f32)>, // (name, trust_change)
}

#[derive(Clone, Debug)]
pub enum ChoiceRequirement {
    MinDread(u8),
    MaxDread(u8),
    CompanionPresent(String),
    CompanionTrust(String, f32),
    ItemRequired(String),
    QuestComplete(String),
}

/// Story node for Cobweb integration
#[derive(Component, Clone, Debug)]
pub struct StoryNode {
    pub node_id: String,
    pub content: String,
    pub branches: Vec<StoryBranch>,
    pub visited: bool,
    pub locked: bool,
}

#[derive(Clone, Debug)]
pub struct StoryBranch {
    pub description: String,
    pub target_node: String,
    pub requirements: Vec<BranchRequirement>,
    pub one_time: bool,
}

#[derive(Clone, Debug)]
pub enum BranchRequirement {
    DreadLevel(u8),
    CompanionAlive(String),
    ChoiceMade(String),
    ItemPossessed(String),
    LocationVisited(String),
}

/// NPC dialogue and behavior
#[derive(Component, Clone, Debug)]
pub struct NPC {
    pub name: String,
    pub role: NPCRole,
    pub dialogue_set: String,
    pub fear_level: f32,
    pub trust_player: f32,
    pub knows_secrets: Vec<String>,
}

#[derive(Clone, Debug)]
pub enum NPCRole {
    Villager,
    Merchant,
    Guard,
    QuestGiver,
    Informant,
    Victim,      // Will die/disappear
    RedHerring,  // Misleads player
}