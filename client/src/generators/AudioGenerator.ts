// Audio generator using procedural synthesis for Dragon's Labyrinth
// Creates dynamic horror audio based on dread level

export class AudioGenerator {
  private audioContext: AudioContext;
  private masterGain: GainNode;
  
  constructor() {
    this.audioContext = new (window.AudioContext || (window as any).webkitAudioContext)();
    this.masterGain = this.audioContext.createGain();
    this.masterGain.connect(this.audioContext.destination);
    this.masterGain.gain.value = 0.5;
  }

  // Generate ambient horror soundscape based on dread level
  generateAmbience(dreadLevel: number): OscillatorNode {
    const oscillator = this.audioContext.createOscillator();
    const gainNode = this.audioContext.createGain();
    const filter = this.audioContext.createBiquadFilter();
    
    // Configure based on dread level
    switch (dreadLevel) {
      case 0: // Peace - gentle nature sounds
        oscillator.type = 'sine';
        oscillator.frequency.value = 200 + Math.random() * 50;
        gainNode.gain.value = 0.05;
        filter.type = 'lowpass';
        filter.frequency.value = 500;
        break;
        
      case 1: // Unease - subtle distortion
        oscillator.type = 'triangle';
        oscillator.frequency.value = 150 + Math.random() * 30;
        gainNode.gain.value = 0.08;
        filter.type = 'bandpass';
        filter.frequency.value = 300;
        filter.Q.value = 5;
        break;
        
      case 2: // Dread - low rumbles
        oscillator.type = 'sawtooth';
        oscillator.frequency.value = 60 + Math.random() * 20;
        gainNode.gain.value = 0.15;
        filter.type = 'lowpass';
        filter.frequency.value = 200;
        break;
        
      case 3: // Terror - harsh dissonance
        oscillator.type = 'square';
        oscillator.frequency.value = 40 + Math.random() * 15;
        gainNode.gain.value = 0.2;
        filter.type = 'highpass';
        filter.frequency.value = 100;
        // Add tremolo effect
        const lfo = this.audioContext.createOscillator();
        lfo.frequency.value = 5 + Math.random() * 3;
        lfo.connect(gainNode.gain);
        lfo.start();
        break;
        
      case 4: // Horror - overwhelming chaos
        oscillator.type = 'sawtooth';
        oscillator.frequency.value = 25 + Math.random() * 10;
        gainNode.gain.value = 0.3;
        filter.type = 'notch';
        filter.frequency.value = 150;
        filter.Q.value = 10;
        // Add distortion
        const distortion = this.createDistortion(400);
        oscillator.connect(distortion);
        distortion.connect(filter);
        break;
    }
    
    // Connect nodes
    if (dreadLevel < 4) {
      oscillator.connect(filter);
    }
    filter.connect(gainNode);
    gainNode.connect(this.masterGain);
    
    return oscillator;
  }

  // Generate monster sounds
  generateMonsterSound(monsterType: string, dreadLevel: number): void {
    const oscillator = this.audioContext.createOscillator();
    const gainNode = this.audioContext.createGain();
    const now = this.audioContext.currentTime;
    
    switch (monsterType) {
      case 'Shadow_Rabbit':
        oscillator.frequency.setValueAtTime(800, now);
        oscillator.frequency.exponentialRampToValueAtTime(400, now + 0.1);
        gainNode.gain.setValueAtTime(0.1, now);
        gainNode.gain.exponentialRampToValueAtTime(0.01, now + 0.2);
        break;
        
      case 'Whispering_Tree':
        oscillator.type = 'triangle';
        oscillator.frequency.setValueAtTime(200, now);
        for (let i = 0; i < 5; i++) {
          oscillator.frequency.setValueAtTime(200 + Math.random() * 50, now + i * 0.1);
        }
        gainNode.gain.setValueAtTime(0.05, now);
        gainNode.gain.linearRampToValueAtTime(0.15, now + 0.5);
        gainNode.gain.linearRampToValueAtTime(0, now + 1);
        break;
        
      case 'Bog_Wraith':
        oscillator.type = 'sawtooth';
        oscillator.frequency.setValueAtTime(100, now);
        oscillator.frequency.linearRampToValueAtTime(50, now + 0.5);
        gainNode.gain.setValueAtTime(0, now);
        gainNode.gain.linearRampToValueAtTime(0.2, now + 0.2);
        gainNode.gain.exponentialRampToValueAtTime(0.01, now + 1);
        break;
        
      case 'Dragon_Echo':
        oscillator.type = 'square';
        oscillator.frequency.setValueAtTime(30, now);
        oscillator.frequency.exponentialRampToValueAtTime(15, now + 2);
        gainNode.gain.setValueAtTime(0.3, now);
        gainNode.gain.linearRampToValueAtTime(0.4, now + 0.5);
        gainNode.gain.exponentialRampToValueAtTime(0.01, now + 3);
        break;
    }
    
    oscillator.connect(gainNode);
    gainNode.connect(this.masterGain);
    
    oscillator.start(now);
    oscillator.stop(now + 3);
  }

