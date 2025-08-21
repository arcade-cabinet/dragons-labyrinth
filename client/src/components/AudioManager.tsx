import { useEffect, useRef } from 'react';
import { useNarrative } from '../lib/stores/useNarrative';
import { useGameState } from '../lib/stores/useGameState';
import { useAudio } from '../lib/stores/useAudio';
import { createAmbientSound, getHorrorAudioFilter } from '../utils/audioUtils';

export default function AudioManager() {
  const { currentStage } = useNarrative();
  const { sanity, playerPosition } = useGameState();
  const ambientAudioRef = useRef<HTMLAudioElement | null>(null);
  const whisperTimerRef = useRef<NodeJS.Timeout | null>(null);
  const dragonSoundRef = useRef<HTMLAudioElement | null>(null);

  // Stage-based ambient audio
  useEffect(() => {
    // Stop current ambient audio
    if (ambientAudioRef.current) {
      ambientAudioRef.current.pause();
      ambientAudioRef.current = null;
    }

    // Start new ambient audio for current stage
    const newAmbient = createAmbientSound(currentStage);
    ambientAudioRef.current = newAmbient;

    // Apply horror audio filters
    if (newAmbient) {
      const filters = getHorrorAudioFilter(currentStage);
      // Note: In production, you'd apply Web Audio API filters here
      // For now, we adjust volume and playback rate based on stage
      if (filters.lowpass) {
        newAmbient.volume *= (1 - filters.distortion);
      }
    }

    return () => {
      if (ambientAudioRef.current) {
        ambientAudioRef.current.pause();
      }
    };
  }, [currentStage]);

  // Sanity-based audio effects
  useEffect(() => {
    if (whisperTimerRef.current) {
      clearInterval(whisperTimerRef.current);
    }

    if (sanity < 70) {
      // Play whispers and false audio cues when sanity is low
      const interval = Math.max(3000, (sanity / 100) * 10000); // More frequent at lower sanity
      
      whisperTimerRef.current = setInterval(() => {
        if (Math.random() < 0.3) {
          playWhisperEffect();
        }
      }, interval);
    }

    return () => {
      if (whisperTimerRef.current) {
        clearInterval(whisperTimerRef.current);
      }
    };
  }, [sanity]);

  // Dragon proximity audio (Horror stage)
  useEffect(() => {
    if (currentStage === 4) {
      const distance = Math.sqrt(playerPosition.q * playerPosition.q + playerPosition.r * playerPosition.r);
      const proximity = Math.max(0, 1 - distance / 8);
      
      if (proximity > 0.3 && !dragonSoundRef.current) {
        playDragonProximitySound(proximity);
      }
      
      if (dragonSoundRef.current) {
        dragonSoundRef.current.volume = proximity * 0.6;
      }
    } else if (dragonSoundRef.current) {
      dragonSoundRef.current.pause();
      dragonSoundRef.current = null;
    }
  }, [currentStage, playerPosition]);

  const playWhisperEffect = () => {
    try {
      const audio = new Audio('/sounds/background.mp3');
      audio.volume = 0.2;
      audio.playbackRate = 0.7; // Slower, more ominous
      
      const playPromise = audio.play();
      if (playPromise !== undefined) {
        playPromise.catch(() => {
          // Audio play prevented, ignore
        });
      }
      
      // Stop after a short duration
      setTimeout(() => {
        audio.pause();
      }, 2000);
    } catch (error) {
      console.log("Whisper audio failed:", error);
    }
  };

  const playDragonProximitySound = (proximity: number) => {
    try {
      const audio = new Audio('/sounds/background.mp3');
      audio.volume = proximity * 0.4;
      audio.loop = true;
      audio.playbackRate = 0.5; // Deep, slow breathing
      
      const playPromise = audio.play();
      if (playPromise !== undefined) {
        playPromise.catch(() => {
          // Audio play prevented, ignore
        });
      }
      
      dragonSoundRef.current = audio;
    } catch (error) {
      console.log("Dragon proximity audio failed:", error);
    }
  };

  // Stage-specific sound effects
  const playStageTransitionSound = () => {
    const { playHit } = useAudio.getState();
    playHit(); // Use existing audio system for transitions
  };

  useEffect(() => {
    playStageTransitionSound();
  }, [currentStage]);

  return null; // This component only manages audio, no visual rendering
}