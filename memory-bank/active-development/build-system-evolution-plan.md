# Build System Evolution Plan

## Current State vs Target State

### Current Build System (HBF Pattern-Based)
```rust
// crates/game-database/build.rs - CURRENT
fn main() {
    // Import HBF data into database
    import_hbf_data();
    
    // Generate some basic manifests
    // Uses HBF HTML patterns for content generation
    // No asset integration
    // No ECS world data generation
}
```

### Target Build System (Entity+Asset Integration)
```rust
// crates/game-database/build.rs - TARGET
fn main() {
    // Phase 1: Import Foundation Data ✅
    import_hbf_data();
    
    // Phase 2: Asset Discovery & Registry (NEW)
    let asset_registry = build_asset_registry();
    
    // Phase 3: Entity-Asset Binding (NEW)
    link_entities_to_assets(&asset_registry);
    
    // Phase 4: ECS World Generation (NEW)
    generate_ecs_world_data();
    
    // Phase 5: Distribution Optimization (NEW)
    optimize_for_distribution();
}
```

## Implementation Phases

### Phase 1: Asset Registry Creation ⭐

**Create `crates/game-database/src/assets/` module:**

```rust
// crates/game-database/src/assets/mod.rs
pub mod registry;
pub mod scanner;
pub mod binding;
pub mod loading;

pub use registry::*;
pub use scanner::*;
pub use binding::*;
pub use loading::*;
```

```rust
// crates/game-database/src/assets/scanner.rs
use std::path::Path;
use std::collections::HashMap;

pub fn scan_static_assets() -> AssetScanResult {
    let base_path = Path::new("../game-content-static/assets");
    
    AssetScanResult {
        hex_tiles: scan_hex_tiles(&base_path.join("models/hex_tiles")),
        characters: scan_characters(&base_path.join("models/characters")),
        horror: scan_horror(&base_path.join("models/horror")),
        weapons: scan_weapons(&base_path.join("models/weapons")),
        dungeon: scan_dungeon(&base_path.join("models/dungeon")),
        audio: scan_audio(&base_path.join("audio")),
        textures: scan_textures(&base_path.join("textures")),
    }
}

fn scan_hex_tiles(path: &Path) -> HashMap<String, Vec<AssetPath>> {
    // Scan for hex_forest.glb, hex_swamp.glb, etc.
    // Extract biome type from filename
    // Return biome_type -> [asset_paths] mapping
}
```

### Phase 2: Database Model Enhancement

**Add Asset Fields to Core Models:**

```rust
// crates/database-orm/src/npcs.rs - ADDITIONS
impl Model {
    // Add these fields to struct Model:
    pub character_model_id: Option<String>,       // "q_horror_characters_zombie_basic"
    pub texture_variant: Option<String>,          // Material/skin variant
    pub animation_set_id: Option<String>,         // Animation package reference
    pub voice_audio_id: Option<String>,           // Dialogue audio reference
    pub footstep_audio_id: Option<String>,        // Movement audio
    pub corruption_model_id: Option<String>,      // Corrupted version asset
}
```

```rust
// crates/database-orm/src/items.rs - NEW MODEL
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "items")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    
    // Core item data
    #[sea_orm(column_type = "Text")]
    pub name: String,                             // "Iron Sword", "Health Potion"
    
    #[sea_orm(column_type = "Text")]
    pub item_type: String,                        // "weapon", "armor", "consumable"
    
    #[sea_orm(column_type = "Text")]
    pub weapon_type: Option<String>,              // "sword", "axe", "dagger"
    
    // Asset Integration
    pub model_asset_id: Option<String>,           // "q_weapons_sword_gold.glb"
    pub icon_sprite_id: Option<String>,           // 2D inventory icon
    pub material_variant: String,                 // "wood", "stone", "gold", "diamond"
    
    // Audio Integration
    pub equip_audio_id: Option<String>,           // Sound when equipped
    pub use_audio_ids: Option<serde_json::Value>, // ["k_sword1.ogg", "k_sword2.ogg"]
    pub impact_audio_ids: Option<serde_json::Value>, // Hit sounds by material
    
    // Dread Progression
    pub corruption_effects: Option<serde_json::Value>, // How item changes with dread
    pub corrupted_model_id: Option<String>,       // Horror version of item
    
    // Game mechanics (existing D&D data)
    // ... rest of item fields ...
}
```

