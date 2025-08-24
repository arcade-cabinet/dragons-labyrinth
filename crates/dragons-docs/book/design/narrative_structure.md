# Dragon's Labyrinth - Narrative Bible Structure

## Overview

This document provides the complete structure for the S1M-001 Narrative Synthesis task. It defines the exact format, content requirements, and interconnections needed to create a comprehensive narrative bible that drives ALL AI content generation.

## Three-Act Structure with Horror Progression

### Act 1: Journey to the Labyrinth (Peace → Unease)
**Dread Progression**: 0 → 1
**Theme**: Discovery and Hope Corrupted
**Duration**: ~40% of game experience

#### Core Narrative Arc
- Player opens door on beautiful morning (vision-critical opening)
- Discovers dragon threat, begins quest
- Meets companions with individual motivations
- World appears beautiful but subtle wrongness emerges
- First moral choices with seemingly clear answers
- Ends with labyrinth entrance discovery

#### Key Emotional Beats
1. **Wonder**: "This adventure will be amazing!"
2. **Confidence**: "We can handle this dragon problem"
3. **Camaraderie**: "These companions are wonderful"
4. **First Doubt**: "Something doesn't feel right..."
5. **Unease**: "The shadows are too long..."

### Act 2: Journey Home (Unease → Terror)
**Dread Progression**: 1 → 3
**Theme**: Truth Revealed and Hope Destroyed  
**Duration**: ~40% of game experience

#### Core Narrative Arc
- Labyrinth reveals horrible truths about dragon
- Companions show trauma and begin breaking
- World visibly degrading (economic collapse, social breakdown)
- Player realizes they're not the hero - they're marked prey
- Moral choices become impossible dilemmas
- Ends with full acceptance of horror scope

#### Key Emotional Beats
1. **Revelation**: "The dragon isn't what we thought"
2. **Trauma**: "My companions are suffering"
3. **Despair**: "The world is ending because of me"
4. **Isolation**: "Some companions abandon/betray me"
5. **Terror**: "I understand what's really happening"

### Act 3: Journey to the Void (Terror → Horror)
**Dread Progression**: 3 → 4
**Theme**: Final Confrontation and Understanding
**Duration**: ~20% of game experience

#### Core Narrative Arc
- Return to labyrinth for final confrontation
- Perspective shift to first-person horror
- Dragon actively hunts player through audio cues
- Remaining companions may sacrifice or betray
- Multiple endings based on understanding, not power
- Resolution reflects entire journey's choices

#### Key Emotional Beats
1. **Acceptance**: "I must face what I've awakened"
2. **Stalked**: "It knows I'm here"
3. **Alone**: "Only I can end this"
4. **Understanding**: "I see what it really is"
5. **Resolution**: "This is how it ends"

## Detailed Scene Structure Requirements

### Scene Documentation Template

Each scene must include ALL of the following elements:

