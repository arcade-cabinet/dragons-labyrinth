import { useState, useEffect } from 'react';
import { useNarrative } from '../lib/stores/useNarrative';
import { useCompanions } from '../lib/stores/useCompanions';
import { useGameState } from '../lib/stores/useGameState';

interface DialogueChoice {
  text: string;
  action: () => void;
  consequence?: string;
}

interface DialogueState {
  speaker: string;
  text: string;
  choices?: DialogueChoice[];
  isVisible: boolean;
}

export default function DialogueSystem() {
  const [dialogue, setDialogue] = useState<DialogueState>({
    speaker: '',
    text: '',
    choices: [],
    isVisible: false,
  });

  const { currentStage, triggerStageTransition } = useNarrative();
  const { companions, updateCompanionMorale } = useCompanions();
  const { adjustSanity } = useGameState();

  // Sample dialogue based on current stage
  useEffect(() => {
    const checkForDialogue = () => {
      // Trigger companion dialogue based on stage
      if (companions.length > 0 && Math.random() < 0.1) {
        const activeCompanions = companions.filter(c => c.isActive);
        if (activeCompanions.length > 0) {
          const companion = activeCompanions[Math.floor(Math.random() * activeCompanions.length)];
          showCompanionDialogue(companion);
        }
      }
    };

    const interval = setInterval(checkForDialogue, 10000); // Check every 10 seconds
    return () => clearInterval(interval);
  }, [companions, currentStage]);

  const showCompanionDialogue = (companion: any) => {
    let dialogueText = '';
    let choices: DialogueChoice[] = [];

    // Get stage-appropriate dialogue
    switch (currentStage) {
      case 0: // Peace
        if (companion.id === 'einar') {
          dialogueText = "Feels good to stretch my legs. Think they'll need us back home soon?";
        } else if (companion.id === 'mira') {
          dialogueText = "This is amazing! I've never been this far from the market.";
        } else if (companion.id === 'tamara') {
          dialogueText = "Do you think the customer will like our bread? I hope it's still warm.";
        }
        break;

      case 1: // Unease
        if (companion.id === 'einar') {
          dialogueText = "Did you hear that whisper? Probably just the wind... right?";
        } else if (companion.id === 'mira') {
          dialogueText = "It's quiet... too quiet. But I'm sure it's nothing.";
        } else if (companion.id === 'tamara') {
          dialogueText = "Why won't anyone talk to us? Did we do something wrong?";
        }
        break;

      case 2: // Dread
        if (companion.id === 'einar') {
          dialogueText = "They're all gone. Why are we still walking? The dragon is real, I know it.";
          choices = [
            {
              text: "Stay strong, we'll get through this",
              action: () => updateCompanionMorale(companion.id, 10),
            },
            {
              text: "Maybe we should turn back",
              action: () => adjustSanity(-5),
            },
          ];
        } else if (companion.id === 'tamara') {
          dialogueText = "I can't sleep. The shadows have teeth.";
        }
        break;

      case 3: // Terror
        if (companion.id === 'einar') {
          dialogueText = "Please, just end it. I can't bear to be hunted anymore.";
          choices = [
            {
              text: "I won't give up on you",
              action: () => updateCompanionMorale(companion.id, 5),
            },
            {
              text: "Maybe... maybe you're right",
              action: () => {
                adjustSanity(-10);
                // Could trigger companion leaving
              },
            },
          ];
        }
        break;
    }

    if (dialogueText) {
      setDialogue({
        speaker: companion.name,
        text: dialogueText,
        choices: choices.length > 0 ? choices : undefined,
        isVisible: true,
      });
    }
  };

  const handleChoice = (choice: DialogueChoice) => {
    choice.action();
    closeDialogue();
  };

  const closeDialogue = () => {
    setDialogue(prev => ({ ...prev, isVisible: false }));
  };

  if (!dialogue.isVisible) return null;

  const getDialogueStyle = () => {
    switch (currentStage) {
      case 0: // Peace
        return {
          background: 'rgba(255, 255, 255, 0.95)',
          color: '#333',
          border: '3px solid #4CAF50',
        };
      case 1: // Unease
        return {
          background: 'rgba(240, 240, 240, 0.9)',
          color: '#444',
          border: '3px solid #FFA726',
        };
      case 2: // Dread
        return {
          background: 'rgba(200, 200, 200, 0.85)',
          color: '#555',
          border: '3px solid #FF7043',
        };
      case 3: // Terror
        return {
          background: 'rgba(100, 100, 100, 0.8)',
          color: '#CCC',
          border: '3px solid #F44336',
        };
      case 4: // Horror
        return {
          background: 'rgba(50, 50, 50, 0.75)',
          color: '#FFF',
          border: '3px solid #B71C1C',
        };
      default:
        return {
          background: 'rgba(255, 255, 255, 0.95)',
          color: '#333',
          border: '3px solid #4CAF50',
        };
    }
  };

  const dialogueStyle = getDialogueStyle();

  return (
    <div
      style={{
        position: 'absolute',
        bottom: '100px',
        left: '50%',
        transform: 'translateX(-50%)',
        width: '80%',
        maxWidth: '600px',
        padding: '20px',
        borderRadius: '12px',
        fontFamily: 'Inter, sans-serif',
        fontSize: '16px',
        zIndex: 1000,
        ...dialogueStyle,
      }}
    >
      <div style={{ fontWeight: 'bold', marginBottom: '10px', fontSize: '18px' }}>
        {dialogue.speaker}
      </div>
      
      <div style={{ marginBottom: '15px', lineHeight: '1.4' }}>
        {dialogue.text}
      </div>

      {dialogue.choices ? (
        <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
          {dialogue.choices.map((choice, index) => (
            <button
              key={index}
              onClick={() => handleChoice(choice)}
              style={{
                padding: '10px 15px',
                borderRadius: '6px',
                border: '2px solid',
                borderColor: dialogueStyle.color,
                background: 'transparent',
                color: dialogueStyle.color,
                fontFamily: 'Inter, sans-serif',
                fontSize: '14px',
                cursor: 'pointer',
                transition: 'all 0.2s',
              }}
              onMouseEnter={(e) => {
                e.currentTarget.style.background = dialogueStyle.color;
                e.currentTarget.style.color = dialogueStyle.background.replace(/rgba?\([^)]+\)/, 'white');
              }}
              onMouseLeave={(e) => {
                e.currentTarget.style.background = 'transparent';
                e.currentTarget.style.color = dialogueStyle.color;
              }}
            >
              {choice.text}
            </button>
          ))}
        </div>
      ) : (
        <button
          onClick={closeDialogue}
          style={{
            padding: '8px 20px',
            borderRadius: '6px',
            border: '2px solid',
            borderColor: dialogueStyle.color,
            background: 'transparent',
            color: dialogueStyle.color,
            fontFamily: 'Inter, sans-serif',
            fontSize: '14px',
            cursor: 'pointer',
            marginLeft: 'auto',
            display: 'block',
          }}
        >
          Continue
        </button>
      )}
    </div>
  );
}
