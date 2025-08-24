# Dragon's Labyrinth - Assets Inspector Architecture

## THREE-LAYER REVOLUTIONARY ARCHITECTURE

### Layer 1: Python AI Generation (Build-Time)
- LangGraph agentic workflows with specialized domain agents
- Database-first asset selection from CC0 library
- Human-in-the-loop review via FastAPI web interface
- SQLite job queues with Huey task management

### Layer 2: Rust Assets Inspector (Validation Bridge) **NEW**
- **Purpose**: Visual validation and ECS mapping between AI generation and game runtime
- **Technology**: PyO3 + bevy-inspector-egui + bevy_ecs_sqlx
- **Critical Functions**: Dogfooding, ECS mapping, human visual inspection

### Layer 3: Rust Game Runtime (Pure Consumption)
- Pure Bevy ECS game systems consuming validated assets
- Zero AI generation code, zero database mapping logic
- Optimized for 60 FPS performance with pre-validated content

## Assets Inspector Crate Design

### Core Purpose: Visual Validation Bridge
```rust
// crates/assets-inspector/src/lib.rs
use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;
use pyo3::prelude::*;
use bevy_ecs_sqlx::prelude::*;

#[derive(Resource, Reflect, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
struct GeneratedAssetsDatabase {
    /// Direct connection to Python AI generation SQLite database
    connection: SqliteConnection,
    /// Asset metadata indexed by dread level and category
    asset_index: HashMap<(u8, String), Vec<AssetMetadata>>,
    /// Validation status for each generated asset
    validation_status: HashMap<String, ValidationStatus>,
}

#[derive(Reflect, InspectorOptions)]
struct AssetMetadata {
    id: String,
    source: AssetSource, // Core, Library, Generated
    dread_level: u8,
    category: String, // "hex_tiles", "companion_trauma", "horror_ui"
    generation_agent: String, // "MapsAgent", "DialogueAgent", etc.
    file_path: PathBuf,
    bevy_handle: Option<Handle<Scene>>, // Loaded Bevy asset handle
    validation_status: ValidationStatus,
    human_approved: bool,
}

#[derive(Reflect, InspectorOptions)]
enum ValidationStatus {
    Generated,      // Python AI agent completed generation
    Loaded,         // Successfully loaded into Bevy inspector
    Validated,      // Passed all validation checks
    Approved,       // Human approved for runtime use
    Rejected,       // Human rejected, needs regeneration
    Error(String),  // Validation failed with error details
}
```

### PyO3 Integration: Python ↔ Rust Bridge
```rust
// crates/assets-inspector/src/python_bridge.rs
use pyo3::prelude::*;

/// Python module for Rust assets inspector integration
#[pymodule]
fn dragon_assets_inspector(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(notify_asset_generated, m)?)?;
    m.add_function(wrap_pyfunction!(get_validation_status, m)?)?;
    m.add_function(wrap_pyfunction!(mark_asset_approved, m)?)?;
    Ok(())
}

/// Called by Python AI agents when asset is generated
#[pyfunction]
fn notify_asset_generated(
    asset_id: String,
    agent_name: String,
    file_path: String,
    metadata: HashMap<String, String>
) -> PyResult<bool> {
    // Notify Rust inspector that new asset is ready for validation
    // Trigger immediate loading and validation in Bevy inspector
    Ok(true)
}

/// Called by Python workflows to check validation status
#[pyfunction] 
fn get_validation_status(asset_id: String) -> PyResult<String> {
    // Return current validation status from Rust inspector
    Ok("validated".to_string())
}

/// Called by Python human-in-the-loop system when asset approved
#[pyfunction]
fn mark_asset_approved(asset_id: String, approved: bool) -> PyResult<()> {
    // Update approval status in Rust inspector
    // Trigger export to final runtime asset manifest
    Ok(())
}
```

### bevy-inspector-egui Integration: Visual Validation Interface
```rust
// crates/assets-inspector/src/inspector_ui.rs
use bevy_inspector_egui::prelude::*;
use egui_dock::{DockArea, DockState, NodeIndex};

/// Custom inspector windows for AI-generated assets
#[derive(Debug, Clone)]
enum DragonInspectorWindow {
    AssetDatabase,      // Browse all generated assets by category/dread level
    ValidationQueue,    // Assets awaiting human approval
    AgentStatus,        // Status of each AI agent workflow
    RuntimePreview,     // Preview how assets will look in game
    PerformanceMetrics, // Asset optimization and performance impact
    ExportQueue,        // Assets ready for runtime consumption
}

/// Main inspector UI system
fn dragon_inspector_ui(
    mut egui_contexts: Query<&mut EguiContext>,
    mut database: ResMut<GeneratedAssetsDatabase>,
    asset_server: Res<AssetServer>,
    mut ui_state: ResMut<InspectorUiState>,
) {
    let Ok(mut egui_context) = egui_contexts.get_single_mut() else { return };
    let ctx = egui_context.get_mut();

    // Create dockable inspector interface
    DockArea::new(&mut ui_state.dock_state)
        .show(ctx, &mut DragonTabViewer {
            database: &mut database,
            asset_server: &asset_server,
        });
}

/// Asset validation preview system  
fn preview_generated_assets(
    mut commands: Commands,
    database: Res<GeneratedAssetsDatabase>,
    asset_server: Res<AssetServer>,
    query: Query<Entity, With<PreviewAsset>>,
) {
    // Load and display assets for visual validation
    // Show dread progression variants side-by-side
    // Enable real-time switching between corruption levels
    // Display performance metrics (poly count, texture size, etc.)
}
```

