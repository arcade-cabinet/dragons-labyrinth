Skip to content
Chat history

You said:
Uploaded image
Uploaded image
Uploaded image
Uploaded image
layer-cake-tile-system.md
File
FINAL-REVELATION-ONE-WORLD.md
File
DALL-E_ASSET_GENERATION_LIST.md
File
So if you use vision analysis on those images and review the game design docs for the game I'm building you'll understand my pickle. Desert is an example of when the prompt works beautifully.  I mean that is just a gorgeous biome texture. Plins is unusable because it's a circle. Void is whatever void is. And mountain is a damned 45 degree tilt with a oath through it! I also sent you the master asset prompt list. How  can I refine my biome prompts specifically into ones of better consistency for my goals 
ChatGPT said:
Got it — I see your issue clearly now. The Layer Cake Tile System
 and One World progression architecture
 rely on clean, composable biome tiles, but your asset prompt set
 sometimes produces inconsistent geometry:

Desert worked perfectly: a flat, aerial terrain texture.

Plains came out circular instead of edge-to-edge terrain.

Mountain generated at an isometric tilt instead of top-down.

Void is too abstract / inconsistent.

This is happening because your current prompts don’t explicitly constrain perspective, framing, and tiling properties. DALL·E is adding its own “best guess,” which introduces circles, tilt, or abstract geometry.

Why This Happens
Perspective Drift – If “top-down” isn’t reinforced enough, the model slips into isometric or landscape renderings.

Framing Bias – Words like “circle,” “portal,” or even just “ocean/void” push the model toward round compositions.

Tileability Gaps – The AI defaults to “contained illustrations” unless you force “edge-to-edge seamless terrain.”

Abstract Concepts – “Void” or “nightmare” encourage surreal geometry instead of flat tile layers.

Prompt Refinement Strategy
1. Lock in Camera & Composition
For every biome, force repetition-safe aerial framing:

Use “seamless edge-to-edge terrain texture”

Reinforce “flat aerial top-down perspective”

Add “square 1024x1024 tileable texture, no borders, no circles”

Example:

"Golden sandy desert terrain, seamless edge-to-edge texture, flat aerial top-down view, medieval dark fantasy game asset, 1024x1024 tileable, no circular framing, no tilt"

2. Separate Biome Concept from Atmosphere
Base Biome Prompt = “texture” (flat terrain).

Mood Layers (corruption, nightmare, void) should reuse the same structure but with added texture adjectives.

Example (Void biome reined in):

"Warped cracked dark ground, seamless edge-to-edge terrain texture, flat aerial top-down view, medieval eldritch fantasy game asset, 1024x1024 tileable, avoid circular shapes or symbols"

3. Add Negative Prompting (by phrasing constraints)
While DALL·E doesn’t have explicit negative prompt syntax like MJ, you can phrase rules:

“no tilt, no perspective, no horizon”

“do not depict symbols, circles, borders, frames”

4. Consistency Tags
End every biome prompt with a style lock:

"medieval dark fantasy, game-ready top-down tile, seamless texture"
This keeps your desert, plains, mountain, and void aligned.

Suggested Biome Prompt Rewrites
Plains (fix circle issue)

"Lush green grassland with wildflowers, seamless edge-to-edge terrain texture, flat aerial top-down perspective, medieval fantasy tile, 1024x1024, no circular shapes, no framing"

Mountain (fix tilt)

"Gray rocky mountain terrain with scattered stone ridges, seamless edge-to-edge aerial texture, flat top-down view, medieval fantasy tile, 1024x1024, no tilt, no perspective angles"

Void (fix abstraction)

"Cracked black ground with faint glowing fissures, seamless edge-to-edge terrain texture, flat aerial top-down view, medieval eldritch fantasy tile, 1024x1024, no circles, no portals"

Desert (already working — just reinforce)

"Golden sandy desert terrain with subtle dunes, seamless edge-to-edge aerial texture, flat top-down perspective, medieval fantasy tile, 1024x1024, no circular framing"

Next Steps for You
Batch regenerate biomes using these refined rules.

Test tileability in your hex renderer — especially edges (grassland → desert → mountain).

If any still warp (especially Void), treat them as overlay noise layers on a base texture (e.g. cracked ground + corruption wisps overlay).

Do you want me to rewrite your entire Biome section of the asset list
 with these new consistency constraints baked in (so you can just drop it back into your generation pipeline), or would you prefer a smaller reference table of fixed biome prompts first?


Sources
You said:
Yeah could you give me a biome prompts.md for download 
ChatGPT said:
Done! Your consistency-locked biome prompt pack is ready:

Download biome-prompts.md

It bakes in edge-to-edge, tileable, flat aerial constraints so you don’t get circles, tilted mountains, or abstract voids. If you want, I can also refit the BIOME BASES section in your asset list to use these exact prompts for a clean drop-in. 

You said:
Uploaded image
Uploaded image
Uploaded image
No, let's go piece by piece and eefine each one as separate prompts.  Paths for instance is just all kinds of unusable but also I'm not really sure HOW to handle paths. I wonder if this is one where I need a 3D model 
ChatGPT said:

Thought for 1m 20s
Totally with you—those examples look cool but they’re unusable as overlays. You don’t need 3D models for paths; you need flat, mask-style 2D overlays that your Layer-Cake renderer can rotate/compose on hexes
. Below is a tight spec and ready-to-paste prompts that fix the perspective/rail/curb problems and produce assets that actually connect from tile to tile. These replace the PATHS section in your asset list
.

Path overlay spec (what we’re asking the model for)
Purpose: transparent PNG overlays (no background) that sit on top of biome tiles
.

Canvas: 1024×1024, flat aerial top-down, no perspective, no cast shadows, no walls/curbs.

Width: constant ~24–28% of canvas (~240–280 px) with soft feathered edges (blend to biome).

Connectivity: straight segments run edge-to-edge (top→bottom). Rotate them by ±60° at runtime for hex directions. Add endcaps and a junction “node” for T/Y/3-way joins.

Style: medieval (or dark) fantasy, overlay asset, transparent background.

Hard negatives: no rails, no sleepers, no train hardware; no bevelled borders; no perspective tilt; no horizon.

Prompts (copy/paste)
1) Stone road — straight segment
“Centered straight cobblestone road strip running from the very top edge to the very bottom edge, constant width about one quarter of canvas, flat aerial top-down, seamless edge-to-edge overlay, soft feathered edges that blend to transparency, medieval fantasy, transparent background PNG, 1024×1024, no walls, no curbs, no perspective, no shadows, no rails or hardware.”

