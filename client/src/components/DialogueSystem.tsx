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

    // Get stage-appropriate dialogue using imported companion data
    const stageNames = ['peace', 'unease', 'dread', 'terror', 'horror'] as const;
    const stageName = stageNames[currentStage] || 'peace';

    switch (companion.id) {
      case 'einar':
        const einarDialogues = {
          peace: ["Feels good to stretch my legs. Think they'll need us back home soon?", "Remember the festival last year? We should hurry back for the next one."],
          unease: ["Did you hear that whisper? Probably just the wind... right?", "People are acting strange. Let's finish this errand quickly."],
          dread: ["They're all gone. Why are we still walking? The dragon is real, I know it.", "If this is what fate holds, maybe it's better to end it now..."],
          terror: ["Please, just end it. I can't bear to be hunted anymore.", "I see my death in every shadow. I don't want the dragon to take me."],
          horror: ["...", "You chose this path."]
        };
        dialogueText = einarDialogues[stageName][Math.floor(Math.random() * einarDialogues[stageName].length)];
        
        if (currentStage >= 2 && companion.morale < 50) {
          choices = [
            {
              text: "Stay strong, we'll get through this together",
              action: () => updateCompanionMorale(companion.id, 15),
            },
            {
              text: "You're right, maybe we should give up",
              action: () => {
                adjustSanity(-10);
                updateCompanionMorale(companion.id, -10);
              },
            },
          ];
        }
        break;

      case 'mira':
        const miraDialogues = {
          peace: ["This is amazing! I've never been this far from the market.", "We should collect some flowers on the way. They'll look lovely at home."],
          unease: ["It's quiet... too quiet. But I'm sure it's nothing.", "Come on, let's make the best of it. Maybe it's a festival for whispers!"],
          dread: ["I can't do this anymore. I'm sorry. I hope you find what you're looking for."],
          terror: ["Why did you leave me? Do you hear them laughing?", "I should have stayed... or maybe none of us should've come."],
          horror: ["You still think there's an ending for you?"]
        };
        dialogueText = miraDialogues[stageName][Math.floor(Math.random() * miraDialogues[stageName].length)];
        break;

      case 'sorin':
        const sorinDialogues = {
          peace: ["Did you know ancient texts mention this path? Most take it for superstition.", "There is rational explanation for dragon myths — but I'm eager to find the truth."],
          unease: ["Whispers? Perhaps they are echoes of a deeper truth.", "This phenomenon is fascinating. I must document it."],
          dread: ["If only I could read these runes... What does 'inevitability' truly mean?", "The dragon is not just a beast. It's an idea. An inevitable end."],
          terror: ["Forgive me. Knowledge must be attained, no matter the cost.", "I see now. It makes sense. You will too, one day."],
          horror: ["There is still a chance. We must align the echoes of our choices."]
        };
        dialogueText = sorinDialogues[stageName][Math.floor(Math.random() * sorinDialogues[stageName].length)];
        
        if (currentStage >= 3 && companion.morale < 40) {
          choices = [
            {
              text: "Don't let the dragon corrupt you",
              action: () => updateCompanionMorale(companion.id, 20),
            },
            {
              text: "Tell me what you've learned",
              action: () => {
                adjustSanity(-5);
                // Sorin shares dangerous knowledge
              },
            },
          ];
        }
        break;

      case 'tamara':
        const tamaraDialogues = {
          peace: ["Do you think the customer will like our bread? I hope it's still warm.", "I've never seen so many trees! They're like a forest from a story."],
          unease: ["Why won't anyone talk to us? Did we do something wrong?", "Please, don't leave me. I don't like the dark."],
          dread: ["I can't sleep. The shadows have teeth.", "If we keep going, will we come back? I don't want to forget home."],
          terror: ["Is it my fault? I'm sorry if it is. Don't let him take me."],
          horror: ["I'm still here. Or am I? Are you?"]
        };
        dialogueText = tamaraDialogues[stageName][Math.floor(Math.random() * tamaraDialogues[stageName].length)];
        
        if (currentStage >= 1 && companion.morale < 70) {
          choices = [
            {
              text: "I'll protect you, don't worry",
              action: () => {
                updateCompanionMorale(companion.id, 20);
                adjustSanity(5); // Protecting innocence helps sanity
              },
            },
            {
              text: "You need to be stronger",
              action: () => {
                updateCompanionMorale(companion.id, -5);
                adjustSanity(-5);
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