```yaml
scene_metadata:
  scene_id: "act{X}_scene_{YY}_{short_name}"
  act: 1-3
  dread_level: 0-4
  duration_minutes: 5-15
  location: "hex_coordinate_or_name"
  companions_present: ["Einar", "Mira", "Sorin", "Tamara"]
  previous_scene: "scene_id"
  next_scenes: ["scene_id_1", "scene_id_2"]  # Branching allowed
  required_completion: true|false
  
narrative_context:
  what_happened_before: "Detailed summary"
  player_understanding: "What player knows/believes"
  world_state: "Economic, social, physical state"
  companion_mental_states:
    Einar: "confident|doubting|breaking|broken"
    Mira: "optimistic|forcing|fled|absent" 
    Sorin: "curious|obsessed|ally|traitor"
    Tamara: "innocent|confused|traumatized|symbolic"
    
location_details:
  setting: "Physical description"
  time_of_day: "dawn|morning|noon|afternoon|evening|night"
  weather: "clear|overcast|stormy|unnatural"
  atmosphere: "peaceful|tense|ominous|horrifying"
  visual_corruption: 0.0-1.0  # How corrupted appears
  audio_environment: "birds|silence|whispers|screams"
  
dialogue_trees:
  # Full branching dialogue with emotional effects
  # See detailed format below
  
horror_progression:
  triggers:
    - trigger: "Shadow moves independently"
      effect: "dread +0.1"
    - trigger: "Companion stares too long"
      effect: "unease +1"
  escalation_events:
    - event: "Bird song stops abruptly"
      companion_reactions: 
        Einar: "Hand moves to sword"
        Mira: "Nervous laugh"
  corruption_manifestations:
    - type: "visual"
      description: "Flowers wilt as player approaches"
    - type: "audio" 
      description: "Distant dragon roar, barely audible"
      
required_assets:
  models:
    - id: "village_hex_peaceful"
      corruption_level: 0.0-1.0
      variants: ["morning", "afternoon", "evening"]
    - id: "einar_model"
      state: "clean|dirty|wounded|broken"
      expressions: ["confident", "worried", "fearful"]
  textures:
    - id: "corruption_overlay_subtle"
      opacity: 0.0-1.0
      blend_mode: "multiply"
  audio:
    - id: "village_ambience_peaceful"
      loop: true
      volume: 0.0-1.0
    - id: "conversation_stinger"
      trigger: "dialogue_start"
  ui_elements:
    - id: "dialogue_box_clean"
      corruption_state: 0.0-1.0  # Later becomes distorted
      
branching_logic:
  conditions:
    - if: "einar_trust > 5"
      then: "act1_scene_05_trusted_path"
    - if: "einar_trust <= 5 AND mira_present"
      then: "act1_scene_05_mira_mediates" 
    - if: "einar_trust <= 5 AND NOT mira_present"
      then: "act1_scene_05_conflict_path"
      
achievement_triggers:
  - achievement: "first_companion_met"
    condition: "scene_complete AND einar_trust > 0"
  - achievement: "shadow_noticed"
    condition: "player_clicked_suspicious_shadow"
```

### Dialogue System Requirements

Each dialogue tree must be formatted for YarnSpinner compatibility:

```yaml
dialogue_structure:
  - node_id: "einar_first_meeting"
    speaker: "Einar"
    emotion: "cautious|friendly|hostile|broken"
    voice_direction: "Confident but with edge of worry"
    text: "You're not from here. What brings you to our troubled village?"
    audio_cues:
      - file: "sword_sharpening_stops.ogg"
        timing: "before_text"
      - file: "leather_armor_creak.ogg"  
        timing: "during_text"
    visual_cues:
      - action: "stops_sharpening_sword"
        timing: "before_text"
      - action: "makes_eye_contact"
        timing: "during_text"
    
    responses:
      - option_text: "I want to help with your dragon problem"
        player_emotion: "determined" 
        effects:
          - "einar_trust += 2"
          - "village_hope += 1"
          - "dread += 0.1"  # Accepting responsibility
        next_node: "einar_hopeful_response"
        
      - option_text: "I need something from the dragon's labyrinth"
        player_emotion: "selfish"
        effects:
          - "einar_trust -= 1"
          - "einar_suspicion += 1"
        next_node: "einar_suspicious_response"
        
      - option_text: "Just passing through"
        player_emotion: "dismissive"
        effects:
          - "einar_disappointment += 1"
          - "missed_opportunity_count += 1"
        next_node: "einar_disappointed_response"
        condition: "act == 1"  # Option disappears later
```

## Companion Arc Documentation 

### Complete Companion Progression Requirements

Each companion needs detailed documentation across ALL acts:

#### Einar - The Loyal Friend → The Broken Protector

**Personality Core**: 
- Protective instinct drives all behavior
- Makes dad jokes to lighten tension
- Takes responsibility for group safety
- Values honor and keeping promises

**Act 1 Progression (Dread 0-1)**:
```yaml
einar_act1:
  starting_state:
    trust: 0
    confidence: 8/10
    trauma: 0/10
    protective_instinct: 10/10
    
  key_interactions:
    - scene: "first_meeting"
      behavior: "Sizes up player as potential threat/ally"
      dialogue_themes: ["Who are you?", "Can you handle yourself?"]
      
    - scene: "first_combat"  
      behavior: "Jumps in front of danger"
      dialogue: "Stay behind me! I've got this!"
      
    - scene: "evening_campfire"
      behavior: "Tells stories to boost morale"
      dialogue: "Back in my guard days..."
      
    - scene: "unease_begins"
      behavior: "Hand moves to sword at wrong times"
      dialogue: "Did you hear that? Probably nothing..."
      
  progression_triggers:
    - event: "companion_injured"
      effect: "guilt +2, over-protective +1"
    - event: "moral_choice_violence" 
      effect: "internal_conflict +1"
    - event: "shadow_anomaly_witnessed"
      effect: "dread_awareness +1, jokes_become_forced +1"
      
  ending_state:
    confidence: 6/10  # Subtle decline
    trauma: 2/10      # Beginning awareness
    dialogue_changes: "Jokes become slightly forced"
```

