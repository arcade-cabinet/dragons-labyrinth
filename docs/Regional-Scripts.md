# Dragon's Labyrinth Regional Scripts
## Horror Progression Through Calculated Regions

Based on HBF analysis revealing 4 distinct regions in "The Lands of Vo'il", each region corresponds to specific acts and emotional progression in Dragon's Labyrinth.

## Regional Mapping to Dragon's Labyrinth Acts

### Region 1: Heartlands (Levels 1-20) 
**HBF Region**: "Fearless Wilds" (20+ hex tiles)  
**Emotional Arc**: Peace → Unease  
**Theme**: Bright meadows, forests, welcoming villages with first hints of wrongness

#### Biome Transformations (Corruption Level 0-1)
- **Base**: Lush jungle, healthy forests, clear streams  
- **Early Corruption**: Occasional withered patches, scorched glades  
- **Settlement Types**: Villages (healing), Inns (rest), Farms/Cabins (encounter opportunities)  
- **Dungeon Types**: Caves (bandit hideouts), Temples (blessing opportunities)  
- **Horror Elements**: Subtle wrongness, missing animals, dead birds

#### Key Encounters & Features
- **Bandit Cave** (Strength path): Moral choice about violence consequences
- **Lost Child** (Harmony path): Compassion vs exploitation discovery
- **Forge Preparation**: Collect first sentimental items from childhood memories
- **Companion Introduction**: Elena, Marcus, Quinn with baseline psychology profiles

#### Extraction Goals from HBF Entities
- **Settlements**: Extract village-scale population, peaceful NPCs with minimal trauma
- **Creatures**: Normal beasts (wolves, bears) with optional "Tainted" corruption variants
- **Items**: Mundane equipment with potential sentimental value flagging
- **Factions**: Early militia presence, helpful guards, no cults yet

### Region 2: Borderlands (Levels 21-40)
**HBF Region**: "Vicious Crags" (15+ hex tiles)  
**Emotional Arc**: Unease → Dread  
**Theme**: Transition zones with dragon scars, visible corruption spreading

#### Biome Transformations (Corruption Level 1-2) 
- **Base**: Rocky mountains, dry hills, scorched forests
- **Growing Corruption**: Bands of blight, charred earth, withered trees
- **Settlement Types**: Towns (defensive), damaged Temples, abandoned Shops
- **Dungeon Types**: Caves (corrupted), Temples (testing faith), early Tombs
- **Horror Elements**: Dragon flight scars, cult presence, missing travelers

#### Key Encounters & Features  
- **Fighting Pit** (Strength path): Leadership through sacrifice
- **Crossroads Meeting** (Harmony path): Trust building under pressure
- **Companion Psychology**: First trauma events, stress accumulation starts
- **Forge Preparation**: Collect items from moral choices and companion bonds

#### Extraction Goals from HBF Entities
- **Settlements**: Extract town-scale with defensive postures, early trauma signs
- **Creatures**: "Scorched" variants of beasts, early cultists, bandit gangs
- **Items**: Equipment showing wear, early corruption, ritual components
- **Factions**: Militias organizing, early cult presence, defensive alliances

### Region 3: Dragonlands (Levels 41-60) 
**HBF Region**: "Ragthorn Woods" (5+ hex tiles)  
**Emotional Arc**: Dread → Terror  
**Theme**: Dragon's hellscape, approaching the labyrinth, reality beginning to fray

#### Biome Transformations (Corruption Level 2-3)
- **Base**: Lava fields, dried riverbeds, jagged rock deserts  
- **Heavy Corruption**: Black sand, void cracks forming, geometry distortion
- **Settlement Types**: Strongholds (fortress-cities), war camps, ruined cities
- **Dungeon Types**: Tombs (ancient horrors), corrupted Temples, cave systems
- **Horror Elements**: Environmental hazards, reality distortion, dragon presence

#### Key Encounters & Features
- **Dying Village** (Light path): Limits of compassion, infection mechanics
- **Siege Command** (Strength path): Leadership costs, fortress warfare  
- **Dragon Proximity**: Stalking behavior begins, audio/visual effects
- **Forge Approach**: High Elves or Dark Forge choice point

