export const questsData = [
  {
    id: "deliver_bread",
    stage: 0,
    description: "Deliver fresh bread to the farm across the field.",
    tasks: [
      "Pick up bread from Tamara's bakery.",
      "Walk through the meadow.",
      "Give bread to farmer Jorin."
    ],
    outcomes: {
      complete: "Farmer Jorin thanks you; Tamara joins your party.",
      fail: "If ignored, the bread goes stale, causing Tamara to fret."
    }
  },
  
  {
    id: "fix_fence",
    stage: 0,
    description: "Help Einar repair the fence around his farm.",
    tasks: [
      "Collect 5 wooden planks.",
      "Hammer planks into the posts."
    ],
    outcomes: {
      complete: "Einar joins your party, grateful for your help."
    }
  },
  
  {
    id: "guard_the_well",
    stage: 1,
    description: "Old man Daris asks you to guard the village well at night due to strange noises.",
    tasks: [
      "Stay by the well from dusk until dawn.",
      "Investigate whispers coming from the depths."
    ],
    choices: {
      investigate: "Climb down and find a Whispering Shade; confronting it grants knowledge but costs sanity.",
      ignore: "Refuse to investigate; gain suspicion from companions."
    }
  },
  
  {
    id: "close_the_gate",
    stage: 2,
    description: "Close the abandoned fort's gates to prevent beasts from entering the village.",
    tasks: [
      "Reach the fort through the swamp.",
      "Find the gate mechanism.",
      "Defend the mechanism from threats."
    ],
    outcomes: {
      complete: "Reduces beast encounters; triggers major boss encounter."
    }
  },
  
  {
    id: "labyrinth_escape",
    stage: 4,
    description: "Navigate the dragon's labyrinth, using sound cues and memory to progress.",
    tasks: [
      "Listen for dragon's breath to avoid encounters.",
      "Use clues from companions to follow the true path.",
      "Reach the heart of the labyrinth."
    ],
    outcomes: {
      completion: "Triggers the final encounter with the dragon."
    }
  }
];
