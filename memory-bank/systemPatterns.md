# Dragon's Labyrinth - System Patterns

## REVOLUTIONARY ARCHITECTURE (January 2025)

### The Clean Separation Principle
**CRITICAL**: Complete separation of build-time AI generation from runtime game systems

```
BUILD-TIME (Python Agentic Workflows)          RUNTIME (Rust Game Engine)
├── LangGraph AI Agents                    ←→  ├── Pure Asset Loading
├── SQLite Job Queues                     ←→  ├── Bevy ECS Systems  
├── Database Asset Search                 ←→  ├── Component Processing
├── Human-in-the-Loop Review              ←→  ├── Event Systems
└── Generated Asset Output                ←→  └── Performance Optimization
```

### Agentic Workflow Patterns

#### 1. Domain-Specific AI Agents
Each agent specializes in one game system domain:

```python
class DragonAgent(BaseAgent):
    def __init__(self, domain: str, search_tool: DragonAssetSearchTool):
        self.domain = domain  # "maps", "levels", "ui", "dialogue", "audio"
        self.search_tool = search_tool  # Database-first asset selection
        self.dread_awareness = True  # All agents understand horror progression
        
    def generate_content(self, dread_level: int, requirements: dict):
        # 1. SEARCH existing CC0 library assets first
        existing = self.search_tool.search_dragon_assets(
            query=requirements["description"],
            dread_level=dread_level,
            category=self.domain
        )
        
        # 2. EVALUATE if existing assets meet needs
        if suitable := self.evaluate_existing(existing, requirements):
            return self.enhance_existing(suitable, dread_level)
            
        # 3. GENERATE only when nothing suitable exists
        return self.ai_generate_new(requirements, dread_level)
```

#### 2. LangGraph Workflow Pattern
Durable execution with checkpointing and human review:

```python
def build_agent_workflow(self) -> StateGraph:
    workflow = StateGraph(AgentState)
    
    # Add nodes for each phase
    workflow.add_node("search_existing", self.search_assets_node)
    workflow.add_node("evaluate_assets", self.evaluate_suitability_node)  
    workflow.add_node("human_review", self.human_review_node)
    workflow.add_node("generate_missing", self.ai_generate_node)
    workflow.add_node("finalize_output", self.finalize_assets_node)
    
    # Conditional edges with intelligent routing
    workflow.add_conditional_edges(
        "evaluate_assets",
        self.should_generate_or_reuse,
        {
            "reuse": "human_review",
            "generate": "generate_missing",
            "hybrid": "generate_missing"  # Enhance existing with AI
        }
    )
    
    # Compile with SQLite checkpointing for durability
    return workflow.compile(checkpointer=self.sqlite_checkpointer)
```

#### 3. Three-Tier Asset Selection Pattern
Smart asset sourcing with maximum reuse:

```python
class AssetSelectionPattern:
    def select_asset(self, requirements: AssetRequirement) -> AssetResult:
        # Tier 1: Core Assets (SACRED - never modify)
        if core_asset := self.check_core_assets(requirements):
            return AssetResult(source="core", asset=core_asset, modifications=[])
        
        # Tier 2: CC0 Library (SEARCH and potentially enhance)  
        if library_matches := self.search_cc0_library(requirements):
            best_match = self.rank_by_suitability(library_matches)[0]
            
            if self.suitability_score(best_match) > 0.8:
                # Use as-is
                return AssetResult(source="library", asset=best_match, modifications=[])
            elif self.suitability_score(best_match) > 0.5:
                # Enhance with AI (corruption overlays, dread variants)
                enhancements = self.plan_ai_enhancements(best_match, requirements)
                return AssetResult(source="hybrid", asset=best_match, modifications=enhancements)
        
        # Tier 3: AI Generation (only when nothing suitable exists)
        generated = self.ai_generate_asset(requirements)
        return AssetResult(source="generated", asset=generated, modifications=[])
```

## Core Rust Runtime Architecture (GENERATOR-FREE)

