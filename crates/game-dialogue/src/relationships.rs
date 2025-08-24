//! Companion relationship dynamics generation

use crate::openai_client::OpenAIClient;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct RelationshipGenerator {
    client: OpenAIClient,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RelationshipMatrix {
    pub companions: Vec<String>,
    pub relationships: HashMap<(String, String), Relationship>,
    pub group_dynamics: Vec<GroupDynamic>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Relationship {
    pub initial_affinity: f32,
    pub trust: f32,
    pub shared_experiences: Vec<String>,
    pub conflict_points: Vec<String>,
    pub evolution_path: Vec<RelationshipStage>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RelationshipStage {
    pub dread_level: u8,
    pub description: String,
    pub dialogue_changes: Vec<String>,
    pub potential_betrayal: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GroupDynamic {
    pub participants: Vec<String>,
    pub dynamic_type: DynamicType,
    pub trigger_conditions: Vec<String>,
    pub outcomes: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DynamicType {
    Alliance,
    Rivalry,
    LoveTriangle,
    MentorStudent,
    Protector,
    Saboteur,
}

impl RelationshipGenerator {
    pub fn new() -> Result<Self> {
        Ok(Self {
            client: OpenAIClient::new()?,
        })
    }
    
    pub async fn generate_relationship_web(
        &self,
        companions: Vec<String>,
    ) -> Result<RelationshipMatrix> {
        let prompt = format!(
            r#"Generate a complex web of relationships between these companions:
{}

Consider:
- Initial relationships based on archetypes
- How relationships evolve with dread
- Potential betrayals and alliances
- Group dynamics that emerge
- Conflicts that arise from moral choices

Return as JSON with detailed relationship data."#,
            companions.join(", ")
        );
        
        let json = self.client.generate_json(&prompt).await?;
        let matrix: RelationshipMatrix = serde_json::from_value(json)?;
        
        Ok(matrix)
    }
}
