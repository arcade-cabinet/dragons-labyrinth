# Dragon's Labyrinth - Balanced Architecture Plan
## Core Code vs Prompt Generation + MCP Server Integration

## Executive Summary

The user identified two critical imbalances in the current approach:
1. **Only balancing CC0 vs Generated assets** - Missing the crucial balance of **Core Code vs Prompt Generation**  
2. **Agents only doing prompt generation** - Limited value when they should own complex domains directly

This plan rebalances the architecture for **quality over quantity** with **MCP server integration** providing runtime asset querying and ECS interconnect overlay.

## Current Imbalance Analysis

### What We've Balanced ✅
- **CC0 Assets (80%) vs AI Generated Assets (20%)**
- **Build-time Python vs Runtime Rust separation**  
- **Asset generation pipeline and human-in-the-loop**

### What We've MISSED ❌
- **Core Game Code vs Prompt-Driven Code Generation balance**
- **Agent direct code ownership of complex domains**
- **Runtime MCP integration for asset querying and ECS overlay**
- **Quality over quantity in agent responsibilities**

## MCP Server Integration Strategy

### Option A: MCP Server in game-assets ⭐⭐⭐
**Purpose**: Runtime querying of CC0 assets by category for intelligent selection

```rust
// crates/game-assets/src/mcp_server.rs
pub struct AssetsQueryServer {
    asset_database: AssetDatabase,
    semantic_search: SemanticSearch,
    cc0_catalog: CC0Catalog,
}

impl McpServer for AssetsQueryServer {
    fn tools(&self) -> Vec<Tool> {
        vec![
            Tool::new("query_assets_by_category")
                .description("Query CC0 assets by category, theme, and requirements")
                .parameter("category", "string", "Asset category: models, textures, audio")
                .parameter("theme", "string", "Theme: medieval, horror, nature, etc.")
                .parameter("requirements", "object", "Specific requirements object"),
                
            Tool::new("search_compatible_assets")
                .description("Semantic search for assets compatible with game requirements")
                .parameter("description", "string", "Natural language asset description")
                .parameter("dread_level", "integer", "Horror progression level 0-5"),
                
            Tool::new("get_asset_metadata")
                .description("Get detailed metadata for specific asset")
                .parameter("asset_id", "string", "Unique asset identifier"),
        ]
    }
    
    async fn call_tool(&self, name: &str, args: serde_json::Value) -> McpResult<serde_json::Value> {
        match name {
            "query_assets_by_category" => {
                let category: String = args["category"].as_str().unwrap().to_string();
                let theme: String = args["theme"].as_str().unwrap().to_string();
                let assets = self.asset_database.query_by_category_theme(&category, &theme).await?;
                Ok(serde_json::to_value(assets)?)
            }
            "search_compatible_assets" => {
                let description: String = args["description"].as_str().unwrap().to_string();
                let dread_level: u8 = args["dread_level"].as_u64().unwrap() as u8;
                let assets = self.semantic_search.find_compatible(&description, dread_level).await?;
                Ok(serde_json::to_value(assets)?)
            }
            // ... other tools
        }
    }
}
```

### Option B: MCP Server in game-database ⭐⭐⭐⭐⭐ (PREFERRED)
**Purpose**: Runtime ECS overlay providing complete interconnect of all systems

