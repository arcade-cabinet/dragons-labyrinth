//! Text-Based Playthrough Mode for Dragon's Labyrinth
//! Allows testing the game flow without any graphical assets

use bevy::prelude::*;
use game_content_static::{
    characters::{CompanionArchetype, elena, marcus, quinn},
    philosophy::PhilosophyPath,
    dread::DreadLevel,
};
use std::io::{self, Write};

/// Playthrough state - tracks everything
#[derive(Resource)]
pub struct PlaythroughState {
    pub current_level: u32,
    pub companion: Option<CompanionArchetype>,
    pub companion_trust: f32,
    pub philosophy: PhilosophyState,
    pub inventory: Vec<String>,
    pub quests: Vec<QuestState>,
    pub death_count: u32,
    pub reputation: std::collections::HashMap<String, f32>,
    pub mount: Option<MountState>,
    pub dread_level: DreadLevel,
    pub choices_made: Vec<String>,
}

#[derive(Default)]
pub struct PhilosophyState {
    pub strength: f32,
    pub harmony: f32,
    pub light: f32,
    pub dark: f32,
}

impl PhilosophyState {
    pub fn dominant_path(&self) -> PhilosophyPath {
        let max = self.strength.max(self.harmony).max(self.light).max(self.dark);
        if self.strength == max { PhilosophyPath::Strength }
        else if self.harmony == max { PhilosophyPath::Harmony }
        else if self.light == max { PhilosophyPath::Light }
        else { PhilosophyPath::Dark }
    }
    
    pub fn crystal_color(&self) -> &str {
        match self.dominant_path() {
            PhilosophyPath::Strength => "🔴 Red",
            PhilosophyPath::Harmony => "🔵 Blue",
            PhilosophyPath::Light => "⚪ White",
            PhilosophyPath::Dark => "⚫ Black",
        }
    }
}

#[derive(Clone)]
pub struct QuestState {
    pub id: String,
    pub title: String,
    pub stage: u32,
    pub completed: bool,
}

#[derive(Clone)]
pub struct MountState {
    pub name: String,
    pub bond_level: f32,
    pub health: f32,
    pub corrupted: bool,
}

/// Text-based scene renderer
pub fn render_scene(state: &PlaythroughState) {
    println!("\n{}", "=".repeat(60));
    println!("LEVEL {} | Dread: {:?}", state.current_level, state.dread_level);
    
    if let Some(companion) = &state.companion {
        let trust_bar = "█".repeat((state.companion_trust * 10.0) as usize);
        println!("Companion: {:?} | Trust: [{}]", companion, trust_bar);
    }
    
    println!("Philosophy Crystal: {}", state.philosophy.crystal_color());
    
    if state.death_count > 0 {
        println!("Death Scars: {} ☠️", state.death_count);
    }
    
    if let Some(mount) = &state.mount {
        let bond_bar = "♥".repeat((mount.bond_level * 5.0) as usize);
        println!("Mount: {} {}", mount.name, bond_bar);
    }
    
    println!("{}", "=".repeat(60));
}

/// Choice presenter
pub fn present_choice(prompt: &str, options: &[(&str, fn(&mut PlaythroughState))]) -> usize {
    println!("\n{}", prompt);
    println!();
    
    for (i, (text, _)) in options.iter().enumerate() {
        println!("{}. {}", i + 1, text);
    }
    
    loop {
        print!("\nYour choice: ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        if let Ok(choice) = input.trim().parse::<usize>() {
            if choice > 0 && choice <= options.len() {
                return choice - 1;
            }
        }
        println!("Please enter a number between 1 and {}", options.len());
    }
}

/// LEVEL 1: The Door
pub fn level_1_the_door(state: &mut PlaythroughState) {
    println!("\n📖 You wake to your mother crying.");
    println!("   'The birds stopped singing. Just like when your father left.'");
    println!("   'He never came back. Please... don't be like him.'");
    println!();
    println!("Your childhood friend waits outside the door.");
    
    let choice = present_choice(
        "What do you say to them?",
        &[
            ("Come with me, I need a friend", |s| {
                s.companion = Some(CompanionArchetype::Elena);
                s.companion_trust = 1.0;
                s.philosophy.harmony += 0.2;
                println!("\n💬 Elena: 'Of course. We've been friends since childhood.'");
            }),
            ("Come with me, we'll split the glory", |s| {
                s.companion = Some(CompanionArchetype::Marcus);
                s.companion_trust = 1.0;
                s.philosophy.strength += 0.2;
                println!("\n💬 Marcus: 'Fifty-fifty! We'll be legends!'");
            }),
            ("Come with me, but follow my lead", |s| {
                s.companion = Some(CompanionArchetype::Quinn);
                s.companion_trust = 1.0;
                println!("\n💬 Quinn: 'As you command. I trust your judgment.'");
            }),
        ]
    );
    
    options[choice].1(state);
    state.choices_made.push(format!("companion_{:?}", state.companion));
    state.current_level = 2;
}

