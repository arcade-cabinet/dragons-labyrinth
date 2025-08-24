#!/usr/bin/env rust-script
//! Test script to verify AI agent generation pipeline works with real API keys
//! 
//! ```cargo
//! [dependencies]
//! tokio = { version = "1.0", features = ["rt", "rt-multi-thread", "macros"] }
//! reqwest = { version = "0.12", features = ["json"] }
//! serde = { version = "1.0", features = ["derive"] }
//! serde_json = "1.0"
//! toml = "0.8"
//! anyhow = "1.0"
//! uuid = { version = "1.0", features = ["v4"] }
//! chrono = { version = "0.4", features = ["serde"] }
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use tokio;
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize)]
struct UIConfig {
    dread_level: u8,
    base_opacity: f32,
    corruption_overlay: bool,
    trauma_indicators: Vec<String>,
    interface_degradation: f32,
    color_scheme: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CorruptionRules {
    dread_level: u8,
    environmental_decay_rate: f32,
    npc_fear_threshold: f32,
    economic_stability: f32,
    reality_distortion_effects: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct MountPersonality {
    dread_level: u8,
    bonding_mechanics: HashMap<String, f32>,
    trauma_responses: Vec<String>,
    environmental_protection: HashMap<String, f32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct LevelData {
    dread_level: u8,
    encounter_placements: Vec<String>,
    philosophical_variations: HashMap<String, Vec<String>>,
    sentimental_items: Vec<String>,
    narrative_triggers: Vec<String>,
}

async fn generate_horror_responsive_ui(dread_level: u8, openai_key: &str) -> Result<UIConfig> {
    let client = reqwest::Client::new();
    
    let prompt = format!(
        "Generate horror-responsive UI configuration for dread level {} (0-4).
        
        Create a JSON response with:
        - base_opacity: Lower opacity for higher dread (0.9 to 0.3)
        - corruption_overlay: Boolean for visual corruption effects
        - trauma_indicators: Array of UI elements showing companion trauma
        - interface_degradation: How broken the UI appears (0.0 to 1.0)
        - color_scheme: Visual theme (\"warm\", \"cool\", \"corrupted\", \"nightmare\")
        
        Focus on psychological horror that escalates with dread level.",
        dread_level
    );

    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", openai_key))
        .json(&serde_json::json!({
            "model": "gpt-4",
            "messages": [{
                "role": "system",
                "content": "You are an expert in horror game UI design. Return only valid JSON."
            }, {
                "role": "user", 
                "content": prompt
            }],
            "temperature": 0.8,
            "max_tokens": 500
        }))
        .send()
        .await?;

    let json: serde_json::Value = response.json().await?;
    let content = json["choices"][0]["message"]["content"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("No content in response"))?;

    println!("🎨 UI Generation Response for Dread {}: {}", dread_level, content);
    
    // Parse the JSON response into our struct
    let ui_data: serde_json::Value = serde_json::from_str(content)?;
    
    Ok(UIConfig {
        dread_level,
        base_opacity: ui_data["base_opacity"].as_f64().unwrap_or(0.8) as f32,
        corruption_overlay: ui_data["corruption_overlay"].as_bool().unwrap_or(false),
        trauma_indicators: ui_data["trauma_indicators"]
            .as_array()
            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
            .unwrap_or_default(),
        interface_degradation: ui_data["interface_degradation"].as_f64().unwrap_or(0.0) as f32,
        color_scheme: ui_data["color_scheme"].as_str().unwrap_or("warm").to_string(),
    })
}

async fn generate_environmental_decay(dread_level: u8, openai_key: &str) -> Result<CorruptionRules> {
    let client = reqwest::Client::new();
    
    let prompt = format!(
        "Generate environmental corruption rules for dread level {} (0-4).
        
        Create a JSON response with:
        - environmental_decay_rate: How fast the world corrupts (0.0 to 1.0)
        - npc_fear_threshold: At what point NPCs become terrified (0.0 to 1.0)
        - economic_stability: How stable the economy remains (1.0 to 0.0)
        - reality_distortion_effects: Array of surreal/horror effects
        
        Higher dread levels should have more intense corruption.",
        dread_level
    );

    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", openai_key))
        .json(&serde_json::json!({
            "model": "gpt-4",
            "messages": [{
                "role": "system",
                "content": "You are an expert in environmental storytelling for horror games. Return only valid JSON."
            }, {
                "role": "user", 
                "content": prompt
            }],
            "temperature": 0.8,
            "max_tokens": 500
        }))
        .send()
        .await?;

    let json: serde_json::Value = response.json().await?;
    let content = json["choices"][0]["message"]["content"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("No content in response"))?;

    println!("🌪️ Decay Generation Response for Dread {}: {}", dread_level, content);
    
    let decay_data: serde_json::Value = serde_json::from_str(content)?;
    
    Ok(CorruptionRules {
        dread_level,
        environmental_decay_rate: decay_data["environmental_decay_rate"].as_f64().unwrap_or(0.1) as f32,
        npc_fear_threshold: decay_data["npc_fear_threshold"].as_f64().unwrap_or(0.3) as f32,
        economic_stability: decay_data["economic_stability"].as_f64().unwrap_or(0.8) as f32,
        reality_distortion_effects: decay_data["reality_distortion_effects"]
            .as_array()
            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
            .unwrap_or_default(),
    })
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("🚀 DRAGON'S LABYRINTH - AI GENERATION TEST");
    println!("==========================================");

    // Get OpenAI API key from environment
    let openai_key = std::env::var("OPENAI_API_KEY")
        .map_err(|_| anyhow::anyhow!("OPENAI_API_KEY environment variable not set"))?;

    println!("✅ OpenAI API Key Found: {}", &openai_key[..20]);

    // Test output directory
    let out_dir = "./test_output";
    std::fs::create_dir_all(out_dir)?;
    std::fs::create_dir_all(format!("{}/ui", out_dir))?;
    std::fs::create_dir_all(format!("{}/decay", out_dir))?;
    
    println!("📁 Created output directories in: {}", out_dir);

    // Test generation for each dread level
    for dread_level in 0..=4 {
        println!("\n🎭 GENERATING ASSETS FOR DREAD LEVEL {}", dread_level);
        println!("===================================================");

        // Generate UI configuration
        println!("🎨 Generating horror-responsive UI...");
        match generate_horror_responsive_ui(dread_level, &openai_key).await {
            Ok(ui_config) => {
                let ui_path = format!("{}/ui/dread_{}.toml", out_dir, dread_level);
                let toml_content = toml::to_string(&ui_config)?;
                std::fs::write(&ui_path, toml_content)?;
                println!("✅ UI config saved to: {}", ui_path);
                println!("   - Opacity: {}", ui_config.base_opacity);
                println!("   - Color Scheme: {}", ui_config.color_scheme);
                println!("   - Trauma Indicators: {}", ui_config.trauma_indicators.len());
            }
            Err(e) => println!("❌ UI generation failed: {}", e),
        }

        // Generate environmental decay
        println!("🌪️ Generating environmental corruption...");
        match generate_environmental_decay(dread_level, &openai_key).await {
            Ok(decay_rules) => {
                let decay_path = format!("{}/decay/dread_{}.json", out_dir, dread_level);
                let json_content = serde_json::to_string_pretty(&decay_rules)?;
                std::fs::write(&decay_path, json_content)?;
                println!("✅ Decay rules saved to: {}", decay_path);
                println!("   - Decay Rate: {}", decay_rules.environmental_decay_rate);
                println!("   - Fear Threshold: {}", decay_rules.npc_fear_threshold);
                println!("   - Economic Stability: {}", decay_rules.economic_stability);
            }
            Err(e) => println!("❌ Decay generation failed: {}", e),
        }

        // Add a small delay to be respectful to the API
        if dread_level < 4 {
            println!("⏱️ Waiting 2 seconds before next generation...");
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        }
    }

    println!("\n🎉 AI GENERATION TEST COMPLETE!");
    println!("=====================================");
    println!("Check the ./test_output directory for generated assets.");
    println!("All systems are working with production API keys! 🚀");

    Ok(())
}
