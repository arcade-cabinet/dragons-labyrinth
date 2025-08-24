//! Maps agent for hex world generation

use async_trait::async_trait;
use anyhow::Result;
use serde_json::json;

use crate::agents::Agent;
use crate::context::BuildContext;
use crate::generation::{GenerationRequest, GenerationResult, AssetSource};

/// Maps agent for generating hex-based world maps
pub struct MapsAgent {
    name: String,
    domain: String,
}

impl MapsAgent {
    /// Create a new maps agent
    pub fn new() -> Self {
        Self {
            name: "MapsAgent".to_string(),
            domain: "maps".to_string(),
        }
    }
}

impl Default for MapsAgent {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Agent for MapsAgent {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn domain(&self) -> &str {
        &self.domain
    }
    
    async fn generate(&mut self, context: &BuildContext, request: GenerationRequest) -> Result<GenerationResult> {
        use openai_dive::v1::resources::chat::{
            ChatCompletionParameters, ChatMessage, ChatMessageContent, Role,
        };
        use std::fs;
        
        // Check cache first
        if request.use_cache {
            let cached = context.memory().find_cached_generations(&request.asset_type, request.dread_level);
            if !cached.is_empty() {
                return Ok(GenerationResult::success(cached[0].asset_id.clone())
                    .with_source(AssetSource::Cached)
                    .with_metadata("cached", json!(true)));
            }
        }
        
        let asset_id = format!("hex_world_dread{}_{}", 
            request.dread_level,
            uuid::Uuid::new_v4().to_string().split('-').next().unwrap()
        );
        
        // Get horror progression context via tool
        let horror_context_result = context.execute_tool(
            "get_horror_progression", 
            &serde_json::to_string(&json!({ "dread_level": request.dread_level }))?
        ).await?;
        
        // Build specialized maps generation prompt
        let system_prompt = format!(
            "You are MapsAgent, specializing in hex-based world generation for Dragon's Labyrinth. 
            You understand horror progression and generate worlds that support the narrative journey.
            
            Your role: Generate hexx-compatible hex world layouts that evolve through dread levels 0-4.
            Key patterns:
            - Dread 0-1: Beautiful, welcoming world with subtle wrongness potential
            - Dread 2-3: Visible corruption, isolation, economic collapse
            - Dread 4: Labyrinth entrance, dragon proximity, nightmare geometry
            
            Horror Context for Dread Level {}: {}",
            request.dread_level,
            serde_json::to_string_pretty(&horror_context_result)?
        );
        
        let user_prompt = format!(
            "Generate a hex world layout for dread level {} with the following requirements:
            
            Description: {}
            Requirements: {}
            
            Output should include:
            1. Hex grid layout (radius, coordinate system)
            2. Biome distribution with corruption patterns
            3. Landmark placement (villages, ruins, corruption sources)
            4. Navigation flow that guides narrative progression
            5. Hidden elements that reveal at appropriate dread levels
            6. Hexx crate integration specifications
            
            Focus on how this world layout serves the horror narrative.",
            request.dread_level,
            request.description,
            serde_json::to_string_pretty(&request.requirements)?
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
            .map_err(|e| anyhow::anyhow!("OpenAI API call failed: {}", e))?;
        
        // Extract generated content
        let content = response.choices[0].message.content.as_ref()
            .ok_or_else(|| anyhow::anyhow!("No content in response"))?;
        
        let generated_content = match content {
            ChatMessageContent::Text(text) => text.clone(),
            _ => return Err(anyhow::anyhow!("Unexpected content type")),
        };
        
        // Create structured hex data based on dread level
        let hex_data = json!({
            "world_name": format!("World at Dread {}", request.dread_level),
            "hex_radius": match request.dread_level {
                0..=2 => 50,  // Larger world for exploration
                3..=4 => 30,  // Smaller, claustrophobic world
                _ => 50,
            },
            "biomes": match request.dread_level {
                0 => vec!["grassland", "forest", "hills", "mountains", "rivers", "villages"],
                1 => vec!["darkening_grassland", "shadowy_forest", "misty_hills", "looming_mountains", "troubled_rivers", "nervous_villages"],
                2 => vec!["corrupted_grassland", "dying_forest", "barren_hills", "cursed_mountains", "poisoned_rivers", "abandoned_villages"],
                3 => vec!["blighted_wastes", "dead_forest", "nightmare_hills", "terror_peaks", "blood_rivers", "ghost_villages"],
                4 => vec!["labyrinth_entrance", "void_spaces", "dragon_domain", "absolute_darkness", "reality_tears", "nightmare_geometry"],
                _ => vec!["unknown"],
            },
            "corruption_level": request.dread_level as f32 * 0.25,
            "description": request.description,
            "generated_content": generated_content,
            "hexx_integration": {
                "coordinate_system": "axial",
                "pathfinding_enabled": true,
                "line_of_sight_enabled": true,
                "supports_corruption_spread": true,
            }
        });
        
        // Create output directory and save
        let output_dir = context.output_dir()
            .join("generated")
            .join("maps");
        fs::create_dir_all(&output_dir)?;
        
        let output_path = output_dir.join(format!("{}.json", asset_id));
        fs::write(&output_path, serde_json::to_string_pretty(&hex_data)?)?;
        
        // Cache the result
        context.memory().cache_generation(
            &request.asset_type,
            request.dread_level,
            &asset_id,
            &generated_content
        );
        
        Ok(GenerationResult::success(asset_id.clone())
            .with_source(AssetSource::Generated)
            .with_metadata("hex_data", hex_data)
            .with_metadata("agent", json!(self.name()))
            .with_metadata("tokens_used", json!(response.usage.as_ref().map(|u| u.total_tokens).unwrap_or(0)))
            .with_output_file(output_path))
    }
}