### Entity Component System (ECS)
The game uses Bevy's ECS architecture where:
- **Entities**: Unique IDs representing game objects
- **Components**: Data attached to entities (position, health, etc.)
- **Systems**: Functions that process components (NO GENERATION CODE)
- **Resources**: Global singleton data (DreadState, HexWorld, GeneratedAssets)

### System Organization (CLEAN)

```
crates/game/src/
├── components/     # Pure data structures
├── systems/        # Logic processors (NO AI generation)
├── resources/      # Global state management
├── assets/         # Asset loading systems (consumes generated content)
├── dialogue/       # YarnSpinner integration
├── board/          # Board rendering (uses generated tiles)
└── hex_board/      # Hexx integration
```

**DELETED**: `generators/` directory - completely removed from Rust codebase

## Key Design Patterns (Updated)

### 1. Dread-Driven Transformation (ENHANCED)
Every system responds to the global dread level, consuming AI-generated variants:

```rust
fn horror_progression_system(
    dread: Res<DreadState>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    generated_assets: Res<GeneratedAssets>,  // AI-generated variants
    mut query: Query<(&mut Handle<StandardMaterial>, &HexTerrain)>
) {
    for (mut material_handle, terrain) in query.iter_mut() {
        // Load AI-generated material variant for current dread level
        if let Some(variant) = generated_assets.get_material_variant(
            terrain.base_type,
            dread.level
        ) {
            *material_handle = variant.clone();
        }
    }
}
```

**Pattern Rules (ENHANCED):**
- Systems MUST check dread level and consume appropriate AI-generated variants
- Behavior changes use AI-generated assets discrete per level (0-4)
- Transitions use AI-generated corruption overlays and audio
- Later stages override with completely AI-generated nightmare assets

### 2. Component Composition Over Inheritance (MAINTAINED)
Instead of complex hierarchies, use component combinations:

```rust
// DON'T: Giant monolithic components
struct NPC {
    health: f32,
    dialogue: DialogueTree,
    ai: AIBehavior,
    inventory: Vec<Item>,
    // ... 50 more fields
}

// DO: Composable components consuming AI-generated content
commands.spawn((
    Health(100.0),
    DialogueCapable { 
        tree_id: "villager",  // References AI-generated YarnSpinner file
        trauma_variants: generated_assets.get_trauma_dialogue("villager")
    },
    FleeAtDread(2),  // Flees at dread level 2+
    VisualProgression {
        base_model: library_assets.get_model("villager_base"),
        trauma_overlays: generated_assets.get_trauma_overlays("villager") 
    },
    Inventory::default(),
));
```

### 3. Event-Driven Narrative (AI-ENHANCED)
Narrative progression through events, triggering AI-generated content:

```rust
#[derive(Event)]
struct NarrativeEvent {
    event_type: NarrativeEventType,
    actor: Entity,
    context: NarrativeContext,
    triggers_ai_content: bool,  // NEW: Flag for AI-generated responses
}

fn narrative_system(
    mut events: EventReader<NarrativeEvent>,
    mut dread: ResMut<DreadState>,
    generated_assets: Res<GeneratedAssets>,  // AI-generated narrative content
) {
    for event in events.read() {
        match event.event_type {
            NarrativeEventType::BossDefeated => {
                dread.advance();
                // Load AI-generated post-boss dialogue and world changes
                if let Some(aftermath) = generated_assets.get_boss_aftermath(event.context.boss_id) {
                    // Apply AI-generated environmental changes
                }
            },
            NarrativeEventType::CompanionTrauma => {
                // Load AI-generated trauma response dialogue and visuals
                if let Some(trauma_response) = generated_assets.get_trauma_response(
                    event.actor, 
                    event.context.trauma_level
                ) {
                    // Apply AI-generated companion degradation
                }
            }
        }
    }
}
```

### 4. Asset Consumption Pattern (NEW)
Pure consumption of AI-generated assets with smart caching:

