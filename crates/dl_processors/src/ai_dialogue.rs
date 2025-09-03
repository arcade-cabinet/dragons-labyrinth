//! OpenAI-powered dialogue generation for Dragon's Labyrinth
//! 
//! This module integrates with OpenAI API to generate contextual dialogue
//! and quests using Seeds data (literature patterns and linguistics).

use anyhow::{Result, Context};
use openai_dive::v1::api::Client;
use openai_dive::v1::resources::chat::{ChatCompletionParameters, ChatMessage, ChatMessageContent};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;

/// AI-powered dialogue generator using OpenAI API
pub struct AiDialogueGenerator {
    client: Client,
    model: String,
}

impl AiDialogueGenerator {
    /// Initialize AI dialogue generator with API key
    pub fn new() -> Result<Self> {
        let api_key = env::var("OPENAI_API_KEY")
            .context("OPENAI_API_KEY environment variable not set")?;
        
        let client = Client::new(api_key);
        let model = env::var("OPENAI_MODEL")
            .unwrap_or_else(|_| "gpt-4o".to_string());
        
        Ok(Self { client, model })
    }
    
    /// Generate NPC dialogue using seeds data and region context
    pub async fn generate_npc_dialogue(
        &self,
        context: &NpcDialogueContext,
        seeds_data: &SeedsDialogueData,
    ) -> Result<GeneratedDialogue> {
        let system_prompt = create_dialogue_system_prompt();
        let user_prompt = create_npc_dialogue_prompt(context, seeds_data);
        
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
            temperature: Some(0.8),
            max_tokens: Some(2000),
            ..Default::default()
        };
        
        let result = self.client.chat().create(parameters).await
            .context("Failed to generate dialogue from OpenAI")?;
        
        let response_content = match &result.choices[0].message {
            ChatMessage::Assistant { content: Some(ChatMessageContent::Text(text)), .. } => text,
            _ => return Err(anyhow::anyhow!("Unexpected message type from OpenAI")),
        };
        
        let dialogue: GeneratedDialogue = serde_json::from_str(response_content)
            .context("Failed to parse OpenAI dialogue response")?;
        
        Ok(dialogue)
    }
    
    /// Generate quest from literature patterns using seeds data
    pub async fn generate_quest(
        &self,
        context: &QuestGenerationContext,
        seeds_data: &SeedsQuestData,
    ) -> Result<GeneratedQuest> {
        let system_prompt = create_quest_system_prompt();
        let user_prompt = create_quest_generation_prompt(context, seeds_data);
        
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
            temperature: Some(0.7),
            max_tokens: Some(3000),
            ..Default::default()
        };
        
        let result = self.client.chat().create(parameters).await
            .context("Failed to generate quest from OpenAI")?;
        
        let response_content = match &result.choices[0].message {
            ChatMessage::Assistant { content: Some(ChatMessageContent::Text(text)), .. } => text,
            _ => return Err(anyhow::anyhow!("Unexpected message type from OpenAI")),
        };
        
        let quest: GeneratedQuest = serde_json::from_str(response_content)
            .context("Failed to parse OpenAI quest response")?;
        
        Ok(quest)
    }
}

/// Context for NPC dialogue generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NpcDialogueContext {
    pub npc_uuid: String,
    pub npc_name: String,
    pub region_uuid: String,
    pub settlement_uuid: String,
    pub region_type: String,
    pub act: u8,
    pub band: u8,
    pub corruption_level: f32,
    pub location_type: String,
    pub archetype: String,
    pub personality_traits: Vec<String>,
    pub speech_patterns: Vec<String>,
}

/// Context for quest generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestGenerationContext {
    pub quest_id: String,
    pub region_uuid: String,
    pub dungeon_uuid: Option<String>,
    pub npc_giver: String,
    pub act: u8,
    pub band: u8,
    pub corruption_level: f32,
    pub preferred_pattern: String,
    pub available_locations: Vec<String>,
    pub horror_themes: Vec<String>,
}

/// Seeds data for dialogue generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeedsDialogueData {
    pub linguistic_patterns: Vec<LinguisticPattern>,
    pub character_archetypes: Vec<CharacterArchetype>,
    pub old_norse_vocabulary: HashMap<String, String>,
    pub cultural_references: Vec<String>,
}

/// Seeds data for quest generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeedsQuestData {
    pub literature_patterns: Vec<LiteraturePattern>,
    pub horror_beats: Vec<String>,
    pub poe_excerpts: Vec<String>,
    pub dracula_themes: Vec<String>,
    pub quest_archetypes: Vec<QuestArchetype>,
}

