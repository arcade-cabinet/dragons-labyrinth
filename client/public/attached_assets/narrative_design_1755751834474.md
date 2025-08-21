# Dragon’s Labyrinth – Narrative Design

## Introduction

**Dragon’s Labyrinth** is a horror‑first RPG disguised as a bright adventure.  Replit acts as the sole developer, designer and orchestrator, responsible for creating all code, assets and narrative logic.  The game follows a strict emotional arc of **Peace → Unease → Dread → Terror → Horror**, with each stage altering every system: biomes, quests, companions, NPC behaviour, economy, combat and user interface.  The world begins cheerful and slowly decays as the player moves closer to the dragon’s labyrinth.

## Emotional Arc and Stage Overview

### Peace (Stage 0)
* The player leaves home on a sunny morning.  The world is vibrant and the music uplifting.
* Quests are mundane and NPCs are friendly.
* Companions introduce themselves and establish bonds.
* Biomes include grasslands, villages, markets and farms.
* There is no combat or boss encounter at this stage.

### Unease (Stage 1)
* Shadows lengthen; birds stop singing.  Whispering voices replace cheerful sounds.
* NPCs avoid eye contact and begin to whisper.  Quests take on a strange tone.
* Companions grow uneasy but remain with the player.
* Biomes include forests, small towns and crypt outskirts.
* **The Hollow Caretaker** is the stage boss, presenting choices of persuasion versus violence.

### Dread (Stage 2)
* Corpse‑laden swamps and ruins become common; abandoned forts loom.
* The economy collapses: gold becomes worthless, survival items are scarce.
* Companions argue and display trauma.  Survival becomes the focus.
* Biomes include swamps, ruins, abandoned forts and caverns.
* **The Forsaken Knight** is the boss, offering empathy versus brutality.

### Terror (Stage 3)
* Reality warps: ghost towns, distorted cities and mirror lakes challenge perception.
* Quests force moral horrors, and hallucinations distort the senses.
* One companion betrays the party under the dragon’s influence.
* Biomes include ghost towns, warped cities, mirror lakes and the labyrinth outskirts.
* **The Traitor Companion** serves as the boss, posing forgiveness versus execution.

### Horror (Stage 4)
* The player enters the dragon’s labyrinth in full first‑person.
* The dragon stalks the player by sound alone; hallucinations peak.
* Companions may die, disappear or turn on the player.
* The labyrinth’s geometry twists endlessly.
* **The Dragon** itself is the final encounter, with acceptance, defiance or understanding endings.

## Systems and Mechanics

* **Narrative Integration** – Every system responds to the current dread level.  NPCs gradually stop talking, shops close, items lose value and companions’ behaviour changes.
* **Sanity System** – Hallucinations and false sound cues undermine trust in the senses.
* **Proximity Horror** – The dragon’s presence is conveyed through sound and environment, rather than direct combat until the final stage.
* **Choice and Consequence** – Boss encounters and major quests offer branching decisions that influence subsequent stages and the ending.
* **Component‑based Implementation** – Each system (biome generator, quest engine, companion AI, etc.) remains small and focused, but is orchestrated by the narrative logic.