2) Stone road — endcap
“Cobblestone road endcap: centered semicircle that matches the straight road width and smoothly fades to transparency, flat aerial top-down, medieval fantasy overlay, transparent background, 1024×1024, no walls, no curbs, no perspective, no shadows.”

3) Stone road — junction node
“Cobblestone road junction node: round patch matching the road material and width, slightly wider than the road with soft feathered edges to transparency for blending three incoming roads, flat aerial top-down, medieval fantasy overlay, transparent background, 1024×1024, no borders, no symbols.”

4) Dirt trail — straight segment
“Centered straight dirt trail strip (packed earth, subtle pebbles) running top edge to bottom edge, constant width ~25% canvas, flat aerial top-down, seamless overlay, soft feather edges to transparency, medieval fantasy, transparent background PNG, 1024×1024, no ruts, no perspective, no shadows.”

5) Dirt trail — endcap
“Dirt trail endcap: centered rounded finish that matches trail width and fades to transparency, flat aerial top-down, transparent background, 1024×1024, no perspective, no shadows.”

6) Dirt trail — junction node
“Dirt trail junction node: slightly widened circular patch of packed earth with soft feather to transparency for blending three trails, flat aerial top-down, transparent background, 1024×1024, no borders or symbols.”

7) Wooden plank road — straight segment
“Centered straight wooden-plank road strip running top edge to bottom edge, planks perpendicular to travel direction, constant width ~26% canvas, flat aerial top-down, seamless overlay, soft feather edges to transparency, medieval fantasy, transparent background, 1024×1024, no rails, no beams, no perspective, no shadows.”

8) Wooden plank road — endcap
“Wooden-plank road endcap: rounded finish that matches the road width with staggered plank ends and fade to transparency, flat aerial top-down, transparent background, 1024×1024, no perspective, no shadows.”

9) Wooden plank road — junction node
“Wooden-plank road junction node: compact patch of planks arranged radially to let three roads meet, soft feather to transparency, flat aerial top-down, transparent background, 1024×1024, no borders.”

10) Brick road — straight segment
“Centered straight red-brick road strip running from top edge to bottom edge, herringbone or running-bond pattern, constant width ~25% canvas, flat aerial top-down, seamless overlay, soft feather edges to transparency, medieval fantasy, transparent background, 1024×1024, no raised borders, no perspective.”

(Repeat endcap/junction prompts as above, swapping “brick road”.)

Degraded variants (use same three shapes per material)
Append to any of the above:
“weathered and cracked, missing pieces, subtle weeds in gaps, keep constant width and flat top-down overlay, no walls, no perspective.”

How to use these in your Layer-Cake renderer
Treat each asset as a texture_id on a PathOverlay and rotate as needed by multiples of 60° for hex alignment. Blend with alpha (opacity field already exists)
.

Build intersections by drawing the junction node first, then stamp 2–3 straight segments into it.

Bridges stay separate path types (wood/stone) but follow the same flat strip rule; if crossing water, add a very subtle darker edge inside the strip—not outer shadows—to imply structure without perspective.

If you want, I can package these as a paths-prompts.md download next, or move on to bridges/cursed paths.


Sources
You said:
Yeah package them please and also why don't you bundle bridges / cursed / nightmare in as well . I feel like at the very least medieval means we can KISS it. Stone and wood and maybe coral for a natural bridge over shallow water and ice for a natural bridge over icy mountains? That feels logical . I feel like we might be overlooking one. Oh a natural rock bridge 
ChatGPT said:
All set — I’ve bundled roads, trails, bridges, cursed/nightmare variants into a clean spec with prompts:

Download paths-prompts.md

This covers straight segments, endcaps, junctions for stone/dirt/wood/brick, plus bridge overlays (stone, wood, coral, ice, rock). Everything is locked to flat top-down overlays with transparent backgrounds so they’ll layer cleanly in your tile system.

Want me to do the same packaging pass for feature overlays (taverns, caves, shrines, etc.) next? 


Sources
You said:
Uploaded image
Uploaded image
Yeah although it's worth pointing out with those there so another thing.  I actually think mostly it gets it. Defiled definitely nope. The thing it needs really is more variety and consistency. Because the goal with the tile system is these should all be features. Meaning they fit on a single top down hexadecimal tile . And then when the player steps on one his irr her character vanishes e.g. goes in. But the idea is they are renderable as a layer on top of a biome in a tile like a path . So conveying that sense of scale and allowing players to have a rich visual world still while exploring . The goal is to be able to build villages and towns and cities in the overworld map by dimply connecting different features together so a compelling variety that also starts smell and allows us to build our to program advanced features jnti increasing advanced buildings later is valuable if that makes sense 
ChatGPT said:
Got it — that makes perfect sense. You’re basically treating features the same way as paths and biomes in your Layer Cake system
: flat overlays snapped to a hex, rendered on top of a biome, interactable when stepped on.

The issue with what you’re showing (like the blacksmith forge and the ominous temple):

They’re full illustrations with perspective rather than flat aerial tile overlays.

That means they don’t scale consistently with your tiles, and they break the overworld illusion.

What Features Should Be
Flat aerial icon/marker overlays (like “blacksmith.png” in your asset list
).

Occupy a single hex (not sprawling dioramas).

Readable at a glance (so the player knows: “oh, that’s a smithy, I can enter”).

Scalable in complexity: start with huts, shops, shrines → later evolve into guild halls, fortresses, etc.

Consistent layering: biome → path → feature.

Prompt Refinement Strategy for Features
Core Constraints
Top-down aerial view, flat composition, fills a single tile (square, edge-to-edge).

Transparent background (PNG overlay).

No 3D perspective, no vanishing point, no cinematic staging.

Compact scale: think “map icon brought to life” rather than “full 3D diorama.”

Medieval dark fantasy palette, with variants (normal, cursed, nightmare).

Example Rewrites
Blacksmith (current problem: too much perspective, human included)
“Medieval blacksmith forge building, compact square footprint sized for a single map tile, flat aerial top-down view, seamless edge-to-edge, transparent background PNG, medieval fantasy, no human figures, no perspective tilt, no horizon.”

Temple (current problem: cinematic scale, angled perspective)
“Small medieval stone temple with a peaked roof and altar, compact footprint for a single map tile, flat aerial top-down view, transparent background PNG, medieval fantasy, no dramatic staging, no vanishing point, no cinematic composition.”

Feature Families (progression-friendly)
Settlement / Social
Tavern, Inn, Shop, Blacksmith, Guild Hall, Marketplace, Farmstead