/// Generated NPC dialogue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedDialogue {
    pub npc_uuid: String,
    pub greeting: String,
    pub casual_lines: Vec<String>,
    pub quest_offer: Option<String>,
    pub farewell: String,
    pub corruption_responses: HashMap<String, String>,
    pub personality_modifiers: Vec<String>,
}

impl GeneratedDialogue {
    pub fn json_schema() -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "npc_uuid": {"type": "string"},
                "greeting": {"type": "string"},
                "casual_lines": {"type": "array", "items": {"type": "string"}},
                "quest_offer": {"type": "string"},
                "farewell": {"type": "string"},
                "corruption_responses": {
                    "type": "object",
                    "additionalProperties": {"type": "string"}
                },
                "personality_modifiers": {"type": "array", "items": {"type": "string"}}
            },
            "required": ["npc_uuid", "greeting", "casual_lines", "farewell", "corruption_responses", "personality_modifiers"]
        })
    }
}

/// Generated quest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedQuest {
    pub id: String,
    pub title: String,
    pub description: String,
    pub pattern_type: String,
    pub acts: Vec<QuestAct>,
    pub horror_progression: HorrorProgression,
    pub estimated_duration: u32,
    pub corruption_impact: f32,
}

impl GeneratedQuest {
    pub fn json_schema() -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "id": {"type": "string"},
                "title": {"type": "string"},
                "description": {"type": "string"},
                "pattern_type": {"type": "string"},
                "acts": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "properties": {
                            "act_number": {"type": "number"},
                            "title": {"type": "string"},
                            "description": {"type": "string"},
                            "objectives": {"type": "array", "items": {"type": "string"}},
                            "horror_beat": {"type": "string"}
                        },
                        "required": ["act_number", "title", "description", "objectives", "horror_beat"]
                    }
                },
                "horror_progression": {
                    "type": "object",
                    "properties": {
                        "initial_dread": {"type": "number"},
                        "climax_terror": {"type": "number"},
                        "resolution_relief": {"type": "number"}
                    },
                    "required": ["initial_dread", "climax_terror", "resolution_relief"]
                },
                "estimated_duration": {"type": "number"},
                "corruption_impact": {"type": "number"}
            },
            "required": ["id", "title", "description", "pattern_type", "acts", "horror_progression", "estimated_duration", "corruption_impact"]
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestAct {
    pub act_number: u32,
    pub title: String,
    pub description: String,
    pub objectives: Vec<String>,
    pub horror_beat: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HorrorProgression {
    pub initial_dread: f32,
    pub climax_terror: f32,
    pub resolution_relief: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinguisticPattern {
    pub pattern_type: String,
    pub examples: Vec<String>,
    pub cultural_context: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterArchetype {
    pub archetype_type: String,
    pub alignment: String,
    pub traits: Vec<String>,
    pub motivations: Vec<String>,
    pub speech_patterns: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiteraturePattern {
    pub source_work: String,
    pub pattern_type: String,
    pub beats: Vec<String>,
    pub themes: Vec<String>,
    pub horror_elements: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestArchetype {
    pub archetype_name: String,
    pub typical_structure: Vec<String>,
    pub horror_integration: Vec<String>,
    pub corruption_themes: Vec<String>,
}

// Prompt generation functions

fn create_dialogue_system_prompt() -> String {
    r#"You are an expert narrative designer for "Dragon's Labyrinth", a horror RPG where players journey through increasingly corrupted lands toward an inevitable confrontation with an ancient dragon.

Key Game Context:
- Horror RPG with inverted power curve (players get weaker/more cursed as they progress)
- 5 Acts of progression: Peace (1-20) → Unease (21-40) → Dread (41-60) → Terror (61-120) → Horror (121-180)
- Corruption level affects NPC behavior and dialogue tone
- NPCs can suffer trauma and psychological changes
- Settings range from pastoral decay to complete horror

Your task is to generate contextual NPC dialogue that:
1. Reflects the corruption level and act progression
2. Uses provided linguistic patterns and cultural references
3. Maintains character archetype consistency
4. Integrates horror elements subtly but effectively
5. Provides multiple dialogue options for different player interactions

Generate dialogue that feels authentic to the game's dark fantasy horror setting while remaining engaging and atmospheric.

Return response as valid JSON matching the GeneratedDialogue schema."#.to_string()
}

fn create_quest_system_prompt() -> String {
    r#"You are an expert quest designer for "Dragon's Labyrinth", a horror RPG with mathematical progression of dread.

Key Game Context:
- Horror RPG where distance from origin increases corruption and dread
- 5 Acts with distinct horror progression: Peace → Unease → Dread → Terror → Horror
- Quests must integrate horror elements from classic literature (Poe, Dracula, etc.)
- Each act has different mechanical and thematic focus
- Player companions can be traumatized by quest events
- Corruption impacts quest outcomes and NPC reactions

Quest Design Principles:
1. Use provided literature patterns (Poe's psychological horror, Dracula's gothic elements)
2. Create 3-5 act quest structure with escalating horror
3. Include corruption consequences and moral choices
4. Integrate location-specific elements (dungeons, settlements, regions)
5. Provide meaningful companion trauma opportunities
6. Ensure horror progression matches game's mathematical dread curve

Generate quests that feel like they belong in classic horror literature while fitting the game's mechanical systems.

Return response as valid JSON matching the GeneratedQuest schema."#.to_string()
}

fn create_npc_dialogue_prompt(context: &NpcDialogueContext, seeds_data: &SeedsDialogueData) -> String {
    format!(r#"Generate dialogue for NPC in Dragon's Labyrinth:

NPC Context:
- UUID: {}
- Name: {}
- Region: {} (Act {}, Band {})
- Location: {} in {}
- Archetype: {}
- Corruption Level: {:.2}
- Personality Traits: {:?}
- Speech Patterns: {:?}

Available Seeds Data:
- Linguistic Patterns: {} patterns available
- Character Archetypes: {} archetypes
- Old Norse Vocabulary: {} terms
- Cultural References: {} references

Requirements:
1. Generate greeting, 3-5 casual dialogue lines, optional quest offer, and farewell
2. Create corruption-specific responses (low/medium/high corruption levels)
3. Use provided linguistic patterns and vocabulary where appropriate
4. Match the archetype's speech patterns and personality traits
5. Reflect the horror progression of Act {} (corruption level {:.2})
6. Include personality modifiers that could change based on player actions

The dialogue should feel authentic to a {} in a {} during the {} phase of the horror progression."#,
        context.npc_uuid,
        context.npc_name,
        context.region_uuid,
        context.act,
        context.band,
        context.location_type,
        context.settlement_uuid,
        context.archetype,
        context.corruption_level,
        context.personality_traits,
        context.speech_patterns,
        seeds_data.linguistic_patterns.len(),
        seeds_data.character_archetypes.len(),
        seeds_data.old_norse_vocabulary.len(),
        seeds_data.cultural_references.len(),
        context.act,
        context.corruption_level,
        context.archetype,
        context.location_type,
        get_horror_phase_name(context.act)
    )
}

fn create_quest_generation_prompt(context: &QuestGenerationContext, seeds_data: &SeedsQuestData) -> String {
    format!(r#"Generate a quest for Dragon's Labyrinth:

Quest Context:
- Quest ID: {}
- Region: {}
- Dungeon: {}
- NPC Giver: {}
- Act: {} (Corruption Level: {:.2})
- Preferred Pattern: {}
- Available Locations: {:?}
- Horror Themes: {:?}

Available Seeds Data:
- Literature Patterns: {} from classic horror works
- Horror Beats: {} progression beats
- Poe Excerpts: {} psychological horror elements
- Dracula Themes: {} gothic horror themes
- Quest Archetypes: {} structural templates

Requirements:
1. Create a 3-5 act quest structure following the preferred pattern
2. Integrate horror elements from the provided literature patterns
3. Use location-specific elements and available horror themes
4. Include escalating horror progression matching Act {} corruption level
5. Design meaningful choices that could traumatize companions
6. Ensure the quest fits the mathematical dread progression
7. Include corruption impact assessment for quest completion

The quest should feel like it belongs in classic horror literature (Poe's psychological depth, Dracula's gothic atmosphere) while fitting the game's mechanical systems and {} setting."#,
        context.quest_id,
        context.region_uuid,
        context.dungeon_uuid.as_deref().unwrap_or("None"),
        context.npc_giver,
        context.act,
        context.corruption_level,
        context.preferred_pattern,
        context.available_locations,
        context.horror_themes,
        seeds_data.literature_patterns.len(),
        seeds_data.horror_beats.len(),
        seeds_data.poe_excerpts.len(),
        seeds_data.dracula_themes.len(),
        seeds_data.quest_archetypes.len(),
        context.act,
        get_horror_phase_name(context.act)
    )
}

fn get_horror_phase_name(act: u8) -> &'static str {
    match act {
        1 => "Peace/Pastoral Decay",
        2 => "Unease/Growing Dread", 
        3 => "Dread/Manifest Horror",
        4 => "Terror/Warped Reality",
        5 => "Horror/Complete Corruption",
        _ => "Unknown Phase",
    }
}