  // Generate UI feedback sounds
  generateUISound(action: string, dreadLevel: number): void {
    const oscillator = this.audioContext.createOscillator();
    const gainNode = this.audioContext.createGain();
    const now = this.audioContext.currentTime;
    
    // Corrupt UI sounds as dread increases
    const corruption = dreadLevel / 4;
    
    switch (action) {
      case 'click':
        oscillator.frequency.value = 600 - (corruption * 200);
        oscillator.type = corruption > 0.5 ? 'square' : 'sine';
        gainNode.gain.setValueAtTime(0.1 + (corruption * 0.1), now);
        gainNode.gain.exponentialRampToValueAtTime(0.01, now + 0.05);
        break;
        
      case 'hover':
        oscillator.frequency.value = 400 - (corruption * 100);
        oscillator.type = 'triangle';
        gainNode.gain.setValueAtTime(0.05, now);
        gainNode.gain.exponentialRampToValueAtTime(0.01, now + 0.1);
        break;
        
      case 'error':
        oscillator.frequency.setValueAtTime(200, now);
        oscillator.frequency.exponentialRampToValueAtTime(100, now + 0.1);
        oscillator.type = 'sawtooth';
        gainNode.gain.setValueAtTime(0.2, now);
        gainNode.gain.exponentialRampToValueAtTime(0.01, now + 0.2);
        break;
    }
    
    oscillator.connect(gainNode);
    gainNode.connect(this.masterGain);
    
    oscillator.start(now);
    oscillator.stop(now + 0.5);
  }

  // Create distortion effect for horror sounds
  private createDistortion(amount: number): WaveShaperNode {
    const distortion = this.audioContext.createWaveShaper();
    const samples = 44100;
    const curve = new Float32Array(samples);
    const deg = Math.PI / 180;
    
    for (let i = 0; i < samples; i++) {
      const x = (i * 2) / samples - 1;
      curve[i] = ((3 + amount) * x * 20 * deg) / (Math.PI + amount * Math.abs(x));
    }
    
    distortion.curve = curve;
    distortion.oversample = '4x';
    return distortion;
  }

  // Generate heartbeat sound that speeds up with fear
  generateHeartbeat(fearLevel: number): void {
    const bpm = 60 + (fearLevel * 40); // 60-140 BPM based on fear
    const interval = 60000 / bpm; // Convert to milliseconds
    
    const beat = () => {
      const osc = this.audioContext.createOscillator();
      const gain = this.audioContext.createGain();
      const now = this.audioContext.currentTime;
      
      osc.frequency.value = 40;
      osc.type = 'sine';
      
      gain.gain.setValueAtTime(0, now);
      gain.gain.linearRampToValueAtTime(0.2, now + 0.02);
      gain.gain.exponentialRampToValueAtTime(0.01, now + 0.1);
      
      osc.connect(gain);
      gain.connect(this.masterGain);
      
      osc.start(now);
      osc.stop(now + 0.1);
    };
    
    // Create double beat pattern
    beat();
    setTimeout(beat, interval * 0.3);
    
    // Continue heartbeat based on fear level
    if (fearLevel > 0) {
      setTimeout(() => this.generateHeartbeat(fearLevel), interval);
    }
  }

  // Generate whisper sounds for psychological horror
  generateWhisper(text: string, dreadLevel: number): void {
    // Simulate whisper with noise and formant filtering
    const noise = this.audioContext.createBufferSource();
    const noiseBuffer = this.audioContext.createBuffer(1, this.audioContext.sampleRate * 0.5, this.audioContext.sampleRate);
    const noiseData = noiseBuffer.getChannelData(0);
    
    for (let i = 0; i < noiseData.length; i++) {
      noiseData[i] = (Math.random() * 2 - 1) * 0.1;
    }
    
    noise.buffer = noiseBuffer;
    
    // Create formant filters for speech-like quality
    const formant1 = this.audioContext.createBiquadFilter();
    formant1.type = 'bandpass';
    formant1.frequency.value = 700 + (dreadLevel * 100);
    formant1.Q.value = 10;
    
    const formant2 = this.audioContext.createBiquadFilter();
    formant2.type = 'bandpass';
    formant2.frequency.value = 1220 - (dreadLevel * 50);
    formant2.Q.value = 10;
    
    const gain = this.audioContext.createGain();
    gain.gain.value = 0.05 + (dreadLevel * 0.02);
    
    noise.connect(formant1);
    formant1.connect(formant2);
    formant2.connect(gain);
    gain.connect(this.masterGain);
    
    noise.start();
  }

  // Clean up audio context
  dispose(): void {
    this.audioContext.close();
  }
}