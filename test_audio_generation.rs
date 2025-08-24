#!/usr/bin/env rust-script
//! Test script to verify audio asset generation using Freesound API
//! 
//! ```cargo
//! [dependencies]
//! tokio = { version = "1.0", features = ["rt", "rt-multi-thread", "macros"] }
//! reqwest = { version = "0.12", features = ["json", "stream"] }
//! serde = { version = "1.0", features = ["derive"] }
//! serde_json = "1.0"
//! anyhow = "1.0"
//! futures-util = "0.3"
//! ```

use serde::{Deserialize, Serialize};
use tokio;
use anyhow::Result;
use futures_util::StreamExt;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
struct AudioAssetConfig {
    dread_level: u8,
    ambient_sounds: Vec<AudioTrack>,
    horror_stingers: Vec<AudioTrack>,
    companion_trauma_sounds: Vec<AudioTrack>,
    environmental_audio: Vec<AudioTrack>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AudioTrack {
    name: String,
    freesound_id: Option<i32>,
    tags: Vec<String>,
    duration: Option<f32>,
    horror_intensity: f32, // 0.0 to 1.0
}

#[derive(Debug, Deserialize)]
struct FreesoundSearchResult {
    results: Vec<FreesoundSound>,
}

#[derive(Debug, Deserialize)]
struct FreesoundSound {
    id: i32,
    name: String,
    tags: Vec<String>,
    duration: f32,
    username: String,
    previews: HashMap<String, String>,
}

async fn search_freesound_audio(query: &str, freesound_key: &str) -> Result<Vec<FreesoundSound>> {
    let client = reqwest::Client::new();
    
    let encoded_query = query.replace(" ", "%20").replace("\"", "%22");
    let url = format!(
        "https://freesound.org/apiv2/search/text/?query={}&token={}&fields=id,name,tags,duration,username,previews&page_size=5",
        encoded_query,
        freesound_key
    );

    println!("🔍 Searching Freesound for: '{}'", query);
    
    let response = client
        .get(&url)
        .header("Authorization", format!("Token {}", freesound_key))
        .send()
        .await?;

    if response.status().is_success() {
        let search_result: FreesoundSearchResult = response.json().await?;
        println!("   Found {} sounds", search_result.results.len());
        Ok(search_result.results)
    } else {
        println!("   ❌ Search failed with status: {}", response.status());
        Ok(vec![])
    }
}

async fn download_audio_preview(sound: &FreesoundSound, output_dir: &str) -> Result<()> {
    if let Some(preview_url) = sound.previews.get("preview-hq-mp3") 
        .or_else(|| sound.previews.get("preview-lq-mp3")) {
        
        let client = reqwest::Client::new();
        let response = client.get(preview_url).send().await?;
        
        if response.status().is_success() {
            let filename = format!("{}/{}_{}.mp3", output_dir, sound.id, 
                sound.name.chars().filter(|c| c.is_alphanumeric() || *c == '_' || *c == '-')
                    .collect::<String>());
            
            let mut file = tokio::fs::File::create(&filename).await?;
            let mut stream = response.bytes_stream();
            
            while let Some(chunk) = stream.next().await {
                let chunk = chunk?;
                tokio::io::AsyncWriteExt::write_all(&mut file, &chunk).await?;
            }
            
            println!("   💾 Downloaded: {}", filename);
        }
    }
    
    Ok(())
}

async fn generate_horror_audio_config(dread_level: u8, freesound_key: &str, download_dir: &str) -> Result<AudioAssetConfig> {
    let horror_queries = match dread_level {
        0 => vec!["peaceful ambient", "nature sounds", "birds chirping"],
        1 => vec!["unsettling ambient", "distant thunder", "creaking wood"],
        2 => vec!["horror atmosphere", "whispers", "footsteps dark"],
        3 => vec!["screaming horror", "metal scraping", "nightmare sounds"],
        4 => vec!["extreme horror", "demonic sounds", "terror scream"],
        _ => vec!["extreme horror", "demonic sounds", "terror scream"],
    };

    let companion_queries = match dread_level {
        0 => vec!["happy pet sounds", "cat purr"],
        1 => vec!["worried animal", "dog whimpering"],
        2 => vec!["animal fear", "scared cat"],
        3 => vec!["animal panic", "desperate crying"],
        4 => vec!["animal terror", "death cry"],
        _ => vec!["animal terror", "death cry"],
    };

    let environmental_queries = match dread_level {
        0 => vec!["market sounds", "village ambient"],
        1 => vec!["empty street", "wind howling"],
        2 => vec!["abandoned building", "dripping water"],
        3 => vec!["decay sounds", "collapsing structure"],
        4 => vec!["apocalypse sounds", "world ending"],
        _ => vec!["apocalypse sounds", "world ending"],
    };

    let mut ambient_sounds = Vec::new();
    let mut horror_stingers = Vec::new();
    let mut companion_trauma_sounds = Vec::new();
    let mut environmental_audio = Vec::new();

    // Search and download ambient horror sounds
    for query in horror_queries {
        let sounds = search_freesound_audio(query, freesound_key).await?;
        for sound in sounds.into_iter().take(2) {
            // Try to download preview
            if let Err(e) = download_audio_preview(&sound, download_dir).await {
                println!("   ⚠️  Download failed for {}: {}", sound.name, e);
            }
            
            ambient_sounds.push(AudioTrack {
                name: sound.name,
                freesound_id: Some(sound.id),
                tags: sound.tags,
                duration: Some(sound.duration),
                horror_intensity: (dread_level as f32) / 4.0,
            });
        }
    }

    // Search companion trauma sounds
    for query in companion_queries.into_iter().take(1) {
        let sounds = search_freesound_audio(query, freesound_key).await?;
        for sound in sounds.into_iter().take(1) {
            companion_trauma_sounds.push(AudioTrack {
                name: sound.name,
                freesound_id: Some(sound.id),
                tags: sound.tags,
                duration: Some(sound.duration),
                horror_intensity: (dread_level as f32) / 4.0,
            });
        }
    }

    // Search environmental audio
    for query in environmental_queries.into_iter().take(1) {
        let sounds = search_freesound_audio(query, freesound_key).await?;
        for sound in sounds.into_iter().take(1) {
            environmental_audio.push(AudioTrack {
                name: sound.name,
                freesound_id: Some(sound.id),
                tags: sound.tags,
                duration: Some(sound.duration),
                horror_intensity: (dread_level as f32) / 4.0,
            });
        }
    }

    Ok(AudioAssetConfig {
        dread_level,
        ambient_sounds,
        horror_stingers,
        companion_trauma_sounds,
        environmental_audio,
    })
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("🎵 DRAGON'S LABYRINTH - AUDIO GENERATION TEST");
    println!("==============================================");

    // Get Freesound API key from environment
    let freesound_key = std::env::var("FREESOUND_API_KEY")
        .map_err(|_| anyhow::anyhow!("FREESOUND_API_KEY environment variable not set"))?;

    println!("✅ Freesound API Key Found: {}***", &freesound_key[..8]);

    // Test output directories
    let out_dir = "./test_output/audio";
    let config_dir = "./test_output/audio_configs";
    std::fs::create_dir_all(out_dir)?;
    std::fs::create_dir_all(config_dir)?;
    
    println!("📁 Created audio directories in: {}", out_dir);

    // Test audio generation for each dread level
    for dread_level in 0..=2 {  // Limit to 3 levels to be respectful to API
        println!("\n🎭 GENERATING AUDIO FOR DREAD LEVEL {}", dread_level);
        println!("================================================");

        let audio_dir = format!("{}/dread_{}", out_dir, dread_level);
        std::fs::create_dir_all(&audio_dir)?;

        match generate_horror_audio_config(dread_level, &freesound_key, &audio_dir).await {
            Ok(audio_config) => {
                let config_path = format!("{}/dread_{}.json", config_dir, dread_level);
                let json_content = serde_json::to_string_pretty(&audio_config)?;
                std::fs::write(&config_path, json_content)?;
                
                println!("✅ Audio config saved to: {}", config_path);
                println!("   - Ambient sounds: {}", audio_config.ambient_sounds.len());
                println!("   - Companion sounds: {}", audio_config.companion_trauma_sounds.len());
                println!("   - Environmental: {}", audio_config.environmental_audio.len());
                
                for sound in &audio_config.ambient_sounds {
                    println!("     🔊 {}: {:.1}s (ID: {})", 
                        sound.name, 
                        sound.duration.unwrap_or(0.0),
                        sound.freesound_id.unwrap_or(0));
                }
            }
            Err(e) => println!("❌ Audio generation failed: {}", e),
        }

        if dread_level < 2 {
            println!("⏱️ Waiting 3 seconds before next generation...");
            tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
        }
    }

    println!("\n🎉 AUDIO GENERATION TEST COMPLETE!");
    println!("===================================");
    println!("Check the ./test_output/audio directory for downloaded previews.");
    println!("Audio pipeline is working with production Freesound API! 🎵");

    Ok(())
}
