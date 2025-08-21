import { useEffect } from 'react';
import { useNarrative } from '../lib/stores/useNarrative';
import { useGameState } from '../lib/stores/useGameState';
import { useCompanions } from '../lib/stores/useCompanions';

export default function QuestSystem() {
  const { currentStage, currentQuest, completeQuest, startQuest } = useNarrative();
  const { playerPosition, adjustSanity } = useGameState();
  const { addCompanion, companions } = useCompanions();

  // Auto-start stage-appropriate quests
  useEffect(() => {
    if (!currentQuest) {
      switch (currentStage) {
        case 0: // Peace
          if (companions.length === 0) {
            startQuest({
              id: 'deliver_bread',
              description: 'Deliver fresh bread to the farm across the field.',
              tasks: [
                'Pick up bread from Tamara\'s bakery.',
                'Walk through the meadow.',
                'Give bread to farmer Jorin.'
              ],
              stage: 0
            });
            
            // Add initial companions
            addCompanion({
              id: 'tamara',
              name: 'Tamara',
              isActive: true,
              morale: 100,
              color: '#FFB74D'
            });
          }
          break;
          
        case 1: // Unease
          startQuest({
            id: 'guard_the_well',
            description: 'Old man Daris asks you to guard the village well at night due to strange noises.',
            tasks: [
              'Stay by the well from dusk until dawn.',
              'Investigate whispers coming from the depths.'
            ],
            stage: 1
          });
          break;
          
        case 2: // Dread
          startQuest({
            id: 'close_the_gate',
            description: 'Close the abandoned fort\'s gates to prevent beasts from entering the village.',
            tasks: [
              'Reach the fort through the swamp.',
              'Find the gate mechanism.',
              'Defend the mechanism from threats.'
            ],
            stage: 2
          });
          break;
          
        case 3: // Terror
          startQuest({
            id: 'choose_companion',
            description: 'You must choose which companion to support at a critical moment.',
            tasks: [
              'Decide whom to save.'
            ],
            stage: 3
          });
          break;
          
        case 4: // Horror
          startQuest({
            id: 'labyrinth_escape',
            description: 'Navigate the dragon\'s labyrinth using sound cues and memory.',
            tasks: [
              'Listen for dragon\'s breath to avoid encounters.',
              'Use clues to follow the true path.',
              'Reach the heart of the labyrinth.'
            ],
            stage: 4
          });
          break;
      }
    }
  }, [currentStage, currentQuest, startQuest, addCompanion, companions.length]);

  // Check quest completion conditions
  useEffect(() => {
    if (!currentQuest) return;

    switch (currentQuest.id) {
      case 'deliver_bread':
        // Complete after moving a few tiles
        const distance = Math.sqrt(playerPosition.q * playerPosition.q + playerPosition.r * playerPosition.r);
        if (distance > 3) {
          completeQuest();
          adjustSanity(5); // Small sanity boost for completing peaceful quest
          
          // Add Einar as companion
          addCompanion({
            id: 'einar',
            name: 'Einar',
            isActive: true,
            morale: 90,
            color: '#8BC34A'
          });
        }
        break;
        
      case 'guard_the_well':
        // Complete after staying in center for a while
        if (playerPosition.q === 0 && playerPosition.r === 0) {
          setTimeout(() => {
            completeQuest();
            adjustSanity(-10); // Sanity loss from witnessing strange events
          }, 5000);
        }
        break;
        
      case 'close_the_gate':
        // Complete when reaching edge of world
        const edgeDistance = Math.sqrt(playerPosition.q * playerPosition.q + playerPosition.r * playerPosition.r);
        if (edgeDistance > 6) {
          completeQuest();
          adjustSanity(-15); // More sanity loss
        }
        break;
    }
  }, [currentQuest, playerPosition, completeQuest, adjustSanity, addCompanion]);

  return null; // This component only handles logic, no rendering
}
