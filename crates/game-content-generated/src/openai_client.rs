//! OpenAI client for narrative generation using openai_dive
//!
//! This handles all OpenAI API interactions for content generation

use anyhow::Result;
use openai_dive::v1::api::Client;
use openai_dive::v1::resources::chat::{
    ChatCompletionParameters, ChatMessage, ChatMessageContent
};
use serde_json::Value;

pub struct OpenAIClient {
    client: Client,
}

impl OpenAIClient {
    pub fn new() -> Result<Self> {
        let api_key = std::env::var("OPENAI_API_KEY")
            .expect("OPENAI_API_KEY must be set for narrative generation");
        
        let client = Client::new(api_key);
        
        Ok(Self { client })
    }
    
    /// Generate narrative content with optimized prompting
    pub async fn generate(&self, prompt: &str, system_prompt: Option<&str>) -> Result<String> {
        let mut messages = vec![];
        
        if let Some(system) = system_prompt {
            messages.push(ChatMessage::System {
                content: ChatMessageContent::Text(system.to_string()),
                name: None,
            });
        }
        
        messages.push(ChatMessage::User {
            content: ChatMessageContent::Text(prompt.to_string()),
            name: None,
        });
        
        let parameters = ChatCompletionParameters {
            model: "gpt-4o".to_string(),
            messages,
            temperature: Some(0.8), // Creative but not too random
            max_tokens: Some(2000),
            ..Default::default()
        };
        
        let response = self.client.chat().create(parameters).await?;
        
        // Extract the generated text
        if let Some(choice) = response.choices.first() {
            match &choice.message {
                ChatMessage::Assistant { content: Some(content), .. } => {
                    match content {
                        ChatMessageContent::Text(text) => return Ok(text.clone()),
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        
        Err(anyhow::anyhow!("No response from OpenAI"))
    }
    
    /// Generate structured JSON content
    pub async fn generate_json(&self, prompt: &str) -> Result<Value> {
        let system_prompt = "You are generating structured JSON data for a horror RPG game. \
                           Return ONLY valid JSON with no markdown formatting or explanation.";
        
        let response = self.generate(prompt, Some(system_prompt)).await?;
        
        // Clean up any markdown formatting if present
        let cleaned = response
            .trim()
            .trim_start_matches("```json")
            .trim_start_matches("```")
            .trim_end_matches("```")
            .trim();
        
        Ok(serde_json::from_str(cleaned)?)
    }
}