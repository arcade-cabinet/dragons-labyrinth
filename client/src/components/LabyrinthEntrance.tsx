import { useRef } from 'react';
import { useFrame } from '@react-three/fiber';
import * as THREE from 'three';
import { Text, Billboard } from '@react-three/drei';
import { useNarrative } from '../lib/stores/useNarrative';
import { useGameState } from '../lib/stores/useGameState';

interface LabyrinthEntranceProps {
  position: [number, number, number];
}

export default function LabyrinthEntrance({ position }: LabyrinthEntranceProps) {
  const { currentStage } = useNarrative();
  const { playerPosition } = useGameState();
  const portalRef = useRef<THREE.Mesh>(null);
  const glowRef = useRef<THREE.PointLight>(null);
  
  useFrame((state) => {
    if (!portalRef.current || !glowRef.current) return;
    
    const time = state.clock.elapsedTime;
    
    // Pulsing effect
    portalRef.current.scale.setScalar(1 + Math.sin(time * 2) * 0.1);
    
    // Rotate portal
    portalRef.current.rotation.y = time * 0.5;
    
    // Glow intensity based on player proximity
    const playerWorldPos = {
      x: Math.sqrt(3) * playerPosition.q + Math.sqrt(3)/2 * playerPosition.r,
      z: 3/2 * playerPosition.r
    };
    
    const distance = Math.sqrt(
      Math.pow(playerWorldPos.x - position[0], 2) + 
      Math.pow(playerWorldPos.z - position[2], 2)
    );
    
    glowRef.current.intensity = Math.max(0, 2 - distance * 0.1);
  });
  
  if (currentStage < 3) return null;
  
  return (
    <group position={position}>
      {/* Portal structure */}
      <mesh ref={portalRef}>
        <torusGeometry args={[2, 0.5, 8, 20]} />
        <meshStandardMaterial 
          color="#1A237E"
          emissive="#311B92"
          emissiveIntensity={0.5}
          roughness={0.2}
          metalness={0.8}
        />
      </mesh>
      
      {/* Dark center */}
      <mesh>
        <circleGeometry args={[1.5, 32]} />
        <meshBasicMaterial color="#000000" opacity={0.9} transparent />
      </mesh>
      
      {/* Glow effect */}
      <pointLight 
        ref={glowRef}
        color="#8B0000"
        distance={10}
        decay={2}
      />
      
      {/* Warning text */}
      <Billboard
        follow={true}
        lockX={false}
        lockY={false}
        lockZ={false}
        position={[0, 3, 0]}
      >
        <Text
          color="#B71C1C"
          fontSize={0.5}
          maxWidth={200}
          lineHeight={1}
          letterSpacing={0.02}
          textAlign="center"
          font="/fonts/inter-v12-latin-regular.woff"
          anchorX="center"
          anchorY="middle"
          outlineWidth={0.1}
          outlineColor="#000000"
        >
          {currentStage === 3 ? "The Labyrinth Calls..." : "Enter the Dragon's Domain"}
        </Text>
      </Billboard>
      
      {/* Interaction zone */}
      <mesh
        position={[0, 0, 0]}
        visible={false}
        onClick={() => {
          if (currentStage >= 4) {
            console.log("Entering the Dragon's Labyrinth...");
            // Trigger labyrinth scene transition
            window.dispatchEvent(new CustomEvent('enterLabyrinth'));
          }
        }}
      >
        <cylinderGeometry args={[3, 3, 0.1, 8]} />
      </mesh>
    </group>
  );
}