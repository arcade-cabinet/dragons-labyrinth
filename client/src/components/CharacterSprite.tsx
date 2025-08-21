import { useRef, useState, useEffect, useMemo } from 'react';
import { useFrame, useLoader } from '@react-three/fiber';
import * as THREE from 'three';
import { Billboard } from '@react-three/drei';
import { useGameState } from '../lib/stores/useGameState';
import { useNarrative } from '../lib/stores/useNarrative';

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

  // Load sprite textures
  const characterTexture = useLoader(THREE.TextureLoader, '/sprites/character_sheet.svg');
  const monsterTexture = useLoader(THREE.TextureLoader, '/sprites/monster_sheet.svg');
  
  // Create sprite material with UV mapping
  const spriteMaterial = useMemo(() => {
    const texture = type === 'boss' ? monsterTexture : characterTexture;
    texture.magFilter = THREE.NearestFilter;
    texture.minFilter = THREE.NearestFilter;
    
    // Calculate UV coordinates for sprite frame
    let uvOffset = { x: 0, y: 0 };
    let uvScale = { x: 0.25, y: 0.5 }; // Default to 32x32 sprite in 128x64 sheet
    
    if (type === 'player') {
      uvOffset = { x: 0, y: 0.5 }; // Top row, first sprite
    } else if (type === 'companion') {
      // Different companions get different sprites
      if (name === 'Einar') uvOffset = { x: 0, y: 0 };
      else if (name === 'Mira') uvOffset = { x: 0.25, y: 0 };
      else if (name === 'Sorin') uvOffset = { x: 0.5, y: 0 };
      else if (name === 'Tamara') uvOffset = { x: 0.75, y: 0 };
      else uvOffset = { x: 0, y: 0 };
    } else if (type === 'boss') {
      uvOffset = { x: 0, y: 0.5 }; // Monster sheet
      if (name === 'Forsaken Knight') uvOffset = { x: 0.25, y: 0.5 };
      else if (name === 'Dragon') {
        uvOffset = { x: 0, y: 0 };
        uvScale = { x: 0.5, y: 0.5 }; // Dragon is 2x2
      }
    }
    
    return new THREE.SpriteMaterial({ 
      map: texture,
      transparent: true,
      alphaTest: 0.1
    });
  }, [characterTexture, monsterTexture, type, name]);

  // Character appearance based on type and stage
  const getCharacterAppearance = () => {
    const baseAppearance = {
      player: { 
        scale: 0.8, 
        color: '#FFD700',
        shape: 'sprite',
        emissive: '#FFD700',
        emissiveIntensity: 0.2
      },
      companion: { 
        scale: 0.7, 
        color: color || '#4CAF50',
        shape: 'sprite',
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

  // Create sprite-based character using actual sprite textures
  const renderCharacterMesh = () => {
    return (
      <Billboard
        follow={true}
        lockX={false}
        lockY={false}
        lockZ={false}
        position={worldPos}
      >
        <sprite ref={meshRef} scale={[appearance.scale, appearance.scale, 1]} material={spriteMaterial} />
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
            <meshBasicMaterial color="rgba(0,0,0,0.7)" transparent opacity={0.8} />
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