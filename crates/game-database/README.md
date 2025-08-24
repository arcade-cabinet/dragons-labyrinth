# Game Database

Comprehensive database abstraction layer for Dragon's Labyrinth with SeaORM and Bevy integration.

## Overview

This crate provides the database foundation for the entire game, managing persistent state, horror progression, and serving as a structured tool for AI agents during asset generation.

## Architecture

```
game-database/
├── src/
│   ├── lib.rs              # Public API
│   ├── models.rs           # SeaORM entity definitions
│   ├── engine.rs           # Database connection management
│   ├── traits.rs           # Clean abstraction traits
│   ├── error.rs            # Error handling
│   ├── bevy_integration.rs # ECS synchronization
│   └── tools.rs            # AI agent query tools
```

## Database Models

### Core Game Systems
- **Players**: Horror progression, save slots, statistics
- **Companions**: Trauma levels, loyalty, betrayal states
- **HexTiles**: World grid with corruption progression
- **Encounters**: Boss battles and moral choices
- **Dialogues**: YarnSpinner integration with trauma variants
- **Items**: Equipment with corruption effects
- **GeneratedAssets**: Three-tier asset tracking
- **AIWorkflows**: LangGraph integration and approval tracking

### Horror Progression
- **Dread Levels**: 0-4 progression tracking
- **World Corruption**: Environmental decay metrics
- **Companion Trauma**: Individual psychological states
- **Dragon Proximity**: Stalking and danger levels

## Features

### SeaORM Integration
- Comprehensive entity models
- Automatic migrations
- Query builders
- Transaction support

### Bevy ECS Synchronization
- Database-backed components
- Automatic entity sync
- Event-driven updates
- Performance optimized batching

### AI Agent Tools
Provides structured tools for build-time AI agents:
- Query hex tiles by biome and corruption
- Get companion states and trauma levels
- Find appropriate encounters by dread level
- Retrieve dialogue variants
- Access horror progression data

## Usage

### Initialization

```rust
use game_database::initialize_database;

let db = initialize_database("sqlite://game.db").await?;
```

### Querying Data

```rust
use game_database::{GameDatabase, hex_tiles};

// Get hex tiles in radius
let tiles = db.get_hex_tiles_in_radius(0, 0, 10).await?;

// Update horror progression
db.update_horror_progression(player_id, dread_level).await?;

// Get dread-appropriate assets
let assets = db.get_dread_appropriate_assets(3, "texture").await?;
```

### AI Tool Interface

```rust
use game_database::{DatabaseTool, DatabaseQueryParams};

let tool = DatabaseTool::new(connection);

// Query companions for AI agent
let params = DatabaseQueryParams {
    query_type: "companions".to_string(),
    filters: Some(hashmap!{
        "min_loyalty" => json!(50.0),
        "is_present" => json!(true),
    }),
};

let companions = execute_database_query(&tool, params).await?;
```

### Bevy Integration

```rust
use game_database::bevy_integration::GameDatabasePlugin;

app.add_plugins(GameDatabasePlugin::new("sqlite://game.db"));

// Components automatically sync with database
#[derive(Component)]
struct DatabaseEntity {
    entity_id: Uuid,
    needs_sync: bool,
}
```

## Traits

### GameDatabaseOperations
Core game functionality:
- Save/load game states
- Manage horror progression
- Track companion states
- Handle world events

### EcsDatabaseOperations
ECS synchronization:
- Spawn entities from database
- Sync component changes
- Batch updates for performance

### AIGenerationOperations
AI workflow support:
- Track generation requests
- Store approval states
- Cache generation results

## Error Handling

Comprehensive error types with:
- User-friendly messages
- Recovery suggestions
- Severity levels
- Context preservation

## Performance

- Connection pooling
- Query optimization
- Batch operations
- Async/await throughout
- Smart caching

## Integration Points

- **build-tools**: Provides query tools for AI agents
- **game-engine**: Runtime state persistence
- **assets-library**: Asset metadata storage
