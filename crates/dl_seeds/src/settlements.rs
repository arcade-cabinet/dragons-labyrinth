//! Settlements seeding module - AI-powered transformation using rich contextual seeds
//! 
//! Uses comprehensive AI prompts with book excerpts + hexroll samples + our themes
//! to transform complex tavern/NPC data into Dragon's Labyrinth settlement seeds.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;
use crate::entities::CategorySamples;

/// Dragon's Labyrinth settlement seed data (AI-generated)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettlementSeed {
    pub name: String,
    pub settlement_type: String,
    pub corruption_level: u8,
    pub population_state: String,
    pub key_npcs: Vec<SimplifiedNPC>,
    pub available_services: Vec<String>,
    pub faction_loyalties: Vec<String>,
    pub quest_hooks: Vec<String>,
    pub thematic_description: String,
}

/// Simplified NPC for game use (not D&D stat blocks)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimplifiedNPC {
    pub name: String,
    pub role: String,           // innkeeper, blacksmith, etc.
    pub emotional_state: String, // for companion psychology system
    pub potential_companion: bool,
    pub faction_allegiance: Option<String>,
}

/// Collection of settlement seeds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettlementSeeds {
    pub settlements: Vec<SettlementSeed>,
    pub generated_from_samples: Vec<String>,
}

impl SettlementSeeds {
    /// Use AI to transform TOML samples with rich contextual seeds
    pub fn generate_from_toml(out_dir: &Path) -> Result<Self> {
        let toml_path = out_dir.join("settlements.toml");
        let books_path = out_dir.join("books.toml");
        
        if !toml_path.exists() {
            return Err(anyhow::anyhow!("settlements.toml not found in {}", out_dir.display()));
        }
        
        // Load TOML samples and books
        let toml_content = std::fs::read_to_string(&toml_path)?;
        let samples: CategorySamples = toml::from_str(&toml_content)?;
        
        let books_content = if books_path.exists() {
            Some(std::fs::read_to_string(&books_path)?)
        } else {
            None
        };
        
        println!("Transforming {} settlement samples using AI with rich contextual seeds...", samples.sample_count);
        
        // Use AI with comprehensive context to transform samples
        let settlements = Self::ai_transform_settlements(&samples, books_content.as_deref())?;
        let sample_names: Vec<String> = samples.entities.iter().map(|e| e.entity_name.clone()).collect();
        
        Ok(Self {
            settlements,
            generated_from_samples: sample_names,
        })
    }
    
    /// Comprehensive AI prompt for settlement transformation
    fn ai_transform_settlements(samples: &CategorySamples, books_toml: Option<&str>) -> Result<Vec<SettlementSeed>> {
        use crate::ai_client::SeedAiClient;
        use tokio::runtime::Runtime;
        
        let rt = Runtime::new()?;
        let ai_client = SeedAiClient::new()?;
        let ai_prompt = Self::create_comprehensive_transformation_prompt(samples, books_toml);
        
        let seeds_json = rt.block_on(async {
            ai_client.transform_samples_to_seeds(&ai_prompt).await
        })?;
        
        // Parse AI response into SettlementSeed structs
        let settlements: Vec<SettlementSeed> = serde_json::from_value(seeds_json)?;
        Ok(settlements)
    }
    
    /// Create comprehensive AI transformation prompt for settlements
    fn create_comprehensive_transformation_prompt(samples: &CategorySamples, books_toml: Option<&str>) -> String {
        format!(r#"
# Dragon's Labyrinth Settlement Transformation

## Your Role
You are transforming complex D&D settlement data (taverns with full NPC stat blocks) into simple settlement seeds for "Dragon's Labyrinth" - a horror RPG where companion psychology and trauma matter more than combat stats.

## Rich Contextual Seeds Available

### Our Game's Themes (from docs/Themes.md):
**Settlement Evolution by Corruption Band:**
- **Band 1**: "welcoming villages" with "cozy taverns, blacksmiths, shrines, farmsteads"
- **Band 2**: "abandoned shops, damaged temples, haunted ruins" 
- **Band 3**: "ruined fortresses, burned villages, cursed temples"
- **Band 4**: "war camps, raider-occupied villages, brutal executions, shrines defiled"
- **Band 5**: "nightmare temples, writhing portals, haunted ruins, impossible architecture"

### Our Companion Psychology System (from docs/Architecture.md):
**Key Focus**: "Deep trauma mechanics where relationships matter more than stats"
**NPC Emotional States**: Track trauma, loyalty, breaking points
**No D&D Stats**: Simple emotional/social data instead of ability scores

### Rich Literary Context for Authentic Medieval Horror:
{}

## Source Material to Transform
You have {} complex D&D settlement samples with:
- Detailed taverns with food menus and pricing
- Full NPC stat blocks with levels, classes, ability scores  
- Complex faction memberships hidden in spoiler tags
- D&D-style quest bulletins and rumor tables

## Your Task: Extract and Transform
For each settlement sample, focus on the SOCIAL and EMOTIONAL elements for our companion system:

1. **Simplify Settlement Type**: Village/town/city/outpost based on complexity, not D&D mechanics
2. **Extract NPC Personalities**: Focus on emotional states (fearful, worried, pained) for trauma system
3. **Identify Potential Companions**: NPCs with interesting psychology, not high combat stats  
4. **Map Faction Politics**: Extract hidden allegiances for political intrigue
5. **Convert Quest Hooks**: Transform complex bulletin boards into simple story threads
6. **Assess Corruption Impact**: How does horror progression affect this settlement?

## Critical Transformation Rules:
- **NO D&D stat blocks** (STR/DEX/CON/INT/WIS/CHA, AC, HP, spell lists)
- **NO complex pricing/inventory** mechanics 
- **YES emotional states** for companion psychology
- **YES faction politics** for social intrigue
- **YES simple services** (lodging, healing, trading)
- **Transform creatures** to fit medieval horror (no D&D monsters)
- **Focus on trauma/hope** dynamics appropriate for horror RPG

## Output Format
Return as JSON array of SettlementSeed objects focusing on social/emotional content.

## Source Samples:
{}

Transform these complex D&D taverns into settlements appropriate for Dragon's Labyrinth's companion psychology system.
"#, 
            books_toml.unwrap_or("No book context available"),
            samples.sample_count,
            serde_json::to_string_pretty(samples).unwrap_or_else(|_| "Failed to serialize samples".to_string())
        )
    }
}
