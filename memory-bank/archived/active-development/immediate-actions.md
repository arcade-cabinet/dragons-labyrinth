# Dragon's Labyrinth - Immediate Next Steps

## Critical Path Actions (Next 24-48 Hours)

This document provides SPECIFIC, ACTIONABLE steps for immediate execution. Each task includes exact files to modify, code snippets, dependencies, and success criteria.

## Priority 1: Complete Build-Tools Wiring (AF-001)

**Agent Required**: Advanced Foreground (Claude 3.5 Sonnet/Opus)
**Estimated Time**: 4-6 hours
**Dependencies**: None
**Success Criteria**: All AI agents make real OpenAI calls and connect to database

### Step 1: Clean Up Legacy Code
```bash
# DELETE completely - replaced by Rust
rm -rf src/generator/ai/
rm -rf src/generator/image/
rm -rf src/generator/audio/
# Keep src/generator/bpy_processor.py - still used by blender-bridge
```

### Step 2: Complete OpenAI API Integration

**File**: `crates/build-tools/src/agents/maps.rs`
```rust
use openai_dive::v1::api::Client;
use openai_dive::v1::resources::chat::{ChatCompletionParameters, ChatMessage, Role};

impl Agent for MapsAgent {
    async fn generate(&mut self, context: &BuildContext, request: GenerationRequest) 
        -> Result<GenerationResult> {
        
        // 1. Check cache first
        let cache_key = self.build_cache_key(&request);
        if let Some(cached) = context.get_cached_result(&cache_key).await? {
            return Ok(cached);
        }
        
        // 2. Build messages with horror context
        let system_prompt = format!(
            "Generate hex map for Dragon's Labyrinth. Dread level: {}. 
             Horror progression: {}. Use existing assets when possible.",
            request.dread_level, request.horror_context
        );
        
        let messages = vec![
            ChatMessage {
                role: Role::System,
                content: system_prompt.into(),
                ..Default::default()
            },
            ChatMessage {
                role: Role::User, 
                content: request.prompt.into(),
                ..Default::default()
            }
        ];
        
        // 3. Call OpenAI with tools
        let parameters = ChatCompletionParameters {
            model: "gpt-4o-mini".to_string(),
            messages,
            tools: Some(self.get_tools()),
            tool_choice: Some("auto".into()),
            temperature: Some(0.7),
            max_tokens: Some(2000),
            ..Default::default()
        };
        
        let response = context.openai_client.chat().create(parameters).await?;
        
        // 4. Process structured response & tool calls
        let mut result = GenerationResult::new();
        
        if let Some(message) = response.choices.first() {
            if let Some(tool_calls) = &message.message.tool_calls {
                for tool_call in tool_calls {
                    let tool_result = crate::tools::execute_tool(
                        context,
                        &tool_call.function.name,
                        &tool_call.function.arguments
                    ).await?;
                    result.add_tool_result(tool_call.id.clone(), tool_result);
                }
            }
            
            if let Some(content) = &message.message.content {
                result.set_content(content.clone());
            }
        }
        
        // 5. Generate hex map data
        let hex_data = self.process_map_generation(&result)?;
        result.set_hex_data(hex_data);
        
        // 6. Store in database & cache
        context.store_result(&cache_key, &result).await?;
        
        Ok(result)
    }
}
```

### Step 3: Wire Database Connection