```rust
#[derive(Resource)]
struct GeneratedAssets {
    // Organized by dread level and asset type
    hex_tiles: HashMap<(TerrainType, u8), Handle<Scene>>,  // (terrain, dread_level)
    companion_states: HashMap<(String, f32), Handle<Scene>>,  // (companion, trauma_level)
    ui_variants: HashMap<(String, u8), Handle<Image>>,  // (ui_element, dread_level)
    dialogue_trees: HashMap<(String, u8), YarnSpinnerAsset>,  // (character, dread_level)
    audio_variants: HashMap<(String, u8), Handle<AudioSource>>,  // (sound, dread_level)
}

impl GeneratedAssets {
    /// Load asset variant for current game state
    fn get_dread_variant<T>(&self, base_asset: &str, dread_level: u8) -> Option<&Handle<T>> {
        // Smart fallback: if dread-specific variant doesn't exist, use base
        self.get_variant(base_asset, dread_level)
            .or_else(|| self.get_variant(base_asset, 0))  // Fallback to peace variant
    }
    
    /// Preload all variants for smooth transitions
    fn preload_dread_progression(&mut self, asset_server: &AssetServer) {
        // Load all 5 dread variants for critical assets to prevent hitches
        for dread_level in 0..=4 {
            self.preload_level_assets(dread_level, asset_server);
        }
    }
}
```

### 5. Staged System Execution (MAINTAINED)
Systems run in specific order for consistency:

```rust
app.add_systems(Update, (
    // Input first
    handle_input,
    // Then movement
    update_positions,
    // Then game logic
    check_collisions,
    update_dread_progression,
    // Then AI-generated content application
    apply_dread_variants,  // NEW: Apply AI-generated asset variants
    update_companion_trauma_visuals,  // NEW: AI-generated trauma states
    // Then reactions
    apply_world_corruption,  // Uses AI-generated corruption overlays
    update_companion_dialogue,  // Uses AI-generated dialogue variants
    // Finally rendering
    update_visuals,
).chain());
```

## Specialized Agent Integration Patterns

### MapsAgent → Rust Integration
```rust
// MapsAgent outputs Hexx-compatible world data
#[derive(Resource)]
struct GeneratedWorldData {
    hex_layout: HexLayout,
    tile_assignments: HashMap<Hex, TileAssignment>,
    corruption_progression: Vec<CorruptionWave>,  // AI-planned corruption spread
    biome_transitions: Vec<BiomeTransition>,  // AI-generated biome changes
}

fn load_generated_world(
    mut commands: Commands,
    world_data: Res<GeneratedWorldData>,
    generated_assets: Res<GeneratedAssets>,
) {
    for (hex_pos, tile_assignment) in &world_data.tile_assignments {
        let tile_entity = commands.spawn((
            HexPosition(*hex_pos),
            HexTerrain {
                base_type: tile_assignment.base_terrain,
                corruption_level: tile_assignment.corruption,
            },
            // Load AI-generated 3D model for this tile
            SceneBundle {
                scene: generated_assets.get_tile_model(
                    tile_assignment.base_terrain,
                    tile_assignment.corruption
                ).clone(),
                transform: Transform::from_translation(
                    world_data.hex_layout.hex_to_world_pos(*hex_pos)
                ),
                ..default()
            }
        )).id();
    }
}
```

### LevelsAgent → Yoleck Integration
```rust
// LevelsAgent outputs Yoleck-compatible level files
#[derive(Resource)]
struct GeneratedLevelData {
    encounter_placements: Vec<EncounterPlacement>,
    interactive_objects: Vec<ObjectPlacement>,
    narrative_triggers: Vec<NarrativeTrigger>,
    dread_progression_points: Vec<DreadTrigger>,
}

fn load_generated_encounters(
    mut commands: Commands,
    level_data: Res<GeneratedLevelData>,
    generated_assets: Res<GeneratedAssets>,
) {
    for encounter in &level_data.encounter_placements {
        commands.spawn((
            HexPosition(encounter.position),
            EncounterTrigger {
                encounter_id: encounter.encounter_id.clone(),
                dread_requirement: encounter.min_dread_level,
            },
            // Load AI-generated encounter assets (enemies, props, etc.)
            encounter.load_assets(&generated_assets),
        ));
    }
}
```