### ECS Mapping Layer: SQLite ↔ Bevy Components
```rust
// crates/assets-inspector/src/ecs_mapping.rs
use bevy_ecs_sqlx::prelude::*;

/// Map SQLite asset database to Bevy ECS components for validation
#[derive(Component, Reflect, FromRow)]
struct GeneratedHexTile {
    id: String,
    terrain_type: TerrainType,
    dread_level: u8,
    corruption_intensity: f32,
    model_path: String,
    texture_paths: Vec<String>,
    performance_score: f32,
    agent_generated: String, // "MapsAgent"
}

#[derive(Component, Reflect, FromRow)]
struct GeneratedCompanionState {
    companion_name: String,
    trauma_level: f32,
    dread_stage: u8,
    model_path: String,
    animation_paths: Vec<String>,
    dialogue_tree_path: String,
    agent_generated: String, // "DialogueAgent"
}

/// System to sync SQLite database with Bevy ECS for validation
fn sync_database_to_ecs(
    mut commands: Commands,
    database: Res<GeneratedAssetsDatabase>,
) {
    // Query SQLite database for new/updated assets
    // Spawn Bevy entities with components for validation
    // Enable inspector UI to browse and validate ECS entities
    // Create preview entities for human review
}

/// Export validated ECS entities to runtime asset manifest
fn export_validated_assets(
    query: Query<&GeneratedHexTile, With<ValidationApproved>>,
    companion_query: Query<&GeneratedCompanionState, With<ValidationApproved>>,
) {
    // Export approved assets to final runtime manifest
    // Generate optimized asset bundles for game consumption
    // Create performance-optimized versions for mobile
}
```

## Integration with Revolutionary Architecture

### Python AI Agent Integration
```python
# dragon-ai/agents/base_agent.py
import dragon_assets_inspector

class DragonAgent(BaseAgent):
    def export_asset(self, asset: GeneratedAsset) -> bool:
        """Export generated asset to Rust inspector for validation."""
        
        # Notify Rust inspector that asset is ready
        success = dragon_assets_inspector.notify_asset_generated(
            asset_id=asset.id,
            agent_name=self.agent_name,
            file_path=str(asset.export_path),
            metadata=asset.metadata
        )
        
        if success:
            # Wait for validation to complete
            while True:
                status = dragon_assets_inspector.get_validation_status(asset.id)
                if status in ["approved", "rejected", "error"]:
                    break
                time.sleep(1.0)
            
            return status == "approved"
        
        return False
```

### Human-in-the-Loop Visual Review Workflow
```rust
/// Human visual review system integrated with inspector
fn human_review_system(
    mut database: ResMut<GeneratedAssetsDatabase>,
    mut ui_state: ResMut<InspectorUiState>,
    keyboard: Res<Input<KeyCode>>,
) {
    // Keyboard shortcuts for rapid asset approval
    if keyboard.just_pressed(KeyCode::A) {
        // Approve currently selected asset
        if let Some(asset_id) = ui_state.selected_asset {
            approve_asset(&mut database, &asset_id);
        }
    }
    
    if keyboard.just_pressed(KeyCode::R) {
        // Reject currently selected asset, request regeneration
        if let Some(asset_id) = ui_state.selected_asset {
            reject_asset(&mut database, &asset_id);
        }
    }
    
    if keyboard.just_pressed(KeyCode::Space) {
        // Cycle through dread level variants
        cycle_dread_variants(&mut ui_state);
    }
}
```

## Benefits of Three-Layer Architecture

### A) Dogfooding: Test Before Runtime
- **Rust Inspector validates SQLite database access patterns** before game runtime
- **Performance testing** of asset loading with actual Bevy systems
- **Integration validation** of all agent outputs with Bevy ecosystem crates
- **Memory usage profiling** to ensure < 200MB target met

### B) ECS Mapping: Structured Data Validation  
- **Component validation** ensures generated assets map correctly to game ECS
- **Performance metrics** calculated during validation (poly count, texture size)
- **Compatibility testing** with Hexx, YarnSpinner, Cobweb, Yoleck integrations
- **Mobile optimization** validation for 30 FPS target

### C) Human-in-the-Loop: Visual Quality Assurance
- **Side-by-side dread progression preview** showing all 5 corruption stages
- **Real-time asset switching** to validate smooth horror transitions  
- **Performance impact visualization** showing FPS impact of each asset
- **Rapid approval workflow** with keyboard shortcuts for efficiency
- **Rejection feedback system** that triggers Python AI regeneration

## Implementation Priority

### Week 1 Addition (Days 1-2)
```
TASK: Create assets-inspector crate with PyO3 bridge
- Create new crate: crates/assets-inspector/
- Add PyO3 dependencies and Python module setup
- Create basic bevy-inspector-egui integration
- Implement SQLite database connection to Python AI output
```

### Week 1 Integration (Days 6-7)  
```
TASK: Connect inspector to Python AI workflow validation
- Python agents export to inspector for validation
- Rust inspector loads and validates all generated assets
- Human review interface operational for asset approval
- ECS mapping validates compatibility with game runtime
```

This three-layer architecture is **revolutionary** - it gives us visual debugging, validation testing, and human quality control that no other game development pipeline has. We're essentially creating a **development-time game engine** that validates AI-generated content before runtime consumption.

The human-in-the-loop becomes a **visual validation workflow** where you can see exactly how horror progression affects every asset, test performance impact in real-time, and approve/reject with keyboard shortcuts for maximum efficiency.

**This is the missing piece that makes our architecture production-ready!** 🚀