**File**: `crates/build-tools/src/context.rs`
```rust
use sqlx::{SqlitePool, Row};

pub struct BuildContext {
    pub openai_client: Client,
    pub database: Option<SqlitePool>,
    pub library_catalog: AssetCatalog,
    cache: HashMap<String, GenerationResult>,
}

impl BuildContext {
    pub async fn connect_database(&mut self, database_url: &str) -> Result<()> {
        // Connect to SQLite in XDG directory (temporary until AF-003 fixes this)
        let pool = SqlitePool::connect(database_url).await
            .map_err(|e| Error::Database(format!("Connection failed: {}", e)))?;
        
        // Run migrations if needed
        sqlx::migrate!("./migrations").run(&pool).await
            .map_err(|e| Error::Database(format!("Migration failed: {}", e)))?;
        
        // Verify game-assets table exists
        let table_exists: bool = sqlx::query_scalar(
            "SELECT COUNT(*) > 0 FROM sqlite_master WHERE type='table' AND name='assets'"
        ).fetch_one(&pool).await
            .map_err(|e| Error::Database(format!("Table check failed: {}", e)))?;
        
        if !table_exists {
            return Err(Error::Database("Assets table not found - run game-assets build first".to_string()));
        }
        
        self.database = Some(pool);
        Ok(())
    }
    
    pub async fn get_cached_result(&self, key: &str) -> Result<Option<GenerationResult>> {
        if let Some(result) = self.cache.get(key) {
            return Ok(Some(result.clone()));
        }
        
        // Check database cache
        if let Some(db) = &self.database {
            let cached_json: Option<String> = sqlx::query_scalar(
                "SELECT result_json FROM generation_cache WHERE cache_key = ? AND created_at > datetime('now', '-1 day')"
            ).bind(key).fetch_optional(db).await?;
            
            if let Some(json) = cached_json {
                let result: GenerationResult = serde_json::from_str(&json)?;
                return Ok(Some(result));
            }
        }
        
        Ok(None)
    }
}
```

### Step 4: Implement Tool Execution

**File**: `crates/build-tools/src/tools.rs`
```rust
use serde_json::Value;
use sqlx::Row;

pub async fn execute_tool(context: &BuildContext, name: &str, arguments: &str) -> Result<Value> {
    let args: Value = serde_json::from_str(arguments)?;
    
    match name {
        "search_assets" => {
            let query = args["query"].as_str().ok_or(Error::InvalidToolArgs)?;
            let category = args["category"].as_str().unwrap_or("any");
            let limit = args["limit"].as_u64().unwrap_or(8) as i32;
            
            search_library_assets(context, query, category, limit).await
        },
        
        "query_database" => {
            let table = args["table"].as_str().ok_or(Error::InvalidToolArgs)?;
            let conditions = args["conditions"].as_object().unwrap_or(&serde_json::Map::new());
            
            query_game_data(context, table, conditions).await
        },
        
        "horror_progression" => {
            let dread_level = args["dread_level"].as_u64().unwrap_or(0) as u8;
            let world_state = args["world_state"].as_object().unwrap_or(&serde_json::Map::new());
            
            calculate_horror_state(dread_level, world_state).await
        },
        
        "generate_asset" => {
            let asset_type = args["asset_type"].as_str().ok_or(Error::InvalidToolArgs)?;
            let specification = args["specification"].as_object().ok_or(Error::InvalidToolArgs)?;
            
            trigger_asset_generation(context, asset_type, specification).await
        },
        
        _ => Err(Error::UnknownTool(name.to_string()))
    }
}

async fn search_library_assets(
    context: &BuildContext, 
    query: &str, 
    category: &str, 
    limit: i32
) -> Result<Value> {
    let db = context.database.as_ref().ok_or(Error::DatabaseNotConnected)?;
    
    let sql = if category == "any" {
        "SELECT file_path, metadata, attribution FROM assets 
         WHERE description LIKE ? OR tags LIKE ? 
         ORDER BY quality_score DESC LIMIT ?"
    } else {
        "SELECT file_path, metadata, attribution FROM assets 
         WHERE (description LIKE ? OR tags LIKE ?) AND category = ?
         ORDER BY quality_score DESC LIMIT ?"
    };
    
    let query_pattern = format!("%{}%", query);
    let rows = if category == "any" {
        sqlx::query(sql)
            .bind(&query_pattern)
            .bind(&query_pattern) 
            .bind(limit)
            .fetch_all(db).await?
    } else {
        sqlx::query(sql)
            .bind(&query_pattern)
            .bind(&query_pattern)
            .bind(category)
            .bind(limit)
            .fetch_all(db).await?
    };
    
    let assets: Vec<Value> = rows.iter().map(|row| {
        json!({
            "path": row.get::<String, _>("file_path"),
            "metadata": serde_json::from_str::<Value>(row.get::<String, _>("metadata")).unwrap_or(Value::Null),
            "attribution": row.get::<String, _>("attribution"),
            "reuse_score": 0.8 // Calculate based on query match
        })
    }).collect();
    
    Ok(json!({
        "found_assets": assets,
        "query": query,
        "category": category,
        "reuse_recommendation": if assets.len() > 0 { "reuse_existing" } else { "generate_new" }
    }))
}
```

