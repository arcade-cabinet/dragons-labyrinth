//! dl_seeds build script for idempotent TOML sampling
//! 
//! Creates 4 TOML files (regions.toml, settlements.toml, factions.toml, dungeons.toml)
//! each containing 5 randomly selected HTML entity samples from the HBF database.
//! 
//! This provides the seed data that dl_analysis will then process into structured JSON.

use std::env;
use std::fs;
use std::path::Path;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use rand::seq::SliceRandom;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

/// Sample HTML entity for TOML storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SampleEntity {
    pub uuid: String,
    pub entity_name: String,
    pub content: String,
}

/// TOML container for category samples
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategorySamples {
    pub category: String,
    pub sample_count: usize,
    pub entities: Vec<SampleEntity>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=game.hbf");
    
    let out_dir = env::var("OUT_DIR")?;
    let out_path = Path::new(&out_dir);
    
    // Check for HBF database
    let hbf_path = Path::new("game.hbf");
    if !hbf_path.exists() {
        return Err("game.hbf not found in dl_seeds directory".into());
    }
    
    // Categories and their known entities
    let categories = [
        ("regions", KNOWN_REGIONS),
        ("settlements", KNOWN_SETTLEMENTS),
        ("factions", KNOWN_FACTIONS),
        ("dungeons", KNOWN_DUNGEONS),
    ];
    
    // Connect to HBF database
    let conn = Connection::open(hbf_path)?;
    
    for (category, known_entities) in categories {
        let toml_path = out_path.join(format!("{}.toml", category));
        
        // Idempotent: only generate if TOML doesn't exist
        if !toml_path.exists() {
            println!("cargo:warning=Generating {} samples for {}", 5, category);
            generate_category_toml(&conn, category, known_entities, &toml_path)?;
        } else {
            println!("cargo:warning={}.toml already exists, skipping generation", category);
        }
    }
    
    // Generate books.toml with rust-bert summaries
    let books_toml_path = out_path.join("books.toml");
    if !books_toml_path.exists() {
        println!("cargo:warning=Generating books.toml with Internet Archive summaries");
        generate_books_toml_with_summaries(&books_toml_path)?;
    } else {
        println!("cargo:warning=books.toml already exists, skipping generation");
    }
    
    println!("cargo:warning=dl_seeds TOML sampling complete");
    Ok(())
}

/// Generate books.toml with Internet Archive downloads and rust-bert summaries
fn generate_books_toml_with_summaries(output_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    use rust_bert::pipelines::summarization::{SummarizationConfig, SummarizationModel};
    
    // Initialize rust-bert summarization model
    let summarization_model = SummarizationModel::new(SummarizationConfig::default())?;
    let mut book_summaries = Vec::new();
    
    for (archive_id, filename, title) in KNOWN_ARCHIVE_ITEMS {
        println!("cargo:warning=Downloading {} from Internet Archive...", title);
        
        // Download using iars Item API
        if let Ok(content) = download_archive_item_with_iars(archive_id) {
            // Use rust-bert to summarize (fail if summarization fails)
            let summaries = summarization_model.summarize(&[content.clone()])?;
            let summary = summaries.first().cloned()
                .ok_or_else(|| format!("CRITICAL: rust-bert failed to generate summary for {}", title))?;
            
            let summary_len = summary.len();
            book_summaries.push(BookSummary {
                id: archive_id.to_string(),
                title: title.to_string(),
                filename: filename.to_string(),
                summary,
                full_length: content.len(),
            });
            
            println!("cargo:warning=Summarized {} ({} chars -> {} chars)", 
                     title, content.len(), summary_len);
        } else {
            return Err(format!("CRITICAL: Failed to download {} from archive item {}", title, archive_id).into());
        }
    }
    
    // Create books TOML with summaries
    let books_container = BooksTomlContainer {
        books: book_summaries,
        generated_at: chrono::Utc::now().to_rfc3339(),
    };
    
    let toml_content = toml::to_string_pretty(&books_container)?;
    fs::write(output_path, toml_content)?;
    
    println!("cargo:warning=Generated books.toml with {} book summaries", books_container.books.len());
    Ok(())
}

