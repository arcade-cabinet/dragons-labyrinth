# Dragon's Labyrinth - Product Context

## Why This Project Exists

### The Problem
Traditional RPGs have become predictable power fantasies where players always become stronger and threats always become manageable. Horror games often rely on jump scares and gore rather than psychological dread. There's a gap in the market for games that use RPG mechanics to deliver genuine emotional horror experiences.

### The Solution
Dragon's Labyrinth inverts the RPG power curve. Instead of growing stronger, the world grows darker. Instead of gaining confidence, players experience mounting dread. The familiar RPG structure becomes a vehicle for horror, using player investment in companions and world to deliver emotional impact when everything falls apart.

### The Architecture (Godot Pivot + Regions)

We are moving from a pure infinite map concept to a Regions‑first structure aligned to Themes:

- **Regions by Bands**: 1–20, 21–40, 41–60, 61–120, 121–180; ~3 named regions per band.
- **Overworld**: Tilted hex 2.5D via Godot TileMap; layer‑cake tiles with adjacency rules.
- **Transitions**: Key beats are 3D first‑person scenes loaded by `TransitionLoader`.
- **Orchestrator**: GPT‑5 structured outputs synthesize region prompts/specs and manifests from compact guides.

**Key Architectural Innovations:**

1. **Infinite Algorithmic World**: Distance = progression, corruption spreads from dragon mathematically
2. **Memory-Optimized Chunks**: Only load what's visible, can run on any device
3. **Algorithmic Dungeons**: Proven maze algorithms (Recursive Backtracker → Kruskal → Non-Euclidean)
4. **Minimal Scripted Content**: Only 9 major boss encounters + mini-bosses scripted
5. **Direct ECS Architecture**: No database layer, components are the source of truth

**Why This Changes Everything:**
- **Infinite Content**: Hex map generates forever, never run out of world
- **Perfect Memory Use**: Only load what's visible, chunks load/unload automatically
- **Natural Progression**: Distance traveled = progression (1-180), corruption spreads organically
- **Minimal Work**: Maybe 5000 lines of code total, could ship in a month

### Database/Bevy De‑emphasis

We have eliminated the Rust/Bevy workspace for this reboot. The runtime is Godot; Python handles generation/orchestration. Prior docs referring to Bevy ECS/DB are historical.

### The Innovation
- **Narrative Orchestration**: The emotional journey IS the game orchestrator
- **Inverted Power Curve**: Growing weaker as horror intensifies
- **Companion Trauma**: NPCs that genuinely break under pressure
- **Proximity Horror**: Being hunted creates real tension
- **Moral Weight**: Choices matter because they affect people you care about
- **Algorithmic Horror**: World darkness increases mathematically with distance from safety

## Target Experience

### Opening Moments
Player opens their front door to a beautiful morning. Birds singing, sun shining, neighbors waving. A quest to deliver bread to the next village. Everything is perfect. Too perfect.

### Mid-Game Transformation
The world literally darkens. NPCs flee. Companions question continuing. The player realizes they're not the hero of this story - they're prey. The dragon isn't a boss to defeat - it's an inevitability to face.

### Climactic Horror
Sudden shift to first-person perspective in the labyrinth. The dragon actively hunts you. Audio cues reveal proximity. Sanity mechanics create false sounds. Your companions are either dead, fled, or turned against you. You're alone in the dark.

## User Journey

### Discovery Phase (Stage 0-1)
- Players think it's a standard fantasy RPG
- Comfortable with familiar mechanics
- Building attachment to companions
- Enjoying the beautiful world

### Unease Phase (Stage 1-2)  
- Something feels wrong but can't place it
- NPCs behavior slightly off
- Shadows seem longer than they should be
- First boss encounter introduces moral complexity

### Dread Phase (Stage 2-3)
- Open acknowledgment that something terrible approaches
- Companions showing stress and trauma
- World visibly corrupting
- Economy and society collapsing

### Terror Phase (Stage 3-4)
- Reality itself becomes unreliable
- Companions breaking or betraying
- Moral choices with no good options
- Preparation for inevitable confrontation

### Horror Phase (Stage 4)
- Complete tonal shift to survival horror
- First-person perspective in labyrinth
- Hunted by incomprehensible intelligence
- Multiple endings based on understanding, not power

## Emotional Goals

### What Players Should Feel
1. **Attachment**: To companions and world before corruption
2. **Unease**: Subtle wrongness that builds gradually
3. **Dread**: Knowing something terrible approaches
4. **Loss**: As companions break or abandon you
5. **Terror**: When hunted in the labyrinth
6. **Catharsis**: Through ending choices

### What Players Should Remember
- The moment they realized this wasn't a normal RPG
- When their favorite companion broke/left/betrayed them
- The first time they heard the dragon hunting them
- The weight of their moral choices
- How the ending reflected their journey

## Gameplay Loop

### Exploration Loop
1. Discover new hex tiles (infinite world generation)
2. Interact with NPCs and environment
3. Notice subtle changes based on dread level
4. Make choices that affect narrative progression

### Combat Loop
1. Not about winning but surviving
2. Boss fights are moral dilemmas
3. Violence has psychological cost
4. Companions react to player choices

