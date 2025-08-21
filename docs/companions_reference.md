# Dragon's Labyrinth - Companions Reference

## Overview
Companions in Dragon's Labyrinth are not just party members - they are emotional anchors that gradually break under the weight of the journey. Each companion represents a different aspect of humanity's response to inevitable horror, and their individual arcs drive the game's emotional impact.

## Core Companions

### Einar - The Loyal Friend
**Archetype**: Steadfast companion who breaks under pressure
**Starting State**: Confident, protective, jokes to lighten mood
**Arc**: Loyalty → Doubt → Desperation → Complete Breakdown

**Progression by Dread Level**:
- **Peace (0)**: Cheerful banter, protective instincts, leadership qualities
- **Unease (1)**: Jokes become forced, starts checking behind the group
- **Dread (2)**: Questions player's decisions, suggests turning back
- **Terror (3)**: Panic attacks, freezes during encounters, begs to stop
- **Horror (4)**: Complete mental breakdown, may become catatonic or violent

**Key Dialogue Themes**:
- Early: "Don't worry, I've got your back!"
- Mid: "Maybe we should reconsider this path..."
- Late: "I can't... I can't do this anymore. Please, let's go home."

**Trauma Triggers**:
- Witnessing boss deaths (especially if player chooses brutality)
- Companion abandonment (especially Mira leaving)
- False audio cues targeting his protective nature
- Moral choices that compromise his values

### Mira - The Optimist
**Archetype**: Eternal optimist who abandons the party when reality becomes undeniable
**Starting State**: Bright, hopeful, sees good in everything
**Arc**: Hope → Forced Cheer → Realization → Abandonment

**Progression by Dread Level**:
- **Peace (0)**: Genuine enthusiasm, points out beauty, encourages others
- **Unease (1)**: Forced positivity, "Everything will be fine!"
- **Dread (2)**: **ABANDONS PARTY** - Cannot handle the growing darkness
- **Terror/Horror**: Gone (may appear in hallucinations)

**Key Dialogue Themes**:
- Early: "Look at those beautiful flowers! This is going to be wonderful!"
- Mid: "We just need to stay positive. It's not that bad, right?"
- Leaving: "I'm sorry. I can't pretend anymore. This isn't an adventure."

**Abandonment Mechanics**:
- Leaves during transition to Dread stage
- Takes portion of supplies (creating resource pressure)
- May send messages/letters found later in the journey
- Haunts other companions' dialogue ("Mira was right to leave")

### Sorin - The Scholar
**Archetype**: Intellectual who becomes traitor boss if mishandled
**Starting State**: Curious, analytical, seeks to understand everything
**Arc**: Curiosity → Obsession → Understanding → Betrayal OR Wisdom

**Progression by Dread Level**:
- **Peace (0)**: Academic excitement, takes notes, asks questions
- **Unease (1)**: Theories about strange occurrences, research obsession
- **Dread (2)**: Dangerous experiments, talks to unknown entities
- **Terror (3)**: **BRANCHING POINT** - Becomes ally or enemy
- **Horror (4)**: Either final companion or traitor boss fight

**Key Dialogue Themes**:
- Early: "Fascinating! I must document this phenomenon."
- Mid: "I'm beginning to understand the patterns here..."
- Betrayal Path: "You don't understand the true nature of what we're facing."
- Loyalty Path: "I've seen the truth, and I choose to stand with you."

**Branching Mechanics**:
- Loyalty determined by player's moral choices
- Respect for life vs. pursuit of power
- How player treats his research and warnings
- Whether player includes him in important decisions

### Tamara - The Innocent
**Archetype**: Young baker's apprentice representing lost innocence
**Starting State**: Wide-eyed wonder, eager to help, trusts completely
**Arc**: Wonder → Confusion → Trauma → Symbol of What's Lost

**Progression by Dread Level**:
- **Peace (0)**: Excited about adventure, helpful, bakes for the party
- **Unease (1)**: Confused by others' fear, tries to help everyone feel better
- **Dread (2)**: First real fear, starts having nightmares
- **Terror (3)**: Severe trauma, selective mutism, clings to player
- **Horror (4)**: Either broken beyond repair or final symbol of hope

**Key Dialogue Themes**:
- Early: "I made extra bread for everyone! Adventures make you hungry!"
- Mid: "Why is everyone so scared? We're together, aren't we?"
- Late: [Often silent, communicates through actions and expressions]

**Innocence Mechanics**:
- Her trauma level affects other companions' morale
- Player choices to protect or expose her to horror
- Her reactions serve as moral compass for player
- Final state influences available endings

## Companion Interaction Systems

### Trauma Accumulation
Each companion has hidden trauma values that increase based on:
- **Witnessing Violence**: Especially player-caused brutality
- **Environmental Horror**: Exposure to corrupted biomes
- **Loss Events**: Other companions leaving or dying
- **Moral Compromises**: Player choices that violate their values
- **Proximity Events**: Dragon stalking, reality distortion

### Dialogue Evolution
Companion dialogue changes based on:
- **Individual Trauma Level**: Affects tone and content
- **Dread Stage**: Universal response to world corruption
- **Relationship to Player**: Trust, fear, loyalty, or resentment
- **Other Companions**: React to each other's states
- **Recent Events**: Immediate responses to player actions

### Support Mechanics
Companions can support each other:
- **Einar** protects **Tamara** (but his breakdown affects her)
- **Mira** encourages **Sorin** (her leaving devastates his loyalty)
- **Sorin** explains things to **Tamara** (can be helpful or harmful)
- **Tamara** reminds others of their humanity (powerful but fragile)

### Ending Influence
Companion final states determine available endings:
- **All Companions Lost**: Only Acceptance ending available
- **Sorin as Traitor**: Defiance ending unlocked
- **Tamara Protected**: Understanding ending possible
- **Einar Intact**: Affects quality of all endings

## Implementation Notes

### Dialogue Trees
- Branching conversations based on trauma states
- Silent options become more common in higher dread levels
- Companion-to-companion conversations that player observes
- Environmental reactions (comments on biomes, audio cues)

### Animation States
- Posture and movement reflect mental state
- Einar: Confident → Hunched → Shaking
- Mira: Bouncy → Forced → Absent
- Sorin: Observant → Obsessed → Calculating or Protective
- Tamara: Curious → Confused → Withdrawn

### Audio Design
- Voice acting reflects increasing trauma
- Breathing patterns change with stress
- Companion-specific audio cues (Einar's protective alerts, Mira's humming that stops)
- False companion voices in horror stage

### Memory System
- Companions remember player choices
- Reference past events in current dialogue
- Build relationships over time
- Trauma affects memory (distorted recollections)

This companion system ensures that the journey's emotional weight is carried not just by the environment, but by the people who matter most to the player, making every loss deeply personal.