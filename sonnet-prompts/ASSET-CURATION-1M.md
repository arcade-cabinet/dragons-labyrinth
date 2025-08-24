# SONNET 4: Asset Curation & Organization (1M Context)

## YOUR MISSION
You have 1.7GB of CC0 assets in `crates/game-content-static/assets/`. They are HORRIBLY organized. You need to:

1. **CURATE** - Identify what's actually useful for Dragon's Labyrinth
2. **ORGANIZE** - Create proper directory structure
3. **DELETE** - Remove irrelevant assets (90% is probably junk)
4. **DOCUMENT** - Create asset manifest with descriptions
5. **MAP** - Connect assets to the 180-progression journey

## THE GAME CONTEXT

**Dragon's Labyrinth**: 180-progression horror RPG where you become what you sought to destroy.

### Visual Progression:
- **Progression 1-20**: Medieval village, forests, basic enemies
- **Progression 20-40**: Towns, dungeons, mounted travel
- **Progression 40-60**: Approaching dragon, corruption visible
- **Progression 60-80**: Post-dragon, world corrupted
- **Progression 80-100**: Fighting back, void emergence
- **Progression 100-120**: Reality breaking
- **Progression 120-140**: Void realm
- **Progression 140-160**: Truth revealed
- **Progression 160-180**: Final transformation

### What We ACTUALLY Need:

#### MODELS (Keep only these categories):
```
assets/models/
├── characters/
│   ├── player/           # Player model + variations
│   ├── companions/       # Elena, Marcus, Quinn
│   ├── villagers/        # Generic NPCs (10-15 variants)
│   ├── guards/           # Military NPCs
│   └── corrupted/        # Void-touched versions
├── enemies/
│   ├── wildlife/         # Wolves, bears, etc
│   ├── bandits/          # Human enemies
│   ├── corrupted/        # Void creatures
│   ├── bosses/           # 9 major bosses
│   └── dragon/           # THE Dragon
├── environment/
│   ├── trees/            # Various tree types
│   ├── rocks/            # Rock formations
│   ├── buildings/        # Houses, shops, temples
│   ├── dungeons/         # Dungeon pieces
│   └── void/             # Corrupted environment
├── props/
│   ├── weapons/          # Swords, bows, etc
│   ├── furniture/        # Tables, chairs, beds
│   ├── containers/       # Chests, barrels
│   └── interactive/      # Doors, levers
└── mounts/
    ├── horses/           # Various breeds
    └── corrupted/        # Void-touched mounts
```

#### TEXTURES (Keep only these):
```
assets/textures/
├── terrain/
│   ├── grass/            # Various grass types
│   ├── stone/            # Stone, cobble, brick
│   ├── dirt/             # Dirt, mud, paths
│   ├── corrupted/        # Void-touched ground
│   └── water/            # Water, ice
├── buildings/
│   ├── wood/             # Wood planks, logs
│   ├── stone/            # Stone walls
│   ├── roofs/            # Thatch, tile, slate
│   └── corrupted/        # Decaying versions
├── ui/
│   ├── buttons/          # UI elements
│   ├── frames/           # Dialog boxes
│   ├── icons/            # Item icons
│   └── corruption/       # UI decay effects
└── effects/
    ├── blood/            # Combat effects
    ├── magic/            # Spell effects
    └── void/             # Corruption effects
```

#### AUDIO (We need to source these):
```
assets/audio/
├── ambient/
│   ├── nature/           # Birds, wind, water
│   ├── village/          # Town sounds
│   ├── dungeon/          # Dripping, echoes
│   └── void/             # Unsettling ambience
├── music/
│   ├── peaceful/         # Early game
│   ├── combat/           # Battle themes
│   ├── boss/             # Boss music
│   └── horror/           # Late game
├── sfx/
│   ├── combat/           # Sword swings, hits
│   ├── footsteps/        # Various surfaces
│   ├── ui/               # Menu sounds
│   └── horror/           # Screams, whispers
└── dialogue/
    ├── companions/       # Elena, Marcus, Quinn
    └── npcs/             # Generic voices
```

## CURATION RULES

