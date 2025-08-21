/**
 * Load pre-generated content for Dragon's Labyrinth
 * This replaces runtime API calls with pre-generated content
 */

interface DialogueData {
  character: string;
  stage: number;
  context: string;
  lines: string[];
}

interface PreGeneratedContent {
  companionDialogues: Record<string, DialogueData>;
  npcDialogues: Record<string, DialogueData>;
  soundEffects: any[];
}

// Cache for loaded content
let contentCache: PreGeneratedContent | null = null;

/**
 * Load pre-generated content from JSON files
 * In production, these would be generated at build time
 */
export async function loadPreGeneratedContent(): Promise<PreGeneratedContent> {
  if (contentCache) {
    return contentCache;
  }
  
  try {
    // Try to load pre-generated content
    const [companionRes, npcRes, soundRes] = await Promise.all([
      fetch('/generated-content/companion-dialogues.json').catch(() => null),
      fetch('/generated-content/npc-dialogues.json').catch(() => null),
      fetch('/generated-content/sound-effects.json').catch(() => null)
    ]);
    
    if (companionRes && npcRes) {
      contentCache = {
        companionDialogues: await companionRes.json(),
        npcDialogues: await npcRes.json(),
        soundEffects: soundRes ? await soundRes.json() : []
      };
      
      console.log('Loaded pre-generated content');
      return contentCache;
    }
  } catch (error) {
    console.warn('Failed to load pre-generated content, using defaults', error);
  }
  
  // Return default content if pre-generated content is not available
  contentCache = getDefaultContent();
  return contentCache;
}

/**
 * Get dialogue for a specific character and stage
 */
export function getDialogue(character: string, stage: number): string[] {
  if (!contentCache) {
    return getDefaultDialogue(character, stage);
  }
  
  const key = `${character}_stage_${stage}`;
  const dialogue = contentCache.companionDialogues[key] || contentCache.npcDialogues[key];
  
  return dialogue?.lines || getDefaultDialogue(character, stage);
}

/**
 * Default content to use when pre-generated content is not available
 */
function getDefaultContent(): PreGeneratedContent {
  return {
    companionDialogues: {
      'einar_stage_0': {
        character: 'einar',
        stage: 0,
        context: 'Peace',
        lines: [
          "Feels good to stretch my legs. Think they'll need us back home soon?",
          "Remember the festival last year? We should hurry back for the next one.",
          "This bread smells amazing. Tamara really outdid herself!"
        ]
      },
      'einar_stage_1': {
        character: 'einar',
        stage: 1,
        context: 'Unease',
        lines: [
          "Did you hear that whisper? Probably just the wind... right?",
          "People are acting strange. Let's finish this errand quickly.",
          "Something feels off about this place."
        ]
      },
      'einar_stage_2': {
        character: 'einar',
        stage: 2,
        context: 'Dread',
        lines: [
          "They're all gone. Why are we still walking? The dragon is real, I know it.",
          "If this is what fate holds, maybe it's better to end it now...",
          "I can't sleep. Every shadow looks like claws."
        ]
      },
      'mira_stage_0': {
        character: 'mira',
        stage: 0,
        context: 'Peace',
        lines: [
          "This is amazing! I've never been this far from the market.",
          "We should collect some flowers on the way. They'll look lovely at home.",
          "What a beautiful day for an adventure!"
        ]
      },
      'mira_stage_1': {
        character: 'mira',
        stage: 1,
        context: 'Unease',
        lines: [
          "It's quiet... too quiet. But I'm sure it's nothing.",
          "Come on, let's make the best of it. Maybe it's a festival for whispers!",
          "Don't worry, everything will be fine. You'll see!"
        ]
      },
      'sorin_stage_0': {
        character: 'sorin',
        stage: 0,
        context: 'Peace',
        lines: [
          "Did you know ancient texts mention this path? Most take it for superstition.",
          "There is rational explanation for dragon myths — but I'm eager to find the truth.",
          "I've been documenting our journey. This will make fascinating research."
        ]
      },
      'tamara_stage_0': {
        character: 'tamara',
        stage: 0,
        context: 'Peace',
        lines: [
          "Do you think the customer will like our bread? I hope it's still warm.",
          "I've never seen so many trees! They're like a forest from a story.",
          "This is so exciting! My first real delivery!"
        ]
      }
    },
    npcDialogues: {
      'villager_stage_0': {
        character: 'villager',
        stage: 0,
        context: 'Peace',
        lines: [
          "Beautiful day, isn't it? The fields have never been greener.",
          "Fresh bread delivery? How wonderful!",
          "Safe travels, young ones!"
        ]
      },
      'merchant_stage_0': {
        character: 'merchant',
        stage: 0,
        context: 'Peace',
        lines: [
          "Fresh bread! Get your fresh bread! Oh, you're delivering some? Thank you for your help.",
          "Business is good today. People seem happy.",
          "The village is thriving. May it always be so."
        ]
      },
      'hollow_caretaker_stage_1': {
        character: 'hollow_caretaker',
        stage: 1,
        context: 'Unease',
        lines: [
          "The well... the well speaks... do you hear it?",
          "They left me here... watching... always watching...",
          "Come closer... let me show you what I've seen..."
        ]
      },
      'forsaken_knight_stage_2': {
        character: 'forsaken_knight',
        stage: 2,
        context: 'Dread',
        lines: [
          "I failed them... I failed them all...",
          "The dragon took everything. Now I guard nothing.",
          "Release me... or join me in this eternal vigil..."
        ]
      },
      'dragon_stage_4': {
        character: 'dragon',
        stage: 4,
        context: 'Horror',
        lines: [
          "You have walked so far to find what was always inside you.",
          "Every choice led you here. Every step was mine.",
          "Choose: Accept your fate, defy it, or understand it."
        ]
      }
    },
    soundEffects: [
      { name: 'whisper_unease', description: 'Soft whispers that grow louder', filename: 'whisper_unease.ogg' },
      { name: 'dragon_breath', description: 'Deep rumbling breath of the dragon', filename: 'dragon_breath.ogg' },
      { name: 'footsteps_labyrinth', description: 'Echoing footsteps in stone corridors', filename: 'footsteps_labyrinth.ogg' }
    ]
  };
}

/**
 * Get default dialogue for a character at a specific stage
 */
function getDefaultDialogue(character: string, stage: number): string[] {
  const stageNames = ['Peace', 'Unease', 'Dread', 'Terror', 'Horror'];
  const stageName = stageNames[Math.min(stage, 4)];
  
  // Return generic dialogue based on character and stage
  return [
    `[${character}]: The ${stageName} grows stronger...`,
    `[${character}]: We must continue our journey.`,
    `[${character}]: Something is changing in the world.`
  ];
}