//! The ACTUAL world system we need - no external tools!

use bevy::prelude::*;
use hexx::{Hex, HexLayout, HexOrientation};
use std::collections::HashMap;

/// The entire world is just a hex map with progression
#[derive(Resource)]
pub struct WorldMap {
    pub hexes: HashMap<Hex, WorldHex>,
    pub progression: u32,  // 1-180
    pub current_hex: Hex,
    pub discovered: HashSet<Hex>,
}

#[derive(Component, Clone)]
pub struct WorldHex {
    pub terrain: TerrainType,
    pub location: Option<Location>,
    pub encounters: Vec<EncounterChance>,
    pub corruption: f32,
}

#[derive(Clone)]
pub enum TerrainType {
    Grassland,
    Forest,
    Mountain,
    Swamp,
    Village,
    Dungeon,
    VoidTouched,
}

#[derive(Clone)]
pub enum Location {
    Settlement {
        name: String,
        size: u32,  // Number of NPCs
        shops: Vec<String>,
    },
    DungeonEntrance {
        name: String,
        required_progression: u32,
        is_3d: bool,
    },
    Landmark {
        name: String,
        description: String,
    },
}

#[derive(Clone)]
pub struct EncounterChance {
    pub enemy_type: String,
    pub chance: f32,
    pub min_progression: u32,
    pub max_progression: u32,
}

/// Simple dialogue system - no YarnSpinner needed!
#[derive(Resource)]
pub struct DialogueDatabase {
    pub trees: HashMap<String, DialogueTree>,
}

#[derive(Clone)]
pub struct DialogueTree {
    pub nodes: HashMap<String, DialogueNode>,
    pub current: String,
}

#[derive(Clone)]
pub struct DialogueNode {
    pub speaker: String,
    pub text: String,
    pub emotion: String,  // For portrait changes
    pub choices: Vec<Choice>,
}

#[derive(Clone)]
pub struct Choice {
    pub text: String,
    pub next_node: String,
    pub conditions: Vec<Condition>,
    pub effects: Vec<Effect>,
}

#[derive(Clone)]
pub enum Condition {
    MinProgression(u32),
    HasCompanion(String),
    PhilosophyPath(String, f32),
    HasItem(String),
    DeathCount(u32),
}

#[derive(Clone)]
pub enum Effect {
    ChangePhilosophy(String, f32),
    ChangeTrust(f32),
    GainItem(String),
    SetFlag(String),
}

/// Movement on the hex map
pub fn hex_movement_system(
    mut player_query: Query<&mut Hex, With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
    mut world_map: ResMut<WorldMap>,
    mut progression: ResMut<Progression>,
) {
    if let Ok(mut player_hex) = player_query.get_single_mut() {
        let mut new_hex = *player_hex;
        
        // Simple hex movement
        if input.just_pressed(KeyCode::KeyQ) { new_hex = new_hex + Hex::new(-1, 0); }
        if input.just_pressed(KeyCode::KeyW) { new_hex = new_hex + Hex::new(0, -1); }
        if input.just_pressed(KeyCode::KeyE) { new_hex = new_hex + Hex::new(1, -1); }
        if input.just_pressed(KeyCode::KeyA) { new_hex = new_hex + Hex::new(-1, 1); }
        if input.just_pressed(KeyCode::KeyS) { new_hex = new_hex + Hex::new(0, 1); }
        if input.just_pressed(KeyCode::KeyD) { new_hex = new_hex + Hex::new(1, 0); }
        
        // Check if we can move there
        if let Some(hex_data) = world_map.hexes.get(&new_hex) {
            *player_hex = new_hex;
            world_map.current_hex = new_hex;
            world_map.discovered.insert(new_hex);
            
            // Progress the journey
            progression.current += 1;
            
            // Check for encounters
            check_encounters(&hex_data, progression.current);
            
            // Check for locations
            if let Some(location) = &hex_data.location {
                trigger_location_event(location);
            }
        }
    }
}

