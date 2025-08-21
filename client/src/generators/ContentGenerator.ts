// AI-powered content generator for Dragon's Labyrinth
// Generates .cob UI scenes, .yol levels, and ECS components

interface DreadLevel {
  level: number; // 0-4 (Peace to Horror)
  name: string;
  description: string;
  colorScheme: {
    primary: string;
    secondary: string;
    background: string;
  };
}

export class ContentGenerator {
  private dreadLevels: DreadLevel[] = [
    {
      level: 0,
      name: "Peace",
      description: "Bright world, mundane quests, friendly NPCs",
      colorScheme: { primary: "#4ade80", secondary: "#86efac", background: "#f0fdf4" }
    },
    {
      level: 1,
      name: "Unease",
      description: "Shadows, whispers, Hollow Caretaker boss",
      colorScheme: { primary: "#facc15", secondary: "#fde047", background: "#fefce8" }
    },
    {
      level: 2,
      name: "Dread",
      description: "Swamps, ruins, economy collapse, Forsaken Knight boss",
      colorScheme: { primary: "#f97316", secondary: "#fb923c", background: "#fff7ed" }
    },
    {
      level: 3,
      name: "Terror",
      description: "Reality warps, companion betrayal, moral horrors",
      colorScheme: { primary: "#ef4444", secondary: "#f87171", background: "#fef2f2" }
    },
    {
      level: 4,
      name: "Horror",
      description: "Dragon's labyrinth, stalking mechanics, final choice",
      colorScheme: { primary: "#991b1b", secondary: "#dc2626", background: "#1f2937" }
    }
  ];

  // Generate Cobweb UI scene (.cob format)
  generateCobScene(dreadLevel: number): string {
    const dread = this.dreadLevels[dreadLevel];
    return JSON.stringify({
      format: "cob_scene",
      version: "1.0",
      dread_level: dreadLevel,
      theme: dread.name.toLowerCase(),
      ui_elements: [
        {
          type: "health_bar",
          style: {
            color: dread.colorScheme.primary,
            width: "200px",
            animated: dreadLevel >= 3
          }
        },
        ...(dreadLevel >= 1 ? [{
          type: "sanity_meter",
          style: {
            color: dread.colorScheme.secondary,
            shake_intensity: dreadLevel * 0.2
          }
        }] : []),
        ...(dreadLevel >= 2 ? [{
          type: "corruption_overlay",
          opacity: 0.1 + (dreadLevel * 0.15)
        }] : [])
      ]
    }, null, 2);
  }

  // Generate Yoleck level data (.yol format)
  generateYolLevel(biome: string, dreadLevel: number): string {
    const entities = [];
    const radius = 15;
    
    // Generate hex tiles
    for (let q = -radius; q <= radius; q++) {
      for (let r = -radius; r <= radius; r++) {
        const s = -q - r;
        if (Math.abs(s) > radius) continue;
        
        if (Math.random() > 0.15) { // 85% chance of tile existing
          entities.push({
            name: "HexTile",
            components: {
              position: { q, r },
              tile_type: this.getTileType(biome, dreadLevel),
              corruption: dreadLevel / 4,
              passable: Math.random() > 0.1 + (dreadLevel * 0.05)
            }
          });
        }
      }
    }
    
    // Add NPCs (fewer as dread increases)
    const npcCount = Math.max(0, 8 - (dreadLevel * 2));
    for (let i = 0; i < npcCount; i++) {
      entities.push(this.generateNPC(dreadLevel));
    }
    
    // Add monsters (more as dread increases)
    const monsterCount = dreadLevel * 4;
    for (let i = 0; i < monsterCount; i++) {
      entities.push(this.generateMonster(biome, dreadLevel));
    }
    
    return JSON.stringify({
      format_version: 3,
      app_format_version: 1,
      biome,
      dread_level: dreadLevel,
      entities
    }, null, 2);
  }

  // Generate ECS prefabs for procedural encounters
  generateECSPrefabs(dreadLevel: number): any {
    return {
      monsters: this.generateMonsterPrefabs(dreadLevel),
      npcs: this.generateNPCPrefabs(dreadLevel),
      items: this.generateItemPrefabs(dreadLevel),
      environmental: this.generateEnvironmentalPrefabs(dreadLevel)
    };
  }

  // Generate dialogue for Yarn Spinner (.yarn format)
  generateDialogue(companion: string, dreadLevel: number): string {
    const dialogues: Record<string, Record<number, string[]>> = {
      Einar: {
        0: ["What a beautiful day for adventure!", "I've got your back, friend."],
        1: ["Something feels... wrong today.", "Did you hear that?"],
        2: ["We should turn back. This isn't right.", "I... I can't shake this feeling."],
        3: ["I can't... I can't do this anymore!", "Why did we come here?!"],
        4: ["*screaming*", "*sobbing uncontrollably*"]
      },
      Mira: {
        0: ["The sun feels so warm!", "Everything's going to be fine!"],
        1: ["That shadow... did it move?", "I'm sure it's nothing..."],
        2: ["I need to go. I'm sorry.", "*runs away*"],
        3: ["*absent*", "*absent*"],
        4: ["*absent*", "*absent*"]
      },
      Sorin: {
        0: ["Knowledge is power.", "Let me examine this..."],
        1: ["These patterns... they're wrong.", "The books warned of this."],
        2: ["The old texts spoke of horrors.", "We're not prepared for this."],
        3: ["Join me or die!", "The dragon promised power!"],
        4: ["*boss fight*", "*transformed*"]
      },
      Tamara: {
        0: ["Fresh bread for everyone!", "The bakery is so peaceful."],
        1: ["The ovens won't light properly...", "The dough... it's wrong."],
        2: ["The bread... it screams when I bake it.", "I can't stop baking..."],
        3: ["*missing*", "*missing*"],
        4: ["*found as corpse*", "*found as corpse*"]
      }
    };

    const lines = dialogues[companion]?.[dreadLevel] || ["..."];
    
    return `title: ${companion}_${dreadLevel}
tags: companion
---
<<if $dread_level == ${dreadLevel}>>
${companion}: ${lines[Math.floor(Math.random() * lines.length)]}
-> [Continue]
-> [Comfort them] <<if $sanity > 50>>
-> [Push them away] <<if $dread_level >= 2>>
<<endif>>
===`;
  }