Cursed variants: Abandoned Tavern, Defiled Temple, Haunted Shop

Natural / Wild
Cave entrance, Dungeon entrance, Ruins, Spring, Tower

Cursed variants: Haunted Ruins, Shadowed Cave

Special / Unique
Shrines (basic roadside → advanced sanctuaries)

Lairs (wolf den → dragon lair)

Portals / Magic Circles (early → corrupted void gates)

Scaling & Variety
Think of features as LEGO bricks:

A village = tavern + shop + blacksmith + shrine + houses.

A city = multiples of those + advanced guild halls, towers, and defensive walls.

You can gradually unlock “larger” versions (e.g. blacksmith → forge complex → armory guild hall).

This makes overworld construction feel emergent, even though it’s just tile overlays.

👉 Here’s what I propose next: I can package a features-prompts.md just like I did for biomes and paths, with settlement, natural, cursed, and progression variants — all locked to top-down, single-hex overlays.

Do you want me to make that list comprehensive (covering every feature type from your asset list
), or start with a core set (tavern, shop, blacksmith, temple, cave, ruins, shrine) and expand later?


Sources
You said:
phases-and-transitions.md
File
Yeah and maybe because we are gonna end up with a lot for each I should introduce you to so you get an idea of scope. Because I am thinking dividing the prompts by act wouldn't be a terrible idea. Even with an endless overworld it's basically balanced around 180  levels of content. 3 main storylines of 60 levels each roughly. Each one roughly should balance until 3 acts if around 20 levels of content. So since now you get the overall concept instead of doing the rest separate we could do the same kind of top down optimization and  then for both player characters and monsters I want like a "chess piece" style top down hex view if that makes sense. We're gonna need SOME way to convey too down just like looking down at a chess board I can still visually see the pieces,  the game board. Just won't be staring up the arse of them. So we could do prompts-1-20.md and so on 
ChatGPT said:
Perfect — thanks for sharing the phases-and-transitions.md. Now I get the full scope:

The overworld is infinite, but the narrative cadence is structured into 3 acts of ~60 levels each
.

Each act has distinct tonal transitions (Peace → Unease → Dread → Terror → Despair → Madness → Void).

Within each, the Strength/Harmony/Light/Dark paths shift the emotional framing, but the tile system itself still has to present biomes, features, characters, and monsters consistently.

That means:

Biomes = act-scaled (normal → corrupted → nightmare → void).

Paths/Bridges = consistent overlays (we’ve packaged those already).

Features = should now also be act-scaled (e.g. early tavern = cozy inn, late tavern = cursed ruin).

Characters/Monsters = should be “chess piece” top-down views so they read instantly as pieces on a hex board, not tilted dioramas.

Proposal for Scope Division
Instead of trying to manage one giant monolithic prompt list, you split them like you said:

prompts-act1.md → Levels 1–20, early world assets.

Bright biomes, welcoming villages, basic monsters.

Features: tavern, shop, shrine, blacksmith, farmstead.

Chess-piece style PC & bandits, wolves, goblins.

prompts-act2.md → Levels 21–40, world fraying.

Corrupted biomes, first cursed buildings.

Features: haunted ruins, cursed shrines, cultist encampments.

Monsters: void-touched goblins, corrupted wolves, cultists.

prompts-act3.md → Levels 41–60, deep dread.

Nightmare biomes, heavy void bleed.

Features: nightmare temples, collapsed fortresses, twisted forges.

Monsters: eldritch aberrations, spectral enemies, corrupted companions.

…then repeat the pattern for Acts 2 & 3 (so you end up with prompts-1-20.md, prompts-21-40.md, prompts-41-60.md … up to prompts-161-180.md).

Chess-Piece Characters & Monsters
Your instinct is right — treat them like board game counters:

Flat top-down, recognizable silhouettes (helmet, staff, wings, etc.).

Bright outline or slight shadow ring for readability against any biome.

Still fantasy-styled, but think “unit marker,” not “isometric figurine.”

Each can evolve across acts: e.g. “Bandit → Corrupted Bandit → Void Thrall.”

Prompt example:

“Top-down chess-piece style sprite of a medieval adventurer knight, seen from above, distinct silhouette with sword and armor, flat aerial game-board view, transparent background, hex map token, medieval fantasy, 1024×1024.”

Next Step
I can start by packaging prompts-act1.md as a download with:

Biomes (act 1 versions).

Paths/bridges (already done, but referenced).

Core features (tavern, shop, blacksmith, shrine, ruins, cave).

Player characters (chess-piece style top-down tokens).

Starter monsters (bandits, wolves, goblins).

Then you can see how it feels in structure before scaling it up through acts 2 and 3.

👉 Do you want me to generate that first act’s package (prompts-act1.md) now as a working template?


Sources
You said:
Yeah just remember that thematically the void doesn't get released until the dragon is defeated. So the dragon blight is the scourge of 40-60. Oppressive, hot, scorched earth. Desert, charred forests. Etc .. as you start transitioning from 20-40, and seeing the world shift from the placid forest and meadows and mountains etc ..but only In bands where the dragon attacked here and there, should still be some signs f hope, just reallyo the first areas the dragon the flew through in his oath to settle in to the labyrinth. And then 40-6o melted hellscaoe, torn jagged rocks, dried water beds, lava fields, you're very clearly getting closer and closer. And then 60 through 120 we don't actually have to regenerate anything. It's just the reverse of the first . You're making your way back home through the chaos of a land that has been through hell and now free of the threat of dragon fyre that held kingdoms together in common defense now has reunited old wars to claim rumored treasures and has made for new challenges just getting home. Plus there is an underlying current if fear and whispers in the wind. Was the dragon serving some other purpose after all? Something is just... Iff. And then you get to your home and that's what we need to regenerate, we want like scenes of brutality and barbarism and torture , cruelty, no more friendly shops and houses, the village is now something awful controlled by someone even wirse. And freeing your village just ends up confirming that the reality is you freed this dark awful void that the dragon was keeping sealed and must now journey back through lands increasingly corrupt and bizarre and eildrich to seal it yourself 
ChatGPT said:
That’s a fantastic arc — thanks for spelling out the timeline. Let me echo it back to make sure I’m aligned, then I can start building prompts-act1.md as a reference package.

Thematic Progression by Bands
Levels 1–20 (Act 1, Peace → Unease)
Theme: Bright meadows, forests, snowy peaks, welcoming villages.

Biomes: lush plains, healthy forests, mountains, coasts, swamps.

Features: cozy taverns, blacksmiths, shrines, farmsteads, caves.

