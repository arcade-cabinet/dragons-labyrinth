# Asset-Database Integration Architecture

## Overview
Complete architecture for integrating 70k+ database entities with comprehensive CC0 asset library into a unified, dread-responsive horror RPG experience.

## Current State Analysis

### ✅ Assets Available
- **6 hex biome models**: Perfect match for world tiles
- **Extensive horror characters**: Zombies, ghosts, skeletons, survivors
- **Complete dungeon toolkit**: Walls, floors, stairs, doors, props
- **Full weapon sets**: Melee, ranged, magical with material variants
- **Rich audio library**: Combat, environment, UI sounds
- **Supporting assets**: Fonts, textures, sprites

### ✅ Database Foundation
- **70k+ HBF entities** imported and ready
- **Dual-database architecture** with intelligent routing
- **Production ECS systems** (11 total) with horror progression
- **Asset reference fields** partially implemented (hex_tiles has `tile_asset_id`)

### ❌ Missing Integration Layer
- No asset registry or manifest system
- Incomplete asset reference fields across all models
- No dread-progression asset variants
- Build system still uses HBF patterns instead of entity+asset data

## Integration Architecture Design

### 1. Asset Registry System

**Core Registry Structure:**
```rust
// crates/game-database/src/assets/registry.rs
use bevy::prelude::*;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetBinding {
    pub entity_id: Uuid,
    pub primary_asset: String,      // Main 3D model path
    pub texture_variants: Vec<String>, // Texture options
    pub audio_cues: Vec<String>,       // Associated sounds
    pub dread_variants: Vec<DreadAsset>, // Horror progression variants
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DreadAsset {
    pub dread_level: i32,           // 0-4
    pub asset_path: String,         // Path to asset variant
    pub asset_type: AssetType,      // Model, texture, audio
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AssetType {
    Model,
    Texture,
    Audio,
    Effect,
}

#[derive(Resource, Debug, Clone)]
pub struct AssetRegistry {
    pub hex_tiles: HashMap<String, AssetBinding>,     // biome_type -> assets
    pub npcs: HashMap<String, Vec<AssetBinding>>,     // npc_type -> asset variants
    pub weapons: HashMap<String, Vec<AssetBinding>>,  // weapon_type -> material variants
    pub dungeons: HashMap<String, AssetBinding>,     // room_type -> assets
    pub audio_events: HashMap<String, Vec<String>>,  // event_type -> audio_paths
}
```

### 2. Database Model Updates

**Add Asset Reference Fields to ALL Models:**

**NPCs Enhancement:**
```rust
// Add to crates/database-orm/src/npcs.rs
pub model_asset_id: Option<String>,           // 3D character model
pub texture_variant: Option<String>,          // Texture/material choice
pub animation_set: Option<String>,            // Animation asset references  
pub voice_audio_id: Option<String>,           // Voice/dialogue audio
pub dread_level_overrides: Option<serde_json::Value>, // Custom dread variants
```

**Items/Equipment Model (New):**
```rust
// crates/database-orm/src/items.rs
pub model_asset_id: Option<String>,           // 3D weapon/item model
pub icon_sprite_id: Option<String>,           // 2D inventory icon
pub material_variant: String,                 // wood/stone/gold/diamond
pub audio_equip_id: Option<String>,          // Equip/unequip sound
pub audio_use_ids: Option<serde_json::Value>, // Usage sounds (sword clashing, etc.)
```

**Dungeon Rooms Enhancement:**
```rust
// Add to crates/database-orm/src/dungeons/rooms.rs
pub floor_asset_id: Option<String>,           // Floor tile model
pub wall_asset_ids: Option<serde_json::Value>, // Wall piece combinations
pub prop_asset_ids: Option<serde_json::Value>, // Furniture, chests, etc.
pub lighting_asset_id: Option<String>,        // Lighting effects
pub ambient_audio_id: Option<String>,         // Room ambience
```

### 3. Asset Server Integration