#### Extraction Goals from HBF Entities
- **Settlements**: Extract fortress-cities, military strongholds, refugee camps
- **Creatures**: "Nightmare" variants, eldritch precursors, cult fanatics  
- **Items**: Corruption-resistant equipment, dark artifacts, forge components
- **Factions**: Syndicates in control, militarized zones, competing cults

### Region 4: Voidlands (Levels 61-180)
**HBF Region**: "Heartseeker Forest" (1+ hex tile)  
**Emotional Arc**: Terror → Despair → Madness → Void  
**Theme**: Post-dragon wasteland, return journey, void corruption, final battles

#### Biome Transformations (Corruption Level 3-4)
- **Base**: Nightmare versions of all previous biomes
- **Maximum Corruption**: Void terrain, impossible architecture, reality breakdown
- **Settlement Types**: Ruined cities, cult strongholds, militarized zones
- **Dungeon Types**: Nightmare temples, void portals, reality fragments  
- **Horror Elements**: Non-Euclidean geometry, companion breakdown, cosmic horror

#### Key Encounters & Features
- **Cleansing Ritual** (Light path): Sacred sacrifice for protection
- **Blood Pact** (Dark path): Trading humanity for power
- **The Forge**: Mythic gear creation with companion sacrifice
- **Void Sealing**: Final choices determining reality's fate

#### Extraction Goals from HBF Entities  
- **Settlements**: Extract ruined cities, cult centers, militarized survivor camps
- **Creatures**: Eldritch horrors, corrupted companions, void entities
- **Items**: Mythic forge reagents, void artifacts, reality anchors
- **Factions**: Cults in power, military remnants, void worshippers

## Database Design Requirements

### Core Tables Aligned with Regional Scripts

#### 1. Biomes Table (Layer Cake Base)
```sql
CREATE TABLE biomes (
    id UUID PRIMARY KEY,
    region_name VARCHAR NOT NULL, -- Fearless Wilds, Vicious Crags, etc.
    biome_type VARCHAR NOT NULL, -- grassland, forest, mountain, lava, void
    corruption_level INTEGER NOT NULL CHECK (corruption_level BETWEEN 0 AND 4),
    
    -- Movement effects for each corruption level
    movement_speed_base FLOAT NOT NULL DEFAULT 1.0,
    movement_corruption_penalty FLOAT NOT NULL DEFAULT 0.0,
    mounted_speed_modifier FLOAT NOT NULL DEFAULT 1.0,
    environmental_damage FLOAT NOT NULL DEFAULT 0.0,
    
    -- Companion psychology effects
    companion_stress_modifier FLOAT NOT NULL DEFAULT 0.0,
    isolation_factor FLOAT NOT NULL DEFAULT 0.0,
    comfort_sources TEXT, -- JSON array of what provides comfort
    
    -- Horror progression
    dread_amplification FLOAT NOT NULL DEFAULT 0.0,
    corruption_spread_rate FLOAT NOT NULL DEFAULT 0.0,
    visual_distortion_level INTEGER DEFAULT 0,
    audio_distortion_level INTEGER DEFAULT 0,
    
    -- Asset integration
    tile_asset_id VARCHAR,
    corruption_variants TEXT, -- JSON array of asset variants by corruption level
    ambient_audio_id VARCHAR
);
```

#### 2. Creatures Table (Horror-Focused, Not D&D-Focused)
```sql
CREATE TABLE creatures (
    id UUID PRIMARY KEY,
    base_name VARCHAR NOT NULL,
    corruption_variant VARCHAR NOT NULL, -- tainted, corrupted, nightmare, unspeakable
    region_origin VARCHAR NOT NULL,
    
    -- Horror progression stats (more important than D&D CR)
    dread_level INTEGER NOT NULL CHECK (dread_level BETWEEN 0 AND 4),
    horror_impact FLOAT NOT NULL, -- How much seeing this affects companions
    corruption_source VARCHAR, -- What corrupted this creature
    
    -- Companion trauma triggers
    triggers_trauma BOOLEAN DEFAULT FALSE,
    trauma_type VARCHAR, -- violence_trauma, corruption_trauma, loss_trauma
    comfort_destroyers TEXT, -- JSON array of what this creature destroys
    
    -- Basic stats (simplified from D&D complexity)
    threat_level INTEGER NOT NULL CHECK (threat_level BETWEEN 1 AND 10),
    health_points INTEGER NOT NULL,
    attack_damage VARCHAR NOT NULL, -- dice notation
    special_abilities TEXT, -- JSON array
    
    -- Asset integration
    model_asset_id VARCHAR,
    corruption_assets TEXT, -- JSON array of corruption variants
    audio_cues TEXT, -- JSON array of associated sounds
    
    -- Raw extraction tracking
    source_entity_id VARCHAR,
    extraction_timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
    raw_content TEXT, -- Original HBF HTML for debugging
    confidence_score FLOAT DEFAULT 0.0
);
```

