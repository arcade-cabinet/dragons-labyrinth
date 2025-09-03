//! Dialogue source data module - provides raw templates and patterns
//! 
//! This module only provides source data. Actual dialogue generation happens in:
//! - dl_analysis: extracts patterns from books, analyzes character archetypes
//! - dl_processors: generates YarnSpinner using AI prompts

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Manager for dialogue source data and templates
#[derive(Debug, Clone)]
pub struct DialogueSourceManager {
    pub cache_dir: std::path::PathBuf,
    pub character_archetypes: Vec<CharacterArchetype>,
    pub trait_templates: Vec<TraitTemplate>,
}

impl DialogueSourceManager {
    /// Initialize dialogue source data
    pub fn initialize(dialogue_dir: &Path) -> Result<Self> {
        std::fs::create_dir_all(dialogue_dir)?;
        
        // Define character archetypes for mercenary/companion generation
        let character_archetypes = vec![
            CharacterArchetype {
                archetype_type: "mercenary".to_string(),
                alignment: "neutral".to_string(),
                traits: vec!["pragmatic".to_string(), "skilled".to_string(), "cynical".to_string()],
                motivations: vec!["gold".to_string(), "survival".to_string(), "reputation".to_string()],
                speech_patterns: vec!["terse".to_string(), "professional".to_string()],
            },
            CharacterArchetype {
                archetype_type: "holy_warrior".to_string(),
                alignment: "light".to_string(),
                traits: vec!["righteous".to_string(), "brave".to_string(), "stubborn".to_string()],
                motivations: vec!["justice".to_string(), "protection".to_string(), "faith".to_string()],
                speech_patterns: vec!["formal".to_string(), "inspiring".to_string()],
            },
            CharacterArchetype {
                archetype_type: "dark_cultist".to_string(),
                alignment: "dark".to_string(),
                traits: vec!["secretive".to_string(), "manipulative".to_string(), "knowledgeable".to_string()],
                motivations: vec!["power".to_string(), "forbidden_knowledge".to_string(), "chaos".to_string()],
                speech_patterns: vec!["cryptic".to_string(), "unsettling".to_string()],
            },
            CharacterArchetype {
                archetype_type: "wandering_scholar".to_string(),
                alignment: "neutral".to_string(),
                traits: vec!["curious".to_string(), "analytical".to_string(), "absent_minded".to_string()],
                motivations: vec!["knowledge".to_string(), "discovery".to_string(), "understanding".to_string()],
                speech_patterns: vec!["verbose".to_string(), "academic".to_string()],
            },
            CharacterArchetype {
                archetype_type: "corrupted_noble".to_string(),
                alignment: "dark".to_string(),
                traits: vec!["arrogant".to_string(), "desperate".to_string(), "haunted".to_string()],
                motivations: vec!["restoration".to_string(), "revenge".to_string(), "legacy".to_string()],
                speech_patterns: vec!["aristocratic".to_string(), "bitter".to_string()],
            },
        ];
        
        // Define trait templates for achievement-based character development
        let trait_templates = vec![
            TraitTemplate {
                trait_name: "battle_hardened".to_string(),
                requirements: vec!["survived_10_battles".to_string()],
                dialogue_modifiers: vec!["adds_war_stories".to_string(), "skeptical_of_peace".to_string()],
                stat_modifiers: vec![("courage".to_string(), 0.2), ("cynicism".to_string(), 0.1)],
            },
            TraitTemplate {
                trait_name: "void_touched".to_string(),
                requirements: vec!["entered_void_zone".to_string()],
                dialogue_modifiers: vec!["speaks_in_whispers".to_string(), "mentions_visions".to_string()],
                stat_modifiers: vec![("madness".to_string(), 0.3), ("insight".to_string(), 0.2)],
            },
            TraitTemplate {
                trait_name: "dragon_slayer".to_string(),
                requirements: vec!["killed_dragon".to_string()],
                dialogue_modifiers: vec!["boastful".to_string(), "references_dragon_lore".to_string()],
                stat_modifiers: vec![("reputation".to_string(), 0.5), ("confidence".to_string(), 0.3)],
            },
            TraitTemplate {
                trait_name: "betrayer".to_string(),
                requirements: vec!["betrayed_faction".to_string()],
                dialogue_modifiers: vec!["defensive".to_string(), "justifies_actions".to_string()],
                stat_modifiers: vec![("trust".to_string(), -0.5), ("guilt".to_string(), 0.3)],
            },
            TraitTemplate {
                trait_name: "blessed".to_string(),
                requirements: vec!["completed_holy_quest".to_string()],
                dialogue_modifiers: vec!["hopeful".to_string(), "references_divine".to_string()],
                stat_modifiers: vec![("faith".to_string(), 0.4), ("light_affinity".to_string(), 0.3)],
            },
        ];
        
        // Save manifest
        let manifest = DialogueSourceManifest {
            version: "1.0.0".to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
            archetypes: character_archetypes.len(),
            traits: trait_templates.len(),
        };
        
        std::fs::write(
            dialogue_dir.join("manifest.json"),
            serde_json::to_string_pretty(&manifest)?
        )?;
        
        Ok(Self {
            cache_dir: dialogue_dir.to_path_buf(),
            character_archetypes,
            trait_templates,
        })
    }
    
    /// Load from existing cache
    pub fn load_from_cache(dialogue_dir: &Path) -> Result<Self> {
        Self::initialize(dialogue_dir)
    }
}

/// Character archetype for NPC generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterArchetype {
    pub archetype_type: String,
    pub alignment: String,
    pub traits: Vec<String>,
    pub motivations: Vec<String>,
    pub speech_patterns: Vec<String>,
}

/// Trait template for achievement-based character development
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraitTemplate {
    pub trait_name: String,
    pub requirements: Vec<String>,
    pub dialogue_modifiers: Vec<String>,
    pub stat_modifiers: Vec<(String, f32)>,
}

/// Quest pattern extracted from literature analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestPattern {
    pub pattern_type: String,
    pub beats: Vec<String>,
    pub themes: Vec<String>,
    pub sources: Vec<String>,
}

/// Manifest for tracking dialogue source data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueSourceManifest {
    pub version: String,
    pub created_at: String,
    pub archetypes: usize,
    pub traits: usize,
}
