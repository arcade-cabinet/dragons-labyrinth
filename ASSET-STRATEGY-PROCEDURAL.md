# Asset Strategy: Procedural + Existing = Perfect!

## THE DISCOVERY
You already have:
1. **Hexagon models** ready to use (grass, overhang variants)
2. **Blender Python scripts** generating procedural assets
3. **1.7GB of CC0 assets** (mostly junk but some gems)

## THE BRILLIANT APPROACH

### 1. Use Existing Hex Models NOW
```
crates/game-content-static/assets/models/nature/
├── k_nature_block_grass_hexagon.glb          ✅ Ready!
├── k_nature_block_grass_low_hexagon.glb      ✅ Ready!
├── k_nature_block_grass_overhang_hexagon.glb ✅ Ready!
└── k_nature_block_grass_overhang_low_hexagon.glb ✅ Ready!
```

### 2. Generate Missing Hex Tiles with Blender
```python
# Your existing scripts already do this!
crates/game-content-static/blender/hex_tiles/
├── hex_grass.py      # ✅ Already written
├── hex_forest.py     # ✅ Already written  
├── hex_ghost_town.py # ✅ Already written
├── hex_corrupted.py  # TODO: Add corruption
├── hex_void.py       # TODO: Add void tiles
└── hex_water.py      # TODO: Add water
```

### 3. Wire into build.rs
```rust
// crates/game-content-static/build.rs
fn generate_procedural_assets() {
    // Only regenerate if Blender scripts changed
    let blender_files = glob("blender/**/*.py").unwrap();
    
    for script in blender_files {
        let output = script.replace(".py", ".glb")
                          .replace("blender/", "assets/generated/");
        
        if should_regenerate(&script, &output) {
            Command::new("blender")
                .args(&["-b", "-P", &script])
                .status()
                .expect("Failed to run Blender");
        }
    }
}
```

## THE PERFECT ASSET STRUCTURE

```
crates/game-content-static/assets/
├── models/
│   ├── hex_tiles/        # All hex tile models
│   │   ├── grass/        # Multiple grass variants
│   │   │   ├── grass_normal.glb     (existing)
│   │   │   ├── grass_low.glb        (existing)
│   │   │   ├── grass_overhang.glb   (existing)
│   │   │   └── grass_dead.glb       (generate)
│   │   ├── forest/       # Forest tiles
│   │   │   ├── forest_dense.glb     (generate)
│   │   │   ├── forest_sparse.glb    (generate)
│   │   │   └── forest_corrupted.glb (generate)
│   │   ├── corrupted/    # Mid-game tiles
│   │   │   ├── corrupted_grass.glb  (generate)
│   │   │   ├── void_cracks.glb      (generate)
│   │   │   └── flesh_ground.glb     (generate)
│   │   └── void/         # Late-game tiles
│   │       ├── nothing.glb          (generate)
│   │       ├── reality_fragment.glb (generate)
│   │       └── final_approach.glb   (generate)
│   ├── characters/       # NPCs and player
│   │   ├── player.glb    (generate or find)
│   │   ├── elena.glb     (generate)
│   │   ├── marcus.glb    (generate)
│   │   ├── quinn.glb     (generate)
│   │   └── villager_*.glb (already generated)
│   ├── bosses/           # 9 major + mini bosses
│   │   ├── dragon_head.glb (already generated!)
│   │   ├── bandit_leader.glb (generate)
│   │   ├── void_herald.glb   (generate)
│   │   └── ...
│   └── props/            # Reusable objects
│       ├── trees/        (find in existing)
│       ├── rocks/        (find in existing)
│       └── buildings/    (find in existing)
├── textures/             # Keep minimal set
│   └── effects/          # Corruption overlays
└── generated/            # Build.rs output
    └── [timestamp]/      # Versioned generation
```

## IMMEDIATE ACTIONS

### 1. Test Hex Tile Loading (RIGHT NOW)
```rust
// Quick test in game-engine
fn test_hex_tiles(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Load existing hex tile
    let grass_tile: Handle<Scene> = asset_server.load(
        "models/nature/k_nature_block_grass_hexagon.glb#Scene0"
    );
    
    // Spawn it
    commands.spawn(SceneBundle {
        scene: grass_tile,
        ..default()
    });
}
```

### 2. Create Missing Hex Tiles
```python
# hex_corrupted.py - Just modify your hex_grass.py
def build_corrupted_tile():
    # Same as grass but:
    # - Darker colors (0.05, 0.02, 0.08)
    # - Add cracks (subdivide and displace)
    # - Purple emission on cracks
```

### 3. Clean Up Existing Assets
```bash
# Move hex tiles to proper location
mkdir -p assets/models/hex_tiles/grass
mv assets/models/nature/*hexagon.glb assets/models/hex_tiles/grass/

# Delete duplicates (.obj, .fbx, .mtl when we have .glb)
find assets -name "*.obj" -o -name "*.fbx" -o -name "*.mtl" | \
  while read f; do
    if [ -f "${f%.*}.glb" ]; then
      rm "$f"
    fi
  done
```

### 4. Generate Corruption Variants
```python
# Add to each Blender script
def generate_corruption_variant(base_obj, corruption_level):
    """0.0 = normal, 1.0 = fully void"""
    # Darken colors
    # Add noise displacement
    # Add void particles
    # Export as separate GLB
```

## WHY THIS IS PERFECT

### 1. **You Already Have Assets**
- Hex tiles exist
- Blender scripts work
- Just need organization

### 2. **Procedural = Infinite Variants**
- Generate corruption levels
- Create seasonal variants
- Make unique bosses

### 3. **Small File Size**
- Low-poly procedural
- No textures needed
- Vertex colors only

### 4. **Build-Time Generation**
- Version controlled
- Reproducible
- No runtime cost

## THE PAYOFF

With this approach:
- **Week 1**: Get hex tiles working
- **Week 2**: Generate all bosses
- **Week 3**: Create corruption variants
- **Week 4**: Polish and ship

You're SO CLOSE to having this working!

## NEXT STEP

1. **Test loading existing hex GLB** ← Do this NOW
2. **Wire Blender to build.rs**
3. **Generate missing tiles**
4. **Delete 90% of unused assets**

The infinite world + procedural assets = SHIP IN A MONTH!
