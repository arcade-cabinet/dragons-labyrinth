//! Linguistics module for downloading and processing Old Norse and other linguistic sources
//! 
//! This module uses cleasby_vigfusson_dictionary and other sources for real linguistic data

use anyhow::Result;
use cleasby_vigfusson_dictionary::{get_no_markup_dictionary, DictionaryEntry};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// Manager for linguistic data downloads and processing
pub struct LinguisticsManager {
    pub cache_dir: std::path::PathBuf,
    pub old_norse_dictionary: Vec<DictionaryEntry>,
    pub thematic_wordlists: HashMap<String, Vec<String>>,
    pub language_blends: HashMap<String, LanguageBlend>,
}

impl LinguisticsManager {
    /// Initialize by loading Cleasby-Vigfusson dictionary and other linguistic sources
    pub fn initialize(linguistics_dir: &Path) -> Result<Self> {
        std::fs::create_dir_all(linguistics_dir)?;
        
        // Load the Cleasby-Vigfusson Old Norse dictionary
        println!("Loading Cleasby-Vigfusson Old Norse dictionary...");
        let old_norse_dictionary = get_no_markup_dictionary()
            .expect("Failed to load Old Norse dictionary");
        
        // Extract thematic wordlists from the dictionary
        let mut thematic_wordlists = HashMap::new();
        
        // Extract words by theme from the dictionary
        let themes = vec![
            ("sea", vec!["sea", "ocean", "wave", "ship", "sail"]),
            ("storm", vec!["storm", "wind", "thunder", "lightning", "tempest"]),
            ("battle", vec!["battle", "war", "fight", "sword", "shield"]),
            ("night", vec!["night", "dark", "shadow", "moon", "star"]),
            ("wolf", vec!["wolf", "beast", "hunt", "pack"]),
            ("winter", vec!["winter", "cold", "frost", "ice", "snow"]),
            ("dragon", vec!["dragon", "serpent", "wyrm", "drake"]),
            ("death", vec!["death", "dead", "grave", "ghost", "spirit"]),
        ];
        
        for (theme_name, keywords) in themes {
            let mut theme_words = Vec::new();
            
            // Search dictionary for entries matching keywords
            for entry in &old_norse_dictionary {
                for keyword in &keywords {
                    // Check all definitions for this entry
                    for definition in &entry.definitions {
                        if definition.to_lowercase().contains(keyword) {
                            theme_words.push(entry.word.clone());
                            break;
                        }
                    }
                }
            }
            
            thematic_wordlists.insert(theme_name.to_string(), theme_words);
        }
        
        // Create language blend configurations for different Acts/Regions
        let mut language_blends = HashMap::new();
        
        language_blends.insert("meadows_act1".to_string(), LanguageBlend {
            old_english: 0.4,
            welsh: 0.4,
            old_norse: 0.2,
            arabic: 0.0,
            hebrew: 0.0,
        });
        
        language_blends.insert("warfront_act2".to_string(), LanguageBlend {
            old_english: 0.3,
            welsh: 0.2,
            old_norse: 0.5,
            arabic: 0.0,
            hebrew: 0.0,
        });
        
        language_blends.insert("void_act3".to_string(), LanguageBlend {
            old_english: 0.2,
            welsh: 0.2,
            old_norse: 0.4,
            arabic: 0.1,
            hebrew: 0.1,
        });
        
        // Save manifest
        let manifest = LinguisticsManifest {
            version: "1.0.0".to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
            dictionary_entries: old_norse_dictionary.len(),
            themes: thematic_wordlists.keys().cloned().collect(),
            blends: language_blends.keys().cloned().collect(),
        };
        
        std::fs::write(
            linguistics_dir.join("manifest.json"),
            serde_json::to_string_pretty(&manifest)?
        )?;
        
        Ok(Self {
            cache_dir: linguistics_dir.to_path_buf(),
            old_norse_dictionary,
            thematic_wordlists,
            language_blends,
        })
    }
    
    /// Load from existing cache
    pub fn load_from_cache(linguistics_dir: &Path) -> Result<Self> {
        let manifest_path = linguistics_dir.join("manifest.json");
        if !manifest_path.exists() {
            return Self::initialize(linguistics_dir);
        }
        
        // Re-initialize with cached data
        Self::initialize(linguistics_dir)
    }
    
    /// Generate a name using linguistic processing
    pub fn generate_name(&self, english_seed: &str, context: &NameContext) -> String {
        // Look up Old Norse translation
        if let Some(entry) = self.lookup_old_norse(english_seed) {
            // Apply language blend based on context
            if let Some(blend) = self.language_blends.get(&context.region_type) {
                if blend.old_norse > 0.5 {
                    return self.transliterate_for_rust(&entry.word);
                }
            }
        }
        
        // Fallback to modified English seed
        format!("{}_of_{}", english_seed, context.region_type)
    }
    
    /// Look up a word in the Old Norse dictionary
    pub fn lookup_old_norse(&self, english_word: &str) -> Option<&DictionaryEntry> {
        self.old_norse_dictionary
            .iter()
            .find(|entry| {
                entry.definitions.iter().any(|def| 
                    def.to_lowercase().contains(&english_word.to_lowercase())
                )
            })
    }
    
    /// Get thematic wordlist
    pub fn get_theme_words(&self, theme: &str) -> Option<&Vec<String>> {
        self.thematic_wordlists.get(theme)
    }
    
    /// Transliterate Old Norse characters to ASCII for Rust identifiers
    fn transliterate_for_rust(&self, old_norse: &str) -> String {
        old_norse
            .replace('á', "a")
            .replace('é', "e")
            .replace('í', "i")
            .replace('ó', "o")
            .replace('ú', "u")
            .replace('ý', "y")
            .replace('æ', "ae")
            .replace('ø', "o")
            .replace('œ', "oe")
            .replace('þ', "th")
            .replace('ð', "d")
            .replace('ǫ', "o")
            .chars()
            .filter(|c| c.is_ascii_alphanumeric() || *c == '_')
            .collect()
    }
}

/// Context for name generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NameContext {
    pub region_type: String,
    pub act_number: u32,
    pub corruption_level: f32,
}

/// Language blend configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageBlend {
    pub old_english: f32,
    pub welsh: f32,
    pub old_norse: f32,
    pub arabic: f32,
    pub hebrew: f32,
}

/// Manifest for tracking linguistic data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinguisticsManifest {
    pub version: String,
    pub created_at: String,
    pub dictionary_entries: usize,
    pub themes: Vec<String>,
    pub blends: Vec<String>,
}
