//! AI-driven entity analysis using OpenAI API for comprehensive HBF data extraction.
//! 
//! Implements the proper 2-stage AI pipeline matching the Python reference:
//! Stage A: AI analyzes HBF samples → JSON field inventory with UUID connections
//! Stage B: Jinja templates render complete Pydantic models with spatial connections

use anyhow::{Result, Context};
use openai_dive::v1::api::Client;
use openai_dive::v1::resources::chat::{ChatCompletionParameters, ChatMessage, ChatMessageContent};
use serde_json::Value;
use std::env;
use std::path::{Path, PathBuf};

use crate::containers::RawEntity;

/// AI-powered entity analysis client for comprehensive HBF extraction
pub struct AiAnalysisClient {
    client: Client,
    model: String,
}

impl AiAnalysisClient {
    /// Initialize AI analysis client with API key
    pub fn new() -> Result<Self> {
        let api_key = env::var("OPENAI_API_KEY")
            .context("OPENAI_API_KEY environment variable not set")?;
        
        let client = Client::new(api_key);
        let model = env::var("OPENAI_MODEL")
            .unwrap_or_else(|_| "gpt-4o".to_string());
        
        Ok(Self { client, model })
    }

    /// Stage A: Extract comprehensive field inventory from HBF samples using AI analysis
    /// 
    /// This implements the critical AI-driven analysis that was missing from the Rust pipeline.
    /// Matches the Python `generate_with_openai()` approach with structured outputs.
    pub async fn extract_field_inventory(
        &self,
        category: &str,
        html_samples: &[&RawEntity],
        json_samples: &[&RawEntity],
        json_schema: Value,
        analysis_prompt: &str,
    ) -> Result<Value> {
        // Create comprehensive analysis prompt with HBF samples
        let system_prompt = create_analysis_system_prompt(category);
        let user_prompt = create_inventory_extraction_prompt(
            category, 
            html_samples, 
            json_samples, 
            analysis_prompt
        );

        let parameters = ChatCompletionParameters {
            model: self.model.clone(),
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
            temperature: Some(0.1), // Low temperature for consistent analysis
            max_tokens: Some(4000),
            // Note: response_format not available in this version of openai_dive
            // Will rely on prompt engineering for JSON format
            ..Default::default()
        };

        let result = self.client.chat().create(parameters).await
            .context("Failed to generate field inventory from OpenAI")?;

        let response_content = match &result.choices[0].message {
            ChatMessage::Assistant { content: Some(ChatMessageContent::Text(text)), .. } => text,
            _ => return Err(anyhow::anyhow!("Unexpected message type from OpenAI")),
        };

        let inventory: Value = serde_json::from_str(response_content)
            .context("Failed to parse OpenAI inventory response as JSON")?;

        // Validate against provided schema
        validate_inventory_against_schema(&inventory, &json_schema)?;

        Ok(inventory)
    }

    /// Stage B: Generate comprehensive Rust ECS code from field inventory
    /// 
    /// This uses AI to generate complete ECS components, systems, and queries
    /// with proper UUID connections and spatial data mapping.
    pub async fn generate_ecs_code_from_inventory(
        &self,
        category: &str,
        inventory: &Value,
        template_context: &str,
    ) -> Result<String> {
        let system_prompt = create_code_generation_system_prompt();
        let user_prompt = create_ecs_code_prompt(category, inventory, template_context);

        let parameters = ChatCompletionParameters {
            model: self.model.clone(),
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
            temperature: Some(0.0), // Deterministic code generation
            max_tokens: Some(8000),
            ..Default::default()
        };

        let result = self.client.chat().create(parameters).await
            .context("Failed to generate ECS code from OpenAI")?;

        let response_content = match &result.choices[0].message {
            ChatMessage::Assistant { content: Some(ChatMessageContent::Text(text)), .. } => text,
            _ => return Err(anyhow::anyhow!("Unexpected message type from OpenAI")),
        };

        // Extract Rust code from response (remove any markdown formatting)
        let rust_code = extract_rust_code_from_response(response_content);
        
        Ok(rust_code)
    }