**Bevy Asset Server Enhancement:**
```rust
// crates/game-database/src/assets/server.rs
use bevy::prelude::*;

#[derive(Resource)]
pub struct DragonLabyrinthAssetServer {
    registry: AssetRegistry,
    dread_level: i32,           // Current world dread (0-4)
    asset_cache: HashMap<String, Handle<Scene>>,
    audio_cache: HashMap<String, Handle<AudioSource>>,
}

impl DragonLabyrinthAssetServer {
    pub fn get_hex_tile_asset(&self, biome: &str) -> Option<Handle<Scene>> {
        let binding = self.registry.hex_tiles.get(biome)?;
        
        // Choose asset variant based on current dread level
        let asset_path = self.select_dread_variant(&binding.dread_variants)?;
        
        self.asset_cache.get(asset_path).cloned()
    }
    
    pub fn get_npc_assets(&self, npc_type: &str, corruption: f32) -> NPCAssetBundle {
        // Select appropriate asset variant based on NPC type and corruption level
        // Return complete asset bundle: model, textures, audio, effects
    }
    
    fn select_dread_variant(&self, variants: &[DreadAsset]) -> Option<&String> {
        // Algorithm to select appropriate asset based on current dread level
        // Falls back gracefully if no variant available for current dread
    }
}
```

### 4. Build System Evolution

**Transform build.rs from HBF patterns → Entity+Asset data:**

```rust
// crates/game-database/build.rs evolution
use std::collections::HashMap;

fn main() {
    // Phase 1: Import 70k+ HBF entities (EXISTING ✅)
    import_hbf_data();
    
    // Phase 2: Generate Asset Registry (NEW)
    let asset_registry = generate_asset_registry();
    
    // Phase 3: Link Entities to Assets (NEW) 
    link_entities_to_assets(&asset_registry);
    
    // Phase 4: Generate ECS World Data (NEW)
    generate_ecs_world_data(&asset_registry);
    
    // Phase 5: Create Distribution Bundles (NEW)
    create_distribution_assets(&asset_registry);
}

fn generate_asset_registry() -> AssetRegistry {
    let mut registry = AssetRegistry::default();
    
    // Scan asset directories and build comprehensive manifest
    registry.hex_tiles = scan_hex_tile_assets();
    registry.npcs = scan_character_assets(); 
    registry.weapons = scan_weapon_assets();
    registry.dungeons = scan_dungeon_assets();
    registry.audio_events = scan_audio_assets();
    
    // Generate dread variants for horror progression
    registry.generate_dread_variants();
    
    registry
}

fn link_entities_to_assets(registry: &AssetRegistry) {
    // Connect each database entity to appropriate asset bundles
    // Examples:
    // - Hex tile entity.biome_type -> registry.hex_tiles[biome_type]
    // - NPC entity.race + entity.role -> registry.npcs[race_role]
    // - Item entity.weapon_type + material -> registry.weapons[weapon_material]
}
```

### 5. Dread Progression Asset System

**Progressive Horror Through Asset Variants:**

```rust
// Asset progression maps clean → corrupted variants
pub struct DreadProgressionAssets {
    pub hex_tiles: HashMap<String, DreadVariants>,     // forest_clean → forest_corrupted
    pub characters: HashMap<String, DreadVariants>,    // human_normal → human_zombified
    pub audio: HashMap<String, DreadVariants>,         // ambient_calm → ambient_terrifying
}

#[derive(Debug, Clone)]
pub struct DreadVariants {
    pub level_0: String,    // Clean/normal state
    pub level_1: String,    // Slight unease
    pub level_2: String,    // Growing corruption  
    pub level_3: String,    // Heavy corruption
    pub level_4: String,    // Maximum horror
}

// Implementation strategy:
// Level 0: Use standard assets (grass.glb, human.glb)
// Level 1-2: Apply shader effects, texture swaps
// Level 3-4: Use horror-specific models (ghost_town.glb, zombie.glb)
```

### 6. Asset Loading Performance Strategy

