import OpenAI from "openai";

// the newest OpenAI model is "gpt-4o" which was released May 13, 2024. do not change this unless explicitly requested by the user
const openai = new OpenAI({ apiKey: process.env.OPENAI_API_KEY });

export async function generateNPCDialogue(
  npcType: string,
  playerContext: {
    stage: number;
    sanity: number;
    recentActions: string[];
  }
): Promise<string> {
  const stageDescriptions = [
    "peaceful village life",
    "growing unease and strange occurrences",
    "dread and decay spreading",
    "terror grips the land",
    "pure horror in the dragon's domain"
  ];

  try {
    const response = await openai.chat.completions.create({
      model: "gpt-4o",
      messages: [
        {
          role: "system",
          content: `You are generating dialogue for an NPC in a horror RPG. The game is in stage ${playerContext.stage} (${stageDescriptions[playerContext.stage]}). 
          The player's sanity is at ${playerContext.sanity}%.
          Keep responses brief (1-2 sentences), atmospheric, and progressively more unsettling as the stage increases.
          NPC type: ${npcType}`
        },
        {
          role: "user",
          content: `Generate a single line of dialogue for this NPC that reflects the current horror stage and player's condition.`
        }
      ],
      max_tokens: 100,
      temperature: 0.8,
    });

    return response.choices[0].message.content || "...";
  } catch (error) {
    console.error("OpenAI dialogue generation failed:", error);
    return getFallbackDialogue(npcType, playerContext.stage);
  }
}

export async function generateQuestDescription(
  questType: string,
  stage: number
): Promise<{ title: string; description: string; objective: string }> {
  try {
    const response = await openai.chat.completions.create({
      model: "gpt-4o",
      messages: [
        {
          role: "system",
          content: `Generate a quest for a horror RPG. Stage ${stage}/4 (0=peace, 4=horror).
          Quest type: ${questType}. Make it progressively darker based on stage.
          Respond in JSON format with keys: title, description, objective`
        },
        {
          role: "user",
          content: "Generate quest details"
        }
      ],
      response_format: { type: "json_object" },
      max_tokens: 200,
    });

    const result = JSON.parse(response.choices[0].message.content || "{}");
    return {
      title: result.title || "Unknown Quest",
      description: result.description || "Something needs to be done.",
      objective: result.objective || "Complete the task"
    };
  } catch (error) {
    console.error("Quest generation failed:", error);
    return {
      title: `${questType} Quest`,
      description: "A task awaits you.",
      objective: "Complete the quest"
    };
  }
}

export async function analyzeSanityEvent(
  event: string,
  currentSanity: number
): Promise<{ sanityChange: number; hallucination?: string }> {
  try {
    const response = await openai.chat.completions.create({
      model: "gpt-4o",
      messages: [
        {
          role: "system",
          content: `Analyze a horror game event and determine sanity impact.
          Current sanity: ${currentSanity}%. 
          Return JSON with: sanityChange (negative number), hallucination (optional string if sanity < 50)`
        },
        {
          role: "user",
          content: `Event: ${event}`
        }
      ],
      response_format: { type: "json_object" },
      max_tokens: 150,
    });

    return JSON.parse(response.choices[0].message.content || '{"sanityChange": -5}');
  } catch (error) {
    console.error("Sanity analysis failed:", error);
    return { sanityChange: -5 };
  }
}

function getFallbackDialogue(npcType: string, stage: number): string {
  const dialogues = {
    villager: [
      "What a lovely day for bread delivery!",
      "Have you noticed the shadows seem longer lately?",
      "The crops... they're withering. Something's wrong.",
      "Don't go out after dark. Please.",
      "We're all going to die here..."
    ],
    merchant: [
      "Fresh goods from the capital!",
      "My suppliers have been... unreliable lately.",
      "I'm running low on everything. People are hoarding.",
      "Gold won't save us now.",
      "Take what you need. Money is meaningless now."
    ],
    guard: [
      "Move along, citizen.",
      "Stay alert. We've had... incidents.",
      "Lock your doors at night. That's an order.",
      "We can't hold them back much longer.",
      "Run. RUN WHILE YOU STILL CAN!"
    ]
  };

  const npcDialogues = dialogues[npcType as keyof typeof dialogues] || dialogues.villager;
  return npcDialogues[Math.min(stage, npcDialogues.length - 1)];
}