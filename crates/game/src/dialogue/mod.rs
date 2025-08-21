use bevy::prelude::*;
use bevy_yarnspinner::prelude::*;
use serde::{Deserialize, Serialize};

// Dialogue system using Yarn Spinner as per vision document
#[derive(Resource)]
pub struct DialogueState {
    pub current_conversation: Option<String>,
    pub companion_dialogue_states: std::collections::HashMap<String, CompanionDialogueState>,
    pub narrative_flags: std::collections::HashMap<String, bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CompanionDialogueState {
    pub companion_name: String,
    pub trauma_level: f32,
    pub current_mood: DialogueMood,
    pub available_topics: Vec<String>,
    pub recent_interactions: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DialogueMood {
    Cheerful,      // Peace stage
    Nervous,       // Unease stage  
    Fearful,       // Dread stage
    Traumatized,   // Terror stage
    Broken,        // Horror stage
    Hostile,       // If companion turns against player
}

impl Default for DialogueState {
    fn default() -> Self {
        Self {
            current_conversation: None,
            companion_dialogue_states: std::collections::HashMap::new(),
            narrative_flags: std::collections::HashMap::new(),
        }
    }
}

impl DialogueState {
    // Generate companion dialogue based on trauma and dread level
    pub fn get_companion_dialogue_node(&self, companion_name: &str, dread_level: u8) -> String {
        if let Some(state) = self.companion_dialogue_states.get(companion_name) {
            match companion_name {
                "einar" => self.generate_einar_dialogue(state, dread_level),
                "mira" => self.generate_mira_dialogue(state, dread_level),
                "sorin" => self.generate_sorin_dialogue(state, dread_level),
                "tamara" => self.generate_tamara_dialogue(state, dread_level),
                _ => format!("{}_generic_node", companion_name),
            }
        } else {
            format!("{}_intro_node", companion_name)
        }
    }
    
    // Update companion state based on recent events
    pub fn update_companion_state(&mut self, companion_name: String, trauma_delta: f32, interaction: String) {
        let state = self.companion_dialogue_states
            .entry(companion_name.clone())
            .or_insert_with(|| CompanionDialogueState {
                companion_name: companion_name.clone(),
                trauma_level: 0.0,
                current_mood: DialogueMood::Cheerful,
                available_topics: vec!["journey".to_string(), "village".to_string()],
                recent_interactions: Vec::new(),
            });
        
        state.trauma_level = (state.trauma_level + trauma_delta).clamp(0.0, 1.0);
        state.recent_interactions.push(interaction);
        
        // Keep only recent interactions
        if state.recent_interactions.len() > 5 {
            state.recent_interactions.remove(0);
        }
        
        // Update mood based on trauma level
        state.current_mood = match state.trauma_level {
            x if x < 0.2 => DialogueMood::Cheerful,
            x if x < 0.4 => DialogueMood::Nervous,
            x if x < 0.6 => DialogueMood::Fearful,
            x if x < 0.8 => DialogueMood::Traumatized,
            _ => DialogueMood::Broken,
        };
    }
    
    // Set narrative flags for story progression
    pub fn set_flag(&mut self, flag_name: String, value: bool) {
        self.narrative_flags.insert(flag_name, value);
        info!("Narrative flag set: {} = {}", 
              self.narrative_flags.keys().last().unwrap(), value);
    }
    
    // Check if narrative conditions are met for dialogue options
    pub fn check_conditions(&self, conditions: &[String]) -> bool {
        conditions.iter().all(|condition| {
            if condition.starts_with("flag:") {
                let flag_name = &condition[5..];
                self.narrative_flags.get(flag_name).unwrap_or(&false)
            } else if condition.starts_with("trauma:") {
                let parts: Vec<&str> = condition.split(':').collect();
                if parts.len() == 3 {
                    let companion = parts[1];
                    let threshold: f32 = parts[2].parse().unwrap_or(0.0);
                    if let Some(state) = self.companion_dialogue_states.get(companion) {
                        state.trauma_level >= threshold
                    } else {
                        false
                    }
                } else {
                    false
                }
            } else {
                true // Unknown condition defaults to true
            }
        })
    }
    
    // Generate companion-specific dialogue nodes
    fn generate_einar_dialogue(&self, state: &CompanionDialogueState, dread_level: u8) -> String {
        match (state.current_mood.clone(), dread_level) {
            (DialogueMood::Cheerful, 0) => "einar_cheerful_peace".to_string(),
            (DialogueMood::Nervous, 1) => "einar_protective_unease".to_string(),
            (DialogueMood::Fearful, 2) => "einar_questioning_dread".to_string(),
            (DialogueMood::Traumatized, 3) => "einar_panic_terror".to_string(),
            (DialogueMood::Broken, 4) => "einar_breakdown_horror".to_string(),
            _ => format!("einar_trauma_{:.1}_dread_{}", state.trauma_level, dread_level),
        }
    }
    
    fn generate_mira_dialogue(&self, state: &CompanionDialogueState, dread_level: u8) -> String {
        if dread_level >= 2 {
            "mira_goodbye_leaving".to_string() // Mira leaves at Dread stage
        } else {
            match state.current_mood {
                DialogueMood::Cheerful => "mira_optimistic_encouraging".to_string(),
                DialogueMood::Nervous => "mira_forced_positivity".to_string(),
                _ => "mira_final_moments".to_string(),
            }
        }
    }
    
    fn generate_sorin_dialogue(&self, state: &CompanionDialogueState, dread_level: u8) -> String {
        let loyalty_flag = self.narrative_flags.get("sorin_loyalty").unwrap_or(&true);
        
        match (state.current_mood.clone(), dread_level, loyalty_flag) {
            (_, 0..=1, _) => "sorin_academic_curiosity".to_string(),
            (_, 2, true) => "sorin_concerned_research".to_string(),
            (_, 2, false) => "sorin_dangerous_experiments".to_string(),
            (_, 3, true) => "sorin_loyal_understanding".to_string(),
            (_, 3, false) => "sorin_betrayal_revelation".to_string(),
            (DialogueMood::Hostile, 4, false) => "sorin_traitor_boss_fight".to_string(),
            (_, 4, true) => "sorin_final_ally".to_string(),
            _ => format!("sorin_complex_state_{}_{}", dread_level, loyalty_flag),
        }
    }
    
    fn generate_tamara_dialogue(&self, state: &CompanionDialogueState, dread_level: u8) -> String {
        match (state.trauma_level, dread_level) {
            (trauma, 0) if trauma < 0.1 => "tamara_innocent_wonder".to_string(),
            (trauma, 1) if trauma < 0.3 => "tamara_confused_helping".to_string(),
            (trauma, 2) if trauma < 0.5 => "tamara_first_fear".to_string(),
            (trauma, 3) if trauma < 0.7 => "tamara_selective_mutism".to_string(),
            (_, 4) => "tamara_broken_or_hope".to_string(),
            _ => format!("tamara_trauma_{:.1}_dread_{}", state.trauma_level, dread_level),
        }
    }
}

// Dialogue events for Yarn Spinner integration
#[derive(Event)]
pub struct StartDialogueEvent {
    pub node_name: String,
    pub companion_name: Option<String>,
}

#[derive(Event)]
pub struct DialogueChoiceEvent {
    pub choice_id: String,
    pub choice_text: String,
    pub consequences: Vec<String>,
}

#[derive(Event)]
pub struct DialogueCompleteEvent {
    pub node_name: String,
    pub flags_set: Vec<String>,
    pub trauma_changes: std::collections::HashMap<String, f32>,
}

// Custom dialogue view component for Cobweb UI integration
#[derive(Component)]
pub struct DialogueView {
    pub current_text: String,
    pub current_speaker: String,
    pub available_choices: Vec<DialogueChoice>,
    pub is_visible: bool,
}

#[derive(Clone, Debug)]
pub struct DialogueChoice {
    pub id: String,
    pub text: String,
    pub conditions: Vec<String>,
    pub consequences: Vec<String>,
}

impl Default for DialogueView {
    fn default() -> Self {
        Self {
            current_text: String::new(),
            current_speaker: String::new(),
            available_choices: Vec::new(),
            is_visible: false,
        }
    }
}

// Plugin for dialogue system
pub struct DialoguePlugin;

impl Plugin for DialoguePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(YarnSpinnerPlugin::new())
            .init_resource::<DialogueState>()
            .add_event::<StartDialogueEvent>()
            .add_event::<DialogueChoiceEvent>()
            .add_event::<DialogueCompleteEvent>()
            .add_systems(Startup, setup_dialogue_system)
            .add_systems(Update, (
                handle_dialogue_events,
                update_dialogue_view,
                process_yarn_events,
                check_companion_dialogue_triggers,
            ));
    }
}