```rust
// crates/game-database/src/mcp_server.rs
pub struct GameDatabaseMcpServer {
    bevy_world: Arc<World>,
    database: Arc<GameDatabase>, 
    ecs_bridge: EcsDatabaseBridge,
}

impl McpServer for GameDatabaseMcpServer {
    fn tools(&self) -> Vec<Tool> {
        vec![
            Tool::new("query_companion_state")
                .description("Get complete companion psychological and relationship state")
                .parameter("companion_id", "string", "Companion identifier")
                .parameter("include_trauma_history", "boolean", "Include trauma event history"),
                
            Tool::new("query_philosophical_progression")  
                .description("Get player's 4-path philosophical development")
                .parameter("player_id", "string", "Player identifier")
                .parameter("include_transition_history", "boolean", "Include all transition choices"),
                
            Tool::new("query_forge_readiness")
                .description("Assess forge trial readiness across all systems")
                .parameter("player_id", "string", "Player identifier")
                .returns("Complete forge assessment with companion sacrifice readiness"),
                
            Tool::new("query_sentimental_items")
                .description("Get sentimental item collection with memory integration")
                .parameter("player_id", "string", "Player identifier")
                .parameter("include_memories", "boolean", "Include accumulated memories"),
                
            Tool::new("query_world_corruption")
                .description("Get environmental decay state by location")
                .parameter("hex_coordinates", "array", "Array of hex positions")
                .parameter("include_npc_behavior", "boolean", "Include NPC fear levels"),
                
            Tool::new("execute_complex_query")
                .description("Execute complex cross-system queries")
                .parameter("query_type", "string", "Type: trauma_impact, philosophy_conflicts, forge_scenarios")
                .parameter("parameters", "object", "Query-specific parameters"),
        ]
    }
}

// ECS Integration Bridge
pub struct EcsDatabaseBridge {
    world: Arc<World>,
    database: Arc<GameDatabase>,
}

impl EcsDatabaseBridge {
    pub async fn sync_companion_trauma(&self, companion_id: &str) -> DatabaseResult<TraumaState> {
        // Get ECS component data
        let world = self.world.read();
        let trauma_query = world.query::<(&CompanionId, &TraumaLevel, &BondStrength)>();
        
        // Sync with database
        let database_trauma = self.database.get_companion_trauma(companion_id).await?;
        
        // Return combined state
        Ok(TraumaState {
            ecs_component: /* ECS data */,
            database_history: database_trauma,
            therapy_progress: self.database.get_therapy_progress(companion_id).await?,
            relationship_matrix: self.database.get_relationship_matrix(companion_id).await?,
        })
    }
    
    pub async fn assess_forge_readiness(&self, player_id: &str) -> DatabaseResult<ForgeAssessment> {
        // Cross-system analysis
        let companions = self.get_all_companion_states(player_id).await?;
        let sentimental_items = self.get_sentimental_collection(player_id).await?;
        let philosophical_state = self.get_philosophical_progression(player_id).await?;
        let trauma_levels = self.calculate_party_trauma_impact().await?;
        
        Ok(ForgeAssessment {
            light_path_viability: self.assess_light_path(&companions, &philosophical_state),
            dark_path_viability: self.assess_dark_path(&companions, &trauma_levels),
            sentimental_power_level: self.calculate_reagent_power(&sentimental_items),
            companion_sacrifice_willingness: self.assess_sacrifice_readiness(&companions),
            trial_difficulty_scaling: self.calculate_trial_scaling(&philosophical_state),
        })
    }
}
```

## Rebalanced Agent Architecture: Core Code Ownership

### Current Problem: Agents Only Generate Prompts ❌
**Limited Value**: Agents produce prompts → Human reviews → Someone else implements
**Missing**: Direct ownership of complex, esoteric domains requiring deep expertise

### New Balanced Approach: Core Code Domains + Selective Prompts ✅

#### Tier 1: DIRECT CODE OWNERSHIP (Quality Focus)
**These domains are too complex/critical for prompt generation - agents own the code**

**SS-006: ForgeAgent** → **OWNS FORGE TRIALS SYSTEM COMPLETELY**
- **Direct Rust Implementation**: Complete forge trial orchestration system
- **Bpy Script Mastery**: Complex 3D trial environments (lava fields, crystalline mazes)
- **ECS Integration**: MythicGear, ForgeAccess, CompanionSacrifice components
- **Database Integration**: ForgeProgress, SentimentalItem memory integration
- **Asset Coordination**: Manages mythic gear visuals, trial environment assets

