# Task S1M-002: Original Vision Expansion Review

## Context

The Dragon's Labyrinth was originally conceived with a much grander scope that was deliberately scaled back for achievability. Now that we have a robust architecture and planning framework, it's time to reconsider what features from the original vision could be reintegrated.

## Required Reading

### Primary Vision Documents
All files in `memory-bank/larger-vision/`:
- `forge-system-design.md` - Dual forge crafting system
- `narrative-orchestration-architecture.md` - Complex narrative systems
- `projectDesign.md` - Original full design scope
- `audio-generation-architecture.md` - Dynamic audio systems
- `phase-transitions-complete.md` - Transition mechanics
- Design subdirectories for detailed examples

### Current Implementation Docs
For comparison with what we currently have:
- `memory-bank/design_bible.md` - Current scaled version
- `memory-bank/systemPatterns.md` - Current systems
- `memory-bank/task-delegation/` - Current planned work

## Key Systems to Investigate

### 1. Dual Forge System
The original vision included TWO forges:
- **Dragon's Forge**: Corruption-based crafting
- **Village Forge**: Traditional/pure crafting
- Items could be "purified" or "corrupted" between forges
- This created moral choices around power vs purity

### 2. Sentimental Items System
Originally planned features:
- Items gain "memories" from use
- Emotional attachment affects companion reactions
- Items could be sacrificed for narrative impact
- Personal history with items affects endings

### 3. Dynamic Economy
More complex than current static economy:
- Supply chains that could be disrupted
- Merchant relationships and reputation
- Economic warfare as dragon influence spreads
- Trade routes that shift with corruption

### 4. Companion Depth Systems
Beyond current trauma mechanics:
- Personal quests for each companion
- Romantic/platonic relationship trees
- Companion-specific abilities that evolve
- Inter-companion relationships and conflicts

### 5. Environmental Storytelling
Rich environmental narrative:
- Discoverable lore fragments
- Environmental puzzles
- Hidden areas with backstory
- Time-based world changes

## Deliverables

### 1. Feature Feasibility Analysis
```markdown
# Feature Feasibility Analysis

## Dual Forge System
### Original Vision
[Detailed description from larger-vision docs]

### Implementation Complexity
- Technical requirements
- AI generation needs
- Asset requirements

### Integration Path
- How to add to current architecture
- Dependencies on other systems
- Phase of implementation

### Value Assessment
- Narrative impact: [High/Medium/Low]
- Gameplay depth: [High/Medium/Low]  
- Development cost: [High/Medium/Low]
- Recommendation: [Include/Defer/Cut]
```

### 2. Expanded Systems Document
Create: `memory-bank/expanded-vision/systems-to-integrate.md`
- Prioritized list of systems to reintegrate
- Technical requirements for each
- Narrative justification
- Asset/generation needs

### 3. Updated Task Documents
For systems we decide to include:
- Update existing task documents
- Create new task documents as needed
- Add to appropriate agent queues

### 4. Rule Generation Updates
For Cursor background agents:
- Additional TOML templates needed
- New generation categories
- Expanded validation rules

## Analysis Framework

### For Each Cut Feature:
1. **Why was it cut?**
   - Technical complexity
   - Scope creep
   - Godot-specific limitations
   - Time constraints

2. **What value does it add?**
   - Narrative depth
   - Player agency
   - Replayability
   - Emotional engagement

3. **Can we implement it now?**
   - Fits with Bevy architecture
   - AI generation capable
   - Asset pipeline supports
   - Database schema accommodates

4. **Should we implement it?**
   - Core to vision vs nice-to-have
   - Implementation cost vs value
   - Impact on other systems
   - Player experience improvement

## Priority Systems to Review

### MUST REVIEW (Core Vision):
- Dual Forge System
- Sentimental Items
- Companion Personal Quests
- Economic Collapse Mechanics

### SHOULD REVIEW (Depth):
- Environmental Storytelling
- Hidden Lore System
- Time-based World Events
- Inter-companion Dynamics

### COULD REVIEW (Polish):
- Achievement System
- New Game+ Mechanics
- Alternate Starting Scenarios
- Seasonal Events

## Integration Considerations

### With Current Architecture:
- How does it fit with hex-based world?
- Database schema extensions needed?
- New AI agents required?
- Asset generation implications?

### With Horror Progression:
- How do features respond to dread levels?
- Do they enhance or dilute horror?
- Companion trauma integration?
- World corruption synergy?

## Output Format

### Main Document Structure:
```markdown
# Dragon's Labyrinth - Vision Expansion Analysis

## Executive Summary
- Features worth reintegrating
- Features to remain cut
- New implementation opportunities

## Dual Forge System
### Original Vision
[Detailed extraction from larger-vision]
### Adaptation for Current Architecture
### Implementation Plan
### Required Tasks

## Sentimental Items System
[Same structure]

## [Continue for each system]

## Task Integration Plan
- Phase 1: Foundation systems
- Phase 2: Depth systems  
- Phase 3: Polish systems

## Updated Scope Assessment
- Core game: [X hours]
- With expansions: [Y hours]
- Resource requirements
```

## Success Criteria

- [ ] All larger-vision documents thoroughly reviewed
- [ ] Clear feasibility assessment for each cut feature
- [ ] Prioritized integration plan created
- [ ] Task documents updated/created for approved features
- [ ] No Godot-specific implementation details carried over
- [ ] Focus remains on enhancing horror narrative
- [ ] Maintains achievable scope while adding depth

## Important Notes

1. **Ignore Implementation Details**: Focus on the VISION and GOALS, not how it was coded in Godot
2. **Maintain Horror Focus**: Every feature should enhance the horror experience
3. **Consider Scope Creep**: Be realistic about what adds value vs complexity
4. **Think Systemically**: How do features interact and support each other
5. **Preserve Achievability**: We scaled back for a reason - don't lose sight of that

Remember: The goal is to identify which "sacrificed" features actually deserve resurrection now that we have better architecture and planning.
