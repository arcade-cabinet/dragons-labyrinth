//! Save system and database management for Dragon's Labyrinth

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub mod database;
pub mod save_manager;
pub mod world_state;

pub use database::*;
pub use save_manager::*;
pub use world_state::*;

pub struct SaveSystemPlugin {
    pub save_directory: PathBuf,
    pub auto_save_interval: f32,
}

impl Default for SaveSystemPlugin {
    fn default() -> Self {
        let save_dir = directories::ProjectDirs::from("com", "dragons-labyrinth", "DragonLabyrinth")
            .map(|dirs| dirs.data_dir().to_path_buf())
            .unwrap_or_else(|| PathBuf::from("saves"));
            
        Self {
            save_directory: save_dir,
            auto_save_interval: 60.0, // Auto-save every minute
        }
    }
}

impl Plugin for SaveSystemPlugin {
    fn build(&self, app: &mut App) {
        // Ensure save directory exists
        std::fs::create_dir_all(&self.save_directory).ok();
        
        app.insert_resource(SaveConfig {
            save_directory: self.save_directory.clone(),
            auto_save_interval: self.auto_save_interval,
        })
        .init_resource::<SaveManager>()
        .init_resource::<AutoSaveTimer>()
        .add_event::<SaveGameEvent>()
        .add_event::<LoadGameEvent>()
        .add_event::<SaveCompleteEvent>()
        .add_event::<LoadCompleteEvent>()
        .add_systems(Update, (
            handle_save_events,
            handle_load_events,
            auto_save_system,
        ));
    }
}

#[derive(Resource)]
pub struct SaveConfig {
    pub save_directory: PathBuf,
    pub auto_save_interval: f32,
}

#[derive(Resource, Default)]
pub struct AutoSaveTimer {
    pub timer: Timer,
}

#[derive(Event)]
pub struct SaveGameEvent {
    pub slot: SaveSlot,
    pub description: Option<String>,
}

#[derive(Event)]
pub struct LoadGameEvent {
    pub slot: SaveSlot,
}

#[derive(Event)]
pub struct SaveCompleteEvent {
    pub slot: SaveSlot,
    pub success: bool,
    pub error: Option<String>,
}

#[derive(Event)]
pub struct LoadCompleteEvent {
    pub slot: SaveSlot,
    pub success: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SaveSlot {
    Auto,
    Quick,
    Manual(u32),
}

fn handle_save_events(
    mut save_events: EventReader<SaveGameEvent>,
    save_manager: Res<SaveManager>,
    mut complete_events: EventWriter<SaveCompleteEvent>,
) {
    for event in save_events.read() {
        let result = save_manager.save_game(&event.slot, event.description.as_deref());
        
        complete_events.send(SaveCompleteEvent {
            slot: event.slot.clone(),
            success: result.is_ok(),
            error: result.err().map(|e| e.to_string()),
        });
    }
}

fn handle_load_events(
    mut load_events: EventReader<LoadGameEvent>,
    save_manager: Res<SaveManager>,
    mut complete_events: EventWriter<LoadCompleteEvent>,
) {
    for event in load_events.read() {
        let result = save_manager.load_game(&event.slot);
        
        complete_events.send(LoadCompleteEvent {
            slot: event.slot.clone(),
            success: result.is_ok(),
            error: result.err().map(|e| e.to_string()),
        });
    }
}

fn auto_save_system(
    mut timer: ResMut<AutoSaveTimer>,
    time: Res<Time>,
    config: Res<SaveConfig>,
    mut save_events: EventWriter<SaveGameEvent>,
) {
    if timer.timer.duration().as_secs_f32() == 0.0 {
        timer.timer = Timer::from_seconds(config.auto_save_interval, TimerMode::Repeating);
    }
    
    timer.timer.tick(time.delta());
    
    if timer.timer.just_finished() {
        save_events.send(SaveGameEvent {
            slot: SaveSlot::Auto,
            description: Some("Auto-save".to_string()),
        });
    }
}