fn setup_dialogue_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Load Yarn project with all dialogue files
    let yarn_project = asset_server.load("dialogue/main.yarn");
    
    commands.spawn((
        DialogueRunner::new(yarn_project),
        Name::new("DialogueRunner"),
    ));
    
    // Initialize dialogue view
    commands.spawn((
        DialogueView::default(),
        Name::new("DialogueView"),
    ));
    
    info!("Dialogue system initialized with Yarn Spinner");
}

fn handle_dialogue_events(
    mut dialogue_events: EventReader<StartDialogueEvent>,
    mut dialogue_state: ResMut<DialogueState>,
    mut dialogue_runner_query: Query<&mut DialogueRunner>,
) {
    for event in dialogue_events.read() {
        if let Ok(mut runner) = dialogue_runner_query.get_single_mut() {
            dialogue_state.current_conversation = Some(event.node_name.clone());
            
            info!("Starting dialogue: {}", event.node_name);
            runner.start_node(&event.node_name);
        }
    }
}

fn update_dialogue_view(
    mut dialogue_view_query: Query<&mut DialogueView>,
    dialogue_runner_query: Query<&DialogueRunner>,
    dialogue_state: Res<DialogueState>,
) {
    if let (Ok(mut view), Ok(runner)) = (
        dialogue_view_query.get_single_mut(),
        dialogue_runner_query.get_single()
    ) {
        view.is_visible = dialogue_state.current_conversation.is_some();
        
        if let Some(current_line) = runner.current_line() {
            view.current_text = current_line.text.clone();
            view.current_speaker = current_line.character_name.clone().unwrap_or_default();
        }
        
        // Update available choices based on narrative state
        if let Some(options) = runner.current_options() {
            view.available_choices = options.iter().map(|option| {
                DialogueChoice {
                    id: option.id.to_string(),
                    text: option.text.clone(),
                    conditions: Vec::new(), // Would be populated from Yarn metadata
                    consequences: Vec::new(),
                }
            }).collect();
        }
    }
}