  private getTileType(biome: string, dreadLevel: number): string {
    const corrupted = Math.random() < (dreadLevel * 0.2);
    if (corrupted) return "corrupted";
    
    const biomeTypes: Record<string, string> = {
      meadow: dreadLevel < 2 ? "grass" : "withered_grass",
      forest: dreadLevel < 3 ? "forest_floor" : "dead_leaves",
      swamp: "murky_water",
      ruins: "cracked_stone",
      labyrinth: "ancient_stone"
    };
    
    return biomeTypes[biome] || "dirt";
  }

  private generateNPC(dreadLevel: number): any {
    const npcTypes = [
      ["Merchant", "Guard", "Villager", "Child"],
      ["Nervous_Guard", "Worried_Merchant", "Hiding_Villager"],
      ["Desperate_Survivor", "Mad_Prophet", "Fleeing_Guard"],
      ["Gibbering_Madman", "Shadow_Person"],
      ["Echo", "Memory"]
    ];
    
    const types = npcTypes[Math.min(dreadLevel, 4)];
    
    return {
      name: "NPC",
      components: {
        position: { 
          q: Math.floor(Math.random() * 20) - 10,
          r: Math.floor(Math.random() * 20) - 10
        },
        npc_type: types[Math.floor(Math.random() * types.length)],
        sanity: 100 - (dreadLevel * 25),
        dialogue_tree: `npc_${dreadLevel}_${Math.floor(Math.random() * 3)}`,
        flee_threshold: dreadLevel * 20
      }
    };
  }

  private generateMonster(biome: string, dreadLevel: number): any {
    const monsterTypes: Record<number, string[]> = {
      0: ["Shadow_Rabbit", "Wrong_Bird"],
      1: ["Whispering_Tree", "Hollow_Deer"],
      2: ["Bog_Wraith", "Faceless_Walker"],
      3: ["Screaming_Void", "Reality_Tear"],
      4: ["Dragon_Echo", "Nightmare_Fragment"]
    };
    
    const types = monsterTypes[dreadLevel];
    
    return {
      name: "Monster",
      components: {
        position: {
          q: Math.floor(Math.random() * 30) - 15,
          r: Math.floor(Math.random() * 30) - 15
        },
        monster_type: types[Math.floor(Math.random() * types.length)],
        health: 50 + (dreadLevel * 30),
        damage: 5 + (dreadLevel * 10),
        detection_radius: 3 + dreadLevel,
        move_speed: 0.5 + (dreadLevel * 0.2),
        behavior: this.getMonsterBehavior(dreadLevel)
      }
    };
  }

  private getMonsterBehavior(dreadLevel: number): string {
    const behaviors = ["Observe", "Follow", "Stalk", "Hunt", "Relentless"];
    return behaviors[Math.min(dreadLevel, 4)];
  }

  private generateMonsterPrefabs(dreadLevel: number): any[] {
    return [
      {
        id: "shadow_rabbit",
        name: "Shadow Rabbit",
        health: 10 + (dreadLevel * 5),
        damage: 1 + dreadLevel,
        min_dread: 0
      },
      {
        id: "whispering_tree",
        name: "Whispering Tree",
        health: 50 + (dreadLevel * 10),
        damage: 5 + (dreadLevel * 3),
        min_dread: 1
      },
      {
        id: "bog_wraith",
        name: "Bog Wraith",
        health: 100 + (dreadLevel * 20),
        damage: 15 + (dreadLevel * 5),
        min_dread: 2
      }
    ].filter(m => m.min_dread <= dreadLevel);
  }

  private generateNPCPrefabs(dreadLevel: number): any[] {
    return [
      {
        id: "villager",
        name: "Villager",
        dialogue_tree: "villager_default",
        sanity: Math.max(20, 100 - (dreadLevel * 20))
      },
      {
        id: "merchant",
        name: "Merchant",
        dialogue_tree: "merchant_default",
        sanity: Math.max(40, 100 - (dreadLevel * 15))
      }
    ];
  }

  private generateItemPrefabs(dreadLevel: number): any[] {
    return [
      { id: "bread", name: "Bread", corrupted: dreadLevel >= 2 },
      { id: "torch", name: "Torch", durability: 100 - (dreadLevel * 20) },
      { id: "compass", name: "Compass", broken: dreadLevel >= 3 }
    ];
  }

  private generateEnvironmentalPrefabs(dreadLevel: number): any[] {
    return [
      { 
        id: "fog", 
        density: 0.1 + (dreadLevel * 0.2),
        color: dreadLevel < 3 ? "#ffffff" : "#ff0000"
      },
      {
        id: "ambient_sound",
        type: dreadLevel < 2 ? "birds" : dreadLevel < 4 ? "whispers" : "screams",
        volume: 0.3 + (dreadLevel * 0.1)
      }
    ];
  }
}