### DialogueAgent → YarnSpinner Integration
```rust
// DialogueAgent outputs .yarn files loaded by bevy_yarnspinner
#[derive(Resource)]
struct GeneratedDialogueAssets {
    yarn_projects: HashMap<String, Handle<YarnProject>>,
    companion_arcs: HashMap<String, CompanionArcData>,
    moral_choice_trees: HashMap<String, MoralChoiceData>,
}

fn update_companion_dialogue(
    mut dialogue_runner: ResMut<DialogueRunner>,
    companions: Query<(&Companion, &DialogueCapable), Changed<Companion>>,
    generated_dialogue: Res<GeneratedDialogueAssets>,
    dread: Res<DreadState>,
) {
    for (companion, dialogue_cap) in companions.iter() {
        // Select AI-generated dialogue variant based on trauma + dread
        let dialogue_variant = format!(
            "{}_{}_trauma{:.1}_dread{}", 
            dialogue_cap.base_character,
            companion.current_state,
            companion.trauma,
            dread.level
        );
        
        if let Some(yarn_project) = generated_dialogue.yarn_projects.get(&dialogue_variant) {
            dialogue_runner.start_dialogue_from_node(yarn_project, &dialogue_variant);
        }
    }
}
```

### AudioAgent → Bevy Audio Integration
```rust
// AudioAgent outputs spatial audio configuration
#[derive(Resource)]
struct GeneratedAudioAssets {
    proximity_configs: HashMap<String, ProximityAudioConfig>,
    dread_soundscapes: HashMap<u8, Handle<AudioSource>>,
    companion_voices: HashMap<(String, f32), Handle<AudioSource>>,  // (name, trauma_level)
}

fn proximity_horror_system(
    player: Query<&HexPosition, With<Player>>,
    dragon: Query<&HexPosition, With<Dragon>>,
    mut audio: ResMut<Audio>,
    generated_audio: Res<GeneratedAudioAssets>,
    dread: Res<DreadState>,
) {
    if let (Ok(player_pos), Ok(dragon_pos)) = (player.get_single(), dragon.get_single()) {
        let distance = hex_distance(player_pos.0, dragon_pos.0);
        let intensity = 1.0 / (distance as f32).max(1.0);
        
        // Use AI-generated proximity audio for current dread level
        if let Some(dragon_audio) = generated_audio.get_dragon_proximity_audio(dread.level, intensity) {
            audio.play_spatial(dragon_audio.clone(), Transform::from_translation(
                hex_layout.hex_to_world_pos(dragon_pos.0)
            ));
        }
    }
}
```

## Performance Patterns (ENHANCED)

### 1. Asset Streaming Pattern
Smart loading of AI-generated content:

```rust
fn asset_streaming_system(
    player_pos: Query<&HexPosition, With<Player>>,
    dread: Res<DreadState>,
    mut generated_assets: ResMut<GeneratedAssets>,
    asset_server: Res<AssetServer>,
) {
    // Preload assets for current and next dread level
    if dread.is_changed() {
        // Unload old dread level assets to save memory
        generated_assets.unload_dread_level(dread.level.saturating_sub(1));
        
        // Load current dread level assets
        generated_assets.load_dread_level(dread.level, &asset_server);
        
        // Preload next dread level in background
        if dread.level < 4 {
            generated_assets.preload_dread_level(dread.level + 1, &asset_server);
        }
    }
}
```

### 2. Dread-Aware LOD System
Quality adjusts based on horror progression:

