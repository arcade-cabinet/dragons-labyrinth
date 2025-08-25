# Second Chances System - Dragon's Labyrinth

## Core Philosophy
Dragon's Labyrinth should never permanently lock players out of cool features or abilities. The journey is hard enough without permanent punishment for failure.

## Key Design Decisions

### Forge Mythic Gear System
1. **First Attempt (Pre-Dragon)**:
   - Success: Mythic gear + Sanctuary Presence/Nightmare Transcendence ability
   - Failure: Legendary gear (still powerful, just not mythic tier)
   - No one leaves empty-handed

2. **Second Chance (Post-Dragon)**:
   - On return journey to seal the void
   - Another opportunity at the forge (easier requirements)
   - If you already have the ability: Gain eternal companion instead
   - "Aethon, Last of the Forge Guards" - undying loyalty companion

### Mythic Abilities
- **Holy Path**: Sanctuary Presence
  - Can earn at Forge of High Elves (pre-dragon)
  - OR earn after defeating dragon (reward for completing without it)
  - Second chance on return journey
  
- **Dark Path**: Nightmare Transcendence  
  - Can earn at Dark Forge (pre-dragon)
  - OR earn after defeating dragon (reward for completing without it)
  - Second chance on return journey

### Design Rationale
- Players who rush and fail aren't locked out forever
- Defeating the dragon without mythic abilities is rewarded
- Return journey offers redemption and second chances
- Those who succeeded first time get different rewards (eternal companion)
- Creates interesting risk/reward decisions without permanent consequences

### Implementation Notes
- Forge tracks completion state across versions (TO_LABYRINTH, FROM_LABYRINTH, SEALING_VOID)
- World state tracks whether player has mythic ability
- Different dialogue and requirements for second attempt
- Eternal companion has unique abilities and shares mythic power

## Player Experience Flow
1. First forge attempt → Success or Legendary fallback
2. Dragon fight → With or without mythic ability
3. Victory → Grants ability if not already possessed
4. Return journey → Second forge chance or eternal companion
5. Final battle → Fully equipped regardless of earlier choices

This ensures every player can experience the full power fantasy by game's end, while still maintaining meaningful choices and consequences throughout the journey.
