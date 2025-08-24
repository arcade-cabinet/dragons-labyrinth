//! Prompt optimization and token management using tiktoken
//!
//! Handles caching, checkpointing, and efficient prompt construction

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tiktoken_rs::{cl100k_base, CoreBPE};

/// Manages prompt optimization and caching
pub struct PromptOptimizer {
    tokenizer: CoreBPE,
    cache_dir: PathBuf,
    checkpoints: HashMap<String, GenerationCheckpoint>,
    token_budget: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationCheckpoint {
    pub id: String,
    pub prompt_hash: String,
    pub generated_content: String,
    pub tokens_used: usize,
    pub timestamp: i64,
    pub context: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptTemplate {
    pub name: String,
    pub base_template: String,
    pub variables: Vec<String>,
    pub estimated_tokens: usize,
}

impl PromptOptimizer {
    pub fn new(cache_dir: &Path) -> Result<Self> {
        std::fs::create_dir_all(cache_dir)?;
        
        Ok(Self {
            tokenizer: cl100k_base()?,
            cache_dir: cache_dir.to_path_buf(),
            checkpoints: HashMap::new(),
            token_budget: 100_000, // Conservative budget for gpt-4
        })
    }
    
    /// Count tokens in a text
    pub fn count_tokens(&self, text: &str) -> usize {
        self.tokenizer.encode_with_special_tokens(text).len()
    }
    
    /// Optimize a prompt to fit within token budget
    pub fn optimize_prompt(&self, prompt: &str, context: &str) -> Result<String> {
        let prompt_tokens = self.count_tokens(prompt);
        let context_tokens = self.count_tokens(context);
        
        if prompt_tokens + context_tokens > self.token_budget {
            // Need to compress
            Ok(self.compress_prompt(prompt, context)?)
        } else {
            Ok(format!("{}\n\n{}", context, prompt))
        }
    }
    
    /// Compress prompt using various strategies
    fn compress_prompt(&self, prompt: &str, context: &str) -> Result<String> {
        // Strategy 1: Use references instead of full text
        let compressed = self.replace_with_references(prompt);
        
        // Strategy 2: Summarize context
        let summarized_context = self.summarize_context(context);
        
        // Strategy 3: Use templates
        let templated = self.apply_template(&compressed);
        
        Ok(format!("{}\n\n{}", summarized_context, templated))
    }
    
    fn replace_with_references(&self, text: &str) -> String {
        // Replace common repeated phrases with references
        let mut result = text.to_string();
        
        // Common replacements to save tokens
        let replacements = [
            ("Dragon's Labyrinth", "[DL]"),
            ("YarnSpinner format", "[YARN]"),
            ("dread level", "[DREAD]"),
            ("companion trust", "[TRUST]"),
            ("moral complexity", "[MORAL]"),
        ];
        
        for (long, short) in replacements {
            result = result.replace(long, short);
        }
        
        result
    }
    
    fn summarize_context(&self, context: &str) -> String {
        // Keep only essential context
        let lines: Vec<&str> = context.lines().collect();
        if lines.len() > 10 {
            format!("Context (summarized):\n{}", lines[0..10].join("\n"))
        } else {
            context.to_string()
        }
    }
    
    fn apply_template(&self, text: &str) -> String {
        // Apply common structure templates to reduce tokens
        if text.contains("Generate") && text.contains("dialogue") {
            // Use shorthand for dialogue generation
            format!("[DIALOGUE_GEN]\n{}", text.replace("Generate a dialogue tree", "[D]"))
        } else if text.contains("quest") {
            // Use shorthand for quest generation
            format!("[QUEST_GEN]\n{}", text.replace("Generate a quest", "[Q]"))
        } else {
            text.to_string()
        }
    }
    
    /// Save a checkpoint for resuming generation
    pub fn save_checkpoint(&mut self, checkpoint: GenerationCheckpoint) -> Result<()> {
        let path = self.cache_dir.join(format!("{}.json", checkpoint.id));
        std::fs::write(&path, serde_json::to_string_pretty(&checkpoint)?)?;
        self.checkpoints.insert(checkpoint.id.clone(), checkpoint);
        Ok(())
    }
    
    /// Load checkpoint if it exists
    pub fn load_checkpoint(&mut self, id: &str) -> Result<Option<GenerationCheckpoint>> {
        if let Some(checkpoint) = self.checkpoints.get(id) {
            return Ok(Some(checkpoint.clone()));
        }
        
        let path = self.cache_dir.join(format!("{}.json", id));
        if path.exists() {
            let content = std::fs::read_to_string(&path)?;
            let checkpoint: GenerationCheckpoint = serde_json::from_str(&content)?;
            self.checkpoints.insert(id.to_string(), checkpoint.clone());
            Ok(Some(checkpoint))
        } else {
            Ok(None)
        }
    }
    
    /// Check if we've already generated content for this prompt
    pub fn check_cache(&self, prompt_hash: &str) -> Option<String> {
        for checkpoint in self.checkpoints.values() {
            if checkpoint.prompt_hash == prompt_hash {
                return Some(checkpoint.generated_content.clone());
            }
        }
        None
    }
    
    /// Batch multiple prompts efficiently
    pub fn batch_prompts(&self, prompts: Vec<String>) -> Vec<String> {
        let mut batches = Vec::new();
        let mut current_batch = String::new();
        let mut current_tokens = 0;
        
        for (i, prompt) in prompts.iter().enumerate() {
            let prompt_with_marker = format!("[PROMPT_{}]\n{}\n[/PROMPT_{}]\n", i, prompt, i);
            let tokens = self.count_tokens(&prompt_with_marker);
            
            if current_tokens + tokens > self.token_budget {
                // Start new batch
                if !current_batch.is_empty() {
                    batches.push(current_batch);
                }
                current_batch = prompt_with_marker;
                current_tokens = tokens;
            } else {
                current_batch.push_str(&prompt_with_marker);
                current_tokens += tokens;
            }
        }
        
        if !current_batch.is_empty() {
            batches.push(current_batch);
        }
        
        batches
    }
    
    /// Generate templates for common patterns
    pub fn create_template(&self, name: &str, template: &str) -> PromptTemplate {
        PromptTemplate {
            name: name.to_string(),
            base_template: template.to_string(),
            variables: self.extract_variables(template),
            estimated_tokens: self.count_tokens(template),
        }
    }
    
    fn extract_variables(&self, template: &str) -> Vec<String> {
        let mut vars = Vec::new();
        for cap in regex::Regex::new(r"\{(\w+)\}").unwrap().captures_iter(template) {
            if let Some(var) = cap.get(1) {
                vars.push(var.as_str().to_string());
            }
        }
        vars
    }
}

/// Templates for common narrative generation patterns
pub mod templates {
    use super::*;
    
    pub fn dialogue_template() -> PromptTemplate {
        PromptTemplate {
            name: "dialogue".to_string(),
            base_template: r#"Generate dialogue node:
Character: {character}
Archetype: {archetype}
Dread: {dread}
Context: {context}

Return as JSON:
{
  "speaker": "{character}",
  "text": "...",
  "emotion": "...",
  "choices": [...]
}"#.to_string(),
            variables: vec!["character".to_string(), "archetype".to_string(), "dread".to_string(), "context".to_string()],
            estimated_tokens: 100,
        }
    }
    
    pub fn quest_template() -> PromptTemplate {
        PromptTemplate {
            name: "quest".to_string(),
            base_template: r#"Generate quest structure:
Type: {quest_type}
Moral: {moral_weight}
Dread: {dread}

Return spanning tree with:
- Root node (quest start)
- Choice nodes
- Outcome nodes
- Edges with conditions"#.to_string(),
            variables: vec!["quest_type".to_string(), "moral_weight".to_string(), "dread".to_string()],
            estimated_tokens: 80,
        }
    }
}
