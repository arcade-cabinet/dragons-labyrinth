import { useRef, useState, useEffect } from 'react';
import { useFrame, useLoader } from '@react-three/fiber';
import * as THREE from 'three';
import { Billboard } from '@react-three/drei';
import { useGameState } from '../lib/stores/useGameState';
import { useNarrative } from '../lib/stores/useNarrative';
import { getSpriteForType } from '../data/sprites';

interface CharacterSpriteProps {
  type: 'player' | 'companion' | 'npc' | 'boss';
  position: { q: number; r: number };
  name?: string;
  color?: string;
}

export default function CharacterSprite({ type, position, name, color = '#FFFFFF' }: CharacterSpriteProps) {
  const meshRef = useRef<THREE.Mesh>(null);
  const { currentStage } = useNarrative();
  const [animationFrame, setAnimationFrame] = useState(0);
  
  // Convert hex to world position
  const hexToWorld = (q: number, r: number) => {
    const size = 1;
    const x = size * (Math.sqrt(3) * q + Math.sqrt(3)/2 * r);
    const z = size * (3/2 * r);
    return [x, 0.8, z] as [number, number, number];
  };

  const worldPos = hexToWorld(position.q, position.r);

  // Character appearance based on type and stage
  const getCharacterAppearance = () => {
    const baseAppearance = {
      player: { 
        scale: 0.7, 
        color: '#FFD700',
        shape: 'cylinder',
        emissive: '#FFD700',
        emissiveIntensity: 0.2
      },
      companion: { 
        scale: 0.6, 
        color: color || '#4CAF50',
        shape: 'cone',
        emissive: undefined,
        emissiveIntensity: 0
      },
      npc: { 
        scale: 0.5, 
        color: '#9E9E9E',
        shape: 'box',
        emissive: undefined,
        emissiveIntensity: 0
      },
      boss: { 
        scale: 1.0, 
        color: '#B71C1C',
        shape: 'octahedron',
        emissive: '#B71C1C',
        emissiveIntensity: 0.3
      }
    };
    
    const appearance = baseAppearance[type];
    
    // Modify appearance based on stage
    if (currentStage >= 3 && type === 'companion') {
      appearance.color = '#424242'; // Companions look darker in terror
    }
    
    return appearance;
  };

  const appearance = getCharacterAppearance();

  // Animate character
  useFrame((state) => {
    if (!meshRef.current) return;
    
    const time = state.clock.elapsedTime;
    
    // Idle animation
    meshRef.current.position.y = 0.8 + Math.sin(time * 2) * 0.05;
    
    // Rotate bosses
    if (type === 'boss') {
      meshRef.current.rotation.y = time * 0.5;
    }
    
    // Shake effect in horror stages
    if (currentStage >= 3 && type === 'companion') {
      meshRef.current.position.x += Math.sin(time * 10) * 0.01;
    }
  });

  // Create sprite-based character
  const renderCharacterMesh = () => {
    // For now, use billboarded planes with solid colors
    // Will replace with actual sprite textures when loaded
    return (
      <Billboard
        follow={true}
        lockX={false}
        lockY={false}
        lockZ={false}
        position={worldPos}
      >
        <mesh ref={meshRef} scale={appearance.scale}>
          <planeGeometry args={[1.2, 1.6]} />
          <meshBasicMaterial 
            color={appearance.color}
            transparent
            opacity={0.9}
            side={THREE.DoubleSide}
          />
        </mesh>
      </Billboard>
    );
  };

  return (
    <group>
      {renderCharacterMesh()}
      
      {/* Name label */}
      {name && (
        <Billboard
          follow={true}
          lockX={false}
          lockY={false}
          lockZ={false}
          position={[worldPos[0], worldPos[1] + 1, worldPos[2]]}
        >
          <mesh>
            <planeGeometry args={[2, 0.5]} />
            <meshBasicMaterial transparent opacity={0}>
              <canvasTexture
                attach="map"
                image={(() => {
                  const canvas = document.createElement('canvas');
                  canvas.width = 256;
                  canvas.height = 64;
                  const ctx = canvas.getContext('2d');
                  if (ctx) {
                    ctx.fillStyle = 'rgba(0, 0, 0, 0.7)';
                    ctx.fillRect(0, 0, 256, 64);
                    ctx.fillStyle = 'white';
                    ctx.font = '24px Inter';
                    ctx.textAlign = 'center';
                    ctx.fillText(name, 128, 40);
                  }
                  return canvas;
                })()}
              />
            </meshBasicMaterial>
          </mesh>
        </Billboard>
      )}
      
      {/* Shadow/aura effect for important characters */}
      {(type === 'player' || type === 'boss') && (
        <mesh
          position={[worldPos[0], 0.02, worldPos[2]]}
          rotation={[-Math.PI / 2, 0, 0]}
        >
          <ringGeometry args={[0.3, 0.6, 16]} />
          <meshBasicMaterial 
            color={type === 'player' ? '#FFD700' : '#B71C1C'}
            transparent
            opacity={0.3}
          />
        </mesh>
      )}
    </group>
  );
}