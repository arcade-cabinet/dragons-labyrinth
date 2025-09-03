//! Build API for dl_seeds crate
//! 
//! Provides comprehensive interface for dl_analysis to obtain processed seeds data

use anyhow::Result;
use std::path::Path;
use crate::{SeedsManager, BookRecord, CharacterArchetype, QuestPattern, TraitTemplate};

/// Complete seeds data bundle ready for analysis
#[derive(Debug, Clone)]
pub struct SeedsBuildData {
    pub literature: LiteratureBuildData,
    pub linguistics: LinguisticsBuildData, 
    pub dialogue: DialogueBuildData,
}

/// Literature data organized for analysis
#[derive(Debug, Clone)]
pub struct LiteratureBuildData {
    pub books: Vec<BookRecord>,
    pub books_by_theme: std::collections::HashMap<String, Vec<String>>,
    pub total_books: usize,
}

/// Linguistics data organized for analysis
#[derive(Debug, Clone)]
pub struct LinguisticsBuildData {
    pub old_norse_dictionary: Vec<(String, String)>,
    pub language_patterns: std::collections::HashMap<String, Vec<String>>,
    pub dictionary_size: usize,
}

/// Dialogue data organized for analysis
#[derive(Debug, Clone)]
pub struct DialogueBuildData {
    pub character_archetypes: Vec<CharacterArchetype>,
    pub quest_patterns: Vec<QuestPattern>,
    pub trait_templates: Vec<TraitTemplate>,
    pub archetypes_count: usize,
}

impl SeedsBuildData {
    /// Load complete seeds data for downstream analysis
    pub fn load_for_analysis(seeds_manager: &SeedsManager) -> Result<Self> {
        let literature = LiteratureBuildData::from_manager(seeds_manager)?;
        let linguistics = LinguisticsBuildData::from_manager(seeds_manager)?;
        let dialogue = DialogueBuildData::from_manager(seeds_manager)?;

        Ok(Self {
            literature,
            linguistics,
            dialogue,
        })
    }

    /// Get literature data organized by thematic content for act analysis
    pub fn get_literature_by_act(&self, act: u8) -> Vec<String> {
        let act_themes = match act {
            1 => vec!["peace", "pastoral", "hope", "journey"],
            2 => vec!["unease", "doubt", "shadow", "warning"],
            3 => vec!["dread", "fear", "darkness", "loss"],
            4 => vec!["terror", "horror", "madness", "corruption"],
            5 => vec!["apocalypse", "void", "ending", "death"],
            _ => vec!["mystery"],
        };

        let mut relevant_content = Vec::new();
        for theme in act_themes {
            if let Some(theme_books) = self.literature.books_by_theme.get(theme) {
                relevant_content.extend(theme_books.clone());
            }
        }
        relevant_content
    }

    /// Get quest patterns organized by archetype
    pub fn get_quest_patterns_by_type(&self, pattern_type: &str) -> Vec<&QuestPattern> {
        self.dialogue.quest_patterns
            .iter()
            .filter(|pattern| pattern.pattern_type == pattern_type)
            .collect()
    }

    /// Get linguistic rules for specific region type
    pub fn get_linguistic_rules_by_region(&self, region_type: &str) -> Option<&Vec<String>> {
        self.linguistics.language_patterns.get(region_type)
    }
}

impl LiteratureBuildData {
    fn from_manager(seeds_manager: &SeedsManager) -> Result<Self> {
        let books = seeds_manager.books.get_downloaded_books().to_vec();
        
        // Organize books by thematic content for analysis
        let mut books_by_theme = std::collections::HashMap::new();
        
        // Categorize books based on their content themes
        // This would ideally analyze actual text content, but for now we'll use metadata
        for book in &books {
            let themes = categorize_book_themes(&book.title);
            for theme in themes {
                books_by_theme
                    .entry(theme.to_string())
                    .or_insert_with(Vec::new)
                    .push(book.title.clone());
            }
        }

        Ok(Self {
            total_books: books.len(),
            books,
            books_by_theme,
        })
    }
}

impl LinguisticsBuildData {
    fn from_manager(seeds_manager: &SeedsManager) -> Result<Self> {
        let old_norse_dict = seeds_manager.linguistics.old_norse_dictionary
            .iter()
            .map(|entry| (entry.word.clone(), entry.definitions.join("; ")))
            .collect::<Vec<_>>();

        // Organize linguistic patterns by region type
        let mut language_patterns = std::collections::HashMap::new();
        
        // Generate patterns for each region type from Old Norse dictionary
        let region_types = [
            "meadows", "forests", "swamps", "deserts", "mountains",
            "villages", "ruins", "dungeons", "shrines", "portals"
        ];

        for region_type in region_types {
            let patterns = extract_patterns_for_region(&old_norse_dict, region_type);
            language_patterns.insert(region_type.to_string(), patterns);
        }

        Ok(Self {
            dictionary_size: old_norse_dict.len(),
            old_norse_dictionary: old_norse_dict,
            language_patterns,
        })
    }
}

