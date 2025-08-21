export function createAmbientSound(stage: number): HTMLAudioElement | null {
  try {
    const audio = new Audio('/sounds/background.mp3');
    audio.loop = true;
    audio.volume = 0.3;
    
    // Adjust audio properties based on stage
    switch (stage) {
      case 0: // Peace
        audio.playbackRate = 1.0;
        audio.volume = 0.2;
        break;
      case 1: // Unease
        audio.playbackRate = 0.9;
        audio.volume = 0.25;
        break;
      case 2: // Dread
        audio.playbackRate = 0.8;
        audio.volume = 0.3;
        break;
      case 3: // Terror
        audio.playbackRate = 0.7;
        audio.volume = 0.35;
        break;
      case 4: // Horror
        audio.playbackRate = 0.6;
        audio.volume = 0.4;
        break;
    }
    
    const playPromise = audio.play();
    if (playPromise !== undefined) {
      playPromise.catch(() => {
        // Audio play prevented by browser policy
        console.log("Ambient audio play prevented");
      });
    }
    
    return audio;
  } catch (error) {
    console.log("Failed to create ambient sound:", error);
    return null;
  }
}

export function getHorrorAudioFilter(stage: number) {
  const intensity = stage / 4; // 0 to 1
  
  return {
    lowpass: intensity > 0.3,
    frequency: 1000 * (1 - intensity),
    reverb: intensity * 0.6,
    distortion: intensity * 0.4,
  };
}