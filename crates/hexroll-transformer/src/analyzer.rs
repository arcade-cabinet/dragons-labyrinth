
use tiktoken_rs::cl100k_base;
use crate::models::PageType;
use anyhow::{Result, anyhow};

// OpenAI Dive (openai_dive) client
use openai_dive::v1::api::Client;
use openai_dive::v1::resources::chat::{
    ChatCompletionParametersBuilder,
    ChatCompletionResponseFormat,
    ChatMessage, ChatMessageContent,
};

#[derive(Clone)]
pub struct OpenAiDiveAnalyzer {
    pub model: String,
    pub max_tokens: usize,
}

impl OpenAiDiveAnalyzer {
    pub fn new(model: impl Into<String>, max_tokens: usize) -> Self {
        Self { model: model.into(), max_tokens }
    }

    fn trim_tokens(&self, s: &str) -> String {
        let bpe = cl100k_base().unwrap();
        let mut ids = bpe.encode_with_special_tokens(s);
        if ids.len() > self.max_tokens {
            ids.truncate(self.max_tokens);
        }
        bpe.decode(ids).unwrap_or_default()
    }

    fn client() -> Client {
        // Uses OPENAI_API_KEY from env by default (recommended).
        Client::new_from_env()
    }

    /// Ask the model to classify each HTML fragment into a PageType.
    /// Returns one PageType per input (same order).
    pub async fn analyse_batch_types(&self, batch_html: Vec<&str>) -> Result<Vec<PageType>> {
        if batch_html.is_empty() {
            return Ok(vec![]);
        }
        // Prepare a compact JSON array as input to keep tokens small.
        let joined: String = batch_html
            .into_iter()
            .enumerate()
            .map(|(i, s)| format!(r#"{{"i":{},"html":{}}}"#, i, json_escape(s)))
            .collect::<Vec<_>>()
            .join(",");
        let payload = format!(r#"[{}]"#, joined);
        let prompt = format!(r#"
You are a strict JSON engine that classifies HTML fragments from a Hexroll sandbox export.
Return a JSON array with one string per input item, in the same order.
Allowed values: ["Hex","Region","Biome","Settlement","City","Town","Village","Inn","Dwelling","FarmsCabins","Stronghold","Dungeon","Cave","Temple","Tomb","NPC","Faction","Monster","RumorTable","WeatherTable","Shop","Unknown"].
ONLY output the JSON array, with double-quoted strings; no commentary.
INPUT:
{}
"#, self.trim_tokens(&payload));

        let parameters = ChatCompletionParametersBuilder::default()
            .model(self.model.clone())
            .messages(vec![
                ChatMessage::User { content: ChatMessageContent::Text(prompt), name: None }
            ])
            .response_format(ChatCompletionResponseFormat::Text)
            .build()?;

        let result = Self::client()
            .chat()
            .create(parameters)
            .await
            .map_err(|e| anyhow!("openai_dive chat error: {e}"))?;

        // Extract text; the API returns content as text in the first choice.
        let text = result
            .choices
            .get(0)
            .and_then(|c| c.message.as_text())
            .unwrap_or_default();

        // Parse the array of strings and map to PageType.
        let raw: Vec<String> = serde_json::from_str(&text)
            .map_err(|e| anyhow!("LLM did not return valid JSON: {e}; got: {}", text))?;

        Ok(raw.into_iter().map(|s| str_to_page_type(&s)).collect())
    }

    /// Generate Yarn Spinner dialogue for an NPC.
    pub async fn generate_npc_dialogue(&self, npc_name: &str, description: Option<&str>) -> Result<String> {
        let desc = description.unwrap_or("");
        let prompt = format!("""# System
You are writing Yarn Spinner dialogue for an RPG. Use *one* node named `Start`.
Keep 3-7 short lines, conversational, grounded in the following NPC context.
Output ONLY Yarn, nothing else.

# NPC
Name: {npc_name}
Context: {desc}

# Output (Yarn):
title: Start
---
(NPC speaks here)
===""""", npc_name=npc_name, desc=desc);
        let parameters = ChatCompletionParametersBuilder::default()
            .model(self.model.clone())
            .messages(vec![
                ChatMessage::User { content: ChatMessageContent::Text(self.trim_tokens(&prompt)), name: None }
            ])
            .response_format(ChatCompletionResponseFormat::Text)
            .build()?;

        let result = Self::client().chat().create(parameters).await
            .map_err(|e| anyhow!("openai_dive chat error: {e}"))?;

        let text = result
            .choices
            .get(0)
            .and_then(|c| c.message.as_text())
            .unwrap_or_default();

        Ok(text)
    }
}

fn str_to_page_type(s: &str) -> PageType {
    use PageType::*;
    match s {
        "Hex" => Hex,
        "Region" => Region,
        "Biome" => Biome,
        "Settlement" => Settlement,
        "City" => City,
        "Town" => Town,
        "Village" => Village,
        "Inn" => Inn,
        "Dwelling" => Dwelling,
        "FarmsCabins" => FarmsCabins,
        "Stronghold" => Stronghold,
        "Dungeon" => Dungeon,
        "Cave" => Cave,
        "Temple" => Temple,
        "Tomb" => Tomb,
        "NPC" => NPC,
        "Faction" => Faction,
        "Monster" => Monster,
        "RumorTable" => RumorTable,
        "WeatherTable" => WeatherTable,
        "Shop" => Shop,
        _ => Unknown,
    }
}

/// Cheap JSON string escaper that preserves valid ASCII and escapes quotes/newlines.
fn json_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 8);
    out.push('"');
    for ch in s.chars() {
        match ch {
            '\\' => out.push_str('\\\\'),
            '"' => out.push_str('\\\"'),
            '\n' => out.push_str('\\n'),
            '\r' => out.push_str('\\r'),
            '\t' => out.push_str('\\t'),
            c if c.is_control() => out.push(' '),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}