/// Download content from Internet Archive item using iars
fn download_archive_item_with_iars(archive_id: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Create iars Item (returns Result in 0.2.0)
    let item = iars::Item::new(archive_id)
        .map_err(|e| format!("Invalid Internet Archive identifier {}: {:?}", archive_id, e))?;
    
    // Get list of files in the item
    let files = item.list().map_err(|e| format!("Failed to list files for {}: {:?}", archive_id, e))?;
    
    // Look for text files
    for file in &files {
        let filename = &file.path;
        if filename.ends_with(".txt") || filename.ends_with("_djvu.txt") {
            let mut content = Vec::new();
            
            match item.download_file(filename, &mut content) {
                Ok(_) => {
                    let text = String::from_utf8_lossy(&content).to_string();
                    if text.len() > 1000 { // Ensure we got substantial content
                        return Ok(text);
                    }
                }
                Err(_) => continue,
            }
        }
    }
    
    Err(format!("No suitable text files found in archive item: {}", archive_id).into())
}

/// Download text from Internet Archive using search keywords
fn download_from_archive(search_keywords: &str, output_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    use reqwest::blocking::get;
    
    // Use Internet Archive search API
    let search_url = format!(
        "https://archive.org/advancedsearch.php?q={}&fl=identifier,title&rows=5&output=json&mediatype=texts",
        urlencoding::encode(search_keywords)
    );
    
    let response = get(&search_url)?;
    let search_results: serde_json::Value = response.json()?;
    
    // Try to download the first few results until one succeeds
    if let Some(docs) = search_results["response"]["docs"].as_array() {
        for doc in docs.iter().take(3) {
            if let Some(identifier) = doc["identifier"].as_str() {
                if download_archive_item_text(identifier, output_path).is_ok() {
                    return Ok(());
                }
            }
        }
    }
    
    Err(format!("No downloadable texts found for: {}", search_keywords).into())
}

/// Download text content from specific Internet Archive item
fn download_archive_item_text(archive_id: &str, output_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    use reqwest::blocking::get;
    
    // Try common text file patterns for Archive items
    let text_patterns = [
        format!("{}_djvu.txt", archive_id),
        format!("{}.txt", archive_id),
        "text.txt".to_string(),
        format!("{}_text.pdf", archive_id), // Some archives have PDF text
    ];
    
    for pattern in &text_patterns {
        let download_url = format!("https://archive.org/download/{}/{}", archive_id, pattern);
        if let Ok(response) = get(&download_url) {
            if response.status().is_success() {
                let content = response.text()?;
                std::fs::write(output_path, content)?;
                return Ok(());
            }
        }
    }
    
    Err(format!("No text files found for Archive item: {}", archive_id).into())
}


fn strip_gutenberg_headers(content: &str) -> String {
    let lines: Vec<&str> = content.lines().collect();
    let mut start_idx = 0;
    let mut end_idx = lines.len();
    
    // Find start of actual content
    for (i, line) in lines.iter().enumerate() {
        if line.contains("*** START OF") || line.contains("CHAPTER") || line.contains("Chapter 1") {
            start_idx = i;
            break;
        }
    }
    
    // Find end before footer
    for (i, line) in lines.iter().enumerate().rev() {
        if line.contains("*** END OF") {
            end_idx = i;
            break;
        }
    }
    
    lines[start_idx..end_idx].join("\n")
}

/// Book excerpt for TOML storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookExcerpt {
    pub id: String,
    pub title: String,
    pub excerpt: String,
}

/// TOML container for book excerpts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BooksContainer {
    pub books: Vec<BookExcerpt>,
    pub excerpt_length: usize,
}

/// Book summary with rust-bert processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookSummary {
    pub id: String,
    pub title: String,
    pub filename: String,
    pub summary: String,
    pub full_length: usize,
}

/// TOML container for book summaries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BooksTomlContainer {
    pub books: Vec<BookSummary>,
    pub generated_at: String,
}