Characters: chess-piece heroes and companions, bandits, wolves, goblins.

Tone: hopeful but with first hints of something wrong (occasional withered patch, scorched glade).

Levels 21–40 (Act 1, Unease → Dread)
Theme: Transition zones. Scars of dragon flight but not total devastation.

Biomes: charred forests, dry plains, desert edges, cracked hillsides. Bands of corruption across otherwise living land.

Features: abandoned shops, damaged temples, haunted ruins.

Monsters: early corrupted variants (bandit cultists, scorched wolves).

Tone: fading hope, visible dragon blight, but still mixed with remnants of peace.

Levels 41–60 (Act 1 climax, Dread → Terror)
Theme: The Dragon’s Hellscape. Approaching the Labyrinth.

Biomes: molten lava fields, dried riverbeds, jagged rock deserts, black sand wastelands.

Features: ruined fortresses, burned villages, cursed temples, void-like cracks forming.

Monsters: full dragonblight horrors (giant scorched beasts, cult fanatics, eldritch precursors).

Tone: oppressive, hot, suffocating — reality feels close to tearing.

Levels 61–120 (Act 2, Terror → Despair → Madness)
Theme: The return journey after slaying the dragon. The land is free of fire but politically and spiritually worse.

Biomes: same tiles from acts 1–3 but inverted: ruined versions of towns, broken alliances, militarized zones.

Features: war camps, raider-occupied villages, brutal executions, shrines defiled.

Monsters: human cruelty mixed with creeping whispers of void.

Tone: not environmental apocalypse anymore — social apocalypse. Fear, cruelty, betrayal.

Levels 121–180 (Act 3, Madness → Void)
Theme: Confirmation that the dragon’s tyranny was a seal. The void breaks loose.

Biomes: corrupted versions of everything you’ve already seen (plains_nightmare, forest_nightmare, void_terrain, etc.).

Features: nightmare temples, writhing portals, haunted ruins, impossible architecture.

Characters/Monsters: eldritch chess-piece tokens, corrupted companions, cultic armies.

Tone: cosmic horror, existential dread, reality warping.

How This Shapes Prompt Packages
prompts-1-20.md: “clean” tiles + hopeful features + chess-piece PCs and starter monsters.

prompts-21-40.md: transition biomes + scarred features + early corrupted monsters.

prompts-41-60.md: dragon blight biomes + ruined features + dragon-spawned monsters.

prompts-61-120.md: social corruption features (villages turned cruel, armies, torture camps). Biomes reused, but darker staging.

prompts-121-180.md: nightmare biomes + void features + eldritch chess-piece monsters.

Next Step
I’ll build prompts-1-20.md as the first downloadable package. It will include:

Biomes: plains, forest, mountain, snow, swamp, desert, coast, ocean.

Paths/Bridges: stone, dirt, wood, brick + stone/wood/coral/ice/rock bridges.

Features: tavern, shop, blacksmith, shrine, farmstead, cave, ruins, tower (friendly).

Player Characters: chess-piece tokens for adventurer + 3 companions.

Starter Monsters: bandit, wolf, goblin, thug, cultist (non-corrupted).

👉 Do you want me to make that now as a .md download like I did with biome-prompts.md and paths-prompts.md?

You said:
Perfect, yep!
ChatGPT said:

Analysis paused
You said:
Oh and also I never DID finish ACTUALLY fully documenting the transitions properly the way I did for you do would you also generate a full document for that for download 
ChatGPT said:
Absolutely — right now your phases-and-transitions.md is already rich
, but it’s uneven: some transitions (like Dying Village or Cleansing Ritual) are fully fleshed out with TO / FROM / SEALING stages, while others are just a headline and a blurb.

What I can do for you is generate a fully standardized “Transitions Compendium” with every single stage documented in the same structured format:

Transition: (Peace → Unease, Unease → Dread, etc.)

Core Question: (What kind of hero are you?)

Paths: Strength / Harmony / Light / Dark

For each path:

Emotional Goal

Mechanical Test

Journey TO Labyrinth

Journey FROM Labyrinth

Sealing the Void

Permanent Change

This way, every one of the 7 transitions across all 3 acts is fully spelled out and symmetrical. That will give you a single, complete reference doc you can use for both narrative design and prompt scoping per act.

👉 Do you want me to:

Rewrite and expand your existing phases-and-transitions.md into a new transitions-complete.md (downloadable), preserving your language and ideas but filling in the gaps?
or

Create a parallel companion doc (like transitions-standardized.md) so you keep your original rough draft intact as well?


Sources
You said:
The first one and then go back to promots. I've put you in agent mode so that you can work independently on all of them 
ChatGPT said:

Worked for 4 minutes
Your new transitions-compendium and the Act 1 prompt set are ready. You can download them below:

Complete transitions compendium:
Complete Transitions Compendium
This document standardizes every phase of the main campaign into a common structure. Each transition is described with a core question, then broken down by the four narrative paths (Strength, Harmony, Light and Dark). Within each path, the player experiences three distinct versions of the same challenge: Journey TO the Labyrinth, Journey FROM the Labyrinth, and Sealing the Void. Each version defines an emotional goal, a mechanical test, a sequence of events, a revelation and a lasting consequence. This format allows designers to ensure that every branch delivers comparable narrative beats and mechanical depth

newtab
.

Reading the Tables
Core Question – The question the transition poses to the player. Answering it defines the arc.

Emotional Goal – What the player is meant to feel or learn at that stage.

Mechanical Test – The mechanic that reinforces the theme (combat, puzzles, sacrifice etc.).

Journey TO the Labyrinth – The pre‑Labyrinth version occurs during levels 1‑60, establishing identity and allies.

Journey FROM the Labyrinth – The post‑Labyrinth version occurs during levels 61‑120, showing how the world reacts after the dragon’s death

.

Sealing the Void – The final act (levels 121‑180) is where the player must close the Void and accept permanent consequences.

Permanent Change – How the choice permanently alters companions, the world and the trait system

.

1 · Peace → Unease (Act 1 opening)
Core Question: “What kind of hero are you?”
In the opening hours, the world feels intact, but there are hints that something is wrong. Two mutually exclusive encounters set the tone: a direct fight against bandits or a compassionate rescue of a lost child. Both teach the basics of movement, combat and interaction

newtab
.