**SS-008: TraumaAgent** → **OWNS PSYCHOLOGICAL SYSTEMS COMPLETELY** 
- **Direct Rust Implementation**: Trauma accumulation algorithms, therapy systems
- **Advanced Dialogue Generation**: Trauma-aware conversation trees with psychological authenticity
- **ECS Integration**: TraumaLevel, TherapyState, BondStrength components  
- **Database Integration**: CompanionTrauma, TherapyProgress, emotional state tracking
- **Memory Integration**: Trauma triggers stored in companion memory systems

**AF-005: AudioAgent** → **OWNS MUSIC21 + SPATIAL AUDIO COMPLETELY**
- **Direct Rust Implementation**: Music21 bindings, algorithmic composition engine
- **Complex Bpy Integration**: Advanced audio generation scripts
- **Bevy Audio Integration**: Spatial audio, proximity horror, dynamic mixing
- **Database Integration**: Audio state tracking, dread-responsive composition
- **Asset Coordination**: OGG generation pipeline, Freesound integration

#### Tier 2: HYBRID OWNERSHIP (Balanced Approach)
**Complex systems with both direct code and guided prompts**

**AF-001: MapsAgent** → **CORE HEX ALGORITHMS + GUIDED BIOME GENERATION**
- **Direct Code**: Hex pathfinding, world generation algorithms, performance optimization
- **Guided Prompts**: Biome variety, environmental storytelling, corruption progression
- **Code Ownership**: Hexx integration, coordinate systems, tile streaming
- **Prompt Generation**: Asset variety, narrative placement, environmental details

**AF-004: DialogueAgent** → **CORE YARNSPINNER + GUIDED CONVERSATION TREES**
- **Direct Code**: YarnSpinner integration, dialogue state management, branching logic
- **Guided Prompts**: Character voice, conversation variety, narrative branches
- **Code Ownership**: Dialogue system architecture, companion relationship tracking
- **Prompt Generation**: Trauma-aware responses, philosophical dialogue variations

#### Tier 3: SELECTIVE PROMPT GENERATION (Quantity Focus)
**Systems where prompts provide value but don't require deep code ownership**

**AF-003: UIAgent** → **GUIDED UI GENERATION WITH CORE FRAMEWORK**
- **Prompt Focus**: UI variation generation, horror degradation visuals
- **Core Framework**: Cobweb integration basics, state management patterns
- **Balance**: Core UI architecture owned, variety generated

**SS-010: DecayAgent** → **GUIDED CORRUPTION GENERATION** 
- **Prompt Focus**: Environmental corruption variants, NPC behavior changes
- **Core Framework**: Corruption calculation algorithms, state tracking
- **Balance**: Mathematics owned, visual variety generated

## MCP Server Value Propositions

### game-assets MCP Server Value
- **Real-time Asset Queries**: Agents can query CC0 catalog during generation
- **Intelligent Asset Selection**: Semantic search for compatible existing assets
- **Build-time Efficiency**: Reduce unnecessary AI generation through better discovery

### game-database MCP Server Value ⭐⭐⭐⭐⭐
- **Complete ECS Overlay**: Runtime access to ALL game state interconnections
- **Cross-System Intelligence**: Queries spanning multiple sophisticated systems  
- **Forge Assessment**: Real-time evaluation of player readiness for trials
- **Companion Analysis**: Deep psychological state analysis for trauma/therapy
- **Philosophical Tracking**: 4-path progression with transition impact analysis
- **Memory Integration**: Sentimental item history with emotional weight calculation

### Combined MCP Architecture
```
Runtime Game (Bevy ECS)
    ↓
game-database MCP Server (Complete overlay)
    ↓ (Asset queries)
game-assets MCP Server (CC0 catalog)
    ↓ (Build coordination)  
AI Agents with Direct Code Ownership
    ↓ (Generated content)
Build Pipeline (Sophisticated systems)
```

## Implementation Priority: Quality Over Quantity