### Step 5: Test Integration

**File**: `crates/build-tools/tests/integration_tests.rs`
```rust
#[tokio::test]
async fn test_maps_agent_generation() {
    let mut context = BuildContext::new("test").await.unwrap();
    context.connect_database("sqlite::memory:").await.unwrap();
    
    let mut maps_agent = MapsAgent::new();
    let request = GenerationRequest {
        prompt: "Generate a corrupted swamp hex tile".to_string(),
        dread_level: 2,
        horror_context: "Economic collapse stage".to_string(),
        ..Default::default()
    };
    
    let result = maps_agent.generate(&context, request).await.unwrap();
    
    assert!(!result.content.is_empty());
    assert!(result.hex_data.is_some());
}
```

## Priority 2: Narrative Bible Synthesis (S1M-001)

**Agent Required**: Sonnet 1M Context
**Estimated Time**: 8-12 hours
**Dependencies**: None (can run parallel with AF-001)
**Success Criteria**: Complete 3-act structure with scene breakdowns

### Required Reading Order
1. `memory-bank/design_bible.md` - Core narrative vision
2. `memory-bank/companions_reference.md` - Character arcs
3. `memory-bank/biomes_reference.md` - World progression
4. `memory-bank/larger-vision/forge-system-design.md` - Endgame systems
5. `memory-bank/expanded-vision/vision-expansion-summary.md` - Integration opportunities

### Output Structure
```
memory-bank/narrative-direction/
├── overview.md                          # Master narrative document
├── act1-journey-to-labyrinth/
│   ├── scene-breakdowns.md             # Every scene in detail  
│   ├── encounter-scripts.md            # Specific encounter dialogue
│   ├── companion-interactions.md       # How each companion behaves
│   └── horror-progression.md           # Dread 0→1 transitions
├── act2-journey-home/
│   ├── scene-breakdowns.md             # Every scene in detail
│   ├── betrayal-mechanics.md           # How betrayals trigger
│   ├── world-degradation.md            # Economic/social collapse
│   └── companion-trauma.md             # Trauma accumulation
└── act3-journey-to-void/
    ├── final-confrontation.md          # Dragon battle variations
    ├── void-mechanics.md                # The void's influence
    ├── ending-variations.md            # All possible endings
    └── narrative-closure.md            # How stories resolve
```

### Scene Documentation Template
```markdown
---
scene_id: "act1_scene_03_first_companion"
act: 1
dread_level: 0
location: "village_outskirts"  
companions_present: ["Einar"]
previous_scene: "act1_scene_02_village_arrival"
next_scenes: ["act1_scene_04_first_quest"]
---

# First Companion Meeting

## Narrative Context
Player has just arrived in the village and heard rumors about the dragon threat. This is where they meet Einar, their first and most loyal companion who will eventually break under the pressure.

## Location Details
- **Setting**: Village outskirts, near the old oak tree
- **Time**: Late afternoon, golden hour lighting
- **Atmosphere**: Peaceful but with subtle signs of unease
- **Visual Cues**: Einar sharpening his sword, eyes occasionally scanning the horizon

## Character States
- **Player**: Curious, confident, unaware of true scope
- **Einar**: Protective instinct activated, sizing up the player
- **World**: Still beautiful but shadows are slightly longer than they should be

## Dialogue Trees
```yaml
dialogue:
  - speaker: "Einar"
    emotion: "cautious"
    text: "You're not from here. What brings you to our troubled village?"
    audio_cues: ["sword_sharpening_stops.ogg"]
    responses:
      - option: "I want to help with your dragon problem"
        effect: "trust +2, hope +1"
        next: "einar_hopeful_response"
      - option: "I need something from the dragon's labyrinth"
        effect: "trust -1, suspicion +1"
        next: "einar_suspicious_response"
      - option: "Just passing through"
        effect: "trust 0, disappointment +1"
        next: "einar_disappointed_response"
```

## Required Assets
```yaml
models:
  - "village_outskirts_hex_tile.glb"
  - "einar_model_clean_state.glb"
  - "old_oak_tree_prop.glb"
