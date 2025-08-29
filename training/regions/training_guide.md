# Regions Training Guide - ML Extraction Reference

## CATEGORY: REGIONS (27 total discovered)

### ML EXTRACTION GOAL
Train ML to recognize and extract region-specific content from HBF entities. Focus on environmental descriptions, political context, level progression hints.

### REGION DATA STRUCTURE (What ML Should Extract)

```json
{
  "region_name": "Javelin Plains", 
  "hex_tiles": ["W6S49", "W5S50", "W6S51", "W5S52", "W5S51"],
  "level_band_hint": "1-8 (starting region)",
  "political_context": "Independent villages, not major kingdom",
  "environmental_theme": "Grasslands with rivers, some hills",
  "corruption_level": 0,
  "key_features": ["Kaelia's Castle", "fighting arena", "caravan camps"],
  "faction_presence": ["The Defiled Wolves"],
  "settlements": ["Kaelia's barony"],
  "weather_pattern": "temperate_grassland",
  "terrain_challenges": "minimal - good starting area"
}
```

### REGION CLASSIFICATION PATTERNS

#### Geographic Naming Patterns (What to Look For)
- **Plains/Fields**: Javelin Plains, Blood Blade Fields, Bonecrusher Plains
- **Forests/Woods**: Heartseeker Forest, Ragthorn Woods, Thunderwave Woodlands  
- **Mountains/Crags**: Vicious Crags, Goldseeker's Cliffs
- **Desert/Wasteland**: Hell's Gate Desert, Nightmare Desert, Darkfall Dunes
- **Wetlands/Swamps**: Javelin Wetlands, Moonwatcher Wetlands
- **Wilderness/Wild**: Fearless Wilds, Goblinchaser Wilderness, Holloweye Wilderness

#### Level Progression Hints (CR Distribution Analysis)
**Starting Regions** (Level 1-8): 
- Mostly CR 1/8 to CR 2 encounters
- Wild animals (horses, toads, goats) 
- Low-level NPCs (levels 1-5)
- Basic treasure (under 500 gp)

**Mid-Level Regions** (Level 9-20):
- CR 3-6 encounters  
- Political NPCs (levels 6-10)
- Faction conflicts emerging
- Moderate treasure (500-2000 gp)

**High-Level Regions** (Level 20+):
- CR 7+ encounters
- Major political figures (levels 10+)
- Serious corruption indicators
- High treasure (2000+ gp)

### ENVIRONMENTAL PATTERNS

#### Peaceful Regions (Corruption Level 0)
- **Descriptors**: "sun-drenched", "peaceful", "meadows", "timeless"
- **Features**: Rivers, grasslands, gentle hills
- **NPCs**: Farmers, merchants, peaceful travelers
- **Weather**: Standard temperate patterns

#### Transitional Regions (Corruption Level 1)  
- **Descriptors**: "abandoned", "fallen", "cracked", "eerie sounds"
- **Features**: Ruins, broken structures, mysterious elements
- **NPCs**: Refugees, guards, concerned citizens  
- **Weather**: More storms, unusual patterns

#### Corrupted Regions (Corruption Level 2+)
- **Descriptors**: "void", "nightmare", "hell's gate", "cursed"
- **Features**: Dark rituals, corruption visible, dangerous terrain
- **NPCs**: Cultists, corrupted creatures, desperate survivors
- **Weather**: Supernatural weather, blizzards, eternal storms

### POLITICAL CONTEXT PATTERNS

#### Independent Regions
- Small settlements, no major kingdoms
- Local lords developing baronies (like Kaelia)  
- Faction conflicts (The Defiled Wolves vs others)
- Player can influence political development

#### Kingdom Border Regions
- Evidence of larger political structures
- Trade routes, established settlements
- Military presence, organized defenses
- Complex faction relationships

#### Wilderness Regions
- No permanent settlements
- Nomadic groups, temporary camps
- Environmental challenges dominate
- Limited political complexity

### JSON/HTML STRUCTURE PATTERNS

#### Region Overview Page
```html
<div hidden id="doc-title"> [Region Name] in The Lands of Vo'il </div>
<p>Regional description with environmental themes</p>
<h5>Notable Locations</h5>
<ul>
  <li>Location Name 1</li>
  <li>Location Name 2</li>
</ul>
```

#### Hex Tile in Region
```html