### DELETE These Types:
- Sci-fi assets (spaceships, robots, etc)
- Modern assets (cars, guns, etc)
- Cartoon/silly assets
- Duplicate variations (keep best 2-3)
- Low-quality models
- Unrelated themes

### KEEP & CATEGORIZE:
- Medieval/fantasy assets
- Horror/dark assets
- Nature assets
- Anything that can be corrupted
- Versatile props

### RENAME Convention:
```
[category]_[subcategory]_[name]_[variant].ext

Examples:
character_villager_male_01.glb
enemy_wolf_starving.glb
prop_weapon_sword_rusty.glb
texture_terrain_grass_dry.png
```

## DOCUMENTATION FORMAT

Create `assets/ASSET_MANIFEST.md`:

```markdown
# Dragon's Labyrinth Asset Manifest

## Characters (23 models)

### Player
- `character_player_base.glb` - Base player model, customizable
- `character_player_scarred.glb` - With death scars

### Companions
- `character_companion_elena.glb` - Caring healer aesthetic
- `character_companion_marcus.glb` - Glory-seeking warrior
- `character_companion_quinn.glb` - Neutral observer

### Villagers
- `character_villager_male_01.glb` - Young farmer
- `character_villager_female_01.glb` - Market vendor
[etc...]

## Enemies (47 models)

### Wildlife
- `enemy_wolf_normal.glb` - Standard wolf
- `enemy_wolf_dire.glb` - Larger variant
- `enemy_wolf_corrupted.glb` - Void-touched
[etc...]
```

## PRIORITY ASSETS TO FIND/KEEP

### CRITICAL (Must have):
1. **Dragon model** - The main antagonist
2. **Player character** - Customizable base
3. **3 Companion models** - Elena, Marcus, Quinn
4. **Wolf variants** - First enemies
5. **Basic villagers** - NPCs
6. **Trees/rocks** - Environment
7. **Village buildings** - First location

### IMPORTANT (Should have):
1. **9 Boss models** - One per 20 progressions
2. **Mount models** - Horses
3. **Dungeon pieces** - Walls, floors
4. **Weapon models** - Swords, bows
5. **Corruption effects** - Void textures

### NICE TO HAVE:
1. **Weather effects**
2. **Particle effects**
3. **Ambient creatures**
4. **Decoration props**

## DELETION CRITERIA

DELETE if:
- Not medieval/fantasy/horror theme
- Duplicate of better asset
- Too high poly (>50k for props)
- Missing textures
- Wrong scale
- Poor quality

KEEP if:
- Fits theme
- Good quality
- Could be corrupted/modified
- Fills specific need
- Versatile use

## OUTPUT STRUCTURE

After curation:
```
crates/game-content-static/assets/
├── models/          # ~100MB (from 551MB)
├── textures/        # ~50MB (from 354MB)
├── audio/           # ~10MB (from 2MB)
├── fonts/           # ~1MB (from 208KB)
├── ASSET_MANIFEST.md
└── DELETED_ASSETS.md   # List of what was removed and why
```

## SPECIAL CONSIDERATIONS

### Corruption Variants:
For key assets, note if they need corrupted versions:
- Normal → Void-touched → Fully corrupted
- Clean → Decaying → Nightmare

### Progression Mapping:
Note when assets are needed:
- Early (1-60): Medieval, natural
- Mid (60-120): Corrupted, breaking
- Late (120-180): Void, transformed

### Reusability:
Prefer assets that can be:
- Retextured for variants
- Scaled for different uses
- Combined modularly

## YOUR DELIVERABLE

1. **Move commands** to reorganize:
```bash
mv crates/game-content-static/assets/models/medieval/village/*.glb \
   crates/game-content-static/assets/models/environment/buildings/
```

2. **Delete commands** for junk:
```bash
rm -rf crates/game-content-static/assets/models/fantasy/
```

3. **Asset manifest** documenting everything kept

4. **Deletion log** explaining what was removed

5. **Gap analysis** - what assets we still need to generate

Remember: We need QUALITY over QUANTITY. Better to have 50 perfect assets than 500 mediocre ones.

The goal is a CURATED, DOCUMENTED, ORGANIZED asset library ready for the game!
