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
use rust_bert::pipelines::common::{ModelResource, ModelType};
use rust_bert::resources::{RemoteResource, LocalResource, ResourceProvider};
use rust_bert::t5::{T5ConfigResources, T5ModelResources, T5VocabResources};
use regex::Regex;
use rust_bert::pipelines::sentiment::SentimentModel;
use rust_bert::pipelines::zero_shot_classification::ZeroShotClassificationModel;
use tch::Device;
use rust_bert::pipelines::zero_shot_classification::ZeroShotClassificationConfig;
use cleasby_vigfusson_dictionary::{get_no_markup_dictionary, DictionaryEntry};
use std::collections::{BTreeMap, BTreeSet, HashMap};

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

    // Single artifact: generate world.toml (integrated) idempotently
    let world_toml_path = out_path.join("world.toml");
    if !world_toml_path.exists() {
        println!("cargo:warning=Generating world.toml (integrated)");
        generate_world_toml(&world_toml_path, out_path)?;
    } else {
        println!("cargo:warning=world.toml already exists, skipping generation");
    }

    println!("cargo:warning=dl_seeds TOML sampling complete");
    Ok(())
}
// Capitalize utility for names
fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() { Some(f) => f.to_uppercase().collect::<String>() + c.as_str(), None => String::new() }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NameEntry {
    pub name: String,
    pub kind: String,   // "person" | "place"
    pub region: String, // e.g., norse, celtic, semitic, sino, meso
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamesTomlContainer {
    pub generated_at: String,
    pub regions: std::collections::BTreeMap<String, Vec<NameEntry>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatureSeed {
    pub name: String,
    pub band: String,
    pub hints: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LandmarkSeed {
    pub name: String,
    pub band: String,
    pub kind: String, // e.g., temple, tower, crypt, forest
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BandNames {
    pub regions: BTreeMap<String, Vec<NameEntry>>, // region -> names
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamesPerBand {
    pub bands: BTreeMap<String, BandNames>, // band key -> BandNames
    pub generated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldTomlContainer {
    pub generated_at: String,
    pub books: BooksTomlContainer,
    pub grammar: GrammarTomlContainer,
    pub names: NamesPerBand,
    pub creatures: Vec<CreatureSeed>,
    pub landmarks: Vec<LandmarkSeed>,
}

/// Generate books.toml with Internet Archive downloads and rust-bert summaries
fn generate_books_toml_with_summaries(output_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // Keep libtorch threading predictable without env vars
    tch::set_num_threads(1);
    tch::set_num_interop_threads(1);
    println!("cargo:warning=libtorch threads set: intra-op=1 inter-op=1");
    use rust_bert::pipelines::summarization::{SummarizationConfig, SummarizationModel};

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

    fn is_disallowed_title(title: &str, identifier: &str) -> bool {
        let t = title.to_ascii_lowercase();
        let id = identifier.to_ascii_lowercase();
        let bad_terms = [
            "manga", "comic", "graphic novel", "z-lib", "zlib", "light novel",
            "rulebook", "core rulebook", "chaosium", "dungeons & dragons", "pathfinder",
            "home remedies", "recipes", "cookbook", "feng shui", "self-help",
            "workbook", "coloring book",
            "reply", "a reply", "criticism", "study guide", "lesson", "works of", "complete works", "collection", "essays", "magazine"
        ];
        let bad_ids = ["pdfy-", "manga_", "comic", "z-lib", "zlib"];
        bad_terms.iter().any(|w| t.contains(w)) || bad_ids.iter().any(|w| id.contains(w))
    }

    fn clean_ocr(text: &str) -> String {
        let mut out = String::with_capacity(text.len());
        for line in text.lines() {
            // skip lines with no letters (pure noise)
            if !line.chars().any(|c| c.is_alphabetic()) { continue; }
            let trimmed = line
                .replace('\u{00A0}', " ")
                .replace('\t', " ")
                .replace('\r', " ");
            let collapsed = trimmed.split_whitespace().collect::<Vec<_>>().join(" ");
            if collapsed.len() < 2 { continue; }
            out.push_str(&collapsed);
            out.push('\n');
        }
        out
    }

    fn strip_boilerplate_gutenberg(text: &str) -> &str {
        let lower = text.to_ascii_lowercase();
        let bytes = text.as_bytes();
        let mut start = 0usize;
        let mut end = bytes.len();

        // Prefer explicit START/END markers
        if let Some(pos) = lower.find("*** start of the project gutenberg ebook") {
            start = pos + 5; // move past the marker line
        } else if let Some(pos) = lower.find("start of this project gutenberg ebook") {
            start = pos + 5;
        } else if let Some(pos) = lower.find("project gutenberg") {
            // If we see Gutenberg early, push start forward a bit to skip header
            if pos < 10_000 { start = pos + 1_000; }
        }
        if let Some(pos) = lower.find("*** end of the project gutenberg ebook") {
            end = pos;
        } else if let Some(pos) = lower.find("end of the project gutenberg ebook") {
            end = pos;
        }
        let slice = &text[start.min(bytes.len())..end.min(bytes.len())];
        slice
    }

    fn extract_narrative_snippet(text: &str, target_chars: usize) -> String {
        // Remove obvious legal/distribution lines before scoring
        let legal_re = Regex::new(r"(?i)project\s+gutenberg|gutenberg\.org|www\.gutenberg|small\s+print|license|
                                    produced\s+by|e\s*-?text|etext|transcrib(er|ed)|\bebook\b").unwrap();
        let mut paragraphs: Vec<&str> = text.split("\n\n").collect();
        paragraphs.retain(|p| {
            let p_trim = p.trim();
            p_trim.len() > 60 && !legal_re.is_match(p_trim)
        });
        if paragraphs.is_empty() {
            return text.chars().take(target_chars).collect();
        }
        // Score paragraphs: prefer ones with letters, sentence punctuation, and quotes
        fn score(p: &str) -> f32 {
            let len = p.len() as f32;
            let letters = p.chars().filter(|c| c.is_alphabetic()).count() as f32;
            let quote = p.matches('"').count() as f32 + p.matches('“').count() as f32 + p.matches('’').count() as f32;
            let punct = p.matches('.').count() as f32 + p.matches('!').count() as f32 + p.matches('?').count() as f32;
            (letters / (len + 1.0)) * 0.6 + (quote.min(8.0) / 8.0) * 0.2 + (punct.min(12.0) / 12.0) * 0.2
        }
        let mut best_idx = 0usize;
        let mut best_score = -1.0f32;
        for (i, p) in paragraphs.iter().enumerate() {
            let s = score(p);
            if s > best_score { best_score = s; best_idx = i; }
        }
        // Assemble a window around best paragraph
        let mut out = String::new();
        let mut i = best_idx.saturating_sub(2);
        while i < paragraphs.len() && out.len() < target_chars {
            out.push_str(paragraphs[i].trim());
            out.push_str("\n\n");
            i += 1;
        }
        if out.is_empty() {
            return text.chars().take(target_chars).collect();
        }
        out
    }

    fn compose_summary(base_summary: &str, labels: &[(String, f64)], sentiment: &str, sent_score: f64) -> String {
        let mut out = String::new();
        out.push_str(base_summary.trim());
        if !labels.is_empty() {
            let tags = labels.iter()
                .take(5)
                .map(|(l, s)| format!("{} ({:.2})", l, s))
                .collect::<Vec<_>>()
                .join(", ");
            out.push_str("\n\nTags: ");
            out.push_str(&tags);
        }
        out.push_str(&format!("\nMood: {} ({:.2})", sentiment, sent_score));
        out
    }

    fn ia_search_keywords(query_keywords: &str, rows: usize) -> Result<Vec<IaDoc>, Box<dyn std::error::Error>> {
        // Multi-variant queries — stop at first non-empty hit set; bias public-domain seams and classics
        let negatives = "-collection:(comicbooks)"; // avoid comic scans
        let mut queries: Vec<String> = Vec::new();
        // Public-domain heavy seams first
        queries.push(format!("collection:(gutenberg) AND mediatype:texts AND language:(eng) AND ({}) {}", query_keywords, negatives));
        queries.push(format!("collection:(folkloreandmythology OR gutenberg) AND mediatype:texts AND language:(eng) AND ({}) {}", query_keywords, negatives));
        // Date-bounded classics (<= 1939)
        queries.push(format!("mediatype:texts AND language:(eng) AND date:[* TO 1939] AND ({}) {}", query_keywords, negatives));
        // Fallbacks
        queries.push(format!("mediatype:texts AND language:(eng) AND ({}) {}", query_keywords, negatives));
        queries.push(format!("mediatype:texts AND ({}) {}", query_keywords, negatives));

        let fields = "identifier,title,format,language,licenseurl,downloads,date,year,collection";
        let mut all_docs: Vec<IaDoc> = Vec::new();

        for (i, q) in queries.iter().enumerate() {
            let url = format!(
                "https://archive.org/advancedsearch.php?q={}&fl={}&sort[]=downloads+desc&rows={}&page=1&output=json",
                urlencoding::encode(q),
                urlencoding::encode(fields),
                rows.max(500)
            );
            let resp: serde_json::Value = match reqwest::blocking::get(&url) {
                Ok(r) => match r.json() { Ok(v) => v, Err(_) => continue },
                Err(_) => continue,
            };

            // Extract docs array defensively
            let docs_v = resp.get("response").and_then(|r| r.get("docs")).and_then(|d| d.as_array());
            let Some(docs_arr) = docs_v else { continue };
            if docs_arr.is_empty() { continue; }

            // Map Value -> IaDoc safely and filter obvious junk by title/id
            let mut mapped: Vec<IaDoc> = Vec::new();
            for e in docs_arr {
                let identifier = e.get("identifier").and_then(|x| x.as_str()).unwrap_or("").to_string();
                if identifier.is_empty() { continue; }
                let title = e.get("title").and_then(|x| x.as_str()).map(|s| s.to_string()).unwrap_or_else(|| identifier.clone());
                if is_disallowed_title(&title, &identifier) { continue; }
                let format = e.get("format").and_then(|x| x.as_array()).map(|arr| {
                    arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect::<Vec<String>>()
                }).unwrap_or_else(|| vec![]);
                let language = e.get("language").and_then(|x| x.as_array()).map(|arr| {
                    arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect::<Vec<String>>()
                }).unwrap_or_else(|| vec![]);
                let licenseurl = e.get("licenseurl").and_then(|x| x.as_str()).map(|s| s.to_string());
                let downloads = e.get("downloads").and_then(|x| x.as_u64());
                mapped.push(IaDoc { identifier, title: Some(title), format, language, licenseurl, downloads });
            }

            if !mapped.is_empty() {
                println!("cargo:warning=IA search variant {} returned {} filtered candidates", i + 1, mapped.len());
                all_docs = mapped;
                break;
            }
        }

        Ok(all_docs)
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
        // Stronger header/footer removal then OCR clean
        let core = strip_boilerplate_gutenberg(&text);
        let stripped = strip_gutenberg_headers(core);
        let cleaned = clean_ocr(&stripped);
        if cleaned.len() < 1500 {
            return Err(format!("Cleaned text too short/noisy for {}", identifier).into());
        }
        Ok((path, cleaned))
    }

    // Prepare a local cache under OUT_DIR for model files
    let cache_root = std::env::var("OUT_DIR").map(|p| Path::new(&p).join("rust_bert_cache")).unwrap_or_else(|_| Path::new("target").join("rust_bert_cache"));
    let _ = std::fs::create_dir_all(&cache_root);

    fn fetch_to_local(res: RemoteResource, cache_root: &Path) -> LocalResource {
        let path = res.get_local_path().expect("model fetch failed");
        let fname = path.file_name().unwrap_or_default();
        let dest = cache_root.join(fname);
        if dest != path {
            let _ = std::fs::copy(&path, &dest);
        }
        LocalResource { local_path: dest }
    }

    // Initialize summarizer once (explicit T5-small)
    let config_resource = fetch_to_local(RemoteResource::from_pretrained(T5ConfigResources::T5_SMALL), &cache_root);
    let vocab_resource = fetch_to_local(RemoteResource::from_pretrained(T5VocabResources::T5_SMALL), &cache_root);
    let weights_resource = fetch_to_local(RemoteResource::from_pretrained(T5ModelResources::T5_SMALL), &cache_root);

    let mut summarization_config = SummarizationConfig::new(
        ModelType::T5,
        ModelResource::Torch(Box::new(weights_resource)),
        config_resource,
        vocab_resource,
        None,
    );
    summarization_config.min_length = 64;
    summarization_config.max_length = Some(256);
    summarization_config.num_beams = 4;
    summarization_config.do_sample = false;
    summarization_config.device = Device::Cpu;
    let summarization_model = SummarizationModel::new(summarization_config)?;
    println!("cargo:warning=Summarizer: T5-SMALL on CPU (beam=4, max=256)");

    // Initialize optional classifiers to enrich summaries
    let sentiment_model = SentimentModel::new(Default::default())?; // ~255MB

    // Zero-shot classifier: use default BART-LARGE-MNLI (stable) but force CPU to avoid GPU surprises
    let mut zsc_config = ZeroShotClassificationConfig::default();
    zsc_config.device = Device::Cpu;
    let zsc_model = ZeroShotClassificationModel::new(zsc_config)?;
    let zsc_labels = [
        "gothic horror",
        "medieval romance",
        "folklore",
        "mythology",
        "witchcraft",
        "demonology",
        "cosmic horror",
        "weird fiction",
        "ghost story",
        "sword and sorcery",
    ];
    // Map labels to our five corruption bands
    let mut label_to_band: std::collections::HashMap<&str, &str> = std::collections::HashMap::new();
    label_to_band.insert("medieval romance", "peace_to_unease");
    label_to_band.insert("folklore", "peace_to_unease");
    label_to_band.insert("mythology", "peace_to_unease");
    label_to_band.insert("ghost story", "unease_to_dread");
    label_to_band.insert("witchcraft", "unease_to_dread");
    label_to_band.insert("weird fiction", "unease_to_dread");
    label_to_band.insert("gothic horror", "dread_to_terror");
    label_to_band.insert("demonology", "dread_to_terror");
    label_to_band.insert("battle & glory", "dread_to_terror");
    label_to_band.insert("curse & fate", "terror_to_despair_madness");
    label_to_band.insert("sea & voyage", "terror_to_despair_madness");
    label_to_band.insert("cosmic horror", "madness_to_void");
    println!(
        "cargo:warning=Zero-shot: BART-LARGE-MNLI (default) on CPU (labels: {})",
        zsc_labels.len()
    );

    let mut book_summaries: Vec<BookSummary> = Vec::new();

    for (band_key, keyword_expr) in BANDS_KEYWORDS {
        println!("cargo:warning=Searching Internet Archive for band '{}': {}", band_key, keyword_expr);
        let mut collected = 0usize;

        // Pull a generous pool, then filter locally
        let mut docs = ia_search_keywords(keyword_expr, 100)?;
        // Print a sample URL for debugging
        if docs.is_empty() {
            println!("cargo:warning=No IA results at all for band '{}' — consider adjusting keywords or collections", band_key);
        }
        // Prefer higher download count first as proxy for quality
        docs.sort_by(|a, b| b.downloads.unwrap_or(0).cmp(&a.downloads.unwrap_or(0)));

        for doc in docs.into_iter() {
            if collected >= SAMPLES_PER_BAND { break; }
            let identifier = doc.identifier;
            match download_text_from_identifier(&identifier) {
                Ok((filename, content)) => {
                    // Summarize cleaned content with narrative-focused extraction and T5 prefix
                    let max_chars = 8_000; // keep memory predictable in build.rs
                    let narrative = extract_narrative_snippet(&content, max_chars);
                    if narrative.len() < 600 {
                        println!("cargo:warning=Narrative snippet too short ({}), skipping {}", narrative.len(), identifier);
                        continue;
                    }
                    // T5 expects a summarization prefix for best results
                    let prefixed = format!("summarize: {}", narrative);
                    let summaries = summarization_model.summarize(&[prefixed.as_str()])?;
                    let base_summary = summaries.first().cloned()
                        .ok_or_else(|| format!("CRITICAL: rust-bert failed to generate summary for {}", identifier))?;

                    // Zero-shot multi-label classification on the summary
                    let zs = zsc_model.predict_multilabel(&[base_summary.as_str()], &zsc_labels, None, 64)?;
                    let mut label_scores: Vec<(String, f64)> = vec![];
                    if let Some(entry) = zs.get(0) {
                        for l in entry {
                            if l.score >= 0.30 { // threshold
                                label_scores.push((l.text.clone(), l.score));
                            }
                        }
                        label_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
                    }

                    // Sentiment on the summary
                    let sent = sentiment_model.predict(&[base_summary.as_str()]);
                    let (sentiment, sent_score) = if let Some(s) = sent.get(0) {
                        (format!("{:?}", s.polarity), s.score)
                    } else { ("Neutral".to_string(), 0.5) };

                    // Compose enriched summary text
                    let final_summary = compose_summary(&base_summary, &label_scores, &sentiment, sent_score);

                    let title = doc.title.unwrap_or_else(|| identifier.clone());
                    println!(
                        "cargo:warning=Band '{}' picked {} ({} chars -> {} chars) via {}",
                        band_key, title, content.len(), final_summary.len(), filename
                    );

                    book_summaries.push(BookSummary {
                        id: identifier,
                        title,
                        filename,
                        summary: final_summary,
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
    /// Primary band inferred from zero-shot labels (fallback to the collection's band)
    pub band: String,
    /// Top zero-shot labels with scores (for downstream seeds)
    pub labels: Vec<(String, f64)>,
    /// Keyphrases extracted from narrative/summary
    pub keywords: Vec<String>,
}

/// TOML container for book summaries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BooksTomlContainer {
    pub books: Vec<BookSummary>,
    pub generated_at: String,
}

/// Dictionary-driven grammar term for TOML storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrammarTerm {
    pub word: String,
    pub gloss: String,
    pub tags: Vec<String>,
}

/// TOML container for grammar
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrammarTomlContainer {
    pub generated_at: String,
    pub source: String,
    pub bands: std::collections::BTreeMap<String, Vec<GrammarTerm>>, // band key -> terms
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
// NOTE: We bias toward rich public-domain seams (Gothic, folklore, medieval, pulp/weird)
// and rely on item-level file listing to find actual text derivatives.
const BANDS_KEYWORDS: &[(&str, &str)] = &[
    // Broad classic fantasy / folklore vein
    (
        "peace_to_unease",
        "(subject:(fantasy OR \"fairy tales\" OR folklore OR mythology OR \"heroic romance\" OR \"romances, chivalry\") OR title:(fairy OR legend OR knight OR wizard OR dragon OR saga))"
    ),
    // Gothic / supernatural starters
    (
        "unease_to_dread",
        "(subject:(\"gothic fiction\" OR gothic OR \"ghost stories\" OR supernatural OR \"weird tales\") OR title:(ghost OR haunted OR vampire OR spectre OR specter OR eldritch))"
    ),
    // Horror / weird escalation
    (
        "dread_to_terror",
        "(subject:(horror OR \"weird fiction\" OR macabre) OR title:(horror OR nightmare OR demon OR daemon OR vampire OR \"strange tales\"))"
    ),
    // Dark medieval / occult lore
    (
        "terror_to_despair_madness",
        "(subject:(witchcraft OR demonology OR occult) OR title:(witch OR grimoire OR demonology OR sorcery OR necromancy))"
    ),
    // Cosmic / eldritch endgame
    (
        "madness_to_void",
        "(subject:(\"weird fiction\" OR cosmic OR \"cosmic horror\") OR title:(eldritch OR abyss OR void OR cthulhu OR unnameable OR nameless))"
    ),
];

const SAMPLES_PER_BAND: usize = 3; // how many texts to grab per band

fn generate_grammar_toml(output_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    use rust_bert::pipelines::zero_shot_classification::{ZeroShotClassificationConfig, ZeroShotClassificationModel};

    // Keep libtorch threading predictable in build scripts
    tch::set_num_threads(1);
    tch::set_num_interop_threads(1);

    // Load Old Norse dictionary (no markup)
    let entries: Vec<DictionaryEntry> = get_no_markup_dictionary()
        .map_err(|e| format!("Failed to load Cleasby & Vigfusson dictionary: {:?}", e))?;

    // Zero-shot classifier on CPU (default BART-large-MNLI)
    let mut zsc_config = ZeroShotClassificationConfig::default();
    zsc_config.device = Device::Cpu;
    let zsc_model = ZeroShotClassificationModel::new(zsc_config)?;

    // Labels reflect game themes; we will attach terms to bands by these labels
    let labels = [
        "gothic horror",
        "medieval romance",
        "folklore",
        "mythology",
        "witchcraft",
        "demonology",
        "cosmic horror",
        "weird fiction",
        "ghost story",
        "sword and sorcery",
        "sea & voyage",
        "battle & glory",
        "curse & fate",
    ];

    // Band mapping for labels
    use std::collections::{BTreeMap, HashMap};
    let mut band_map: HashMap<&str, &str> = HashMap::new();
    band_map.insert("medieval romance", "peace_to_unease");
    band_map.insert("folklore", "peace_to_unease");
    band_map.insert("mythology", "peace_to_unease");

    band_map.insert("ghost story", "unease_to_dread");
    band_map.insert("witchcraft", "unease_to_dread");
    band_map.insert("weird fiction", "unease_to_dread");

    band_map.insert("gothic horror", "dread_to_terror");
    band_map.insert("demonology", "dread_to_terror");
    band_map.insert("battle & glory", "dread_to_terror");

    band_map.insert("curse & fate", "terror_to_despair_madness");
    band_map.insert("sea & voyage", "terror_to_despair_madness");

    band_map.insert("cosmic horror", "madness_to_void");

    // Set up output bins per band
    let mut bands: BTreeMap<String, Vec<GrammarTerm>> = BTreeMap::new();
    for (key, _) in BANDS_KEYWORDS { bands.insert(key.to_string(), Vec::new()); }

    // Prefilter to reduce compute: look for definition keywords first
    let prefilter = Regex::new(r"(?i)\b(dragon|wyrm|serpent|giant|elf|dwarf|troll|witch|seer|seeress|saga|battle|sword|helm|shield|king|queen|thane|jar(l|l)|god|odin|thor|frey|sea|ship|voyage|curse|doom|fate|rune|magic|seidr|war|blood|night|ghost|haunt|spirit|demon|hell|void|abyss)\b").unwrap();

    // Limit total classified items to keep build time reasonable
    let mut kept = 0usize;
    let max_total = 2_000usize; // cap classification load

    for entry in entries.iter() {
        if kept >= max_total { break; }
        let word = entry.word.trim();
        if word.is_empty() || word.len() > 30 { continue; }
        // Skip obvious inflections (ends with '-' or contains spaces/punct heavy)
        if word.contains(' ') || word.contains('/') { continue; }

        let gloss = entry.definitions.get(0).map(|s| s.as_str()).unwrap_or("");
        if gloss.len() < 12 { continue; }
        if !prefilter.is_match(gloss) { continue; }

        // Zero-shot multilabel over the gloss
        let zs = zsc_model.predict_multilabel(&[gloss], &labels, None, 32)?;
        if let Some(first) = zs.get(0) {
            // Collect labels above threshold and push to band bins
            let mut tags: Vec<String> = Vec::new();
            for l in first {
                if l.score >= 0.45 { tags.push(l.text.clone()); }
            }
            if tags.is_empty() { continue; }

            // Attach to all mapped bands represented by tags
            let mut attached = false;
            for t in &tags {
                if let Some(band) = band_map.get(t.as_str()) {
                    bands.entry((*band).to_string()).or_default().push(GrammarTerm {
                        word: word.to_string(),
                        gloss: gloss.to_string(),
                        tags: tags.clone(),
                    });
                    attached = true;
                }
            }
            if attached { kept += 1; }
        }
    }

    // Post-process: de-duplicate terms per band and cap per-band size
    for (_band, terms) in bands.iter_mut() {
        terms.sort_by(|a, b| a.word.cmp(&b.word));
        terms.dedup_by(|a, b| a.word == b.word);
        if terms.len() > 200 { terms.truncate(200); }
    }

    // Build container and write TOML
    let container = GrammarTomlContainer {
        generated_at: chrono::Utc::now().to_rfc3339(),
        source: "cleasby_vigfusson_dictionary".to_string(),
        bands,
    };
    let toml_content = toml::to_string_pretty(&container)?;
    fs::write(output_path, toml_content)?;
    println!("cargo:warning=Generated grammar.toml with per-band Old Norse terms");
    Ok(())
}
fn generate_world_toml(world_path: &Path, out_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // Build books & grammar into temporary files under OUT_DIR, then compose World TOML
    let books_tmp = out_dir.join(".__books_tmp.toml");
    let grammar_tmp = out_dir.join(".__grammar_tmp.toml");

    // Reuse existing generators but write into hidden temp files
    generate_books_toml_with_summaries(&books_tmp)?;
    generate_grammar_toml(&grammar_tmp)?;

    // Load components
    let books_content = std::fs::read_to_string(&books_tmp)?;
    let books: BooksTomlContainer = toml::from_str(&books_content)?;

    let grammar_content = std::fs::read_to_string(&grammar_tmp)?;
    let grammar: GrammarTomlContainer = toml::from_str(&grammar_content)?;

    // Names per band (reuse grammar label bands to anchor Norse stems + synthetic regional names)
    let names = synthesize_names_per_band(&grammar)?;

    // Seeds from summaries (creatures + landmarks)
    let (creatures, landmarks) = extract_creatures_and_landmarks_from_books(&books)?;

    // Compose world
    let world = WorldTomlContainer {
        generated_at: chrono::Utc::now().to_rfc3339(),
        books,
        grammar,
        names,
        creatures,
        landmarks,
    };

    let toml_content = toml::to_string_pretty(&world)?;
    fs::write(world_path, toml_content)?;
    // Best-effort cleanup of temps
    let _ = std::fs::remove_file(books_tmp);
    let _ = std::fs::remove_file(grammar_tmp);

    println!("cargo:warning=Generated world.toml (integrated)");
    Ok(())
}

fn synthesize_names_per_band(grammar: &GrammarTomlContainer) -> Result<NamesPerBand, Box<dyn std::error::Error>> {
    use rand::{Rng, SeedableRng};
    use rand_chacha::ChaCha8Rng;

    let mut rng = ChaCha8Rng::seed_from_u64(0xD1A6_7B);

    // Helper phonotactic generator
    fn synth(count: usize, onset: &[&str], nucleus: &[&str], coda: &[&str], cap: bool, region: &str, kind: &str, rng: &mut ChaCha8Rng) -> Vec<NameEntry> {
        let mut out = Vec::new();
        for _ in 0..count {
            let s = format!("{}{}{}",
                onset[rng.gen_range(0..onset.len())],
                nucleus[rng.gen_range(0..nucleus.len())],
                coda[rng.gen_range(0..coda.len())]
            );
            out.push(NameEntry{ name: if cap { capitalize(&s) } else { s }, kind: kind.into(), region: region.into() });
        }
        out
    }

    // Regions: existing + expanded (welsh, slavic_old, akan, tamil)
    let celtic_on = ["ab","bal","bryn","caer","dal","dun","eog","fin","glen","kil","lan","loch","pen","strath","tre","llan","aber","gwyn","cwm","rhys","owen"];
    let celtic_v  = ["a","ai","e","ei","i","o","oi","u","y"];
    let celtic_cd = ["an","ach","aid","aidh","ael","wen","wyn","ydd","yth","more","bryn","gill","glen","og","yddin","fael"];

    let norse_prefix = ["as","bal","bjorn","ein","thor","ing","ulf","skj","sig","har","kar","gar","val","vid","yr"];
    let norse_suffix = ["gar","grim","vald","brand","mir","ric","ulf","stein","heim","fjord","borg","vik","gard","halla","svik","dr"];
    let mut mk_norse_name = |st: &str, rng: &mut ChaCha8Rng| -> String {
        match rng.gen_range(0..3) {
            0 => format!("{}{}", st, norse_suffix[rng.gen_range(0..norse_suffix.len())]),
            1 => format!("{}{}", norse_prefix[rng.gen_range(0..norse_prefix.len())], st),
            _ => format!("{}{}{}", norse_prefix[rng.gen_range(0..norse_prefix.len())], st, norse_suffix[rng.gen_range(0..norse_suffix.len())]),
        }
    };

    let sem_on = ["al","ibn","abd","sar","zar","qal","mar","ben","dar","sam","nah","ram","nur","hak","sul","amir","yas"];
    let sem_v  = ["a","aa","i","u","ou","ia","ei"];
    let sem_cd = ["im","ir","un","ar","el","al"," ibn"," as"," ashar"," amun"," aldin"," allah"];

    let sino_on = ["xi","li","shi","zh","ch","q","han","yuan","ming","song","tang","qin","gao","lin","bao","cheng","xun"];
    let sino_v  = ["a","e","i","o","u","ia","ian","iao","uo","ui","ong","ang","eng","uan","ian"];
    let sino_cd = ["ng","n","r","an","ang","ong","en","er","ian","iao","uan","un"];

    let meso_on = ["teo","itz","qui","xac","tol","az","mix","zap","mon","quet","tlal","cua","co","aco","teno"];
    let meso_v  = ["a","e","i","o","u","oa","ua","ia","ie","ui"];
    let meso_cd = ["tl","can","lan","pan","co","tlan","hua","pan","chan","co","cotl","n","nah"];

    // Welsh (subset)
    let wel_on = ["ll", "gw", "rh", "br", "cr", "dr", "gr", "pl", "pr", "tr", "pen", "caer", "aber", "llan"];
    let wel_v  = ["a","e","i","o","u","y","ae","ai","ei","oe","wy","yw"];
    let wel_cd = ["dd","ff","ll","n","r","rn","th","fryn","glyn","nant","wyn","wen","ydd","faen","mawr"];

    // Old Church Slavonic-ish (slavic_old)
    let slv_on = ["sv", "sl", "vl", "dr", "pr", "kr", "br", "vladi", "yar", "bog", "mir", "rad", "dobr", "gor", "vol"];
    let slv_v  = ["a","e","i","o","u","ya","yo","ye","iu","ia"];
    let slv_cd = ["mir","slav","grad","gorod","bor","pol","vin","gor","vetz","oslav","dan","dor","nik"];

    // Akan-ish
    let akan_on = ["ko", "kw", "ya", "yao", "kwan", "ofa", "aku", "ebo", "ka", "ab", "ad", "af", "ak", "kojo"];
    let akan_v  = ["a","e","i","o","u","oa","ia","aa","ee"];
    let akan_cd = ["na","ne","no","ba","ma","fo","pon","suo","kwa","mansa","man","bo","kra"];

    // Tamil-ish
    let tamil_on = ["ara","tha","sha","ra","ka","pa","na","vi","ma","sa","sur","kan","tam","ram","van"];
    let tamil_v  = ["a","aa","e","ee","i","ii","o","oo","u","uu","ai","au"];
    let tamil_cd = ["n","m","nd","nt","th","ppan","ram","ran","kan","kum","var","vel","puram","patti","nadu"];

    let mut bands_out: BTreeMap<String, BandNames> = BTreeMap::new();

    // Gather Norse stems from grammar per band
    for (band, terms) in &grammar.bands {
        let mut stems: Vec<String> = Vec::new();
        for t in terms {
            let w = t.word.replace(['ð','þ','á','é','í','ó','ú','ý','ö','æ','ø'], "");
            let w = w.replace(|c: char| !c.is_ascii_alphabetic(), "");
            if (3..=10).contains(&w.len()) { stems.push(w.to_lowercase()); }
        }
        stems.sort();
        stems.dedup();

        // Compose NORSE names for this band
        let mut norse_person: Vec<NameEntry> = Vec::new();
        let mut norse_place: Vec<NameEntry> = Vec::new();
        for st in stems.iter().take(60) {
            let p = mk_norse_name(st, &mut rng);
            norse_person.push(NameEntry { name: capitalize(&p), kind: "person".into(), region: "norse".into() });
        }
        for st in stems.iter().rev().take(40) {
            let v = format!("{}{}", st, ["heim","borg","vik","fjord","gard"][rng.gen_range(0..5)]);
            norse_place.push(NameEntry { name: capitalize(&v), kind: "place".into(), region: "norse".into() });
        }

        let (p_count, l_count) = (60usize, 30usize);

        // Per-band regional weights (sum doesn't need to be 1; will be normalized)
        let weights: HashMap<&str, f32> = match band.as_str() {
            "peace_to_unease" => HashMap::from([
                ("celtic", 3.0),("welsh", 2.5),("norse", 2.0),("semitic", 1.0),("sino", 1.0),("meso", 1.0),("slavic_old",1.0),("akan", 1.0),("tamil",1.0)
            ]),
            "unease_to_dread" => HashMap::from([
                ("celtic", 2.0),("norse", 2.0),("slavic_old", 2.0),("welsh", 1.5),("semitic", 1.2),("sino", 1.0),("meso",1.0),("akan",1.0),("tamil",1.0)
            ]),
            "dread_to_terror" => HashMap::from([
                ("slavic_old", 3.0),("norse", 2.5),("meso", 1.5),("semitic", 1.2),("celtic",1.0),("sino",1.0),("welsh",1.0),("akan",1.0),("tamil",1.0)
            ]),
            "terror_to_despair_madness" => HashMap::from([
                ("semitic", 2.5),("tamil", 2.0),("meso", 2.0),("slavic_old",1.5),("norse",1.2),("sino",1.0),("celtic",1.0),("welsh",1.0),("akan",1.0)
            ]),
            _ /* madness_to_void */ => HashMap::from([
                ("slavic_old", 2.8),("sino", 2.2),("norse", 2.0),("meso",1.5),("semitic",1.2),("tamil",1.2),("celtic",0.8),("welsh",0.8),("akan",0.8)
            ]),
        };

        let sum_w: f32 = weights.values().sum();
        let mut regions: BTreeMap<String, Vec<NameEntry>> = BTreeMap::new();
        let mut emit_region = |region: &str, w: f32| {
            let pf = ((p_count as f32) * (w / sum_w)).max(2.0) as usize;
            let lf = ((l_count as f32) * (w / sum_w)).max(1.0) as usize;
            let v = match region {
                "norse" => {
                    let mut v=Vec::new();
                    // already computed norse_person/place above
                    v.extend(norse_person.iter().cloned().take(pf));
                    v.extend(norse_place.iter().cloned().take(lf));
                    v
                },
                "celtic" => { let mut v=Vec::new(); v.extend(synth(pf,&celtic_on,&celtic_v,&celtic_cd,true,"celtic","person",&mut rng)); v.extend(synth(lf,&celtic_on,&celtic_v,&celtic_cd,true,"celtic","place",&mut rng)); v },
                "semitic"=> { let mut v=Vec::new(); v.extend(synth(pf,&sem_on,&sem_v,&sem_cd,true,"semitic","person",&mut rng)); v.extend(synth(lf,&sem_on,&sem_v,&sem_cd,true,"semitic","place",&mut rng)); v },
                "sino"   => { let mut v=Vec::new(); v.extend(synth(pf,&sino_on,&sino_v,&sino_cd,true,"sino","person",&mut rng)); v.extend(synth(lf,&sino_on,&sino_v,&sino_cd,true,"sino","place",&mut rng)); v },
                "meso"   => { let mut v=Vec::new(); v.extend(synth(pf,&meso_on,&meso_v,&meso_cd,true,"meso","person",&mut rng)); v.extend(synth(lf,&meso_on,&meso_v,&meso_cd,true,"meso","place",&mut rng)); v },
                "welsh"  => { let mut v=Vec::new(); v.extend(synth(pf,&wel_on,&wel_v,&wel_cd,true,"welsh","person",&mut rng)); v.extend(synth(lf,&wel_on,&wel_v,&wel_cd,true,"welsh","place",&mut rng)); v },
                "slavic_old"=>{ let mut v=Vec::new(); v.extend(synth(pf,&slv_on,&slv_v,&slv_cd,true,"slavic_old","person",&mut rng)); v.extend(synth(lf,&slv_on,&slv_v,&slv_cd,true,"slavic_old","place",&mut rng)); v },
                "akan"   => { let mut v=Vec::new(); v.extend(synth(pf,&akan_on,&akan_v,&akan_cd,true,"akan","person",&mut rng)); v.extend(synth(lf,&akan_on,&akan_v,&akan_cd,true,"akan","place",&mut rng)); v },
                "tamil"  => { let mut v=Vec::new(); v.extend(synth(pf,&tamil_on,&tamil_v,&tamil_cd,true,"tamil","person",&mut rng)); v.extend(synth(lf,&tamil_on,&tamil_v,&tamil_cd,true,"tamil","place",&mut rng)); v },
                _ => Vec::new(),
            };
            if !v.is_empty() { regions.insert(region.to_string(), v); }
        };
        for (r, w) in &weights { emit_region(r, *w); }

        bands_out.insert(band.clone(), BandNames { regions });
    }
// Very small RAKE-like keyphrase extractor
fn rake_like_keyphrases(text: &str, top_k: usize) -> Vec<String> {
    // very small RAKE-ish: split on stopwords & punctuation, score by word freq * degree
    use std::collections::HashMap;
    let stop: BTreeSet<&'static str> = [
        "the","a","an","and","or","but","if","then","so","because","as","of","to","in","on","for","with","by","from","at","into","over","after","before","about","between","through","during","without","within","against","among","across","per","is","are","was","were","be","been","being","it","its","this","that","these","those","he","she","they","them","his","her","their","we","you","i","my","our"
    ].into_iter().collect();

    let mut phrases: Vec<Vec<String>> = Vec::new();
    let mut current: Vec<String> = Vec::new();
    let mut push_current = |phrases: &mut Vec<Vec<String>>, current: &mut Vec<String>| {
        if current.len() >= 1 && current.iter().any(|w| w.chars().any(|c| c.is_alphabetic())) {
            phrases.push(current.clone());
        }
        current.clear();
    };

    for raw in text.split(|c: char| !c.is_alphanumeric() && c != '\'' && c != '-') {
        if raw.is_empty() { continue; }
        let w = raw.to_ascii_lowercase();
        if stop.contains(w.as_str()) { push_current(&mut phrases, &mut current); } else { current.push(w); }
    }
    push_current(&mut phrases, &mut current);

    let mut freq: HashMap<String, usize> = HashMap::new();
    let mut degree: HashMap<String, usize> = HashMap::new();
    for p in &phrases {
        let deg = p.len().saturating_sub(1);
        for w in p {
            *freq.entry(w.clone()).or_default() += 1;
            *degree.entry(w.clone()).or_default() += deg;
        }
    }
    let mut scored: Vec<(String, f32)> = phrases.into_iter().map(|p| {
        let mut s = 0f32;
        for w in &p {
            let f = *freq.get(w).unwrap_or(&1) as f32;
            let d = (*degree.get(w).unwrap_or(&0) + 1) as f32;
            s += d / f;
        }
        (p.join(" "), s)
    }).collect();
    scored.sort_by(|a,b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    scored.into_iter().map(|(p,_)| p).take(top_k).collect()
}

    Ok(NamesPerBand { bands: bands_out, generated_at: chrono::Utc::now().to_rfc3339() })
}

fn extract_creatures_and_landmarks_from_books(books: &BooksTomlContainer) -> Result<(Vec<CreatureSeed>, Vec<LandmarkSeed>), Box<dyn std::error::Error>> {
    // Map zero-shot tags (embedded in summary text) to bands
    let mut band_map: HashMap<&str, &str> = HashMap::new();
    band_map.insert("medieval romance", "peace_to_unease");
    band_map.insert("folklore", "peace_to_unease");
    band_map.insert("mythology", "peace_to_unease");
    band_map.insert("ghost story", "unease_to_dread");
    band_map.insert("witchcraft", "unease_to_dread");
    band_map.insert("weird fiction", "unease_to_dread");
    band_map.insert("gothic horror", "dread_to_terror");
    band_map.insert("demonology", "dread_to_terror");
    band_map.insert("battle & glory", "dread_to_terror");
    band_map.insert("curse & fate", "terror_to_despair_madness");
    band_map.insert("sea & voyage", "terror_to_despair_madness");
    band_map.insert("cosmic horror", "madness_to_void");

    // Simple lexicons
    let creature_lex = [
        "dragon","wyrm","serpent","demon","daemon","devil","imp","goblin","orc","ogre","troll","giant",
        "wraith","ghost","specter","spectre","vampire","ghoul","lich","harpy","hydra","wolf","werewolf",
        "spider","kraken","leviathan","basilisk","manticore","chimera","gargoyle","hag","witch","warlock",
    ];
    let landmark_kinds: &[(&str, &[&str])] = &[
        ("temple", &["temple","shrine","sanctum","abbey"]),
        ("fortress", &["fortress","stronghold","citadel","castle","keep","bastion","redoubt","tower"]),
        ("crypt", &["crypt","tomb","sepulchre","catacomb","barrow"]),
        ("cave", &["cave","cavern","grotto","hollow"]),
        ("ruin", &["ruin","ruins"]),
        ("forest", &["forest","wood","grove","wold","copse"]),
        ("desert", &["desert","wasteland","dune","dunes"]),
        ("mountain", &["mountain","peak","crag"]),
        ("swamp", &["swamp","bog","fen","marsh"]),
        ("abyss", &["abyss","chasm","rift","fissure","void"]),
        ("labyrinth", &["labyrinth","maze"]),
    ];

    let mut creatures: Vec<CreatureSeed> = Vec::new();
    let mut landmarks: Vec<LandmarkSeed> = Vec::new();

    for b in &books.books {
        // Determine band from embedded "Tags: ..." line if present
        let mut band = "unease_to_dread".to_string(); // default
        if let Some(idx) = b.summary.find("Tags:") {
            let tags_line = &b.summary[idx..].lines().next().unwrap_or("");
            for (lbl, band_key) in &band_map {
                if tags_line.to_ascii_lowercase().contains(&lbl.to_ascii_lowercase()) {
                    band = (*band_key).to_string();
                    break;
                }
            }
        }

        let text_l = b.summary.to_ascii_lowercase();

        // Keyphrases from summary to seed names (best-effort)
        let phrases = rake_like_keyphrases(&b.summary, 20);

        // Creatures
        let mut seen: BTreeSet<String> = BTreeSet::new();
        for w in creature_lex {
            if text_l.contains(w) { seen.insert(w.to_string()); }
        }
        for c in seen {
            creatures.push(CreatureSeed { name: capitalize(&c), band: band.clone(), hints: vec![b.title.clone()] });
        }

        // Landmarks: look for lexicon matches and augment with keyphrases ending in landmark terms
        let mut pushed_kind = BTreeSet::new();
        for (kind, variants) in landmark_kinds {
            let mut added = false;
            // 1) direct lexicon hits
            for v in *variants {
                if let Some(idx) = text_l.find(v) {
                    let prefix = &b.summary[..idx.min(b.summary.len())];
                    let name = prefix.split_whitespace().rev().take(3)
                        .filter(|tok| tok.chars().next().map(|c| c.is_uppercase()).unwrap_or(false))
                        .collect::<Vec<&str>>()
                        .into_iter().rev().collect::<Vec<&str>>().join(" ");
                    let label = if name.is_empty() { capitalize(v) } else { format!("{} {}", name, v) };
                    landmarks.push(LandmarkSeed { name: label, band: band.clone(), kind: (*kind).to_string() });
                    added = true;
                    break;
                }
            }
            // 2) keyphrase-derived (e.g., "Ivory Spire", "Abbey of Night")
            if !added {
                for ph in &phrases {
                    let pl = ph.to_ascii_lowercase();
                    if variants.iter().any(|v| pl.ends_with(v)) {
                        landmarks.push(LandmarkSeed { name: capitalize(ph), band: band.clone(), kind: (*kind).to_string() });
                        added = true; break;
                    }
                }
            }
            if added { pushed_kind.insert(kind.to_string()); }
        }

        // Keyphrase-based creature guesses: single or hyphenated nouns that look like beings
        for ph in &phrases {
            let pl = ph.to_ascii_lowercase();
            if pl.split_whitespace().count() <= 3 && (pl.ends_with("lord") || pl.ends_with("beast") || pl.ends_with("fiend") || pl.ends_with("spirit") || pl.ends_with("witch") || pl.ends_with("demon") || pl.ends_with("wyrm") ) {
                let name = ph.split_whitespace().map(|t| capitalize(t)).collect::<Vec<_>>().join(" ");
                creatures.push(CreatureSeed { name, band: band.clone(), hints: vec![b.title.clone()] });
            }
        }
    }

    // De-dup (case-insensitive) and trim empties
    creatures.retain(|c| !c.name.trim().is_empty());
    landmarks.retain(|l| !l.name.trim().is_empty());

    let mut seen_c = BTreeSet::new();
    creatures.retain(|c| seen_c.insert((c.name.to_ascii_lowercase(), c.band.clone())));
    let mut seen_l = BTreeSet::new();
    landmarks.retain(|l| seen_l.insert((l.name.to_ascii_lowercase(), l.band.clone())));

    Ok((creatures, landmarks))
}