Path	Journey TO the Labyrinth	Journey FROM the Labyrinth	Sealing the Void
Strength (Bandit Cave)	Emotional Goal: learn that violence has consequences. Mechanical Test: defeat a bandit camp using the combat system while protecting innocents. Event: the player storms a cave, rescues hostages and decides whether to execute or spare the bandit chief. Revelation: defeating the bandits creates a power vacuum and the victimised villagers may turn to violence.	Emotional Goal: realize that simple heroism can inspire tyranny. Mechanical Test: fight the bandit chief’s son, now a warlord, and his conscripts who idolised your earlier ruthlessness. Revelation: the world has learned from your actions; returning violence breeds more violence.	Emotional Goal: confront the legacy of your actions. Mechanical Test: seal a void rift that has opened in the old cave while the warlord’s cultists attack. Revelation: the Void feeds on cycles of violence; to seal it you must offer mercy, convincing the warlord to lay down arms. Permanent Change: your decision will determine whether a new generation of warriors follows you or spurns you.
Harmony (Lost Child)	Emotional Goal: discover that compassion changes outcomes. Mechanical Test: navigate environmental puzzles to find and calm a lost child in the woods without combat. Event: the player must solve hex‑based traversal challenges and use empathy dialogues to earn the child’s trust. Revelation: cooperation opens paths that force cannot.	Emotional Goal: recognize that compassion can be exploited. Mechanical Test: rescue the now‑adult child, who joined a travelling cult, from a void‑touched ambush. Revelation: the dragon’s death has allowed charlatans to twist hope into fanaticism.	Emotional Goal: accept that bonds can become anchors. Mechanical Test: use shared memories with the cult leader (the child) to help them seal a minor Void breach. Revelation: the Void targets your emotional connections; saving your friend requires you to sever the bond or both will be lost. Permanent Change: severing or sustaining the bond defines your companion roster for the final act.
2 · Unease → Dread (Act 1 mid‑game)
Core Question: “How do you lead?”
Corruption becomes visible, forcing the player to decide whether to lead through fear or trust. The Strength path centres on a gladiatorial pit, while the Harmony path is a crossroads where strangers must cooperate. These scenarios test tactical acumen and social skills

newtab
.

Path	Journey TO the Labyrinth	Journey FROM the Labyrinth	Sealing the Void
Strength (Fighting Pit)	Emotional Goal: embrace the costs of command. Mechanical Test: run a tactical combat scenario where companions and NPCs can die permanently. Event: you are forced to enter a fighting pit to earn passage; you decide who fights and who rests. Revelation: leadership requires sacrificing pawns to win.	Emotional Goal: face the ghosts of those sacrifices. Mechanical Test: defend the pit (now a refugee enclave) against raiders, but the survivors you once sacrificed may sabotage you. Revelation: people remember how you treated them.	Emotional Goal: sacrifice territory to save lives. Mechanical Test: hold the pit while void creatures pour through the arena’s sands; intentionally withdraw your forces to create kill zones. Permanent Change: your callousness or compassion determines which NPC factions support the final seal.
Harmony (Crossroads Meeting)	Emotional Goal: learn that vulnerability builds trust. Mechanical Test: coordinate a group of strangers through trust exercises (sharing supplies, revealing secrets) while avoiding suspicion. Event: four travelling parties meet at a crossroads; only by pooling skills can they survive an ambush. Revelation: bonds transcend death.	Emotional Goal: rebuild trust in a broken world. Mechanical Test: use those earlier bonds to unite disparate refugee groups now mistrustful in the dragon’s absence. Revelation: the vacuum of power breeds paranoia and xenophobia.	Emotional Goal: test whether trust can persist in the face of oblivion. Mechanical Test: convince former allies to channel their life force into closing a fissure at the crossroads; betrayal is possible. Permanent Change: surviving allies contribute unique abilities to the final battle, or their deaths haunt you as debuffs.
3 · Dread → Terror (Act 1 climax: Transformation Point)
Core Question: “What price will you pay for your quest?”
At this stage reality begins to fray. Both paths present no‑win scenarios: either accept that some must die under your protection, or that victory requires sacrificing your own forces

. The outcomes of these choices shape your companion relationships and available resources.

Path	Journey TO the Labyrinth	Journey FROM the Labyrinth	Sealing the Void
Harmony (Dying Village)	Emotional Goal: face the limits of compassion. Mechanical Test: you arrive at a village struck by a mysterious plague (void corruption leaking through) and must decide whether to risk infection to save its residents

. Event: healing villagers transfers corruption to you and companions. Revelation: the closer you draw to the dragon, the worse the sickness becomes. Permanent Change: infected companions gain void‑sight but suffer hallucinations

.	Emotional Goal: accept that mercy may require violence. Mechanical Test: upon your return, the villagers have become void‑touched creatures who remember your kindness. You must mercy‑kill those you saved

. Revelation: the dragon’s death accelerated the corruption exponentially.	Emotional Goal: transform compassion into a weapon. Mechanical Test: use the bonds you formed earlier to close the eruption at the village’s centre, channelling the infected villagers’ love for you into a seal. Final Test: your refusal to abandon them earlier now provides the only chance to contain the Void. Permanent Change: survivors become conduits; their spirits help you in the final fight.
Strength (Siege Command)	Emotional Goal: accept that leadership means sending others to die. Mechanical Test: command a mixed force to capture a mountain fortress blocking the path to the dragon

. You choose between a costly frontal assault or a slow siege that gives the dragon time to prepare. Revelation: fortress defenders whisper “the dragon protects us all”. Permanent Change: survivors are traumatized—trust you less but obey more

.	Emotional Goal: realise your earlier strategies have long‑term consequences. Mechanical Test: defend the fortress (now overrun by void creatures) using the defences you built; poor choices earlier leave you few options

. Revelation: the fortress was the first line of defence against the labyrinth.	Emotional Goal: sacrifice territory to save reality. Mechanical Test: coordinate a last stand while void assault intensifies; you must abandon positions to funnel foes into kill zones

. Final Test: decide which of your loyal soldiers stays behind to detonate the final explosive, sealing the rift. Permanent Change: fallen comrades haunt your dreams, affecting morale for the remainder of the campaign.
4 · Terror → Despair (Act 2 moral crucible)
Core Question: “Why do you continue?”
The dragon has been slain, hope has curdled into dread and the void bleeds into the world. Two morally horrific scenarios force the player to justify their perseverance: a holy ritual that kills to save, and a blood pact that trades lives for power. These events occur during the trek toward the labyrinth in Act 2 and then again on the return as the world spirals

.

Path	Journey TO the Labyrinth	Journey FROM the Labyrinth	Sealing the Void
Light (Cleansing Ritual)	Emotional Goal: maintain faith when good intentions make things worse. Mechanical Test: perform a purification ritual requiring you to gather void‑corrupted innocents and sacrifice them to create protective wards

