# Dragon's Labyrinth - Technical Context

## REVOLUTIONARY TECHNOLOGY STACK (January 2025)

### Dual-Language Architecture
**CRITICAL**: Complete separation of build-time AI generation from runtime game engine

#### Build-Time Asset Generation (Rust + RON)
- **Language**: Rust (build.rs scripts)
- **Model Definitions**: RON format (declarative 3D model specs)
- **GLTF Generation**: Direct creation without Blender dependency
- **AI Integration**: OpenAI GPT-4o for narrative content
- **Audio**: Freesound API for sound effects
- **Caching**: XDG-compliant project caching
- **No External Dependencies**: CI/CD friendly, pure Rust solution

#### Runtime Game Engine (Rust)
- **Language**: Rust (Edition 2021, Version 1.88+)
- **Game Engine**: Bevy 0.16.1
- **Build Targets**: Native (Linux/Windows/Mac) + WebAssembly
- **Graphics**: wgpu (cross-platform graphics API)
- **Audio**: Bevy Audio (Rodio backend)

### Python AI Dependencies

#### Core AI Libraries
- **langchain**: Core agent framework and tool integration
- **langchain-openai**: OpenAI API integration for GPT-4o and DALL-E 3
- **langgraph**: Durable workflow execution with SQLite checkpointing
- **openai**: Direct OpenAI API access for specialized operations
- **sentence-transformers**: Semantic similarity for asset search
- **vector-db**: Embedding storage and retrieval

#### Database & Processing
- **sqlalchemy**: ORM for complex database operations
- **sqlite3**: Lightweight database for development and deployment
- **pandas**: Data processing for asset metadata analysis
- **numpy**: Numerical operations for asset analysis
- **pillow**: Image processing for texture manipulation
- **librosa**: Audio analysis and processing

#### 3D Asset Generation
- **bpy**: Blender Python API for automated 3D model creation
- **mathutils**: Blender mathematical operations
- **bmesh**: Blender mesh editing operations
- **gltf-tools**: GLTF file format manipulation

#### Job Queue & Workflow
- **huey**: Redis-like task queue with SQLite backend
- **asyncio**: Asynchronous operation handling
- **concurrent.futures**: Parallel processing for batch operations
- **schedule**: Periodic task management

### Rust Game Dependencies

#### Game Systems (GENERATOR-FREE)
- **hexx** (0.21): Hexagonal grid math, pathfinding, FOV
- **bevy_ecs_tilemap** (0.16): Efficient tile rendering
- **bevy_yarnspinner**: Dialogue system integration
- **yoleck**: Level editor integration for encounter placement
- **cobweb-ui**: Declarative UI system for horror-responsive interfaces

#### Core Libraries (MAINTAINED)
- **serde** (1.0): Serialization for save games and asset metadata
- **serde_json** (1.0): JSON data format for AI-generated content
- **rand** (0.8): Random number generation
- **fastrand** (2.0): Fast RNG for non-crypto uses

#### Database & Storage (CONSUMPTION ONLY)
- **rusqlite** (0.32): SQLite integration for consuming generated data
- **serde_rusqlite**: Serde integration for SQLite queries

#### Development Tools
- **wasm-bindgen**: WebAssembly bindings
- **web-sys**: Web API access
- **console_error_panic_hook**: WASM debugging
- **wasm-bindgen-futures**: Async support in WASM

## Development Setup

### Prerequisites (UPDATED)
```bash
# Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default stable
rustup target add wasm32-unknown-unknown

# WASM tools
cargo install wasm-bindgen-cli
cargo install basic-http-server

# Python environment (NEW)
python3 -m venv dragon-ai-env
source dragon-ai-env/bin/activate
pip install langchain langgraph openai sqlalchemy huey pillow librosa

# Blender (for 3D asset generation)
# Download Blender 4.0+ and add to PATH
# Or use system package manager

# Optional: Database tools
cargo install diesel_cli --no-default-features --features sqlite
```