### Phase 1: Core Code Architecture (Weeks 1-2)
**FOCUS**: Implement direct code ownership domains

1. **ForgeAgent**: Complete forge trial system implementation
   - Direct Rust code for trial orchestration
   - Complex Blender scripts for trial environments  
   - MythicGear ECS integration
   - Sentimental item reagent system

2. **TraumaAgent**: Complete psychological systems implementation
   - Direct Rust trauma accumulation algorithms
   - Therapy quest dialogue generation
   - ECS trauma component integration
   - Database trauma history tracking

3. **AudioAgent**: Complete Music21 + spatial audio implementation
   - Direct Rust Music21 bindings
   - Algorithmic composition engine
   - Bevy spatial audio integration
   - Proximity horror system

### Phase 2: MCP Server Integration (Weeks 3-4)
**FOCUS**: Runtime querying and ECS overlay systems

1. **game-database MCP Server**: Complete ECS interconnect overlay
   - All sophisticated system queries
   - Cross-system analysis tools
   - Forge readiness assessment
   - Companion psychological analysis

2. **game-assets MCP Server**: CC0 catalog runtime querying
   - Semantic asset search
   - Category-based queries
   - Asset compatibility analysis
   - Build-time coordination

### Phase 3: Hybrid Systems Integration (Weeks 5-6)
**FOCUS**: Balance code ownership with selective prompts

1. **MapsAgent**: Core algorithms + guided generation
2. **DialogueAgent**: YarnSpinner core + conversation variety
3. **Selective prompt systems**: UI, environmental decay

### Phase 4: Quality Integration (Weeks 7-8)
**FOCUS**: Ensure direct code ownership provides superior results

1. **Performance Validation**: Core code performance vs generated code
2. **Quality Assessment**: Direct ownership complexity vs prompt output
3. **Integration Testing**: MCP servers enhancing agent capabilities
4. **System Sophistication**: Revolutionary features working seamlessly

## Success Metrics: Quality Over Quantity

### Code Ownership Success
- [ ] **Forge Trials**: Most complex system implemented directly, not generated
- [ ] **Trauma Psychology**: Authentic psychological systems owned, not prompted
- [ ] **Audio Composition**: Music21 integration owned, providing unique audio
- [ ] **Performance**: Direct code ownership provides better performance than prompts
- [ ] **Maintainability**: Owned code is more maintainable than generated alternatives

### MCP Integration Success  
- [ ] **Real-time Queries**: Agents can query complete game state during operation
- [ ] **Cross-System Analysis**: Complex queries spanning multiple sophisticated systems
- [ ] **Asset Intelligence**: Smart CC0 selection reducing unnecessary generation
- [ ] **ECS Overlay**: Complete runtime visibility into all game interconnections
- [ ] **Development Velocity**: MCP integration accelerates development vs pure prompts

### Revolutionary Game Success
- [ ] **Market Innovation**: Features impossible without direct code ownership
- [ ] **Technical Excellence**: Performance and sophistication beyond prompt generation
- [ ] **Narrative Depth**: Psychological authenticity requiring algorithmic complexity
- [ ] **Audio Innovation**: Musical composition creating unique horror atmosphere
- [ ] **Player Experience**: Seamless systems that feel organic, not generated

## Conclusion

This rebalanced architecture addresses the critical gaps:

1. **Quality over Quantity**: Direct code ownership of complex domains vs broad prompt generation
2. **Core Game Code**: Like core assets, certain systems need direct implementation
3. **MCP Integration**: Runtime querying and ECS overlay providing unprecedented capabilities
4. **Agent Value**: Transformed from prompt generators to domain experts with code ownership

**The game will succeed because the most complex and critical systems are owned and implemented by specialized agents, while MCP servers provide runtime intelligence that prompt generation alone cannot achieve.**

**This creates a hybrid architecture where prompts generate variety and agents own complexity - the best of both approaches.**
