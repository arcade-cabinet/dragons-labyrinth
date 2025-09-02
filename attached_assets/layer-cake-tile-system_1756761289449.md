# Layer Cake Tile System: The Ultimate Simplification

## The Paradigm Shift

Instead of complex concepts like "villages", "cities", "taverns" - just have **Tiles** that are layer cake containers.

## The Layer Cake Architecture

### Base Structure
```rust
#[derive(Component)]
struct Tile {
    coords: HexCoord,
    biome: Entity,           // The base terrain layer
    paths: Vec<Entity>,      // Overlay paths (roads, trails)
    features: Vec<Entity>,   // Overlay features (buildings, encounters)
}

#[derive(Component)]  
struct Biome {
    biome_type: BiomeType,   // grassland, forest, lava, snow, etc.
    texture_id: String,      // Base texture for rendering
    
    // Gameplay effects
    movement_speed_multiplier: f32,  // 1.0 = normal, 0.5 = slow, 2.0 = fast
    mounted_speed_multiplier: f32,   // Different speed for mounts
    damage_per_turn: f32,           // Environmental damage (lava, poison)
    companion_stress_modifier: f32,  // How this affects companion psychology
    
    // Adjacency rules
    compatible_neighbors: Vec<BiomeType>,
}

#[derive(Component)]
struct PathOverlay {
    path_type: PathType,     // wooden_planks, stone_road, dirt_trail
    texture_id: String,      // Overlay texture
    opacity: f32,           // How transparent the overlay is
    
    // Gameplay effects  
    speed_bonus: f32,       // Movement speed bonus on paths
    comfort_bonus: f32,     // Companion comfort from civilization
}

#[derive(Component)]
struct FeatureOverlay {
    feature_type: FeatureType,  // tavern, dungeon_entrance, shrine, etc.
    model_id: String,          // 3D model to render
    interaction_type: InteractionType,  // enter_dungeon, talk_to_npc, rest_at_inn
    
    // Horror integration
    dread_level_modifier: f32,
    corruption_resistance: f32,
    companion_reactions: HashMap<CompanionType, EmotionalResponse>,
}
```

## Visual Rendering Pipeline

### Layer Cake Rendering
```rust
fn render_tile_system(
    tiles: Query<&Tile>,
    biomes: Query<&Biome>,
    paths: Query<&PathOverlay>,
    features: Query<&FeatureOverlay>,
    mut tilemap: ResMut<TileMap>,
) {
    for tile in tiles.iter() {
        // Layer 1: Base biome
        if let Ok(biome) = biomes.get(tile.biome) {
            tilemap.set_tile(tile.coords, biome.texture_id.clone());
        }
        
        // Layer 2: Path overlays
        for path_entity in &tile.paths {
            if let Ok(path) = paths.get(*path_entity) {
                tilemap.add_overlay(tile.coords, path.texture_id.clone(), path.opacity);
            }
        }
        
        // Layer 3: Feature overlays
        for feature_entity in &tile.features {
            if let Ok(feature) = features.get(*feature_entity) {
                tilemap.add_model(tile.coords, feature.model_id.clone());
            }
        }
    }
}
```

## Biome Adjacency System

### Smart Biome Placement
```rust
enum BiomeType {
    Grassland,
    Forest, 
    Mountain,
    Desert,
    Swamp,
    Tundra,
    Lava,
    Void,        // Late game corruption
    Shattered,   // Reality fragments
}

impl BiomeType {
    fn compatible_neighbors(&self) -> Vec<BiomeType> {
        match self {
            Grassland => vec![Forest, Mountain, Desert, Swamp],
            Forest => vec![Grassland, Mountain, Swamp],
            Desert => vec![Grassland, Mountain, Lava], // Hot climates
            Tundra => vec![Mountain],                  // Cold climates  
            Lava => vec![Desert, Mountain, Void],      // Extreme heat
            Void => vec![Lava, Shattered],            // Corruption spreads
            Mountain => vec![Grassland, Forest, Desert, Tundra], // Mountains border everything
            _ => vec![],
        }
    }
    
    fn movement_effects(&self) -> MovementEffects {
        match self {
            Grassland => MovementEffects::normal(),
            Forest => MovementEffects { speed: 0.8, mounted_penalty: 0.6 },
            Mountain => MovementEffects { speed: 0.6, mounted_penalty: 0.3 },
            Desert => MovementEffects { speed: 0.7, damage: 1.0 }, // Heat damage
            Swamp => MovementEffects { speed: 0.5, mounted_penalty: 0.2 },
            Lava => MovementEffects { speed: 0.3, damage: 10.0 }, // Severe damage
            Void => MovementEffects { speed: 1.5, sanity_loss: 5.0 }, // Fast but horrific
            _ => MovementEffects::normal(),
        }
    }
}
```

## Feature Complexity Scaling

### Simple Feature System
```rust
enum FeatureType {
    // Social features (generate NPCs, shops, rumors)
    Tavern { keeper: NPC, staff: Vec<NPC>, rumors: RumorTable },
    Shop { owner: NPC, shop_type: ShopType, inventory: Vec<Item> },
    Shrine { deity: String, blessing_effect: Effect },
    
    // Encounter features
    DungeonEntrance { dungeon_type: DungeonType, max_cr: u32 },
    MonsterLair { monster_type: MonsterType, cr: u32 },
    TreasureCache { loot_table: LootTable },
    
    // Interactive features  
    Campsite { rest_bonus: f32, safety_level: f32 },
    Bridge { crosses_obstacle: ObstacleType },
    Portal { destination: HexCoord },
}
```

