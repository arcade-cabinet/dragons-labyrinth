//! Tool definitions and execution for AI agents

use std::collections::HashMap;
use anyhow::Result;
use openai_dive::v1::resources::chat::{
    ChatCompletionFunction, ChatCompletionTool, ChatCompletionToolType,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::context::BuildContext;
use crate::error::BuildToolError;

// Parameter structs for tools
#[derive(Debug, Serialize, Deserialize)]
pub struct SearchAssetsParams {
    pub query: String,
    pub category: String,
    pub dread_level: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryDatabaseParams {
    pub query_type: String,
    pub filters: Option<HashMap<String, Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HorrorProgressionParams {
    pub dread_level: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateAssetParams {
    pub asset_type: String,
    pub description: String,
    pub dread_level: u8,
    pub metadata: Option<HashMap<String, Value>>,
}

/// Create the search assets tool definition
pub fn create_search_assets_tool() -> ChatCompletionTool {
    ChatCompletionTool {
        r#type: ChatCompletionToolType::Function,
        function: ChatCompletionFunction {
            name: "search_assets".to_string(),
            description: Some("Search existing CC0 library assets before generating new ones".to_string()),
            parameters: json!({
                "type": "object",
                "properties": {
                    "query": {
                        "type": "string",
                        "description": "Search query describing the asset needed"
                    },
                    "category": {
                        "type": "string",
                        "enum": ["models", "textures", "audio", "sprites"],
                        "description": "Asset category to search"
                    },
                    "dread_level": {
                        "type": "integer",
                        "minimum": 0,
                        "maximum": 4,
                        "description": "Horror progression level"
                    }
                },
                "required": ["query", "category"]
            }),
        },
    }
}

/// Create the query database tool definition
pub fn create_query_database_tool() -> ChatCompletionTool {
    ChatCompletionTool {
        r#type: ChatCompletionToolType::Function,
        function: ChatCompletionFunction {
            name: "query_game_database".to_string(),
            description: Some("Query game database for context about hex tiles, companions, encounters, etc.".to_string()),
            parameters: json!({
                "type": "object",
                "properties": {
                    "query_type": {
                        "type": "string",
                        "enum": ["hex_tiles", "companions", "encounters", "dialogues", "items"],
                        "description": "Type of game data to query"
                    },
                    "filters": {
                        "type": "object",
                        "description": "Filters to apply to the query"
                    }
                },
                "required": ["query_type"]
            }),
        },
    }
}

/// Create the horror progression tool definition
pub fn create_horror_progression_tool() -> ChatCompletionTool {
    ChatCompletionTool {
        r#type: ChatCompletionToolType::Function,
        function: ChatCompletionFunction {
            name: "get_horror_progression".to_string(),
            description: Some("Get information about horror progression and dread levels".to_string()),
            parameters: json!({
                "type": "object",
                "properties": {
                    "dread_level": {
                        "type": "integer",
                        "minimum": 0,
                        "maximum": 4,
                        "description": "Dread level to get information about"
                    }
                },
                "required": ["dread_level"]
            }),
        },
    }
}

/// Create the generate asset tool definition
pub fn create_generate_asset_tool() -> ChatCompletionTool {
    ChatCompletionTool {
        r#type: ChatCompletionToolType::Function,
        function: ChatCompletionFunction {
            name: "generate_asset".to_string(),
            description: Some("Generate a new asset when no suitable existing asset is found".to_string()),
            parameters: json!({
                "type": "object",
                "properties": {
                    "asset_type": {
                        "type": "string",
                        "enum": ["hex_tile", "sprite", "ui_element", "dialogue", "audio_description"],
                        "description": "Type of asset to generate"
                    },
                    "description": {
                        "type": "string",
                        "description": "Detailed description of the asset to generate"
                    },
                    "dread_level": {
                        "type": "integer",
                        "minimum": 0,
                        "maximum": 4,
                        "description": "Horror progression level for the asset"
                    },
                    "metadata": {
                        "type": "object",
                        "description": "Additional metadata for the asset"
                    }
                },
                "required": ["asset_type", "description", "dread_level"]
            }),
        },
    }
}

/// Execute a tool call
pub async fn execute_tool(context: &BuildContext, name: &str, arguments: &str) -> Result<Value> {
    match name {
        "search_assets" => {
            let params: SearchAssetsParams = serde_json::from_str(arguments)?;
            search_assets(context, params).await
        }
        "query_game_database" => {
            let params: QueryDatabaseParams = serde_json::from_str(arguments)?;
            query_database(context, params).await
        }
        "get_horror_progression" => {
            let params: HorrorProgressionParams = serde_json::from_str(arguments)?;
            get_horror_progression(params).await
        }
        "generate_asset" => {
            let params: GenerateAssetParams = serde_json::from_str(arguments)?;
            generate_asset(context, params).await
        }
        _ => Err(BuildToolError::ToolExecutionFailed(
            format!("Unknown tool: {}", name)
        ).into())
    }
}

async fn search_assets(_context: &BuildContext, params: SearchAssetsParams) -> Result<Value> {
    // Implementation would search CC0 library
    // For now, return mock result
    Ok(json!({
        "found": false,
        "reason": "No existing assets match the query",
        "query": params.query,
        "category": params.category,
        "dread_level": params.dread_level,
        "suggestions": []
    }))
}

async fn query_database(context: &BuildContext, params: QueryDatabaseParams) -> Result<Value> {
    if let Some(_db) = context.database() {
        // Query the database based on query_type
        match params.query_type.as_str() {
            "hex_tiles" => {
                // Example: query hex tiles
                Ok(json!({
                    "hex_tiles": [],
                    "total": 0,
                    "query_type": params.query_type
                }))
            }
            "companions" => {
                Ok(json!({
                    "companions": [
                        {
                            "name": "Einar",
                            "loyalty": 100,
                            "trauma_level": 0.0,
                            "description": "Loyal friend and warrior"
                        },
                        {
                            "name": "Mira",
                            "loyalty": 90,
                            "trauma_level": 0.0,
                            "description": "Optimistic healer"
                        },
                        {
                            "name": "Sorin",
                            "loyalty": 80,
                            "trauma_level": 0.0,
                            "description": "Scholar seeking knowledge"
                        },
                        {
                            "name": "Tamara",
                            "loyalty": 95,
                            "trauma_level": 0.0,
                            "description": "Young baker's apprentice"
                        }
                    ],
                    "total": 4
                }))
            }
            _ => Ok(json!({ 
                "error": "Query type not implemented",
                "query_type": params.query_type
            }))
        }
    } else {
        Err(BuildToolError::ToolExecutionFailed(
            "Database not connected".to_string()
        ).into())
    }
}

async fn get_horror_progression(params: HorrorProgressionParams) -> Result<Value> {
    // Return horror progression information
    let progression_info = match params.dread_level {
        0 => json!({
            "stage": "Peace",
            "description": "Beautiful world, helpful NPCs, bright textures",
            "visual_style": "Bright, colorful, welcoming",
            "audio_style": "Cheerful, peaceful, ambient nature sounds",
            "companion_state": "Happy, optimistic, full of energy",
            "world_state": "Pristine, thriving villages, clear skies"
        }),
        1 => json!({
            "stage": "Unease",
            "description": "Shadow overlays, whispered audio, first signs of wrongness",
            "visual_style": "Slightly darker, longer shadows, subtle distortions",
            "audio_style": "Occasional whispers, distant sounds, tension building",
            "companion_state": "Slightly worried, noticing oddities, questioning",
            "world_state": "Subtle corruption, NPCs acting strange, animals fleeing"
        }),
        2 => json!({
            "stage": "Dread",
            "description": "Corruption masks, economic collapse, visible decay",
            "visual_style": "Corrupted textures, decay overlays, darker palette",
            "audio_style": "Ominous ambient, corruption sounds, fear responses",
            "companion_state": "Stressed, showing trauma, considering leaving",
            "world_state": "Visible corruption, abandoned buildings, dying nature"
        }),
        3 => json!({
            "stage": "Terror",
            "description": "Reality distortion, companion betrayal, moral horror",
            "visual_style": "Heavy distortion, nightmare imagery, broken reality",
            "audio_style": "Intense horror sounds, screams, madness",
            "companion_state": "Breaking down, betrayal possible, severe trauma",
            "world_state": "Reality breaking, geometry warping, time distortions"
        }),
        4 => json!({
            "stage": "Horror",
            "description": "Complete transformation, dragon stalking, first-person nightmare",
            "visual_style": "Full nightmare mode, extreme darkness, horror transformation",
            "audio_style": "Dragon proximity sounds, pure horror audio",
            "companion_state": "Gone or transformed, complete breakdown",
            "world_state": "Labyrinth reality, dragon domain, no escape"
        }),
        _ => json!({ "error": "Invalid dread level" })
    };
    
    Ok(progression_info)
}

async fn generate_asset(context: &BuildContext, params: GenerateAssetParams) -> Result<Value> {
    use openai_dive::v1::resources::chat::{
        ChatCompletionParameters, ChatMessage, ChatMessageContent, Role,
    };
    use std::fs;
    
    // Create asset ID
    let asset_id = format!("{}_dread{}_{}", 
        params.asset_type, 
        params.dread_level,
        uuid::Uuid::new_v4().to_string().split('-').next().unwrap()
    );
    
    // Get horror progression context
    let horror_context = get_horror_progression(HorrorProgressionParams {
        dread_level: params.dread_level
    }).await?;
    
    // Build generation prompt
    let system_prompt = format!(
        "You are an expert asset designer for a horror-first RPG. Generate detailed specifications for a {} at dread level {} ({}). 
        
        Horror Context: {}
        
        Focus on how this asset contributes to the horror progression. Be specific about visual details, materials, and emotional impact.",
        params.asset_type,
        params.dread_level,
        horror_context.get("stage").unwrap_or(&json!("Unknown")).as_str().unwrap_or("Unknown"),
        serde_json::to_string_pretty(&horror_context)?
    );
    
    let user_prompt = format!(
        "Asset Type: {}
        Dread Level: {} 
        Description: {}
        Additional Requirements: {}
        
        Generate a detailed asset specification that serves the horror narrative.",
        params.asset_type,
        params.dread_level, 
        params.description,
        serde_json::to_string_pretty(&params.metadata.unwrap_or_default())?
    );
    
    // Make OpenAI API call
    let chat_params = ChatCompletionParameters {
        model: context.config().model.clone(),
        messages: vec![
            ChatMessage {
                role: Role::System,
                content: ChatMessageContent::Text(system_prompt),
                name: None,
                tool_calls: None,
                tool_call_id: None,
            },
            ChatMessage {
                role: Role::User,
                content: ChatMessageContent::Text(user_prompt),
                name: None,
                tool_calls: None,
                tool_call_id: None,
            },
        ],
        tools: Some(context.get_available_tools()),
        temperature: Some(context.config().temperature),
        max_tokens: Some(context.config().max_tokens as u32),
        ..Default::default()
    };
    
    let response = context.client()
        .chat()
        .create(chat_params)
        .await
        .map_err(|e| BuildToolError::ApiError(format!("OpenAI API call failed: {}", e)))?;
    
    // Extract generated content
    let content = response.choices[0].message.content.as_ref()
        .ok_or_else(|| BuildToolError::GenerationFailed("No content in response".to_string()))?;
    
    let generated_content = match content {
        ChatMessageContent::Text(text) => text.clone(),
        _ => return Err(BuildToolError::GenerationFailed("Unexpected content type".to_string()).into()),
    };
    
    // Create output directory
    let output_dir = context.output_dir()
        .join("generated")
        .join(&params.asset_type);
    fs::create_dir_all(&output_dir)?;
    
    // Save generated content
    let output_path = output_dir.join(format!("{}.json", asset_id));
    let asset_data = json!({
        "asset_id": asset_id,
        "asset_type": params.asset_type,
        "dread_level": params.dread_level,
        "description": params.description,
        "generated_content": generated_content,
        "generation_metadata": {
            "model": context.config().model,
            "temperature": context.config().temperature,
            "tokens_used": response.usage.as_ref().map(|u| u.total_tokens).unwrap_or(0),
            "generated_at": chrono::Utc::now().to_rfc3339(),
        },
        "requirements": params.metadata.unwrap_or_default(),
    });
    
    fs::write(&output_path, serde_json::to_string_pretty(&asset_data)?)?;
    
    Ok(json!({
        "generated": true,
        "asset_id": asset_id,
        "description": params.description,
        "dread_level": params.dread_level,
        "output_path": output_path.to_string_lossy(),
        "generated_content": generated_content,
        "tokens_used": response.usage.as_ref().map(|u| u.total_tokens).unwrap_or(0),
        "metadata": asset_data
    }))
}
