import { useState, useEffect } from 'react';
import { useNarrative } from '../lib/stores/useNarrative';
import { useGameState } from '../lib/stores/useGameState';
import { useCompanions } from '../lib/stores/useCompanions';

interface BossChoice {
  id: string;
  text: string;
  consequence: string;
  action: () => void;
}

interface Boss {
  id: string;
  name: string;
  description: string;
  choices: BossChoice[];
}

export default function BossEncounter() {
  const [currentBoss, setCurrentBoss] = useState<Boss | null>(null);
  const [showBossUI, setShowBossUI] = useState(false);
  
  const { currentStage, triggerStageTransition } = useNarrative();
  const { adjustSanity, playerPosition } = useGameState();
  const { updateCompanionMorale, removeCompanion, companions } = useCompanions();

  // Trigger boss encounters based on stage and conditions
  useEffect(() => {
    const checkBossEncounter = () => {
      const edgeDistance = Math.sqrt(playerPosition.q * playerPosition.q + playerPosition.r * playerPosition.r);
      
      // Trigger boss when player reaches edge of current stage area
      if (edgeDistance > 5 && !showBossUI) {
        switch (currentStage) {
          case 1: // Unease - Hollow Caretaker
            setCurrentBoss({
              id: 'hollow_caretaker',
              name: 'Hollow Caretaker',
              description: 'An old man with sunken eyes appears, holding a lantern that emits no light. His voice speaks in loops of the past.',
              choices: [
                {
                  id: 'persuade',
                  text: 'Engage in dialogue, soothing him with memories',
                  consequence: 'Free his soul and gain information. Companions gain morale.',
                  action: () => {
                    companions.forEach(c => updateCompanionMorale(c.id, 10));
                    adjustSanity(5);
                    triggerStageTransition();
                    setShowBossUI(false);
                  }
                },
                {
                  id: 'kill',
                  text: 'Attack and defeat him',
                  consequence: 'He drops a cursed lantern. Companions lose morale.',
                  action: () => {
                    companions.forEach(c => updateCompanionMorale(c.id, -15));
                    adjustSanity(-10);
                    triggerStageTransition();
                    setShowBossUI(false);
                  }
                }
              ]
            });
            setShowBossUI(true);
            break;
            
          case 2: // Dread - Forsaken Knight
            setCurrentBoss({
              id: 'forsaken_knight',
              name: 'Forsaken Knight',
              description: 'A knight in rusted armor blocks your path. His cracked helm reveals emptiness within, yet he continues his eternal vigil.',
              choices: [
                {
                  id: 'empathy',
                  text: 'Sheathe your weapon and listen to his story',
                  consequence: 'He recounts his failure. You gain a relic but the dragon draws closer.',
                  action: () => {
                    adjustSanity(10);
                    // Dragon proximity increases (represented by faster stage progression)
                    triggerStageTransition();
                    setShowBossUI(false);
                  }
                },
                {
                  id: 'brutality',
                  text: 'Attack and defeat him brutally',
                  consequence: 'You obtain his armor but it whispers constantly, causing hallucinations.',
                  action: () => {
                    adjustSanity(-20);
                    triggerStageTransition();
                    setShowBossUI(false);
                  }
                }
              ]
            });
            setShowBossUI(true);
            break;
            
          case 3: // Terror - Traitor Companion
            const activeCompanions = companions.filter(c => c.isActive);
            if (activeCompanions.length > 0) {
              const traitor = activeCompanions[0]; // First companion becomes traitor
              setCurrentBoss({
                id: 'traitor_companion',
                name: `${traitor.name} (Traitor)`,
                description: `${traitor.name} has been twisted by the dragon's influence. Their eyes glow with unnatural light as they turn against you.`,
                choices: [
                  {
                    id: 'forgive',
                    text: 'Try to reason with them and offer forgiveness',
                    consequence: 'They regain some sanity and may help in the labyrinth. Other companions may distrust you.',
                    action: () => {
                      updateCompanionMorale(traitor.id, 50);
                      // Mark as redeemed for ending purposes
                      adjustSanity(-5);
                      triggerStageTransition();
                      setShowBossUI(false);
                    }
                  },
                  {
                    id: 'execute',
                    text: 'Kill them to protect the others',
                    consequence: 'You gain immediate safety but lose the possibility of redemption.',
                    action: () => {
                      removeCompanion(traitor.id);
                      adjustSanity(-25);
                      triggerStageTransition();
                      setShowBossUI(false);
                    }
                  }
                ]
              });
              setShowBossUI(true);
            }
            break;
            
          case 4: // Horror - The Dragon (final encounter)
            setCurrentBoss({
              id: 'dragon',
              name: 'The Dragon',
              description: 'In the heart of the labyrinth, you finally confront the ancient intelligence. It speaks not with words, but directly into your mind.',
              choices: [
                {
                  id: 'acceptance',
                  text: 'Step toward the dragon and allow yourself to be consumed',
                  consequence: 'The cycle continues, and the world forgets you.',
                  action: () => {
                    // Acceptance ending
                    alert('Acceptance Ending: You become one with the dragon, perpetuating the cycle.');
                    setShowBossUI(false);
                  }
                },
                {
                  id: 'defiance',
                  text: 'Hurl yourself into the pit, denying the dragon your essence',
                  consequence: 'You die, but prevent the dragon\'s victory.',
                  action: () => {
                    // Defiance ending
                    alert('Defiance Ending: Your sacrifice breaks the immediate cycle, but at great cost.');
                    setShowBossUI(false);
                  }
                },
                {
                  id: 'understanding',
                  text: 'Engage the dragon in dialogue, seeking to understand its nature',
                  consequence: 'You attempt to break the cycle through comprehension.',
                  action: () => {
                    // Understanding ending (requires certain conditions)
                    const redemptionPossible = companions.some(c => c.morale > 50);
                    if (redemptionPossible) {
                      alert('Understanding Ending: Through wisdom and compassion, you free all trapped souls and end the dragon\'s cycle.');
                    } else {
                      alert('Failed Understanding: Without enough positive connections, your attempt fails.');
                    }
                    setShowBossUI(false);
                  }
                }
              ]
            });
            setShowBossUI(true);
            break;
        }
      }
    };

    const interval = setInterval(checkBossEncounter, 1000);
    return () => clearInterval(interval);
  }, [currentStage, playerPosition, showBossUI, companions, updateCompanionMorale, adjustSanity, triggerStageTransition, removeCompanion]);

  if (!showBossUI || !currentBoss) return null;

  return (
    <div
      style={{
        position: 'absolute',
        top: '0',
        left: '0',
        width: '100%',
        height: '100%',
        backgroundColor: 'rgba(0, 0, 0, 0.8)',
        display: 'flex',
        justifyContent: 'center',
        alignItems: 'center',
        zIndex: 2000,
      }}
    >
      <div
        style={{
          background: 'rgba(50, 50, 50, 0.95)',
          color: '#FFF',
          padding: '30px',
          borderRadius: '12px',
          border: '3px solid #B71C1C',
          maxWidth: '600px',
          fontFamily: 'Inter, sans-serif',
          textAlign: 'center',
        }}
      >
        <h2 style={{ marginBottom: '20px', fontSize: '24px', color: '#F44336' }}>
          {currentBoss.name}
        </h2>
        
        <p style={{ marginBottom: '30px', fontSize: '16px', lineHeight: '1.4' }}>
          {currentBoss.description}
        </p>
        
        <div style={{ display: 'flex', flexDirection: 'column', gap: '15px' }}>
          {currentBoss.choices.map((choice) => (
            <div key={choice.id} style={{ textAlign: 'left' }}>
              <button
                onClick={choice.action}
                style={{
                  width: '100%',
                  padding: '15px',
                  borderRadius: '8px',
                  border: '2px solid #F44336',
                  background: 'transparent',
                  color: '#FFF',
                  fontFamily: 'Inter, sans-serif',
                  fontSize: '16px',
                  cursor: 'pointer',
                  marginBottom: '8px',
                  transition: 'all 0.2s',
                }}
                onMouseEnter={(e) => {
                  e.currentTarget.style.background = '#F44336';
                }}
                onMouseLeave={(e) => {
                  e.currentTarget.style.background = 'transparent';
                }}
              >
                {choice.text}
              </button>
              <p style={{ fontSize: '12px', color: '#CCC', fontStyle: 'italic' }}>
                Consequence: {choice.consequence}
              </p>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}
