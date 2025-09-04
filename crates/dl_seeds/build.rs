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
use serde_json; // used for IA search response handling
use reqwest;    // blocking client used for IA advancedsearch
use urlencoding; // to safely encode lucene queries

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
    
    // Generate books.toml with rust-bert summaries (idempotent)
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

    #[derive(Debug, serde::Deserialize)]
    struct IaResponse { response: IaDocs }
    #[derive(Debug, serde::Deserialize)]
    struct IaDocs { docs: Vec<IaDoc> }
    #[derive(Debug, serde::Deserialize)]
    struct IaDoc {
        identifier: String,
        title: Option<String>,
        #[serde(default)]
        format: Vec<String>,
        #[serde(default)]
        language: Vec<String>,
        licenseurl: Option<String>,
        downloads: Option<u64>,
    }

    fn ia_search_keywords(query_keywords: &str, rows: usize) -> Result<Vec<IaDoc>, Box<dyn std::error::Error>> {
        // Advanced Search: constrain to texts + English; put keyword expr inside parentheses
        let lucene_q = format!(
            "mediatype:texts AND language:(eng) AND ({})",
            query_keywords
        );
        let fields = "identifier,title,format,language,licenseurl,downloads";
        let url = format!(
            "https://archive.org/advancedsearch.php?q={}&fl={}&sort[]=downloads+desc&rows={}&page=1&output=json",
            urlencoding::encode(&lucene_q),
            urlencoding::encode(fields),
            rows
        );
        let resp: serde_json::Value = reqwest::blocking::get(&url)?.json()?;
        // Deserialize into our structs; tolerate missing fields
        let docs: IaDocs = serde_json::from_value(resp.get("response").cloned().unwrap_or_default()
            .as_object()
            .map(|_| resp["response"].clone())
            .unwrap_or_else(|| serde_json::json!({"docs": []})))
            .unwrap_or(IaDocs { docs: vec![] });
        Ok(docs.docs)
    }

    fn looks_texty(formats: &[String]) -> bool {
        let needles = ["DjvuTXT", "TXT", "Text", "OCR Page Text"]; // common textual derivatives
        formats.iter().any(|f| needles.iter().any(|n| f.contains(n)))
    }


    fn download_text_from_identifier(identifier: &str) -> Result<(String, String), Box<dyn std::error::Error>> {
        let item = iars::Item::new(identifier)
            .map_err(|e| format!("Invalid IA identifier {}: {:?}", identifier, e))?;
        let files = item.list()
            .map_err(|e| format!("Failed to list files for {}: {:?}", identifier, e))?;
        // Inline selection of a good text derivative from the item file list
        let mut candidates: Vec<&str> = files.iter().map(|f| f.path.as_str()).collect();
        candidates.sort_by_key(|p| {
            if p.ends_with("_djvu.txt") { 0 }
            else if p.ends_with(".txt") && !p.ends_with("_scandata.txt") { 1 }
            else if p.ends_with(".hocr.html") { 2 }
            else { 9 }
        });
        let path = match candidates.into_iter().find(|p|
            p.ends_with("_djvu.txt") ||
            (p.ends_with(".txt") && !p.ends_with("_scandata.txt")) ||
            p.ends_with(".hocr.html")
        ) {
            Some(p) => p.to_string(),
            None => return Err(format!("No text-like files on item {}", identifier).into()),
        };
        let mut buf = Vec::new();
        item.download_file(&path, &mut buf)
            .map_err(|e| format!("Download failed for {} -> {}: {:?}", identifier, path, e))?;
        let text = String::from_utf8_lossy(&buf).to_string();
        if text.len() < 1000 {
            return Err(format!("Downloaded text too short for {} ({} bytes)", identifier, text.len()).into());
        }
        Ok((path, text))
    }

    // Initialize summarizer once
    let summarization_model = SummarizationModel::new(SummarizationConfig::default())?;
    let mut book_summaries: Vec<BookSummary> = Vec::new();

    for (band_key, keyword_expr) in BANDS_KEYWORDS {
        println!("cargo:warning=Searching Internet Archive for band '{}': {}", band_key, keyword_expr);
        let mut collected = 0usize;

        // Pull a generous pool, then filter locally
        let mut docs = ia_search_keywords(keyword_expr, 100)?;
        // Prefer higher download count first as proxy for quality
        docs.sort_by(|a, b| b.downloads.unwrap_or(0).cmp(&a.downloads.unwrap_or(0)));

        for doc in docs.into_iter().filter(|d| looks_texty(&d.format)) {
            if collected >= SAMPLES_PER_BAND { break; }
            let identifier = doc.identifier;
            match download_text_from_identifier(&identifier) {
                Ok((filename, content)) => {
                    // Summarize
                    let summaries = summarization_model.summarize(&[content.clone()])?;
                    let summary = summaries.first().cloned()
                        .ok_or_else(|| format!("CRITICAL: rust-bert failed to generate summary for {}", identifier))?;

                    let title = doc.title.unwrap_or_else(|| identifier.clone());
                    println!(
                        "cargo:warning=Band '{}' picked {} ({} chars -> {} chars) via {}",
                        band_key, title, content.len(), summary.len(), filename
                    );

                    book_summaries.push(BookSummary {
                        id: identifier,
                        title,
                        filename,
                        summary,
                        full_length: content.len(),
                    });
                    collected += 1;
                }
                Err(e) => {
                    println!("cargo:warning=Skipping {}: {}", identifier, e);
                    continue;
                }
            }
        }

        if collected == 0 {
            println!("cargo:warning=No downloadable texts found for band '{}' with query: {}", band_key, keyword_expr);
        }
    }

    // CRITICAL: Fail if we didn't get any book summaries
    if book_summaries.is_empty() {
        return Err("CRITICAL: Failed to download and summarize any Internet Archive texts for any band".into());
    }

    // Write TOML
    let books_container = BooksTomlContainer {
        books: book_summaries,
        generated_at: chrono::Utc::now().to_rfc3339(),
    };

    let toml_content = toml::to_string_pretty(&books_container)?;
    fs::write(output_path, toml_content)?;
    println!("cargo:warning=Generated books.toml with {} summaries ({} bands x {} each)",
             books_container.books.len(), BANDS_KEYWORDS.len(), SAMPLES_PER_BAND);
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

// Keyword-driven bands for thematic sampling from Internet Archive
const BANDS_KEYWORDS: &[(&str, &str)] = &[
    // Lv 1–20 · Peace → Unease
    ("peace_to_unease", "(pastoral OR idyll OR \"village life\" OR woodland OR \"snowy peaks\" OR tavern)"),
    // Lv 21–40 · Unease → Dread
    ("unease_to_dread", "(scorched OR blight OR desolation OR \"abandoned temple\" OR ruins OR cultist)"),
    // Lv 41–60 · Dread → Terror
    ("dread_to_terror", "(wasteland OR \"black sand\" OR \"lava field\" OR eldritch OR \"void crack\" OR fanatic)"),
    // Lv 61–120 · Terror → Despair → Madness
    ("terror_to_despair_madness", "(\"war camp\" OR raider OR betrayal OR \"social collapse\" OR execution OR \"defiled shrine\")"),
    // Lv 121–180 · Madness → Void
    ("madness_to_void", "(nightmare OR \"impossible geometry\" OR non-euclidean OR \"cosmic horror\" OR \"reality warping\")"),
];

const SAMPLES_PER_BAND: usize = 3; // how many texts to grab per band
