//! Quest generation with moral complexity

use crate::openai_client::OpenAIClient;
use anyhow::Result;
use minijinja::Environment;
use serde::{Deserialize, Serialize};
use std::path::Path;

pub struct QuestGenerator {
    client: OpenAIClient,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum QuestType {
    // Dread 0 - Peace
    SimpleDelivery,
    VillageHelp,
    
    // Dread 1 - Unease  
    Investigation,
    MinorDilemma,
    
    // Dread 2 - Anxiety
    ResourceScarcity,
    TrustBetrayal,
    
    // Dread 3 - Horror
    SacrificialChoice,
    CompanionCrisis,
    
    // Dread 4 - Madness
    ImpossibleChoice,
    FinalConfrontation,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum MoralComplexity {
    Simple,      // Clear right/wrong
    Nuanced,     // Trade-offs
    Ambiguous,   // No good choice
    Devastating, // All choices harm someone
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Quest {
    pub id: String,
    pub title: String,
    pub description: String,
    pub quest_type: QuestType,
    pub moral_weight: MoralComplexity,
    pub stages: Vec<QuestStage>,
    pub outcomes: Vec<QuestOutcome>,
    pub dialogue_nodes: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QuestStage {
    pub description: String,
    pub objectives: Vec<String>,
    pub choices: Vec<QuestChoice>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QuestChoice {
    pub text: String,
    pub requirements: Vec<String>,
    pub consequences: Vec<String>,
    pub companion_reactions: Vec<(String, f32)>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QuestOutcome {
    pub choice_path: String,
    pub description: String,
    pub world_changes: Vec<String>,
    pub companion_impacts: Vec<(String, String)>,
}

pub struct QuestResult {
    pub quests_created: usize,
    pub tokens_used: usize,
}

impl QuestGenerator {
    pub fn new() -> Result<Self> {
        Ok(Self {
            client: OpenAIClient::new()?,
        })
    }
    
    pub async fn generate_quest_chain(
        &mut self,
        dread_level: u8,
        output_dir: &Path,
    ) -> Result<QuestResult> {
        std::fs::create_dir_all(output_dir)?;
        
        let mut result = QuestResult {
            quests_created: 0,
            tokens_used: 0,
        };
        
        // Select quest types based on dread level
        let quest_types = match dread_level {
            0 => vec![QuestType::SimpleDelivery, QuestType::VillageHelp],
            1 => vec![QuestType::Investigation, QuestType::MinorDilemma],
            2 => vec![QuestType::ResourceScarcity, QuestType::TrustBetrayal],
            3 => vec![QuestType::SacrificialChoice, QuestType::CompanionCrisis],
            4 => vec![QuestType::ImpossibleChoice, QuestType::FinalConfrontation],
            _ => vec![],
        };
        
        for quest_type in quest_types {
            let quest = self.generate_quest(dread_level, quest_type.clone()).await?;
            
            // Save quest definition
            let filename = format!(
                "quest_dread{}_{:?}.json",
                dread_level,
                quest_type,
            ).to_lowercase();
            
            let filepath = output_dir.join(filename);
            std::fs::write(filepath, serde_json::to_string_pretty(&quest)?)?;
            
            // Generate associated YarnSpinner dialogue
            let dialogue = self.generate_quest_dialogue(&quest).await?;
            
            let dialogue_filename = format!(
                "quest_dread{}_{:?}.yarn",
                dread_level,
                quest_type,
            ).to_lowercase();
            
            let dialogue_path = output_dir.join(dialogue_filename);
            std::fs::write(dialogue_path, dialogue)?;
            
            result.quests_created += 1;
        }
        
        Ok(result)
    }
    
    async fn generate_quest(
        &self,
        dread_level: u8,
        quest_type: QuestType,
    ) -> Result<Quest> {
        let moral_complexity = match dread_level {
            0 => MoralComplexity::Simple,
            1..=2 => MoralComplexity::Nuanced,
            3 => MoralComplexity::Ambiguous,
            4 => MoralComplexity::Devastating,
            _ => MoralComplexity::Simple,
        };
        
        // Load template
        let template_str = std::fs::read_to_string("crates/game-content-generated/templates/quest_structure.j2")?;
        let mut env = Environment::new();
        env.add_template("quest", &template_str)?;
        let tmpl = env.get_template("quest")?;
        
        // Render template
        let prompt = tmpl.render(minijinja::context! {
            dread_level => dread_level,
            quest_type => format!("{:?}", quest_type),
            moral_complexity => format!("{:?}", moral_complexity),
        })?;
        
        let json = self.client.generate_json(&prompt).await?;
        let quest: Quest = serde_json::from_value(json)?;
        
        Ok(quest)
    }
    
    async fn generate_quest_dialogue(&self, quest: &Quest) -> Result<String> {
        let prompt = format!(
            r#"Generate YarnSpinner dialogue for this quest:

TITLE: {}
DESCRIPTION: {}
STAGES: {} stages

Create dialogue for:
1. Quest introduction
2. Key decision points
3. Companion reactions
4. Outcome revelations

Use proper YarnSpinner format with variables and branching.
Include emotional weight appropriate to the moral complexity."#,
            quest.title,
            quest.description,
            quest.stages.len(),
        );
        
        self.client.generate(&prompt, None).await
    }
}