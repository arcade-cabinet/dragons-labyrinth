use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use std::fs;

// TODO: Integrate with actual stats/abilities/alignment resources when implemented
#[derive(Serialize, Deserialize)]
struct SaveData { /* fill when systems exist */ }

pub fn save_system(_keys: Res<ButtonInput<KeyCode>>) {
	// Placeholder no-op until stats/abilities/alignment resources are integrated
}

pub fn load_system(_commands: Commands) {
	// Placeholder no-op
}