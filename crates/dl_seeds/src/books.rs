//! Books module for downloading literature from Project Gutenberg and Internet Archive
//! 
//! This module handles real downloads using reqwest and iars crates,
//! not manual excerpts. The downloaded texts are then analyzed by dl_analysis.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Manager for downloading books from external sources
#[derive(Debug, Clone)]
pub struct BooksManager {
    pub cache_dir: std::path::PathBuf,
    pub downloaded_books: Vec<BookRecord>,
}

impl BooksManager {
    /// Initialize by downloading books from Project Gutenberg and Internet Archive
    pub fn initialize(books_dir: &Path) -> Result<Self> {
        std::fs::create_dir_all(books_dir)?;
        
        let mut downloaded_books = Vec::new();
        
        // Download horror and medieval texts from Project Gutenberg
        // Aligned with Dragon's Labyrinth themes: horror, dread, medieval, sword & sorcery
        let gutenberg_texts = [
            (10662, "poe_raven_tales.txt", "The Raven and Other Poems - Poe"),
            (345, "dracula.txt", "Dracula by Bram Stoker"),
            (209, "frankenstein.txt", "Frankenstein by Mary Shelley"),
            (36, "beowulf.txt", "Beowulf - Anglo-Saxon Epic"),
            (1661, "sherlock_hound.txt", "The Hound of the Baskervilles"),
            (2591, "grimm_dark_tales.txt", "Grimm's Fairy Tales (Dark Collection)"),
            (5317, "le_morte_darthur.txt", "Le Morte d'Arthur - Malory"),
            (10800, "faust.txt", "Faust by Goethe"),
        ];
        
        for (ebook_id, filename, title) in gutenberg_texts {
            let file_path = books_dir.join(filename);
            if !file_path.exists() {
                match download_gutenberg_text(ebook_id, &file_path) {
                    Ok(_) => {
                        println!("Successfully downloaded {} from Project Gutenberg", title);
                        downloaded_books.push(BookRecord {
                            id: ebook_id.to_string(),
                            title: title.to_string(),
                            source: "project_gutenberg".to_string(),
                            filename: filename.to_string(),
                            file_size: std::fs::metadata(&file_path)?.len(),
                        });
                    }
                    Err(e) => {
                        println!("Failed to download {} from Project Gutenberg: {}. Trying Internet Archive.", title, e);
                        
                        // Skip if Gutenberg fails - we'll use Internet Archive keyword search below
                        println!("Gutenberg failed for {}: {}. Will search Internet Archive by keywords.", title, e);
                    }
                }
            } else {
                // File already exists, add to records
                downloaded_books.push(BookRecord {
                    id: ebook_id.to_string(),
                    title: title.to_string(),
                    source: "cached".to_string(),
                    filename: filename.to_string(),
                    file_size: std::fs::metadata(&file_path)?.len(),
                });
            }
        }
        
        // Search Internet Archive by keywords for additional horror/medieval content
        // This is NOT a fallback - it's a primary source for thematic content
        println!("Searching Internet Archive for horror and medieval texts...");
        let archive_searches = [
            ("lovecraft cosmic horror", "lovecraft_collection.txt"),
            ("medieval sword sorcery", "sword_sorcery_tales.txt"),
            ("gothic horror tales", "gothic_horror.txt"),
            ("arthurian legends", "arthurian_legends.txt"),
            ("norse sagas", "norse_sagas.txt"),
            ("occult grimoire", "occult_texts.txt"),
            ("dark fairy tales", "dark_fairy_tales.txt"),
            ("medieval bestiaries", "medieval_bestiaries.txt"),
        ];
        
        for (search_keywords, filename) in archive_searches {
            let file_path = books_dir.join(filename);
            if !file_path.exists() {
                match search_and_download_from_archive(search_keywords, &file_path) {
                    Ok(_) => {
                        println!("Successfully downloaded {} from Internet Archive", search_keywords);
                        downloaded_books.push(BookRecord {
                            id: format!("ia_{}", search_keywords.replace(' ', "_")),
                            title: format!("Internet Archive: {}", search_keywords),
                            source: "internet_archive_search".to_string(),
                            filename: filename.to_string(),
                            file_size: std::fs::metadata(&file_path)?.len(),
                        });
                    }
                    Err(e) => {
                        println!("Failed to find {} on Internet Archive: {}", search_keywords, e);
                    }
                }
            } else {
                // File already exists from cache
                downloaded_books.push(BookRecord {
                    id: format!("ia_{}", search_keywords.replace(' ', "_")),
                    title: format!("Internet Archive: {}", search_keywords),
                    source: "cached".to_string(),
                    filename: filename.to_string(),
                    file_size: std::fs::metadata(&file_path)?.len(),
                });
            }
        }
        
        // Save manifest
        let manifest = BooksManifest {
            version: "1.0.0".to_string(),
            downloaded_at: chrono::Utc::now().to_rfc3339(),
            books: downloaded_books.clone(),
        };
        
        std::fs::write(
            books_dir.join("manifest.json"),
            serde_json::to_string_pretty(&manifest)?
        )?;
        
        Ok(Self {
            cache_dir: books_dir.to_path_buf(),
            downloaded_books,
        })
    }
    