fn check_encounters(hex: &WorldHex, progression: u32) {
    for encounter in &hex.encounters {
        if progression >= encounter.min_progression && 
           progression <= encounter.max_progression {
            let roll = rand::random::<f32>();
            if roll < encounter.chance {
                // Spawn encounter
                println!("Encounter: {}", encounter.enemy_type);
            }
        }
    }
}

fn trigger_location_event(location: &Location) {
    match location {
        Location::Settlement { name, .. } => {
            println!("Entered settlement: {}", name);
        },
        Location::DungeonEntrance { name, is_3d, .. } => {
            println!("Found dungeon: {} (3D: {})", name, is_3d);
        },
        Location::Landmark { name, description } => {
            println!("Discovered: {} - {}", name, description);
        },
    }
}

/// The progression tracker - not "levels"!
#[derive(Resource)]
pub struct Progression {
    pub current: u32,  // 1-180
    pub milestones: Vec<Milestone>,
    pub flags: HashSet<String>,
}

#[derive(Clone)]
pub struct Milestone {
    pub at_progression: u32,
    pub event_id: String,
    pub description: String,
    pub trigger_dialogue: Option<String>,
}

/// Generate the world ONCE at startup
pub fn generate_world_map() -> WorldMap {
    let mut hexes = HashMap::new();
    
    // Starting area
    hexes.insert(Hex::new(0, 0), WorldHex {
        terrain: TerrainType::Village,
        location: Some(Location::Settlement {
            name: "Home Village".to_string(),
            size: 5,
            shops: vec!["Mother's House".to_string()],
        }),
        encounters: vec![],
        corruption: 0.0,
    });
    
    // Path to first village
    for i in 1..7 {
        hexes.insert(Hex::new(i, 0), WorldHex {
            terrain: TerrainType::Forest,
            location: None,
            encounters: vec![
                EncounterChance {
                    enemy_type: "Wolf".to_string(),
                    chance: 0.3,
                    min_progression: 1,
                    max_progression: 10,
                },
            ],
            corruption: 0.0,
        });
    }
    
    // First real village
    hexes.insert(Hex::new(7, 0), WorldHex {
        terrain: TerrainType::Village,
        location: Some(Location::Settlement {
            name: "Haven's Rest".to_string(),
            size: 20,
            shops: vec![
                "Inn".to_string(),
                "Blacksmith".to_string(),
                "General Store".to_string(),
            ],
        }),
        encounters: vec![],
        corruption: 0.0,
    });
    
    // Continue for all 180 progression points...
    // This would be generated by AI based on our design
    
    WorldMap {
        hexes,
        progression: 1,
        current_hex: Hex::new(0, 0),
        discovered: HashSet::from([Hex::new(0, 0)]),
    }
}

/// Simple UI - no Cobweb needed!
pub fn spawn_dialogue_ui(
    mut commands: Commands,
    dialogue: Res<CurrentDialogue>,
) {
    if let Some(node) = &dialogue.current_node {
        // Black box at bottom
        commands.spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Px(200.0),
                bottom: Val::Px(0.0),
                padding: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            background_color: Color::rgba(0.0, 0.0, 0.0, 0.9).into(),
            ..default()
        }).with_children(|parent| {
            // Speaker name
            parent.spawn(TextBundle::from_section(
                format!("{}: {}", node.speaker, node.text),
                TextStyle {
                    font_size: 24.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));
            
            // Choices
            for (i, choice) in node.choices.iter().enumerate() {
                parent.spawn(ButtonBundle {
                    style: Style {
                        margin: UiRect::top(Val::Px(10.0)),
                        padding: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    ..default()
                }).with_children(|button| {
                    button.spawn(TextBundle::from_section(
                        format!("{}. {}", i + 1, choice.text),
                        TextStyle {
                            font_size: 20.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                });
            }
        });
    }
}

#[derive(Resource, Default)]
pub struct CurrentDialogue {
    pub current_node: Option<DialogueNode>,
}

/// Plugin to tie it all together
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(generate_world_map())
            .insert_resource(Progression {
                current: 1,
                milestones: vec![],
                flags: HashSet::new(),
            })
            .insert_resource(CurrentDialogue::default())
            .add_systems(Update, (
                hex_movement_system,
                spawn_dialogue_ui,
            ));
    }
}