/// LEVEL 2-3: Wolf Encounters
pub fn level_2_3_wolves(state: &mut PlaythroughState) {
    println!("\n🌲 The path through the forest is dangerous...");
    
    // Starving Wolf
    println!("\n🐺 A starving wolf blocks your path. Its ribs are showing.");
    
    let companion_comment = match &state.companion {
        Some(CompanionArchetype::Elena) => "Elena: 'The poor creature...'",
        Some(CompanionArchetype::Marcus) => "Marcus: 'First blood!'",
        Some(CompanionArchetype::Quinn) => "Quinn: 'Your call.'",
        None => "",
    };
    
    if !companion_comment.is_empty() {
        println!("💬 {}", companion_comment);
    }
    
    let choice = present_choice(
        "How do you handle this?",
        &[
            ("Fight quickly and cleanly", |s| {
                s.philosophy.strength += 0.1;
                println!("\n⚔️ You dispatch the wolf swiftly.");
            }),
            ("Try to scare it away", |s| {
                s.philosophy.harmony += 0.1;
                println!("\n👐 You make yourself large and loud. The wolf flees.");
                if matches!(s.companion, Some(CompanionArchetype::Elena)) {
                    s.companion_trust += 0.1;
                    println!("💬 Elena: 'Sometimes mercy is the strongest choice.'");
                }
            }),
            ("Throw it some food", |s| {
                s.philosophy.harmony += 0.2;
                s.inventory.push("Less Food".to_string());
                println!("\n🍖 The wolf takes the food and leaves peacefully.");
            }),
        ]
    );
    
    options[choice].1(state);
    
    // Wolf Mother
    println!("\n🐺 A wolf stands in your path... wait, there are cubs behind her!");
    
    let choice = present_choice(
        "A mother protecting her young blocks the path.",
        &[
            ("Go around (takes longer)", |s| {
                s.philosophy.harmony += 0.2;
                println!("\n🚶 You take the long way around.");
                if matches!(s.companion, Some(CompanionArchetype::Elena)) {
                    s.companion_trust += 0.1;
                }
            }),
            ("Fight through", |s| {
                s.philosophy.strength += 0.1;
                s.philosophy.dark += 0.1;
                println!("\n⚔️ You fight the mother. The cubs scatter.");
                if matches!(s.companion, Some(CompanionArchetype::Elena)) {
                    s.companion_trust -= 0.2;
                    println!("💬 Elena: 'That was cruel!'");
                }
            }),
        ]
    );
    
    options[choice].1(state);
    state.current_level = 3;
}

/// Main playthrough runner
pub fn run_text_playthrough() {
    let mut state = PlaythroughState {
        current_level: 1,
        companion: None,
        companion_trust: 1.0,
        philosophy: PhilosophyState::default(),
        inventory: vec!["Rusty Sword".to_string(), "3 Gold".to_string()],
        quests: vec![],
        death_count: 0,
        reputation: std::collections::HashMap::new(),
        mount: None,
        dread_level: DreadLevel::Peace,
        choices_made: vec![],
    };
    
    println!("\n🐉 DRAGON'S LABYRINTH - Text Playthrough Mode 🐉");
    println!("Testing game flow without graphics...\n");
    
    loop {
        render_scene(&state);
        
        match state.current_level {
            1 => level_1_the_door(&mut state),
            2..=3 => level_2_3_wolves(&mut state),
            _ => {
                println!("\n🎮 Playthrough ends at Level {}.", state.current_level);
                println!("Choices made: {:?}", state.choices_made);
                break;
            }
        }
        
        // Check for death
        if state.death_count > 0 {
            println!("\n☠️ You died! Death scar added. Respawning...");
        }
    }
}

/// Bevy plugin for text mode
pub struct PlaythroughPlugin;

impl Plugin for PlaythroughPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, || {
            println!("Starting text-based playthrough mode...");
            run_text_playthrough();
        });
    }
}
