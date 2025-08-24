# Build Tools

AI-powered build tools for Dragon's Labyrinth asset generation using structured AI agents.

## Overview

This crate provides the build-time asset generation system using OpenAI's GPT-4 and structured tools. It implements the revolutionary 80/20 asset strategy: 80% intelligent reuse of CC0 assets, 20% targeted AI generation for horror-specific content.

## Architecture

```
build-tools/
├── src/
│   ├── lib.rs          # Public API
│   ├── agents/         # Domain-specific AI agents
│   │   ├── maps.rs     # Hex world generation
│   │   ├── levels.rs   # Encounter placement
│   │   ├── ui.rs       # Horror UI generation
│   │   ├── dialogue.rs # YarnSpinner dialogue
│   │   └── audio.rs    # Spatial audio
│   ├── context.rs      # Build context and config
│   ├── error.rs        # Error types
│   ├── generation.rs   # Generation requests/results
│   ├── memory.rs       # Agent memory system
│   └── tools.rs        # OpenAI tool definitions
```

## Features

### Structured AI Agents
- **MapsAgent**: Hexx-compatible hex world generation
- **LevelsAgent**: Yoleck encounter placement
- **UIAgent**: Cobweb horror-responsive interfaces
- **DialogueAgent**: YarnSpinner narrative generation
- **AudioAgent**: Freesound integration and spatial audio

### Smart Asset Selection
1. **Search First**: Query CC0 library before generating
2. **Cache Results**: Reuse previous generations
3. **Generate Only When Needed**: Fill specific gaps
4. **Horror-Aware**: All agents understand dread progression

### Database Integration
- Queries game-database for context
- Uses ECS data to inform generation
- Maintains consistency across agents

## Usage

### In build.rs

```rust
use build_tools::{BuildContext, GenerationRequest};

#[tokio::main]
async fn main() {
    // Initialize build context
    let mut context = BuildContext::new("assets/generated")?;
    
    // Connect to game database
    context.connect_database("sqlite://game.db").await?;
    
    // Generate assets for each dread level
    for dread_level in 0..=4 {
        let request = GenerationRequest::new(
            "hex_world",
            dread_level,
            "Generate world for horror progression"
        );
        
        let result = context.generate(request).await?;
        println!("Generated: {}", result.asset_id);
    }
}
```

### Configuration

```rust
use build_tools::BuildConfig;

let config = BuildConfig {
    model: "gpt-4o".to_string(),
    max_tokens: 4000,
    temperature: 0.7,
    enable_cache: true,
    enable_database_tools: true,
    dread_level: 0,
    asset_categories: vec![
        "hex_tiles".to_string(),
        "companions".to_string(),
    ],
};
```

## Memory System

Each agent maintains context through:
- **Conversation History**: Recent AI interactions
- **Generation Cache**: Previous results
- **Horror State**: Current dread level and world state
- **Companion States**: Trauma and loyalty tracking

## Tools

The system provides structured tools for:
- **search_assets**: Query CC0 library
- **query_game_database**: Get game context
- **get_horror_progression**: Understand dread levels
- **generate_asset**: Create new content

## Integration

- **game-database**: Provides runtime context via tools
- **assets-library**: Receives generated assets
- **openai_dive**: Structured AI interactions
- **tiktoken-rs**: Token counting and management

## Cost Optimization

- Token counting before requests
- Caching to avoid regeneration
- Batch processing when possible
- Smart truncation of context

## Error Handling

Comprehensive error types for:
- API failures
- Token limits
- Database errors
- Generation failures
- Tool execution errors