### Project Structure (REVOLUTIONARY)
```
dragons-labyrinth/
├── crates/                          # Rust workspace (GENERATOR-FREE)
│   ├── game/                       # Main game crate
│   │   ├── src/
│   │   │   ├── main.rs             # Entry point
│   │   │   ├── components/         # ECS components
│   │   │   ├── systems/            # Game systems (NO AI)
│   │   │   ├── resources/          # Global resources
│   │   │   ├── assets/             # Asset loading (consumes AI output)
│   │   │   ├── dialogue/           # YarnSpinner integration
│   │   │   ├── board/              # Board rendering
│   │   │   └── hex_board/          # Hex grid logic
│   │   └── Cargo.toml
│   ├── core/                       # Shared components
│   ├── audio/                      # Audio systems
│   ├── ui/                         # UI systems
│   └── maps/                       # Map systems
├── dragon-ai/                      # Python agentic workflows (NEW)
│   ├── agents/
│   │   ├── maps_agent.py           # Hexx world generation
│   │   ├── levels_agent.py         # Yoleck encounter placement
│   │   ├── ui_agent.py             # Cobweb horror UI
│   │   ├── dialogue_agent.py       # YarnSpinner narrative
│   │   └── audio_agent.py          # Freesound + proximity horror
│   ├── workflows/
│   │   ├── asset_generation.py     # LangGraph orchestration
│   │   ├── human_review.py         # Review interface
│   │   └── batch_processing.py     # Job queue management
│   ├── database/
│   │   ├── models.py               # SQLAlchemy models
│   │   ├── asset_search.py         # Semantic search tools
│   │   └── migrations/             # Database migrations
│   ├── tools/
│   │   ├── dragon_search_tool.py   # Asset database search
│   │   ├── blender_automation.py   # 3D model generation
│   │   └── freesound_integration.py# CC0 audio acquisition
│   └── requirements.txt
├── assets/                         # Three-tier asset structure
│   ├── core/                       # Sacred assets (AI never touches)
│   │   ├── intro_video.mp4
│   │   ├── outro_video.mp4
│   │   └── dragon_roar.ogg
│   ├── library/                    # CC0 collection (AI searches)
│   │   ├── models/                 # .gltf files (Bevy direct loading)
│   │   ├── textures/               # Base textures
│   │   └── audio/                  # Base audio
│   └── generated/                  # AI output (fills gaps)
│       ├── hex_tiles/              # Dread-level variants
│       ├── companion_trauma/       # Character progression
│       ├── horror_ui/              # Degrading interfaces
│       └── proximity_audio/        # Spatial horror audio
├── memory-bank/                    # Documentation
├── target/                         # Rust build outputs
├── ai-output/                      # Python generation outputs (NEW)
└── Cargo.toml                      # Workspace dependencies
```

### Build Commands (UPDATED)

#### Python AI Generation (NEW)
```bash
# Activate Python environment
source dragon-ai-env/bin/activate

# Generate all game assets
python -m dragon-ai.main --stage peace --generate-all

# Generate specific domain assets
python -m dragon-ai.main --agent maps --dread-level 2
python -m dragon-ai.main --agent dialogue --companion einar --trauma 0.6

# Run human-in-the-loop review interface
python -m dragon-ai.review_interface --port 8080

# Batch process with job queue
python -m dragon-ai.batch_process --config production.yaml
```

#### Rust Game Engine (MAINTAINED)
```bash
# Debug build (fast compile, slow runtime)
cargo run

# Release build (slow compile, fast runtime)  
cargo build --release
./target/release/dragons_labyrinth

# Run with features
cargo run --features debug_ui
```

#### WebAssembly Build (MAINTAINED)
```bash
# Build WASM module
./build_wasm.sh

# Or manually:
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./out/ --target web \
    ./target/wasm32-unknown-unknown/release/dragons_labyrinth.wasm

# Serve locally
python3 -m http.server 8000
# Visit http://localhost:8000
```

## Technical Constraints (UPDATED)

### Performance Requirements (MAINTAINED)
- **Frame Rate**: 60 FPS (desktop), 30 FPS (mobile)
- **Memory**: < 200MB total usage
- **Load Time**: < 2 seconds per area
- **Hex Tiles**: Support 10,000+ simultaneous

### AI Generation Constraints (NEW)
- **API Rate Limits**: OpenAI tier-based quotas
- **Generation Time**: Human patience limits (~30 seconds per asset)
- **Quality Thresholds**: Human review standards (>0.8 approval rating)
- **Cost Limits**: Budget for API calls ($200/month target)
- **Storage Limits**: Generated asset database size (<10GB)

