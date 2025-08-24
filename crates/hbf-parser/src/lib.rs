//! Hexroll exporter library
//!
//! This crate provides two major pieces of functionality:
//!
//! * A loader for Hexroll Backpack (`.hbf`) files (`loader` module) which
//!   parses the internal SQLite database, extracts the map JSON and HTML
//!   entities and builds an in‑memory snapshot.
//! * An exporter (`export` module) that writes the contents of a snapshot
//!   into a directory structure on disk.  It produces a `map.json`,
//!   individual HTML files for every entity and a `refs.json` with search
//!   metadata.
//!
//! The library also re‑exports a Bevy plugin that loads a snapshot at
//! startup and spawns one entity per hex tile.  Developers can build
//! additional systems on top to handle encounters, dungeons, NPCs, etc.

pub mod loader;
pub mod export;

pub use loader::{HexTileComponent, HexrollPlugin, HexrollResource, HexrollSnapshot};