## Porting Existing Game-Database Code

### What to Move to Game-Engine
```rust
// Port these directly (remove SeaORM, keep logic)
crates/game-database/src/systems/hex_rendering/ → crates/game-engine/src/systems/tile_rendering/
crates/game-database/src/systems/weather/ → crates/game-engine/src/systems/weather/
crates/game-database/src/systems/corruption/ → crates/game-engine/src/systems/corruption/
crates/game-database/src/systems/dread_progression/ → crates/game-engine/src/systems/dread/
crates/game-database/src/systems/companion_psychology/ → crates/game-engine/src/systems/companions/

// Transform these to layer cake approach
crates/game-database/src/systems/settlement/ → DELETE (replace with FeatureOverlay)
crates/game-database/src/systems/faction/ → crates/game-engine/src/systems/social_networks/
crates/game-database/src/systems/dungeon/ → crates/game-engine/src/systems/encounters/
```

### Simplification Benefits
```rust
// OLD: Complex hierarchy
struct Village {
    shops: Vec<Shop>,
    taverns: Vec<Tavern>, 
    inns: Vec<Inn>,
    npcs: Vec<NPC>,
    districts: Vec<District>,
}

// NEW: Simple feature overlays
struct Tile {
    biome: Entity,              // Grassland with movement effects
    features: Vec<Entity>,      // [TavernFeature, ShopFeature, ShopFeature]
}

// Each TavernFeature generates its own NPCs, rumors, etc.
// No complex inter-relationships to manage
// Just individual features that work independently
```

## AI Content Generation Integration

### Generate Layer Cake Content
```python
class TileGenerationWorkflow:
    def generate_settlement_tiles(self, hex_coords: list[tuple], horror_level: int) -> list[TileSpec]:
        """Generate 3-5 connected tiles for a settlement"""
        
        # Center tile: Main tavern/social hub
        center_tile = TileSpec(
            coords=hex_coords[0],
            biome=BiomeSpec(type="grassland", corruption=horror_level * 0.1),
            paths=[PathSpec(type="stone_road", connects_to=hex_coords[1:])],
            features=[
                TavernFeature(
                    name=self.generate_tavern_name(horror_level),
                    keeper=self.generate_npc_with_trauma(horror_level),
                    staff=self.generate_staff_npcs(3, horror_level),
                    rumors=self.generate_horror_rumors(horror_level)
                )
            ]
        )
        
        # Feature tiles: Shops, services, encounters
        feature_tiles = []
        for coord in hex_coords[1:]:
            feature_tiles.append(TileSpec(
                coords=coord,
                biome=BiomeSpec(type="grassland", corruption=horror_level * 0.1),
                paths=[PathSpec(type="wooden_planks", connects_to=[hex_coords[0]])],
                features=[self.generate_random_feature(horror_level)]
            ))
        
        return [center_tile] + feature_tiles
```

## The Blender Template Integration

### Perfect Match with hex_tile.py.j2
The existing template already supports layer cake rendering:
- **Base geometry**: Hexagon with height variation
- **Primary texture**: Biome texture (grassland, forest, etc.)
- **Detail textures**: Path and feature overlays
- **Height variation**: Noise-based terrain complexity

### Template Enhancement for Layer Cake
```python
# Enhanced template variables
{
  "biome_texture": "forest_tile.png",
  "path_overlays": ["wooden_planks.png", "stone_road.png"],
  "feature_overlays": ["tavern_sign.png"],
  "height_variation": 0.1,
  "corruption_intensity": 0.3
}
```

## Game Database Migration Plan

### Phase 1: Port Core Systems
1. **hex_rendering** → **tile_rendering** (remove DB, keep ECS logic)
2. **weather** → **weather** (direct port, already pure ECS)
3. **corruption** → **corruption** (direct port, perfect for layer cake)
4. **dread_progression** → **dread** (direct port, horror integration)
5. **companion_psychology** → **companions** (direct port, tile reactions)

### Phase 2: Simplify Complex Systems  
1. **settlement** → DELETE (replace with FeatureOverlay generation)
2. **faction** → **social_networks** (simplified NPC relationships)
3. **dungeon** → **encounters** (simplified feature interactions)

### Phase 3: Asset Library Mirror
```python
class AssetLibraryMirror:
    def scan_rust_assets(self) -> None:
        # Create SQLite database of all available assets
        # Categorize by: biome compatibility, corruption level, feature type
        
    def find_biome_textures(self, biome_type: str) -> list[str]:
        # Return textures for specific biome
        
    def find_path_overlays(self, path_type: str) -> list[str]:
        # Return path overlay textures
        
    def find_feature_models(self, feature_type: str, corruption: float) -> list[str]:
        # Return 3D models for features based on corruption
```

## The Ultimate Architecture

**Everything becomes composable layers:**
- **Tile**: Container for coordinates
- **Biome**: Base layer with gameplay effects and adjacency rules
- **Paths**: Transparent overlays connecting tiles  
- **Features**: Interactive overlays (taverns, dungeons, encounters)

**AI generates content using features.json patterns:**
- Taverns with keepers, staff, patrons, rumors
- Dungeons with CR limits, monster tables, treasure
- NPCs with psychology integration and trauma responses
- Shops with owners, inventory, and horror-appropriate goods

**No more complex hierarchies - just composable, reusable layers!**