### Platform Constraints (ENHANCED)

#### WebAssembly Limitations (MAINTAINED)
- No threading (use Bevy's async systems)
- Limited memory (4GB maximum)
- No file system access (use browser storage)
- Audio restrictions (user interaction required)

#### Mobile Considerations (ENHANCED)
- Touch input only (no hover states)
- Limited GPU memory (prefer CC0 library assets)
- Battery usage optimization (smart asset streaming)
- Variable screen sizes/ratios (responsive AI-generated UI)

### Asset Constraints (ENHANCED)
- **Models**: .glb format, < 100k vertices, Bevy-compatible
- **Textures**: Power-of-2 dimensions, < 2048x2048, mobile-optimized
- **Audio**: .ogg format, compressed, spatial audio compatible
- **Total Size**: < 50MB initial download (core + library assets)
- **Generated Size**: < 100MB additional for complete horror progression

## Development Patterns (REVOLUTIONARY)

### Python Agentic Patterns (NEW)

#### LangGraph Workflow Pattern
```python
from langgraph.graph import StateGraph
from langchain.tools import StructuredTool

class DragonAgent(BaseAgent):
    def build_workflow(self) -> StateGraph:
        workflow = StateGraph(DragonAgentState)
        
        # Phase 1: Search existing assets
        workflow.add_node("search_assets", self.search_cc0_library)
        # Phase 2: Evaluate suitability
        workflow.add_node("evaluate_match", self.evaluate_asset_match)
        # Phase 3: Human review
        workflow.add_node("human_review", self.human_review_node)
        # Phase 4: AI generation (if needed)
        workflow.add_node("ai_generate", self.ai_generate_asset)
        # Phase 5: Export for Rust consumption
        workflow.add_node("export_asset", self.export_for_rust)
        
        # Conditional routing based on asset match quality
        workflow.add_conditional_edges(
            "evaluate_match",
            self.should_generate_or_reuse,
            {
                "use_existing": "human_review",
                "enhance_existing": "ai_generate", 
                "generate_new": "ai_generate"
            }
        )
        
        return workflow.compile(checkpointer=self.sqlite_checkpointer)
```

#### Database-First Asset Selection
```python
from langchain.tools import StructuredTool
from dragon_ai.database.asset_search import semantic_search

@StructuredTool
def search_dragon_assets(
    query: str,           # "corrupted medieval stone hex tile"
    dread_level: int,     # 0-4 horror progression context
    category: str,        # "textures", "models", "audio" 
    limit: int = 8
) -> List[AssetMetadata]:
    """
    Search CC0 asset library with horror progression awareness.
    Returns ranked results prioritizing performance and suitability.
    """
    # Semantic similarity search
    results = semantic_search(query, category, limit * 2)
    
    # Filter and rank by horror compatibility
    filtered = []
    for asset in results:
        horror_score = calculate_horror_compatibility(asset, dread_level)
        performance_score = calculate_performance_suitability(asset)
        
        if horror_score > 0.5 and performance_score > 0.7:
            asset.composite_score = horror_score * performance_score
            filtered.append(asset)
    
    # Return top matches
    return sorted(filtered, key=lambda x: x.composite_score, reverse=True)[:limit]
```

#### Human-in-the-Loop Pattern
```python
from langgraph.types import interrupt

def human_review_node(self, state: AgentState) -> dict:
    """Present generated content for human approval."""
    
    review_data = {
        "agent_type": self.domain,
        "dread_level": state.current_dread_level,
        "asset_candidates": state.candidate_assets,
        "generation_context": state.requirements,
        "estimated_cost": state.generation_cost,
        "performance_impact": state.performance_analysis
    }
    
    # Interrupt workflow for human review
    human_response = interrupt({
        "type": "asset_review",
        "message": f"Review {self.domain} assets for dread level {state.current_dread_level}",
        "data": review_data,
        "actions": [
            "approve_all - Accept all generated assets",
            "approve_selected - Choose specific assets to approve",
            "request_changes - Provide feedback for regeneration",
            "reject_all - Restart generation process"
        ]
    })
    
    # Process human feedback
    return self.process_human_feedback(human_response, state)
```

### Rust Integration Patterns (ENHANCED)

#### Asset Consumption Pattern
```rust
#[derive(Resource)]
struct GeneratedAssets {
    // Organized by domain agent and dread level
    maps_assets: HashMap<(String, u8), GeneratedMapData>,      // (biome, dread_level)
    level_assets: HashMap<String, GeneratedLevelData>,         // encounter_id
    ui_assets: HashMap<(String, u8), Handle<Image>>,          // (component, dread_level)  
    dialogue_assets: HashMap<String, Handle<YarnProject>>,     // character_dialogue_tree
    audio_assets: HashMap<(String, u8), Handle<AudioSource>>, // (sound_type, dread_level)
}

impl GeneratedAssets {
    /// Load all AI-generated content at startup
    fn load_from_ai_output(asset_server: &AssetServer) -> Self {
        let mut assets = Self::default();
        
        // Load assets generated by each AI agent
        assets.load_maps_agent_output(asset_server);
        assets.load_levels_agent_output(asset_server);
        assets.load_ui_agent_output(asset_server);
        assets.load_dialogue_agent_output(asset_server);
        assets.load_audio_agent_output(asset_server);
        
        assets
    }
    
    /// Get asset variant for current game state
    fn get_dread_variant(&self, asset_type: &str, dread_level: u8) -> Option<Handle<Image>> {
        // Smart fallback hierarchy: dread-specific → base → error texture
        self.ui_assets.get(&(asset_type.to_string(), dread_level))
            .or_else(|| self.ui_assets.get(&(asset_type.to_string(), 0)))
            .cloned()
    }
}
```

#### Bevy-Specific Patterns (MAINTAINED)

##### System Registration (NO GENERATORS)
```rust
app.add_systems(Startup, (
        setup_generated_assets,     // Load AI-generated content
        setup_cc0_library,          // Load CC0 library assets  
        setup_core_assets,          // Load sacred core assets
        setup_world_from_agents,    // Apply AI agent configurations
    ))
    .add_systems(Update, (
        // Input and movement (unchanged)
        input_system,
        movement_system,
        collision_system,
        
        // AI-enhanced systems (NEW)
        apply_dread_variants,       // Switch AI-generated asset variants
        update_companion_trauma,    // Use AI-generated trauma states
        proximity_horror_audio,     // Use AI-generated spatial audio
        
        // Rendering (enhanced with AI assets)
        update_visuals,
    ).chain())
    .add_systems(FixedUpdate, physics_system);
```

##### Resource Management (AI-ENHANCED)
```rust
// Initialize with AI-generated content
app.init_resource::<GeneratedAssets>()
   .insert_resource(CC0LibraryAssets::load_from_disk())
   .insert_resource(CoreAssets::load_sacred_content())
   .insert_resource(DreadProgression::from_ai_agent_config());

// Access in systems with AI variants
fn horror_progression_system(
    dread: Res<DreadState>,
    generated: Res<GeneratedAssets>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Use AI-generated material variants for current dread level
    if let Some(variant) = generated.get_dread_variant("hex_tile_corruption", dread.level) {
        // Apply AI-generated corruption overlay
    }
}
```

## Integration Points (REVOLUTIONARY)

### Python → Rust Asset Pipeline
```python
# Python AI Agent Output Format
class RustAssetExport:
    def export_for_bevy(self, generated_assets: List[GeneratedAsset]) -> None:
        """Export AI-generated assets in Bevy-compatible format."""
        
        for asset in generated_assets:
            # Export 3D models as .glb files
            if asset.type == "model":
                self.export_gltf(asset, f"assets/generated/models/{asset.name}.glb")
            
            # Export textures as optimized .png files
            elif asset.type == "texture":
                self.export_texture(asset, f"assets/generated/textures/{asset.name}.png")
            
            # Export YarnSpinner dialogue as .yarn files
            elif asset.type == "dialogue":
                self.export_yarn(asset, f"assets/generated/dialogue/{asset.name}.yarn")
            
            # Export audio as compressed .ogg files  
            elif asset.type == "audio":
                self.export_audio(asset, f"assets/generated/audio/{asset.name}.ogg")
        
        # Generate Rust-compatible asset manifest
        self.generate_asset_manifest(generated_assets)
```

```rust
// Rust Asset Loading System
fn load_ai_generated_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Load asset manifest generated by Python AI agents
    let manifest = AssetManifest::load_from_file("assets/generated/manifest.json")
        .expect("AI-generated asset manifest not found");
    
    let mut generated_assets = GeneratedAssets::default();
    
    // Load each AI-generated asset
    for asset_entry in manifest.entries {
        match asset_entry.asset_type.as_str() {
            "hex_tile" => {
                let handle = asset_server.load(&asset_entry.path);
                generated_assets.hex_tiles.insert(
                    (asset_entry.terrain_type, asset_entry.dread_level),
                    handle
                );
            },
            "companion_trauma" => {
                let handle = asset_server.load(&asset_entry.path);
                generated_assets.companion_states.insert(
                    (asset_entry.companion_name, asset_entry.trauma_level),
                    handle
                );
            },
            // ... handle other asset types
        }
    }
    
    commands.insert_resource(generated_assets);
}
```

### Specialized Agent Integrations

#### MapsAgent → Hexx Integration
```rust
// Load AI-generated world data compatible with Hexx
fn load_generated_world_data(
    mut commands: Commands,
    generated_assets: Res<GeneratedAssets>,
) {
    if let Some(world_data) = generated_assets.maps_assets.get(&("overworld".to_string(), 0)) {
        // Apply AI-generated hex layout
        for (hex_pos, tile_data) in &world_data.hex_assignments {
            commands.spawn((
                HexPosition(*hex_pos),
                HexTerrain {
                    base_type: tile_data.terrain_type,
                    corruption_level: tile_data.corruption,
                },
                // Load AI-selected or AI-generated 3D model
                SceneBundle {
                    scene: tile_data.model_handle.clone(),
                    ..default()
                }
            ));
        }
    }
}
```

#### DialogueAgent → YarnSpinner Integration  
```rust
// Load AI-generated .yarn files via bevy_yarnspinner
fn setup_ai_dialogue_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    generated_assets: Res<GeneratedAssets>,
) {
    // Load AI-generated YarnSpinner projects
    for (character, yarn_handle) in &generated_assets.dialogue_assets {
        commands.spawn((
            YarnProject(yarn_handle.clone()),
            DialogueCharacter {
                name: character.clone(),
                trauma_variants: generated_assets.get_trauma_dialogue_variants(character),
            }
        ));
    }
}
```

## Performance Optimization (AI-ENHANCED)

### Smart Asset Streaming
```rust
fn ai_asset_streaming_system(
    dread: Res<DreadState>,
    mut generated_assets: ResMut<GeneratedAssets>,
    asset_server: Res<AssetServer>,
) {
    // Preload AI-generated assets for current + next dread level
    if dread.is_changed() {
        // Unload previous dread level to save memory
        generated_assets.unload_dread_level(dread.level.saturating_sub(1));
        
        // Load current dread level assets
        generated_assets.load_dread_level(dread.level, &asset_server);
        
        // Preload next dread level in background
        if dread.level < 4 {
            generated_assets.async_preload_dread_level(dread.level + 1, &asset_server);
        }
    }
}
```

### Database Query Optimization
```python
# Optimized asset search with caching
from functools import lru_cache

class DragonAssetDatabase:
    @lru_cache(maxsize=1000)
    def search_cached(self, query: str, category: str, dread_level: int) -> List[AssetMetadata]:
        """Cached semantic search to avoid repeated API calls."""
        return self._semantic_search(query, category, dread_level)
    
    def build_search_index(self) -> None:
        """Pre-compute embeddings for all CC0 library assets."""
        for asset in self.get_all_cc0_assets():
            if asset.embedding is None:
                asset.embedding = self.generate_embedding(asset.description)
                self.update_asset_embedding(asset)
```

## Deployment Configuration (ENHANCED)

### Python AI Environment
```yaml
# docker-compose.yml for AI generation services
version: '3.8'
services:
  dragon-ai:
    build: ./dragon-ai
    environment:
      - OPENAI_API_KEY=${OPENAI_API_KEY}
      - FREESOUND_API_KEY=${FREESOUND_API_KEY}
    volumes:
      - ./assets:/app/output
      - ./ai-cache:/app/cache
    ports:
      - "8080:8080"  # Human review interface
```

### Rust Game Release (MAINTAINED)
```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
strip = true

[profile.wasm-release] 
inherits = "release"
opt-level = "z"  # Size optimization for web
```

### CI/CD Pipeline (ENHANCED)
```yaml
# GitHub Actions with AI generation
name: Dragon's Labyrinth Build & Deploy
jobs:
  generate-assets:
    runs-on: ubuntu-latest
    steps:
    - name: Setup Python AI Environment
      run: |
        python -m venv dragon-ai-env
        source dragon-ai-env/bin/activate  
        pip install -r dragon-ai/requirements.txt
        
    - name: Generate AI Assets
      env:
        OPENAI_API_KEY: ${{ secrets.OPENAI_API_KEY }}
      run: |
        source dragon-ai-env/bin/activate
        python -m dragon-ai.main --generate-all --autonomous-mode
        
    - name: Upload Generated Assets
      uses: actions/upload-artifact@v3
      with:
        name: generated-assets
        path: assets/generated/

  build-game:
    needs: generate-assets
    runs-on: ubuntu-latest
    steps:
    - name: Download Generated Assets
      uses: actions/download-artifact@v3
      with:
        name: generated-assets
        path: assets/generated/
        
    - name: Build Rust Game
      run: |
        cargo build --release --target wasm32-unknown-unknown
        wasm-bindgen --out-dir dist --target web target/wasm32-unknown-unknown/release/*.wasm
```

## Known Technical Debt (UPDATED)

### Resolved Issues (AI Architecture)
- ✅ **Asset generation approach**: Agentic workflows with database-first selection
- ✅ **Content pipeline**: Python AI agents → Rust asset consumption
- ✅ **Quality consistency**: Human-in-the-loop review built into workflows

### Current Issues (Remaining)
- **Splatmap shader**: Not yet implemented for terrain blending
- **Performance optimization**: Asset streaming system needs refinement
- **Error handling**: Need robust fallbacks when AI generation fails
- **Cost monitoring**: Need usage tracking for OpenAI API costs

### Future Improvements
- **Multi-model support**: Add Claude, local LLMs as alternatives to OpenAI
- **Real-time generation**: Streaming asset generation during gameplay
- **Player feedback integration**: Use player behavior to improve AI generation
- **Advanced caching**: Predictive asset generation based on player patterns

### Performance Bottlenecks (UPDATED)
- **AI API latency**: Generation time impacts development workflow
- **Asset database queries**: Semantic search optimization needed
- **Memory usage**: Large asset variants consume significant memory
- **Network bandwidth**: Generated assets increase download size

## External Resources (ENHANCED)

### AI & ML Documentation
- [LangGraph Documentation](https://langchain-ai.github.io/langgraph/)
- [OpenAI API Reference](https://platform.openai.com/docs)
- [Huey Task Queue](https://huey.readthedocs.io/)
- [Blender Python API](https://docs.blender.org/api/current/)

### Game Development (MAINTAINED)
- [Bevy Book](https://bevyengine.org/learn/book/) 
- [Hexx Docs](https://docs.rs/hexx/)
- [bevy_yarnspinner](https://github.com/YarnSpinnerTool/YarnSpinner-Rust)
- [Cobweb UI](https://github.com/UkoeHB/bevy_cobweb_ui)

### Asset Creation
- [Freesound API](https://freesound.org/docs/api/)
- [CC0 Asset Libraries](https://creativecommons.org/share-your-work/public-domain/cc0/)
- [GLTF Specification](https://registry.khronos.org/glTF/)
- [Blender Asset Browser](https://docs.blender.org/manual/en/latest/editors/asset_browser.html)

### Community & Support
- [Bevy Discord](https://discord.gg/bevy)
- [LangChain Community](https://discord.gg/langchain)
- [Rust GameDev](https://gamedev.rs/)
- [OpenAI Developer Community](https://community.openai.com/)

This revolutionary technical architecture enables Dragon's Labyrinth to intelligently leverage existing CC0 assets while generating precisely targeted horror content, achieving both cost efficiency and high quality through the power of specialized AI agents working in harmony with a high-performance Rust game engine.