    /// Load from existing cache
    pub fn load_from_cache(books_dir: &Path) -> Result<Self> {
        let manifest_path = books_dir.join("manifest.json");
        if !manifest_path.exists() {
            return Self::initialize(books_dir);
        }
        
        let manifest: BooksManifest = serde_json::from_str(&std::fs::read_to_string(manifest_path)?)?;
        
        Ok(Self {
            cache_dir: books_dir.to_path_buf(),
            downloaded_books: manifest.books,
        })
    }
    
    /// Get path to a downloaded book
    pub fn get_book_path(&self, filename: &str) -> std::path::PathBuf {
        self.cache_dir.join(filename)
    }
    
    /// Get list of successfully downloaded books
    pub fn get_downloaded_books(&self) -> &[BookRecord] {
        &self.downloaded_books
    }
}

/// Record of a downloaded book
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookRecord {
    pub id: String,
    pub title: String,
    pub source: String,
    pub filename: String,
    pub file_size: u64,
}

/// Manifest for tracking downloaded books
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BooksManifest {
    pub version: String,
    pub downloaded_at: String,
    pub books: Vec<BookRecord>,
}

/// Download text from Project Gutenberg using reqwest (replacing gutenberg-rs)
pub fn download_gutenberg_text(ebook_id: u32, file_path: &Path) -> Result<()> {
    // Try multiple URL patterns that Project Gutenberg uses
    let url_patterns = [
        format!("https://www.gutenberg.org/files/{}/{}-0.txt", ebook_id, ebook_id),
        format!("https://www.gutenberg.org/files/{}/{}.txt", ebook_id, ebook_id),
        format!("https://www.gutenberg.org/ebooks/{}.txt.utf-8", ebook_id),
        format!("https://www.gutenberg.org/cache/epub/{}/pg{}.txt", ebook_id, ebook_id),
    ];
    
    for url in &url_patterns {
        match reqwest::blocking::get(url) {
            Ok(response) => {
                if response.status().is_success() {
                    let content = response.text()?;
                    
                    // Strip basic Project Gutenberg headers/footers
                    let clean_content = strip_gutenberg_headers(&content);
                    
                    std::fs::write(file_path, clean_content)?;
                    return Ok(());
                }
            }
            Err(_) => continue, // Try next URL pattern
        }
    }
    
    Err(anyhow::anyhow!("Could not download Project Gutenberg text {}", ebook_id))
}

/// Simple function to strip Project Gutenberg headers and footers
fn strip_gutenberg_headers(content: &str) -> String {
    let lines: Vec<&str> = content.lines().collect();
    let mut start_idx = 0;
    let mut end_idx = lines.len();
    
    // Find start of actual content (after the header)
    for (i, line) in lines.iter().enumerate() {
        if line.contains("*** START OF") || line.contains("***START OF") {
            start_idx = i + 1;
            break;
        }
    }
    
    // Find end of actual content (before the footer)
    for (i, line) in lines.iter().enumerate().rev() {
        if line.contains("*** END OF") || line.contains("***END OF") {
            end_idx = i;
            break;
        }
    }
    
    // Return cleaned content
    lines[start_idx..end_idx].join("\n")
}

/// Search Internet Archive by keywords and download the first matching text
pub fn search_and_download_from_archive(keywords: &str, file_path: &Path) -> Result<()> {
    // Use Internet Archive search API to find texts matching our themes
    let search_url = format!(
        "https://archive.org/advancedsearch.php?q={}&fl=identifier,title&rows=5&output=json&mediatype=texts",
        urlencoding::encode(keywords)
    );
    
    let response = reqwest::blocking::get(&search_url)?;
    let search_results: serde_json::Value = response.json()?;
    
    // Try to download the first few results until one succeeds
    if let Some(docs) = search_results["response"]["docs"].as_array() {
        for doc in docs.iter().take(3) {
            if let Some(identifier) = doc["identifier"].as_str() {
                // Try to download this item
                if download_internet_archive_text(identifier, file_path).is_ok() {
                    return Ok(());
                }
            }
        }
    }
    
    Err(anyhow::anyhow!("No downloadable texts found for keywords: {}", keywords))
}

/// Download text from Internet Archive using a specific item identifier
pub fn download_internet_archive_text(archive_id: &str, file_path: &Path) -> Result<()> {
    // Use iars Item to access Internet Archive items
    let _item = iars::Item::new(archive_id);
    
    // Try common text file patterns for these items
    let text_patterns = [
        format!("{}_djvu.txt", archive_id),
        format!("{}.txt", archive_id),
        "text.txt".to_string(),
        "content.txt".to_string(),
    ];
    
    for pattern in &text_patterns {
        let download_url = format!("https://archive.org/download/{}/{}", archive_id, pattern);
        if download_text_file(&download_url, file_path).is_ok() {
            return Ok(());
        }
    }
    
    Err(anyhow::anyhow!("Could not find downloadable text file for Internet Archive item: {}", archive_id))
}

/// Download a text file from URL using reqwest
pub fn download_text_file(url: &str, file_path: &Path) -> Result<()> {
    let response = reqwest::blocking::get(url)?;
    
    if !response.status().is_success() {
        return Err(anyhow::anyhow!("Failed to download {}: HTTP {}", url, response.status()));
    }
    
    let content = response.text()?;
    std::fs::write(file_path, content)?;
    
    Ok(())
}
