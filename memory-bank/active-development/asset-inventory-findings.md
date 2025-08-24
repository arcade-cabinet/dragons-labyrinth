# Asset Inventory & Integration Findings

## Asset Inventory Complete ✅

### Asset Categories Available

**1. Hex Tiles (World Foundation)**
- 6 complete biome types: `forest`, `ghost_town`, `grass`, `labyrinth`, `ruins`, `swamp`
- Both `.glb` and `.gltf` formats for each
- Perfect match for database `hex_tiles.biome_type` field
- **Database integration**: `tile_asset_id` field already exists ✅

**2. Horror Assets (Core Theme)**
- **Horror Characters**: Ghost, skeleton, zombie variants (multiple types)
- **Horror Ships**: Ghost ships (small and large)
- **Horror Demons**: Standalone demon models
- **Human Survivors**: Matt, Lis, Sam, Shaun with weapon variants
- **Animals**: German Shepherd, Pug
- **Environmental**: Street segments, water towers, streetlights
- **Props**: Barrels, chests (normal and special)

**3. Character Models**
- **Base Characters**: Human male/female, various professions
- **Specialized**: Digger, employee, gamer, skater variants
- **Fantasy**: Vampire character model
- **Military**: Soldier variants
- Both Kenney (`k_`) and Quaternius (`q_`) asset sets

**4. Dungeon Architecture**
- **Structural**: Walls (full, half, narrow, with openings)
- **Features**: Stairs, crypt doors, roof pieces
- **Floors**: Basic floors and detailed variants
- **Props**: Chests, coins, barrels, rocks
- Complete dungeon building toolkit

**5. Weapons & Combat**
- **Melee**: Swords (wood/stone/gold/diamond), axes, daggers, spears
- **Medieval**: Shields, lutes (bard weapons)
- **Ranged**: Cannons, ballistas, catapults, rocket launchers
- **Ammunition**: Arrows, bullets, cannonballs, boulders
- **Storage**: Weapon racks

**6. Audio Assets**
- **Combat**: Sword clashing (11 variants), metal impacts, stone hits
- **Environment**: Footsteps on grass (5 variants), flowing rocks
- **UI**: Coin sounds (5 variants), insert coin prompts
- **Doors**: Open/close sounds (multiple variants)
- **Effects**: Force fields, shield effects
- **Voice**: Character selection prompts

**7. Supporting Assets**
- **Fonts**: Kenney input mappings, Rocket typefaces
- **Sprites**: 2D versions of all model categories
- **Textures**: Organized by category matching models
- **Other**: Alternative formats and variations

## Database Integration Analysis

### Existing Asset References ✅
- **Hex Tiles**: `tile_asset_id` and `ambient_audio_id` fields present
- **NPCs**: No explicit asset fields yet (needs addition)
- **Dungeons**: Need to check for asset reference fields
- **Equipment/Weapons**: Need to check for asset reference fields

### Asset-Entity Mapping Strategy

**Perfect Matches Identified:**
1. **Hex biomes** → Hex tile models (6 biomes = 6 assets)
2. **Horror characters** → NPC/Enemy entities (extensive variety)
3. **Dungeon components** → Dungeon room/architecture entities
4. **Weapons** → Equipment/Item entities
5. **Audio effects** → Event-triggered sound system

### Critical Gap: No Asset Manifest System

**Current State**: Assets scattered across directories without proper indexing
**Needed**: Asset server integration with:
- Asset ID registry
- Format standardization (prefer `.glb` for 3D, `.ogg` for audio)
- Metadata extraction (poly counts, texture sizes, etc.)
- Dread-level variants for progressive horror

## Integration Architecture Requirements

### 1. Asset Server Pattern
```rust
pub struct AssetRegistry {
    pub hex_tiles: HashMap<String, Vec<AssetPath>>, // biome -> [variants]
    pub characters: HashMap<String, Vec<AssetPath>>, // race/type -> [variants]
    pub weapons: HashMap<String, Vec<AssetPath>>, // weapon_type -> [materials]
    pub audio: HashMap<String, Vec<AssetPath>>, // event_type -> [variants]
}
```

### 2. Database Model Updates Needed
- Add asset reference fields to ALL entity models
- Support for dread-level asset variants
- Asset metadata caching
- Format preference system

### 3. Build System Evolution
- Generate asset manifests from directory scanning
- Link 70k+ database entities to appropriate assets
- Create asset bundles for distribution
- Optimize loading for performance

## Next Steps Priority

1. **Update Database Models**: Add asset reference fields to all entity types
2. **Create Asset Registry**: Build comprehensive asset indexing system  
3. **Design Asset Server**: Bevy asset server integration for 70k+ entities
4. **Build System Update**: Generate ECS data with asset links
5. **Dread Progression**: Asset variants for horror progression

This inventory confirms we have comprehensive CC0 assets ready for full integration with the 70k+ entity database. The missing piece is the asset registry and linking system.