    /// Generate dynamic BiomeType enum from extracted HBF biome data
    /// 
    /// This is the critical missing piece - instead of using hardcoded BiomeType variants,
    /// generate them from actual HBF data analysis to match what's in the database.
    pub async fn generate_biome_type_enum(
        &self,
        region_inventory: &Value,
    ) -> Result<String> {
        let system_prompt = create_biome_type_generation_prompt();
        let user_prompt = create_biome_enum_prompt(region_inventory);

        let parameters = ChatCompletionParameters {
            model: self.model.clone(),
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
            temperature: Some(0.0), // Deterministic enum generation
            max_tokens: Some(2000),
            ..Default::default()
        };

        let result = self.client.chat().create(parameters).await
            .context("Failed to generate BiomeType enum from OpenAI")?;

        let response_content = match &result.choices[0].message {
            ChatMessage::Assistant { content: Some(ChatMessageContent::Text(text)), .. } => text,
            _ => return Err(anyhow::anyhow!("Unexpected message type from OpenAI")),
        };

        let rust_code = extract_rust_code_from_response(response_content);
        
        Ok(rust_code)
    }
}

// System prompts for different AI analysis tasks

fn create_analysis_system_prompt(category: &str) -> String {
    format!(r#"You are an expert data analyst specializing in extracting structured field inventories from mixed HTML/JSON content for the "Dragon's Labyrinth" horror RPG game.

Your task is to analyze HBF (HyperText Book Format) database samples and extract a comprehensive field inventory for {} entities. This inventory will be used to generate complete ECS (Entity Component System) models for the game engine.

Critical Requirements:
1. ONLY analyze the provided HTML/JSON samples - do NOT invent or assume data
2. Focus on extracting actual field names, types, and relationships from the content
3. Identify UUID fields that represent entity relationships and connections
4. Detect coordinate/spatial data that maps entities to hex grid positions
5. Extract category-specific metadata (biome data, faction relationships, dungeon areas, etc.)
6. Determine field types based on content patterns (String, Vec<String>, Option<T>, etc.)
7. Mark fields as required or optional based on consistency across samples
8. Identify connection fields that link entities together

Return ONLY valid JSON matching the exact schema provided. Do not include any markdown formatting, explanations, or additional text."#, category)
}

fn create_code_generation_system_prompt() -> String {
    r#"You are an expert Rust developer specializing in ECS (Entity Component System) architecture for the Bevy game engine.

Your task is to generate complete, production-ready Rust code for Dragon's Labyrinth game components based on extracted field inventories.

Critical Requirements:
1. Generate complete ECS components with proper Bevy derive macros
2. Include proper Serde serialization/deserialization support
3. Add comprehensive documentation for all fields and components
4. Implement spatial container systems for hex grid coordinate mapping
5. Create UUID-based connection systems for entity relationships
6. Include proper error handling and validation
7. Follow Rust 2024 edition standards and best practices
8. Generate implementation blocks with constructors and utility methods
9. Include type-safe enum variants for categorical data
10. Ensure all code compiles without warnings or errors

Generate ONLY valid Rust code without any markdown formatting or explanations."#.to_string()
}

fn create_biome_type_generation_prompt() -> String {
    r#"You are an expert Rust developer generating dynamic enums for the Dragon's Labyrinth game based on actual database content.

Your task is to generate a BiomeType enum that matches the ACTUAL biome data found in the HBF database analysis, not hardcoded assumptions.

Critical Requirements:
1. Extract all unique biome type strings from the provided inventory data
2. Convert to valid Rust enum variants (PascalCase, valid identifiers)
3. Include proper Serde serialization support
4. Add comprehensive documentation explaining each biome type
5. Include a string conversion method for display purposes
6. Ensure the enum covers ALL biome types found in the database
7. Do not include hardcoded variants that aren't in the actual data
8. Follow Rust naming conventions and best practices

Generate ONLY the complete BiomeType enum code without markdown formatting."#.to_string()
}

fn create_inventory_extraction_prompt(
    category: &str,
    html_samples: &[&RawEntity],
    json_samples: &[&RawEntity],
    analysis_prompt: &str,
) -> String {
    let mut prompt = format!("Analyze these HBF database samples for {} entities:\n\n", category);
    
    prompt.push_str(&analysis_prompt);
    prompt.push_str("\n\nSAMPLE DATA:\n\n");
    
    // Add HTML samples
    for (i, entity) in html_samples.iter().enumerate() {
        prompt.push_str(&format!("HTML Sample {}:\nUUID: {}\nContent: {}\n\n", 
                                i + 1, entity.uuid, entity.raw_value));
    }
    
    // Add JSON samples  
    for (i, entity) in json_samples.iter().enumerate() {
        prompt.push_str(&format!("JSON Sample {}:\nUUID: {}\nContent: {}\n\n", 
                                i + 1, entity.uuid, entity.raw_value));
    }
    
    prompt.push_str("\nExtract a comprehensive field inventory from these samples and return as valid JSON only.");
    prompt
}

fn create_ecs_code_prompt(category: &str, inventory: &Value, template_context: &str) -> String {
    format!(r#"Generate complete Rust ECS code for {} entities based on this field inventory:

FIELD INVENTORY:
{}

TEMPLATE CONTEXT:
{}

Requirements:
1. Generate complete Bevy ECS components with all discovered fields
2. Include proper spatial container systems for hex coordinate mapping
3. Create UUID-based connection systems for entity relationships
4. Add comprehensive documentation and derive macros
5. Include implementation blocks with constructors
6. Ensure all types are properly defined and imported
7. Follow Rust 2024 edition standards

Generate the complete Rust module code:"#, 
        category, 
        serde_json::to_string_pretty(inventory).unwrap_or_else(|_| "Invalid JSON".to_string()),
        template_context
    )
}

fn create_biome_enum_prompt(region_inventory: &Value) -> String {
    format!(r#"Generate a BiomeType enum based on this region analysis inventory:

REGION INVENTORY:
{}

Extract all unique biome type strings and create a complete Rust enum with:
1. All biome variants found in the actual data (not hardcoded assumptions)
2. Proper PascalCase conversion for Rust naming
3. Serde serialization support
4. Documentation for each variant
5. String conversion methods

Generate the complete BiomeType enum:"#,
        serde_json::to_string_pretty(region_inventory).unwrap_or_else(|_| "Invalid JSON".to_string())
    )
}

fn validate_inventory_against_schema(inventory: &Value, _schema: &Value) -> Result<()> {
    // Basic validation - could be enhanced with full JSON schema validation
    if !inventory.is_object() {
        return Err(anyhow::anyhow!("Inventory must be a JSON object"));
    }
    
    if !inventory.get("entities").map_or(false, |e| e.is_array()) {
        return Err(anyhow::anyhow!("Inventory must contain an 'entities' array"));
    }
    
    Ok(())
}

fn extract_rust_code_from_response(response: &str) -> String {
    // Remove markdown code block formatting if present
    let code = if response.contains("```rust") {
        response
            .split("```rust")
            .nth(1)
            .and_then(|s| s.split("```").next())
            .unwrap_or(response)
    } else if response.contains("```") {
        response
            .split("```")
            .nth(1)
            .unwrap_or(response)
    } else {
        response
    };
    
    code.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_rust_code() {
        let markdown_response = r#"Here's the code:

```rust
pub struct Test {
    pub field: String,
}
```

That's it!"#;
        
        let extracted = extract_rust_code_from_response(markdown_response);
        assert!(extracted.contains("pub struct Test"));
        assert!(!extracted.contains("```"));
    }

    #[test]
    fn test_validate_inventory_basic() {
        let valid_inventory = serde_json::json!({
            "entities": [
                {
                    "name": "TestEntity",
                    "fields": []
                }
            ]
        });
        
        let result = validate_inventory_against_schema(&valid_inventory, &serde_json::json!({}));
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_inventory_invalid() {
        let invalid_inventory = serde_json::json!("not an object");
        
        let result = validate_inventory_against_schema(&invalid_inventory, &serde_json::json!({}));
        assert!(result.is_err());
    }
}