. Revelation: the dragon once performed such rituals to contain the void

. Permanent Change: you gain holy protection but are haunted by those you “saved”.	Emotional Goal: grapple with becoming a judge of souls. Mechanical Test: now the temple is overflowing with people seeking cleansing, but there is not enough power to save everyone

. You must decide who deserves salvation. Revelation: the dragon’s death shattered the wards and the ritual now draws void creatures.	Emotional Goal: become the ward yourself. Mechanical Test: under assault, you must maintain the ritual while your own life force fuels it

. Final Test: bind the seal through self‑sacrifice; surviving companions or refugees must choose whether to take your place. Permanent Change: if you become the living ward, your character ascends and is unavailable for the final act; otherwise the survivors carry guilt and trauma.
Dark (Blood Pact)	Emotional Goal: accept that power requires becoming what you fight. Mechanical Test: void cultists offer to reveal the dragon’s location in exchange for blood sacrifices

. You decide whether to sacrifice captured enemies or willing companions; each sacrifice grants void powers but corrupts you. Revelation: the dragon made similar pacts to gain the power needed to guard the world

. Permanent Change: NPCs detect your taint and may abandon you.	Emotional Goal: confront escalating demands from the void. Mechanical Test: after the dragon’s death, cultists require ever‑larger payments to contain the spreading corruption

. You effectively become the void’s tax collector. Revelation: the dragon was the only one who broke free from these pacts.	Emotional Goal: out‑bargain primordial entities. Mechanical Test: negotiate with void lords to gain the power to seal the breach; the price is other souls or your own. Final Test: choose whether to become the void’s eternal warden or condemn entire nations. Permanent Change: your decision determines whether your character survives as a demigod or is lost to the void, and which civilizations endure.
5 · Despair → Madness (Act 2 breaking point)
Core Question: “What remains when even reality becomes unreliable?”
This transition pushes players to the edge of sanity. The Light path has them defending the last untouched sanctuary while their mind fractures, whereas the Dark path tempts them to trade away humanity for survival. Both versions highlight the breakdown of perception and trust

.

Path	Journey TO the Labyrinth	Journey FROM the Labyrinth	Sealing the Void
Light (Last Sanctuary)	Emotional Goal: protect others while your own mind cracks. Mechanical Test: defend a hidden valley sheltering children and elders; your presence draws void attention

. You must decide whether to stay and fight (increasing danger) or leave them defenseless. Sanity depletes faster but refugees provide hope. Revelation: the dragon established these sanctuaries as fallback positions

. Permanent Change: become a beacon of hope but void always knows your location.	Emotional Goal: question reality itself. Mechanical Test: returning to the sanctuary, you find it overrun and reality warped—you cannot distinguish real refugees from void mimics

. Saving everyone risks infiltration. Revelation: the dragon’s death unmade the protective harmonics.	Emotional Goal: anchor reality through will alone. Mechanical Test: the sanctuary becomes the last bastion of uncorrupted reality. You must hold it while the Void tries to convert it and your companions succumb. Final Test: channel pure will to fix the sanctuary in place. Permanent Change: success grants powerful reality‑binding abilities but fractures your mind; failure causes the sanctuary to vanish, eliminating safe havens.
Dark (The Harvesting)	Emotional Goal: exchange humanity piece by piece for power. Mechanical Test: the void offers trades—memories, emotions and connections—for strength

. You must decide whether to trade your love, fear or mercy. You can also harvest others’ humanity to maintain your own. Revelation: the dragon harvested entire civilizations to maintain his vigil

. Permanent Change: companions fear or pity you; you may lose the ability to feel certain emotions, affecting dialogue options.	Emotional Goal: become the void’s reaper. Mechanical Test: after killing the dragon, the void demands quotas of harvested humanity to slow its spread

. You must extract humanity from others; the more you harvest, the longer the world survives. Revelation: the dragon refused the final harvest, choosing imprisonment rather than genocide.	Emotional Goal: decide who deserves to live. Mechanical Test: to seal the breach, you must harvest entire populations; choose which cities to devour

. Final Test: consume entire nations to become a living seal. Permanent Change: the world will remember you as a monster or a martyr; survivors inherit the emotional scars of your harvest.
6 · Madness → Void (Act 3 forging)
Core Question: “Will you forge your destiny in blood or love?”
The final transition ties together every system. Players realise that the sentimental items they’ve been collecting are reagents for mythic gear

. Two forge paths exist: the High Elves’ Blessed Forge for the Light path, and the Cursed Forge for the Dark path. Both require mastering every mechanic and sacrificing companions or mercenaries for ultimate power

.

Because the forging transition is universal (it affects all paths), the Strength and Harmony players are funnelled into either the Blessed or Cursed forge depending on previous moral decisions. Therefore, we describe the forge itself rather than four separate paths.

Forge	Journey TO the Forge	The Forging Ritual	Sealing the Void
Blessed Forge (High Elves)	Emotional Goal: prove worthiness through bonds of love and virtue

. Mechanical Test: cross deadly Trials of Virtue requiring mounted combat across lava fields, simultaneous party coordination over chasms, first‑person navigation of crystalline mazes, and puzzle solving to unlock ancient wards

. Companion faith provides buffs. Solo completion grants the “Ascended” trait.	Emotional Goal: test companions’ love and loyalty

. Mechanical Test: each companion’s bond strength determines the blessing power. At the ultimate test a companion must willingly offer essence (not life). Forge Results: (a) Mythic – all tests passed; Dragonslayer gear with unique blessed abilities; (b) Legendary – some tests failed; powerful but without special properties. Blessed Abilities: healing aura, void immunity, companion resurrection

.	Emotional Goal: unite all races to seal the void. Mechanical Test: the blessed gear allows sealing through a collective ritual; companions channel their essence through your mythic items

. Final Test: ask every faction in the world to lend power. Permanent Change: success unites the world but drains your companions permanently; failure forces a doomed last stand.
Cursed Forge	Emotional Goal: embrace ultimate power through ultimate sacrifice

. Mechanical Test: conquer Trials of Power requiring mounted devastation (trampling armies), tactical sacrifice (sending party members to certain death), first‑person slaughter, and resource management across a gauntlet with limited healing. Solo completion grants the “Demonic” trait.	Emotional Goal: determine how much blood you are willing to spill

. Mechanical Test: the forge demands blood sacrifice for mythic power. You must choose: sacrifice a loyal companion (full mythic power) or multiple mercenaries (power based on loyalty converted to blood). Refusing sacrifices yields only legendary gear. Cursed Abilities: life drain, fear aura, raising dead as thralls