textures:
  - "golden_hour_lighting.png"
  - "subtle_shadow_overlay.png"  
audio:
  - "village_ambience_peaceful.ogg"
  - "sword_sharpening.ogg"
  - "wind_through_leaves.ogg"
```

## Horror Progression Triggers
- First subtle wrongness: Shadows don't match sun position
- Audio cue: Bird song stops abruptly during conversation
- Visual cue: Einar's eyes reflect something that isn't there
- Foreshadowing: "I've been having strange dreams lately..."
```

### Companion Arc Documentation Requirements
Each companion needs complete progression through all acts:

**Einar (The Loyal Friend)**
- Act 1: Confident protector, takes charge, jokes to lighten mood
- Act 2: Questions decisions, suggests retreat, protective instincts become desperation  
- Act 3: Complete breakdown, may become catatonic or violent
- Trauma triggers: Witnessing brutality, companion abandonment, moral compromises
- Recovery paths: Player protection, avoiding trauma sources, therapeutic dialogue

### Success Criteria for S1M-001
- [ ] Every scene from game start to all endings documented
- [ ] All 4 companion arcs fully detailed with branching paths
- [ ] Horror progression triggers explicitly defined for each scene
- [ ] Asset requirements complete for each scene and transition
- [ ] Dialogue maintains voice consistency across all characters
- [ ] TOML generation agents can convert without ambiguity
- [ ] No narrative gaps or contradictions between acts

## Priority 3: Wire Game-Engine Build Dependencies (AF-002)

**Agent Required**: Advanced Foreground
**Estimated Time**: 2-3 hours
**Dependencies**: AF-001 (build-tools wiring)
**Success Criteria**: game-engine builds with generated code from OUT_DIR

### Step 1: Update game-engine/Cargo.toml
```toml
[package]
name = "game-engine"
version = "0.1.0"
edition = "2021"
build = "build.rs"

[build-dependencies]
build-tools = { path = "../build-tools" }
game-database = { path = "../game-database", features = ["build"] }

[dependencies]
bevy = { version = "0.16.1", features = ["default"] }
bevy_sqlx = "0.4.0"
hexx = "0.21"
bevy_yarnspinner = "0.3.0"
# ... other runtime dependencies
```

### Step 2: Implement game-engine/build.rs
```rust
use build_tools::{BuildContext, GenerationRequest, agents::*};
use std::env;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=rules/");
    
    let out_dir = env::var("OUT_DIR")?;
    let out_path = Path::new(&out_dir);
    
    // Create generated code directory
    let generated_dir = out_path.join("generated");
    std::fs::create_dir_all(&generated_dir)?;
    
    // Initialize build context
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async {
        let mut context = BuildContext::new(&out_dir).await?;
        
        // Connect to database (will be in OUT_DIR after AF-003)
        let database_url = format!("sqlite://{}/game.db", out_dir);
        context.connect_database(&database_url).await?;
        
        // Load rules from game-engine/rules/
        let rules_dir = Path::new("rules");
        if rules_dir.exists() {
            generate_from_rules(&mut context, rules_dir, &generated_dir).await?;
        } else {
            // Fallback: generate basic starter content
            generate_minimal_content(&mut context, &generated_dir).await?;
        }
        
        Ok::<(), Box<dyn std::error::Error>>(())
    })?;
    
    // Tell Rust to include generated code
    println!("cargo:rustc-env=GENERATED_CODE_DIR={}", generated_dir.display());
    
    Ok(())
}

async fn generate_from_rules(
    context: &mut BuildContext,
    rules_dir: &Path, 
    out_dir: &Path
) -> Result<(), Box<dyn std::error::Error>> {
    
    // Generate maps
    if rules_dir.join("maps").exists() {
        let mut maps_agent = MapsAgent::new();
        let request = GenerationRequest::from_rules_dir(rules_dir.join("maps"))?;
        let result = maps_agent.generate(context, request).await?;
        
        std::fs::write(
            out_dir.join("maps.rs"),
            result.generated_code
        )?;
    }
    
    // Generate encounters  
    if rules_dir.join("encounters").exists() {
        let mut levels_agent = LevelsAgent::new();
        let request = GenerationRequest::from_rules_dir(rules_dir.join("encounters"))?;
        let result = levels_agent.generate(context, request).await?;
        
        std::fs::write(
            out_dir.join("encounters.rs"),
            result.generated_code
        )?;
    }
    
    // Generate dialogue
    if rules_dir.join("dialogue").exists() {
        let mut dialogue_agent = DialogueAgent::new();
        let request = GenerationRequest::from_rules_dir(rules_dir.join("dialogue"))?;
        let result = dialogue_agent.generate(context, request).await?;
        
        std::fs::write(
            out_dir.join("dialogue.rs"),
            result.generated_code
        )?;
    }
    
    Ok(())
}
```