**Act 2 Progression (Dread 1-3)**:
```yaml
einar_act2:
  major_trauma_events:
    - event: "mira_abandonment" 
      impact: "Questions own judgment, guilt spiral"
      dialogue_change: "Maybe she was smart to leave..."
      
    - event: "first_boss_moral_choice"
      impact: "If player chose brutality: horror at player's nature"
      dialogue_change: "I don't recognize you anymore"
      
    - event: "economic_collapse_witnessed"
      impact: "Feels responsible for not protecting everyone"
      dialogue_change: "I should have seen this coming"
      
  behavioral_changes:
    - stage: "early_act2"
      changes: "Suggests retreat more often, checks behind group"
    - stage: "mid_act2" 
      changes: "Panic attacks during combat, freezes up"
    - stage: "late_act2"
      changes: "Begs player to abandon quest, offers to sacrifice himself"
      
  dialogue_evolution:
    - dread_level: 1
      sample_dialogue: "I'm starting to think we're in over our heads..."
    - dread_level: 2  
      sample_dialogue: "Please, let's just go home. This isn't worth dying for."
    - dread_level: 3
      sample_dialogue: "I can't sleep. I keep seeing... things. Things that aren't there."
```

**Act 3 Progression (Dread 3-4)**:
```yaml
einar_act3:
  branching_paths:
    - path: "complete_breakdown"
      condition: "trauma > 8 AND player_brutal_choices > 3"
      behavior: "Catatonic, unresponsive, may need to be left behind"
      final_dialogue: "I'm sorry... I can't... I just can't..."
      
    - path: "violent_break"
      condition: "trauma > 7 AND protective_instinct_triggered" 
      behavior: "Attacks anything perceived as threat, including player"
      final_dialogue: "I won't let you hurt anyone else!"
      
    - path: "sacrifice_heroic"
      condition: "trust > 8 AND player_protected_tamara"
      behavior: "Willing sacrifice to save others"
      final_dialogue: "This is what I was meant to do. Protect the innocent."
      
    - path: "broken_but_loyal"
      condition: "trauma 5-7 AND trust > 6"
      behavior: "Stays but clearly damaged, needs constant reassurance"
      final_dialogue: "I trust you... even when I can't trust myself."
```

### Mira - The Optimist → The Fled → The Haunting Memory

**Act 1**: Genuine enthusiasm, points out beauty, encourages others
**Act 2**: Forces positivity until breaking point, abandons party at dread level 2
**Act 3**: Appears in hallucinations, found letters, haunts other companions' dialogue

[Similar detailed progression for Mira, Sorin, and Tamara...]

## Horror Progression Mechanics

### Dread Level System Implementation

```yaml
dread_mechanics:
  level_0_peace:
    world_state: "Beautiful, helpful NPCs, clear skies"
    companion_behavior: "Normal interactions, confident"
    player_power: "Standard RPG progression"
    audio: "Birds, wind, pleasant music"
    visual_corruption: 0.0
    
    subtle_wrongness:
      - "Shadows 10% longer than sun angle suggests"
      - "NPCs occasionally stare 2 seconds too long"
      - "Flowers close when player approaches"
      - "Bird song stops for 3 seconds randomly"
      
  level_1_unease:
    world_state: "Still beautiful but obviously wrong"
    companion_behavior: "Forced optimism, checking surroundings"
    player_power: "Feels weaker despite levels"
    audio: "Fewer birds, distant whispers, music goes minor key"
    visual_corruption: 0.1-0.3
    
    obvious_signs:
      - "Shadows move independently"
      - "NPCs speak in hushed tones"  
      - "Shops have fewer items"
      - "Children stop playing outside"
      
  level_2_dread:
    world_state: "Open fear, economic collapse, mass migration"
    companion_behavior: "Some abandon party, others show trauma"
    player_power: "Items feel heavier, attacks less effective"
    audio: "Silence punctuated by distant screams"
    visual_corruption: 0.3-0.6
    
    dramatic_changes:
      - "Half the NPCs have fled"
      - "Shops closed, boarding windows"
      - "Companions suggest turning back"
      - "Player reflection shows something behind them"
```