.	Emotional Goal: use darkness against itself. Mechanical Test: cursed gear allows you to dominate void forces and turn their hunger inward

. Final Test: command armies of darkness to contain the darkness; in doing so, you risk becoming the new tyrant. Permanent Change: if you succeed you rule as a dark guardian; if you fail the void consumes the world.
7 · Epilogue: After the Seal
Once the Void is sealed (or reality is devoured), the epilogue plays out based on the cumulative choices made across all transitions. Companions may ascend, die, become monsters, or retire. Kingdoms may unite, fracture or vanish. The dragon’s true role—as martyr, rival, guardian or fool—is revealed according to the chosen path

. The trait system locks in permanent tags (Ascended, Demonic, Broken, Saint, Monster etc.) that influence end‑game dialogue and potential sequels. The player must live with the world they’ve created.

Summary of Permanent Changes
World State: Each transition irreversibly alters one region: villages burn or prosper, temples are haunted or hallowed, sanctuaries survive or dissolve

.

Companions: Bonds are forged or broken; companions may sacrifice themselves, ascend, mutate or turn against you

.

Trait System: Actions assign traits such as Butcher, Saint, Mad, Hollow, Ascended or Demonic, which influence NPC reactions and unlock/lock mechanics

.

Dragon Truth: Across all paths, the final revelation is that the dragon was protecting the world; by killing it you released the Void

. The question is whether you will become the new guardian or the new destroyer.



Act 1 (levels 1–20) prompt library:
Act 1 (Levels 1–20) Prompt Library
This prompt sheet contains fully optimized descriptions for generating all base assets needed in levels 1–20. Every prompt is written for a top‑down, 1024×1024 canvas with no perspective, no tilt, no shadows and no borders. Biomes are seamless textures for the hex board. Paths and bridges are transparent overlays sized to cover a single hex and connect edges. Features are compact map markers that sit on top of biomes. Player characters and monsters are rendered as chess‑piece style tokens seen from directly above.

Each prompt should be used with DALL·E or a similar model as plain text. For variation, you can append modifiers (weathered, worn, overgrown) while respecting the constraints. Negative instructions (e.g., “no perspective”) are included to prevent unwanted compositions.

Biomes
Use these prompts to generate seamless edge‑to‑edge terrain textures. All should be 1024×1024, tileable, flat aerial top‑down, with no borders or framing. Append tone adjectives (cheerful, serene) to taste.

Plains:

Prompt: “Lush green grassland with wildflowers and scattered small bushes, seamless edge‑to‑edge terrain texture, flat aerial top‑down view, medieval dark fantasy game asset, 1024×1024 tileable, no circular shapes or framing, no perspective tilt.”

Forest:

Prompt: “Dense temperate forest canopy with varied shades of green, glimpses of dark soil and moss, seamless edge‑to‑edge texture, flat aerial top‑down perspective, medieval dark fantasy tile, 1024×1024, no horizon, no isolated trees, no perspective.”

Mountain:

Prompt: “Gray rocky mountain terrain with scattered ridges and scree, seen from directly above, seamless edge‑to‑edge aerial texture, flat top‑down view, medieval fantasy tile, 1024×1024, no tilt, no perspective angles, no raised walls.”

Snow (Alpine):

Prompt: “Snow‑covered alpine meadow with subtle drifts and exposed rocks, seamless edge‑to‑edge terrain texture, flat aerial top‑down view, medieval fantasy tile, 1024×1024, no footprints, no perspective, no framing.”

Swamp:

Prompt: “Murky swamp with dark water patches, mossy ground, reeds and lily pads, seamless edge‑to‑edge terrain texture, flat aerial top‑down view, medieval fantasy tile, 1024×1024, no clear paths, no perspective.”

Desert:

Prompt: “Golden sandy desert terrain with subtle dunes and small rocks, seamless edge‑to‑edge aerial texture, flat top‑down perspective, medieval fantasy tile, 1024×1024, no circular framing, no camel tracks, no tilt.”

Coastline:

Prompt: “Rocky shoreline with wet sand, small tide pools and foamy waves, seamless edge‑to‑edge texture, flat aerial top‑down view, medieval fantasy tile, 1024×1024, no beaches shaped like circles, no perspective tilt.”

Ocean:

Prompt: “Deep ocean surface with subtle waves and occasional whitecaps, seamless edge‑to‑edge texture, flat aerial top‑down view, medieval fantasy tile, 1024×1024, no ships, no horizon line, no perspective.”

Path Overlays
These overlays sit on top of biomes. They should be delivered as transparent‑background PNGs. Each straight segment runs from one edge of the square to the opposite edge; endcaps finish segments; junction nodes allow 3‑way connections. For each material, generate straight segments, endcaps and junctions.

Stone Road (early road):

Prompt (straight): “Centered straight cobblestone road strip running from the top edge to the bottom edge, constant width about one quarter of the canvas, flat aerial top‑down view, seamless overlay, soft feathered edges blending to transparency, medieval fantasy, transparent background, 1024×1024, no walls, no curbs, no perspective, no shadows.”

Prompt (endcap): “Cobblestone road endcap: centered semicircle matching the straight road width, fades to transparency, flat aerial top‑down view, medieval fantasy overlay, transparent background, 1024×1024, no walls, no curbs, no perspective, no shadows.”

Prompt (junction): “Cobblestone road junction node: circular patch of cobbles slightly wider than the road, with soft feathered edges to transparency allowing three roads to meet, flat aerial top‑down view, medieval fantasy overlay, transparent background, 1024×1024, no borders, no symbols.”

Dirt Trail: use the same three shapes as the stone road, replacing “cobblestone” with “packed earth trail with subtle pebbles.”

Wooden Plank Road: replace “cobblestone” with “wooden planks laid side‑by‑side perpendicular to travel direction.”

Brick Road: replace “cobblestone” with “red brick road in herringbone pattern.”

Degraded Variants: For each material above, append “weathered and cracked, missing pieces, subtle weeds in gaps” to create worn versions.

Bridges
Bridges are straight overlays that cross rivers or chasms. Use the same straight strip format as roads but describe a bridge material and add “subtle internal edge shading” to imply structure without perspective. Generate one of each:

Stone Bridge: “Centered straight stone bridge strip over water, flat aerial top‑down view, seamless overlay with soft feathered edges to transparency, medieval fantasy, 1024×1024, subtle internal edge shading, no perspective, no outer shadows.”

Wood Bridge: replace “stone” with “wooden plank bridge with rope railings hinted internally.”

