import { useAudio } from '../lib/stores/useAudio';

export interface AudioConfig {
  volume?: number;
  loop?: boolean;
  fadeIn?: number;
  fadeOut?: number;
}

export function playAudioEffect(audioPath: string, config: AudioConfig = {}) {
  const { volume = 0.5, loop = false } = config;
  
  try {
    const audio = new Audio(audioPath);
    audio.volume = volume;
    audio.loop = loop;
    
    const playPromise = audio.play();
    if (playPromise !== undefined) {
      playPromise.catch(error => {
        console.log("Audio play prevented:", error);
      });
    }
    
    return audio;
  } catch (error) {
    console.error("Error playing audio:", error);
    return null;
  }
}

export function createAmbientSound(stage: number): HTMLAudioElement | null {
  let soundPath = '';
  let volume = 0.3;
  
  switch (stage) {
    case 0: // Peace
      soundPath = '/sounds/background.mp3';
      volume = 0.4;
      break;
    case 1: // Unease
      soundPath = '/sounds/background.mp3';
      volume = 0.3;
      break;
    case 2: // Dread
      soundPath = '/sounds/background.mp3';
      volume = 0.2;
      break;
    case 3: // Terror
      soundPath = '/sounds/background.mp3';
      volume = 0.15;
      break;
    case 4: // Horror
      soundPath = '/sounds/background.mp3';
      volume = 0.1;
      break;
  }
  
  return playAudioEffect(soundPath, { volume, loop: true });
}

export function playUISound(type: 'hit' | 'success' | 'interact') {
  const { playHit, playSuccess } = useAudio.getState();
  
  switch (type) {
    case 'hit':
    case 'interact':
      playHit();
      break;
    case 'success':
      playSuccess();
      break;
  }
}

export function getHorrorAudioFilter(stage: number) {
  // Return audio filter parameters for horror effects
  switch (stage) {
    case 0: // Peace
      return { lowpass: null, distortion: 0, reverb: 0 };
    case 1: // Unease
      return { lowpass: 8000, distortion: 0.1, reverb: 0.2 };
    case 2: // Dread
      return { lowpass: 6000, distortion: 0.2, reverb: 0.4 };
    case 3: // Terror
      return { lowpass: 4000, distortion: 0.4, reverb: 0.6 };
    case 4: // Horror
      return { lowpass: 2000, distortion: 0.6, reverb: 0.8 };
    default:
      return { lowpass: null, distortion: 0, reverb: 0 };
  }
}

export function generateProximityAudio(distance: number, maxDistance: number) {
  // Generate dragon proximity audio cues
  const proximity = 1 - Math.min(distance / maxDistance, 1);
  
  return {
    volume: proximity * 0.8,
    pitch: 1 - proximity * 0.3,
    breathing: proximity > 0.7,
    footsteps: proximity > 0.5,
    whispers: proximity > 0.3,
  };
}