#### 3. NPCs Table (Psychology-First Design)
```sql
CREATE TABLE npcs (
    id UUID PRIMARY KEY,
    name VARCHAR NOT NULL,
    role VARCHAR NOT NULL, -- tavern_keeper, shop_owner, faction_member
    region VARCHAR NOT NULL,
    settlement_type VARCHAR, -- village, town, city, stronghold
    
    -- Psychology core (more important than D&D stats)
    baseline_trauma INTEGER NOT NULL DEFAULT 0 CHECK (baseline_trauma BETWEEN 0 AND 10),
    current_stress INTEGER NOT NULL DEFAULT 0 CHECK (current_stress BETWEEN 0 AND 10),
    stress_triggers TEXT, -- JSON array of what increases stress
    comfort_sources TEXT, -- JSON array of what provides comfort
    breaking_point INTEGER NOT NULL DEFAULT 8,
    
    -- Companion potential (can they join party?)
    can_be_companion BOOLEAN DEFAULT FALSE,
    companion_type VARCHAR, -- warrior, healer, scholar, rogue
    loyalty_threshold INTEGER DEFAULT 5,
    max_loyalty INTEGER DEFAULT 10,
    
    -- Forge integration (sentimental item sources)
    has_sentimental_items BOOLEAN DEFAULT FALSE,
    sentimental_item_ids TEXT, -- JSON array of item IDs
    emotional_weight FLOAT DEFAULT 0.0, -- How much they matter to player
    
    -- Philosophy alignment tracking
    philosophy_lean VARCHAR, -- strength, harmony, light, dark, neutral
    moral_flexibility INTEGER DEFAULT 5, -- How easily they change alignment
    
    -- Basic stats (minimal compared to psychology)
    level INTEGER DEFAULT 1,
    armor_class INTEGER DEFAULT 10,
    hit_points INTEGER DEFAULT 4,
    
    -- Asset integration
    model_asset_id VARCHAR,
    trauma_state_assets TEXT, -- JSON array of trauma visualization assets
    voice_audio_id VARCHAR,
    
    -- Raw extraction tracking  
    source_entity_id VARCHAR,
    extraction_timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
    raw_content TEXT,
    confidence_score FLOAT DEFAULT 0.0
);
```

#### 4. Items Table (Sentimental-First Design)
```sql
CREATE TABLE items (
    id UUID PRIMARY KEY,
    name VARCHAR NOT NULL,
    item_type VARCHAR NOT NULL, -- weapon, armor, trinket, tool, reagent
    material VARCHAR NOT NULL DEFAULT 'mundane', -- wood, stone, gold, diamond, mythic
    
    -- Sentimental value system (CORE to Dragon's Labyrinth)
    is_sentimental BOOLEAN DEFAULT FALSE,
    emotional_weight FLOAT DEFAULT 0.0 CHECK (emotional_weight BETWEEN 0.0 AND 10.0),
    memory_description TEXT, -- Why this matters emotionally
    forge_reagent_type VARCHAR, -- What this becomes at forge
    forge_path_requirement VARCHAR, -- light, dark, either, neither
    
    -- Companion connections (items tied to specific companions)
    belongs_to_companion VARCHAR, -- companion name/id
    trauma_association VARCHAR, -- What trauma this triggers/soothes
    gifting_impact FLOAT DEFAULT 0.0, -- How giving this affects relationships
    
    -- Horror progression effects
    corruption_resistance INTEGER DEFAULT 0,
    dread_amplification FLOAT DEFAULT 0.0,
    void_interaction VARCHAR, -- how this interacts with void
    
    -- Basic stats (simplified)
    damage_dice VARCHAR,
    armor_bonus INTEGER DEFAULT 0,
    magical_properties TEXT, -- JSON array
    
    -- Asset integration
    model_asset_id VARCHAR,
    material_variants TEXT, -- JSON array of material upgrade assets
    icon_sprite_id VARCHAR,
    audio_equip_id VARCHAR,
    audio_use_ids TEXT, -- JSON array of usage sounds
    
    -- Raw extraction tracking
    source_entity_id VARCHAR,
    extraction_timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
    raw_content TEXT,
    confidence_score FLOAT DEFAULT 0.0
);
```

