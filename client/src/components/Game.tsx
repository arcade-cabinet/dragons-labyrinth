import { useEffect } from 'react';
import HexWorld from './HexWorld';
import Player from './Player';
import Camera from './Camera';
import BiomeRenderer from './BiomeRenderer';
import QuestSystem from './QuestSystem';
import BossEncounter from './BossEncounter';
import { useGameState } from '../lib/stores/useGameState';
import { useNarrative } from '../lib/stores/useNarrative';
import { useAudio } from '../lib/stores/useAudio';

export default function Game() {
  const { initializeGame } = useGameState();
  const { currentStage } = useNarrative();
  const { toggleMute } = useAudio();

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
      <HexWorld />
      <BiomeRenderer stage={currentStage} />
      <Player />
      <QuestSystem />
      <BossEncounter />
    </>
  );
}
