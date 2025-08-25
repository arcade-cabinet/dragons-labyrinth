
//! Yarn Spinner integration helpers.
//!
//! Strategy A (recommended): **build-time export**
//!   - Write Yarn scripts from the transformer into `assets/dialogue/*.yarn`
//!   - Add `YarnSpinnerPlugin::new()` to your Bevy game and it will compile all files.
//!
//! Strategy B (advanced): dynamically compile yarn strings into a YarnProject
//!   - Not all APIs are stable, so keep Strategy A as your baseline.

use std::fs;
use std::path::{Path, PathBuf};
use anyhow::Result;

pub fn write_npc_dialogue_assets(npc_name: &str, nodes: &[(String, String)], assets_dir: &Path) -> Result<PathBuf> {
    let dir = assets_dir.join("dialogue");
    fs::create_dir_all(&dir)?;
    let safe_npc = sanitize(npc_name);
    let mut last = None;
    for (node, yarn) in nodes {
        let file = dir.join(format!("{}__{}.yarn", safe_npc, sanitize(node)));
        fs::write(&file, yarn)?;
        last = Some(file);
    }
    Ok(last.unwrap_or(dir))
}

fn sanitize(s: &str) -> String {
    s.chars().map(|c| if c.is_ascii_alphanumeric() { c } else { '_' }).collect()
}

/// Minimal Bevy app showing how to run dialogues compiled from `assets/dialogue`.
/// Requires adding `bevy_yarnspinner` and (optionally) the example dialogue view.
///
/// ```no_run
/// use bevy::prelude::*;
/// use bevy_yarnspinner::prelude::*;
/// // Optional: a ready-made dialogue view
/// // use bevy_yarnspinner_example_dialogue_view::prelude::*;
///
/// fn main() {
///     App::new()
///         .add_plugins((
///             DefaultPlugins,
///             YarnSpinnerPlugin::new(), // compiles `assets/dialogue/*.yarn`
///             // ExampleYarnSpinnerDialogueViewPlugin::new(),
///         ))
///         .add_systems(Startup, setup_camera)
///         .add_systems(Update, spawn_dialogue_runner.run_if(resource_added::<YarnProject>))
///         .run();
/// }
///
/// fn setup_camera(mut commands: Commands) { commands.spawn(Camera2d::default()); }
///
/// fn spawn_dialogue_runner(mut commands: Commands, project: Res<YarnProject>) {
///     let mut runner = project.create_dialogue_runner(&mut commands);
///     runner.start_node("Start");
///     commands.spawn(runner);
/// }
/// ```
pub mod bevy_example {}