fn process_yarn_events(
    mut yarn_events: EventReader<YarnEvent>,
    mut dialogue_state: ResMut<DialogueState>,
    mut choice_events: EventWriter<DialogueChoiceEvent>,
    mut complete_events: EventWriter<DialogueCompleteEvent>,
) {
    for event in yarn_events.read() {
        match event {
            YarnEvent::NodeStart { node_name } => {
                info!("Yarn node started: {}", node_name);
            },
            YarnEvent::NodeComplete { node_name } => {
                info!("Yarn node completed: {}", node_name);
                dialogue_state.current_conversation = None;
                
                complete_events.send(DialogueCompleteEvent {
                    node_name: node_name.clone(),
                    flags_set: Vec::new(),
                    trauma_changes: std::collections::HashMap::new(),
                });
            },
            YarnEvent::Command { command } => {
                // Process custom commands for narrative effects
                process_dialogue_command(command, &mut dialogue_state);
            },
            _ => {}
        }
    }
}

fn check_companion_dialogue_triggers(
    mut dialogue_events: EventWriter<StartDialogueEvent>,
    dialogue_state: Res<DialogueState>,
    dread_state: Res<crate::resources::DreadState>,
    companion_query: Query<&crate::components::Companion>,
    time: Res<Time>,
) {
    // Trigger contextual companion dialogue based on game state
    // This would check for specific narrative moments and companion states
    
    for companion in companion_query.iter() {
        if dialogue_state.current_conversation.is_none() {
            // Check if companion has something to say based on recent events
            let should_speak = match companion.companion_type {
                crate::components::CompanionType::Mira => {
                    dread_state.current_level == 2 && companion.current_state == crate::components::CompanionState::Normal
                },
                crate::components::CompanionType::Einar => {
                    companion.trauma_level > 0.7 && companion.current_state != crate::components::CompanionState::Broken
                },
                _ => false,
            };
            
            if should_speak {
                let node_name = dialogue_state.get_companion_dialogue_node(
                    &format!("{:?}", companion.companion_type).to_lowercase(),
                    dread_state.current_level
                );
                
                dialogue_events.send(StartDialogueEvent {
                    node_name,
                    companion_name: Some(format!("{:?}", companion.companion_type)),
                });
                break; // Only one companion speaks at a time
            }
        }
    }
}

fn process_dialogue_command(command: &str, dialogue_state: &mut DialogueState) {
    let parts: Vec<&str> = command.split_whitespace().collect();
    
    match parts.get(0) {
        Some(&"set_flag") => {
            if let (Some(&flag), Some(&value)) = (parts.get(1), parts.get(2)) {
                dialogue_state.set_flag(flag.to_string(), value == "true");
            }
        },
        Some(&"add_trauma") => {
            if let (Some(&companion), Some(&amount)) = (parts.get(1), parts.get(2)) {
                if let Ok(trauma_delta) = amount.parse::<f32>() {
                    dialogue_state.update_companion_state(
                        companion.to_string(),
                        trauma_delta,
                        "dialogue_trauma".to_string()
                    );
                }
            }
        },
        Some(&"trigger_event") => {
            if let Some(&event_name) = parts.get(1) {
                info!("Dialogue triggered event: {}", event_name);
                // Would trigger specific game events here
            }
        },
        _ => {
            info!("Unknown dialogue command: {}", command);
        }
    }
}