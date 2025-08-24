# The Forge System - Dragon's Labyrinth

## Overview
The Forge represents the ultimate test before facing the dragon - a culmination of every skill learned, every item collected, and every choice made throughout the journey. It transforms seemingly random "sentimental items" into the key to forging mythic-tier dragonslayer gear.

## Core Design Philosophy

### The Sentimental Item System
Throughout the game, players unknowingly collect reagents for the ultimate forge:
- Items marked as "sentimental" cannot be discarded
- Seem arbitrary: eagle feathers, golden scales, crystallized tears
- Only at the forge does their true purpose reveal itself
- Creates an "aha!" moment of realization

### Why This Works
1. **Mechanical Payoff**: Every system mastered has purpose in trials
2. **Narrative Payoff**: Journey's collection gains meaning
3. **Emotional Payoff**: Final moral choice before dragon
4. **Player Agency**: Multiple paths to power
5. **Skill Gate**: Only masters can achieve mythic tier

## The Dual Forge System

### The Forge of High Elves (Light Path)
- **Theme**: Worthiness through love and mastery
- **Location**: Hidden elven citadel in crystalline peaks
- **Access**: Only appears to those who've shown consistent compassion
- **Test**: Companions' loyalty determines blessing strength
- **Sacrifice**: Essence willingly given, not life taken

### The Cursed Forge (Dark Path)
- **Theme**: Power through blood and domination  
- **Location**: Volcanic hellscape, forge of the first dragonslayer
- **Access**: Revealed through displays of ruthless power
- **Test**: How much blood equals mythic power?
- **Sacrifice**: Life taken, loyalty converted to blood value

## Trial Design - Testing Everything

### Universal Trial Elements
Both forge paths test mastery of ALL game systems:

1. **Hex Navigation Under Pressure**
   - Lava fields requiring mount speed/flight
   - Timed segments with environmental hazards
   - Resource nodes that test risk/reward

2. **Combat Mastery**
   - Solo combat in first-person segments
   - Party coordination for group challenges
   - Mounted combat across dangerous terrain

3. **System Integration**
   - Crafting temporary items from environment
   - Managing party morale/health over extended trial
   - Using traits/achievements for advantages

4. **Puzzle Solving**
   - Light: Elven ward mechanisms
   - Dark: Blood channel configurations
   - Both require understanding of game's logic

### Scaling Difficulty
- **Solo Attempt**: Near-impossible, grants Ascended/Demonic trait
- **With Companions**: Emotional bonds provide advantages
- **With Mercenaries**: Numbers help but lack loyalty
- **Mixed Party**: Most flexible but complex to manage

## Forge Mechanics

### Sentimental Item Categories

#### Journey Items (Collected Throughout Game)
- **Natural**: Eagle feather, golden scale, ancient bark
- **Mystical**: Dragon whisper, void ore, star metal
- **Emotional**: Crystallized tears, heart of hero, final words
- **Corrupted**: Blood of willing, shadow essence, fear incarnate

#### Forge Enhancement Matrix
Each item enhances specific aspects:
```
Eagle Feather → +Movement/Flight
Golden Scale → +Divine Protection  
Void Ore → +Void Resistance
Dragon Whisper → Dragon Language Understanding
Crystallized Tears → Empathy-based abilities
Heart of Hero → Courage/Morale buffs
Star Metal → +Damage vs Ancient Beings
Blood of Willing → Life Drain abilities
```

### Gear Tier System

#### Standard Progression (Throughout Game)
- **Common** (White): Basic starting gear
- **Uncommon** (Green): First upgrades
- **Rare** (Blue): Mid-game equipment
- **Epic** (Purple): Late-game gear
- **Legendary** (Orange): Pre-forge maximum

#### Mythic Tier (Red/Gold)
- Only achievable through forge
- Requires complete sentimental collection
- Requires successful trial completion
- Requires appropriate sacrifice

### Sacrifice Mechanics

#### Light Path - Test of Love
- Companion offers essence (not death)
- Bond strength = blessing power
- Multiple companions can contribute
- Creates "Blessed" prefix abilities:
  - Healing Aura
  - Void Immunity  
  - Resurrection Chance
  - Morale Inspiration