This approach designs our extraction to serve the actual game vision first!
</thinking>

Perfect! You're absolutely right - we need to first create the regional breakdowns and act scripts based on our actual Dragon's Labyrinth vision, then design database structures that serve THOSE goals. Let me update the docs and create proper breakdowns first.

<write_to_file>
<path>docs/Regional-Scripts.md</path>
<content># Dragon's Labyrinth Regional Scripts
## Horror Progression Through Calculated Regions

Based on HBF analysis revealing 4 distinct regions in "The Lands of Vo'il", each region corresponds to specific acts and emotional progression in Dragon's Labyrinth.

## Regional Mapping to Dragon's Labyrinth Acts

### Region 1: Heartlands (Levels 1-20) 
**HBF Region**: "Fearless Wilds" (20+ hex tiles)  
**Emotional Arc**: Peace → Unease  
**Theme**: Bright meadows, forests, welcoming villages with first hints of wrongness

#### Biome Transformations (Corruption Level 0-1)
- **Base**: Lush jungle, healthy forests, clear streams  
- **Early Corruption**: Occasional withered patches, scorched glades  
- **Settlement Types**: Villages (healing), Inns (rest), Farms/Cabins (encounter opportunities)  
- **Dungeon Types**: Caves (bandit hideouts), Temples (blessing opportunities)  
- **Horror Elements**: Subtle wrongness, missing animals, dead birds

#### Key Encounters & Features
- **Bandit Cave** (Strength path): Moral choice about violence consequences
- **Lost Child** (Harmony path): Compassion vs exploitation discovery
- **Forge Preparation**: Collect first sentimental items from childhood memories
- **Companion Introduction**: Elena, Marcus, Quinn with baseline psychology profiles

#### Extraction Goals from HBF Entities
- **Settlements**: Extract village-scale population, peaceful NPCs with minimal trauma
- **Creatures**: Normal beasts (wolves, bears) with optional "Tainted" corruption variants
- **Items**: Mundane equipment with potential sentimental value flagging
- **Factions**: Early militia presence, helpful guards, no cults yet

### Region 2: Borderlands (Levels 21-40)
**HBF Region**: "Vicious Crags" (15+ hex tiles)  
**Emotional Arc**: Unease → Dread  
**Theme**: Transition zones with dragon scars, visible corruption spreading

#### Biome Transformations (Corruption Level 1-2) 
- **Base**: Rocky mountains, dry hills, scorched forests
- **Growing Corruption**: Bands of blight, charred earth, withered trees
- **Settlement Types**: Towns (defensive), damaged Temples, abandoned Shops
- **Dungeon Types**: Caves (corrupted), Temples (testing faith), early Tombs
- **Horror Elements**: Dragon flight scars, cult presence, missing travelers

#### Key Encounters & Features  
- **Fighting Pit** (Strength path): Leadership through sacrifice
- **Crossroads Meeting** (Harmony path): Trust building under pressure
- **Companion Psychology**: First trauma events, stress accumulation starts
- **Forge Preparation**: Collect items from moral choices and companion bonds

#### Extraction Goals from HBF Entities
- **Settlements**: Extract town-scale with defensive postures, early trauma signs
- **Creatures**: "Scorched" variants of beasts, early cultists, bandit gangs
- **Items**: Equipment showing wear, early corruption, ritual components
- **Factions**: Militias organizing, early cult presence, defensive alliances

### Region 3: Dragonlands (Levels 41-60) 
**HBF Region**: "Ragthorn Woods" (5+ hex tiles)  
**Emotional Arc**: Dread → Terror  
**Theme**: Dragon's hellscape, approaching the labyrinth, reality beginning to fray

