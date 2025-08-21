import { useEffect } from 'react';
import HexagonalWorld from './HexagonalWorld';
import CharacterSprite from './CharacterSprite';
import PathfindingSystem from './PathfindingSystem';
import Camera from './Camera';
import QuestSystem from './QuestSystem';
import BossEncounter from './BossEncounter';
import AudioManager from './AudioManager';
import MonsterSystem from './MonsterSystem';
import SanityEffects from './SanityEffects';
import { useGameState } from '../lib/stores/useGameState';
import { useNarrative } from '../lib/stores/useNarrative';
import { useAudio } from '../lib/stores/useAudio';
import { useCompanions } from '../lib/stores/useCompanions';

export default function Game() {
  const { initializeGame, playerPosition } = useGameState();
  const { currentStage } = useNarrative();
  const { toggleMute } = useAudio();
  const { companions } = useCompanions();

  useEffect(() => {
    initializeGame();
    
    // Initialize audio on first user interaction
    const handleFirstInteraction = () => {
      // Unmute audio on first interaction
      document.removeEventListener('click', handleFirstInteraction);
      document.removeEventListener('keydown', handleFirstInteraction);
    };
    
    document.addEventListener('click', handleFirstInteraction);
    document.addEventListener('keydown', handleFirstInteraction);
    
    return () => {
      document.removeEventListener('click', handleFirstInteraction);
      document.removeEventListener('keydown', handleFirstInteraction);
    };
  }, [initializeGame]);

  return (
    <>
      <Camera />
      <HexagonalWorld />
      <PathfindingSystem />
      <CharacterSprite type="player" position={playerPosition} name="You" />
      {companions.filter(c => c.isActive).map(companion => (
        <CharacterSprite 
          key={companion.id}
          type="companion" 
          position={{ 
            q: playerPosition.q + Math.round(Math.random() * 2 - 1), 
            r: playerPosition.r + Math.round(Math.random() * 2 - 1) 
          }}
          name={companion.name}
          color={companion.color}
        />
      ))}
      <SanityEffects />
      <QuestSystem />
      <BossEncounter />
      <MonsterSystem />
      <AudioManager />
    </>
  );
}
