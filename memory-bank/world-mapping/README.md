# World Mapping - Dragon's Labyrinth Internal Planning

## APPROACH: Plan Everything, Then Extract It

**NEW PARADIGM**: Instead of extracting everything from HBF and hoping it fits, we plan YOUR world region-by-region, then use ML to find exactly what we need.

## TILE-BY-TILE ANALYSIS WORKFLOW

### Current Focus: Establishing Starting Region
**Working backwards from fighting pit at E1S57 (Fallen Star Steppe)**
- Fighting arena = perfect for Strength path transition "embrace the costs of command"
- Need to determine level progression from starting area to this point
- Starting area candidates: Javelin Plains (accessible, reasonable terrain)

### Regional Analysis Structure
Each region gets documented with:
- **Level Band**: What player level this region supports (1-20, 21-40, etc)
- **Political Context**: Kingdom borders, settlements, factions
- **Environmental**: Biomes, weather patterns, terrain challenges  
- **Encounters**: CR balance, thematic appropriateness
- **Narrative Role**: How this fits the 3-act political progression
- **Naming Strategy**: Organic renaming to fit our world vision

## TECHNICAL IMPLEMENTATION

### Godot Integration Pattern
Using godot-hexagon-tile-map-layer for layer cake approach:
- `resources/hex_tiles/hex_[coordinate].tres` for each tile
- Layer cake: biome + paths + features
- Weather tables, encounter tables, loot tables all in .tres format
- Build game as we analyze tiles

### Human-in-Loop Analysis
- Click through HBF links for deeper analysis
- Direct narrative interpretation based on player journey stage  
- Organic worldbuilding and naming decisions
- Quest and dialog development as we encounter relevant content

## REGIONS TO ANALYZE

### Priority Order (Working Backwards from Fighting Pit)
1. **Starting Region**: Javelin Plains (bottom center, accessible)
2. **Early Progression**: Work north through regions toward Fallen Star Steppe
3. **Fighting Pit Location**: Fallen Star Steppe (E1S57 - established questline anchor)
4. **Kingdom Centers**: Map political boundaries based on settlements
5. **Corruption Zones**: Hell's Gate Desert, Nightmare Desert (higher level)
6. **Mountain Barriers**: Grey Mist Snowlands, Vicious Crags (mount-required)

### Geographic Constraints from Map
- **Impassable mountains** at top (Fearless Wilds isolated)
- **Natural progression** south to north
- **Central river systems** for navigation
- **Desert barriers** for higher-level content
- **Swamps/plains below mountains** = small villages, not kingdoms

## NAMING VISION

Replace D&D names with:
- **Old Germanic**: Regional names, settlement names
- **Semitic**: Ancient locations, religious sites
- **Asiatic**: Eastern cultural elements
- **Tribal**: Wilderness areas, nomadic groups

**Example Recastings**:
- "Bowel of the Raging Pits" → something with history and dignity
- Generic D&D monsters → creatures that fit our world's ecology
- Political figures that make sense in our kingdom structure

## SUCCESS CRITERIA

Each analyzed region should have:
- ✅ Clear level band assignment
- ✅ Political/narrative context
- ✅ Rebalanced encounters for our progression
- ✅ Organic naming that fits our world
- ✅ Working .tres files for immediate Godot integration
- ✅ Quest hooks and narrative threads identified