fn generate_category_toml(
    conn: &Connection,
    category: &str,
    known_entities: &[&str],
    output_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    // Randomly shuffle and select up to 5 samples
    let mut rng = ChaCha8Rng::seed_from_u64(42); // Fixed seed for deterministic sampling
    let mut shuffled = known_entities.to_vec();
    shuffled.shuffle(&mut rng);
    let selected = &shuffled[..std::cmp::min(5, shuffled.len())];
    
    let mut samples = Vec::new();
    
    for &entity_name in selected {
        // Query for HTML entities (NOT JSON) matching this entity name
        let query = "SELECT uuid, value FROM Entities WHERE value LIKE ? AND value NOT LIKE '{%'";
        let pattern = format!("%{}%", entity_name);
        
        let mut stmt = conn.prepare(query)?;
        let mut rows = stmt.query_map([&pattern], |row| {
            let uuid: String = row.get(0)?;
            let value: String = row.get(1)?;
            Ok((uuid, value))
        })?;
        
        // Take first HTML entity that matches
        if let Some(Ok((uuid, content))) = rows.next() {
            samples.push(SampleEntity {
                uuid,
                entity_name: entity_name.to_string(),
                content,
            });
        }
    }
    
    // Create TOML container
    let category_samples = CategorySamples {
        category: category.to_string(),
        sample_count: samples.len(),
        entities: samples,
    };
    
    // Write TOML file
    let toml_content = toml::to_string_pretty(&category_samples)?;
    fs::write(output_path, toml_content)?;
    
    println!("cargo:warning=Generated {} with {} samples", 
             output_path.display(), category_samples.sample_count);
    
    Ok(())
}

// Known entities (local constants)
const KNOWN_REGIONS: &[&str] = &[
    "Aurora Bushes", "Black Shield Timberlands", "Blood Blade Fields", "Bonecrusher Plains",
    "Darkfall Dunes", "Darkfall Plains", "Fallen Star Steppe", "Fearless Wilds", 
    "Firefly Cliffs", "Goblinchaser Jungle", "Goblinchaser Wilderness", "Goldenswan Timberlands",
    "Goldseeker's Cliffs", "Grey Mist Snowlands", "Heartseeker Forest", "Heartseeker Moors",
    "Hell's Gate Desert", "Holloweye Wilderness", "Iceborn Wilderness", "Javelin Plains",
    "Javelin Wetlands", "Moonwatcher Wetlands", "Nightmare Desert", "Ragthorn Meadows",
    "Ragthorn Woods", "Thunderwave Woodlands", "Vicious Crags",
];

const KNOWN_SETTLEMENTS: &[&str] = &[
    "Village of Ashamar", "Village of Balaal", "Town of Devilville",
    "Village of Dokar", "Village of Dorith", "Village of Harad",
    "Village of Headbone", "City of Headsmen", "Village of Kothian",
    "City of Palemoon",
];

const KNOWN_FACTIONS: &[&str] = &[
    "The Defiled Wolves", "The Fists Of Justice", "The Red Snakes",
    "The Swords Of Justice", "The White Wyverns",
];

const KNOWN_DUNGEONS: &[&str] = &[
    "Bowel of the Raging Pits", "Caverns of the Burning Souls",
    "Caverns of the Infernal Lich", "Crypt of the Corrupted Order",
    "Crypt of the Infernal Blades", "Crypt of the Mourning Goblin",
    "Crypt of the Unholy Goblin", "Crypt of the Violent Ogre",
    "Hideout of the Corrupted Order", "Hideout of the Unspoken Desire",
    "Lair of the Foresaken Desire", "Lair of the Mourning Hopes",
    "Shrine of the Infernal Blades", "Shrine of the Infernal Desire",
    "Temple of the Violent Ogre", "Tomb of the Cursed Pits",
    "Tomb of the Grey Ogre", "Tomb of the Unspoken Skeletons",
];

const KNOWN_ARCHIVE_ITEMS: &[(&str, &str, &str)] = &[
    ("lovecraftcollection", "lovecraft_collection.txt", "Complete Works of H.P. Lovecraft"),
    ("grimmsfairytal00grim", "grimms_fairy_tales.txt", "Grimm's Fairy Tales"),
    ("arthurianlegends00knowuoft", "arthurian_legends.txt", "Arthurian Legends"),
    ("norsemythology00gueruoft", "norse_mythology.txt", "Norse Mythology"),
    ("gothictales00various", "gothic_tales.txt", "Gothic Tales"),
    ("medievalbestiaries", "medieval_bestiaries.txt", "Medieval Bestiaries"),
    ("beowulfanglosaxo00unknuoft", "beowulf.txt", "Beowulf Anglo-Saxon Epic"),
    ("draculabramstok00stokuoft", "dracula.txt", "Dracula by Bram Stoker"),
];
