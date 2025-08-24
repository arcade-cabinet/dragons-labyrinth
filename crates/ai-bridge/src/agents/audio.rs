//! Audio agent for spatial horror audio generation

use async_trait::async_trait;
use anyhow::Result;
use serde_json::json;

use crate::agents::Agent;
use crate::context::BuildContext;
use crate::generation::{GenerationRequest, GenerationResult, AssetSource};

/// Audio agent for generating spatial horror audio
pub struct AudioAgent {
    name: String,
    domain: String,
}

impl AudioAgent {
    /// Create a new audio agent
    pub fn new() -> Self {
        Self {
            name: "AudioAgent".to_string(),
            domain: "audio".to_string(),
        }
    }
}

impl Default for AudioAgent {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Agent for AudioAgent {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn domain(&self) -> &str {
        &self.domain
    }
    
    async fn generate(&mut self, _context: &BuildContext, request: GenerationRequest) -> Result<GenerationResult> {
        let asset_id = format!("audio_dread{}_{}", 
            request.dread_level,
            uuid::Uuid::new_v4().to_string().split('-').next().unwrap()
        );
        
        let audio_data = json!({
            "audio_type": request.requirements.get("audio_type").unwrap_or(&json!("ambient")),
            "spatial": true,
            "intensity": match request.dread_level {
                0 => 0.0,
                1 => 0.2,
                2 => 0.5,
                3 => 0.8,
                4 => 1.0,
                _ => 0.0,
            },
            "audio_characteristics": match request.dread_level {
                0 => vec!["peaceful", "nature", "birds", "wind"],
                1 => vec!["whispers", "distant_sounds", "unease"],
                2 => vec!["corruption", "decay", "fear"],
                3 => vec!["screams", "madness", "terror"],
                4 => vec!["dragon", "proximity", "absolute_horror"],
                _ => vec!["unknown"],
            },
            "dragon_proximity": request.dread_level as f32 * 0.25,
            "description": request.description,
        });
        
        Ok(GenerationResult::success(asset_id)
            .with_source(AssetSource::Generated)
            .with_metadata("audio_data", audio_data)
            .with_metadata("agent", json!(self.name())))
    }
}
