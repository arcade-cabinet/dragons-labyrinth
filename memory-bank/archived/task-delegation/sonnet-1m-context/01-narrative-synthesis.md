# Task S1M-001: Complete Narrative Bible Synthesis

## Context Window Advantage

You have access to a 1M token context window, allowing you to read and synthesize ALL narrative documents simultaneously. This is critical for creating a cohesive, interconnected narrative structure.

## Required Reading (ALL documents)

### Primary Documents
- `memory-bank/design_bible.md` - Core narrative vision
- `memory-bank/biomes_reference.md` - World structure
- `memory-bank/companions_reference.md` - Character arcs
- `memory-bank/projectbrief.md` - Game scope
- `memory-bank/productContext.md` - User experience goals

### Technical Context
- `memory-bank/systemPatterns.md` - Horror progression mechanics
- `memory-bank/technical_architecture.md` - How systems interconnect

## Three-Act Structure

### Act 1: Journey to the Labyrinth
- Player discovers the dragon threat
- Companions join with their own agendas
- World still has hope (Dread 0-1)
- Introduction to hex-based exploration
- First encounters with minor corruption

### Act 2: Journey Home
- Labyrinth reveals the truth about the dragon
- Companions begin showing trauma/betrayal
- World visibly degrading (Dread 2-3)
- Economic systems collapsing
- Player choices affect companion loyalty

### Act 3: Journey to the Void
- Final confrontation approaches
- Companions may betray or sacrifice
- World near total corruption (Dread 4)
- The void beckons
- Multiple ending paths based on choices

## Deliverable Structure

```
memory-bank/narrative-direction/
├── overview.md                          # Master narrative document
├── act1-journey-to-labyrinth/
│   ├── scene-breakdowns.md             # Every scene in detail
│   ├── encounter-scripts.md            # Specific encounter dialogue
│   ├── companion-interactions.md       # How each companion behaves
│   └── horror-progression.md           # Dread 0→1 transitions
├── act2-journey-home/
│   ├── scene-breakdowns.md             # Every scene in detail
│   ├── betrayal-mechanics.md           # How betrayals trigger
│   ├── world-degradation.md            # Economic/social collapse
│   └── companion-trauma.md             # Trauma accumulation
└── act3-journey-to-void/
    ├── final-confrontation.md          # Dragon battle variations
    ├── void-mechanics.md                # The void's influence
    ├── ending-variations.md            # All possible endings
    └── narrative-closure.md            # How stories resolve
```

## Scene Breakdown Requirements

For EACH scene, document:

### 1. Scene Metadata
```yaml
scene_id: "act1_scene_03_first_companion"
act: 1
dread_level: 0
location: "village_outskirts"
companions_present: ["Elena"]
```

### 2. Narrative Context
- What just happened
- Player's current understanding
- World state at this moment
- Companion mental states

### 3. Dialogue Trees
```yaml
dialogue:
  - speaker: "Elena"
    emotion: "suspicious"
    text: "You're not from here. Why do you care about our dragon problem?"
    responses:
      - option: "I want to help"
        effect: "trust +1"
        next: "elena_grateful"
      - option: "I need something from the labyrinth"
        effect: "trust -1"
        next: "elena_suspicious"
```

### 4. Horror Progression Triggers
- What increases dread
- Visual changes
- Audio cues
- Companion reactions

### 5. Asset Requirements
```yaml
required_assets:
  models:
    - "village_house_corrupted_stage_1"
    - "elena_model_clean"
  textures:
    - "corruption_overlay_subtle"
  audio:
    - "ambient_unease_level_1"
```

## Companion Arc Documentation

For EACH companion across ALL acts:

### Elena (The Skeptic → The Believer → The Betrayer/Savior)
- **Act 1**: Joins reluctantly, questions everything
- **Act 2**: Sees the truth, becomes fervent
- **Act 3**: Either betrays for "greater good" or sacrifices herself

### Marcus (The Protector → The Broken → The Monster/Hero)
- **Act 1**: Strong, confident, protective
- **Act 2**: Trauma from labyrinth breaks him
- **Act 3**: Either becomes void-touched or finds redemption

### Luna (The Innocent → The Corrupted → The Lost/Redeemed)
- **Act 1**: Young, hopeful, naive
- **Act 2**: Dragon's influence grows on her
- **Act 3**: Either fully corrupted or breaks free

### Theron (The Scholar → The Obsessed → The Mad/Enlightened)
- **Act 1**: Seeks knowledge about the dragon
- **Act 2**: Knowledge drives him to extremes
- **Act 3**: Either insane or transcendent

## Horror Progression Specifics

### Dread Level 0 (Peace)
- Subtle wrongness
- Occasional glitches
- Companions occasionally stare too long
- Example: "The merchant's smile doesn't reach his eyes"

### Dread Level 1 (Unease)
- Clear signs something is wrong
- Shadows move independently
- Companions show stress
- Example: "Elena's hand trembles as she grips her weapon"

### Dread Level 2 (Dread)
- Reality distortions
- Companions show trauma
- World visibly corrupted
- Example: "Marcus hasn't slept in days, muttering about 'the eyes'"

### Dread Level 3 (Terror)
- Constant threat presence
- Companions breaking down
- Apocalyptic signs
- Example: "Luna's tears are black now"

### Dread Level 4 (Horror)
- Full cosmic horror
- Companions may be lost
- World ending
- Example: "Theron's flesh ripples with something underneath"

## Dialogue Writing Guidelines

### Voice Consistency
- Elena: Sharp, questioning, analytical
- Marcus: Direct, protective, increasingly desperate
- Luna: Soft-spoken, poetic, increasingly cryptic
- Theron: Verbose, academic, increasingly fragmented

### Horror Integration
- Early acts: Subtext and implication
- Middle acts: Direct references to wrongness
- Late acts: Full existential horror

### Example Progression
**Act 1**: "The dragon sleeps, but for how long?"
**Act 2**: "I've seen it in my dreams. It's not sleeping. It's waiting."
**Act 3**: "It was never a dragon. We are the dream it's having."

## Critical Requirements

1. **Idempotency**: Same scene_id always produces same base dialogue
2. **Variation**: Support multiple paths through each scene
3. **Asset Alignment**: Every scene lists exact asset requirements
4. **Tool Compatibility**: Format for TOML conversion by background agents
5. **Complete Coverage**: NO gaps in narrative flow

## Output Format

Use Markdown with YAML frontmatter for structured data:

```markdown
---
scene_id: "act2_scene_15_betrayal_reveal"
dread_level: 3
companions: ["Elena", "Marcus"]
location: "corrupted_village"
---

# The Betrayal Reveal

## Context
The party returns to find the village completely corrupted...

## Dialogue
[Structured dialogue trees in YAML]

## Horror Elements
- Visual: Buildings melt like wax
- Audio: Children singing backwards
- Companion: Elena's eyes reflect something that isn't there
```

## Success Metrics

- [ ] Every scene from game start to all endings documented
- [ ] All companion arcs fully detailed with branching paths
- [ ] Horror progression triggers explicitly defined
- [ ] Asset requirements complete for each scene
- [ ] Dialogue maintains voice consistency
- [ ] Background agents can convert to TOML without ambiguity
- [ ] No narrative gaps or contradictions

Remember: You're creating the COMPLETE narrative bible that will drive ALL AI generation. Be exhaustive, be specific, be consistent.
