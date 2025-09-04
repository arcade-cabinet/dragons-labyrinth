//! Books seeding module - AI-powered transformation using rich literary context
//! 
//! Uses comprehensive AI prompts to transform Internet Archive book summaries
//! into world building, quest, and dialogue seeds for Dragon's Labyrinth.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Book summary from build.rs (matches build.rs types)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookSummary {
    pub id: String,
    pub title: String,
    pub filename: String,
    pub summary: String,
    pub full_length: usize,
}

/// TOML container for book summaries (matches build.rs types)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BooksTomlContainer {
    pub books: Vec<BookSummary>,
    pub generated_at: String,
}

/// World building seed extracted from literature
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldSeed {
    pub source_book: String,
    pub biome_inspiration: String,
    pub environmental_features: Vec<String>,
    pub atmospheric_elements: Vec<String>,
    pub corruption_themes: Vec<String>,
}

/// Quest pattern seed extracted from literature
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestSeed {
    pub source_book: String,
    pub quest_archetype: String,
    pub moral_dilemma: String,
    pub companion_impact: String,
    pub forge_relevance: String,
}

/// Dialogue pattern seed extracted from literature
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueSeed {
    pub source_book: String,
    pub emotional_tone: String,
    pub character_archetype: String,
    pub trauma_indicators: Vec<String>,
    pub speech_patterns: Vec<String>,
}

/// Manager for generating seeds from literature summaries
#[derive(Debug)]
pub struct BooksManager {
    pub cache_dir: std::path::PathBuf,
    pub downloaded_books: Vec<BookRecord>,
    pub world_seeds: Vec<WorldSeed>,
    pub quest_seeds: Vec<QuestSeed>,
    pub dialogue_seeds: Vec<DialogueSeed>,
}

impl BooksManager {
    /// Generate seeds from books.toml using AI transformation
    pub fn generate_seeds_from_texts(out_dir: &Path) -> Result<Self> {
        let books_path = out_dir.join("books.toml");
        
        if !books_path.exists() {
            return Err(anyhow::anyhow!("books.toml not found in {}", out_dir.display()));
        }
        
        // Load TOML book summaries
        let toml_content = std::fs::read_to_string(&books_path)?;
        let books_container: BooksTomlContainer = toml::from_str(&toml_content)?;
        
        println!("Transforming {} book summaries using AI...", books_container.books.len());
        
        // Use AI to transform book summaries into seeds
        let (world_seeds, quest_seeds, dialogue_seeds) = Self::ai_transform_book_summaries(&books_container)?;
        
        // Convert summaries to download records
        let downloaded_books: Vec<BookRecord> = books_container.books.iter().map(|book| {
            BookRecord {
                id: book.id.clone(),
                title: book.title.clone(),
                source: "internet_archive".to_string(),
                filename: book.filename.clone(),
                file_size: book.full_length as u64,
            }
        }).collect();
        
        Ok(Self {
            cache_dir: out_dir.to_path_buf(),
            downloaded_books,
            world_seeds,
            quest_seeds,
            dialogue_seeds,
        })
    }
    
    /// Use AI to transform book summaries into structured seeds
    fn ai_transform_book_summaries(books_container: &BooksTomlContainer) -> Result<(Vec<WorldSeed>, Vec<QuestSeed>, Vec<DialogueSeed>)> {
        use crate::ai_client::SeedAiClient;
        use tokio::runtime::Runtime;
        
        let rt = Runtime::new()?;
        let ai_client = SeedAiClient::new()?;
        let ai_prompt = Self::create_comprehensive_transformation_prompt(books_container);
        
        let seeds_json = rt.block_on(async {
            ai_client.transform_samples_to_seeds(&ai_prompt).await
        })?;
        
        // Parse AI response into seed types
        let world_seeds: Vec<WorldSeed> = serde_json::from_value(
            seeds_json.get("world_seeds").cloned().unwrap_or_default()
        )?;
        let quest_seeds: Vec<QuestSeed> = serde_json::from_value(
            seeds_json.get("quest_seeds").cloned().unwrap_or_default()
        )?;
        let dialogue_seeds: Vec<DialogueSeed> = serde_json::from_value(
            seeds_json.get("dialogue_seeds").cloned().unwrap_or_default()
        )?;
        
        Ok((world_seeds, quest_seeds, dialogue_seeds))
    }
    
    /// Create comprehensive AI transformation prompt for book summaries
    fn create_comprehensive_transformation_prompt(books_container: &BooksTomlContainer) -> String {
        format!(r#"
# Dragon's Labyrinth Literature Seed Generation

## Your Role
Extract world building, quest, and dialogue seeds from Internet Archive book summaries for "Dragon's Labyrinth" - a horror RPG with 5-band corruption progression and companion psychology.

## Dragon's Labyrinth Context
**5-Band Progression**: Peace → Unease → Dread → Terror → Horror
**Inverted Power**: Players grow cursed, not stronger
**Companion Psychology**: Deep trauma mechanics, relationships over stats  
**Forge System**: Light/dark paths using sentimental items for mythic gear

## Source Material
You have {} book summaries from Internet Archive covering medieval literature, horror, mythology, and folklore.

## Extract These Seeds:

### World Seeds
- **Biome Inspiration**: How can this literature inspire our horror biome progression?
- **Environmental Features**: Atmospheric elements for different corruption bands
- **Corruption Themes**: How corruption manifests in this literary tradition

### Quest Seeds  
- **Quest Archetypes**: Classic quest patterns from this literature
- **Moral Dilemmas**: Choices that affect companion psychology
- **Forge Relevance**: How quests provide materials for light/dark forge paths

### Dialogue Seeds
- **Emotional Tones**: How characters express trauma, hope, despair
- **Character Archetypes**: Personality types for our companion system  
- **Speech Patterns**: Language patterns fitting our medieval horror aesthetic

## Critical: Transform to Match Our Horror Progression
- Focus on elements that fit medieval horror (not modern/sci-fi)
- Extract psychological themes for companion trauma system
- Identify moral choices that affect light/dark forge paths
- Convert complex literary themes to simple game mechanics

## Output Format
Return JSON with three arrays: world_seeds, quest_seeds, dialogue_seeds

## Book Summaries:
{}

Transform these literary summaries into seeds appropriate for Dragon's Labyrinth's horror progression and companion psychology system.
"#, 
            books_container.books.len(),
            serde_json::to_string_pretty(books_container).unwrap_or_else(|_| "Failed to serialize books".to_string())
        )
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
