import { useRef } from 'react';
import { useFrame } from '@react-three/fiber';
import { useNarrative } from '../lib/stores/useNarrative';
import * as THREE from 'three';

export default function Lighting() {
  const { currentStage } = useNarrative();
  const sunRef = useRef<THREE.DirectionalLight>(null);
  
  useFrame((state) => {
    if (!sunRef.current) return;
    
    // Dynamic lighting based on horror stage
    const time = state.clock.elapsedTime;
    
    switch (currentStage) {
      case 0: // Peace - bright sunny day
        sunRef.current.intensity = 1.2;
        sunRef.current.color.setHex(0xFFFAE5);
        break;
      case 1: // Unease - overcast
        sunRef.current.intensity = 0.8;
        sunRef.current.color.setHex(0xE0E0E0);
        break;
      case 2: // Dread - storm approaching
        sunRef.current.intensity = 0.6 + Math.sin(time * 2) * 0.1;
        sunRef.current.color.setHex(0x9E9E9E);
        break;
      case 3: // Terror - darkness with flashes
        sunRef.current.intensity = 0.3 + (Math.random() < 0.01 ? 0.5 : 0);
        sunRef.current.color.setHex(0x4A0E0E);
        break;
      case 4: // Horror - pitch black with red glow
        sunRef.current.intensity = 0.15;
        sunRef.current.color.setHex(0x8B0000);
        break;
    }
  });
  
  return (
    <>
      {/* Ambient light for base visibility */}
      <ambientLight 
        intensity={currentStage === 0 ? 0.5 : 0.3 - (currentStage * 0.05)}
        color={currentStage >= 3 ? '#2A0A0A' : '#FFFFFF'}
      />
      
      {/* Main sunlight */}
      <directionalLight
        ref={sunRef}
        position={[10, 20, 10]}
        castShadow
        shadow-mapSize={[2048, 2048]}
        shadow-camera-far={50}
        shadow-camera-left={-20}
        shadow-camera-right={20}
        shadow-camera-top={20}
        shadow-camera-bottom={-20}
      />
      
      {/* Horror stage point lights for dramatic effect */}
      {currentStage >= 3 && (
        <>
          <pointLight
            position={[0, 5, 0]}
            intensity={0.5}
            color="#FF0000"
            distance={15}
            decay={2}
          />
          <pointLight
            position={[10, 2, -10]}
            intensity={0.3}
            color="#FF4444"
            distance={10}
            decay={2}
          />
        </>
      )}
      
      {/* Fog for atmosphere */}
      <fog
        attach="fog"
        args={[
          currentStage === 0 ? '#87CEEB' : 
          currentStage === 1 ? '#B0BEC5' :
          currentStage === 2 ? '#616161' :
          currentStage === 3 ? '#2A2A2A' :
          '#000000',
          5,
          40 - (currentStage * 5)
        ]}
      />
    </>
  );
}