#### Dark Path - Test of Blood
- Actual death required
- Loyalty converts to blood value
- May need multiple deaths for mythic
- Creates "Cursed" prefix abilities:
  - Life Drain
  - Fear Aura
  - Raise Thralls
  - Blood Magic

#### The Final Companion
- Offered just before forge if player has none
- Old wise warrior who understands the stakes
- Provides one last chance for sacrifice option
- Their dialogue changes based on your path

## Technical Implementation

### Sentimental Item Tracking
```gdscript
extends Resource
class_name SentimentalItem

@export var id: String
@export var display_name: String  
@export var description: String
@export var icon: Texture2D
@export var forge_aspects: Dictionary = {
    "power_type": "",
    "power_value": 0,
    "special_ability": ""
}
@export var cannot_discard: bool = true
@export var collection_phase: String # "peace", "unease", etc
```

### Forge Result Calculation
```gdscript
func calculate_forge_result(items: Array[SentimentalItem], sacrifice_made: bool, trial_score: float) -> ForgeResult:
    var result = ForgeResult.new()
    
    # Base tier from trial performance
    if trial_score >= 0.95:
        result.base_tier = "legendary"
    elif trial_score >= 0.70:
        result.base_tier = "epic"
    else:
        result.base_tier = "rare"
    
    # Upgrade to mythic if conditions met
    if sacrifice_made and items.size() >= REQUIRED_ITEMS:
        result.final_tier = "mythic"
        result.prefix = _get_path_prefix() # "blessed" or "cursed"
    
    # Calculate abilities from items
    for item in items:
        result.abilities.append(_convert_item_to_ability(item))
    
    return result
```

### Trial Orchestration
```gdscript
class_name ForgeTrialManager

signal trial_phase_complete(phase: String, score: float)
signal trial_failed(reason: String)
signal trial_complete(total_score: float)

var phases = [
    "hex_navigation",
    "combat_gauntlet",
    "puzzle_chamber",
    "final_approach"
]

func start_trial(forge_type: String, party_composition: Dictionary):
    _setup_trial_parameters(forge_type)
    _scale_difficulty_to_party(party_composition)
    _begin_phase(0)
```

## Narrative Integration

### Pre-Forge Build Up
- NPCs mention legendary forges in passing
- Ancient texts hint at reagent purposes
- Merchants puzzled by sentimental items
- Companions comment on growing collection

### The Reveal Moment
- Arrival at forge triggers realization
- UI highlights all sentimental items
- Forge master explains their purpose
- Player sees what they've been building toward

### Post-Forge Consequences

#### With Mythic Gear
- World reacts to your presence
- Dragon acknowledges you as equal
- Unique dialogue options unlocked
- Ending paths expanded

#### Without Mythic Gear  
- Struggle more in dragon encounter
- Limited ending options
- Dragon pities your ignorance
- Cannot access certain void-sealing methods

## Balance Considerations

### Making Solo Possible but Painful
- Environmental aids for solo players
- Longer time limits
- More checkpoints
- But no companion buffs/assistance

### Preventing Exploitation
- Cannot farm sentimental items
- Each is unique to specific events
- Missing items = missing potential
- No trading between players

### Meaningful Choice
- Light forge easier trials, harder sacrifice
- Dark forge harder trials, easier sacrifice  
- Both paths viable but different
- Neither is "correct" choice

## Connection to Greater Systems

### Trait System Integration
- Existing traits affect trial options
- New traits earned (Ascended/Demonic)
- Traits influence forge dialogue
- Mythic abilities scale with traits

### Achievement Integration  
- "Forged in Blood/Love" achievements
- "Solo Forge" legendary achievement
- "Complete Collection" for all items
- Hidden achievements for specific combinations

### Ending Variations
- Mythic gear unlocks unique endings
- Blessed gear → Unity ending possible
- Cursed gear → Dominion ending possible
- No mythic → Standard endings only

This forge system creates the perfect crescendo before the dragon encounter, rewarding long-term play while testing everything learned. It's not just a gear check - it's a final examination of who you've become.
