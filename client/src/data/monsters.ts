export const monstersData = {
  whispering_shade: {
    name: "Whispering Shade",
    stages: [1, 2, 3],
    description: "Invisible entities that linger just outside the player's perception. They whisper secrets and lies, distorting audio and causing paranoia.",
    behaviors: [
      "Whispers random words to simulate voices of lost villagers.",
      "Inflicts minor sanity damage when ignored.",
      "Flees when confronted with light sources or loud sounds."
    ]
  },
  
  hollow_npc: {
    name: "Hollow NPC",
    stages: [2, 3],
    description: "Former villagers whose bodies continue simple routines without a soul. They repeat fragmented sentences with no awareness.",
    behaviors: [
      "Repeats lines from Peace stage NPCs out of context.",
      "Ignores the player unless attacked.",
      "Falls apart when confronted, releasing a cloud of despair."
    ]
  },
  
  forsaken_knight: {
    name: "Forsaken Knight",
    stages: [2],
    description: "A once-noble knight bound in rusted armour. He wanders ruins, searching for release. He serves as the boss of Stage 2.",
    behaviors: [
      "Patrols a narrow corridor, muttering vows and regrets.",
      "Attacks with heavy sword swings and shield bashes.",
      "Stops to speak when the player approaches unarmed."
    ],
    choices: {
      empathy: "Listen to his memories, free his soul; player gains a relic that restores sanity but increases dragon proximity.",
      brutality: "Defeat him violently; obtain cursed armour that whispers to the player, granting defence but causing hallucinations."
    }
  },
  
  dragon: {
    name: "The Dragon",
    stages: [4],
    description: "The ancient intelligence that hunts the player in the labyrinth. It is not merely a creature but an embodiment of inevitability.",
    behaviors: [
      "Stalks the player using audio cues (footsteps, breath, silence).",
      "Manipulates labyrinth geometry to mislead and trap.",
      "Confrontation triggers endings based on understanding, defiance or acceptance."
    ],
    choices: {
      acceptance: "Merge with the dragon; cycle continues.",
      defiance: "Destroy yourself; deny the dragon your essence.",
      understanding: "Solve labyrinth's truth; free yourself and the dragon's victims."
    }
  }
};