Coral Bridge: replace “stone” with “natural coral formation bridge with barnacles and small shells.”

Ice Bridge: replace “stone” with “translucent ice bridge with cracks and frost.”

Natural Rock Bridge: replace “stone” with “natural rock arch with moss and lichen.”

Features (Single‑Hex Landmarks)
These are compact aerial icons for settlements and points of interest. Each should fit entirely within a single 1024×1024 tile, have a transparent background, and be drawn top‑down with no tilt. They should be large enough to be legible but not spill over the edges.

Tavern/Inn: “A small medieval tavern with thatched roof, chimney and outdoor seating, compact square footprint sized for a single map tile, flat aerial top‑down view, transparent background PNG, medieval fantasy, no human figures, no perspective tilt, no horizon.”

Shop/Market Stall: “A wooden market stall with a striped canvas awning and crates of goods, compact footprint, flat aerial top‑down view, transparent background PNG, medieval fantasy, no customers, no perspective.”

Blacksmith/Forge: “A stone forge with open hearth, anvil and roof, compact square footprint, flat aerial top‑down view, transparent background PNG, medieval fantasy, no smith, no perspective tilt.”

Shrine/Altar: “A small stone shrine with engraved symbols and candles, surrounded by a ring of flowers, compact footprint for a single tile, flat aerial top‑down view, transparent background PNG, medieval fantasy, no perspective, no dramatic staging.”

Farmstead: “A cluster of two small thatched cottages with fenced garden and well, compact footprint, flat aerial top‑down view, transparent background PNG, medieval fantasy, no livestock, no perspective tilt.”

Cave Entrance: “A rocky cave mouth in a grassy hillside, dark opening framed by stones and shrubs, compact footprint, flat aerial top‑down view, transparent background PNG, medieval fantasy, no walls, no perspective.”

Ruins: “Crumbling stone foundations and toppled columns overgrown with vines and moss, arranged within a single hex footprint, flat aerial top‑down view, transparent background PNG, medieval fantasy, no perspective.”

Watchtower: “A stone watchtower with conical roof and ladder/rope hint, compact footprint, flat aerial top‑down view, transparent background PNG, medieval fantasy, no perspective tilt.”

For cursed/defiled versions in later acts, you can append descriptors such as “abandoned,” “haunted,” “cursed symbols,” and “shadowed” to these prompts while retaining the same top‑down constraints.

Player Characters (Chess‑Piece Tokens)
Represent player avatars as top‑down chess pieces. They should be recognizable silhouettes with distinct gear. Use a transparent background and slight shadow ring for contrast against any biome.

Adventurer Knight: “Top‑down chess‑piece style sprite of a medieval adventurer knight, distinct silhouette with sword and shield, flat aerial game‑board view, transparent background, 1024×1024, medieval fantasy, no perspective.”

Ranger/Archer: “Top‑down chess‑piece style sprite of a ranger with bow and quiver, hooded cloak and light armor, distinct silhouette, flat aerial view, transparent background, 1024×1024, medieval fantasy.”

Mage: “Top‑down chess‑piece style sprite of a wizard with staff and flowing robes, pointed hat or hood, distinct silhouette, flat aerial view, transparent background, 1024×1024, medieval fantasy.”

Healer: “Top‑down chess‑piece style sprite of a healer or priest carrying a staff topped with a sun symbol, wearing light robes, distinct silhouette, flat aerial view, transparent background, 1024×1024, medieval fantasy.”

If needed, create variants by changing equipment (axe instead of sword) or gendered silhouettes while keeping the same style.

Monsters (Chess‑Piece Tokens)
These tokens represent early‑game enemies. They should be menacing but easily readable. Use the same top‑down chess‑piece style as player characters.

Bandit: “Top‑down chess‑piece style token of a bandit with hood, sword or dagger drawn, ragged clothing, flat aerial view, transparent background, medieval fantasy.”

Wolf: “Top‑down chess‑piece style token of a snarling wolf, seen from directly above, fur texture visible, flat aerial view, transparent background, medieval fantasy.”

Goblin: “Top‑down chess‑piece style token of a goblin with spear and small shield, hunched posture, flat aerial view, transparent background, medieval fantasy.”

Thug/Brigand: “Top‑down chess‑piece style token of a muscular brigand wielding a club, wearing leather armor, flat aerial view, transparent background, medieval fantasy.”

Cultist (non‑corrupted): “Top‑down chess‑piece style token of a hooded cultist with dagger and dark robe, flat aerial view, transparent background, medieval fantasy, no supernatural effects.”

To create corrupted or void‑touched variants in later acts, append descriptors such as “void‑touched,” “glowing fissures,” or “mutated limbs” while keeping the top‑down chess‑piece style consistent.

Usage Notes
Consistency: Use the same camera angle and composition across all assets to ensure that biomes, overlays, features and tokens align correctly on the hex grid. Always include explicit instructions such as “flat aerial top‑down view,” “transparent background,” and “no perspective.”

Scale: Biomes are full textures. Paths, bridges and features occupy the central portion of the canvas with soft feathered edges to blend seamlessly into the underlying biome. Character and monster tokens are slightly smaller within the tile, with a subtle circular shadow to improve legibility.

Variation: To generate multiple variants (e.g., different taverns or road segments), adjust descriptive adjectives (colour of roof, state of repair) but keep the structural constraints unchanged. Avoid adding extra context (no sky, no horizon, no people) to maintain clarity.

Future Acts: Later prompt sheets (Levels 21–40, 41–60, etc.) will introduce corrupted, scorched, nightmare and void variants. To create those, start from these base prompts and add adjectives describing decay or eldritch effects while retaining the top‑down composition.



The transitions document standardizes every phase of the campaign. Each transition now has a core question, detailed emotional and mechanical goals, and separate sequences for the journey to the Labyrinth, the return journey after the dragon’s death, and the final sealing of the Void

newtab

. It covers all seven transitions across the three acts, filling in gaps and adding permanent consequences, revelations and tests that match the design themes

.

The Act 1 prompt sheet contains refined text prompts for biomes, paths, bridges, features, and chess‑piece‑style tokens for characters and monsters. All prompts enforce a flat aerial view, seamless edges, and consistent scale, ensuring they work together in your hex tile system

. The file also provides guidance on using these prompts and hints for generating variants and future act adaptations.

Let me know if you’d like similar packages for the later acts or if you have more features or creature types to include!




Sources



No file chosenNo file chosen
ChatGPT can make mistakes. OpenAI doesn't use FSC workspace data to train its models.
