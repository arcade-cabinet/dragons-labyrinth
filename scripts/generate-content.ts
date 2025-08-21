#!/usr/bin/env tsx
/**
 * Pre-generate all dynamic content for Dragon's Labyrinth
 * This script runs at build time to generate all dialogues, sounds, and other dynamic content
 * Uses checksums to avoid regenerating existing content
 */

import fs from 'fs';
import path from 'path';
import crypto from 'crypto';
import OpenAI from 'openai';

const CONTENT_DIR = path.join(process.cwd(), 'generated-content');
const CHECKSUM_FILE = path.join(CONTENT_DIR, 'checksums.json');

// Content types to generate
interface DialogueData {
  character: string;
  stage: number;
  context: string;
  lines: string[];
}

interface SoundEffect {
  name: string;
  description: string;
  filename: string;
}

interface GeneratedContent {
  dialogues: Record<string, DialogueData>;
  soundEffects: SoundEffect[];
  narrativeEvents: any[];
  checksums: Record<string, string>;
}

// Initialize OpenAI client
const openai = new OpenAI({ 
  apiKey: process.env.OPENAI_API_KEY 
});

// Calculate checksum for content
function calculateChecksum(content: any): string {
  const hash = crypto.createHash('sha256');
  hash.update(JSON.stringify(content));
  return hash.digest('hex');
}

// Load existing checksums
function loadChecksums(): Record<string, string> {
  if (fs.existsSync(CHECKSUM_FILE)) {
    return JSON.parse(fs.readFileSync(CHECKSUM_FILE, 'utf-8'));
  }
  return {};
}

// Save checksums
function saveChecksums(checksums: Record<string, string>) {
  fs.writeFileSync(CHECKSUM_FILE, JSON.stringify(checksums, null, 2));
}

// Generate companion dialogues using OpenAI
async function generateCompanionDialogues(): Promise<Record<string, DialogueData>> {
  const companions = ['einar', 'mira', 'sorin', 'tamara'];
  const stages = [0, 1, 2, 3, 4]; // Peace through Horror
  const dialogues: Record<string, DialogueData> = {};
  
  for (const companion of companions) {
    for (const stage of stages) {
      const key = `${companion}_stage_${stage}`;
      
      // Generate context-aware dialogue
      const prompt = `Generate 5 dialogue lines for ${companion} in Dragon's Labyrinth during stage ${stage} (${['Peace', 'Unease', 'Dread', 'Terror', 'Horror'][stage]}). 
        Character traits:
        - Einar: Loyal friend, becomes paranoid
        - Mira: Optimist who abandons party
        - Sorin: Scholar who may betray
        - Tamara: Innocent baker's apprentice
        
        Return JSON: { "lines": ["line1", "line2", ...] }`;
      
      try {
        const response = await openai.chat.completions.create({
          model: "gpt-4o",
          messages: [{ role: "user", content: prompt }],
          response_format: { type: "json_object" }
        });
        
        const result = JSON.parse(response.choices[0].message.content || '{}');
        
        dialogues[key] = {
          character: companion,
          stage,
          context: ['Peace', 'Unease', 'Dread', 'Terror', 'Horror'][stage],
          lines: result.lines || []
        };
        
        console.log(`Generated dialogue for ${companion} at stage ${stage}`);
      } catch (error) {
        console.error(`Failed to generate dialogue for ${companion} at stage ${stage}:`, error);
      }
    }
  }
  
  return dialogues;
}

// Generate NPC dialogues
async function generateNPCDialogues(): Promise<Record<string, DialogueData>> {
  const npcs = [
    { id: 'villager', stages: [0] },
    { id: 'merchant', stages: [0, 1] },
    { id: 'hollow_caretaker', stages: [1] },
    { id: 'forsaken_knight', stages: [2] },
    { id: 'swamp_hermit', stages: [2] },
    { id: 'ghost_town_echo', stages: [3] },
    { id: 'dragon', stages: [4] }
  ];
  
  const dialogues: Record<string, DialogueData> = {};
  
  for (const npc of npcs) {
    for (const stage of npc.stages) {
      const key = `${npc.id}_stage_${stage}`;
      
      const prompt = `Generate 3 dialogue lines for ${npc.id} in Dragon's Labyrinth during stage ${stage}. 
        Context: Horror RPG where world transitions from bright Peace to dark Horror.
        Return JSON: { "lines": ["line1", "line2", "line3"] }`;
      
      try {
        const response = await openai.chat.completions.create({
          model: "gpt-4o",
          messages: [{ role: "user", content: prompt }],
          response_format: { type: "json_object" }
        });
        
        const result = JSON.parse(response.choices[0].message.content || '{}');
        
        dialogues[key] = {
          character: npc.id,
          stage,
          context: ['Peace', 'Unease', 'Dread', 'Terror', 'Horror'][stage],
          lines: result.lines || []
        };
        
        console.log(`Generated dialogue for ${npc.id} at stage ${stage}`);
      } catch (error) {
        console.error(`Failed to generate dialogue for ${npc.id} at stage ${stage}:`, error);
      }
    }
  }
  
  return dialogues;
}

