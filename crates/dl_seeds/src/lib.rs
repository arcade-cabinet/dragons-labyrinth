//! Seeds crate for Dragon's Labyrinth
//! 
//! This crate handles downloading of Seeds data sources:
//! - Project Gutenberg medieval/horror literature (via reqwest)
//! - Internet Archive public domain texts (via iars)
//! - Old Norse dictionary (via cleasby_vigfusson_dictionary)
//! 
//! Analysis happens in dl_analysis, generation in dl_processors

pub mod books;

use anyhow::Result;
use std::path::Path;

/// Main Seeds data manager - downloads and caches source data
pub struct SeedsManager {
    pub books: books::BooksManager,
}

impl SeedsManager {
    /// Initialize Seeds data by downloading and caching all sources
    pub fn initialize(cache_dir: &Path) -> Result<Self> {
        println!("Initializing Seeds data sources...");
        std::fs::create_dir_all(cache_dir)?;
        
        let books = books::BooksManager::initialize(&cache_dir.join("books"))?;
        
        Ok(Self {
            books,
        })
    }
    
    /// Load Seeds data from existing cache
    pub fn load_from_cache(cache_dir: &Path) -> Result<Self> {
        let books = books::BooksManager::load_from_cache(&cache_dir.join("books"))?;
        
        Ok(Self {
            books,
        })
    }
    
    /// Get path to a downloaded book for analysis
    pub fn get_book_path(&self, filename: &str) -> std::path::PathBuf {
        self.books.get_book_path(filename)
    }
    
    /// Get list of successfully downloaded books
    pub fn get_downloaded_books(&self) -> &[books::BookRecord] {
        self.books.get_downloaded_books()
    }
}

/// Re-export key types for external usage
pub use books::{BooksManager, BookRecord};
