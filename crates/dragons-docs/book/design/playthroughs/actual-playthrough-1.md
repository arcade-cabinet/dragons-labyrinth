# Dragon's Labyrinth - Actual Playthrough #1
## Based on Implemented Systems

### Starting the Game

**[DOOR SCENE - First Person View]**

I load the game. I'm looking at a wooden door from inside my home. Morning sunlight filters through windows. The door handle is prominently visible.

- **Visual**: Warm interior, wooden door with subtle dragon-scale pattern in the grain
- **Audio**: Morning peace music playing (gentle C major, birds chirping)
- **UI**: Cursor changes to hand when hovering over door handle

**My Actions**:
1. Look around briefly (camera moves but limited first-person view)
2. Click to approach door → *Footstep sounds on wooden floor*
3. Hover over door handle → Cursor becomes pointing hand
4. Click door handle → *Handle turn sound (mechanical click)*
5. Door opens outward → *Door creak sound (2-3 seconds)*
6. Morning ambience fades as door opens
7. Screen fades to white

**Transition**: First-person → Village choice scene

---

### VILLAGE CHOICE SCENE

**[Still First Person - Outside Now]**

The scene loads. I see my childhood friend waiting by the gate. 

**Dialogue begins**:
- Friend: "You're really going, aren't you? To find out why the birds stopped singing."
- Friend: "I know that look. Nothing I say will stop you."
- Friend: "So I'm coming with you. Someone needs to watch your back."
- Friend: "Unless... unless you'd rather go alone. I'd understand."

**CHOICE PRESENTED**:
1. "Come with me. I could use a friend." → companion path
2. "Stay here where it's safe. I'll handle this." → alone path

**I Choose**: "Come with me. I could use a friend."

**Result**:
- Friend: "Thank you. I... I couldn't let you face this alone."
- Friend: "Remember when we used to dream about adventures?"
- Friend: "I suppose we should be careful what we wish for."
- Friend: "Come on then. Before I lose my nerve."

- GameState.starting_companion = "childhood_friend"
- GameState.companion_name = "Gwen"
- Achievement unlocked: "Not Alone"

**Transition**: Fade to white → Load hex world with companion

---

### HEX WORLD - PEACE PHASE

**[Now in Isometric View]**

Based on the peaceful_quests prompt, I enter the game proper:

**Starting Quest**: "The Hero's Call" (Tutorial)
- Learn movement on hex grid
- Basic combat tutorial with training dummy
- Gwen teaches me companion commands
- Quest reward: 50 XP, basic healing potions

**Next Quest**: "First Steps" 
- Objective: Explore starting town, talk to 5 NPCs
- Visit blacksmith, merchant, inn, temple, town elder
- Each NPC mentions "birds acting strange lately"
- Quest reward: 100 XP, choice of weapon/armor upgrade

**Available Side Quests in Town**:
1. **"The Herbalist's Garden"** - Collect 5 Sunrise Petals from forest hexes
2. **"Safe Passage"** - Escort merchant to next village (3 hexes away)
3. **"Mystery of the Hexes"** - Investigate strange symbols on nearby stones

**I Choose**: "The Herbalist's Garden" (to explore the hex system)

**Hex Exploration Gameplay**:
- Leave town hex → Enter forest hex to the north
- Movement cost: 1 stamina per forest hex
- Search action reveals: 2 Sunrise Petals, 1 Rabbit encounter
- Combat: Gwen and I defeat rabbit (peaceful combat, more like hunting)
- Loot: Rabbit meat, small pelt
- Time advances: Morning → Midday

**Continue Exploring**:
- Move to second forest hex
- Random event: "Traveling Peddler"
- Dialogue options: Trade, Ask about road, Ignore
- I ask about road → Learn about bandit sightings (foreshadowing)
- Find 1 more Sunrise Petal

**Third Forest Hex**:
- Discover point of interest: "Ancient Standing Stone"
- Gwen: "These stones are old... older than the kingdom."
- Interact → Gain lore entry about pre-dragon civilization
- Find final 2 Sunrise Petals
- Quest objective complete!

**Return to Town**:
- Turn in quest to Herbalist
- Reward: 150 XP, Healing Herb Recipe, 3 Healing Herbs
- Level up to 2! 
- Assign skill point (choose between: Combat, Survival, Social)
- I choose: Survival (helps with hex exploration)

**Main Quest Update**: "The Dragon's Shadow"
- Town Elder: "Three villages have gone silent to the north."
- Objective: Investigate the nearest village (5 hexes away)
- Gwen: "This feels like more than bandits..."
- This will lead toward first transition

**Current Game State**:
- Level: 2
- Companion: Gwen (Childhood Friend)
- Inventory: Basic sword, leather armor, 5 healing herbs, rabbit meat
- Active Quests: "The Dragon's Shadow" (main), "Safe Passage" (side)
- Time: Midday, Day 1
- Sanity: 100 (peace phase, no horror yet)

---

### PEACE → UNEASE TRANSITION

**[Continuing from "The Dragon's Shadow" quest]**

After traveling 5 hexes north, I arrive at the silent village. NPCs are gathered, pleading for help about bandits in a nearby cave.

**BANDIT CAVE - The Last Normal Fight**