// Generate sound descriptions (actual sound generation would require different API)
function generateSoundEffects(): SoundEffect[] {
  // For now, just generate metadata for sounds
  // Actual sound files would be generated with a different service
  return [
    { name: 'whisper_unease', description: 'Soft whispers that grow louder', filename: 'whisper_unease.ogg' },
    { name: 'dragon_breath', description: 'Deep rumbling breath of the dragon', filename: 'dragon_breath.ogg' },
    { name: 'footsteps_labyrinth', description: 'Echoing footsteps in stone corridors', filename: 'footsteps_labyrinth.ogg' },
    { name: 'companion_scream', description: 'Terrified scream of a companion', filename: 'companion_scream.ogg' },
    { name: 'ambient_peace', description: 'Birds chirping and gentle breeze', filename: 'ambient_peace.ogg' },
    { name: 'ambient_dread', description: 'Swamp sounds with distant growls', filename: 'ambient_dread.ogg' },
    { name: 'ambient_horror', description: 'Heartbeat and labyrinth echoes', filename: 'ambient_horror.ogg' }
  ];
}

// Main generation function
async function generateContent() {
  console.log('Starting content generation for Dragon\'s Labyrinth...');
  
  // Create content directory
  if (!fs.existsSync(CONTENT_DIR)) {
    fs.mkdirSync(CONTENT_DIR, { recursive: true });
  }
  
  const existingChecksums = loadChecksums();
  const newChecksums: Record<string, string> = {};
  
  // Generate companion dialogues
  console.log('\n=== Generating Companion Dialogues ===');
  const companionDialoguesFile = path.join(CONTENT_DIR, 'companion-dialogues.json');
  const companionChecksum = existingChecksums['companion-dialogues'];
  
  let companionDialogues: Record<string, DialogueData> = {};
  if (companionChecksum && fs.existsSync(companionDialoguesFile)) {
    console.log('Companion dialogues already generated, skipping...');
    companionDialogues = JSON.parse(fs.readFileSync(companionDialoguesFile, 'utf-8'));
  } else {
    companionDialogues = await generateCompanionDialogues();
    fs.writeFileSync(companionDialoguesFile, JSON.stringify(companionDialogues, null, 2));
    newChecksums['companion-dialogues'] = calculateChecksum(companionDialogues);
  }
  
  // Generate NPC dialogues
  console.log('\n=== Generating NPC Dialogues ===');
  const npcDialoguesFile = path.join(CONTENT_DIR, 'npc-dialogues.json');
  const npcChecksum = existingChecksums['npc-dialogues'];
  
  let npcDialogues: Record<string, DialogueData> = {};
  if (npcChecksum && fs.existsSync(npcDialoguesFile)) {
    console.log('NPC dialogues already generated, skipping...');
    npcDialogues = JSON.parse(fs.readFileSync(npcDialoguesFile, 'utf-8'));
  } else {
    npcDialogues = await generateNPCDialogues();
    fs.writeFileSync(npcDialoguesFile, JSON.stringify(npcDialogues, null, 2));
    newChecksums['npc-dialogues'] = calculateChecksum(npcDialogues);
  }
  
  // Generate sound effect metadata
  console.log('\n=== Generating Sound Effect Metadata ===');
  const soundEffectsFile = path.join(CONTENT_DIR, 'sound-effects.json');
  const soundEffects = generateSoundEffects();
  fs.writeFileSync(soundEffectsFile, JSON.stringify(soundEffects, null, 2));
  newChecksums['sound-effects'] = calculateChecksum(soundEffects);
  
  // Create final manifest
  const manifest = {
    generated: new Date().toISOString(),
    dialogueCount: Object.keys({...companionDialogues, ...npcDialogues}).length,
    soundEffectCount: soundEffects.length,
    files: [
      'companion-dialogues.json',
      'npc-dialogues.json', 
      'sound-effects.json'
    ]
  };
  
  fs.writeFileSync(
    path.join(CONTENT_DIR, 'manifest.json'),
    JSON.stringify(manifest, null, 2)
  );
  
  // Save checksums
  saveChecksums({...existingChecksums, ...newChecksums});
  
  console.log('\n=== Content Generation Complete ===');
  console.log(`Generated ${manifest.dialogueCount} dialogue sets`);
  console.log(`Generated ${manifest.soundEffectCount} sound effect definitions`);
  console.log(`Content saved to: ${CONTENT_DIR}`);
}

// Run if called directly
generateContent().catch(console.error);

export { generateContent };