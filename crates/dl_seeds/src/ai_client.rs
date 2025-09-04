//! Reusable AI client extracted from clusters.rs for seed transformation

use anyhow::{Result, Context};
use openai_dive::v1::api::Client;
use openai_dive::v1::resources::chat::{ChatCompletionParameters, ChatMessage, ChatMessageContent};
use serde_json::Value;
use std::env;

/// AI-powered seed transformation client
pub struct SeedAiClient {
    client: Client,
    model: String,
}

impl SeedAiClient {
    /// Initialize AI client with API key
    pub fn new() -> Result<Self> {
        let api_key = env::var("OPENAI_API_KEY")
            .context("OPENAI_API_KEY environment variable not set")?;
        
        let client = Client::new(api_key);
        let model = env::var("OPENAI_MODEL")
            .unwrap_or_else(|_| "gpt-4o".to_string());
        
        Ok(Self { client, model })
    }

    /// Transform TOML samples into structured seeds using comprehensive AI analysis
    pub async fn transform_samples_to_seeds(
        &self,
        transformation_prompt: &str,
    ) -> Result<Value> {
        let parameters = ChatCompletionParameters {
            model: self.model.clone(),
            messages: vec![
                ChatMessage::System {
                    content: ChatMessageContent::Text("You are an expert game designer transforming D&D content into horror RPG seeds. Return only valid JSON.".to_string()),
                    name: None,
                },
                ChatMessage::User {
                    content: ChatMessageContent::Text(transformation_prompt.to_string()),
                    name: None,
                },
            ],
            temperature: Some(0.1), // Low temperature for consistent analysis
            max_tokens: Some(4000),
            ..Default::default()
        };

        let result = self.client.chat().create(parameters).await
            .context("Failed to transform seeds with OpenAI")?;

        let response_content = match &result.choices[0].message {
            ChatMessage::Assistant { content: Some(ChatMessageContent::Text(text)), .. } => text,
            _ => return Err(anyhow::anyhow!("Unexpected message type from OpenAI")),
        };

        let seeds_json: Value = serde_json::from_str(response_content)
            .context("Failed to parse OpenAI seeds response as JSON")?;

        Ok(seeds_json)
    }
}