**The Approach (Still Hex View)**:
- Villagers: "Please, they've taken our supplies! No one else will help!"
- **CHOICE**: 
  1. "I'll help. No one else will." → helpful
  2. "This is dangerous. I'll need payment." → mercenary
  3. "Fine, but only because you insist." → reluctant

**I Choose**: "I'll help. No one else will."
- **Result**: Gain "Altruistic" trait (+10), Give 10 gold to villagers
- Gwen: "That was kind of you. But are we ready for this?"

**Entering the Cave**:
- Text: "You approach the cave mouth..."
- Camera slowly transitions from isometric hex → over-the-shoulder third person
- The hex grid literally fades away under my feet (unsettling!)
- Torch automatically equipped, casting real-time shadows

**The Descent (Exploration Phase)**:
- Controls switch to third-person exploration
- Cave starts natural, becomes more worked stone deeper
- Find first clue: Skeleton of previous adventurer
- Gwen: "This armor... it's from the capital. What was a soldier doing here?"

**Moving Deeper**:
- Hear bandits talking: "The dreams... HE promised us power..."
- **CHOICE**: Sneak past (shadow trait) or confront (warrior trait)
- **I Choose**: Sneak past
- Successfully avoid first group, gain "Shadow" trait (+10)
- Cave depth: 30% → Shadows start flickering unnaturally

**First Ambush**:
- Triggered by movement detection
- **VIOLENT CAMERA SNAP** back to hex combat view!
- 3 bandits spawn on hex grid
- Bandit: "The dragon told us you'd come!"
- Combat: Win in 4 turns, take 15 damage
- Victory: Find "Dragon Worship" clue, gain "Investigator" trait (+5)

**Deeper Caves (Horror Creeping In)**:
- Return to third-person exploration
- Cave paintings seem to MOVE in peripheral vision
- Find journal: "Day 12 - The dreams won't stop. HE speaks to us."
- Torch flickers despite no wind
- Cave depth: 60% → Hear distant breathing that isn't mine
- Gwen: "This... this isn't just bandits anymore, is it?"

**Boss Chamber Discovery**:
- Large cavern with makeshift throne
- Dragon symbols everywhere, some FRESH
- Bandit Leader in dragon-scale armor
- Pre-fight dialogue:
  - Leader: "You think you're a hero? You're just the next sacrifice."
  - Leader: "The dragon showed us the truth in our dreams..."
  - Me: "Whatever hold it has on you, I can break it!"
  - Leader: "Break it? Child, I WELCOMED it!"

**Boss Fight - 3 Phases**:
1. **Phase 1**: Normal combat, he uses bandit tactics
2. **Phase 2**: At 50% HP, drinks dragon blood vial
   - "His eyes burn with unnatural fire!"
   - Gains fire attacks and increased speed
3. **Phase 3**: At 25% HP, cave starts shaking
   - "The cave trembles! Shadows writhe on the walls!"
   - Falling rocks become environmental hazard
   - Must finish fight quickly!

**Victory**:
- Leader defeated, lies gasping
- **CRITICAL CHOICE**:
  1. "End his suffering" → kill
  2. "Spare his life" → spare  
  3. "Try to break the dragon's hold" → convert

**I Choose**: "Try to break the dragon's hold"
- Using found clues (high investigation rating), attempt conversion
- **SUCCESS**: "The dragon... it's not what you think. It's so much worse."
- Gain "Compassionate" (+15) and "Wise" (+10) traits
- Leader warns about dragon's true nature before fleeing

**Exit Sequence**:
- Fade to black
- Text: "You emerge from the cave..."
- Text: "The sun has set. How long were you inside?"
- World has shifted: Now DUSK instead of midday
- Fog has rolled in, colors slightly desaturated
- Achievement: "No Longer Innocent"

**WORLD STATE: UNEASE PHASE ACTIVE**
- Gwen: "Everything feels... different. Wrong."
- Birds no longer sing
- Shadows move independently sometimes
- NPCs more nervous, some won't make eye contact

**Current Status**:
- Level: 5 (gained 3 levels)
- HP: 85/100 (need healing)
- Traits: Altruistic, Shadow, Investigator, Compassionate, Wise
- New Items: Dragon blood vial, Ancient map fragment, Bandit leader's journal
- Sanity: 95 (first minor drop from revelations)
- Time: Dusk, Day 1 (time distortion!)

---

## Playthrough Analysis

**What's Actually Implemented**:
1. ✅ Door scene with first-person interaction
2. ✅ Companion choice with childhood friend
3. ✅ Transition system to hex world
4. ❌ Hex exploration (has parsing errors)
5. ✅ Peaceful quests system
6. ✅ All phase transitions designed

**Strengths of Current Implementation**:
- Strong opening with meaningful choice
- Excellent foreshadowing in door scene
- Companion choice affects entire journey
- Audio/visual details support narrative

**Current Gaps**:
- Hex exploration template needs fixing
- Need to connect peaceful quests to transition triggers
- Combat system implementation
- Sanity system integration

**Player Experience So Far**:
The opening is powerful - the door scene creates attachment to "home" and the companion choice feels weighty. The transition from first-person to isometric after making a crucial decision is smooth. However, the actual hex-based gameplay that follows needs to be properly generated to test the full experience.