### Step 3: Create Rule Structure
```bash
mkdir -p crates/game-engine/rules/{maps,encounters,dialogue,progression}
```

**File**: `crates/game-engine/rules/maps/basic_overworld.toml`
```toml
[metadata]
id = "basic_overworld_map"
dread_level = 0
act = 1
priority = 1

[generation]
prompt_template = """
Generate a hex-based overworld map for the peaceful stage of Dragon's Labyrinth.
The world should appear beautiful but with subtle signs of wrongness.
Include 7 hex tile types: grass, forest, hills, water, village, road, ancient_site.
Each hex should be 5 units across for Bevy rendering.
"""
required_assets = ["grass_hex", "forest_hex", "village_hex"]
ai_model = "gpt-4o-mini"

[validation]
required_fields = ["hex_layout", "tile_assignments", "spawn_point"]
constraints = { max_hexes = 100, min_hexes = 50 }
```

### Step 4: Include Generated Code
**File**: `crates/game-engine/src/generated.rs`
```rust
// Include all generated modules from OUT_DIR
include!(concat!(env!("OUT_DIR"), "/generated/maps.rs"));
include!(concat!(env!("OUT_DIR"), "/generated/encounters.rs"));
include!(concat!(env!("OUT_DIR"), "/generated/dialogue.rs"));

pub use maps::*;
pub use encounters::*;
pub use dialogue::*;
```

**File**: `crates/game-engine/src/main.rs`
```rust
mod generated;

use bevy::prelude::*;
use generated::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_generated_world)
        .add_systems(Update, update_generated_systems)
        .run();
}

fn setup_generated_world(mut commands: Commands) {
    // Use generated map data
    spawn_generated_world(&mut commands);
}
```

### Step 5: Test Build
```bash
cd crates/game-engine
cargo build
# Should succeed and include generated code in target/
```

## Success Validation

### AF-001 Complete When:
- [ ] All 5 agents (Maps, Levels, UI, Dialogue, Audio) make real OpenAI API calls
- [ ] Database connection established and queries work
- [ ] Tools execute with real data from game-assets library
- [ ] Cache system prevents duplicate generation
- [ ] Legacy Python AI code deleted
- [ ] Integration tests pass

### S1M-001 Complete When:
- [ ] Complete 3-act narrative structure documented
- [ ] All companion arcs detailed with trauma progression
- [ ] Every scene has asset requirements specified
- [ ] Horror progression triggers explicitly defined
- [ ] Dialogue trees maintain voice consistency
- [ ] Background agents can generate TOML rules from documentation

### AF-002 Complete When:
- [ ] game-engine builds successfully with build-tools dependency
- [ ] Generated code included from OUT_DIR works
- [ ] Rule structure created and loading
- [ ] Basic content generation functional
- [ ] No circular dependencies

## Next Steps After Immediate Actions

1. **AF-003**: Fix database architecture (move to OUT_DIR)
2. **CB-001**: Generate TOML rules from narrative synthesis
3. **SS-001**: Complete blender-bridge implementation
4. **Content Pipeline**: Begin full asset generation
5. **Integration Testing**: End-to-end workflow validation

## Critical Notes

- **Parallel Execution**: AF-001 and S1M-001 can run simultaneously
- **Quality Over Speed**: Better to complete one task perfectly than three partially
- **Documentation**: Document all architectural decisions made
- **Testing**: Test each integration point before moving on
- **Communication**: Update task progress regularly for coordination

The next 48 hours are CRITICAL for project success. Focus, execute, deliver.