### Corruption Manifestation System

```yaml
corruption_types:
  environmental:
    - type: "vegetation_death"
      progression: "Healthy → Wilting → Dead → Ash"
      trigger: "player_proximity"
    - type: "sky_darkening"  
      progression: "Blue → Gray → Brown → Black"
      trigger: "dread_level"
    - type: "weather_unnatural"
      progression: "Normal → Wrong_wind → Blood_rain → Reality_tears"
      
  structural:
    - type: "building_decay"
      progression: "Well_maintained → Cracking → Collapsing → Void_holes"
    - type: "road_corruption"
      progression: "Stone_path → Broken → Overgrown → Maze_loops"
      
  audio_distortion:
    - type: "false_sounds"
      progression: "None → Whispers → Fake_companion_voices → Dragon_proximity"
    - type: "silence_zones"
      progression: "Natural_quiet → Unnatural_silence → Sound_absorption → Void_zones"
      
  companion_physical:
    - type: "trauma_manifestation"
      einar: "Hand_shake → Thousand_yard_stare → Catatonic_episodes"
      mira: "Forced_smile → Nervous_laughter → Absence"
      sorin: "Note_taking → Frantic_scribbling → Incomprehensible_symbols"
      tamara: "Wide_eyes → Selective_mutism → Symbolic_presence"
```

## Asset Integration Requirements

### Model Variants System

Every major asset needs dread-level variants:

```yaml
asset_variant_system:
  hex_tiles:
    base_types: ["grass", "forest", "village", "water", "mountain", "desert", "swamp"]
    
    variants_per_type:
      - dread_0: "Beautiful, vibrant colors"
      - dread_1: "Subtle wrongness, longer shadows" 
      - dread_2: "Obvious decay, color desaturation"
      - dread_3: "Heavy corruption, reality distortion"
      - dread_4: "Nightmare landscape, impossible geometry"
      
    example_grass_progression:
      - dread_0: "grass_hex_peaceful.glb"
      - dread_1: "grass_hex_uneasy.glb"    # Some brown patches
      - dread_2: "grass_hex_dying.glb"      # Half dead
      - dread_3: "grass_hex_corrupted.glb"  # Twisted, wrong colors
      - dread_4: "grass_hex_void.glb"       # Barely recognizable
      
  companion_models:
    einar_progression:
      - state_confident: "Clean armor, upright posture"
      - state_doubting: "Armor scratched, slight hunch"
      - state_breaking: "Armor damaged, shaking hands"
      - state_broken: "Armor ruined, collapsed posture"
      
  ui_elements:
    dialogue_box_corruption:
      - dread_0: "Clean medieval parchment"
      - dread_1: "Slightly torn edges"
      - dread_2: "Water stains, hard to read"
      - dread_3: "Blood stains, letters shift"
      - dread_4: "Barely legible, reality glitches"
```

### Audio Progression Requirements

```yaml
audio_system:
  ambient_layers:
    peaceful: 
      - "birds_chirping.ogg" (volume: 0.8)
      - "wind_gentle.ogg" (volume: 0.3)
      - "village_life.ogg" (volume: 0.5)
      
    unease:
      - "birds_chirping.ogg" (volume: 0.4)  # Reduced
      - "wind_wrong_direction.ogg" (volume: 0.3)
      - "village_life.ogg" (volume: 0.2)  # Reduced
      - "whispers_distant.ogg" (volume: 0.1)  # Added
      
    dread:
      - "wind_howling.ogg" (volume: 0.6)
      - "whispers_close.ogg" (volume: 0.3)
      - "dragon_breath_distant.ogg" (volume: 0.1)
      
  companion_voice_evolution:
    einar_voice_progression:
      - confident: "Strong, clear, warm tones"
      - doubting: "Slightly quieter, occasional tremor"
      - breaking: "Shaky, frequent pauses, whispers"
      - broken: "Monotone or silent, occasional whimpers"
      
  proximity_horror_system:
    dragon_stalking_audio:
      - distance_far: "Barely audible breath"
      - distance_medium: "Clear breathing, footsteps"
      - distance_close: "Heavy breathing behind player"
      - distance_touching: "Breath on neck, reality tears"
```