### Phase 3: Asset Registry Implementation

**Core Registry Logic:**
```rust
// crates/game-database/src/assets/registry.rs
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetRegistry {
    // Core asset mappings
    pub hex_tiles: HashMap<String, HexTileAssets>,      // biome -> assets
    pub characters: HashMap<String, CharacterAssets>,   // type -> variants
    pub weapons: HashMap<String, WeaponAssets>,         // weapon -> materials
    pub audio_events: HashMap<String, Vec<String>>,     // event -> sound_files
    
    // Dread progression mappings
    pub dread_variants: HashMap<String, DreadVariants>, // base_asset -> dread_levels
    
    // Performance optimization
    pub asset_metadata: HashMap<String, AssetMetadata>, // asset_path -> metadata
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HexTileAssets {
    pub model_path: String,           // "models/hex_tiles/hex_forest.glb"
    pub texture_variants: Vec<String>, // Different seasonal/corruption variants
    pub ambient_audio: Vec<String>,   // Background sounds for this biome
    pub dread_progression: DreadVariants,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterAssets {
    pub base_model: String,           // "models/characters/k_characters_character_human.glb"
    pub texture_variants: Vec<String>, // Skin/clothing options
    pub animation_set: String,        // Animation package
    pub audio_package: CharacterAudio,
    pub corruption_variants: Vec<String>, // Progressive corruption models
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterAudio {
    pub footsteps: Vec<String>,       // Movement sounds
    pub voice_clips: Vec<String>,     // Dialogue audio
    pub combat_sounds: Vec<String>,   // Attack/hurt sounds
    pub death_sounds: Vec<String>,    // Death audio
}

impl AssetRegistry {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn build_from_scan(scan_result: AssetScanResult) -> Self {
        let mut registry = Self::new();
        
        // Process scan results into structured registry
        registry.process_hex_tiles(scan_result.hex_tiles);
        registry.process_characters(scan_result.characters);
        registry.process_weapons(scan_result.weapons);
        registry.process_audio(scan_result.audio);
        
        // Generate dread progression mappings
        registry.generate_dread_variants();
        
        registry
    }
    
    pub fn get_asset_for_entity(&self, entity_type: &str, entity_data: &serde_json::Value, dread_level: i32) -> Option<EntityAssetBundle> {
        match entity_type {
            "hex_tile" => self.get_hex_tile_assets(entity_data, dread_level),
            "npc" => self.get_character_assets(entity_data, dread_level),
            "weapon" => self.get_weapon_assets(entity_data, dread_level),
            _ => None,
        }
    }
}
```

### Phase 4: Build Process Integration

