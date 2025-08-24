//! Dialogue generation using narrative graphs

use crate::narrative_graph::{NarrativeTree, NarrativeNode, NodeContent, NodeMetadata, NarrativeEdge, ChoiceOption, Consequence, ConsequenceType};
use crate::openai_client::OpenAIClient;
use crate::prompt_optimizer::{PromptOptimizer, GenerationCheckpoint};
use anyhow::Result;
use minijinja::Environment;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use game_content_static::characters::{CompanionArchetype, CompanionData};

pub struct DialogueGenerator {
    client: OpenAIClient,
    optimizer: PromptOptimizer,
}

// CompanionArchetype comes from game_content_static::characters
// This ensures we have a single source of truth for character data

#[derive(Serialize, Deserialize, Debug)]
pub enum DialogueContext {
    FirstMeeting,
    CampfireRest,
    EnteringDungeon,
    AfterCombat,
    WitnessingHorror,
    CompanionDeath,
    MoralChoice,
    BreakingDown,
    FinalGoodbye,
}

pub struct DialogueResult {
    pub files_created: usize,
    pub tokens_used: usize,
}

impl DialogueGenerator {
    pub fn new() -> Result<Self> {
        let cache_dir = Path::new("target/content_cache");
        Ok(Self {
            client: OpenAIClient::new()?,
            optimizer: PromptOptimizer::new(cache_dir)?,
        })
    }
    
    pub async fn generate_companion_set(
        &mut self,
        archetype: CompanionArchetype,
        dread_level: u8,
        output_dir: &Path,
    ) -> Result<DialogueResult> {
        std::fs::create_dir_all(output_dir)?;
        
        let mut result = DialogueResult {
            files_created: 0,
            tokens_used: 0,
        };
        
        // Check for cached content
        let cache_key = format!("{:?}_dread_{}", archetype, dread_level);
        if let Some(checkpoint) = self.optimizer.load_checkpoint(&cache_key)? {
            println!("Using cached dialogue for {}", cache_key);
            // Write cached content
            self.write_cached_dialogue(&checkpoint, output_dir)?;
            return Ok(result);
        }
        
        // Generate dialogue trees for each context
        for context in [
            DialogueContext::FirstMeeting,
            DialogueContext::CampfireRest,
            DialogueContext::WitnessingHorror,
            DialogueContext::BreakingDown,
        ] {
            // Generate primitive narrative tree
            let tree = self.generate_narrative_tree(archetype, dread_level, &context).await?;
            
            // Convert to YarnSpinner
            let yarn = tree.to_yarnspinner();
            
            // Save YarnSpinner file
            let data = archetype.data();
            let filename = format!(
                "{}_{}_dread{}_{:?}.yarn",
                data.name.to_lowercase().replace(" ", "_"),
                format!("{:?}", archetype).to_lowercase(),
                dread_level,
                context,
            ).to_lowercase();
            
            let filepath = output_dir.join(&filename);
            std::fs::write(&filepath, &yarn)?;
            
            // Also save the raw tree for debugging/analysis
            let tree_file = filepath.with_extension("json");
            std::fs::write(&tree_file, serde_json::to_string_pretty(&tree)?)?;
            
            result.files_created += 2; // .yarn and .json
            result.tokens_used += self.optimizer.count_tokens(&yarn);
            
            // Save checkpoint
            let checkpoint = GenerationCheckpoint {
                id: format!("{}_{}", cache_key, format!("{:?}", context).to_lowercase()),
                prompt_hash: format!("{:x}", md5::compute(&yarn)),
                generated_content: yarn,
                tokens_used: result.tokens_used,
                timestamp: chrono::Utc::now().timestamp(),
                context: HashMap::from([
                    ("archetype".to_string(), format!("{:?}", archetype)),
                    ("dread".to_string(), dread_level.to_string()),
                    ("context".to_string(), format!("{:?}", context)),
                ]),
            };
            self.optimizer.save_checkpoint(checkpoint)?;
        }
        
        Ok(result)
    }
    
    async fn generate_narrative_tree(
        &self,
        archetype: CompanionArchetype,
        dread_level: u8,
        context: &DialogueContext,
    ) -> Result<NarrativeTree> {
        // Load template from file
        let template_str = std::fs::read_to_string("crates/game-content-generated/templates/dialogue_tree.j2")?;
        let mut env = Environment::new();
        env.add_template("dialogue", &template_str)?;
        let tmpl = env.get_template("dialogue")?;
        
        // Get character data from style guide
        let data = archetype.data();
        
        // Determine emotion state based on dread level
        let emotion_state = match dread_level {
            0 => "calm and collected",
            1 => "slightly uneasy",
            2 => "anxious and stressed",
            3 => "terrified and breaking",
            4 => "completely shattered",
            _ => "confused",
        };
        
        // Render template with character data
        let prompt = tmpl.render(minijinja::context! {
            character_name => data.name,
            archetype => format!("{:?}", archetype),
            personality => data.personality,
            backstory => data.backstory,
            fear => data.fear,
            breaking_point => data.breaking_point,
            dread_level => dread_level,
            context => format!("{:?}", context),
            emotion_state => emotion_state,
        })?;
        
        // Optimize prompt to fit token budget
        let optimized = self.optimizer.optimize_prompt(&prompt, "")?;
        
        // Generate the tree structure
        let json = self.client.generate_json(&optimized).await?;
        let tree: NarrativeTree = serde_json::from_value(json)?;
        
        Ok(tree)
    }
    
    fn write_cached_dialogue(&self, checkpoint: &GenerationCheckpoint, output_dir: &Path) -> Result<()> {
        let filename = format!("{}.yarn", checkpoint.id);
        let filepath = output_dir.join(filename);
        std::fs::write(filepath, &checkpoint.generated_content)?;
        Ok(())
    }
}