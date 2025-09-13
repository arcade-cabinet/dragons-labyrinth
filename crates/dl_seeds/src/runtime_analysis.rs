//! Runtime seed analysis engine for organized JSON pools
//! 
//! This module provides AI-driven analysis of categorized data pools
//! for dynamic seed generation during gameplay.

use anyhow::Result;
use openai_dive::v1::api::Client;
use serde_json::Value;
use std::collections::HashMap;
use std::path::Path;

use crate::data_pools::CategorizedDataPools;

/// Runtime AI analysis engine for seed generation
pub struct SeedAnalysisEngine {
    ai_client: Client,
    categorized_pools: CategorizedDataPools,
    analysis_cache: HashMap<String, AnalysisResult>,
}

impl SeedAnalysisEngine {
    /// Initialize runtime analysis engine with organized data pools
    pub fn new(pools_dir: &Path) -> Result<Self> {
        let api_key = std::env::var("OPENAI_API_KEY")
            .map_err(|_| anyhow::anyhow!("OPENAI_API_KEY environment variable not set"))?;
            
        let ai_client = Client::new(api_key);
        let categorized_pools = CategorizedDataPools::load_from_dir(pools_dir)?;
        
        Ok(Self {
            ai_client,
            categorized_pools,
            analysis_cache: HashMap::new(),
        })
    }

    /// Analyze organized data pools for a specific category
    pub async fn analyze_pools(&mut self, category: &str) -> Result<AnalysisResult> {
        // Check cache first
        if let Some(cached) = self.analysis_cache.get(category) {
            return Ok(cached.clone());
        }

        // Get pool data for category
        let pool_data = self.categorized_pools.get_category_data(category)?;
        
        // Run AI analysis on the organized data
        let analysis = self.run_ai_analysis(category, pool_data).await?;
        
        // Cache the result
        self.analysis_cache.insert(category.to_string(), analysis.clone());
        
        Ok(analysis)
    }

    /// Generate game seeds from analyzed pools
    pub async fn generate_game_seeds(&mut self, category: &str, count: usize) -> Result<Vec<GameSeed>> {
        let analysis = self.analyze_pools(category).await?;
        let seeds = self.synthesize_seeds(&analysis, count).await?;
        Ok(seeds)
    }

    /// Analyze patterns in organized data pools using AI
    async fn run_ai_analysis(&self, category: &str, data: &[Value]) -> Result<AnalysisResult> {
        use openai_dive::v1::resources::chat::{ChatCompletionParameters, ChatMessage, ChatMessageContent};
        
        // Create analysis prompt
        let system_prompt = format!(
            "You are an expert game data analyst for Dragon's Labyrinth, a horror RPG. \
             Analyze the organized {} data pools to identify patterns, themes, and seed generation potential.",
            category
        );
        
        let user_prompt = format!(
            "Analyze this organized data pool for {} and identify:\n\
             1. Common patterns and structures\n\
             2. Thematic elements for horror progression\n\
             3. Seed generation potential (0.0-1.0 scale)\n\
             4. Key themes for the 5-band corruption system\n\n\
             Data: {}",
            category,
            serde_json::to_string_pretty(data)?
        );

        let parameters = ChatCompletionParameters {
            model: "gpt-4o".to_string(),
            messages: vec![
                ChatMessage::System {
                    content: ChatMessageContent::Text(system_prompt),
                    name: None,
                },
                ChatMessage::User {
                    content: ChatMessageContent::Text(user_prompt),
                    name: None,
                },
            ],
            temperature: Some(0.3),
            max_tokens: Some(2000),
            ..Default::default()
        };

        let result = self.ai_client.chat().create(parameters).await?;
        
        // Parse AI response into analysis result
        let response_text = match &result.choices[0].message {
            ChatMessage::Assistant { content: Some(ChatMessageContent::Text(text)), .. } => text,
            _ => return Err(anyhow::anyhow!("Unexpected AI response format")),
        };

        // For now, create a basic analysis result
        // TODO: Parse structured response from AI
        Ok(AnalysisResult {
            category: category.to_string(),
            patterns: vec!["pattern1".to_string(), "pattern2".to_string()], // TODO: Parse from AI
            themes: vec!["horror".to_string(), "corruption".to_string()], // TODO: Parse from AI
            seed_potential: 0.8, // TODO: Parse from AI
            ai_summary: Some(response_text.clone()),
        })
    }

    /// Synthesize game seeds from analysis results
    async fn synthesize_seeds(&self, analysis: &AnalysisResult, count: usize) -> Result<Vec<GameSeed>> {
        let mut seeds = Vec::new();
        
        // Generate seeds based on analysis patterns and themes
        for i in 0..count {
            let seed = GameSeed {
                id: format!("{}-seed-{}", analysis.category, i),
                category: analysis.category.clone(),
                data: serde_json::json!({
                    "patterns": &analysis.patterns,
                    "themes": &analysis.themes,
                    "index": i,
                    "source_analysis": &analysis.ai_summary
                }),
                confidence: analysis.seed_potential * 0.9, // Slightly reduce confidence for generated seeds
            };
            seeds.push(seed);
        }
        
        Ok(seeds)
    }

    /// Clear analysis cache
    pub fn clear_cache(&mut self) {
        self.analysis_cache.clear();
    }

    /// Get cached analysis for debugging
    pub fn get_cached_analysis(&self, category: &str) -> Option<&AnalysisResult> {
        self.analysis_cache.get(category)
    }
}

/// Result of AI analysis on organized data pools
#[derive(Debug, Clone)]
pub struct AnalysisResult {
    pub category: String,
    pub patterns: Vec<String>,
    pub themes: Vec<String>,
    pub seed_potential: f32,
    pub ai_summary: Option<String>,
}

/// Generated game seed from runtime analysis
#[derive(Debug, Clone)]
pub struct GameSeed {
    pub id: String,
    pub category: String,
    pub data: Value,
    pub confidence: f32,
}

impl GameSeed {
    /// Check if this seed meets minimum confidence threshold
    pub fn is_viable(&self, min_confidence: f32) -> bool {
        self.confidence >= min_confidence
    }

    /// Get seed data as a specific type
    pub fn get_typed_data<T>(&self) -> Result<T> 
    where
        T: serde::de::DeserializeOwned,
    {
        serde_json::from_value(self.data.clone())
            .map_err(|e| anyhow::anyhow!("Failed to deserialize seed data: {}", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_analysis_result_creation() {
        let result = AnalysisResult {
            category: "regions".to_string(),
            patterns: vec!["swamp".to_string(), "forest".to_string()],
            themes: vec!["decay".to_string(), "corruption".to_string()],
            seed_potential: 0.85,
            ai_summary: Some("Test summary".to_string()),
        };
        
        assert_eq!(result.category, "regions");
        assert_eq!(result.seed_potential, 0.85);
    }

    #[test]
    fn test_game_seed_viability() {
        let seed = GameSeed {
            id: "test-seed".to_string(),
            category: "regions".to_string(),
            data: serde_json::json!({"test": "data"}),
            confidence: 0.7,
        };
        
        assert!(seed.is_viable(0.5));
        assert!(!seed.is_viable(0.8));
    }
}
