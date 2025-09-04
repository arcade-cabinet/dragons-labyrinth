//! Build script for dl_analysis crate
//! 
//! Sets up the environment for HBF processing and downloads Seeds data

use std::env;
use std::fs;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=game.hbf");
    println!("cargo:rerun-if-changed=build.rs");
    
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    
    // Create directories for analysis output
    let analysis_dir = out_dir.join("analysis");
    let models_dir = out_dir.join("models");
    let html_dir = out_dir.join("html");
    let json_dir = out_dir.join("json");
    let ron_dir = out_dir.join("ron");
    let seeds_cache_dir = out_dir.join("seeds_cache");
    
    // Create all output directories
    let dirs = [
        &analysis_dir, &models_dir, &html_dir, &json_dir, &ron_dir, &seeds_cache_dir
    ];
    
    for dir in dirs {
        if let Err(e) = fs::create_dir_all(dir) {
            println!("cargo:warning=Failed to create directory {:?}: {}", dir, e);
        }
    }
    
    // Check if HBF database exists and report status
    let hbf_path = std::path::Path::new("game.hbf");
    if hbf_path.exists() {
        println!("cargo:warning=HBF database found at {:?}", hbf_path);
    } else {
        println!("cargo:warning=HBF database not found at {:?} - analysis will be skipped at runtime", hbf_path);
    }
    
    // Initialize Seeds data directly using SeedsManager
    println!("cargo:warning=Initializing Seeds data...");
    match dl_seeds::SeedsManager::initialize(&seeds_cache_dir) {
        Ok(seeds_manager) => {
            println!("cargo:warning=Seeds data initialized successfully");
            println!("cargo:warning=  Books loaded: {}", seeds_manager.books.get_downloaded_books().len());
            
            // Generate analyzed seeds data for dl_processors
            let analyzed_seeds_dir = out_dir.join("analyzed_seeds");
            if let Err(e) = generate_analyzed_seeds(&seeds_manager, &analyzed_seeds_dir) {
                println!("cargo:warning=Failed to generate analyzed seeds: {}. Build will continue.", e);
            } else {
                println!("cargo:warning=Analyzed seeds data generated successfully");
            }
        }
        Err(e) => {
            println!("cargo:warning=Failed to initialize Seeds data: {}. Build will continue.", e);
        }
    }
    
    // Create environment variables for runtime use
    println!("cargo:rustc-env=DL_ANALYSIS_OUT_DIR={}", out_dir.display());
    println!("cargo:rustc-env=DL_HBF_PATH=game.hbf");
    println!("cargo:rustc-env=DL_SEEDS_CACHE_DIR={}", seeds_cache_dir.display());
    
    println!("cargo:warning=dl_analysis build script completed successfully");
    
    Ok(())
}