impl DialogueBuildData {
    fn from_manager(seeds_manager: &SeedsManager) -> Result<Self> {
        let character_archetypes = seeds_manager.dialogue.character_archetypes.clone();
        
        // Create some default quest patterns since they don't exist in the current structure
        let quest_patterns = vec![
            QuestPattern {
                pattern_type: "investigation".to_string(),
                beats: vec!["discover_clue".to_string(), "follow_trail".to_string(), "solve_mystery".to_string()],
                themes: vec!["mystery".to_string(), "truth".to_string()],
                sources: vec!["classic_detective".to_string()],
            },
            QuestPattern {
                pattern_type: "purification".to_string(),
                beats: vec!["identify_corruption".to_string(), "gather_materials".to_string(), "perform_ritual".to_string()],
                themes: vec!["cleansing".to_string(), "redemption".to_string()],
                sources: vec!["religious_texts".to_string()],
            },
            QuestPattern {
                pattern_type: "escort".to_string(),
                beats: vec!["meet_client".to_string(), "navigate_dangers".to_string(), "deliver_safely".to_string()],
                themes: vec!["protection".to_string(), "responsibility".to_string()],
                sources: vec!["adventure_stories".to_string()],
            },
        ];

        let trait_templates = seeds_manager.dialogue.trait_templates.clone();

        Ok(Self {
            archetypes_count: character_archetypes.len(),
            character_archetypes,
            quest_patterns,
            trait_templates,
        })
    }
}

/// Categorize book themes based on title/content
fn categorize_book_themes(title: &str) -> Vec<&'static str> {
    let title_lower = title.to_lowercase();
    let mut themes = Vec::new();

    // Gothic/Horror themes
    if title_lower.contains("dracula") || title_lower.contains("vampire") {
        themes.extend(&["horror", "darkness", "corruption"]);
    }
    if title_lower.contains("poe") || title_lower.contains("raven") {
        themes.extend(&["mystery", "death", "dread"]);
    }
    if title_lower.contains("frankenstein") {
        themes.extend(&["horror", "madness", "corruption"]);
    }
    
    // Adventure/Journey themes
    if title_lower.contains("adventure") || title_lower.contains("journey") {
        themes.extend(&["journey", "hope", "discovery"]);
    }
    
    // Pastoral/Peace themes
    if title_lower.contains("pastoral") || title_lower.contains("garden") {
        themes.extend(&["peace", "pastoral", "hope"]);
    }

    // Default fallback
    if themes.is_empty() {
        themes.push("mystery");
    }

    themes
}

/// Extract linguistic patterns for specific region type from dictionary
fn extract_patterns_for_region(
    dictionary: &[(String, String)], 
    region_type: &str
) -> Vec<String> {
    let mut patterns = Vec::new();
    
    // Look for words related to the region type
    let region_keywords = match region_type {
        "meadows" => vec!["grass", "field", "green", "pastoral", "gentle"],
        "forests" => vec!["tree", "wood", "forest", "leaf", "branch"],
        "swamps" => vec!["bog", "marsh", "wet", "dark", "murky"],
        "deserts" => vec!["sand", "dry", "hot", "barren", "waste"],
        "mountains" => vec!["hill", "peak", "stone", "high", "rock"],
        "villages" => vec!["house", "home", "people", "settle", "dwell"],
        "ruins" => vec!["old", "ancient", "broken", "decay", "ruin"],
        "dungeons" => vec!["dark", "deep", "prison", "cave", "under"],
        "shrines" => vec!["holy", "sacred", "god", "divine", "worship"],
        "portals" => vec!["gate", "door", "passage", "magic", "other"],
        _ => vec!["place"],
    };

    // Extract relevant words from dictionary
    for (word, definition) in dictionary.iter().take(100) {  // Limit for performance
        let def_lower = definition.to_lowercase();
        for keyword in &region_keywords {
            if def_lower.contains(keyword) {
                patterns.push(word.clone());
                break;
            }
        }
    }

    // Add fallback patterns if none found
    if patterns.is_empty() {
        patterns.extend([
            format!("{}heim", region_type),
            format!("{}gard", region_type),
            format!("{}vik", region_type),
        ]);
    }

    patterns
}

/// Public API function for dl_analysis to call during build
pub fn provide_seeds_data_for_analysis(cache_dir: &Path) -> Result<SeedsBuildData> {
    // Load or initialize seeds data
    let seeds_manager = match SeedsManager::load_from_cache(cache_dir) {
        Ok(manager) => manager,
        Err(_) => {
            // Cache doesn't exist, initialize fresh
            SeedsManager::initialize(cache_dir)?
        }
    };

    // Convert to analysis-ready format
    SeedsBuildData::load_for_analysis(&seeds_manager)
}