**Enhanced build.rs with Asset Processing:**
```rust
// crates/game-database/build.rs - ENHANCED
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=../game-content-static/assets");
    println!("cargo:rerun-if-changed=import/hbf_export_data.zip");
    
    // Phase 1: Foundation Data Import ✅
    import_hbf_entities()?;
    
    // Phase 2: Asset Registry Generation 🆕
    let asset_registry = build_comprehensive_asset_registry()?;
    write_asset_registry(&asset_registry)?;
    
    // Phase 3: Entity-Asset Binding 🆕  
    bind_entities_to_assets(&asset_registry)?;
    
    // Phase 4: ECS World Data Generation 🆕
    generate_bevy_world_data(&asset_registry)?;
    
    // Phase 5: Distribution Bundles 🆕
    create_distribution_manifest(&asset_registry)?;
    
    Ok(())
}

fn build_comprehensive_asset_registry() -> Result<AssetRegistry, Box<dyn std::error::Error>> {
    use crate::assets::scanner::scan_static_assets;
    
    println!("Scanning static assets...");
    let scan_result = scan_static_assets();
    
    println!("Building asset registry...");
    let registry = AssetRegistry::build_from_scan(scan_result);
    
    println!("Assets registered: {} hex tiles, {} characters, {} weapons, {} audio files", 
        registry.hex_tiles.len(),
        registry.characters.len(), 
        registry.weapons.len(),
        registry.audio_events.len()
    );
    
    Ok(registry)
}

fn bind_entities_to_assets(registry: &AssetRegistry) -> Result<(), Box<dyn std::error::Error>> {
    // Connect each of the 70k+ database entities to appropriate assets
    
    // Hex tiles: Match biome_type to asset registry
    update_hex_tile_assets(registry)?;
    
    // NPCs: Match race/role to character assets  
    update_npc_assets(registry)?;
    
    // Items: Match weapon_type/material to weapon assets
    update_item_assets(registry)?;
    
    // Dungeons: Match room types to dungeon component assets
    update_dungeon_assets(registry)?;
    
    Ok(())
}

fn generate_bevy_world_data(registry: &AssetRegistry) -> Result<(), Box<dyn std::error::Error>> {
    // Generate ECS world data that can be loaded directly into Bevy
    // Replace HBF HTML pattern generation with actual ECS entities + asset references
    
    let world_data = WorldDataGenerator::new(registry)
        .include_viewport_optimization()
        .include_dread_progression()
        .include_performance_metadata()
        .generate()?;
    
    // Write to crates/game-database/generated/world_data.ron
    write_world_data(&world_data)?;
    
    Ok(())
}

fn create_distribution_manifest(registry: &AssetRegistry) -> Result<(), Box<dyn std::error::Error>> {
    // Create optimized asset bundles for distribution
    // Only include assets that are actually used by the 70k+ entities
    
    let used_assets = calculate_used_assets(registry)?;
    let distribution_manifest = DistributionManifest {
        required_assets: used_assets,
        optional_assets: calculate_optional_assets(registry)?,
        asset_dependencies: calculate_dependencies(registry)?,
        loading_priority: calculate_loading_priority(registry)?,
    };
    
    write_distribution_manifest(&distribution_manifest)?;
    
    Ok(())
}
```

### Phase 5: ECS Integration Enhancement

**Update Bevy Integration for Asset Loading:**
```rust
// crates/game-database/src/bevy_integration.rs - ENHANCEMENTS
use bevy::prelude::*;
use crate::assets::AssetRegistry;

pub struct DatabaseAssetsPlugin;

impl Plugin for DatabaseAssetsPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(AssetRegistry::load_from_manifest())
            .add_systems(Startup, initialize_asset_loading)
            .add_systems(Update, (
                viewport_asset_loading,
                dread_progression_asset_swapping,
                audio_event_triggering,
                asset_cache_management,
            ));
    }
}

fn viewport_asset_loading(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    registry: Res<AssetRegistry>,
    player_query: Query<&Transform, With<Player>>,
    hex_query: Query<(Entity, &HexCoord, &BiomeType), Without<LoadedAssets>>,
) {
    // Load assets for hex tiles within viewport
    // Preload based on player movement direction
    // Unload distant assets to manage memory
}

fn dread_progression_asset_swapping(
    mut commands: Commands,
    asset_server: Res<AssetServer>, 
    registry: Res<AssetRegistry>,
    dread_state: Res<DreadState>,
    changed_dread: Query<Entity, (With<Handle<Scene>>, Changed<DreadLevel>)>,
) {
    // Swap assets when dread level changes
    // Smooth transitions: clean forest → corrupted forest → ghost town
    // Progressive audio changes: calm → unsettling → terrifying
}
```

### Phase 6: Content Pipeline Transformation

**From HBF Patterns → Structured World Generation:**

```rust
// crates/game-database/src/world_generation.rs - NEW
pub struct WorldGenerator {
    registry: AssetRegistry,
    entity_database: DatabaseConnection,
}

impl WorldGenerator {
    pub fn generate_world_region(&self, center: HexCoord, radius: u32) -> WorldRegion {
        // Query database for entities in region
        let hex_tiles = self.entity_database.get_hex_tiles_in_region(center, radius);
        let npcs = self.entity_database.get_npcs_in_region(center, radius);
        let dungeons = self.entity_database.get_dungeons_in_region(center, radius);
        
        // Generate ECS world data with asset bindings
        WorldRegion {
            hex_entities: hex_tiles.into_iter().map(|tile| {
                HexEntity {
                    transform: hex_to_world_transform(tile.q, tile.r),
                    mesh: self.registry.get_hex_tile_mesh(&tile.biome_type),
                    material: self.registry.get_hex_tile_material(&tile.biome_type, tile.corruption_level),
                    audio: self.registry.get_ambient_audio(&tile.biome_type),
                    metadata: tile,
                }
            }).collect(),
            
            npc_entities: npcs.into_iter().map(|npc| {
                NPCEntity {
                    transform: hex_to_world_transform(npc.hex_q, npc.hex_r),
                    mesh: self.registry.get_character_mesh(&npc.race, &npc.role),
                    animation: self.registry.get_character_animations(&npc.race),
                    audio: self.registry.get_character_audio(&npc.race),
                    metadata: npc,
                }
            }).collect(),
            
            // ... dungeon_entities, item_entities, etc.
        }
    }
}
```

