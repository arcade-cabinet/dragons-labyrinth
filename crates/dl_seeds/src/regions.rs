//! Regions seeding module - AI-powered transformation using rich contextual seeds
//! 
//! Uses comprehensive AI prompts with book excerpts + hexroll samples + our themes
//! to transform D&D content into Dragon's Labyrinth appropriate horror content.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;
use crate::entities::{SampleEntity, CategorySamples};

/// Dragon's Labyrinth region seed data (AI-generated)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionSeed {
    pub name: String,
    pub biome_type: String,
    pub corruption_band: u8,
    pub environmental_features: Vec<String>,
    pub horror_encounters: Vec<String>,
    pub faction_presence: Vec<String>,
    pub settlement_types: Vec<String>,
    pub thematic_description: String,
}

/// Collection of region seeds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionSeeds {
    pub regions: Vec<RegionSeed>,
    pub generated_from_samples: Vec<String>,
}

impl RegionSeeds {
    /// Use AI to transform TOML samples with rich contextual seeds
    pub fn generate_from_toml(out_dir: &Path) -> Result<Self> {
        let toml_path = out_dir.join("regions.toml");
        let books_path = out_dir.join("books.toml");
        
        if !toml_path.exists() {
            return Err(anyhow::anyhow!("regions.toml not found in {}", out_dir.display()));
        }
        
        // Load TOML samples and books
        let toml_content = std::fs::read_to_string(&toml_path)?;
        let samples: CategorySamples = toml::from_str(&toml_content)?;
        
        let books_content = if books_path.exists() {
            Some(std::fs::read_to_string(&books_path)?)
        } else {
            None
        };
        
        println!("Transforming {} region samples using AI with rich contextual seeds...", samples.sample_count);
        
        // Use AI with comprehensive context to transform samples
        let regions = Self::ai_transform_regions(&samples, books_content.as_deref())?;
        let sample_names: Vec<String> = samples.entities.iter().map(|e| e.entity_name.clone()).collect();
        
        Ok(Self {
            regions,
            generated_from_samples: sample_names,
        })
    }
    
    /// Comprehensive AI prompt using book seeds + entity samples + our themes
    fn ai_transform_regions(samples: &CategorySamples, books_toml: Option<&str>) -> Result<Vec<RegionSeed>> {
        use crate::ai_client::SeedAiClient;
        use tokio::runtime::Runtime;
        
        let rt = Runtime::new()?;
        let ai_client = SeedAiClient::new()?;
        let ai_prompt = Self::create_comprehensive_transformation_prompt(samples, books_toml);
        
        let seeds_json = rt.block_on(async {
            ai_client.transform_samples_to_seeds(&ai_prompt).await
        })?;
        
        // Parse AI response into RegionSeed structs
        let regions: Vec<RegionSeed> = serde_json::from_value(seeds_json)?;
        Ok(regions)
    }
    
    /// Create comprehensive AI transformation prompt
    fn create_comprehensive_transformation_prompt(samples: &CategorySamples, books_toml: Option<&str>) -> String {
        format!(r#"
# Dragon's Labyrinth Region Transformation

## Your Role
You are transforming complex D&D hexroll data into content for "Dragon's Labyrinth" - a horror RPG with inverted power progression where players grow cursed, not stronger, as they journey toward a dragon.

## Rich Contextual Seeds Available

### Our Game's Themes (from docs/Themes.md):
**5-Band Corruption Progression:**
- **Band 1 (Peace)**: Bright meadows, forests, welcoming villages, chess-piece heroes, bandits, wolves, goblins
- **Band 2 (Unease)**: Charred forests, dry plains, abandoned shops, early corrupted variants (bandit cultists, scorched wolves)  
- **Band 3 (Dread)**: Molten lava fields, dried riverbeds, ruined fortresses, full dragonblight horrors (giant scorched beasts, cult fanatics)
- **Band 4 (Terror)**: War camps, militarized zones, human cruelty mixed with void whispers (mad warlords, fallen priests)
- **Band 5 (Horror)**: Nightmare versions of everything, eldritch manifestations, corrupted companions, void spawn

### Our Game's Architecture (from docs/Architecture.md):
**Key Design Philosophy:** "A horror experience that happens to have RPG mechanics"
**Inverted Power Curve:** Instead of getting stronger, every "advancement" makes you weaker
**Companion Psychology:** Deep trauma mechanics where relationships matter more than stats
**Geographic Coherence:** Logical continent with natural biome transitions

### Rich Literary Context:
{}

## Source Material to Transform
You have {} complex D&D hexroll region samples with taverns, NPC stat blocks, faction memberships, and encounter tables designed for tabletop play.

## Your Task: Extract and Transform
For each region sample, extract the essential elements and transform them to fit our horror progression:

1. **Map to Corruption Band (1-5)**: Based on region name and horror level
2. **Transform Biome**: From D&D terrain to our horror biome progression
3. **Convert Creatures**: Transform D&D stat blocks into our horror encounters fitting the band
4. **Simplify Environment**: Extract key features without complex mechanics  
5. **Extract Factions**: Note political elements for our companion psychology system
6. **Transform Settlements**: Convert complex tavern data to appropriate settlement types

## Critical: Transform, Don't Copy
- NO D&D stat blocks or spell lists
- NO complex encounter tables or dice mechanics
- NO tyrannasaurus rex - use creatures that fit medieval horror
- YES simple environmental features, horror-appropriate encounters, faction politics
- YES thematic descriptions matching our corruption progression

## Output Format
Return as JSON array of RegionSeed objects with the above fields.

## Source Samples:
{}

Transform these D&D samples into content worthy of Dragon's Labyrinth's horror progression.
"#, 
            books_toml.unwrap_or("No book context available"),
            samples.sample_count,
            serde_json::to_string_pretty(samples).unwrap_or_else(|_| "Failed to serialize samples".to_string())
        )
    }
}

fn count_creatures(content: &str) -> usize {
    // Count creature mentions for transformation notes
    let creatures = ["owlbear", "troll", "minotaur", "veteran", "fighter", "rogue", "cleric"];
    creatures.iter().map(|&c| content.matches(c).count()).sum()
}