### Narrative Loop
1. Complete quests that advance dread
2. Experience companion story beats
3. Face increasingly horrible revelations
4. Make choices that determine ending

## Success Metrics

### Engagement Metrics
- Players complete full emotional journey
- High companion attachment scores
- Meaningful choice engagement
- Multiple playthrough for different endings

### Emotional Metrics
- Genuine dread reported by players
- Attachment to companions before loss
- Surprise at genre subversion
- Discussion of moral choices

### Technical Metrics
- Smooth performance on target devices
- No breaking bugs in critical moments
- Seamless perspective shift to first-person
- Effective proximity audio system

## Market Position

### Unique Selling Points
- First true horror-first RPG with infinite algorithmic world
- Inverted power progression with companion trauma system
- Revolutionary architecture: One infinite hex map + algorithmic dungeons
- Proximity horror mechanics with spatial audio
- Meaningful moral choices that affect companion psychology
- Direct ECS architecture eliminates loading times

### Target Audience
- RPG players seeking deeper experiences
- Horror fans wanting psychological dread over jump scares
- Narrative gamers valuing emotional journeys
- Players who appreciated Spec Ops: The Line's subversion
- Indie game enthusiasts looking for innovative mechanics

### Platform Strategy
- **Primary**: Native builds (Windows, macOS, Linux) for best performance
- **Secondary**: WebAssembly for instant play (no download friction)
- **Mobile**: Optimized builds leveraging infinite chunk loading
- **No platform exclusivity**: Maximum accessibility

## Content Philosophy

### AI-Generated Assets (Layer Cake System)
All visual and audio assets use our revolutionary layer cake approach:
- **Tile**: Base hex coordinate container
- **Biome**: Base layer with adjacency rules (no lava next to snow)
- **Path**: Transparent overlay for connections
- **Feature**: Interactive overlay for content

**Benefits:**
- Unique aesthetic not seen elsewhere
- Consistent art style across all content
- No licensing or attribution issues
- Infinite variation within horror progression parameters

### Direct ECS Architecture
**Game-Database Migration Complete:**
- 20+ sophisticated models (companions, corruption, forge, weather) → Pure Bevy components
- Complete ECS systems with horror integration → Direct game-engine integration
- Production-ready Bevy integration (bevy_integration.rs) → Foundation already built
- SeaORM dependencies removed → No runtime database overhead

### Component Independence
Every system stands alone, ensuring:
- Modular development and testing
- Easy addition of new features
- Graceful degradation if needed
- Clear architectural boundaries

## Core Experience

### The Emotional Journey
**Peace → Unease → Dread → Terror → Horror**

This progression never reverses. Each phase transforms every system:
- NPCs go from helpful to fearful to hostile
- Quests twist from innocent to horrifying
- Your companions develop trauma and may abandon you
- The world literally darkens as you progress toward the dragon

### The Horror Loop
1. **Door**: Open your front door (first-person) - the last peaceful moment
2. **Explore**: Infinite hex world that reacts to your growing curse
3. **Survive**: Combat makes you weaker, not stronger
4. **Question**: Reality breaks down through algorithmic corruption
5. **Labyrinth**: First-person horror where the dragon hunts YOU

### Key Emotions
- **Nostalgia**: For that peaceful morning at your door
- **Unease**: Something is wrong but you can't place it
- **Dread**: The weight of mathematical inevitability
- **Terror**: Being hunted by something ancient
- **Understanding**: Multiple endings based on comprehension, not power

## Unique Selling Points

1. **The Opening**: First-person door scene that haunts players hours later
2. **Infinite Algorithmic Horror**: World that grows darker with mathematical precision
3. **Companion Trauma**: Sophisticated psychology system with therapy mechanics
4. **No Power Fantasy**: You become weaker, more cursed as you progress
5. **The Dragon**: Not a boss but a hunter with proximity mechanics
6. **Revolutionary Architecture**: 5000 lines of code, infinite content

## What Makes It Special

- **Perspective Shifts**: Jarring transitions from hex to first-person
- **Audio-Driven Horror**: Spatial audio is the primary fear vector
- **Sanity System**: False sounds, hallucinations, algorithmic reality breakdown
- **Environmental Storytelling**: World tells story through mathematical corruption
- **Narrative Coherence**: Every mechanic serves the horror arc
- **Direct ECS Performance**: No loading times, everything compiled

## Core Philosophy

"We're not building 'an RPG with horror elements' - we're building a horror experience that happens to have RPG mechanics, powered by infinite algorithms and sophisticated companion psychology systems."

## The Dragon's Truth

The dragon is never just a boss to defeat. It's a force of nature, perhaps even protecting the world from something worse. The multiple endings depend on understanding this truth, not defeating it through power.

## Development Reality

**We're 80% done** - just need to liberate existing sophisticated logic from game-database and add infinite generation algorithms. The core horror RPG systems (companion psychology, dread progression, forge mechanics) are already built and production-ready.

**Quote that Captures Everything:**
"It's a horror RPG. I really like the idea... Literally an entire book is a hobbit walking through this entire world. It doesn't even bloody end with him accomplishing anything. He's just on a mountain. It's just... You can feel it like a weight, a chill in the air."