## Asset-Entity Binding Logic

### Hex Tile Binding Strategy
```rust
fn bind_hex_tile_assets() -> Result<(), DatabaseError> {
    let hex_tiles = HexTile::find().all(&db).await?;
    
    for mut tile in hex_tiles {
        // Match biome_type to available assets
        tile.tile_asset_id = match tile.biome_type.as_str() {
            "forest" => Some("models/hex_tiles/hex_forest.glb".to_string()),
            "swamp" => Some("models/hex_tiles/hex_swamp.glb".to_string()),
            "ruins" => Some("models/hex_tiles/hex_ruins.glb".to_string()),
            "ghost_town" => Some("models/hex_tiles/hex_ghost_town.glb".to_string()),
            "labyrinth" => Some("models/hex_tiles/hex_labyrinth.glb".to_string()),
            "grass" => Some("models/hex_tiles/hex_grass.glb".to_string()),
            _ => None,
        };
        
        // Add ambient audio based on biome
        tile.ambient_audio_id = match tile.biome_type.as_str() {
            "forest" => Some("audio/nature/forest_ambient.ogg".to_string()),
            "swamp" => Some("audio/nature/swamp_ambient.ogg".to_string()),
            _ => None,
        };
        
        tile.update(&db).await?;
    }
    
    Ok(())
}
```

### NPC Asset Binding Strategy
```rust
fn bind_npc_assets() -> Result<(), DatabaseError> {
    let npcs = NPC::find().all(&db).await?;
    
    for mut npc in npcs {
        // Select character model based on race and role
        npc.character_model_id = select_character_model(&npc.race, &npc.role, npc.current_corruption_level);
        
        // Assign audio package
        npc.voice_audio_id = select_voice_audio(&npc.personality);
        npc.footstep_audio_id = Some("audio/nature/k_nature_footstep_grass_000.ogg".to_string());
        
        // Corruption variants for dread progression
        if npc.current_corruption_level > 0.5 {
            npc.character_model_id = select_corrupted_model(&npc.race);
        }
        
        npc.update(&db).await?;
    }
    
    Ok(())
}

fn select_character_model(race: &str, role: &str, corruption: f32) -> Option<String> {
    match (race, role, corruption) {
        ("human", "villager", c) if c < 0.3 => Some("models/characters/k_characters_character_human.glb".to_string()),
        ("human", "villager", c) if c >= 0.3 => Some("models/horror/characters/q_horror_characters_zombie_basic.glb".to_string()),
        ("human", "soldier", _) => Some("models/characters/q_characters_character_soldier.glb".to_string()),
        _ => Some("models/characters/q_characters_basecharacter.glb".to_string()),
    }
}
```

### Weapon Asset Binding Strategy
```rust
fn bind_weapon_assets() -> Result<(), DatabaseError> {
    let items = Item::find()
        .filter(item::Column::ItemType.eq("weapon"))
        .all(&db).await?;
    
    for mut weapon in items {
        // Map weapon type + material to specific asset
        weapon.model_asset_id = match (&weapon.weapon_type, &weapon.material_variant) {
            (Some(wtype), material) => select_weapon_model(wtype, material),
            _ => None,
        };
        
        // Assign combat audio based on weapon type
        weapon.use_audio_ids = Some(serde_json::json!(
            select_combat_audio(&weapon.weapon_type.unwrap_or_default())
        ));
        
        weapon.update(&db).await?;
    }
    
    Ok(())
}

fn select_weapon_model(weapon_type: &str, material: &str) -> Option<String> {
    match (weapon_type, material) {
        ("sword", "wood") => Some("models/weapons/q_weapons_sword_wood.glb".to_string()),
        ("sword", "stone") => Some("models/weapons/q_weapons_sword_stone.glb".to_string()),
        ("sword", "gold") => Some("models/weapons/q_weapons_sword_gold.glb".to_string()),
        ("sword", "diamond") => Some("models/weapons/q_weapons_sword_diamond.glb".to_string()),
        ("axe", _) => Some("models/weapons/q_weapons_weapon_axe.glb".to_string()),
        ("dagger", _) => Some("models/weapons/q_weapons_weapon_dagger.glb".to_string()),
        _ => None,
    }
}

fn select_combat_audio(weapon_type: &str) -> Vec<String> {
    match weapon_type {
        "sword" => vec![
            "audio/audio/k_sword1.ogg".to_string(),
            "audio/audio/k_sword2.ogg".to_string(),
            "audio/audio/k_sword3.ogg".to_string(),
        ],
        "axe" => vec![
            "audio/audio/k_swordmetal1.ogg".to_string(),
            "audio/audio/k_swordmetal2.ogg".to_string(),
        ],
        _ => vec!["audio/audio/k_sword1.ogg".to_string()],
    }
}
```