**For 70k+ Entities:**
```rust
// crates/game-database/src/assets/loading.rs

#[derive(Resource)]
pub struct AssetLoadingStrategy {
    pub viewport_radius: u32,           // Load assets within N hex tiles
    pub preload_cache_size: usize,      // Number of assets to keep preloaded
    pub dread_variant_preload: bool,    // Preload next dread level variants
}

impl AssetLoadingStrategy {
    pub fn get_viewport_entities(&self, center: HexCoord) -> Vec<EntityAssetBinding> {
        // Return all entities within viewport that need assets loaded
        // Prioritize by distance from player and current dread level
    }
    
    pub fn preload_dread_variants(&self, current_dread: i32) {
        // Preload assets for current + next dread level
        // Enables smooth progression without loading hitches
    }
}
```

## Implementation Plan

### Phase 1: Asset Registry Foundation
```bash
# Create asset registry system
crates/game-database/src/assets/
├── mod.rs              # Public API
├── registry.rs         # AssetRegistry core
├── scanner.rs          # Directory scanning logic
├── binding.rs          # Entity-asset binding logic
└── loading.rs          # Performance loading strategies
```

### Phase 2: Database Model Updates
```bash
# Add asset reference fields to all entity models
crates/database-orm/src/
├── hex_tiles.rs        # ✅ Already has asset fields
├── npcs.rs             # Add model_asset_id, texture_variant, etc.
├── items.rs            # Create new model with full asset integration
├── dungeons/rooms.rs   # Add asset reference fields
└── settlements.rs      # Add asset reference fields
```

### Phase 3: Build System Integration
```bash
# Evolution from HBF patterns to Entity+Asset generation
crates/game-database/build.rs:
1. Scan assets → Generate registry
2. Link 70k+ entities → Asset bindings  
3. Generate ECS world data with asset references
4. Create optimized distribution bundles
```

### Phase 4: Bevy Integration
```bash
# Complete ECS integration with asset loading
crates/game-database/src/bevy_integration.rs:
1. AssetRegistry as Bevy Resource
2. Asset loading systems with viewport optimization
3. Dread progression asset swapping
4. Audio event system integration
```

### Phase 5: Horror Progression Assets
```bash
# Implement dread-responsive asset system
1. Map clean assets → corrupted variants
2. Implement runtime asset swapping based on dread level
3. Audio progression system (calm → terrifying)
4. Shader effects for intermediate corruption levels
```

## Success Criteria

### ✅ Complete Asset Integration
1. **All 70k+ entities** have appropriate asset bindings
2. **Asset registry** indexes all static assets efficiently
3. **Dread progression** drives asset selection dynamically
4. **Performance optimized** for mobile/web deployment

### ✅ Build System Evolution
1. **build.rs generates ECS data** from entities + assets (not HBF patterns)
2. **Distribution bundles** include only used assets
3. **Asset manifest** enables efficient loading
4. **Dual-database + assets** ready for production deployment

### ✅ Horror Experience Complete
1. **Progressive asset corruption** from clean → terrifying
2. **Audio landscapes** evolve with dread progression
3. **Visual storytelling** through environmental asset changes
4. **Emotional journey** supported by asset-driven atmosphere

This architecture bridges the gap between 70k+ database entities and comprehensive CC0 assets, creating the foundation for a fully playable, visually rich horror RPG experience with progressive corruption driven by the dread system.

## Technical Notes

**Format Preferences:**
- **3D Models**: `.glb` for performance, `.gltf` for development
- **Audio**: `.ogg` for compression and compatibility
- **Textures**: Embedded in models or separate based on dread variants

**Asset Naming Convention:**
- Kenney assets (`k_`): Clean, cartoonish baseline
- Quaternius assets (`q_`): More detailed, horror-appropriate
- Use Quaternius for higher dread levels, Kenney for baseline

**Memory Management:**
- Viewport-based loading for large world
- Asset streaming based on movement prediction
- Aggressive unloading of distant assets
- Dread variant preloading for smooth progression

This completes the comprehensive asset integration architecture design for Dragon's Labyrinth.
