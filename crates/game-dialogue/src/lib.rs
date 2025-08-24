//! AI-powered narrative content generation for Dragon's Labyrinth
//!
//! This crate handles all AI-generated narrative content:
//! - Dialogue trees (YarnSpinner format)
//! - Quest chains with moral complexity
//! - Companion relationship dynamics
//! - Story graphs (Cobweb format)
//!
//! Key Architecture:
//! 1. AI generates primitive structures (spanning trees, graphs)
//! 2. We convert those to YarnSpinner/Cobweb formats
//! 3. Token optimization via tiktoken
//! 4. Caching and checkpointing for expensive operations

pub mod dialogue;
pub mod quests;
pub mod relationships;
pub mod narrative_graph;
pub mod openai_client;
pub mod prompt_optimizer;
pub mod trait_aware;

use anyhow::Result;
use std::path::Path;
use game_content_static::CompanionArchetype;

pub use dialogue::{DialogueGenerator, DialogueContext};
pub use quests::{QuestGenerator, QuestType, MoralComplexity};
pub use narrative_graph::{NarrativeTree, NarrativeNode, NodeContent};
pub use prompt_optimizer::PromptOptimizer;

/// Master content generator that orchestrates all narrative generation
pub struct ContentGenerator {
    dialogue_gen: DialogueGenerator,
    quest_gen: QuestGenerator,
    output_dir: std::path::PathBuf,
}

impl ContentGenerator {
    pub fn new(output_dir: &Path) -> Result<Self> {
        std::fs::create_dir_all(output_dir)?;
        
        Ok(Self {
            dialogue_gen: DialogueGenerator::new()?,
            quest_gen: QuestGenerator::new()?,
            output_dir: output_dir.to_path_buf(),
        })
    }
    
    /// Generate all narrative content for the game
    pub async fn generate_all_content(&mut self) -> Result<GenerationReport> {
        let mut report = GenerationReport::default();
        
        println!("Generating narrative content...");
        println!("This will use AI to create dialogue trees and quest chains.");
        
        // Generate dialogue for all companions
        println!("\n=== Generating Companion Dialogues ===");
        for archetype in CompanionArchetype::all() {
            println!("Generating dialogue for {:?}...", archetype);
            
            for dread_level in 0..=4 {
                let result = self.dialogue_gen.generate_companion_set(
                    archetype,
                    dread_level,
                    &self.output_dir.join("dialogues")
                ).await?;
                
                report.dialogues_generated += result.files_created;
                report.total_tokens_used += result.tokens_used;
                
                println!("  Dread {}: {} files, {} tokens", 
                    dread_level, result.files_created, result.tokens_used);
            }
        }
        
        // Generate quest chains for each dread level
        println!("\n=== Generating Quest Chains ===");
        for dread_level in 0..=4 {
            println!("Generating quests for dread level {}...", dread_level);
            
            let result = self.quest_gen.generate_quest_chain(
                dread_level,
                &self.output_dir.join("quests")
            ).await?;
            
            report.quests_generated += result.quests_created;
            report.total_tokens_used += result.tokens_used;
            
            println!("  Created {} quests, {} tokens", 
                result.quests_created, result.tokens_used);
        }
        
        println!("\n=== Generation Complete ===");
        println!("{}", report.summary());
        
        Ok(report)
    }
}

#[derive(Debug, Default)]
pub struct GenerationReport {
    pub dialogues_generated: usize,
    pub quests_generated: usize,
    pub relationships_generated: usize,
    pub story_nodes_generated: usize,
    pub total_tokens_used: usize,
    pub errors: Vec<String>,
}

impl GenerationReport {
    pub fn summary(&self) -> String {
        format!(
            r#"Content Generation Report:
- Dialogues: {} files
- Quests: {} files
- Relationships: {} files
- Story Nodes: {} nodes
- Total Tokens: {} (~${:.2} at GPT-4 rates)
- Errors: {}
"#,
            self.dialogues_generated,
            self.quests_generated,
            self.relationships_generated,
            self.story_nodes_generated,
            self.total_tokens_used,
            self.total_tokens_used as f64 * 0.00003, // Rough GPT-4 pricing
            if self.errors.is_empty() { "None".to_string() } else { format!("{} errors", self.errors.len()) }
        )
    }
}