```rust
fn horror_lod_system(
    camera: Query<&Transform, With<Camera>>,
    dread: Res<DreadState>,
    mut renderables: Query<(&HexPosition, &mut Visibility, &mut Handle<Scene>)>,
    generated_assets: Res<GeneratedAssets>,
) {
    let camera_pos = camera.single();
    
    for (hex_pos, mut visibility, mut scene_handle) in renderables.iter_mut() {
        let distance_from_camera = camera_pos.translation.distance(hex_pos.world_position());
        
        // In horror stages, reduce LOD distance to create claustrophobia
        let lod_distance = match dread.level {
            0..=1 => 50.0,  // Normal viewing distance
            2..=3 => 30.0,  // Reduced visibility
            4 => 15.0,      // Claustrophobic horror mode
        };
        
        if distance_from_camera > lod_distance {
            *visibility = Visibility::Hidden;
        } else {
            *visibility = Visibility::Inherited;
            // Load appropriate LOD model from AI-generated assets
            if let Some(lod_model) = generated_assets.get_lod_variant(
                scene_handle.id(), 
                distance_from_camera,
                dread.level
            ) {
                *scene_handle = lod_model.clone();
            }
        }
    }
}
```

## Critical Implementation Paths (UPDATED)

### 1. Asset Generation → Consumption Pipeline
```
Python Agent Analysis → CC0 Library Search → AI Enhancement/Generation → 
SQLite Storage → Human Review → Asset Export → 
Rust Asset Loading → Bevy Scene Spawning → Runtime Consumption
```

### 2. Dread Progression Path (AI-ENHANCED)
```
Player Action → Narrative Event → Dread Check → 
Level Advance → AI Asset Variant Loading → 
System Updates → World Transform → Horror Escalation
```

### 3. Companion Arc Path (AI-DRIVEN)
```
Narrative Progress → Trauma Accumulation → AI-Generated State Check →
Dialogue Variant Loading → Visual Progression Update → 
Companion Behavior Change → Potential Departure/Betrayal
```

### 4. Horror Audio Proximity Path (AI-ENHANCED)
```
Position Update → Distance Calculation → Dread Level Check →
AI-Generated Audio Variant Selection → Spatial Audio Update →
Sanity Effect Application → Hallucination Spawn
```

## Database Integration Pattern (CRITICAL)

### Asset Database as StructuredTool
```python
# Used by all AI agents for asset selection
class DragonAssetSearchTool(StructuredTool):
    def search_dragon_assets(
        query: str,           # "corrupted medieval stone texture"
        dread_level: int,     # 0-4 horror progression context
        category: str,        # "textures", "models", "audio"
        limit: int = 8
    ) -> List[AssetMetadata]:
        # Semantic search through indexed CC0 library
        # Returns ranked results with suitability scores
        # Considers horror progression requirements
        # Filters by performance requirements (mobile-friendly)
```

### Smart Asset Selection Logic
```python
def select_optimal_assets(self, requirements: AssetRequirement) -> AssetSelection:
    """
    80/20 Rule Implementation:
    - 80% reuse existing CC0 assets with intelligent enhancement
    - 20% generate new content for horror-specific needs
    """
    # Search existing library first
    candidates = self.search_tool.search_dragon_assets(
        query=requirements.description,
        dread_level=requirements.dread_level,
        category=requirements.category
    )
    
    # Evaluate each candidate
    for candidate in candidates:
        suitability = self.evaluate_suitability(candidate, requirements)
        
        if suitability > 0.9:
            # Perfect match - use as-is
            return AssetSelection(source="library", asset=candidate, modifications=[])
        elif suitability > 0.7:
            # Good match - enhance with AI
            enhancements = self.plan_horror_enhancements(candidate, requirements)
            return AssetSelection(source="hybrid", asset=candidate, modifications=enhancements)
    
    # No suitable existing asset - generate new
    return AssetSelection(source="generated", asset=None, 
                         generate_spec=requirements)
```

These patterns ensure Dragon's Labyrinth achieves its revolutionary architecture: clean separation between AI generation and runtime systems, intelligent reuse of existing assets, and a horror-driven progression that transforms every aspect of the game through specialized AI agents.
