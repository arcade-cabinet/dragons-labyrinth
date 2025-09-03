//! Seeds crate for Dragon's Labyrinth
//! 
//! This crate handles downloading of Seeds data sources:
//! - Project Gutenberg medieval/horror literature (via gutenberg-rs)
//! - Internet Archive public domain texts (via iars)
//! - Old Norse dictionary (via cleasby_vigfusson_dictionary)
//! 
//! Analysis happens in dl_analysis, generation in dl_processors

pub mod books;
pub mod dialogue;
pub mod linguistics;
pub mod build_api;

use anyhow::Result;
use std::path::Path;

/// Main Seeds data manager - downloads and caches source data
pub struct SeedsManager {
    pub books: books::BooksManager,
    pub linguistics: linguistics::LinguisticsManager,
    pub dialogue: dialogue::DialogueSourceManager,
}

impl SeedsManager {
    /// Initialize Seeds data by downloading and caching all sources
    pub fn initialize(cache_dir: &Path) -> Result<Self> {
        println!("Initializing Seeds data sources...");
        std::fs::create_dir_all(cache_dir)?;
        
        let books = books::BooksManager::initialize(&cache_dir.join("books"))?;
        let linguistics = linguistics::LinguisticsManager::initialize(&cache_dir.join("linguistics"))?;
        let dialogue = dialogue::DialogueSourceManager::initialize(&cache_dir.join("dialogue"))?;
        
        Ok(Self {
            books,
            linguistics,
            dialogue,
        })
    }
    
    /// Load Seeds data from existing cache
    pub fn load_from_cache(cache_dir: &Path) -> Result<Self> {
        let books = books::BooksManager::load_from_cache(&cache_dir.join("books"))?;
        let linguistics = linguistics::LinguisticsManager::load_from_cache(&cache_dir.join("linguistics"))?;
        let dialogue = dialogue::DialogueSourceManager::load_from_cache(&cache_dir.join("dialogue"))?;
        
        Ok(Self {
            books,
            linguistics,
            dialogue,
        })
    }
    
    /// Get path to a downloaded book for analysis
    pub fn get_book_path(&self, filename: &str) -> std::path::PathBuf {
        self.books.get_book_path(filename)
    }
    
    /// Get Old Norse dictionary entries for linguistic processing
    pub fn get_old_norse_dictionary(&self) -> &[cleasby_vigfusson_dictionary::DictionaryEntry] {
        &self.linguistics.old_norse_dictionary
    }
    
    /// Get character archetypes for NPC generation
    pub fn get_character_archetypes(&self) -> &[dialogue::CharacterArchetype] {
        &self.dialogue.character_archetypes
    }
}

/// Re-export key types for external usage
pub use books::{BooksManager, BookRecord};
pub use dialogue::{CharacterArchetype, DialogueSourceManager, QuestPattern, TraitTemplate};
pub use linguistics::{LanguageBlend, LinguisticsManager, NameContext};
