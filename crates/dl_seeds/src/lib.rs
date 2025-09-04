//! Seeds crate for Dragon's Labyrinth
//! 
//! This crate handles downloading of Seeds data sources:
//! - Project Gutenberg medieval/horror literature (via reqwest)
//! - Internet Archive public domain texts (via iars)
//! - Old Norse dictionary (via cleasby_vigfusson_dictionary)
//! 
//! Analysis happens in dl_analysis, generation in dl_processors

pub mod books;
pub mod entities;
pub mod ai_client;
pub mod regions;
pub mod settlements;
pub mod dungeons;
pub mod factions;

use anyhow::Result;
use std::path::Path;

/// Main Seeds data manager - downloads and caches source data
pub struct SeedsManager {
    pub books: books::BooksManager,
}

impl SeedsManager {
    /// Generate all seeds from TOML samples using AI transformation
    pub fn generate_from_toml(out_dir: &Path) -> Result<Self> {
        println!("Generating Dragon's Labyrinth seeds from TOML samples...");
        
        let books = books::BooksManager::generate_seeds_from_texts(out_dir)?;
        
        Ok(Self {
            books,
        })
    }
    
    /// Get world seeds from literature
    pub fn get_world_seeds(&self) -> &[books::WorldSeed] {
        &self.books.world_seeds
    }
    
    /// Get quest seeds from literature  
    pub fn get_quest_seeds(&self) -> &[books::QuestSeed] {
        &self.books.quest_seeds
    }
    
    /// Get dialogue seeds from literature
    pub fn get_dialogue_seeds(&self) -> &[books::DialogueSeed] {
        &self.books.dialogue_seeds
    }
}

/// Re-export key types for external usage
pub use books::{BooksManager, BookRecord};
pub use entities::*;
pub use regions::*;
pub use settlements::*;
pub use dungeons::*;
pub use factions::*;
