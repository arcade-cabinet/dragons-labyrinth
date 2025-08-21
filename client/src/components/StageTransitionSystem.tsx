import { useState, useEffect } from 'react';
import { useNarrative } from '../lib/stores/useNarrative';
import { useGameState } from '../lib/stores/useGameState';
import { useCompanions } from '../lib/stores/useCompanions';

export default function StageTransitionSystem() {
  const { currentStage, triggerStageTransition } = useNarrative();
  const { playerPosition, sanity, adjustSanity } = useGameState();
  const { companions, updateCompanionMorale, setCompanionActive, addCompanion } = useCompanions();
  const [transitionTimer, setTransitionTimer] = useState(0);
  const [showTransition, setShowTransition] = useState(false);
  const [transitionText, setTransitionText] = useState('');

  // Auto-progression based on time and conditions
  useEffect(() => {
    const timer = setInterval(() => {
      setTransitionTimer(prev => prev + 1);
    }, 1000);

    return () => clearInterval(timer);
  }, []);

  // Check for stage transition conditions
  useEffect(() => {
    const distance = Math.sqrt(playerPosition.q * playerPosition.q + playerPosition.r * playerPosition.r);
    let shouldTransition = false;
    let triggerText = '';

    switch (currentStage) {
      case 0: // Peace -> Unease
        if (transitionTimer > 30 || distance > 4) { // 30 seconds or moving far from center
          shouldTransition = true;
          triggerText = "The world grows quieter. Birds have stopped singing.";
          // Add Einar as companion if not present
          if (!companions.find(c => c.id === 'einar')) {
            addCompanion({
              id: 'einar',
              name: 'Einar',
              isActive: true,
              morale: 85,
              color: '#8BC34A',
              archetype: 'loyal_friend'
            });
          }
        }
        break;
        
      case 1: // Unease -> Dread  
        if (transitionTimer > 60 || sanity < 80) {
          shouldTransition = true;
          triggerText = "The air grows thick with dread. Something is very wrong.";
          // Reduce all companion morale
          companions.forEach(c => updateCompanionMorale(c.id, -20));
          adjustSanity(-10);
        }
        break;
        
      case 2: // Dread -> Terror
        if (transitionTimer > 90 || sanity < 60) {
          shouldTransition = true;
          triggerText = "Reality begins to warp. The dragon's influence grows stronger.";
          // Mira abandons the party in Dread stage
          const mira = companions.find(c => c.name === 'Mira');
          if (mira) {
            setCompanionActive(mira.id, false);
          }
          adjustSanity(-15);
        }
        break;
        
      case 3: // Terror -> Horror
        if (transitionTimer > 120 || sanity < 40 || distance > 7) {
          shouldTransition = true;
          triggerText = "You enter the dragon's labyrinth. There is no turning back.";
          // Massive sanity loss
          adjustSanity(-20);
          // All remaining companions show extreme trauma
          companions.forEach(c => {
            if (c.isActive) {
              updateCompanionMorale(c.id, -30);
            }
          });
        }
        break;
    }

    if (shouldTransition && currentStage < 4) {
      handleStageTransition(triggerText);
    }
  }, [transitionTimer, playerPosition, currentStage, sanity, companions]);

  // Handle the actual stage transition with visual effects
  const handleStageTransition = (text: string) => {
    setTransitionText(text);
    setShowTransition(true);
    
    // Transition after showing text
    setTimeout(() => {
      triggerStageTransition();
      setTransitionTimer(0); // Reset timer for next stage
      setShowTransition(false);
    }, 3000);
  };

  // Apply stage-specific effects to companions
  useEffect(() => {
    switch (currentStage) {
      case 1: // Unease - companions grow worried
        companions.forEach(c => {
          if (c.morale > 70) {
            updateCompanionMorale(c.id, -5);
          }
        });
        break;
        
      case 2: // Dread - companions show trauma
        companions.forEach(c => {
          if (c.morale > 50) {
            updateCompanionMorale(c.id, -10);
          }
        });
        break;
        
      case 3: // Terror - companion betrayal mechanics
        const sorin = companions.find(c => c.name === 'Sorin' || c.archetype === 'scholar');
        if (sorin && sorin.morale < 30 && sorin.isActive) {
          // Sorin becomes potential traitor
          updateCompanionMorale(sorin.id, -20);
        }
        break;
        
      case 4: // Horror - extreme effects
        companions.forEach(c => {
          if (c.isActive && c.morale > 20) {
            updateCompanionMorale(c.id, -15);
          }
        });
        break;
    }
  }, [currentStage]);

  return showTransition ? (
    <div
      style={{
        position: 'absolute',
        top: '0',
        left: '0',
        width: '100%',
        height: '100%',
        backgroundColor: 'rgba(0, 0, 0, 0.9)',
        display: 'flex',
        justifyContent: 'center',
        alignItems: 'center',
        zIndex: 3000,
      }}
    >
      <div
        style={{
          color: '#FFF',
          fontSize: '24px',
          fontFamily: 'Inter, sans-serif',
          textAlign: 'center',
          maxWidth: '600px',
          padding: '40px',
          border: '2px solid #B71C1C',
          borderRadius: '12px',
          background: 'rgba(50, 50, 50, 0.95)',
        }}
      >
        {transitionText}
      </div>
    </div>
  ) : null;
}