#### Biome Transformations (Corruption Level 2-3)
- **Base**: Lava fields, dried riverbeds, jagged rock deserts  
- **Heavy Corruption**: Black sand, void cracks forming, geometry distortion
- **Settlement Types**: Strongholds (fortress-cities), war camps, ruined cities
- **Dungeon Types**: Tombs (ancient horrors), corrupted Temples, cave systems
- **Horror Elements**: Environmental hazards, reality distortion, dragon presence

#### Key Encounters & Features
- **Dying Village** (Light path): Limits of compassion, infection mechanics
- **Siege Command** (Strength path): Leadership costs, fortress warfare  
- **Dragon Proximity**: Stalking behavior begins, audio/visual effects
- **Forge Approach**: High Elves or Dark Forge choice point

#### Extraction Goals from HBF Entities
- **Settlements**: Extract fortress-cities, military strongholds, refugee camps
- **Creatures**: "Nightmare" variants, eldritch precursors, cult fanatics  
- **Items**: Corruption-resistant equipment, dark artifacts, forge components
- **Factions**: Syndicates in control, militarized zones, competing cults

### Region 4: Voidlands (Levels 61-180)
**HBF Region**: "Heartseeker Forest" (1+ hex tile)  
**Emotional Arc**: Terror → Despair → Madness → Void  
**Theme**: Post-dragon wasteland, return journey, void corruption, final battles

#### Biome Transformations (Corruption Level 3-4)
- **Base**: Nightmare versions of all previous biomes
- **Maximum Corruption**: Void terrain, impossible architecture, reality breakdown
- **Settlement Types**: Ruined cities, cult strongholds, militarized zones
- **Dungeon Types**: Nightmare temples, void portals, reality fragments  
- **Horror Elements**: Non-Euclidean geometry, companion breakdown, cosmic horror

#### Key Encounters & Features
- **Cleansing Ritual** (Light path): Sacred sacrifice for protection
- **Blood Pact** (Dark path): Trading humanity for power
- **The Forge**: Mythic gear creation with companion sacrifice
- **Void Sealing**: Final choices determining reality's fate

#### Extraction Goals from HBF Entities  
- **Settlements**: Extract ruined cities, cult centers, militarized survivor camps
- **Creatures**: Eldritch horrors, corrupted companions, void entities
- **Items**: Mythic forge reagents, void artifacts, reality anchors
- **Factions**: Cults in power, military remnants, void worshippers

## Act Structure Integration

### Act 1: Journey TO the Labyrinth (Levels 1-60)
**Regions**: Heartlands → Borderlands → Dragonlands  
**Goal**: Establish identity, gather companions, collect sentimental items
**Psychology Focus**: Building bonds, establishing baseline trauma, early moral choices

### Act 2: Journey FROM the Labyrinth (Levels 61-120) 
**Regions**: Voidlands (return through corrupted versions of previous regions)
**Goal**: Deal with dragon's death consequences, social collapse, void emergence
**Psychology Focus**: Companion breakdown, stress management, trauma therapy

### Act 3: Sealing the Void (Levels 121-180)
**Regions**: Pure Voidlands (reality breaking down)
**Goal**: Use forge-created gear, sacrifice for seal, final moral choices  
**Psychology Focus**: Ultimate trauma tests, companion sacrifice, reality anchoring

## System Requirements for Database Design

### Horror-First Entity Design
1. **Every table includes corruption_level and dread_amplification**
2. **Psychology columns more important than D&D stats**  
3. **Sentimental value tracking for forge system**
4. **Philosophy alignment for Light/Dark paths**
5. **Companion trauma triggers and comfort sources**

### Regional Context Awareness
1. **All entities tagged with source region**
2. **Corruption progression tracked across regions**
3. **Settlement evolution from peaceful→militarized→ruined**
4. **Creature corruption variants by region**
5. **Faction evolution aligned with regional progression**

### Asset Integration Planning
1. **Corruption asset variants for each dread level** 
2. **Trauma visualization assets for companions**
3. **Environmental decay progression assets**
4. **Audio progression from peaceful→terrifying**
5. **Sentimental item visual evolution for forge**

This regional framework will guide our database table design to serve Dragon's Labyrinth's actual horror progression goals instead of generic D&D extraction.