## Integration with Expanded Vision Features

### Sentimental Items Integration

```yaml
sentimental_items_narrative:
  collection_scenes:
    - item: "eagle_feather"
      scene: "act1_scene_07_mountain_peak"
      context: "Einar points out majestic eagle, feather falls"
      player_reaction: "Seems meaningful, can't discard"
      
    - item: "crystallized_tear"  
      scene: "act2_scene_23_mira_departure"
      context: "Mira cries, tear crystallizes from dragon magic"
      player_reaction: "Physical memory of loss"
      
  revelation_scene:
    scene: "act3_scene_45_forge_approach"
    context: "Player realizes all 'random' items were reagents"
    dialogue: "The feather... the tear... they all connect..."
    emotional_impact: "Everything had meaning, I was being prepared"
    
  forge_integration:
    light_forge: "Items willingly transform, companions blessed"
    dark_forge: "Items consumed, companions sacrificed"
    narrative_weight: "Choice reflects entire journey's values"
```

### Dual Forge System Integration

```yaml
forge_narrative_buildup:
  act1_hints:
    - scene: "ancient_smith_mention"
      dialogue: "They say the old smith made weapons that could hurt dragons..."
    - scene: "mysterious_reagent_discovery"
      dialogue: "This metal feels warm... almost alive..."
      
  act2_revelation:
    - scene: "forge_lore_discovery"
      context: "Learn about light vs dark forge paths"
      choice_preview: "Power through sacrifice vs worthiness through love"
      
  act3_trials:
    trial_types:
      - navigation_trial: "Tests hex movement mastery"
      - combat_trial: "Tests all fighting skills learned"  
      - wisdom_trial: "Tests understanding of journey"
      - sacrifice_trial: "Tests commitment to chosen path"
      
    narrative_integration:
      - forge_master_character: "Judge of worthiness"
      - companion_involvement: "Their condition affects trial difficulty"
      - moral_weight: "Final test before dragon shows true nature"
```

## Documentation Deliverables Summary

### Primary Documents (Required for TOML Generation)

1. **overview.md** - Master narrative document with complete journey arc
2. **act1-journey-to-labyrinth/** - 15-20 scenes, companion introductions, world establishment
3. **act2-journey-home/** - 20-25 scenes, trauma development, world degradation
4. **act3-journey-to-void/** - 10-15 scenes, final confrontation, multiple endings

### Supporting Documents

4. **companion-arcs/** - Complete psychological progression for all 4 companions
5. **horror-progression/** - Detailed dread level mechanics and manifestations  
6. **asset-requirements/** - Complete model, texture, audio specifications
7. **dialogue-reference/** - Voice patterns, emotional ranges, character consistency
8. **world-state-progression/** - How world changes with dread levels
9. **ending-variations/** - All possible conclusions and their requirements

### Integration Documents

10. **sentimental-items-guide.md** - Complete item list and narrative integration
11. **forge-system-narrative.md** - Moral choice buildup and trial design
12. **companion-personal-quests.md** - Individual relationship development

## Success Criteria

The narrative bible is complete when:

- [ ] Every scene from opening door to final endings is documented
- [ ] All 4 companions have complete psychological progression arcs
- [ ] Horror progression mechanics are precisely defined for all dread levels
- [ ] Asset requirements are specific enough for AI generation
- [ ] Dialogue maintains consistent character voices throughout
- [ ] All branching paths and conditions are clearly specified
- [ ] Background agents can generate TOML rules without ambiguity
- [ ] Expanded vision features (forge, sentimental items) are integrated
- [ ] No narrative gaps or contradictions exist between acts
- [ ] The complete emotional journey from peace to horror is mapped

This narrative bible will serve as the definitive source for ALL content generation, ensuring that every AI-generated asset, dialogue line, and game mechanic serves the horror-first vision and creates the intended emotional journey for players.
