# Dragon's Labyrinth - Variant-Based Asset Generation System

## Architecture Revolution: From Level-Banded to Universal Variants

### Current Problem
- Separate TOML files for each level range (01-20, 21-60, etc.)
- Redundant asset definitions with only corruption differences
- Wasteful 1024x1024 resolution for all assets
- Proper names baked into prompts instead of generic archetypes
- No systematic variant generation

### New Variant-Based System

#### Core Concept
- **Universal Base Assets** with multiple variant dimensions
- **Automatic Combinatorial Generation** from variant matrices
- **Dynamic Naming** based on variant combinations
- **Efficient Sprite Sheeting** with Python/Pillow automation
- **Variable Resolutions** based on asset type needs

#### Variant Dimensions

##### 1. **Character Variants**
```toml
[variants]
gender = ["male", "female", "non_binary"]
build = ["thin", "average", "stocky", "muscular"]
age = ["young", "middle_aged", "elder"]
hair = ["bald", "brown", "black", "blonde", "grey", "red", "white"]
clothing = ["tunic", "robes", "leather", "chainmail", "plate"]
corruption = ["clean", "stressed", "traumatized", "broken", "void_touched"]
skin_tone = ["pale", "fair", "olive", "brown", "dark", "grey_undead", "void_black"]
```

##### 2. **Biome Variants**
```toml
[variants]
base_terrain = ["plains", "forest", "mountain", "swamp", "desert", "coast", "snow"]
corruption_level = ["clean", "blight", "hellscape", "social", "void"]
texture = ["grass", "stone", "sand", "mud", "ice", "lava", "void_matter"]
density = ["sparse", "normal", "dense"]
season = ["spring", "summer", "autumn", "winter"]
```

##### 3. **Monster Variants**
```toml
[variants]
base_creature = ["human", "goblin", "wolf", "skeleton", "elemental", "aberration"]
size = ["tiny", "small", "medium", "large", "huge"]
corruption = ["natural", "dragon_touched", "socially_broken", "void_warped"]
equipment = ["unarmed", "basic_weapons", "armor", "magic_items"]
allegiance = ["neutral", "bandit", "cult", "regime", "void_servant"]
```

#### Resolution Optimization
```toml
[resolution_tiers]
ui_elements = "256x256"      # Small UI components
tokens = "512x512"           # Character/monster tokens  
tiles = "1024x1024"          # Biome tiles (need tiling)
features = "768x768"         # Buildings, dungeons
effects = "512x512"          # Particle effects, overlays
```

#### Automatic Sprite Sheet Generation
- Python workflow generates individual variants
- Pillow automatically combines related variants into sprite sheets
- Dynamic validation based on expected dimensions
- Automatic atlas generation with JSON metadata

#### Example Combinatorial Output
From one "shopkeeper" base definition with variants:
- shopkeeper_male_stocky_middle_aged_brown_tunic_clean_olive
- shopkeeper_female_thin_young_blonde_robes_stressed_fair  
- shopkeeper_male_muscular_elder_bald_leather_traumatized_dark
- (30+ variants from single definition)

### Implementation Strategy

1. **Phase 1: Variant Schema Definition**
   - Create universal variant system TOML structure
   - Define variant matrices for each asset category
   - Test combinatorial generation logic

2. **Phase 2: Base Asset Conversion**
   - Convert existing level-banded files to universal variants
   - Remove proper names, use generic archetypes
   - Implement corruption as variant dimension

3. **Phase 3: Python Workflow Enhancement**
   - Extend LangGraph workflow to handle variants
   - Implement sprite sheet generation with Pillow
   - Add resolution optimization logic

4. **Phase 4: Validation & Testing**
   - Test variant generation with sample batches
   - Validate sprite sheet creation and atlas generation
   - Performance testing with optimized resolutions

### Benefits
- **Exponential Asset Coverage** from smaller prompt sets
- **Consistent Variant Quality** across all corruption levels  
- **Memory/Storage Efficiency** with optimized resolutions
- **Modular Expansion** - easy to add new variant dimensions
- **Automated Asset Management** with sprite sheet generation
- **Generic Archetype System** - names assigned via ECS entity data