## Performance Optimization Strategy

### Viewport-Based Loading
```rust
// Only load assets for entities within player viewport
// Preload adjacent hexes based on movement direction
// Aggressive unloading of distant assets

pub struct ViewportManager {
    pub current_center: HexCoord,
    pub load_radius: u32,           // Load assets within N hexes
    pub preload_radius: u32,        // Preload movement predictions
    pub unload_radius: u32,         // Unload beyond N hexes
}
```

### Dread Progression Optimization
```rust
// Preload next dread level assets for smooth transitions
// Cache commonly used corruption variants
// Progressive loading: clean assets first, corruption variants as needed

pub struct DreadAssetCache {
    pub current_level_assets: HashMap<String, Handle<Scene>>,
    pub next_level_assets: HashMap<String, Handle<Scene>>,    // Preloaded for transitions
    pub transition_effects: HashMap<String, Handle<Shader>>, // Smooth corruption effects
}
```

## Distribution Strategy

### Asset Bundle Optimization
```rust
// Create asset bundles containing only used assets
// Eliminate unused variants and formats
// Optimize file sizes for web/mobile deployment

pub struct DistributionManifest {
    pub core_assets: Vec<String>,           // Always loaded
    pub streaming_assets: Vec<String>,      // Loaded on-demand
    pub dread_progression: Vec<String>,     // Corruption variants
    pub total_size: u64,                    // Bundle size estimate
    pub loading_priority: HashMap<String, u32>, // Load order optimization
}
```

## Implementation Timeline

### Week 1: Foundation
- [ ] Create `crates/game-database/src/assets/` module structure
- [ ] Implement asset scanning and registry generation
- [ ] Add asset reference fields to all database models
- [ ] Basic asset registry generation in build.rs

### Week 2: Integration
- [ ] Implement entity-asset binding logic
- [ ] Update all 70k+ entities with appropriate asset references
- [ ] Create ECS world data generation from entities + assets
- [ ] Test asset loading with sample hex tiles

### Week 3: Optimization
- [ ] Implement viewport-based asset loading
- [ ] Create dread progression asset swapping
- [ ] Optimize for mobile/web performance
- [ ] Generate distribution manifests

### Week 4: Polish
- [ ] Audio event system integration
- [ ] Asset caching and memory management
- [ ] Final testing with complete asset pipeline
- [ ] Documentation and handoff preparation

## Success Criteria

### ✅ Complete Integration
1. **Every database entity** has appropriate asset bindings
2. **Asset registry** efficiently indexes all static assets
3. **Build system** generates ECS data from entities + assets
4. **Dread progression** drives dynamic asset selection

### ✅ Performance Ready
1. **Viewport loading** handles 70k+ entities efficiently
2. **Asset streaming** supports mobile/web deployment
3. **Memory management** prevents asset bloat
4. **Distribution bundles** contain only necessary assets

### ✅ Horror Experience
1. **Progressive corruption** through asset variants
2. **Audio landscapes** evolve with dread
3. **Visual storytelling** through environmental changes
4. **Emotional journey** supported by asset-driven atmosphere

This build system evolution plan transforms the current HBF pattern-based system into a comprehensive entity+asset integration pipeline, enabling the 70k+ database entities to become a fully playable, visually rich horror RPG world.
