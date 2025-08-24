use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use hexroll_exporter::export::export_snapshot;
use hexroll_exporter::loader::{load_snapshot, HexrollPlugin};
use hexroll_exporter::loader::HexrollResource;
use bevy::prelude::*;

/// Command line interface for exporting Hexroll Backpack files and running a demo.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to the `.hbf` snapshot file
    #[arg(short, long)]
    input: PathBuf,

    /// Subcommand to execute
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Export the snapshot to a directory
    Export {
        /// Output directory for the exported files
        #[arg(short, long)]
        output: PathBuf,
    },
    /// Run a minimal Bevy demo that loads the snapshot and spawns hex tiles
    Demo,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Export { output } => {
            let snapshot = load_snapshot(&cli.input)?;
            export_snapshot(&snapshot, output)?;
            println!("Export completed successfully.");
            Ok(())
        }
        Commands::Demo => {
            // Launch a minimal Bevy app.  We disable all Bevy default plugins
            // except the bare minimum (winit + log) to keep the demo simple.
            App::new()
                .add_plugins(DefaultPlugins.set(WindowPlugin {
                    primary_window: Some(Window { title: "Hexroll Demo".into(), ..Default::default() }),
                    ..Default::default()
                }))
                .add_plugin(HexrollPlugin(cli.input.clone()))
                .add_system(|snapshot: Res<HexrollResource>| {
                    println!("Loaded {} tiles and {} entities", snapshot.0.map.tiles.len(), snapshot.0.entities.len());
                })
                .run();
            Ok(())
        }
    }
}