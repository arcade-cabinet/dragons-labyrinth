//! Factions seeding module - AI-powered transformation using rich contextual seeds
//! 
//! Uses comprehensive AI prompts with book excerpts + hexroll samples + our themes
//! to transform D&D faction data into Dragon's Labyrinth political intrigue content.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;
use crate::entities::{SampleEntity, CategorySamples};

/// Dragon's Labyrinth faction seed data (AI-generated)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactionSeed {
    pub name: String,
    pub faction_type: String,
    pub corruption_band: u8,
    pub political_philosophy: String,
    pub key_members: Vec<FactionMember>,
    pub territorial_claims: Vec<String>,
    pub alliance_dynamics: Vec<String>,
    pub companion_impact: String, // how faction affects companion psychology
    pub forge_alignment: String, // light/dark/neutral for forge system
    pub thematic_description: String,
}

/// Simplified faction member (not D&D stat blocks)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactionMember {
    pub name: String,
    pub role: String,
    pub emotional_state: String,
    pub loyalty_level: String,
    pub potential_companion: bool,
}

/// Collection of faction seeds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactionSeeds {
    pub factions: Vec<FactionSeed>,
    pub generated_from_samples: Vec<String>,
}

impl FactionSeeds {
    /// Use AI to transform TOML samples with rich contextual seeds
    pub fn generate_from_toml(out_dir: &Path) -> Result<Self> {
        let toml_path = out_dir.join("factions.toml");
        let books_path = out_dir.join("books.toml");
        
        if !toml_path.exists() {
            return Err(anyhow::anyhow!("factions.toml not found in {}", out_dir.display()));
        }
        
        // Load TOML samples and books
        let toml_content = std::fs::read_to_string(&toml_path)?;
        let samples: CategorySamples = toml::from_str(&toml_content)?;
        
        let books_content = if books_path.exists() {
            Some(std::fs::read_to_string(&books_path)?)
        } else {
            None
        };
        
        println!("Transforming {} faction samples using AI with rich contextual seeds...", samples.sample_count);
        
        // Use AI with comprehensive context to transform samples
        let factions = Self::ai_transform_factions(&samples, books_content.as_deref())?;
        let sample_names: Vec<String> = samples.entities.iter().map(|e| e.entity_name.clone()).collect();
        
        Ok(Self {
            factions,
            generated_from_samples: sample_names,
        })
    }
    
    /// Comprehensive AI prompt for faction transformation
    fn ai_transform_factions(samples: &CategorySamples, books_toml: Option<&str>) -> Result<Vec<FactionSeed>> {
        use crate::ai_client::SeedAiClient;
        use tokio::runtime::Runtime;
        
        let rt = Runtime::new()?;
        let ai_client = SeedAiClient::new()?;
        let ai_prompt = Self::create_comprehensive_transformation_prompt(samples, books_toml);
        
        let seeds_json = rt.block_on(async {
            ai_client.transform_samples_to_seeds(&ai_prompt).await
        })?;
        
        // Parse AI response into FactionSeed structs
        let factions: Vec<FactionSeed> = serde_json::from_value(seeds_json)?;
        Ok(factions)
    }
    
    /// Create comprehensive AI transformation prompt for factions
    fn create_comprehensive_transformation_prompt(samples: &CategorySamples, books_toml: Option<&str>) -> String {
        format!(r#"
# Dragon's Labyrinth Faction Transformation

## Your Role
You are transforming D&D faction data into political intrigue seeds for "Dragon's Labyrinth" - a horror RPG where companion psychology and moral choices drive the narrative.

## Rich Contextual Seeds Available

### Our Game's Themes (from docs/Themes.md):
**Political Evolution by Corruption Band:**
- **Band 1**: Basic faction rivalries and traditional conflicts
- **Band 2**: Political decay and failing alliances
- **Band 3**: Faction militarization and territorial wars
- **Band 4**: "social apocalypse, Fear, cruelty, betrayal" - factions become brutal
- **Band 5**: Void-corrupted faction remnants and eldritch cults

### Our Companion Psychology System (from docs/Architecture.md):
**Key Focus**: "Deep trauma mechanics where relationships matter more than stats"
**Political Impact**: Faction choices affect companion loyalty and trust
**Moral Choices**: Faction allegiances have permanent consequences

### Our Forge Paths (from docs/Themes.md):
**Light Path**: Factions supporting hope, healing, protection
**Dark Path**: Factions embracing power, sacrifice, domination
**Forge Trials**: Test companion bonds through faction conflicts

### Rich Literary Context for Political Intrigue:
{}

## Source Material to Transform
You have {} D&D faction samples with:
- Complex shop/business data (cobbler, smokehouse, craft)
- NPC stat blocks with faction memberships hidden in spoiler tags
- Tavern-based political connections
- Business-focused faction activities

## Your Task: Extract and Transform
For each faction sample, focus on POLITICAL INTRIGUE and COMPANION IMPACT:

1. **Identify Faction Type**: Military/religious/criminal/mercantile based on activities
2. **Map Political Philosophy**: How does this faction view power, justice, survival?
3. **Extract Key Members**: Focus on emotional states and loyalty dynamics
4. **Assess Companion Impact**: How do faction choices affect companion psychology?
5. **Determine Forge Alignment**: Light/dark/neutral based on faction methods
6. **Map Territorial Claims**: What regions/settlements does this faction control?

## Critical Transformation Rules:
- **NO business inventory** details (cobbler tools, smokehouse goods)
- **NO D&D stat blocks** for faction members
- **YES political philosophy** and power dynamics
- **YES companion psychology** impact and loyalty consequences
- **YES forge path alignment** (light/dark forge material access)
- **Transform members** to focus on emotional states, not combat stats
- **Focus on moral choices** that affect companion relationships

## Output Format
Return as JSON array of FactionSeed objects focusing on political intrigue and companion psychology.

## Source Samples:
{}

Transform these D&D business/faction samples into political entities appropriate for Dragon's Labyrinth's companion psychology system.
"#, 
            books_toml.unwrap_or("No book context available"),
            samples.sample_count,
            serde_json::to_string_pretty(samples).unwrap_or_else(|_| "Failed to serialize samples".to_string())
        )
    }
}