/// Generate analyzed and categorized seeds data for dl_processors
fn generate_analyzed_seeds(seeds_manager: &dl_seeds::SeedsManager, analyzed_seeds_dir: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    use std::collections::HashMap;
    use ron::ser::{to_string_pretty, PrettyConfig};
    
    // Create categorized directories
    let dialogue_dir = analyzed_seeds_dir.join("dialogue");
    let quests_dir = analyzed_seeds_dir.join("quests");
    let linguistics_dir = analyzed_seeds_dir.join("linguistics");
    
    fs::create_dir_all(&dialogue_dir)?;
    fs::create_dir_all(&quests_dir)?;
    fs::create_dir_all(&linguistics_dir)?;
    
    // Create act-specific dialogue directories (1-5 for corruption progression)
    for act in 1..=5 {
        fs::create_dir_all(dialogue_dir.join(format!("act{}", act)))?;
    }
    
    // Create quest pattern directories
    let quest_patterns = ["investigation", "purification", "escort", "rescue", "exploration"];
    for pattern in &quest_patterns {
        fs::create_dir_all(quests_dir.join(pattern))?;
    }
    
    // Create region-specific linguistics directories  
    let region_types = ["meadows", "forests", "swamps", "mountains", "plains", "deserts", "ruins"];
    for region_type in &region_types {
        fs::create_dir_all(linguistics_dir.join(region_type))?;
    }
    
    // Generate categorized seeds data from the downloaded books
    let books = seeds_manager.books.get_downloaded_books();
    let mut dialogue_by_act: HashMap<u8, Vec<String>> = HashMap::new();
    let mut quests_by_pattern: HashMap<String, Vec<String>> = HashMap::new();
    let mut linguistics_by_region: HashMap<String, Vec<String>> = HashMap::new();
    
    // Initialize empty collections for each category
    for act in 1..=5 {
        dialogue_by_act.insert(act, Vec::new());
    }
    for pattern in &quest_patterns {
        quests_by_pattern.insert(pattern.to_string(), Vec::new());
    }
    for region_type in &region_types {
        linguistics_by_region.insert(region_type.to_string(), Vec::new());
    }
    
    // Process each book and categorize content
    for book_record in books {
        // Read actual book content from file
        let book_path = seeds_manager.get_book_path(&book_record.filename);
        let book_content = match std::fs::read_to_string(&book_path) {
            Ok(content) => content,
            Err(e) => {
                println!("cargo:warning=Failed to read book {}: {}. Skipping.", book_record.filename, e);
                continue;
            }
        };
        
        // Simple categorization based on content patterns
        let content_lower = book_content.to_lowercase();
        let book_id = &book_record.id;
        
        // Categorize dialogue by corruption level/act
        if content_lower.contains("peaceful") || content_lower.contains("hope") {
            dialogue_by_act.get_mut(&1).unwrap().push(format!("Book {}: {}", book_id, extract_dialogue_sample(&content_lower)));
        } else if content_lower.contains("unease") || content_lower.contains("worry") {
            dialogue_by_act.get_mut(&2).unwrap().push(format!("Book {}: {}", book_id, extract_dialogue_sample(&content_lower)));
        } else if content_lower.contains("dread") || content_lower.contains("fear") {
            dialogue_by_act.get_mut(&3).unwrap().push(format!("Book {}: {}", book_id, extract_dialogue_sample(&content_lower)));
        } else if content_lower.contains("terror") || content_lower.contains("horror") {
            dialogue_by_act.get_mut(&4).unwrap().push(format!("Book {}: {}", book_id, extract_dialogue_sample(&content_lower)));
        } else {
            dialogue_by_act.get_mut(&5).unwrap().push(format!("Book {}: {}", book_id, extract_dialogue_sample(&content_lower)));
        }
        
        // Categorize quests by pattern
        if content_lower.contains("investigate") || content_lower.contains("mystery") {
            quests_by_pattern.get_mut("investigation").unwrap().push(format!("Book {}: Quest seed", book_id));
        } else if content_lower.contains("purify") || content_lower.contains("cleanse") {
            quests_by_pattern.get_mut("purification").unwrap().push(format!("Book {}: Quest seed", book_id));
        } else if content_lower.contains("escort") || content_lower.contains("guide") {
            quests_by_pattern.get_mut("escort").unwrap().push(format!("Book {}: Quest seed", book_id)); 
        } else if content_lower.contains("rescue") || content_lower.contains("save") {
            quests_by_pattern.get_mut("rescue").unwrap().push(format!("Book {}: Quest seed", book_id));
        } else {
            quests_by_pattern.get_mut("exploration").unwrap().push(format!("Book {}: Quest seed", book_id));
        }
        
        // Categorize linguistics by region type
        if content_lower.contains("meadow") || content_lower.contains("grass") {
            linguistics_by_region.get_mut("meadows").unwrap().push(format!("Book {}: Language pattern", book_id));
        } else if content_lower.contains("forest") || content_lower.contains("tree") {
            linguistics_by_region.get_mut("forests").unwrap().push(format!("Book {}: Language pattern", book_id));
        } else if content_lower.contains("swamp") || content_lower.contains("marsh") {
            linguistics_by_region.get_mut("swamps").unwrap().push(format!("Book {}: Language pattern", book_id));
        } else if content_lower.contains("mountain") || content_lower.contains("peak") {
            linguistics_by_region.get_mut("mountains").unwrap().push(format!("Book {}: Language pattern", book_id));
        } else if content_lower.contains("plain") || content_lower.contains("field") {
            linguistics_by_region.get_mut("plains").unwrap().push(format!("Book {}: Language pattern", book_id));
        } else if content_lower.contains("desert") || content_lower.contains("sand") {
            linguistics_by_region.get_mut("deserts").unwrap().push(format!("Book {}: Language pattern", book_id));
        } else {
            linguistics_by_region.get_mut("ruins").unwrap().push(format!("Book {}: Language pattern", book_id));
        }
    }
    
    // Write categorized data to RON files
    for (act, dialogue_lines) in dialogue_by_act {
        let ron_content = to_string_pretty(&dialogue_lines, PrettyConfig::default())?;
        fs::write(dialogue_dir.join(format!("act{}/dialogue.ron", act)), ron_content)?;
    }
    
    for (pattern, quest_seeds) in quests_by_pattern {
        let ron_content = to_string_pretty(&quest_seeds, PrettyConfig::default())?;
        fs::write(quests_dir.join(format!("{}/quests.ron", pattern)), ron_content)?;
    }
    
    for (region_type, language_patterns) in linguistics_by_region {
        let ron_content = to_string_pretty(&language_patterns, PrettyConfig::default())?;
        fs::write(linguistics_dir.join(format!("{}/linguistics.ron", region_type)), ron_content)?;
    }
    
    println!("cargo:warning=Generated categorized seeds data: {} acts, {} quest patterns, {} region types", 
             5, quest_patterns.len(), region_types.len());
    
    Ok(())
}

/// Extract a sample dialogue line from book content
fn extract_dialogue_sample(content: &str) -> String {
    // Find first sentence that looks like dialogue
    for line in content.lines().take(10) {
        let line = line.trim();
        if line.len() > 20 && line.len() < 100 && (line.contains("said") || line.contains("asked") || line.contains("\"")) {
            return line.to_string();
        }
    }
    // Fallback to first substantial line
    content.lines()
        .find(|line| line.trim().len() > 30)
        .unwrap_or("Sample dialogue line")
        .trim()
        .to_string()
}
