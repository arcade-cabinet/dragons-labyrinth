import { useRef, useEffect } from 'react';
import { useFrame } from '@react-three/fiber';
import * as THREE from 'three';
import { useNarrative } from '../lib/stores/useNarrative';
import { useGameState } from '../lib/stores/useGameState';
import { getHexPosition } from '../utils/hexUtils';

interface CompanionProps {
  companion: {
    id: string;
    name: string;
    isActive: boolean;
    morale: number;
    color?: string;
    archetype?: string;
  };
  playerPosition: { q: number; r: number };
  index: number;
}

export default function Companion({ companion, playerPosition, index }: CompanionProps) {
  const meshRef = useRef<THREE.Mesh>(null);
  const { currentStage } = useNarrative();
  const { sanity } = useGameState();
  
  // Calculate companion position relative to player
  const getCompanionOffset = (index: number) => {
    const positions = [
      { q: -1, r: 0 },   // West of player
      { q: 1, r: 0 },    // East of player  
      { q: 0, r: -1 },   // Northwest of player
      { q: 0, r: 1 },    // Southeast of player
    ];
    return positions[index % positions.length] || { q: 0, r: 0 };
  };

  const offset = getCompanionOffset(index);
  const companionPos = getHexPosition(
    playerPosition.q + offset.q,
    playerPosition.r + offset.r,
    1
  );

  // Calculate companion appearance based on stage and morale
  const getCompanionAppearance = () => {
    let color = companion.color || "#FFA726";
    let scale = [0.5, 1, 0.5];
    let opacity = 1;
    
    // Modify appearance based on horror stage
    switch (currentStage) {
      case 0: // Peace
        opacity = 1;
        break;
      case 1: // Unease
        color = adjustColorForStage(color, 0.9);
        break;
      case 2: // Dread
        color = adjustColorForStage(color, 0.7);
        if (companion.morale < 50) {
          scale = [0.4, 0.8, 0.4]; // Appear smaller/hunched
        }
        break;
      case 3: // Terror
        color = adjustColorForStage(color, 0.5);
        opacity = companion.morale / 100;
        if (companion.morale < 30) {
          scale = [0.3, 0.6, 0.3]; // Very hunched
        }
        break;
      case 4: // Horror
        color = "#424242"; // Monochrome
        opacity = Math.max(0.3, companion.morale / 100);
        scale = [0.3, 0.5, 0.3];
        break;
    }
    
    // Adjust for sanity hallucinations
    if (sanity < 50 && Math.random() < 0.1) {
      opacity *= 0.5; // Flickering effect for hallucinations
    }
    
    return { color, scale, opacity };
  };

  const adjustColorForStage = (originalColor: string, factor: number): string => {
    const color = new THREE.Color(originalColor);
    color.multiplyScalar(factor);
    return `#${color.getHexString()}`;
  };

  const { color, scale, opacity } = getCompanionAppearance();

  // Animation for companion behavior
  useFrame((state, delta) => {
    if (!meshRef.current || !companion.isActive) return;

    const time = state.clock.elapsedTime;
    
    // Idle animation - subtle breathing/swaying
    const breathingScale = 1 + Math.sin(time * 2 + index) * 0.02;
    meshRef.current.scale.set(
      scale[0] * breathingScale,
      scale[1] * breathingScale,
      scale[2] * breathingScale
    );

    // Add nervous fidgeting in higher horror stages
    if (currentStage >= 2) {
      const fidget = Math.sin(time * 5 + index) * 0.05;
      meshRef.current.rotation.y = fidget;
    }

    // Proximity to player affects behavior
    const distance = Math.sqrt(offset.q * offset.q + offset.r * offset.r);
    if (distance > 1.5) {
      // Move closer to player when far away
      meshRef.current.position.lerp(
        new THREE.Vector3(companionPos.x, 0.5, companionPos.z),
        delta * 3
      );
    }

    // Morale-based animations
    if (companion.morale < 30) {
      // Low morale - hunched posture
      meshRef.current.rotation.x = -0.2;
    } else if (companion.morale > 80) {
      // High morale - upright posture
      meshRef.current.rotation.x = 0.1;
    }
  });

  // Don't render if companion is not active
  if (!companion.isActive) {
    return null;
  }

  return (
    <group>
      {/* Main companion mesh */}
      <mesh
        ref={meshRef}
        position={[companionPos.x, 0.5, companionPos.z]}
        castShadow
        receiveShadow
      >
        <boxGeometry args={[0.5, 1, 0.5]} />
        <meshLambertMaterial 
          color={color} 
          transparent={opacity < 1}
          opacity={opacity}
        />
      </mesh>

      {/* Morale indicator (floating icon above companion) */}
      {currentStage <= 2 && (
        <mesh position={[companionPos.x, 1.8, companionPos.z]}>
          <sphereGeometry args={[0.1, 8, 8]} />
          <meshBasicMaterial 
            color={companion.morale > 60 ? "#4CAF50" : companion.morale > 30 ? "#FFC107" : "#F44336"}
          />
        </mesh>
      )}

      {/* Name tag (only in peaceful stages) */}
      {currentStage === 0 && (
        <mesh position={[companionPos.x, 2.2, companionPos.z]}>
          <planeGeometry args={[1, 0.3]} />
          <meshBasicMaterial 
            color="white" 
            transparent 
            opacity={0.8}
          />
        </mesh>
      )}

      {/* Horror stage effects */}
      {currentStage >= 3 && companion.morale < 20 && (
        <>
          {/* Trauma visualization - floating dark particles */}
          <mesh position={[companionPos.x + Math.sin(Date.now() * 0.01) * 0.2, 1.2, companionPos.z]}>
            <sphereGeometry args={[0.05, 6, 6]} />
            <meshBasicMaterial color="#212121" transparent opacity={0.6} />
          </mesh>
          <mesh position={[companionPos.x - Math.cos(Date.now() * 0.01) * 0.15, 1.4, companionPos.z]}>
            <sphereGeometry args={[0.03, 6, 6]} />
            <meshBasicMaterial color="#424242" transparent opacity={0.4} />
          </mesh>
        </>
      )}

      {/* Betrayal indicator for terror stage */}
      {currentStage === 3 && companion.archetype === 'scholar' && companion.morale < 40 && (
        <mesh position={[companionPos.x, 1.5, companionPos.z]}>
          <octahedronGeometry args={[0.1]} />
          <meshBasicMaterial color="#B71C1C" />
        </mesh>
      )}
    </group>
  );
}
