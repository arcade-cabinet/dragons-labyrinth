//! Dungeons seeding module - AI-powered transformation using rich contextual seeds
//! 
//! Uses comprehensive AI prompts with book excerpts + hexroll samples + our themes
//! to transform complex D&D dungeon data into Dragon's Labyrinth appropriate content.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;
use crate::entities::CategorySamples;

/// Dragon's Labyrinth dungeon seed data (AI-generated)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DungeonSeed {
    pub name: String,
    pub dungeon_type: String,
    pub corruption_band: u8,
    pub horror_theme: String,
    pub key_areas: Vec<DungeonArea>,
    pub environmental_hazards: Vec<String>,
    pub horror_encounters: Vec<String>,
    pub loot_philosophy: String, // light/dark forge materials
    pub thematic_description: String,
}

/// Simplified dungeon area (not complex D&D room descriptions)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DungeonArea {
    pub area_type: String,
    pub emotional_impact: String, // for companion trauma system
    pub environmental_feature: String,
    pub encounter_type: String,
}

/// Collection of dungeon seeds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DungeonSeeds {
    pub dungeons: Vec<DungeonSeed>,
    pub generated_from_samples: Vec<String>,
}

impl DungeonSeeds {
    /// Use AI to transform TOML samples with rich contextual seeds
    pub fn generate_from_toml(out_dir: &Path) -> Result<Self> {
        let toml_path = out_dir.join("dungeons.toml");
        let books_path = out_dir.join("books.toml");
        
        if !toml_path.exists() {
            return Err(anyhow::anyhow!("dungeons.toml not found in {}", out_dir.display()));
        }
        
        // Load TOML samples and books
        let toml_content = std::fs::read_to_string(&toml_path)?;
        let samples: CategorySamples = toml::from_str(&toml_content)?;
        
        let books_content = if books_path.exists() {
            Some(std::fs::read_to_string(&books_path)?)
        } else {
            None
        };
        
        println!("Transforming {} dungeon samples using AI with rich contextual seeds...", samples.sample_count);
        
        // Use AI with comprehensive context to transform samples
        let dungeons = Self::ai_transform_dungeons(&samples, books_content.as_deref())?;
        let sample_names: Vec<String> = samples.entities.iter().map(|e| e.entity_name.clone()).collect();
        
        Ok(Self {
            dungeons,
            generated_from_samples: sample_names,
        })
    }
    
    /// Comprehensive AI prompt for dungeon transformation
    fn ai_transform_dungeons(samples: &CategorySamples, books_toml: Option<&str>) -> Result<Vec<DungeonSeed>> {
        use crate::ai_client::SeedAiClient;
        use tokio::runtime::Runtime;
        
        let rt = Runtime::new()?;
        let ai_client = SeedAiClient::new()?;
        let ai_prompt = Self::create_comprehensive_transformation_prompt(samples, books_toml);
        
        let seeds_json = rt.block_on(async {
            ai_client.transform_samples_to_seeds(&ai_prompt).await
        })?;
        
        // Parse AI response into DungeonSeed structs
        let dungeons: Vec<DungeonSeed> = serde_json::from_value(seeds_json)?;
        Ok(dungeons)
    }
    
    /// Create comprehensive AI transformation prompt for dungeons
    fn create_comprehensive_transformation_prompt(samples: &CategorySamples, books_toml: Option<&str>) -> String {
        format!(r#"
# Dragon's Labyrinth Dungeon Transformation

## Your Role
You are transforming complex D&D dungeon data into horror dungeon seeds for "Dragon's Labyrinth" - a horror RPG focused on companion trauma and forge trials.

## Rich Contextual Seeds Available

### Our Game's Themes (from docs/Themes.md):
**Dungeon Evolution by Corruption Band:**
- **Band 1**: "caves" - simple exploration with minimal horror
- **Band 2**: "haunted ruins" - environmental storytelling of decay
- **Band 3**: "cursed temples" - oppressive religious horror atmosphere
- **Band 4**: "brutal execution sites, shrines defiled" - social horror and desecration
- **Band 5**: "nightmare temples, writhing portals, impossible architecture" - cosmic horror

### Our Forge System (from docs/Themes.md):
**Key Focus**: Dungeons provide "sentimental items" for forge trials
**Light Path**: Holy relics and blessed materials
**Dark Path**: Cursed artifacts and blood-soaked items
**Companion Trials**: Dungeons test companion bonds and loyalty

### Rich Literary Context for Gothic Horror Atmosphere:
{}

## Source Material to Transform
You have {} complex D&D dungeon samples with:
- Multiple room descriptions with stat blocks
- Complex encounter tables and treasure distributions
- D&D-style area connections and navigation rules
- Detailed monster stat blocks with challenge ratings

## Your Task: Extract and Transform
For each dungeon sample, focus on HORROR ATMOSPHERE and FORGE MATERIALS:

1. **Map Dungeon Type**: Crypt/cave/temple/lair based on horror theme, not D&D mechanics
2. **Assess Horror Impact**: What corruption band (1-5) does this dungeon represent?
3. **Extract Emotional Themes**: How does this dungeon affect companion trauma?
4. **Identify Forge Materials**: What sentimental/cursed items fit our light/dark forge paths?
5. **Transform Encounters**: Convert D&D monsters to horror-appropriate threats
6. **Simplify Areas**: Key environmental features, not complex room-by-room maps

## Critical Transformation Rules:
- **NO complex room maps** or D&D-style area connections
- **NO stat blocks** (AC, HP, spell lists, challenge ratings)
- **YES atmospheric horror** descriptions fitting our 5-band progression
- **YES companion trauma** triggers and emotional impact
- **YES forge materials** (sentimental items for mythic gear creation)
- **Transform monsters** to medieval horror threats (no D&D creatures)
- **Focus on horror progression** appropriate for our inverted power curve

## Output Format
Return as JSON array of DungeonSeed objects focusing on horror atmosphere and forge materials.

## Source Samples:
{}

Transform these complex D&D dungeons into horror sites appropriate for Dragon's Labyrinth's forge and trauma systems.
"#, 
            books_toml.unwrap_or("No book context available"),
            samples.sample_count,
            serde_json::to_string_pretty(samples).unwrap_or_else(|_| "Failed to serialize samples".to_string())
        )
    }
}
