//! Books module for generating world, quest, and dialogue seeds from literature
//! 
//! Uses rust-bert for text processing and AI for seed generation from downloaded
//! Internet Archive texts focused on Dragon's Labyrinth themes.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;
use rust_bert::pipelines::summarization::{SummarizationConfig, SummarizationModel};

/// Manager for generating seeds from literature using rust-bert
#[derive(Debug)]
pub struct BooksManager {
    pub cache_dir: std::path::PathBuf,
    pub downloaded_books: Vec<BookRecord>,
    pub world_seeds: Vec<WorldSeed>,
    pub quest_seeds: Vec<QuestSeed>,
    pub dialogue_seeds: Vec<DialogueSeed>,
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

impl BooksManager {
    /// Generate seeds from downloaded txt files using rust-bert
    pub fn generate_seeds_from_texts(out_dir: &Path) -> Result<Self> {
        println!("Generating world/quest/dialogue seeds from Internet Archive texts...");
        
        // Initialize rust-bert summarization model
        let mut summarization_model = SummarizationModel::new(SummarizationConfig::default())?;
        
        let mut world_seeds = Vec::new();
        let mut quest_seeds = Vec::new();
        let mut dialogue_seeds = Vec::new();
        let mut downloaded_books = Vec::new();
        
        // Process each downloaded txt file
        let txt_files = [
            ("lovecraft_collection.txt", "Cosmic Horror"),
            ("sword_sorcery_tales.txt", "Medieval Fantasy"),
            ("gothic_horror.txt", "Gothic Horror"),
            ("arthurian_legends.txt", "Arthurian Romance"),
            ("norse_sagas.txt", "Norse Mythology"),
            ("occult_texts.txt", "Occult Literature"),
            ("dark_fairy_tales.txt", "Dark Folklore"),
            ("medieval_bestiaries.txt", "Medieval Bestiaries"),
        ];
        
        for (filename, genre) in txt_files {
            let txt_path = out_dir.join(filename);
            if txt_path.exists() {
                let content = std::fs::read_to_string(&txt_path)?;
                
                // Use rust-bert to summarize key themes
                let summaries = summarization_model.summarize(&[content.clone()])?;
                
                // Generate seeds using AI analysis of summaries + original content
                let (world, quest, dialogue) = Self::generate_seeds_from_book(
                    filename, genre, &content, &summaries
                )?;
                
                world_seeds.extend(world);
                quest_seeds.extend(quest);
                dialogue_seeds.extend(dialogue);
                
                downloaded_books.push(BookRecord {
                    id: filename.replace(".txt", ""),
                    title: format!("{} Collection", genre),
                    source: "internet_archive".to_string(),
                    filename: filename.to_string(),
                    file_size: content.len() as u64,
                });
                
                println!("Generated seeds from {}", filename);
            }
        }
        
        Ok(Self {
            cache_dir: out_dir.to_path_buf(),
            downloaded_books,
            world_seeds,
            quest_seeds,
            dialogue_seeds,
        })
    }
    
    /// Generate world, quest, and dialogue seeds from a single book using AI
    fn generate_seeds_from_book(
        filename: &str,
        genre: &str,
        content: &str,
        summaries: &[String],
    ) -> Result<(Vec<WorldSeed>, Vec<QuestSeed>, Vec<DialogueSeed>)> {
        use crate::ai_client::SeedAiClient;
        use tokio::runtime::Runtime;
        
        let rt = Runtime::new()?;
        let ai_client = SeedAiClient::new()?;
        
        // Create comprehensive prompt for seed generation
        let seed_prompt = format!(r#"
# Dragon's Labyrinth Seed Generation from Literature

## Your Role
Extract world building, quest, and dialogue seeds from this {} literature for "Dragon's Labyrinth" - a horror RPG with 5-band corruption progression.

## Source Material
**Book**: {}
**Genre**: {}
**Summaries**: {}
**Content Sample**: {}

## Dragon's Labyrinth Context
**5-Band Progression**: Peace → Unease → Dread → Terror → Horror
**Inverted Power**: Players grow cursed, not stronger
**Companion Psychology**: Deep trauma mechanics, relationships over stats
**Forge System**: Light/dark paths using sentimental items for mythic gear

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

## Output Format
Return JSON with three arrays: world_seeds, quest_seeds, dialogue_seeds

Transform this literature into seeds appropriate for Dragon's Labyrinth's horror progression and companion psychology system.
"#, 
            genre,
            filename,
            genre,
            summaries.join(" | "),
            content.chars().take(2000).collect::<String>()
        );
        
        let seeds_json = rt.block_on(async {
            ai_client.transform_samples_to_seeds(&seed_prompt).await
        })?;
        
        // Parse AI response
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
